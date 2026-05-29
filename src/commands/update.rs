use chrono::Utc;
use poise::serenity_prelude as serenity;

use crate::utils::fixtures::{append_fixture, Fixture};
use crate::utils::history::{
    load_history, load_table, push_snapshot, save_history, save_table, HISTORY_LIMIT,
};
use crate::utils::permissions::{is_admin, post_to_log_channel};
use crate::utils::table_utils::{
    find_team_index, normalize_team_name, recalculate_positions, sort_table,
};
use crate::utils::validator::validate_match_input;
use crate::{Context, Error};

async fn autocomplete_team(ctx: Context<'_>, partial: &str) -> Vec<String> {
    let guild_id = ctx.guild_id().map(|g| g.to_string()).unwrap_or_default();
    let table = crate::utils::history::load_table(&guild_id).unwrap_or_default();
    let lower = partial.to_lowercase();
    table
        .into_iter()
        .map(|t| t.club)
        .filter(|name| name.to_lowercase().starts_with(&lower))
        .collect()
}

#[poise::command(slash_command)]
pub async fn update(
    ctx: Context<'_>,
    #[description = "Home team name"]
    #[autocomplete = "autocomplete_team"]
    home_team: String,
    #[description = "Home team score"] home_score: i64,
    #[description = "Away team name"]
    #[autocomplete = "autocomplete_team"]
    away_team: String,
    #[description = "Away team score"] away_score: i64,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.to_string(),
        None => {
            ctx.send(
                poise::CreateReply::default()
                    .content("❌ This command can only be used in a server.")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    if !is_admin(&ctx).await? {
        ctx.send(
            poise::CreateReply::default()
                .content("❌ You need the league admin role to use this command!")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let home_team_key = normalize_team_name(&home_team);
    let away_team_key = normalize_team_name(&away_team);

    let mut table = load_table(&guild_id)?;

    if let Err(e) = validate_match_input(
        &table,
        &home_team_key,
        home_score,
        &away_team_key,
        away_score,
    ) {
        ctx.send(
            poise::CreateReply::default()
                .content(format!("❌ {}", e))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let mut history = load_history(&guild_id)?;
    push_snapshot(&mut history, table.clone(), HISTORY_LIMIT);

    let home_idx = find_team_index(&table, &home_team_key).unwrap();
    let away_idx = find_team_index(&table, &away_team_key).unwrap();

    table[home_idx].pl += 1;
    table[away_idx].pl += 1;

    let goal_diff = home_score - away_score;
    table[home_idx].gd += goal_diff as i32;
    table[away_idx].gd -= goal_diff as i32;

    match home_score.cmp(&away_score) {
        std::cmp::Ordering::Greater => {
            table[home_idx].w += 1;
            table[home_idx].pts += 3;
            table[away_idx].l += 1;
        }
        std::cmp::Ordering::Less => {
            table[away_idx].w += 1;
            table[away_idx].pts += 3;
            table[home_idx].l += 1;
        }
        std::cmp::Ordering::Equal => {
            table[home_idx].d += 1;
            table[away_idx].d += 1;
            table[home_idx].pts += 1;
            table[away_idx].pts += 1;
        }
    }

    sort_table(&mut table);
    recalculate_positions(&mut table);

    let home_pos = find_team_index(&table, &home_team_key).unwrap() + 1;
    let away_pos = find_team_index(&table, &away_team_key).unwrap() + 1;

    save_table(&guild_id, &table)?;
    save_history(&guild_id, &history)?;

    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    append_fixture(
        &guild_id,
        Fixture {
            home: home_team_key.clone(),
            home_score,
            away: away_team_key.clone(),
            away_score,
            timestamp: timestamp.clone(),
        },
    )?;

    let embed = serenity::CreateEmbed::new()
        .title("⚽ Match Result Applied")
        .description(format!(
            "**{} {}-{} {}**\nNew positions:\n- {}: #{}\n- {}: #{}",
            home_team_key,
            home_score,
            away_score,
            away_team_key,
            home_team_key,
            home_pos,
            away_team_key,
            away_pos
        ))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    let log_msg = format!(
        "⚽ **{}** used `/update`: {} {}-{} {} at {}",
        ctx.author().name,
        home_team_key,
        home_score,
        away_score,
        away_team_key,
        timestamp
    );
    post_to_log_channel(&ctx, &guild_id, &log_msg).await;

    Ok(())
}
