use std::time::Duration;

use poise::serenity_prelude as serenity;

use crate::utils::history::{
    load_history, load_table, push_snapshot, save_history, save_table, HISTORY_LIMIT,
};
use crate::utils::permissions::{is_admin, post_to_log_channel};
use crate::utils::table_utils::{
    find_team_index, normalize_team_name, recalculate_positions, sort_table,
};
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
pub async fn deleteteam(
    ctx: Context<'_>,
    #[description = "Team name(s) to delete, separated by commas — e.g. Chelsea, PSG"]
    #[autocomplete = "autocomplete_team"]
    teams: String,
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

    if !is_admin(&ctx).await? {
        ctx.send(
            poise::CreateReply::default()
                .content("❌ You need the league admin role to use this command!")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let table = load_table(&guild_id)?;

    // Determine which requested teams exist.
    let mut to_delete: Vec<String> = Vec::new();
    let mut not_found: Vec<String> = Vec::new();

    for raw_name in teams.split(',') {
        let name = normalize_team_name(raw_name);
        if name.is_empty() {
            continue;
        }
        if find_team_index(&table, &name).is_some() {
            to_delete.push(name);
        } else {
            not_found.push(name);
        }
    }

    if to_delete.is_empty() {
        let msg = if not_found.is_empty() {
            "❌ No team names were provided.".to_string()
        } else {
            format!(
                "❌ None of those teams were found: {}",
                not_found.join(", ")
            )
        };
        ctx.send(poise::CreateReply::default().content(msg).ephemeral(true))
            .await?;
        return Ok(());
    }

    // Confirmation prompt.
    let confirm_id = format!("confirm_delete:{}", ctx.author().id);
    let cancel_id = format!("cancel_delete:{}", ctx.author().id);

    let mut prompt_desc = format!(
        "You are about to delete: **{}**\n\nAre you sure?",
        to_delete.join(", ")
    );
    if !not_found.is_empty() {
        prompt_desc.push_str(&format!(
            "\n\n⚠️ Not found (will be skipped): {}",
            not_found.join(", ")
        ));
    }

    ctx.send(
        poise::CreateReply::default()
            .embed(
                serenity::CreateEmbed::new()
                    .title("🗑️ Confirm Delete")
                    .description(prompt_desc)
                    .color(serenity::Color::RED),
            )
            .components(vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new(confirm_id.clone())
                    .label("Yes, delete")
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
                .content("⌛ Request timed out — nothing was deleted.")
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
                        .content("❎ Delete cancelled.")
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
                        .content("❎ Unknown response — delete cancelled.")
                        .embeds(vec![])
                        .components(vec![]),
                ),
            )
            .await?;
        return Ok(());
    }

    // Perform deletion.
    let mut table = load_table(&guild_id)?;
    let mut history = load_history(&guild_id)?;
    push_snapshot(&mut history, table.clone(), HISTORY_LIMIT);

    let mut actually_deleted: Vec<String> = Vec::new();
    for name in &to_delete {
        if let Some(index) = find_team_index(&table, name) {
            table.remove(index);
            actually_deleted.push(name.clone());
        }
    }

    sort_table(&mut table);
    recalculate_positions(&mut table);
    save_table(&guild_id, &table)?;
    save_history(&guild_id, &history)?;

    let mut result_desc = format!("✅ **Deleted:** {}", actually_deleted.join(", "));
    if !not_found.is_empty() {
        result_desc.push_str(&format!("\n⚠️ **Not found:** {}", not_found.join(", ")));
    }

    interaction
        .create_response(
            ctx.serenity_context(),
            serenity::CreateInteractionResponse::UpdateMessage(
                serenity::CreateInteractionResponseMessage::new()
                    .embeds(vec![serenity::CreateEmbed::new()
                        .title("🗑️ Teams Deleted")
                        .description(result_desc)
                        .color(serenity::Color::RED)])
                    .components(vec![]),
            ),
        )
        .await?;

    let log_msg = format!(
        "🗑️ **{}** used `/deleteteam`: removed {}",
        ctx.author().name,
        actually_deleted.join(", ")
    );
    post_to_log_channel(&ctx, &guild_id, &log_msg).await;

    Ok(())
}
