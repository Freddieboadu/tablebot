use poise::serenity_prelude as serenity;

use crate::utils::config::load_config;
use crate::{Context, Error};

/// Returns `true` if the command author has permission to run admin commands.
/// Checks: MANAGE_GUILD permission OR the configured admin role.
pub async fn is_admin(ctx: &Context<'_>) -> Result<bool, Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.to_string(),
        None => return Ok(false),
    };

    // Check MANAGE_GUILD permission (set by Discord on interaction member objects).
    if let Some(member) = ctx.author_member().await {
        if let Some(perms) = member.permissions {
            if perms.manage_guild() {
                return Ok(true);
            }
        }
    }

    // Check configured admin role.
    let config = load_config(&guild_id)?;
    if let Some(role_id) = config.admin_role_id {
        if let Some(member) = ctx.author_member().await {
            if member.roles.contains(&serenity::RoleId::new(role_id)) {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

/// If a log channel is configured for this guild, post a message there.
/// Failures are silently ignored so a missing-channel never breaks a command.
pub async fn post_to_log_channel(ctx: &Context<'_>, guild_id: &str, message: &str) {
    if let Ok(config) = load_config(guild_id) {
        if let Some(channel_id) = config.log_channel_id {
            let ch = serenity::ChannelId::new(channel_id);
            let _ = ch.say(&ctx.serenity_context().http, message).await;
        }
    }
}
