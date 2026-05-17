use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::utils::table_utils::{recalculate_positions, sort_table, Table, Team};

pub const TABLE_PATH: &str = "data/table.json";
pub const HISTORY_PATH: &str = "data/history.json";
pub const HISTORY_LIMIT: usize = 20;

pub fn seeded_table() -> Table {
    vec![
        Team {
            pos: 1,
            club: "CHELSEA".to_string(),
            pl: 18,
            w: 9,
            d: 5,
            l: 4,
            gd: 6,
            pts: 32,
        },
        Team {
            pos: 2,
            club: "PSG".to_string(),
            pl: 18,
            w: 9,
            d: 3,
            l: 6,
            gd: 19,
            pts: 31,
        },
        Team {
            pos: 3,
            club: "BARCELONA".to_string(),
            pl: 18,
            w: 10,
            d: 1,
            l: 7,
            gd: 15,
            pts: 31,
        },
        Team {
            pos: 4,
            club: "NEWCASTLE".to_string(),
            pl: 17,
            w: 9,
            d: 2,
            l: 6,
            gd: 13,
            pts: 29,
        },
        Team {
            pos: 5,
            club: "CELTIC".to_string(),
            pl: 17,
            w: 8,
            d: 3,
            l: 6,
            gd: 10,
            pts: 27,
        },
        Team {
            pos: 6,
            club: "DORTMUND".to_string(),
            pl: 18,
            w: 8,
            d: 2,
            l: 8,
            gd: -2,
            pts: 26,
        },
        Team {
            pos: 7,
            club: "MAN UNITED".to_string(),
            pl: 18,
            w: 7,
            d: 4,
            l: 7,
            gd: -9,
            pts: 25,
        },
        Team {
            pos: 8,
            club: "REAL SALT LAKE".to_string(),
            pl: 18,
            w: 6,
            d: 5,
            l: 7,
            gd: -9,
            pts: 23,
        },
        Team {
            pos: 9,
            club: "SPORTING CP".to_string(),
            pl: 18,
            w: 4,
            d: 3,
            l: 11,
            gd: -18,
            pts: 15,
        },
        Team {
            pos: 10,
            club: "MALMO FF".to_string(),
            pl: 16,
            w: 3,
            d: 2,
            l: 11,
            gd: -25,
            pts: 11,
        },
    ]
}

pub fn ensure_data_files() -> Result<()> {
    fs::create_dir_all("data").context("Failed to create data directory")?;

    if !Path::new(TABLE_PATH).exists() {
        let mut table = seeded_table();
        sort_table(&mut table);
        recalculate_positions(&mut table);
        save_table(&table)?;
    }

    if !Path::new(HISTORY_PATH).exists() {
        save_history(&vec![])?;
    }

    Ok(())
}

pub fn load_table() -> Result<Table> {
    let raw = fs::read_to_string(TABLE_PATH).context("Failed to read data/table.json")?;
    let mut table: Table = serde_json::from_str(&raw).context("Failed to parse data/table.json")?;

    for team in &mut table {
        team.club = team.club.trim().to_uppercase();
    }

    sort_table(&mut table);
    recalculate_positions(&mut table);
    Ok(table)
}

pub fn save_table(table: &Table) -> Result<()> {
    let json = serde_json::to_string_pretty(table).context("Failed to serialize table")?;
    fs::write(TABLE_PATH, format!("{}\n", json)).context("Failed to write data/table.json")?;
    Ok(())
}

pub fn load_history() -> Result<Vec<Table>> {
    let raw = fs::read_to_string(HISTORY_PATH).context("Failed to read data/history.json")?;
    let mut history: Vec<Table> =
        serde_json::from_str(&raw).context("Failed to parse data/history.json")?;

    for table in &mut history {
        for team in table.iter_mut() {
            team.club = team.club.trim().to_uppercase();
        }
        sort_table(table);
        recalculate_positions(table);
    }

    Ok(history)
}

pub fn save_history(history: &[Table]) -> Result<()> {
    let json = serde_json::to_string_pretty(history).context("Failed to serialize history")?;
    fs::write(HISTORY_PATH, format!("{}\n", json)).context("Failed to write data/history.json")?;
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
