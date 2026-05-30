use std::env;

use poise::serenity_prelude as serenity;

use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn website(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.get(),
        None => {
            ctx.send(poise::CreateReply::default().content("This command must be used in a server.").ephemeral(true)).await?;
            return Ok(());
        }
    };

    // Prefer an explicit public base URL (set this to your tunnel/domain).
    // Falls back to localhost using the configured WEB_PORT for local testing.
    let base = env::var("WEB_BASE_URL").unwrap_or_else(|_| {
        let port = env::var("WEB_PORT").unwrap_or_else(|_| "8080".to_string());
        format!("http://localhost:{port}")
    });
    let base = base.trim_end_matches('/');
    let url = format!("{base}/g/{guild_id}");

    let embed = serenity::CreateEmbed::new()
        .title("🌐 Live League Table")
        .description(format!(
            "View this server's league table on the web:\n**[Open table]({url})**\n\nThe page updates automatically as results are entered."
        ))
        .url(&url)
        .color(serenity::Color::new(0x37003c));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
