#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TARGET_DIR="$HOME/.claude"

echo "Installing Claude Code configuration..."
echo "  Source: $SCRIPT_DIR"
echo "  Target: $TARGET_DIR"

mkdir -p "$TARGET_DIR"

# CLAUDE.md
cp "$SCRIPT_DIR/CLAUDE.md" "$TARGET_DIR/CLAUDE.md"
echo "  Copied CLAUDE.md"

# skills/
rsync -a --delete "$SCRIPT_DIR/skills/" "$TARGET_DIR/skills/"
echo "  Synced skills/"

# agents/
rsync -a --delete "$SCRIPT_DIR/agents/" "$TARGET_DIR/agents/"
echo "  Synced agents/"

echo ""
echo "Done. The following were NOT copied (device-specific):"
echo "  settings.json, projects/, sessions/, history.jsonl, cache/, plugins/, todos/"