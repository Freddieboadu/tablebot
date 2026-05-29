# 🏆 PBL League Table Bot

A Discord bot for managing your FIFA Pro Clubs league table. Enter match results, track standings, view form guides and head-to-head records — all without leaving Discord.

---

## 📋 For New Users — How to Use the Bot

> **Copy and paste any of these commands directly into Discord!**

---

### 👋 First Time? Start Here

Type this to see all available commands:
```
/help
```

---

### 📊 Viewing the Table

See the full league standings:
```
/table
```

---

### ⚽ Entering a Match Result

After every game, enter the result using:
```
/update home_team:CHELSEA home_score:2 away_team:PSG away_score:1
```
> Just replace the team names and scores with your actual result. The table updates automatically!

---

### 📈 Checking a Team's Form

See a team's last 5 results (Wins, Draws, Losses):
```
/form team_name:CHELSEA
```
Example output: `🟢 W  🟢 W  🔴 L  🟡 D  🟢 W`

---

### 📋 Recent Results

See the last 10 match results entered:
```
/fixtures
```

---

### ⚔️ Head-to-Head Record

See all results between two specific teams:
```
/head2head team1:CHELSEA team2:PSG
```

---

### ↩️ Made a Mistake?

Undo the last change to the table *(admin only)*:
```
/revert
```

---

## 🔐 For League Admins

> These commands require the **league admin role** or **Manage Server** permission.

---

### ➕ Adding Teams

Add one team:
```
/addteam teams:CHELSEA
```

Add multiple teams at once (separated by commas):
```
/addteam teams:CHELSEA, PSG, BARCELONA, CELTIC, DORTMUND
```

---

### ❌ Removing Teams

Remove one team:
```
/deleteteam teams:CHELSEA
```

Remove multiple teams at once:
```
/deleteteam teams:CHELSEA, PSG
```
> You will be asked to confirm before anything is deleted.

---

### 🗑️ Clear the Entire Table

Wipe the table and start completely fresh:
```
/cleartable
```
> You will be asked to confirm. This can be undone with `/revert`.

---

### 🔐 Set the Admin Role

Restrict table editing to a specific role:
```
/setadminrole role:@LeagueAdmin
```
> Anyone without this role (or Manage Server) will be blocked from making changes.

---

### 📢 Set a Log Channel

Automatically post every table change to a channel:
```
/setlogchannel channel:#league-logs
```
> Every `/update`, `/revert`, `/addteam` etc. will be posted here automatically.

---

## 🏁 New Season Setup (Step by Step)

Follow these steps at the start of every new season:

**Step 1 — Clear the old table**
```
/cleartable
```

**Step 2 — Add all your teams**
```
/addteam teams:CHELSEA, PSG, BARCELONA, CELTIC, DORTMUND, MAN UNITED
```

**Step 3 — Verify the table looks right**
```
/table
```

**Step 4 — You're ready! Enter results after every game**
```
/update home_team:CHELSEA home_score:3 away_team:PSG away_score:1
```

---

## 📖 Full Command Reference

| Command | What it does | Admin only? |
|---------|-------------|-------------|
| `/table` | Show the league table | ❌ |
| `/form team_name:X` | Show a team's last 5 results | ❌ |
| `/fixtures` | Show last 10 results | ❌ |
| `/head2head team1:X team2:Y` | Show H2H record between 2 teams | ❌ |
| `/help` | List all commands | ❌ |
| `/update home_team:X home_score:N away_team:Y away_score:N` | Enter a result | ✅ |
| `/revert` | Undo last change | ✅ |
| `/addteam teams:X, Y, Z` | Add teams (comma separated) | ✅ |
| `/deleteteam teams:X, Y` | Delete teams (comma separated) | ✅ |
| `/cleartable` | Wipe the table | ✅ |
| `/setadminrole role:@Role` | Set who can edit the table | ✅ |
| `/setlogchannel channel:#channel` | Set log channel | ✅ |

---

## ↩️ Revert System

- Every change to the table is saved automatically before it happens
- `/revert` undoes the most recent change
- Up to **20 previous states** are stored per server
- Works after `/update`, `/cleartable`, `/addteam`, `/deleteteam`

---

## 🌍 Multi-Server Support

Each Discord server has its own completely independent table, history, and settings. Data is never shared between servers.

---

## 🛠️ Self-Hosting Setup (For Developers)

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) installed
- A Discord bot token from the [Discord Developer Portal](https://discord.com/developers/applications)

### Installation

**1. Clone the repo**
```bash
git clone https://github.com/Freddieboadu/tablebot.git
cd tablebot
```

**2. Create your `.env` file**
```bash
cp .env.example .env
```

**3. Fill in your `.env`**
```env
DISCORD_TOKEN=your_bot_token_here
CLIENT_ID=your_application_client_id_here
```

**4. Run the bot**
```bash
cargo run
```
You should see: `✅ PBL TableBot is online!`

**5. Invite the bot to your server**

Paste this in your browser (replace `YOUR_CLIENT_ID`):
```
https://discord.com/oauth2/authorize?client_id=YOUR_CLIENT_ID&permissions=277025508352&scope=bot+applications.commands
```

> ⏱️ Slash commands can take up to **1 hour** to appear after first launch. This is a Discord limitation.

---

### Getting Your Bot Token

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Click your application → **Bot** tab
3. Click **Reset Token** and copy it
4. Paste it as `DISCORD_TOKEN` in your `.env`
5. Your **Application ID** on the General Information page = `CLIENT_ID`

> ⚠️ Never share your token or commit it to GitHub!

---

## 🖥️ Keeping the Bot Online 24/7

The bot only runs while your terminal is open. To keep it always on:

| Platform | Cost | How |
|----------|------|-----|
| [Railway.app](https://railway.app) | Free tier | Connect GitHub repo → auto-deploys on every push |
| [Render.com](https://render.com) | Free tier | Similar to Railway |
| VPS (Hetzner/DigitalOcean) | ~$5/mo | Full control |

**Railway is recommended** — it redeploys automatically every time you merge changes.
