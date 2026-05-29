use poise::serenity_prelude as serenity;

use crate::utils::fixtures::load_fixtures;
use crate::utils::history::load_table;
use crate::utils::table_utils::find_team_index;
use crate::{Context, Error};

async fn autocomplete_team(ctx: Context<'_>, partial: &str) -> Vec<String> {
    let guild_id = ctx.guild_id().map(|g| g.to_string()).unwrap_or_default();
    let table = crate::utils::history::load_table(&guild_id).unwrap_or_default();
    let lower = partial.to_lowercase();
    table
        .into_iter()
        .map(|t| t.club)
        .filter(|name| name.to_lowercase().starts_with(&lower))
        .collect()
}

#[poise::command(slash_command)]
pub async fn head2head(
    ctx: Context<'_>,
    #[description = "First team"]
    #[autocomplete = "autocomplete_team"]
    team1: String,
    #[description = "Second team"]
    #[autocomplete = "autocomplete_team"]
    team2: String,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.to_string(),
        None => {
            ctx.send(
                poise::CreateReply::default()
                    .content("❌ This command can only be used in a server.")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    let table = load_table(&guild_id)?;
    let t1 = team1.trim().to_uppercase();
    let t2 = team2.trim().to_uppercase();

    if find_team_index(&table, &t1).is_none() {
        ctx.send(
            poise::CreateReply::default()
                .content(format!("❌ Team **{}** is not in the league table.", t1))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    if find_team_index(&table, &t2).is_none() {
        ctx.send(
            poise::CreateReply::default()
                .content(format!("❌ Team **{}** is not in the league table.", t2))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    if t1 == t2 {
        ctx.send(
            poise::CreateReply::default()
                .content("❌ Please enter two different teams.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let all_fixtures = load_fixtures(&guild_id)?;

    // Filter to matches involving both teams (either way round).
    let h2h: Vec<_> = all_fixtures
        .iter()
        .filter(|f| (f.home == t1 && f.away == t2) || (f.home == t2 && f.away == t1))
        .collect();

    if h2h.is_empty() {
        let embed = serenity::CreateEmbed::new()
            .title(format!("🆚 {} vs {}", t1, t2))
            .description("No matches have been played between these teams yet.")
            .color(serenity::Color::BLUE);
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let mut t1_wins = 0u32;
    let mut t2_wins = 0u32;
    let mut draws = 0u32;

    let results: Vec<String> = h2h
        .iter()
        .map(|f| {
            let line = format!("{} {} - {} {}", f.home, f.home_score, f.away_score, f.away);
            // Tally
            if f.home == t1 {
                match f.home_score.cmp(&f.away_score) {
                    std::cmp::Ordering::Greater => t1_wins += 1,
                    std::cmp::Ordering::Less => t2_wins += 1,
                    std::cmp::Ordering::Equal => draws += 1,
                }
            } else {
                match f.away_score.cmp(&f.home_score) {
                    std::cmp::Ordering::Greater => t1_wins += 1,
                    std::cmp::Ordering::Less => t2_wins += 1,
                    std::cmp::Ordering::Equal => draws += 1,
                }
            }
            line
        })
        .collect();

    let summary = format!(
        "{} wins: {}\n{} wins: {}\nDraws: {}",
        t1, t1_wins, t2, t2_wins, draws
    );

    let embed = serenity::CreateEmbed::new()
        .title(format!("🆚 {} vs {}", t1, t2))
        .field("Match History", results.join("\n"), false)
        .field("Summary", summary, false)
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
