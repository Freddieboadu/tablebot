mod commands;
mod utils;

use std::env;

use anyhow::{Context as AnyhowContext, Result};
use dotenv::dotenv;
use poise::serenity_prelude as serenity;

use crate::utils::history::ensure_data_dir;

pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").context("Missing DISCORD_TOKEN in environment")?;

    ensure_data_dir()?;

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
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
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
