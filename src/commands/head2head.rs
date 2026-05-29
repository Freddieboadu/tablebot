use poise::serenity_prelude as serenity;

use crate::utils::table_utils::{find_team_index, normalize_team_name};
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn head2head(
    ctx: Context<'_>,
    #[description = "First team"] team1: String,
    #[description = "Second team"] team2: String,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.get(),
        None => {
            ctx.send(poise::CreateReply::default().content("This command must be used in a server.").ephemeral(true)).await?;
            return Ok(());
        }
    };

    let t1 = normalize_team_name(&team1);
    let t2 = normalize_team_name(&team2);

    if t1 == t2 {
        ctx.send(poise::CreateReply::default().content("Teams must be different.").ephemeral(true)).await?;
        return Ok(());
    }

    let guild_lock = ctx.data().get_guild(guild_id).await;
    let guild = guild_lock.lock().await;

    if find_team_index(&guild.table, &t1).is_none() {
        ctx.send(poise::CreateReply::default().content(format!("Team '{}' was not found.", t1)).ephemeral(true)).await?;
        return Ok(());
    }
    if find_team_index(&guild.table, &t2).is_none() {
        ctx.send(poise::CreateReply::default().content(format!("Team '{}' was not found.", t2)).ephemeral(true)).await?;
        return Ok(());
    }

    let matches: Vec<_> = guild
        .fixtures
        .iter()
        .filter(|f| {
            (f.home_team == t1 && f.away_team == t2)
                || (f.home_team == t2 && f.away_team == t1)
        })
        .collect();

    if matches.is_empty() {
        ctx.send(poise::CreateReply::default().content(format!("No head-to-head results found between **{}** and **{}**.", t1, t2)).ephemeral(true)).await?;
        return Ok(());
    }

    let mut t1_wins = 0;
    let mut t2_wins = 0;
    let mut draws = 0;

    let lines: Vec<String> = matches
        .iter()
        .map(|f| {
            if f.home_score == f.away_score {
                draws += 1;
            } else if (f.home_team == t1 && f.home_score > f.away_score)
                || (f.away_team == t1 && f.away_score > f.home_score)
            {
                t1_wins += 1;
            } else {
                t2_wins += 1;
            }
            format!("**{}** {}-{} **{}**", f.home_team, f.home_score, f.away_score, f.away_team)
        })
        .collect();

    let summary = format!(
        "**{}** {} — {} draws — {} **{}**\n\n{}",
        t1, t1_wins, draws, t2_wins, t2,
        lines.join("\n")
    );

    let embed = serenity::CreateEmbed::new()
        .title(format!("{} vs {}", t1, t2))
        .description(summary)
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
