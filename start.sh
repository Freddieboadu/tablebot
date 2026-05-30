#!/usr/bin/env bash
#
# Starts the PBL Table Bot together with a Cloudflare quick tunnel.
#
#   ./start.sh
#
# It will:
#   1. Stop any existing bot / tunnel processes.
#   2. Start a Cloudflare quick tunnel to the local web server.
#   3. Capture the generated public URL and write it into .env (WEB_BASE_URL).
#   4. Build and start the bot.
#   5. Print the shareable link.
#
# Logs:  /tmp/bot.log  and  /tmp/cf-tunnel.log
# Stop:  ./stop.sh   (or)   pkill -f pbl-tablebot && pkill cloudflared

set -euo pipefail

cd "$(dirname "$0")"

PORT="${WEB_PORT:-8080}"
BOT_LOG="/tmp/bot.log"
TUNNEL_LOG="/tmp/cf-tunnel.log"

echo "==> Stopping any existing bot / tunnel..."
pkill -f "target/debug/pbl-tablebot" 2>/dev/null || true
pkill -f "target/release/pbl-tablebot" 2>/dev/null || true
pkill -f "cargo run" 2>/dev/null || true
pkill -x cloudflared 2>/dev/null || true
sleep 1

echo "==> Building bot..."
cargo build

echo "==> Starting Cloudflare tunnel on port ${PORT}..."
: > "$TUNNEL_LOG"
nohup cloudflared tunnel --url "http://localhost:${PORT}" > "$TUNNEL_LOG" 2>&1 &

# Wait for the public URL to appear in the log.
URL=""
for _ in $(seq 1 30); do
  URL="$(grep -oE 'https://[a-z0-9-]+\.trycloudflare\.com' "$TUNNEL_LOG" | head -1 || true)"
  [ -n "$URL" ] && break
  sleep 1
done

if [ -z "$URL" ]; then
  echo "!! Could not detect tunnel URL. Check $TUNNEL_LOG"
  exit 1
fi

echo "==> Tunnel URL: $URL"

echo "==> Updating WEB_BASE_URL in .env..."
if grep -q '^WEB_BASE_URL=' .env; then
  # Replace existing line (use | as sed delimiter since URL has slashes).
  sed -i "s|^WEB_BASE_URL=.*|WEB_BASE_URL=${URL}|" .env
else
  printf '\nWEB_BASE_URL=%s\n' "$URL" >> .env
fi

echo "==> Starting bot..."
: > "$BOT_LOG"
nohup cargo run > "$BOT_LOG" 2>&1 &

# Wait for the web server to come up.
for _ in $(seq 1 30); do
  grep -q "Web server listening" "$BOT_LOG" && break
  sleep 1
done

echo ""
echo "============================================================"
echo "  PBL Table Bot is running."
echo "  Share this link:  $URL"
echo "============================================================"
echo "  Bot log:    $BOT_LOG"
echo "  Tunnel log: $TUNNEL_LOG"
echo "  Stop with:  ./stop.sh"
echo ""
