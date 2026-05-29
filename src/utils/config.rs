use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::utils::history::{ensure_guild_dir, guild_config_path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildConfig {
    pub admin_role_id: Option<u64>,
    pub log_channel_id: Option<u64>,
}

impl Default for GuildConfig {
    fn default() -> Self {
        GuildConfig {
            admin_role_id: None,
            log_channel_id: None,
        }
    }
}

pub fn load_config(guild_id: &str) -> Result<GuildConfig> {
    ensure_guild_dir(guild_id)?;
    let path = guild_config_path(guild_id);
    if !Path::new(&path).exists() {
        return Ok(GuildConfig::default());
    }
    let raw = fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path))?;
    let config: GuildConfig =
        serde_json::from_str(&raw).with_context(|| format!("Failed to parse {}", path))?;
    Ok(config)
}

pub fn save_config(guild_id: &str, config: &GuildConfig) -> Result<()> {
    ensure_guild_dir(guild_id)?;
    let path = guild_config_path(guild_id);
    let json = serde_json::to_string_pretty(config).context("Failed to serialize config")?;
    fs::write(&path, format!("{}\n", json)).with_context(|| format!("Failed to write {}", path))?;
    Ok(())
}
