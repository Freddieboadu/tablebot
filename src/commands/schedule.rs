use poise::serenity_prelude as serenity;

use crate::utils::table_utils::normalize_team_name;
use crate::{Context, Error};

/// View one schedule or list all schedules.
#[poise::command(slash_command)]
pub async fn schedule(
    ctx: Context<'_>,
    #[description = "Schedule name (e.g. 'Group A', 'Knockout') — leave blank to list all"]
    name: Option<String>,
    #[description = "Gameweek number to view (leave blank for next upcoming)"] gameweek: Option<u32>,
    #[description = "Filter by team name"] team: Option<String>,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.get(),
        None => {
            ctx.send(
                poise::CreateReply::default()
                    .content("This command must be used in a server.")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    let guild_lock = ctx.data().get_guild(guild_id).await;
    let guild = guild_lock.lock().await;

    if guild.schedules.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .content("No schedules found. Ask an admin to run `/generateschedule` first.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    if name.is_none() {
        let lines: Vec<String> = guild
            .schedules
            .iter()
            .map(|s| {
                let total = s.matches.len();
                let played = s.matches.iter().filter(|m| m.played).count();
                let gws = s.matches.iter().map(|m| m.gameweek).max().unwrap_or(0);
                format!("**{}** — {} fixtures, {} played, {} gameweeks", s.name, total, played, gws)
            })
            .collect();

        let embed = serenity::CreateEmbed::new()
            .title("📅 Schedules")
            .description(lines.join("\n"))
            .footer(serenity::CreateEmbedFooter::new("Use /schedule name:<name> to view a schedule's fixtures"))
            .color(serenity::Color::BLUE);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let name_key = name.as_deref().unwrap().to_lowercase();
    let sched = match guild.schedules.iter().find(|s| s.name.to_lowercase() == name_key) {
        Some(s) => s,
        None => {
            let available: Vec<_> = guild.schedules.iter().map(|s| s.name.as_str()).collect();
            ctx.send(
                poise::CreateReply::default()
                    .content(format!("Schedule **{}** not found. Available: {}", name.unwrap(), available.join(", ")))
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    let team_filter = team.as_deref().map(normalize_team_name);
    let target_gw = if let Some(gw) = gameweek {
        gw
    } else if team_filter.is_some() {
        0
    } else {
        sched
            .matches
            .iter()
            .filter(|m| !m.played)
            .map(|m| m.gameweek)
            .min()
            .unwrap_or(1)
    };

    let matches: Vec<_> = sched
        .matches
        .iter()
        .filter(|m| {
            let gw_ok = target_gw == 0 || m.gameweek == target_gw;
            let team_ok = match &team_filter {
                Some(t) => normalize_team_name(&m.home_team) == *t || normalize_team_name(&m.away_team) == *t,
                None => true,
            };
            gw_ok && team_ok
        })
        .collect();

    if matches.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .content("No fixtures found for that filter.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let mut lines: Vec<String> = Vec::new();
    let mut current_gw = 0u32;

    for m in &matches {
        if m.gameweek != current_gw {
            current_gw = m.gameweek;
            if !lines.is_empty() {
                lines.push(String::new());
            }
            lines.push(format!("__**Gameweek {}**__", current_gw));
        }

        let result_str = if m.played {
            let home_n = normalize_team_name(&m.home_team);
            let away_n = normalize_team_name(&m.away_team);
            if let Some(f) = guild.fixtures.iter().find(|f| {
                normalize_team_name(&f.home_team) == home_n && normalize_team_name(&f.away_team) == away_n
            }) {
                let outcome = if let Some(ref t) = team_filter {
                    if normalize_team_name(&f.home_team) == *t {
                        if f.home_score > f.away_score { "✅ W" } else if f.home_score == f.away_score { "🟡 D" } else { "❌ L" }
                    } else {
                        if f.away_score > f.home_score { "✅ W" } else if f.home_score == f.away_score { "🟡 D" } else { "❌ L" }
                    }
                } else {
                    ""
                };
                format!("~~{} **{} – {}** {}~~ {}", f.home_team, f.home_score, f.away_score, f.away_team, outcome)
            } else {
                format!("~~{} vs {}~~ ✅", m.home_team, m.away_team)
            }
        } else {
            format!("**{}** vs **{}**", m.home_team, m.away_team)
        };

        lines.push(result_str);
    }

    let title = match (&team_filter, gameweek) {
        (Some(t), _) => format!("📅 {} — {}", sched.name, t),
        (None, Some(gw)) => format!("📅 {} — Gameweek {}", sched.name, gw),
        (None, None) => format!("📅 {} — Gameweek {} (Next Up)", sched.name, target_gw),
    };

    let desc = lines.join("\n");
    let desc = if desc.len() > 3900 {
        format!("{}\n*(truncated — use `gameweek:X` to narrow)*", &desc[..3900])
    } else {
        desc
    };

    let total_gws = sched.matches.iter().map(|m| m.gameweek).max().unwrap_or(0);
    let played = sched.matches.iter().filter(|m| m.played).count();
    let remaining = sched.matches.len() - played;

    let embed = serenity::CreateEmbed::new()
        .title(title)
        .description(desc)
        .footer(serenity::CreateEmbedFooter::new(format!(
            "{} gameweeks · {} played · {} remaining",
            total_gws, played, remaining
        )))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
