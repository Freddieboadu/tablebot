use poise::serenity_prelude as serenity;

use crate::utils::history::save_table;
use crate::utils::table_utils::{normalize_team_name, recalculate_positions, sort_table, Team};
use crate::utils::validator::validate_new_team_name;
use crate::{Context, Error};

#[poise::command(slash_command, required_permissions = "MANAGE_GUILD")]
pub async fn addteam(
    ctx: Context<'_>,
    #[description = "Team name"] team_name: String,
) -> Result<(), Error> {
    let normalized_name = normalize_team_name(&team_name);

    let table_to_save = {
        let mut table = ctx.data().table.lock().await;
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
        table.clone()
    };

    save_table(&table_to_save)?;

    let embed = serenity::CreateEmbed::new()
        .title("Team Added")
        .description(format!(
            "{} has been added to the league table.",
            normalized_name
        ))
        .color(serenity::Color::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
