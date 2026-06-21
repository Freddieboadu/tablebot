use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub pos: usize,
    pub club: String,
    pub pl: i32,
    pub w: i32,
    pub d: i32,
    pub l: i32,
    pub gd: i32,
    pub pts: i32,
}

pub type Table = Vec<Team>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    pub home_team: String,
    pub away_team: String,
    pub home_score: i32,
    pub away_score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    pub admin_role_id: Option<u64>,
    pub log_channel_id: Option<u64>,
}

/// A single fixture in a named schedule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledMatch {
    pub gameweek: u32,
    pub home_team: String,
    pub away_team: String,
    pub played: bool,
}

/// The type of competition a `NamedSchedule` represents.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleKind {
    /// Full double round-robin (home & away) — standard league season.
    RoundRobin,
    /// Single round-robin — every pair meets once.
    SingleRoundRobin,
    /// Single-elimination knockout bracket.
    Knockout,
}

/// A named collection of fixtures — e.g. "Group A", "Preseason", "Knockout".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedSchedule {
    pub name: String,
    pub kind: ScheduleKind,
    pub matches: Vec<ScheduledMatch>,
}

/// Generate a full double round-robin schedule for `teams` using the circle
/// method. Groups every `games_per_week` rounds into a numbered gameweek.
pub fn generate_schedule(teams: &[String], games_per_week: u32) -> Vec<ScheduledMatch> {
    let mut roster = teams.to_vec();
    // If odd number of teams, add a BYE placeholder so the algorithm works.
    let had_bye = roster.len() % 2 != 0;
    if had_bye {
        roster.push("BYE".to_string());
    }
    let n = roster.len();

    // Build rounds using the circle/Berger method.
    // First half: rounds 1..(n-1)
    // Second half: same pairs but home/away swapped
    let mut all_rounds: Vec<Vec<(String, String)>> = Vec::new();

    for half in 0..2u32 {
        for round in 0..(n - 1) {
            let mut pairs: Vec<(String, String)> = Vec::new();

            // Fixed team is always roster[0]; rotate the rest.
            let fixed = &roster[0];
            let mut circle: Vec<&String> = roster[1..].iter().collect();
            circle.rotate_left(round);

            // Pair fixed vs last in circle
            let opponent = circle[n / 2 - 1];
            if fixed != "BYE" && opponent != "BYE" {
                if half == 0 {
                    pairs.push((fixed.clone(), opponent.clone()));
                } else {
                    pairs.push((opponent.clone(), fixed.clone()));
                }
            }

            // Pair remaining circle members
            for i in 0..(n / 2 - 1) {
                let home = circle[i];
                let away = circle[n - 2 - i];
                if home != "BYE" && away != "BYE" {
                    if half == 0 {
                        pairs.push((home.clone(), away.clone()));
                    } else {
                        pairs.push((away.clone(), home.clone()));
                    }
                }
            }

            if !pairs.is_empty() {
                all_rounds.push(pairs);
            }
        }
    }

    // Assign gameweeks: every `games_per_week` rounds = 1 gameweek.
    let mut matches: Vec<ScheduledMatch> = Vec::new();
    for (round_idx, pairs) in all_rounds.iter().enumerate() {
        let gameweek = (round_idx as u32 / games_per_week) + 1;
        for (home, away) in pairs {
            matches.push(ScheduledMatch {
                gameweek,
                home_team: home.clone(),
                away_team: away.clone(),
                played: false,
            });
        }
    }

    matches
}

