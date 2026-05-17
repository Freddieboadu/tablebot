/**
 * deploy-commands.js
 * Register all slash commands with the Discord API.
 * Run this script once after setup, and any time you modify command definitions:
 *   node deploy-commands.js
 */

require('dotenv').config();

const { REST, Routes } = require('discord.js');
const fs = require('fs');
const path = require('path');

// ── Validate required environment variables ──────────────────────────────────
const { DISCORD_TOKEN, CLIENT_ID, GUILD_ID } = process.env;

if (!DISCORD_TOKEN || !CLIENT_ID) {
  console.error('[ERROR] DISCORD_TOKEN and CLIENT_ID must be set in your .env file.');
  process.exit(1);
}

// ── Collect command data ──────────────────────────────────────────────────────
const commands = [];
const commandsPath = path.join(__dirname, 'commands');
const commandFiles = fs.readdirSync(commandsPath).filter(f => f.endsWith('.js'));

for (const file of commandFiles) {
  const command = require(path.join(commandsPath, file));
  if (command.data) {
    commands.push(command.data.toJSON());
    console.log(`[INFO] Loaded command definition: /${command.data.name}`);
  }
}

// ── Register commands via REST ────────────────────────────────────────────────
const rest = new REST().setToken(DISCORD_TOKEN);

(async () => {
  try {
    console.log(`[INFO] Deploying ${commands.length} slash command(s)...`);

    let route;
    if (GUILD_ID) {
      // Guild-scoped deployment — instant update (ideal for development)
      route = Routes.applicationGuildCommands(CLIENT_ID, GUILD_ID);
      console.log(`[INFO] Deploying to guild ${GUILD_ID} (instant)`);
    } else {
      // Global deployment — may take up to 1 hour to propagate
      route = Routes.applicationCommands(CLIENT_ID);
      console.log('[INFO] Deploying globally (may take up to 1 hour)');
    }

    const data = await rest.put(route, { body: commands });
    console.log(`[SUCCESS] Deployed ${data.length} command(s) successfully!`);
  } catch (error) {
    console.error('[ERROR] Failed to deploy commands:', error);
    process.exit(1);
  }
})();
