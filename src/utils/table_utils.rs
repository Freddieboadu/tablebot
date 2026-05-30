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
