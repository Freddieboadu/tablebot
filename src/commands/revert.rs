use poise::serenity_prelude as serenity;

use crate::utils::history::{
    load_history, pop_snapshot, save_history, save_table, table_exists, FRESH_TABLE_MESSAGE,
    GUILD_ONLY_MESSAGE,
};
use crate::{Context, Error};

#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn revert(ctx: Context<'_>) -> Result<(), Error> {
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
    if is_new_server {
        ctx.send(poise::CreateReply::default().content(FRESH_TABLE_MESSAGE))
            .await?;
    }

    let mut history = load_history(&guild_id)?;

    let Some(previous_state) = pop_snapshot(&mut history) else {
        ctx.send(
            poise::CreateReply::default()
                .content("No history entries available to revert.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    };

    save_table(&guild_id, &previous_state)?;
    save_history(&guild_id, &history)?;

    let embed = serenity::CreateEmbed::new()
        .title("Revert Complete")
        .description("The previous league table state has been restored.")
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
