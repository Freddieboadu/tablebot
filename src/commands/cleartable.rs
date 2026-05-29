use std::time::Duration;

use poise::serenity_prelude as serenity;

use crate::utils::history::{
    load_history, load_table, push_snapshot, save_history, save_table, HISTORY_LIMIT,
};
use crate::utils::permissions::{is_admin, post_to_log_channel};
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn cleartable(ctx: Context<'_>) -> Result<(), Error> {
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

    let confirm_id = format!("confirm_clear:{}", ctx.author().id);
    let cancel_id = format!("cancel_clear:{}", ctx.author().id);

    ctx.send(
        poise::CreateReply::default()
            .embed(
                serenity::CreateEmbed::new()
                    .title("⚠️ Clear Table")
                    .description(
                        "This will **wipe the entire league table** and start fresh.\n\
                        The current table will be saved so you can undo this with `/revert`.\n\n\
                        Are you sure?",
                    )
                    .color(serenity::Color::ORANGE),
            )
            .components(vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new(confirm_id.clone())
                    .label("Yes, clear it")
                    .style(serenity::ButtonStyle::Danger),
                serenity::CreateButton::new(cancel_id.clone())
                    .label("No, cancel")
                    .style(serenity::ButtonStyle::Secondary),
            ])])
            .ephemeral(true),
    )
    .await?;

    let interaction = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(Duration::from_secs(30))
        .await;

    let Some(interaction) = interaction else {
        ctx.send(
            poise::CreateReply::default()
                .content("⌛ Request timed out — table was not cleared.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    };

    if interaction.data.custom_id == cancel_id {
        interaction
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new()
                        .content("❎ Clear cancelled — table is unchanged.")
                        .embeds(vec![])
                        .components(vec![]),
                ),
            )
            .await?;
        return Ok(());
    }

    if interaction.data.custom_id != confirm_id {
        interaction
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new()
                        .content("❎ Unknown response — table is unchanged.")
                        .embeds(vec![])
                        .components(vec![]),
                ),
            )
            .await?;
        return Ok(());
    }

    // Save current state to history before clearing.
    let current_table = load_table(&guild_id)?;
    let mut history = load_history(&guild_id)?;
    push_snapshot(&mut history, current_table, HISTORY_LIMIT);
    save_history(&guild_id, &history)?;
    save_table(&guild_id, &[])?;

    interaction
        .create_response(
            ctx.serenity_context(),
            serenity::CreateInteractionResponse::UpdateMessage(
                serenity::CreateInteractionResponseMessage::new()
                    .embeds(vec![serenity::CreateEmbed::new()
                        .title("✅ Table Cleared!")
                        .description(
                            "The league table has been wiped.\n\
                            Use `/addteam` to add your teams, or `/revert` to undo this.",
                        )
                        .color(serenity::Color::BLUE)])
                    .components(vec![]),
            ),
        )
        .await?;

    let log_msg = format!(
        "🗑️ **{}** used `/cleartable` — table wiped",
        ctx.author().name
    );
    post_to_log_channel(&ctx, &guild_id, &log_msg).await;

    Ok(())
}