/// Generate a single round-robin schedule (each pair plays once).
/// Groups every `games_per_week` rounds into a numbered gameweek.
pub fn generate_single_robin(teams: &[String], games_per_week: u32) -> Vec<ScheduledMatch> {
    let mut roster = teams.to_vec();
    if roster.len() % 2 != 0 {
        roster.push("BYE".to_string());
    }
    let n = roster.len();
    let mut all_rounds: Vec<Vec<(String, String)>> = Vec::new();

    for round in 0..(n - 1) {
        let mut pairs: Vec<(String, String)> = Vec::new();
        let fixed = &roster[0];
        let mut circle: Vec<&String> = roster[1..].iter().collect();
        circle.rotate_left(round);

        let opponent = circle[n / 2 - 1];
        if fixed != "BYE" && opponent != "BYE" {
            pairs.push((fixed.clone(), opponent.clone()));
        }
        for i in 0..(n / 2 - 1) {
            let home = circle[i];
            let away = circle[n - 2 - i];
            if home != "BYE" && away != "BYE" {
                pairs.push((home.clone(), away.clone()));
            }
        }
        if !pairs.is_empty() {
            all_rounds.push(pairs);
        }
    }

    let mut matches = Vec::new();
    for (round_idx, pairs) in all_rounds.iter().enumerate() {
        let gameweek = (round_idx as u32 / games_per_week) + 1;
        for (home, away) in pairs {
            matches.push(ScheduledMatch { gameweek, home_team: home.clone(), away_team: away.clone(), played: false });
        }
    }
    matches
}

/// Generate the next knockout round. `teams` should be supplied in seed order
/// (highest seed first). Matches are seeded 1 vs N, 2 vs N-1, etc.
/// `start_gameweek` is the gameweek number to stamp on these matches.
/// If there is an odd team out, the highest seed receives a bye (no match).
pub fn generate_knockout_round(teams: &[String], start_gameweek: u32) -> Vec<ScheduledMatch> {
    let n = teams.len();
    let pairs = n / 2;
    (0..pairs)
        .map(|i| ScheduledMatch {
            gameweek: start_gameweek,
            home_team: teams[i].clone(),
            away_team: teams[n - 1 - i].clone(),
            played: false,
        })
        .collect()
}

pub fn normalize_team_name(name: &str) -> String {
    name.trim().to_uppercase()
}

pub fn find_team_index(table: &Table, team_name: &str) -> Option<usize> {
    let needle = normalize_team_name(team_name);
    table
        .iter()
        .position(|team| normalize_team_name(&team.club) == needle)
}

pub fn sort_table(table: &mut Table) {
    table.sort_by(|a, b| {
        b.pts
            .cmp(&a.pts)
            .then_with(|| b.gd.cmp(&a.gd))
            .then_with(|| b.w.cmp(&a.w))
            .then_with(|| a.club.cmp(&b.club))
    });
}

pub fn recalculate_positions(table: &mut Table) {
    for (index, team) in table.iter_mut().enumerate() {
        team.pos = index + 1;
    }
}

pub fn format_table_monospace(table: &Table) -> String {
    // Keep lines narrow (~30 chars) and strictly ASCII so the table renders
    // identically on Discord mobile and desktop without wrapping. Emoji are
    // double-width and break alignment inside code blocks, so they are avoided.
    const CLUB_WIDTH: usize = 8;

    fn row(pos: &str, club: &str, pl: &str, w: &str, d: &str, l: &str, gd: &str, pts: &str) -> String {
        format!(
            "{:>2} {:<width$} {:>2} {:>2} {:>2} {:>2} {:>3} {:>3}\n",
            pos, club, pl, w, d, l, gd, pts,
            width = CLUB_WIDTH,
        )
    }

    let mut output = String::new();
    output.push_str(&row("#", "CLUB", "PL", "W", "D", "L", "GD", "PTS"));

    for team in table {
        // Truncate long club names so columns stay aligned.
        let club = if team.club.chars().count() > CLUB_WIDTH {
            let truncated: String = team.club.chars().take(CLUB_WIDTH - 1).collect();
            format!("{}…", truncated)
        } else {
            team.club.clone()
        };

        output.push_str(&row(
            &team.pos.to_string(),
            &club,
            &team.pl.to_string(),
            &team.w.to_string(),
            &team.d.to_string(),
            &team.l.to_string(),
            &format!("{:+}", team.gd),
            &team.pts.to_string(),
        ));
    }

    output.trim_end().to_string()
}
