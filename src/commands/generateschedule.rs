use poise::serenity_prelude as serenity;

use crate::utils::history::save_schedules;
use crate::utils::permissions::check_admin;
use crate::utils::table_utils::{
    generate_knockout_round, generate_schedule, generate_single_robin, normalize_team_name,
    NamedSchedule, ScheduleKind,
};
use crate::{Context, Error};

/// Generate or advance a named schedule.
///
/// Types:
/// - `roundrobin` = home-and-away league season
/// - `single` = each pair plays once
/// - `knockout` = single-elimination bracket
#[poise::command(slash_command, rename = "generateschedule")]
pub async fn generateschedule(
    ctx: Context<'_>,
    #[description = "Name for this schedule (e.g. 'Group A', 'Knockout')"] name: String,
    #[description = "Type: roundrobin | single | knockout (default: roundrobin)"]
    schedule_type: Option<String>,
    #[description = "Comma-separated teams to include (default: all table teams)"]
    teams: Option<String>,
    #[description = "Games per club per gameweek for round-robin types (default 3)"]
    #[min = 1]
    #[max = 5]
    games_per_week: Option<u32>,
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
    let settings = guild_lock.lock().await.settings.clone();
    if !check_admin(ctx, &settings).await {
        ctx.send(
            poise::CreateReply::default()
                .content("You don't have permission to use this command.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let kind = match schedule_type.as_deref().unwrap_or("roundrobin").to_lowercase().as_str() {
        "single" | "singlerobin" | "single_robin" => ScheduleKind::SingleRoundRobin,
        "knockout" | "ko" => ScheduleKind::Knockout,
        _ => ScheduleKind::RoundRobin,
    };
    let gpw = games_per_week.unwrap_or(3);
    let sched_name = name.trim().to_string();

    let (team_list, mut schedules) = {
        let guild = guild_lock.lock().await;
        if guild.table.len() < 2 {
            ctx.send(
                poise::CreateReply::default()
                    .content("You need at least 2 teams in the table first.")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }

        let resolved: Vec<String> = if let Some(ref t) = teams {
            t.split(',')
                .map(normalize_team_name)
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            guild.table.iter().map(|t| t.club.clone()).collect()
        };
        (resolved, guild.schedules.clone())
    };

    if team_list.len() < 2 {
        ctx.send(
            poise::CreateReply::default()
                .content("Need at least 2 teams.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let schedule = match kind {
        ScheduleKind::RoundRobin => NamedSchedule {
            name: sched_name.clone(),
            kind: ScheduleKind::RoundRobin,
            matches: generate_schedule(&team_list, gpw),
        },
        ScheduleKind::SingleRoundRobin => NamedSchedule {
            name: sched_name.clone(),
            kind: ScheduleKind::SingleRoundRobin,
            matches: generate_single_robin(&team_list, gpw),
        },
        ScheduleKind::Knockout => {
            let existing = schedules.iter().find(|s| {
                s.name.eq_ignore_ascii_case(&sched_name) && s.kind == ScheduleKind::Knockout
            });

            if let Some(existing) = existing {
                let max_gw = existing.matches.iter().map(|m| m.gameweek).max().unwrap_or(0);
                let current_round: Vec<_> = existing
                    .matches
                    .iter()
                    .filter(|m| m.gameweek == max_gw)
                    .collect();
                if current_round.is_empty() || !current_round.iter().all(|m| m.played) {
                    let done = current_round.iter().filter(|m| m.played).count();
                    ctx.send(
                        poise::CreateReply::default()
                            .content(format!(
                                "Round {} is not finished yet ({}/{} matches played).",
                                max_gw,
                                done,
                                current_round.len()
                            ))
                            .ephemeral(true),
                    )
                    .await?;
                    return Ok(());
                }

                let winners: Vec<String> = {
                    let guild = guild_lock.lock().await;
                    current_round
                        .iter()
                        .filter_map(|m| {
                            let hn = normalize_team_name(&m.home_team);
                            let an = normalize_team_name(&m.away_team);
                            guild.fixtures.iter().find(|f| {
                                normalize_team_name(&f.home_team) == hn
                                    && normalize_team_name(&f.away_team) == an
                            }).map(|f| if f.home_score >= f.away_score { hn } else { an })
                        })
                        .collect()
                };

                if winners.len() < 2 {
                    ctx.send(
                        poise::CreateReply::default()
                            .content("Could not determine enough winners to advance. Enter all results first.")
                            .ephemeral(true),
                    )
                    .await?;
                    return Ok(());
                }

                let next_gw = max_gw + 1;
                let mut updated = existing.matches.clone();
                updated.extend(generate_knockout_round(&winners, next_gw));
                NamedSchedule { name: sched_name.clone(), kind: ScheduleKind::Knockout, matches: updated }
            } else {
                NamedSchedule {
                    name: sched_name.clone(),
                    kind: ScheduleKind::Knockout,
                    matches: generate_knockout_round(&team_list, 1),
                }
            }
        }
    };

    if let Some(pos) = schedules.iter().position(|s| s.name.eq_ignore_ascii_case(&sched_name)) {
        schedules[pos] = schedule;
    } else {
        schedules.push(schedule);
    }

    {
        let mut guild = guild_lock.lock().await;
        guild.schedules = schedules.clone();
    }
    save_schedules(guild_id, &schedules)?;

    let schedule_list: Vec<String> = schedules
        .iter()
        .map(|s| format!("• **{}** ({})", s.name, kind_label(&s.kind)))
        .collect();

    let embed = serenity::CreateEmbed::new()
        .title(format!("📅 Schedule — {}", sched_name))
        .description(format!(
            "**{}** created/updated.\n\n**All schedules:**\n{}",
            sched_name,
            schedule_list.join("\n")
        ))
        .footer(serenity::CreateEmbedFooter::new("Use /schedule to view fixtures · /predict to project standings"))
        .color(serenity::Color::new(0x57ff14));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

fn kind_label(kind: &ScheduleKind) -> &'static str {
    match kind {
        ScheduleKind::RoundRobin => "round-robin",
        ScheduleKind::SingleRoundRobin => "single round-robin",
        ScheduleKind::Knockout => "knockout",
    }
}
