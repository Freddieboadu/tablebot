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
    let mut output = String::from("POS CLUB               PL  W  D  L   GD  PTS\n");

    for team in table {
        let pos_label = if team.pos == 1 {
            format!("🏆{}", team.pos)
        } else {
            team.pos.to_string()
        };

        output.push_str(&format!(
            "{:<3} {:<18} {:>2} {:>2} {:>2} {:>2} {:>4} {:>4}\n",
            pos_label, team.club, team.pl, team.w, team.d, team.l, team.gd, team.pts
        ));
    }

    output.trim_end().to_string()
}
