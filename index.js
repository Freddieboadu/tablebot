/**
 * index.js
 * PBL TableBot — Discord bot entry point.
 * Loads all slash command handlers and dispatches interactions.
 * Includes rate-limiting (1 update per 5 seconds per user).
 */

require('dotenv').config();

const fs = require('fs');
const path = require('path');
const { Client, GatewayIntentBits, Collection, Events } = require('discord.js');

// ── Validate required environment variables ──────────────────────────────────
const REQUIRED_ENV = ['DISCORD_TOKEN', 'CLIENT_ID'];
for (const key of REQUIRED_ENV) {
  if (!process.env[key]) {
    console.error(`[ERROR] Missing required environment variable: ${key}`);
    console.error('Copy .env.example to .env and fill in your credentials.');
    process.exit(1);
  }
}

// ── Ensure data directory and files exist ────────────────────────────────────
const DATA_DIR = path.join(__dirname, 'data');
const TABLE_PATH = path.join(DATA_DIR, 'table.json');
const HISTORY_PATH = path.join(DATA_DIR, 'history.json');

if (!fs.existsSync(DATA_DIR)) {
  fs.mkdirSync(DATA_DIR, { recursive: true });
}
if (!fs.existsSync(TABLE_PATH)) {
  console.warn('[WARN] data/table.json not found — creating empty table.');
  fs.writeFileSync(TABLE_PATH, '[]', 'utf8');
}
if (!fs.existsSync(HISTORY_PATH)) {
  console.warn('[WARN] data/history.json not found — creating empty history.');
  fs.writeFileSync(HISTORY_PATH, '[]', 'utf8');
}

// ── Create Discord client ────────────────────────────────────────────────────
const client = new Client({ intents: [GatewayIntentBits.Guilds] });

// ── Load slash commands ──────────────────────────────────────────────────────
client.commands = new Collection();
const commandsPath = path.join(__dirname, 'commands');
const commandFiles = fs.readdirSync(commandsPath).filter(f => f.endsWith('.js'));

for (const file of commandFiles) {
  const command = require(path.join(commandsPath, file));
  if (!command.data || !command.execute) {
    console.warn(`[WARN] Skipping ${file} — missing data or execute export.`);
    continue;
  }
  client.commands.set(command.data.name, command);
  console.log(`[INFO] Loaded command: /${command.data.name}`);
}

// ── Rate limiting (1 update per 5 seconds per user) ─────────────────────────
// Only rate-limit the /update command to prevent spam
const RATE_LIMITED_COMMANDS = new Set(['update']);
const RATE_LIMIT_MS = 5000;
const cooldowns = new Collection(); // Map<userId, Map<commandName, lastUsedTimestamp>>

function checkRateLimit(userId, commandName) {
  if (!RATE_LIMITED_COMMANDS.has(commandName)) return null;

  if (!cooldowns.has(userId)) cooldowns.set(userId, new Collection());
  const userCooldowns = cooldowns.get(userId);

  const now = Date.now();
  const lastUsed = userCooldowns.get(commandName) ?? 0;
  const elapsed = now - lastUsed;

  if (elapsed < RATE_LIMIT_MS) {
    const remaining = ((RATE_LIMIT_MS - elapsed) / 1000).toFixed(1);
    return `⏳ Please wait **${remaining}s** before using \`/${commandName}\` again.`;
  }

  userCooldowns.set(commandName, now);
  return null;
}

// ── Interaction handler ──────────────────────────────────────────────────────
client.on(Events.InteractionCreate, async interaction => {
  if (!interaction.isChatInputCommand()) return;

  const command = client.commands.get(interaction.commandName);
  if (!command) {
    console.error(`[ERROR] No handler found for command: /${interaction.commandName}`);
    return interaction.reply({
      content: '❌ Unknown command. Try `/table`, `/update`, `/revert`, `/addteam`, or `/deleteteam`.',
      ephemeral: true,
    });
  }

  // Check rate limit
  const rateLimitMsg = checkRateLimit(interaction.user.id, interaction.commandName);
  if (rateLimitMsg) {
    return interaction.reply({ content: rateLimitMsg, ephemeral: true });
  }

  // Execute the command
  try {
    await command.execute(interaction);
  } catch (error) {
    console.error(`[ERROR] Failed to execute /${interaction.commandName}:`, error);
    const errorMsg = '❌ An error occurred while running this command. Please try again.';
    if (interaction.replied || interaction.deferred) {
      await interaction.followUp({ content: errorMsg, ephemeral: true }).catch(() => {});
    } else {
      await interaction.reply({ content: errorMsg, ephemeral: true }).catch(() => {});
    }
  }
});

// ── Ready event ──────────────────────────────────────────────────────────────
client.once(Events.ClientReady, c => {
  console.log(`[INFO] PBL TableBot is online as ${c.user.tag}`);
  console.log(`[INFO] Serving ${c.guilds.cache.size} guild(s)`);
  console.log(`[INFO] Loaded ${client.commands.size} command(s)`);
});

// ── Login ────────────────────────────────────────────────────────────────────
client.login(process.env.DISCORD_TOKEN).catch(err => {
  console.error('[ERROR] Failed to login:', err.message);
  process.exit(1);
});
