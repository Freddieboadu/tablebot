# PBL League Table Bot (Rust)

A Discord league table bot for PBL, built in **Rust** using **Serenity + Poise**.  
Supports multiple servers — each server has its own isolated league table.

---

## Features

<<<<<<< HEAD
- Per-server (per-guild) league tables — invite the bot to as many servers as you like
- Slash commands with autocomplete on team names
- Match result history and `/revert` to undo mistakes
- Form guide (last 5 results per team), recent fixtures, and head-to-head stats
- Designated admin role support — restrict who can edit the table
- Change log channel — automatically post every update to a channel of your choice

---
=======
- `/table` to display the current league table in an embed
- `/update` to enter match results and update both teams
- `/revert` to undo the latest table mutation using history
- `/addteam` to add a new team
- `/deleteteam` to remove a team with Yes/No button confirmation
- Multi-server support with per-guild persistence in `data/{guild_id}/table.json` and `data/{guild_id}/history.json`
>>>>>>> origin/copilot/rebuild-league-table-bot

## Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- A Discord bot token ([Discord Developer Portal](https://discord.com/developers/applications))

---

## Setting Up Your League (Step by Step)

1. **Invite the bot** to your server using the OAuth2 URL Generator in the Developer Portal.  
   Enable scopes: `bot` and `applications.commands`.  
   Enable permissions: *Send Messages*, *Embed Links*, *Use Slash Commands*.

2. **Add your teams** — use `/addteam` with a comma-separated list:
   ```
   /addteam teams:Chelsea, PSG, Barcelona, Celtic
   ```

3. **Enter results** after each match:
   ```
   /update home_team:Chelsea home_score:2 away_team:PSG away_score:1
   ```

<<<<<<< HEAD
4. **View the standings** at any time:
=======
3. Fill in `.env` values:

   - `DISCORD_TOKEN`
   - `CLIENT_ID`

4. Run the bot:

   ```bash
   cargo run
>>>>>>> origin/copilot/rebuild-league-table-bot
   ```
   /table
   ```

5. **Restrict who can edit** the table (optional):
   ```
   /setadminrole role:@LeagueAdmin
   ```

6. **Track all changes** in a dedicated channel (optional):
   ```
   /setlogchannel channel:#league-log
   ```

---

5. Global slash command propagation can take up to 1 hour after first run.

## Multi-server support

- Commands are registered globally (no `GUILD_ID` required).
- Each Discord server gets an independent table/history under `data/{guild_id}/`.
- On first use in a server, the bot creates that server's data files and starts with an empty table.
- Development tip: for instant command updates while iterating, you can temporarily switch to guild-specific command registration in `src/main.rs`.

## Commands

### 📊 Table Commands

| Command | Description |
|---------|-------------|
| `/table` | Show the current league standings |
| `/form <team>` | Show a team's last 5 results (🟢 W  🔴 L  🟡 D) with current streak |
| `/fixtures` | Show the last 10 match results |
| `/head2head <team1> <team2>` | Show all results between two teams with a win/draw/loss summary |

### ⚽ Match Commands

| Command | Description |
|---------|-------------|
| `/update <home_team> <home_score> <away_team> <away_score>` | Enter a match result — updates both teams in the table |
| `/revert` | Undo the last change made to the table |

### 👥 Team Commands

| Command | Description |
|---------|-------------|
| `/addteam <teams>` | Add one or more teams separated by commas: `Chelsea, PSG, Barcelona` |
| `/deleteteam <teams>` | Remove one or more teams separated by commas (requires confirmation) |
| `/cleartable` | Wipe the table and start fresh — requires Yes/No confirmation (admin only) |

### 🔒 Admin Setup

| Command | Description |
|---------|-------------|
| `/setadminrole <role>` | Set which role can manage the league table (requires Manage Server) |
| `/setlogchannel <channel>` | Set a channel where all changes are posted (requires Manage Server) |

### ℹ️ Info

| Command | Description |
|---------|-------------|
| `/help` | Show all commands with descriptions |

---

## Permissions

Admin commands (`/update`, `/revert`, `/addteam`, `/deleteteam`, `/cleartable`) check:

1. Does the user have **Manage Server** permission? → ✅ allowed  
2. Is there an admin role set AND does the user have it? → ✅ allowed  
3. Neither → ❌ ephemeral error (only visible to the user)

Use `/setadminrole` to assign a role so your league managers don't need full server admin rights.

---

## Data Storage

Each server stores its own files in:

```
data/
└── {guild_id}/
    ├── table.json      — current standings
    ├── history.json    — revert stack (last 20 states)
    ├── fixtures.json   — all match results log
    └── config.json     — admin role + log channel
```

---

## Setup

```bash
git clone https://github.com/Freddieboadu/tablebot.git
cd tablebot
cp .env.example .env
# Edit .env with your DISCORD_TOKEN and CLIENT_ID
cargo run
```

`.env` file:

```env
DISCORD_TOKEN=your_bot_token_here
CLIENT_ID=your_application_id_here
# Optional: set GUILD_ID to register commands to one server instantly during development
# GUILD_ID=your_server_id_here
```

> **Note:** Without `GUILD_ID` set, commands are registered globally (up to 1 hour delay).  
> During development, set `GUILD_ID` to your test server for instant updates.

---

## Revert System

<<<<<<< HEAD
- Every destructive change (`/update`, `/addteam`, `/deleteteam`, `/cleartable`) saves the current table to history first
- `/revert` restores the most recent saved state
- History is capped at 20 entries per server
=======
- Every destructive mutation pushes the previous table into that server's `data/{guild_id}/history.json`
- History acts as a stack
- `/revert` pops the latest state and restores it
- History is capped to the latest 20 entries
>>>>>>> origin/copilot/rebuild-league-table-bot

---

## Creating a Discord Bot (Quick Steps)

1. Open [Discord Developer Portal](https://discord.com/developers/applications)
<<<<<<< HEAD
2. Click **New Application**, give it a name
3. Open the **Bot** tab and click **Reset Token** — copy it to `.env` as `DISCORD_TOKEN`
4. Your **Application ID** on the General Information page is your `CLIENT_ID`
5. In **OAuth2 → URL Generator**, enable scopes: `bot` and `applications.commands`
6. Enable permissions: *Send Messages*, *Embed Links*, *Use Slash Commands*
7. Copy the generated URL, open it in your browser, and invite the bot to your server

=======
2. Create **New Application**
3. Open **Bot** tab and create bot user
4. Copy bot token into `.env` as `DISCORD_TOKEN`
5. In **OAuth2 > URL Generator**, enable:
   - `bot` and `applications.commands` scopes
   - Required permissions (including Manage Guild for admin commands)
6. Open generated URL and invite bot to your server
>>>>>>> origin/copilot/rebuild-league-table-bot
