use poise::serenity_prelude as serenity;

use crate::utils::table_utils::{
    find_team_index, format_table_monospace, normalize_team_name, recalculate_positions,
    sort_table,
};
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn predict(
    ctx: Context<'_>,
    #[description = "Schedule name (leave blank to use the first/only one)"] schedule_name: Option<String>,
    #[description = "Team to run the scenario for (leave blank for full projection)"] team: Option<String>,
    #[description = "Target finishing position for the scenario (e.g. 1 = champion)"]
    #[min = 1]
    target_position: Option<u32>,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.get(),
        None => {
            ctx.send(
                poise::CreateReply::default()
                    .content("This command must be used in a server.")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    let guild_lock = ctx.data().get_guild(guild_id).await;
    let guild = guild_lock.lock().await;

    if guild.table.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .content("The table is empty.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    if guild.schedules.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .content("No schedules found. Ask an admin to run `/generateschedule` first.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let schedule = resolve_schedule(&guild.schedules, schedule_name, ctx).await?;
    let unplayed: Vec<_> = schedule.matches.iter().filter(|m| !m.played).collect();

    if unplayed.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .content(format!(
                    "All fixtures in **{}** have been played — the table is final!",
                    schedule.name
                ))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    if team.is_none() {
        let mut projected_table = guild.table.clone();
        simulate_remaining_fixtures(&mut projected_table, &unplayed);
        sort_table(&mut projected_table);
        recalculate_positions(&mut projected_table);

        let embed = serenity::CreateEmbed::new()
            .title(format!("🔮 Projected Standings — {}", schedule.name))
            .description(format!("```\n{}\n```", format_table_monospace(&projected_table)))
            .footer(serenity::CreateEmbedFooter::new(format!(
                "{} played · {} remaining simulated by PPG + home advantage",
                schedule.matches.iter().filter(|m| m.played).count(),
                unplayed.len()
            )))
            .color(serenity::Color::new(0x57ff14));

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let team_key = normalize_team_name(team.as_deref().unwrap());
    let target_pos = target_position.unwrap_or(1) as usize;

    if find_team_index(&guild.table, &team_key).is_none() {
        ctx.send(
            poise::CreateReply::default()
                .content(format!("Team **{}** not found in the table.", team_key))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let team_remaining: Vec<_> = unplayed
        .iter()
        .filter(|m| normalize_team_name(&m.home_team) == team_key || normalize_team_name(&m.away_team) == team_key)
        .collect();

    let games_left = team_remaining.len();
    let mut found_scenario: Option<String> = None;

    'search: for wins_needed in 0..=games_left {
        let remaining_after_wins = games_left - wins_needed;
        for draws in 0..=remaining_after_wins {
            let losses = remaining_after_wins - draws;
            let mut projected = guild.table.clone();

            for m in &unplayed {
                let home_name = normalize_team_name(&m.home_team);
                let away_name = normalize_team_name(&m.away_team);
                if home_name == team_key || away_name == team_key {
                    continue;
                }

                if let (Some(home_idx), Some(away_idx)) = (
                    find_team_index(&projected, &home_name),
                    find_team_index(&projected, &away_name),
                ) {
                    projected[home_idx].pl += 1;
                    projected[home_idx].w += 1;
                    projected[home_idx].pts += 3;
                    projected[home_idx].gd += 1;
                    projected[away_idx].pl += 1;
                    projected[away_idx].l += 1;
                    projected[away_idx].gd -= 1;
                }
            }

            let mut wins_left = wins_needed;
            let mut draws_left = draws;
            let mut losses_left = losses;

            for m in &team_remaining {
                let opponent_name = if normalize_team_name(&m.home_team) == team_key {
                    normalize_team_name(&m.away_team)
                } else {
                    normalize_team_name(&m.home_team)
                };

                let team_idx = find_team_index(&projected, &team_key);
                let opponent_idx = find_team_index(&projected, &opponent_name);

                if let (Some(team_idx), Some(opponent_idx)) = (team_idx, opponent_idx) {
                    projected[team_idx].pl += 1;
                    projected[opponent_idx].pl += 1;

                    if wins_left > 0 {
                        projected[team_idx].w += 1;
                        projected[team_idx].pts += 3;
                        projected[team_idx].gd += 1;
                        projected[opponent_idx].l += 1;
                        projected[opponent_idx].gd -= 1;
                        wins_left -= 1;
                    } else if draws_left > 0 {
                        projected[team_idx].d += 1;
                        projected[team_idx].pts += 1;
                        projected[opponent_idx].d += 1;
                        projected[opponent_idx].pts += 1;
                        draws_left -= 1;
                    } else if losses_left > 0 {
                        projected[opponent_idx].w += 1;
                        projected[opponent_idx].pts += 3;
                        projected[opponent_idx].gd += 1;
                        projected[team_idx].l += 1;
                        projected[team_idx].gd -= 1;
                        losses_left -= 1;
                    }
                }
            }

            sort_table(&mut projected);
            recalculate_positions(&mut projected);

            if let Some(team_idx) = find_team_index(&projected, &team_key) {
                if projected[team_idx].pos <= target_pos {
                    found_scenario = Some(format!(
                        "**{} wins**, **{} draws**, **{} losses** from {} remaining games\n*(worst-case: all rivals win theirs)*",
                        wins_needed, draws, losses, games_left
                    ));
                    break 'search;
                }
            }
        }
    }

    let team_index = find_team_index(&guild.table, &team_key).unwrap();
    let current_points = guild.table[team_index].pts;
    let current_position = guild.table[team_index].pos;

    let description = match found_scenario {
        Some(scenario) => format!(
            "**{}** · currently **{}th** · **{} pts** · **{} games left** in _{}_\n\nTo finish **{}th or better:**\n{}",
            team_key, current_position, current_points, games_left, schedule.name, target_pos, scenario
        ),
        None => format!(
            "**{}** · currently **{}th** · **{} pts** · **{} games left** in _{}_\n\n❌ Finishing **{}th** is not mathematically possible even if {} wins every game.",
            team_key, current_position, current_points, games_left, schedule.name, target_pos, team_key
        ),
    };

    let embed = serenity::CreateEmbed::new()
        .title(format!("🔮 Scenario: {} → Top {}", team_key, target_pos))
        .description(description)
        .color(serenity::Color::new(0x57ff14));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

async fn resolve_schedule<'a>(
    schedules: &'a [crate::utils::table_utils::NamedSchedule],
    schedule_name: Option<String>,
    ctx: Context<'_>,
) -> Result<&'a crate::utils::table_utils::NamedSchedule, Error> {
    if let Some(name) = schedule_name {
        let key = name.to_lowercase();
        match schedules.iter().find(|schedule| schedule.name.to_lowercase() == key) {
            Some(schedule) => Ok(schedule),
            None => {
                let available: Vec<_> = schedules.iter().map(|schedule| schedule.name.as_str()).collect();
                ctx.send(
                    poise::CreateReply::default()
                        .content(format!("Schedule **{}** not found. Available: {}", name, available.join(", ")))
                        .ephemeral(true),
                )
                .await?;
                Err(anyhow::anyhow!("schedule not found"))
            }
        }
    } else if schedules.len() == 1 {
        Ok(&schedules[0])
    } else {
        let available: Vec<_> = schedules.iter().map(|schedule| schedule.name.as_str()).collect();
        ctx.send(
            poise::CreateReply::default()
                .content(format!("Multiple schedules exist. Use `schedule_name:` to pick one: {}", available.join(", ")))
                .ephemeral(true),
        )
        .await?;
        Err(anyhow::anyhow!("multiple schedules"))
    }
}

fn simulate_remaining_fixtures(
    projected_table: &mut crate::utils::table_utils::Table,
    remaining_matches: &[&crate::utils::table_utils::ScheduledMatch],
) {
    for scheduled_match in remaining_matches {
        let home_index = find_team_index(projected_table, &scheduled_match.home_team);
        let away_index = find_team_index(projected_table, &scheduled_match.away_team);

        let (Some(home_index), Some(away_index)) = (home_index, away_index) else {
            continue;
        };

        let home_ppg = ppg(&projected_table[home_index]);
        let away_ppg = ppg(&projected_table[away_index]);
        let diff = (home_ppg + 0.3) - away_ppg;

        projected_table[home_index].pl += 1;
        projected_table[away_index].pl += 1;

        if diff > 0.15 {
            projected_table[home_index].w += 1;
            projected_table[home_index].pts += 3;
            projected_table[home_index].gd += 1;
            projected_table[away_index].l += 1;
            projected_table[away_index].gd -= 1;
        } else if diff < -0.15 {
            projected_table[away_index].w += 1;
            projected_table[away_index].pts += 3;
            projected_table[away_index].gd += 1;
            projected_table[home_index].l += 1;
            projected_table[home_index].gd -= 1;
        } else {
            projected_table[home_index].d += 1;
            projected_table[home_index].pts += 1;
            projected_table[away_index].d += 1;
            projected_table[away_index].pts += 1;
        }
    }
}

fn ppg(team: &crate::utils::table_utils::Team) -> f64 {
    if team.pl == 0 {
        1.0
    } else {
        team.pts as f64 / team.pl as f64
    }
}
