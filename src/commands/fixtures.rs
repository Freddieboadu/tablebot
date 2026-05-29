use poise::serenity_prelude as serenity;

use crate::utils::fixtures::load_fixtures;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn fixtures(ctx: Context<'_>) -> Result<(), Error> {
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

    let all_fixtures = load_fixtures(&guild_id)?;

    if all_fixtures.is_empty() {
        let embed = serenity::CreateEmbed::new()
            .title("📋 Recent Results")
            .description("No results have been entered yet!")
            .color(serenity::Color::BLUE);
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    // Show the last 10, most recent first.
    let recent: Vec<String> = all_fixtures
        .iter()
        .rev()
        .take(10)
        .map(|f| {
            format!(
                "**{}** {} - {} **{}**",
                f.home, f.home_score, f.away_score, f.away
            )
        })
        .collect();

    let embed = serenity::CreateEmbed::new()
        .title("📋 Recent Results")
        .description(recent.join("\n"))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
