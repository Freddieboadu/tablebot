use poise::serenity_prelude as serenity;

use crate::utils::config::{load_config, save_config};
use crate::{Context, Error};

#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn setadminrole(
    ctx: Context<'_>,
    #[description = "The role that is allowed to manage the league table"] role: serenity::Role,
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

    let mut config = load_config(&guild_id)?;
    config.admin_role_id = Some(role.id.get());
    save_config(&guild_id, &config)?;

    let embed = serenity::CreateEmbed::new()
        .title("✅ Admin Role Set")
        .description(format!(
            "Only members with the **{}** role (or Manage Server permission) can now modify the league table.",
            role.name
        ))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
