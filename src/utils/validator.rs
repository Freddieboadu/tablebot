use anyhow::{bail, Result};

use crate::utils::table_utils::{find_team_index, normalize_team_name, Table};

pub fn validate_match_input(
    table: &Table,
    home_team: &str,
    home_score: i64,
    away_team: &str,
    away_score: i64,
) -> Result<()> {
    if home_score < 0 || away_score < 0 {
        bail!("Scores must be zero or greater.");
    }

    let home = normalize_team_name(home_team);
    let away = normalize_team_name(away_team);

    if home == away {
        bail!("Home and away teams must be different.");
    }

    if find_team_index(table, &home).is_none() {
        bail!("Home team '{}' does not exist.", home);
    }

    if find_team_index(table, &away).is_none() {
        bail!("Away team '{}' does not exist.", away);
    }

    Ok(())
}

pub fn validate_new_team_name(table: &Table, team_name: &str) -> Result<()> {
    let normalized = normalize_team_name(team_name);

    if normalized.is_empty() {
        bail!("Team name cannot be empty.");
    }

    if find_team_index(table, &normalized).is_some() {
        bail!("Team '{}' already exists.", normalized);
    }

    Ok(())
}
