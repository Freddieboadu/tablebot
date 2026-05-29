use poise::serenity_prelude as serenity;

use crate::utils::table_utils::{find_team_index, normalize_team_name};
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn form(
    ctx: Context<'_>,
    #[description = "Team name"] team_name: String,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.get(),
        None => {
            ctx.send(poise::CreateReply::default().content("This command must be used in a server.").ephemeral(true)).await?;
            return Ok(());
        }
    };

    let team_key = normalize_team_name(&team_name);
    let guild_lock = ctx.data().get_guild(guild_id).await;
    let guild = guild_lock.lock().await;

    if find_team_index(&guild.table, &team_key).is_none() {
        ctx.send(poise::CreateReply::default().content(format!("Team '{}' was not found.", team_key)).ephemeral(true)).await?;
        return Ok(());
    }

    let relevant: Vec<_> = guild
        .fixtures
        .iter()
        .filter(|f| f.home_team == team_key || f.away_team == team_key)
        .collect();

    if relevant.is_empty() {
        ctx.send(poise::CreateReply::default().content(format!("No results found for **{}**.", team_key)).ephemeral(true)).await?;
        return Ok(());
    }

    let start = relevant.len().saturating_sub(5);
    let last5 = &relevant[start..];

    let form: Vec<&str> = last5
        .iter()
        .map(|f| {
            if f.home_score == f.away_score {
                "🟡 D"
            } else if (f.home_team == team_key && f.home_score > f.away_score)
                || (f.away_team == team_key && f.away_score > f.home_score)
            {
                "🟢 W"
            } else {
                "🔴 L"
            }
        })
        .collect();

    let embed = serenity::CreateEmbed::new()
        .title(format!("{} — Last {} Results", team_key, last5.len()))
        .description(form.join("  "))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
