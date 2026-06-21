use poise::serenity_prelude as serenity;

use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let embed = serenity::CreateEmbed::new()
        .title("PBL Bot — Command List")
        .field(
            "📊 Everyone",
            "`/table` — Show the league table\n\
             `/form team_name:X` — Team's last 5 results\n\
             `/fixtures` — Last 10 match results\n\
             `/head2head team1:X team2:Y` — H2H record\n\
             `/schedule` — List all schedules or view fixtures\n\
             `/predict` — Project final standings\n\
             `/predict team:X target_position:N` — What does X need to finish Nth?\n\
             `/website` — Get the live web table link\n\
             `/help` — Show this message",
            false,
        )
        .field(
            "🔐 Admin only",
            "`/update home_team:X home_score:N away_team:Y away_score:N` — Enter a result\n\
             `/revert` — Undo last change\n\
             `/addteam teams:X, Y, Z` — Add teams (comma-separated)\n\
             `/deleteteam teams:X, Y` — Delete teams (comma-separated)\n\
             `/cleartable` — Wipe the entire table\n\
             `/generateschedule name:X` — Generate a season / group / knockout schedule\n\
             `/setadminrole role:@Role` — Set who can edit the table\n\
             `/setlogchannel channel:#channel` — Set log channel",
            false,
        )
        .field(
            "📅 Schedule types for /generateschedule",
            "`roundrobin` — Full home & away season (default)\n\
             `single` — Each pair plays once (group stage / preseason)\n\
             `knockout` — Single-elimination bracket (re-run to advance rounds)",
            false,
        )
        .color(serenity::Color::GOLD);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
