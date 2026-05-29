use poise::serenity_prelude as serenity;

use crate::utils::history::{load_history, pop_snapshot, save_history, save_table};
use crate::utils::permissions::{is_admin, post_to_log_channel};
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn revert(ctx: Context<'_>) -> Result<(), Error> {
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

    if !is_admin(&ctx).await? {
        ctx.send(
            poise::CreateReply::default()
                .content("❌ You need the league admin role to use this command!")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let mut history = load_history(&guild_id)?;
    let Some(previous_state) = pop_snapshot(&mut history) else {
        ctx.send(
            poise::CreateReply::default()
                .content("❌ There's nothing to undo — no previous state is saved.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    };

    save_table(&guild_id, &previous_state)?;
    save_history(&guild_id, &history)?;

    let embed = serenity::CreateEmbed::new()
        .title("↩️ Revert Complete")
        .description("The previous league table state has been restored.")
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    let log_msg = format!("↩️ **{}** used `/revert`", ctx.author().name);
    post_to_log_channel(&ctx, &guild_id, &log_msg).await;

    Ok(())
}
