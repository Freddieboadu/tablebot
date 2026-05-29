mod commands;
mod utils;

use std::env;

use anyhow::{Context as AnyhowContext, Result};
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
<<<<<<< HEAD
=======

use crate::utils::history::ensure_data_dir;
>>>>>>> origin/copilot/rebuild-league-table-bot

pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").context("Missing DISCORD_TOKEN in environment")?;

<<<<<<< HEAD
=======
    ensure_data_dir()?;

>>>>>>> origin/copilot/rebuild-league-table-bot
    let intents = serenity::GatewayIntents::non_privileged();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::table::table(),
                commands::update::update(),
                commands::revert::revert(),
                commands::addteam::addteam(),
                commands::deleteteam::deleteteam(),
                commands::cleartable::cleartable(),
                commands::help::help(),
                commands::form::form(),
                commands::fixtures::fixtures(),
                commands::head2head::head2head(),
                commands::setadminrole::setadminrole(),
                commands::setlogchannel::setlogchannel(),
            ],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
<<<<<<< HEAD
                if let Some(id) = guild_id {
                    poise::builtins::register_in_guild(ctx, &framework.options().commands, id)
                        .await?;
                } else {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                }
=======
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
>>>>>>> origin/copilot/rebuild-league-table-bot
                Ok(Data {})
            })
        })
        .build();

    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .await
        .with_context(|| "Failed to create Discord client")?;

    client
        .start()
        .await
        .with_context(|| "Discord client exited unexpectedly")?;

    Ok(())
}
