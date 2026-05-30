use std::env;
use std::fs;
use std::net::SocketAddr;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Json, Router,
};
use tower_http::cors::CorsLayer;

use crate::utils::history::load_table;

/// Build the axum router for the public league-table website.
fn app() -> Router {
    Router::new()
        .route("/", get(root_redirect))
        .route("/g/:guild_id", get(table_page))
        .route("/api/table/:guild_id", get(table_api))
        .layer(CorsLayer::permissive())
}

/// Start the web server. Runs forever; intended to be spawned as a task.
pub async fn serve(port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            println!("🌐 Web server listening on http://0.0.0.0:{port}");
            if let Err(err) = axum::serve(listener, app()).await {
                eprintln!("Web server error: {err}");
            }
        }
        Err(err) => {
            eprintln!("Failed to bind web server on port {port}: {err}");
        }
    }
}

/// Scan the data directory for guilds that have a table.json file.
fn discover_guilds() -> Vec<u64> {
    let mut ids = Vec::new();
    if let Ok(entries) = fs::read_dir("data") {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    if let Ok(id) = name.parse::<u64>() {
                        if entry.path().join("table.json").exists() {
                            ids.push(id);
                        }
                    }
                }
            }
        }
    }
    ids.sort_unstable();
    ids
}

/// JSON API: returns the sorted league table for a guild.
async fn table_api(Path(guild_id): Path<u64>) -> Response {
    match load_table(guild_id) {
        Ok(table) => Json(table).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "No table found for this server." })),
        )
            .into_response(),
    }
}

/// Root: send visitors straight to a table page instead of a landing list.
/// Uses DEFAULT_GUILD_ID if set, otherwise the only discovered guild.
async fn root_redirect() -> Response {
    // 1. Explicit default wins.
    if let Ok(id) = env::var("DEFAULT_GUILD_ID") {
        if id.trim().parse::<u64>().is_ok() {
            return Redirect::to(&format!("/g/{}", id.trim())).into_response();
        }
    }

    // 2. If there's exactly one table on disk, go to it.
    let guilds = discover_guilds();
    if guilds.len() == 1 {
        return Redirect::to(&format!("/g/{}", guilds[0])).into_response();
    }

    // 3. Otherwise fall back to a simple chooser.
    let cards = if guilds.is_empty() {
        "<p class=\"empty\">No league tables yet. Add teams with the Discord bot to get started.</p>".to_string()
    } else {
        guilds
            .iter()
            .map(|id| {
                format!(
                    "<a class=\"card\" href=\"/g/{id}\"><span class=\"card-id\">Server {id}</span><span class=\"card-go\">View table →</span></a>"
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let body = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<title>PBL League Tables</title>
<style>{styles}</style>
</head>
<body>
<header class="topbar"><div class="wrap"><h1>PBL <span>League Tables</span></h1></div></header>
<main class="wrap">
<div class="cards">
{cards}
</div>
</main>
<footer class="foot"><div class="wrap">Powered by PBL Table Bot</div></footer>
</body>
</html>"#,
        styles = STYLES,
        cards = cards,
    );

    Html(body).into_response()
}

/// Premier League-styled table page for a single guild.
async fn table_page(Path(guild_id): Path<u64>) -> Response {
    if load_table(guild_id).is_err() {
        return (
            StatusCode::NOT_FOUND,
            Html("<h1>No league table found for this server.</h1>".to_string()),
        )
            .into_response();
    }

    let body = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<title>PBL League Table</title>
<style>{styles}</style>
</head>
<body>
<header class="topbar"><div class="wrap"><h1>PBL <span>League Table</span></h1></div></header>
<main class="wrap">
<div class="table-card">
<div class="table-head">
<div class="th-pos">Pos</div>
<div class="th-club">Club</div>
<div class="th-num">Pl</div>
<div class="th-num">W</div>
<div class="th-num">D</div>
<div class="th-num">L</div>
<div class="th-num">GD</div>
<div class="th-num th-pts">Pts</div>
</div>
<div id="rows" class="rows"><div class="loading">Loading…</div></div>
</div>
<p class="updated" id="updated"></p>
</main>
<footer class="foot"><div class="wrap">Powered by PBL Table Bot · auto-refreshes</div></footer>
<script>
const GUILD_ID = "{guild_id}";

function badge(pos, total) {{
  // Top spot = champion zone, bottom three = relegation zone (PL-style accent bars).
  if (pos === 1) return "champion";
  if (pos <= 4) return "ucl";
  if (total > 4 && pos > total - 3) return "releg";
  return "";
}}

async function load() {{
  try {{
    const res = await fetch(`/api/table/${{GUILD_ID}}`, {{ cache: "no-store" }});
    if (!res.ok) throw new Error("not found");
    const data = await res.json();
    const total = data.length;
    const rows = document.getElementById("rows");
    if (!total) {{
      rows.innerHTML = '<div class="loading">No teams yet.</div>';
      return;
    }}
    rows.innerHTML = data.map(t => {{
      const zone = badge(t.pos, total);
      const gd = (t.gd > 0 ? "+" : "") + t.gd;
      return `<div class="row ${{zone}}">
        <div class="td-pos"><span class="zonebar"></span>${{t.pos}}</div>
        <div class="td-club">${{escapeHtml(t.club)}}</div>
        <div class="td-num">${{t.pl}}</div>
        <div class="td-num">${{t.w}}</div>
        <div class="td-num">${{t.d}}</div>
        <div class="td-num">${{t.l}}</div>
        <div class="td-num">${{gd}}</div>
        <div class="td-num td-pts">${{t.pts}}</div>
      </div>`;
    }}).join("");
    document.getElementById("updated").textContent =
      "Last updated " + new Date().toLocaleTimeString();
  }} catch (e) {{
    document.getElementById("rows").innerHTML =
      '<div class="loading">Could not load table.</div>';
  }}
}}

function escapeHtml(s) {{
  return String(s).replace(/[&<>"']/g, c => ({{
    "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;"
  }}[c]));
}}

load();
setInterval(load, 10000);
</script>
</body>
</html>"#,
        styles = STYLES,
        guild_id = guild_id,
    );

    Html(body).into_response()
}

