use poise::serenity_prelude as serenity;

use crate::utils::history::{push_snapshot, save_history, save_table, HISTORY_LIMIT};
use crate::utils::permissions::check_admin;
use crate::utils::table_utils::{normalize_team_name, recalculate_positions, sort_table, Team};
use crate::utils::validator::validate_new_team_name;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn addteam(
    ctx: Context<'_>,
    #[description = "Team name(s), comma-separated"] teams: String,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.get(),
        None => {
            ctx.send(poise::CreateReply::default().content("This command must be used in a server.").ephemeral(true)).await?;
            return Ok(());
        }
    };

    let guild_lock = ctx.data().get_guild(guild_id).await;

    let settings = guild_lock.lock().await.settings.clone();
    if !check_admin(ctx, &settings).await {
        ctx.send(poise::CreateReply::default().content("You don't have permission to use this command.").ephemeral(true)).await?;
        return Ok(());
    }

    let names: Vec<String> = teams
        .split(',')
        .map(|s| normalize_team_name(s))
        .filter(|s| !s.is_empty())
        .collect();

    if names.is_empty() {
        ctx.send(poise::CreateReply::default().content("No valid team names provided.").ephemeral(true)).await?;
        return Ok(());
    }

    let (table_to_save, history_to_save, added) = {
        let mut guild = guild_lock.lock().await;

        // Validate all before adding any
        for name in &names {
            validate_new_team_name(&guild.table, name)?;
        }

        let snapshot = guild.table.clone();
        push_snapshot(&mut guild.history, snapshot, HISTORY_LIMIT);

        for name in &names {
            let next_pos = guild.table.len() + 1;
            guild.table.push(Team {
                pos: next_pos,
                club: name.clone(),
                pl: 0, w: 0, d: 0, l: 0, gd: 0, pts: 0,
            });
        }

        sort_table(&mut guild.table);
        recalculate_positions(&mut guild.table);
        (guild.table.clone(), guild.history.clone(), names.clone())
    };

    save_table(guild_id, &table_to_save)?;
    save_history(guild_id, &history_to_save)?;

    let list = added.join(", ");
    let embed = serenity::CreateEmbed::new()
        .title("Team(s) Added")
        .description(format!("{} added to the league table.", list))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

