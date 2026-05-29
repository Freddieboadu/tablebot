use poise::serenity_prelude as serenity;

use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn fixtures(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.get(),
        None => {
            ctx.send(poise::CreateReply::default().content("This command must be used in a server.").ephemeral(true)).await?;
            return Ok(());
        }
    };

    let guild_lock = ctx.data().get_guild(guild_id).await;
    let guild = guild_lock.lock().await;

    if guild.fixtures.is_empty() {
        ctx.send(poise::CreateReply::default().content("No results have been entered yet.").ephemeral(true)).await?;
        return Ok(());
    }

    let total = guild.fixtures.len();
    let start = total.saturating_sub(10);
    let last10 = &guild.fixtures[start..];

    let lines: Vec<String> = last10
        .iter()
        .map(|f| format!("**{}** {}-{} **{}**", f.home_team, f.home_score, f.away_score, f.away_team))
        .collect();

    let embed = serenity::CreateEmbed::new()
        .title("Recent Results")
        .description(lines.join("\n"))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
