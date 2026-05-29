use poise::serenity_prelude as serenity;

use crate::utils::table_utils::{format_table_monospace, recalculate_positions, sort_table};
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn table(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.get(),
        None => {
            ctx.send(poise::CreateReply::default().content("This command must be used in a server.").ephemeral(true)).await?;
            return Ok(());
        }
    };

    let guild_lock = ctx.data().get_guild(guild_id).await;
    let mut table_snapshot = guild_lock.lock().await.table.clone();
    sort_table(&mut table_snapshot);
    recalculate_positions(&mut table_snapshot);

    if table_snapshot.is_empty() {
        ctx.send(poise::CreateReply::default().content("The league table is empty. Use `/addteam` to add teams.").ephemeral(true)).await?;
        return Ok(());
    }

    let formatted = format_table_monospace(&table_snapshot);
    let embed = serenity::CreateEmbed::new()
        .title("PBL League Table")
        .description(format!("```\n{}\n```", formatted))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
