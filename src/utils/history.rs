use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::utils::table_utils::{recalculate_positions, sort_table, Table};

<<<<<<< HEAD
=======
pub const DATA_DIR: &str = "data";
>>>>>>> origin/copilot/rebuild-league-table-bot
pub const HISTORY_LIMIT: usize = 20;
pub const GUILD_ONLY_MESSAGE: &str = "This bot can only be used inside a Discord server!";
pub const FRESH_TABLE_MESSAGE: &str =
    "No table found for this server. Starting fresh! Use `/addteam` to add teams.";

<<<<<<< HEAD
// ── Path helpers ─────────────────────────────────────────────────────────────

pub fn guild_dir(guild_id: &str) -> String {
    format!("data/{}", guild_id)
}

pub fn guild_table_path(guild_id: &str) -> String {
    format!("data/{}/table.json", guild_id)
}

pub fn guild_history_path(guild_id: &str) -> String {
    format!("data/{}/history.json", guild_id)
}

pub fn guild_fixtures_path(guild_id: &str) -> String {
    format!("data/{}/fixtures.json", guild_id)
}

pub fn guild_config_path(guild_id: &str) -> String {
    format!("data/{}/config.json", guild_id)
}

// ── Guild initialisation ─────────────────────────────────────────────────────

pub fn ensure_guild_dir(guild_id: &str) -> Result<()> {
    let dir = guild_dir(guild_id);
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create guild directory {}", dir))?;

    let table_path = guild_table_path(guild_id);
    if !Path::new(&table_path).exists() {
        save_table(guild_id, &[])?;
    }

    let history_path = guild_history_path(guild_id);
    if !Path::new(&history_path).exists() {
        save_history(guild_id, &[])?;
    }
=======
pub fn get_table_path(guild_id: &str) -> PathBuf {
    PathBuf::from(format!("{}/{}/table.json", DATA_DIR, guild_id))
}

pub fn get_history_path(guild_id: &str) -> PathBuf {
    PathBuf::from(format!("{}/{}/history.json", DATA_DIR, guild_id))
}
>>>>>>> origin/copilot/rebuild-league-table-bot

pub fn ensure_data_dir() -> Result<()> {
    fs::create_dir_all(DATA_DIR).context("Failed to create data directory")?;
    Ok(())
}

<<<<<<< HEAD
// ── Table ────────────────────────────────────────────────────────────────────

pub fn load_table(guild_id: &str) -> Result<Table> {
    ensure_guild_dir(guild_id)?;
    let path = guild_table_path(guild_id);
    let raw = fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path))?;
    let mut table: Table =
        serde_json::from_str(&raw).with_context(|| format!("Failed to parse {}", path))?;
=======
pub fn table_exists(guild_id: &str) -> bool {
    get_table_path(guild_id).exists()
}

pub fn load_table(guild_id: &str) -> Result<Table> {
    let path = get_table_path(guild_id);

    if !path.exists() {
        save_table(guild_id, &vec![])?;
        return Ok(vec![]);
    }

    let raw =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;
    let mut table: Table = serde_json::from_str(&raw)
        .with_context(|| format!("Failed to parse {}", path.display()))?;
>>>>>>> origin/copilot/rebuild-league-table-bot

    for team in &mut table {
        team.club = team.club.trim().to_uppercase();
    }
    sort_table(&mut table);
    recalculate_positions(&mut table);
    Ok(table)
}

<<<<<<< HEAD
pub fn save_table(guild_id: &str, table: &[Team]) -> Result<()> {
    let path = guild_table_path(guild_id);
    let json = serde_json::to_string_pretty(table).context("Failed to serialize table")?;
    fs::write(&path, format!("{}\n", json)).with_context(|| format!("Failed to write {}", path))?;
    Ok(())
}

// ── History ──────────────────────────────────────────────────────────────────

pub fn load_history(guild_id: &str) -> Result<Vec<Table>> {
    ensure_guild_dir(guild_id)?;
    let path = guild_history_path(guild_id);
    let raw = fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path))?;
    let mut history: Vec<Table> =
        serde_json::from_str(&raw).with_context(|| format!("Failed to parse {}", path))?;
=======
pub fn save_table(guild_id: &str, table: &Table) -> Result<()> {
    let path = get_table_path(guild_id);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }

    let json = serde_json::to_string_pretty(table).context("Failed to serialize table")?;
    fs::write(&path, format!("{}\n", json))
        .with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

pub fn load_history(guild_id: &str) -> Result<Vec<Table>> {
    let path = get_history_path(guild_id);
    if !path.exists() {
        save_history(guild_id, &vec![])?;
        return Ok(vec![]);
    }

    let raw =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;
    let mut history: Vec<Table> = serde_json::from_str(&raw)
        .with_context(|| format!("Failed to parse {}", path.display()))?;
>>>>>>> origin/copilot/rebuild-league-table-bot

    for table in &mut history {
        for team in table.iter_mut() {
            team.club = team.club.trim().to_uppercase();
        }
        sort_table(table);
        recalculate_positions(table);
    }
    Ok(history)
}

pub fn save_history(guild_id: &str, history: &[Table]) -> Result<()> {
<<<<<<< HEAD
    let path = guild_history_path(guild_id);
    let json = serde_json::to_string_pretty(history).context("Failed to serialize history")?;
    fs::write(&path, format!("{}\n", json)).with_context(|| format!("Failed to write {}", path))?;
=======
    let path = get_history_path(guild_id);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }

    let json = serde_json::to_string_pretty(history).context("Failed to serialize history")?;
    fs::write(&path, format!("{}\n", json))
        .with_context(|| format!("Failed to write {}", path.display()))?;
>>>>>>> origin/copilot/rebuild-league-table-bot
    Ok(())
}

pub fn push_snapshot(history: &mut Vec<Table>, snapshot: Table, max_entries: usize) {
    if history.len() >= max_entries {
        history.remove(0);
    }
    history.push(snapshot);
}

pub fn pop_snapshot(history: &mut Vec<Table>) -> Option<Table> {
    history.pop()
}
