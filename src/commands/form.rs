use poise::serenity_prelude as serenity;

use crate::utils::fixtures::{format_result_badge, load_fixtures, streak_description, team_form};
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
pub async fn form(
    ctx: Context<'_>,
    #[description = "Team name"]
    #[autocomplete = "autocomplete_team"]
    team: String,
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
    let team_upper = team.trim().to_uppercase();

    if find_team_index(&table, &team_upper).is_none() {
        ctx.send(
            poise::CreateReply::default()
                .content(format!(
                    "❌ Team **{}** is not in the league table.",
                    team_upper
                ))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let fixtures = load_fixtures(&guild_id)?;
    let form_results = team_form(&fixtures, &team_upper, 5);

    let form_display = if form_results.is_empty() {
        "No results recorded yet".to_string()
    } else {
        form_results
            .iter()
            .map(|&r| format_result_badge(r))
            .collect::<Vec<_>>()
            .join("  ")
    };

    let streak = streak_description(&form_results);

    let embed = serenity::CreateEmbed::new()
        .title(format!("📈 Form — {}", team_upper))
        .field("Last 5 Results", form_display, false)
        .field("Current Streak", streak, false)
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
