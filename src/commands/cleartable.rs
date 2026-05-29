use std::time::Duration;

use poise::serenity_prelude as serenity;

use crate::utils::history::{push_snapshot, save_history, save_table, HISTORY_LIMIT};
use crate::utils::permissions::check_admin;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn cleartable(ctx: Context<'_>) -> Result<(), Error> {
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

    let confirm_id = format!("confirm_clear:{}", ctx.author().id);
    let cancel_id = format!("cancel_clear:{}", ctx.author().id);

    let prompt = poise::CreateReply::default()
        .content("Are you sure you want to clear the entire league table?")
        .components(vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(confirm_id.clone())
                .label("Yes")
                .style(serenity::ButtonStyle::Success),
            serenity::CreateButton::new(cancel_id.clone())
                .label("No")
                .style(serenity::ButtonStyle::Danger),
        ])])
        .ephemeral(true);

    ctx.send(prompt).await?;

    let interaction = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(Duration::from_secs(30))
        .await;

    let Some(interaction) = interaction else {
        ctx.send(poise::CreateReply::default().content("Clear request timed out.").ephemeral(true)).await?;
        return Ok(());
    };

    if interaction.data.custom_id == cancel_id {
        interaction
            .create_response(ctx.serenity_context(), serenity::CreateInteractionResponse::UpdateMessage(
                serenity::CreateInteractionResponseMessage::new().content("Clear cancelled.").components(vec![]),
            ))
            .await?;
        return Ok(());
    }

    if interaction.data.custom_id != confirm_id {
        interaction
            .create_response(ctx.serenity_context(), serenity::CreateInteractionResponse::UpdateMessage(
                serenity::CreateInteractionResponseMessage::new().content("Unknown response; clear cancelled.").components(vec![]),
            ))
            .await?;
        return Ok(());
    }

    let (table_to_save, history_to_save) = {
        let mut guild = guild_lock.lock().await;
        let snapshot = guild.table.clone();
        push_snapshot(&mut guild.history, snapshot, HISTORY_LIMIT);
        guild.table.clear();
        (guild.table.clone(), guild.history.clone())
    };

    save_table(guild_id, &table_to_save)?;
    save_history(guild_id, &history_to_save)?;

    interaction
        .create_response(ctx.serenity_context(), serenity::CreateInteractionResponse::UpdateMessage(
            serenity::CreateInteractionResponseMessage::new()
                .content("League table has been cleared. Use `/addteam` to start fresh.")
                .components(vec![]),
        ))
        .await?;

    Ok(())
}

