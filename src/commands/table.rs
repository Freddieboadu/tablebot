use poise::serenity_prelude as serenity;

use crate::utils::history::{load_table, table_exists, FRESH_TABLE_MESSAGE, GUILD_ONLY_MESSAGE};
use crate::utils::table_utils::{format_table_monospace, recalculate_positions, sort_table};
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn table(ctx: Context<'_>) -> Result<(), Error> {
    let Some(guild_id) = ctx.guild_id().map(|id| id.to_string()) else {
        ctx.send(
            poise::CreateReply::default()
                .content(GUILD_ONLY_MESSAGE)
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    };

    let is_new_server = !table_exists(&guild_id);
    let mut table_snapshot = load_table(&guild_id)?;
    sort_table(&mut table_snapshot);
    recalculate_positions(&mut table_snapshot);

    if is_new_server {
        ctx.send(poise::CreateReply::default().content(FRESH_TABLE_MESSAGE))
            .await?;
    }

    let formatted = format_table_monospace(&table_snapshot);
    let embed = serenity::CreateEmbed::new()
        .title("PBL League Table")
        .description(format!("```\n{}\n```", formatted))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
