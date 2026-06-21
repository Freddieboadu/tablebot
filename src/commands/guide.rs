use poise::serenity_prelude as serenity;

use crate::{Context, Error};

/// Step-by-step guide on how to use the PBL bot.
#[poise::command(slash_command)]
pub async fn guide(ctx: Context<'_>) -> Result<(), Error> {
    let embed = serenity::CreateEmbed::new()
        .title("📖 PBL Bot — How to Use")
        .field(
            "1️⃣ View the table & results",
            "`/table` — Current standings\n\
             `/fixtures` — Last 10 results\n\
             `/form team_name:X` — Team's last 5 results\n\
             `/head2head team1:X team2:Y` — H2H record\n\
             `/website` — Live web table",
            false,
        )
        .field(
            "2️⃣ View the schedule",
            "`/schedule` — List all schedules\n\
             `/schedule name:Season 1` — Next unplayed gameweek\n\
             `/schedule name:Season 1 gameweek:3` — Specific gameweek\n\
             `/schedule name:Season 1 team:CHELSEA` — Your team's fixtures with W/L/D",
            false,
        )
        .field(
            "3️⃣ Predict & scenarios",
            "`/predict` — Projected final standings (uses PPG)\n\
             `/predict schedule_name:Season 1 team:CHELSEA target_position:1`\n\
             → Shows minimum wins Chelsea needs to finish 1st",
            false,
        )
        .field(
            "4️⃣ Admin — setting up a season",
            "`/addteam teams:CHELSEA, PSG, BARCA` — Add teams\n\
             `/generateschedule name:Season 1` — Full home & away schedule\n\
             `/generateschedule name:Group A schedule_type:single teams:CHELSEA,PSG` — Group stage\n\
             `/generateschedule name:Knockout schedule_type:knockout` — Bracket\n\
             `/update home_team:X home_score:N away_team:Y away_score:N` — Enter result",
            false,
        )
        .field(
            "5️⃣ Admin — fixes",
            "`/revert` — Undo last table change\n\
             `/deleteteam teams:X` — Remove a team\n\
             `/cleartable` — Wipe the table (new season)",
            false,
        )
        .footer(serenity::CreateEmbedFooter::new(
            "Type /help for a full command list · Results auto-update the schedule",
        ))
        .color(serenity::Color::GOLD);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