/// Shared Premier League-inspired stylesheet.
const STYLES: &str = r#"
* { box-sizing: border-box; margin: 0; padding: 0; }
:root {
  --pl-purple: #37003c;
  --pl-magenta: #e90052;
  --pl-green: #00ff85;
  --pl-cyan: #04f5ff;
  --ink: #2c2c2c;
  --line: #ececff;
}
body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  background: #f3f3fb;
  color: var(--ink);
  min-height: 100vh;
}
.wrap { max-width: 760px; margin: 0 auto; padding: 0 16px; }
.topbar {
  background: linear-gradient(90deg, var(--pl-purple), #5a0a63);
  padding: 22px 0;
  box-shadow: 0 2px 12px rgba(55,0,60,.25);
}
.topbar h1 {
  color: #fff; font-size: 22px; font-weight: 800; letter-spacing: .5px;
}
.topbar h1 span { color: var(--pl-green); }
main.wrap { padding-top: 24px; padding-bottom: 40px; }

/* Table card */
.table-card {
  background: #fff;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 6px 24px rgba(55,0,60,.10);
}
.table-head, .row {
  display: grid;
  grid-template-columns: 56px 1fr 34px 34px 34px 34px 44px 48px;
  align-items: center;
}
.table-head {
  background: var(--pl-purple);
  color: #fff;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: .4px;
  font-weight: 700;
  padding: 12px 10px;
}
.table-head .th-club { text-align: left; padding-left: 6px; }
.table-head [class^="th-num"] { text-align: center; }
.th-pts { color: var(--pl-green); }

.rows .row {
  padding: 10px;
  border-bottom: 1px solid var(--line);
  font-size: 15px;
  transition: background .15s ease;
}
.rows .row:last-child { border-bottom: none; }
.rows .row:hover { background: #faf7ff; }

.td-pos {
  position: relative;
  font-weight: 700;
  padding-left: 14px;
  color: var(--ink);
}
.td-pos .zonebar {
  position: absolute;
  left: 0; top: 50%;
  transform: translateY(-50%);
  width: 4px; height: 22px;
  border-radius: 2px;
  background: transparent;
}
.row.champion .zonebar { background: var(--pl-green); }
.row.ucl .zonebar { background: var(--pl-cyan); }
.row.releg .zonebar { background: var(--pl-magenta); }

.td-club {
  font-weight: 700;
  text-align: left;
  padding-left: 6px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.td-num { text-align: center; color: #555; }
.td-pts { font-weight: 800; color: var(--pl-purple); }

.loading { padding: 24px; text-align: center; color: #999; }
.updated { text-align: center; color: #999; font-size: 12px; margin-top: 12px; }

/* Landing cards */
.cards { display: grid; gap: 12px; }
.card {
  display: flex; justify-content: space-between; align-items: center;
  background: #fff; border-radius: 12px; padding: 18px 20px;
  text-decoration: none; color: var(--pl-purple); font-weight: 700;
  box-shadow: 0 4px 16px rgba(55,0,60,.08);
  transition: transform .12s ease, box-shadow .12s ease;
}
.card:hover { transform: translateY(-2px); box-shadow: 0 8px 22px rgba(55,0,60,.16); }
.card-go { color: var(--pl-magenta); font-size: 14px; }
.empty { color: #777; text-align: center; padding: 40px 0; }

.foot { padding: 20px 0 36px; }
.foot .wrap { text-align: center; color: #aaa; font-size: 12px; }

@media (max-width: 520px) {
  .table-head, .row { grid-template-columns: 44px 1fr 28px 28px 28px 28px 38px 40px; }
  .rows .row { font-size: 13px; }
  .table-head { font-size: 10px; }
}
"#;
