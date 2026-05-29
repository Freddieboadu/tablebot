use poise::serenity_prelude as serenity;

use crate::utils::history::load_table;
use crate::utils::table_utils::{format_table_monospace, recalculate_positions, sort_table};
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn table(ctx: Context<'_>) -> Result<(), Error> {
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

    let mut table_snapshot = load_table(&guild_id)?;
    sort_table(&mut table_snapshot);
    recalculate_positions(&mut table_snapshot);

    let description = if table_snapshot.is_empty() {
        "No teams have been added yet! Use `/addteam` to get started.".to_string()
    } else {
        format!("```\n{}\n```", format_table_monospace(&table_snapshot))
    };

    let embed = serenity::CreateEmbed::new()
        .title("📊 PBL League Table")
        .description(description)
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
