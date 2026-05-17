use poise::serenity_prelude as serenity;

use crate::utils::table_utils::{format_table_monospace, recalculate_positions, sort_table};
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn table(ctx: Context<'_>) -> Result<(), Error> {
    let mut table_snapshot = ctx.data().table.lock().await.clone();
    sort_table(&mut table_snapshot);
    recalculate_positions(&mut table_snapshot);

    let formatted = format_table_monospace(&table_snapshot);
    let embed = serenity::CreateEmbed::new()
        .title("PBL League Table")
        .description(format!("```\n{}\n```", formatted))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
