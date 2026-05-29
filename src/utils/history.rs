use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use crate::utils::table_utils::{recalculate_positions, sort_table, Fixture, Settings, Table};

pub const HISTORY_LIMIT: usize = 20;
pub const FIXTURES_LIMIT: usize = 50;

fn guild_dir(guild_id: u64) -> PathBuf {
    PathBuf::from(format!("data/{}", guild_id))
}

#[derive(Debug, Default, Clone)]
pub struct GuildData {
    pub table: Table,
    pub history: Vec<Table>,
    pub fixtures: Vec<Fixture>,
    pub settings: Settings,
}

impl GuildData {
    pub fn load(guild_id: u64) -> Result<Self> {
        let dir = guild_dir(guild_id);
        fs::create_dir_all(&dir)?;
        Ok(GuildData {
            table: load_table(guild_id)?,
            history: load_history(guild_id)?,
            fixtures: load_fixtures(guild_id)?,
            settings: load_settings(guild_id)?,
        })
    }
}

pub fn load_table(guild_id: u64) -> Result<Table> {
    let dir = guild_dir(guild_id);
    fs::create_dir_all(&dir)?;
    let path = dir.join("table.json");
    if !path.exists() {
        fs::write(&path, "[]\n")?;
    }
    let raw = fs::read_to_string(&path)?;
    let mut table: Table = serde_json::from_str(&raw)?;
    for team in &mut table {
        team.club = team.club.trim().to_uppercase();
    }
    sort_table(&mut table);
    recalculate_positions(&mut table);
    Ok(table)
}

pub fn save_table(guild_id: u64, table: &Table) -> Result<()> {
    let dir = guild_dir(guild_id);
    fs::create_dir_all(&dir)?;
    let json = serde_json::to_string_pretty(table)?;
    fs::write(dir.join("table.json"), format!("{}\n", json))?;
    Ok(())
}

pub fn load_history(guild_id: u64) -> Result<Vec<Table>> {
    let dir = guild_dir(guild_id);
    fs::create_dir_all(&dir)?;
    let path = dir.join("history.json");
    if !path.exists() {
        fs::write(&path, "[]\n")?;
    }
    let raw = fs::read_to_string(&path)?;
    let mut history: Vec<Table> = serde_json::from_str(&raw)?;
    for table in &mut history {
        for team in table.iter_mut() {
            team.club = team.club.trim().to_uppercase();
        }
        sort_table(table);
        recalculate_positions(table);
    }
    Ok(history)
}

pub fn save_history(guild_id: u64, history: &[Table]) -> Result<()> {
    let dir = guild_dir(guild_id);
    fs::create_dir_all(&dir)?;
    let json = serde_json::to_string_pretty(history)?;
    fs::write(dir.join("history.json"), format!("{}\n", json))?;
    Ok(())
}

pub fn load_fixtures(guild_id: u64) -> Result<Vec<Fixture>> {
    let dir = guild_dir(guild_id);
    fs::create_dir_all(&dir)?;
    let path = dir.join("fixtures.json");
    if !path.exists() {
        fs::write(&path, "[]\n")?;
    }
    let raw = fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&raw)?)
}

pub fn save_fixtures(guild_id: u64, fixtures: &[Fixture]) -> Result<()> {
    let dir = guild_dir(guild_id);
    fs::create_dir_all(&dir)?;
    let json = serde_json::to_string_pretty(fixtures)?;
    fs::write(dir.join("fixtures.json"), format!("{}\n", json))?;
    Ok(())
}

pub fn load_settings(guild_id: u64) -> Result<Settings> {
    let dir = guild_dir(guild_id);
    fs::create_dir_all(&dir)?;
    let path = dir.join("settings.json");
    if !path.exists() {
        let json = serde_json::to_string_pretty(&Settings::default())?;
        fs::write(&path, format!("{}\n", json))?;
    }
    let raw = fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&raw)?)
}

pub fn save_settings(guild_id: u64, settings: &Settings) -> Result<()> {
    let dir = guild_dir(guild_id);
    fs::create_dir_all(&dir)?;
    let json = serde_json::to_string_pretty(settings)?;
    fs::write(dir.join("settings.json"), format!("{}\n", json))?;
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

pub fn push_fixture(fixtures: &mut Vec<Fixture>, fixture: Fixture) {
    fixtures.push(fixture);
    if fixtures.len() > FIXTURES_LIMIT {
        fixtures.remove(0);
    }
}
