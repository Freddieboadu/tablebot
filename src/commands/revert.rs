use poise::serenity_prelude as serenity;

use crate::utils::history::{pop_snapshot, save_history, save_table};
use crate::utils::permissions::check_admin;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn revert(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.get(),
        None => {
            ctx.send(poise::CreateReply::default().content("This command must be used in a server.").ephemeral(true)).await?;
            return Ok(());
        }
    };

    let guild_lock = ctx.data().get_guild(guild_id).await;

    let settings = guild_lock.lock().await.settings.clone();
    if !check_admin(ctx, &settings).await {
        ctx.send(poise::CreateReply::default().content("You don't have permission to use this command.").ephemeral(true)).await?;
        return Ok(());
    }

    let maybe_result = {
        let mut guild = guild_lock.lock().await;
        pop_snapshot(&mut guild.history).map(|previous_state| {
            guild.table = previous_state;
            (guild.table.clone(), guild.history.clone(), guild.settings.log_channel_id)
        })
    };

    let Some((table_to_save, history_to_save, log_channel)) = maybe_result else {
        ctx.send(poise::CreateReply::default().content("No history entries available to revert.").ephemeral(true)).await?;
        return Ok(());
    };

    save_table(guild_id, &table_to_save)?;
    save_history(guild_id, &history_to_save)?;

    let embed = serenity::CreateEmbed::new()
        .title("Revert Complete")
        .description("The previous league table state has been restored.")
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    if let Some(channel_id) = log_channel {
        let _ = serenity::ChannelId::new(channel_id)
            .say(ctx.serenity_context(), "↩️ Table reverted to previous state.")
            .await;
    }

    Ok(())
}

