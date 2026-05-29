use poise::serenity_prelude as serenity;

use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let embed = serenity::CreateEmbed::new()
        .title("ℹ️ PBL Table Bot — Help")
        .description("Here's everything this bot can do. All commands start with `/`.")
        .field(
            "📊 Table Commands",
            "`/table` — Show the current league standings\n\
             `/form <team>` — Show a team's last 5 results (W/D/L)\n\
             `/fixtures` — Show the last 10 match results entered\n\
             `/head2head <team1> <team2>` — Show all results between two teams",
            false,
        )
        .field(
            "⚽ Match Commands",
            "`/update` — Enter a match result to update the league table\n\
             `/revert` — Undo the last change made to the table",
            false,
        )
        .field(
            "👥 Team Commands",
            "`/addteam` — Add one or more teams, separated by commas\n\
             `/deleteteam` — Remove one or more teams, separated by commas\n\
             `/cleartable` — Wipe the table and start fresh (admin only)",
            false,
        )
        .field(
            "🔒 Admin Setup",
            "`/setadminrole` — Choose which role is allowed to edit the table\n\
             `/setlogchannel` — Choose a channel to log all changes to",
            false,
        )
        .field("ℹ️ Info", "`/help` — Show this help message", false)
        .footer(serenity::CreateEmbedFooter::new(
            "Admin commands need Manage Server permission OR the configured admin role.",
        ))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
