use anyhow::Context as AnyhowContext;
use poise::serenity_prelude as serenity;

use crate::utils::history::{
    load_history, load_table, push_snapshot, save_history, save_table, table_exists,
    FRESH_TABLE_MESSAGE, GUILD_ONLY_MESSAGE, HISTORY_LIMIT,
};
use crate::utils::table_utils::{
    find_team_index, normalize_team_name, recalculate_positions, sort_table,
};
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
    let Some(guild_id) = ctx.guild_id().map(|id| id.to_string()) else {
        ctx.send(
            poise::CreateReply::default()
                .content(GUILD_ONLY_MESSAGE)
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    };

    let is_new_server = !table_exists(&guild_id);
    if is_new_server {
        ctx.send(poise::CreateReply::default().content(FRESH_TABLE_MESSAGE))
            .await?;
    }

    let home_team_key = normalize_team_name(&home_team);
    let away_team_key = normalize_team_name(&away_team);
    let mut table = load_table(&guild_id)?;
    let mut history = load_history(&guild_id)?;

    validate_match_input(
        &table,
        &home_team_key,
        home_score,
        &away_team_key,
        away_score,
    )?;

    let home_idx = find_team_index(&table, &home_team_key)
        .with_context(|| "Home team was not found during update")?;
    let away_idx = find_team_index(&table, &away_team_key)
        .with_context(|| "Away team was not found during update")?;

    push_snapshot(&mut history, table.clone(), HISTORY_LIMIT);

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

    let home_pos = find_team_index(&table, &home_team_key)
        .with_context(|| "Updated home team position could not be determined")?
        + 1;
    let away_pos = find_team_index(&table, &away_team_key)
        .with_context(|| "Updated away team position could not be determined")?
        + 1;

    save_table(&guild_id, &table)?;
    save_history(&guild_id, &history)?;

    let embed = serenity::CreateEmbed::new()
        .title("Match Result Applied")
        .description(format!(
            "**{} {}-{} {}**\nNew positions:\n- {}: {}\n- {}: {}",
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
    Ok(())
}
