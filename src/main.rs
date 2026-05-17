mod commands;
mod utils;

use std::env;
use std::sync::Arc;

use anyhow::{Context as AnyhowContext, Result};
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use tokio::sync::Mutex;

use crate::utils::history::{ensure_data_files, load_history, load_table};
use crate::utils::table_utils::Table;

pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub table: Arc<Mutex<Table>>,
    pub history: Arc<Mutex<Vec<Table>>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").context("Missing DISCORD_TOKEN in environment")?;
    let guild_id = env::var("GUILD_ID")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(serenity::GuildId::new);

    ensure_data_files()?;

    let intents = serenity::GatewayIntents::non_privileged();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::table::table(),
                commands::update::update(),
                commands::revert::revert(),
                commands::addteam::addteam(),
                commands::deleteteam::deleteteam(),
            ],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                if let Some(id) = guild_id {
                    poise::builtins::register_in_guild(ctx, &framework.options().commands, id)
                        .await?;
                } else {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                }

                let mut table = load_table()?;
                crate::utils::table_utils::sort_table(&mut table);
                crate::utils::table_utils::recalculate_positions(&mut table);

                Ok(Data {
                    table: Arc::new(Mutex::new(table)),
                    history: Arc::new(Mutex::new(load_history()?)),
                })
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
