use poise::serenity_prelude as serenity;

use crate::utils::history::save_settings;
use crate::{Context, Error};

#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn setadminrole(
    ctx: Context<'_>,
    #[description = "Role to grant admin access to bot commands"] role: serenity::Role,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.get(),
        None => {
            ctx.send(poise::CreateReply::default().content("This command must be used in a server.").ephemeral(true)).await?;
            return Ok(());
        }
    };

    let guild_lock = ctx.data().get_guild(guild_id).await;

    let settings_to_save = {
        let mut guild = guild_lock.lock().await;
        guild.settings.admin_role_id = Some(role.id.get());
        guild.settings.clone()
    };

    save_settings(guild_id, &settings_to_save)?;

    let embed = serenity::CreateEmbed::new()
        .title("Admin Role Set")
        .description(format!("<@&{}> can now use admin bot commands.", role.id.get()))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
