use std::time::Duration;

use poise::serenity_prelude as serenity;

use crate::utils::history::{push_snapshot, save_history, save_table, HISTORY_LIMIT};
use crate::utils::table_utils::{
    find_team_index, normalize_team_name, recalculate_positions, sort_table,
};
use crate::{Context, Error};

#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn deleteteam(
    ctx: Context<'_>,
    #[description = "Team name"] team_name: String,
) -> Result<(), Error> {
    let normalized_name = normalize_team_name(&team_name);
    {
        let table = ctx.data().table.lock().await;
        if find_team_index(&table, &normalized_name).is_none() {
            ctx.send(
                poise::CreateReply::default()
                    .content(format!("Team '{}' was not found.", normalized_name))
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    }

    let confirm_id = format!("confirm_delete:{}:{}", ctx.author().id, normalized_name);
    let cancel_id = format!("cancel_delete:{}:{}", ctx.author().id, normalized_name);

    let prompt = poise::CreateReply::default()
        .content(format!("Delete {} from the league table?", normalized_name))
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
        ctx.send(
            poise::CreateReply::default()
                .content("Delete request timed out.")
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
                        .content("Delete cancelled.")
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
                        .content("Unknown response; delete cancelled.")
                        .components(vec![]),
                ),
            )
            .await?;
        return Ok(());
    }

    let (table_to_save, history_to_save) = {
        let mut table = ctx.data().table.lock().await;
        let mut history = ctx.data().history.lock().await;

        push_snapshot(&mut history, table.clone(), HISTORY_LIMIT);

        let Some(index) = find_team_index(&table, &normalized_name) else {
            interaction
                .create_response(
                    ctx.serenity_context(),
                    serenity::CreateInteractionResponse::UpdateMessage(
                        serenity::CreateInteractionResponseMessage::new()
                            .content("Team no longer exists; nothing to delete.")
                            .components(vec![]),
                    ),
                )
                .await?;
            return Ok(());
        };

        table.remove(index);
        sort_table(&mut table);
        recalculate_positions(&mut table);

        (table.clone(), history.clone())
    };

    save_table(&table_to_save)?;
    save_history(&history_to_save)?;

    interaction
        .create_response(
            ctx.serenity_context(),
            serenity::CreateInteractionResponse::UpdateMessage(
                serenity::CreateInteractionResponseMessage::new()
                    .content(format!("{} has been deleted.", normalized_name))
                    .components(vec![]),
            ),
        )
        .await?;

    Ok(())
}
