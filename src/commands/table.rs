use poise::serenity_prelude as serenity;

<<<<<<< HEAD
use crate::utils::history::load_table;
=======
use crate::utils::history::{load_table, table_exists, FRESH_TABLE_MESSAGE, GUILD_ONLY_MESSAGE};
>>>>>>> origin/copilot/rebuild-league-table-bot
use crate::utils::table_utils::{format_table_monospace, recalculate_positions, sort_table};
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn table(ctx: Context<'_>) -> Result<(), Error> {
<<<<<<< HEAD
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

=======
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
>>>>>>> origin/copilot/rebuild-league-table-bot
    let mut table_snapshot = load_table(&guild_id)?;
    sort_table(&mut table_snapshot);
    recalculate_positions(&mut table_snapshot);

<<<<<<< HEAD
    let description = if table_snapshot.is_empty() {
        "No teams have been added yet! Use `/addteam` to get started.".to_string()
    } else {
        format!("```\n{}\n```", format_table_monospace(&table_snapshot))
    };

=======
    if is_new_server {
        ctx.send(poise::CreateReply::default().content(FRESH_TABLE_MESSAGE))
            .await?;
    }

    let formatted = format_table_monospace(&table_snapshot);
>>>>>>> origin/copilot/rebuild-league-table-bot
    let embed = serenity::CreateEmbed::new()
        .title("📊 PBL League Table")
        .description(description)
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
