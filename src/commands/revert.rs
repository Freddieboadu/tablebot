use poise::serenity_prelude as serenity;

use crate::utils::history::{pop_snapshot, save_history, save_table};
use crate::{Context, Error};

#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn revert(ctx: Context<'_>) -> Result<(), Error> {
    let (restored_table, updated_history) = {
        let mut history = ctx.data().history.lock().await;
        let Some(previous_state) = pop_snapshot(&mut history) else {
            ctx.send(
                poise::CreateReply::default()
                    .content("No history entries available to revert.")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        };

        let mut table = ctx.data().table.lock().await;
        *table = previous_state;

        (table.clone(), history.clone())
    };

    save_table(&restored_table)?;
    save_history(&updated_history)?;

    let embed = serenity::CreateEmbed::new()
        .title("Revert Complete")
        .description("The previous league table state has been restored.")
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
