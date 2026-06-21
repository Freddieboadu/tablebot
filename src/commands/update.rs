use anyhow::Context as AnyhowContext;
use poise::serenity_prelude as serenity;

use crate::utils::history::{push_fixture, push_snapshot, save_fixtures, save_history, save_schedules, save_table, HISTORY_LIMIT};
use crate::utils::permissions::check_admin;
use crate::utils::table_utils::{find_team_index, normalize_team_name, recalculate_positions, sort_table, Fixture};
use crate::utils::validator::validate_match_input;
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn update(
    ctx: Context<'_>,
    #[description = "Home team name"] home_team: String,
    #[description = "Home team score"] home_score: i64,
    #[description = "Away team name"] away_team: String,
    #[description = "Away team score"] away_score: i64,
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

    let home_team_key = normalize_team_name(&home_team);
    let away_team_key = normalize_team_name(&away_team);

    let (table_to_save, history_to_save, fixtures_to_save, schedules_to_save, home_pos, away_pos, log_channel) = {
        let mut guild = guild_lock.lock().await;

        validate_match_input(&guild.table, &home_team_key, home_score, &away_team_key, away_score)?;

        let home_idx = find_team_index(&guild.table, &home_team_key)
            .with_context(|| "Home team was not found during update")?;
        let away_idx = find_team_index(&guild.table, &away_team_key)
            .with_context(|| "Away team was not found during update")?;

        let snapshot = guild.table.clone();
        push_snapshot(&mut guild.history, snapshot, HISTORY_LIMIT);

        guild.table[home_idx].pl += 1;
        guild.table[away_idx].pl += 1;

        let goal_diff = (home_score - away_score) as i32;
        guild.table[home_idx].gd += goal_diff;
        guild.table[away_idx].gd -= goal_diff;

        match home_score.cmp(&away_score) {
            std::cmp::Ordering::Greater => {
                guild.table[home_idx].w += 1;
                guild.table[home_idx].pts += 3;
                guild.table[away_idx].l += 1;
            }
            std::cmp::Ordering::Less => {
                guild.table[away_idx].w += 1;
                guild.table[away_idx].pts += 3;
                guild.table[home_idx].l += 1;
            }
            std::cmp::Ordering::Equal => {
                guild.table[home_idx].d += 1;
                guild.table[away_idx].d += 1;
                guild.table[home_idx].pts += 1;
                guild.table[away_idx].pts += 1;
            }
        }

        sort_table(&mut guild.table);
        recalculate_positions(&mut guild.table);

        let home_pos = find_team_index(&guild.table, &home_team_key)
            .with_context(|| "Updated home team position could not be determined")?
            + 1;
        let away_pos = find_team_index(&guild.table, &away_team_key)
            .with_context(|| "Updated away team position could not be determined")?
            + 1;

        push_fixture(&mut guild.fixtures, Fixture {
            home_team: home_team_key.clone(),
            away_team: away_team_key.clone(),
            home_score: home_score as i32,
            away_score: away_score as i32,
        });

        let log_channel = guild.settings.log_channel_id;
        (guild.table.clone(), guild.history.clone(), guild.fixtures.clone(), guild.schedules.clone(), home_pos, away_pos, log_channel)
    };

    // Mark the first matching unplayed fixture in any schedule as played.
    let updated_schedules = {
        let mut scheds = schedules_to_save;
        for sched in &mut scheds {
            if let Some(m) = sched.matches.iter_mut().find(|m| {
                !m.played
                    && normalize_team_name(&m.home_team) == home_team_key
                    && normalize_team_name(&m.away_team) == away_team_key
            }) {
                m.played = true;
                break;
            }
        }
        scheds
    };
    {
        let mut guild = guild_lock.lock().await;
        guild.schedules = updated_schedules.clone();
    }

    save_table(guild_id, &table_to_save)?;
    save_history(guild_id, &history_to_save)?;
    save_fixtures(guild_id, &fixtures_to_save)?;
    save_schedules(guild_id, &updated_schedules)?;

    let embed = serenity::CreateEmbed::new()
        .title("Match Result Applied")
        .description(format!(
            "**{} {}-{} {}**\nNew positions:\n- {}: #{}\n- {}: #{}",
            home_team_key, home_score, away_score, away_team_key,
            home_team_key, home_pos,
            away_team_key, away_pos,
        ))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    if let Some(channel_id) = log_channel {
        let _ = serenity::ChannelId::new(channel_id)
            .say(ctx.serenity_context(), format!("⚽ **{} {}-{} {}**", home_team_key, home_score, away_score, away_team_key))
            .await;
    }

    Ok(())
}


