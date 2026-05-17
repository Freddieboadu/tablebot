# PBL League Table Bot (Rust)

A Discord league table bot for PBL, rebuilt in **Rust** using **Serenity + Poise**.

## Features

- `/table` to display the current league table in an embed
- `/update` to enter match results and update both teams
- `/revert` to undo the latest table mutation using history
- `/addteam` to add a new team
- `/deleteteam` to remove a team with Yes/No button confirmation
- File-based persistence using `data/table.json` and `data/history.json`

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
   - `GUILD_ID`

4. Run the bot:

   ```bash
   cargo run
   ```

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

- Every destructive mutation pushes the previous table into `data/history.json`
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

## Current Seeded League Table

| POS | CLUB | PL | W | D | L | GD | PTS |
|---:|---|---:|---:|---:|---:|---:|---:|
| 1 | CHELSEA | 18 | 9 | 5 | 4 | 6 | 32 |
| 2 | PSG | 18 | 9 | 3 | 6 | 19 | 31 |
| 3 | BARCELONA | 18 | 10 | 1 | 7 | 15 | 31 |
| 4 | NEWCASTLE | 17 | 9 | 2 | 6 | 13 | 29 |
| 5 | CELTIC | 17 | 8 | 3 | 6 | 10 | 27 |
| 6 | DORTMUND | 18 | 8 | 2 | 8 | -2 | 26 |
| 7 | MAN UNITED | 18 | 7 | 4 | 7 | -9 | 25 |
| 8 | REAL SALT LAKE | 18 | 6 | 5 | 7 | -9 | 23 |
| 9 | SPORTING CP | 18 | 4 | 3 | 11 | -18 | 15 |
| 10 | MALMO FF | 16 | 3 | 2 | 11 | -25 | 11 |
