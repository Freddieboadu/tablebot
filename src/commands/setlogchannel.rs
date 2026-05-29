use poise::serenity_prelude as serenity;

use crate::utils::config::{load_config, save_config};
use crate::{Context, Error};

#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn setlogchannel(
    ctx: Context<'_>,
    #[description = "The channel where all table changes will be posted"]
    channel: serenity::Channel,
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

    let channel_id = channel.id().get();
    let channel_name = channel
        .guild()
        .map(|c| c.name.clone())
        .unwrap_or_else(|| "the selected channel".to_string());

    let mut config = load_config(&guild_id)?;
    config.log_channel_id = Some(channel_id);
    save_config(&guild_id, &config)?;

    let embed = serenity::CreateEmbed::new()
        .title("✅ Log Channel Set")
        .description(format!(
            "All league changes will now be posted to **#{}**.",
            channel_name
        ))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
