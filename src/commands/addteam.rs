use poise::serenity_prelude as serenity;

<<<<<<< HEAD
use crate::utils::history::{load_table, save_table};
use crate::utils::permissions::{is_admin, post_to_log_channel};
=======
use crate::utils::history::{
    load_table, save_table, table_exists, FRESH_TABLE_MESSAGE, GUILD_ONLY_MESSAGE,
};
>>>>>>> origin/copilot/rebuild-league-table-bot
use crate::utils::table_utils::{normalize_team_name, recalculate_positions, sort_table, Team};
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn addteam(
    ctx: Context<'_>,
    #[description = "Team name(s), separated by commas — e.g. Chelsea, PSG, Barcelona"]
    teams: String,
) -> Result<(), Error> {
<<<<<<< HEAD
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

    let mut table = load_table(&guild_id)?;

    let mut added: Vec<String> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();

    for raw_name in teams.split(',') {
        let name = normalize_team_name(raw_name);
        if name.is_empty() {
            continue;
        }
        let already_exists = table.iter().any(|t| t.club == name);
        if already_exists {
            skipped.push(name);
        } else {
            let next_pos = table.len() + 1;
            table.push(Team {
                pos: next_pos,
                club: name.clone(),
                pl: 0,
                w: 0,
                d: 0,
                l: 0,
                gd: 0,
                pts: 0,
            });
            added.push(name);
        }
    }

    if added.is_empty() && skipped.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .content("❌ No team names were provided.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    sort_table(&mut table);
    recalculate_positions(&mut table);
    save_table(&guild_id, &table)?;

    let mut desc = String::new();
    if !added.is_empty() {
        desc.push_str(&format!("✅ **Added:** {}\n", added.join(", ")));
    }
    if !skipped.is_empty() {
        desc.push_str(&format!(
            "⚠️ **Skipped (already in table):** {}",
            skipped.join(", ")
        ));
    }
=======
    let Some(guild_id) = ctx.guild_id().map(|id| id.to_string()) else {
        ctx.send(
            poise::CreateReply::default()
                .content(GUILD_ONLY_MESSAGE)
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    };

    let normalized_name = normalize_team_name(&team_name);
    let is_new_server = !table_exists(&guild_id);
    if is_new_server {
        ctx.send(poise::CreateReply::default().content(FRESH_TABLE_MESSAGE))
            .await?;
    }

    let mut table = load_table(&guild_id)?;
    validate_new_team_name(&table, &normalized_name)?;
    let next_pos = table.len() + 1;

    table.push(Team {
        pos: next_pos,
        club: normalized_name.clone(),
        pl: 0,
        w: 0,
        d: 0,
        l: 0,
        gd: 0,
        pts: 0,
    });
    sort_table(&mut table);
    recalculate_positions(&mut table);

    save_table(&guild_id, &table)?;
>>>>>>> origin/copilot/rebuild-league-table-bot

    let embed = serenity::CreateEmbed::new()
        .title("👥 Teams Updated")
        .description(desc.trim())
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    if !added.is_empty() {
        let log_msg = format!(
            "👥 **{}** used `/addteam`: added {}",
            ctx.author().name,
            added.join(", ")
        );
        post_to_log_channel(&ctx, &guild_id, &log_msg).await;
    }

    Ok(())
}
