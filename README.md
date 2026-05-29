# PBL League Table Bot (Rust)

A Discord league table bot for PBL, rebuilt in **Rust** using **Serenity + Poise**.

## Features

- `/table` to display the current league table in an embed
- `/update` to enter match results and update both teams
- `/revert` to undo the latest table mutation using history
- `/addteam` to add a new team
- `/deleteteam` to remove a team with Yes/No button confirmation
- Multi-server support with per-guild persistence in `data/{guild_id}/table.json` and `data/{guild_id}/history.json`

## Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- `cargo`

## Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/Freddieboadu/tablebot.git
   cd tablebot
   ```

2. Create your environment file:

   ```bash
   cp .env.example .env
   ```

3. Fill in `.env` values:

   - `DISCORD_TOKEN`
   - `CLIENT_ID`

4. Run the bot:

   ```bash
   cargo run
   ```

5. Global slash command propagation can take up to 1 hour after first run.

## Multi-server support

- Commands are registered globally (no `GUILD_ID` required).
- Each Discord server gets an independent table/history under `data/{guild_id}/`.
- On first use in a server, the bot creates that server's data files and starts with an empty table.
- Development tip: for instant command updates while iterating, you can temporarily switch to guild-specific command registration in `src/main.rs`.

## Commands

### `/table`
Display the full table.

Example:

```text
/table
```

### `/update <home_team> <home_score> <away_team> <away_score>`
Update the table based on a match result.

Example:

```text
/update home_team:PSG home_score:4 away_team:DORTMUND away_score:0
```

### `/revert`
Revert the latest saved table state.

Example:

```text
/revert
```

### `/addteam <team_name>`
Add a team with all zero stats.

Example:

```text
/addteam team_name:ARSENAL
```

### `/deleteteam <team_name>`
Delete a team after clicking **Yes** on the confirmation buttons.

Example:

```text
/deleteteam team_name:MALMO FF
```

## Permissions

Admin commands require **Manage Server / MANAGE_GUILD**:

- `/revert`
- `/addteam`
- `/deleteteam`

If a user lacks permission, Discord returns an ephemeral permission error.

## Revert System

- Every destructive mutation pushes the previous table into that server's `data/{guild_id}/history.json`
- History acts as a stack
- `/revert` pops the latest state and restores it
- History is capped to the latest 20 entries

## Creating a Discord Bot (quick steps)

1. Open [Discord Developer Portal](https://discord.com/developers/applications)
2. Create **New Application**
3. Open **Bot** tab and create bot user
4. Copy bot token into `.env` as `DISCORD_TOKEN`
5. In **OAuth2 > URL Generator**, enable:
   - `bot` and `applications.commands` scopes
   - Required permissions (including Manage Guild for admin commands)
6. Open generated URL and invite bot to your server
