#!/usr/bin/env bash
#
# Stops the PBL Table Bot and the Cloudflare tunnel.
#
echo "==> Stopping bot and tunnel..."
pkill -f "target/debug/pbl-tablebot" 2>/dev/null || true
pkill -f "target/release/pbl-tablebot" 2>/dev/null || true
pkill -f "cargo run" 2>/dev/null || true
pkill -x cloudflared 2>/dev/null || true
echo "Done."
