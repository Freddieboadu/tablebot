mod commands;
mod utils;
mod web;

use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use anyhow::{Context as AnyhowContext, Result};
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use tokio::sync::Mutex;

use crate::utils::history::GuildData;

pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub guilds: Arc<Mutex<HashMap<u64, Arc<Mutex<GuildData>>>>>,
}

impl Data {
    pub async fn get_guild(&self, guild_id: u64) -> Arc<Mutex<GuildData>> {
        let mut guilds = self.guilds.lock().await;
        Arc::clone(guilds.entry(guild_id).or_insert_with(|| {
            Arc::new(Mutex::new(
                GuildData::load(guild_id).unwrap_or_default(),
            ))
        }))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").context("Missing DISCORD_TOKEN in environment")?;
    // Accept either GUILD_IDS (comma-separated) or the legacy GUILD_ID (single).
    let guild_ids: Vec<serenity::GuildId> = env::var("GUILD_IDS")
        .or_else(|_| env::var("GUILD_ID"))
        .ok()
        .map(|value| {
            value
                .split(',')
                .filter_map(|part| part.trim().parse::<u64>().ok())
                .map(serenity::GuildId::new)
                .collect()
        })
        .unwrap_or_default();

    // Start the public league-table website alongside the bot.
    let web_port: u16 = env::var("WEB_PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(8080);
    tokio::spawn(async move {
        web::serve(web_port).await;
    });

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
                commands::form::form(),
                commands::fixtures::fixtures(),
                commands::head2head::head2head(),
                commands::help::help(),
                commands::setadminrole::setadminrole(),
                commands::setlogchannel::setlogchannel(),
                commands::website::website(),
                commands::generateschedule::generateschedule(),
                commands::schedule::schedule(),
                commands::predict::predict(),
            ],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                // Global registration only — avoids duplicate commands that
                // appear when both guild and global commands are registered.
                // Commands propagate to all servers within ~1 hour of the bot joining.
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                // Clear any leftover guild-specific commands that cause duplicates.
                for id in guild_ids {
                    poise::builtins::register_in_guild(ctx, &framework.options().commands[..0], id).await?;
                }

                Ok(Data {
                    guilds: Arc::new(Mutex::new(HashMap::new())),
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
