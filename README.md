# 🏆 PBL TableBot

A fully-functional Discord bot for managing the **PBL (Pro Bot League)** football/soccer league table. Built with [Discord.js v14](https://discord.js.org/) and slash commands.

---

## ✨ Features

| Command | Description |
|---|---|
| `/table` | Display the full league table as a formatted embed |
| `/update` | Record a match result and update both teams' stats |
| `/revert` | Undo the last `/update` (Admin only) |
| `/addteam` | Add a new team to the table (Admin only) |
| `/deleteteam` | Remove a team with confirmation (Admin only) |

---

## 🚀 Setup Instructions

### 1. Clone the repository
```bash
git clone https://github.com/Freddieboadu/tablebot.git
cd tablebot
```

### 2. Install dependencies
```bash
npm install
```

### 3. Configure environment variables
```bash
cp .env.example .env
```

Open `.env` and fill in your credentials:

```env
DISCORD_TOKEN=your_bot_token_here
CLIENT_ID=your_application_client_id_here
GUILD_ID=your_guild_id_here   # optional — omit for global deploy
```

- **DISCORD_TOKEN** — Found in the [Discord Developer Portal](https://discord.com/developers/applications) under your app → Bot → Token
- **CLIENT_ID** — Your application's Client ID (General Information tab)
- **GUILD_ID** — Your server's ID (right-click server icon → Copy Server ID). When set, commands deploy instantly to that server. Omit for global deployment (takes up to 1 hour).

### 4. Deploy slash commands
```bash
npm run deploy
# or: node deploy-commands.js
```

### 5. Start the bot
```bash
npm start
# or: node index.js
```

---

## 🔗 Add the Bot to Your Server

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications)
2. Select your application → **OAuth2** → **URL Generator**
3. Scopes: ✅ `bot` ✅ `applications.commands`
4. Bot Permissions: ✅ `Send Messages` ✅ `Use Slash Commands` ✅ `Embed Links`
5. Copy the generated URL and open it in your browser to invite the bot

---

## 📋 Command Reference

### `/table`
Displays the current league table sorted by PTS → GD → W.

```
/table
```

### `/update`
Record a match result. Updates PL, W, D, L, GD, and PTS for both teams.

```
/update home_team:CHELSEA home_score:2 away_team:PSG away_score:1
/update home_team:man united home_score:3 away_team:celtic away_score:3 force:True
```

**Options:**
- `home_team` — Home team name (case-insensitive)
- `home_score` — Goals scored by the home team (non-negative integer)
- `away_team` — Away team name (case-insensitive)
- `away_score` — Goals scored by the away team (non-negative integer)
- `force` — (optional) Override a duplicate fixture warning

**Rate limit:** 1 update per 5 seconds per user.

### `/revert` *(Admin only)*
Undo the last `/update` command. Restores the table to its previous state.

```
/revert
```

- Keeps up to **20** history entries
- Shows what result was undone and who reverted it
- Also reverts `/addteam` and `/deleteteam` actions (team additions/deletions are saved to history)

### `/addteam` *(Admin only)*
Add a new team to the table starting with all-zero stats.

```
/addteam team_name:ARSENAL
```

### `/deleteteam` *(Admin only)*
Remove a team from the table. Shows a confirmation prompt (Yes/No buttons) before deleting.

```
/deleteteam team_name:MALMO FF
```

> ⚠️ The team's historical match data remains in the revert history and can be restored with `/revert`.

---

## 🔐 Permission Requirements

| Command | Permission Required |
|---|---|
| `/table` | None (any user) |
| `/update` | None (any user) |
| `/revert` | **Manage Server** |
| `/addteam` | **Manage Server** |
| `/deleteteam` | **Manage Server** |

---

## ↩️ How Revert Works

Every time `/update`, `/addteam`, or `/deleteteam` runs, a **snapshot of the entire table** is saved to `data/history.json` before the change is applied. The revert stack:

- Holds up to **20 entries** (oldest are automatically removed)
- Each entry stores: full table snapshot, a description of what changed, and a timestamp
- `/revert` pops the most recent snapshot and restores the table to that state
- Reverting after a `/deleteteam` **brings the deleted team back**
- Reverting after an `/addteam` **removes the added team**

---

## 📊 Current Seeded League Table

| POS | CLUB | PL | W | D | L | GD | PTS |
|---|---|---|---|---|---|---|---|
| 🏆 1 | CHELSEA | 18 | 9 | 5 | 4 | +6 | 32 |
| 2 | PSG | 18 | 9 | 3 | 6 | +19 | 31 |
| 3 | BARCELONA | 18 | 10 | 1 | 7 | +15 | 31 |
| 4 | NEWCASTLE | 17 | 9 | 2 | 6 | +13 | 29 |
| 5 | CELTIC | 17 | 8 | 3 | 6 | +10 | 27 |
| 6 | DORTMUND | 18 | 8 | 2 | 8 | -2 | 26 |
| 7 | MAN UNITED | 18 | 7 | 4 | 7 | -9 | 25 |
| 8 | REAL SALT LAKE | 18 | 6 | 5 | 7 | -9 | 23 |
| 9 | SPORTING CP | 18 | 4 | 3 | 11 | -18 | 15 |
| 10 | MALMO FF | 16 | 3 | 2 | 11 | -25 | 11 |

---

## 🗂️ File Structure

```
tablebot/
├── index.js              # Bot entry point, command loader, rate limiting
├── deploy-commands.js    # Register slash commands with Discord API
├── package.json
├── .env.example          # Environment variable template
├── README.md
├── data/
│   ├── table.json        # Current league table (auto-created if missing)
│   └── history.json      # Revert history stack (auto-created if missing)
├── commands/
│   ├── table.js          # /table
│   ├── update.js         # /update
│   ├── revert.js         # /revert
│   ├── addteam.js        # /addteam
│   └── deleteteam.js     # /deleteteam
└── utils/
    ├── tableUtils.js     # sort, recalculate positions, format table, read/write
    ├── validator.js      # input validation helpers
    └── history.js        # push/pop history stack
```

---

## ⚙️ Edge Cases Handled

- **Score validation** — Scores must be non-negative integers (enforced at the Discord API level and in code)
- **Team not found** — Clear error message with spelling hint
- **Duplicate fixture** — Warning displayed; override with `force:True`
- **Team plays itself** — Rejected with an error
- **Case-insensitive matching** — All team lookups are case-insensitive
- **Tiebreaker order** — PTS → GD → W → alphabetical
- **Rate limiting** — 1 `/update` per 5 seconds per user
- **Missing data files** — Bot recreates them on startup
- **Atomic file writes** — Writes to a temp file then renames to avoid partial writes
- **Revert after deletion** — Restoring a deleted team is fully supported
- **History cap** — Maximum 20 history entries; oldest are dropped automatically
- **Button confirmation timeout** — `/deleteteam` confirmation expires after 30 seconds
