use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::utils::history::{ensure_guild_dir, guild_fixtures_path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    pub home: String,
    pub home_score: i64,
    pub away: String,
    pub away_score: i64,
    pub timestamp: String,
}

pub fn load_fixtures(guild_id: &str) -> Result<Vec<Fixture>> {
    ensure_guild_dir(guild_id)?;
    let path = guild_fixtures_path(guild_id);
    if !Path::new(&path).exists() {
        return Ok(vec![]);
    }
    let raw = fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path))?;
    let fixtures: Vec<Fixture> =
        serde_json::from_str(&raw).with_context(|| format!("Failed to parse {}", path))?;
    Ok(fixtures)
}

pub fn save_fixtures(guild_id: &str, fixtures: &[Fixture]) -> Result<()> {
    ensure_guild_dir(guild_id)?;
    let path = guild_fixtures_path(guild_id);
    let json = serde_json::to_string_pretty(fixtures).context("Failed to serialize fixtures")?;
    fs::write(&path, format!("{}\n", json)).with_context(|| format!("Failed to write {}", path))?;
    Ok(())
}

pub fn append_fixture(guild_id: &str, fixture: Fixture) -> Result<()> {
    let mut fixtures = load_fixtures(guild_id)?;
    fixtures.push(fixture);
    save_fixtures(guild_id, &fixtures)
}

/// Return the last `count` results for `team` (oldest first).
pub fn team_form(fixtures: &[Fixture], team: &str, count: usize) -> Vec<char> {
    let team_upper = team.to_uppercase();
    let results: Vec<char> = fixtures
        .iter()
        .filter(|f| f.home == team_upper || f.away == team_upper)
        .map(|f| {
            if f.home == team_upper {
                match f.home_score.cmp(&f.away_score) {
                    std::cmp::Ordering::Greater => 'W',
                    std::cmp::Ordering::Less => 'L',
                    std::cmp::Ordering::Equal => 'D',
                }
            } else {
                match f.away_score.cmp(&f.home_score) {
                    std::cmp::Ordering::Greater => 'W',
                    std::cmp::Ordering::Less => 'L',
                    std::cmp::Ordering::Equal => 'D',
                }
            }
        })
        .collect();

    // Take the last `count` results, keeping chronological (oldest first) order.
    let start = results.len().saturating_sub(count);
    results[start..].to_vec()
}

/// Return a human-readable streak description, e.g. "3 game winning streak 🔥".
pub fn streak_description(form: &[char]) -> String {
    if form.is_empty() {
        return "No games played yet".to_string();
    }
    let last = *form.last().unwrap();
    let count = form.iter().rev().take_while(|&&r| r == last).count();
    match last {
        'W' => format!("{} game winning streak 🔥", count),
        'L' => format!("{} game losing streak", count),
        'D' => {
            if count == 1 {
                "1 draw".to_string()
            } else {
                format!("{} game unbeaten run", count)
            }
        }
        _ => String::new(),
    }
}

pub fn format_result_badge(r: char) -> &'static str {
    match r {
        'W' => "🟢 W",
        'L' => "🔴 L",
        'D' => "🟡 D",
        _ => "❓",
    }
}
