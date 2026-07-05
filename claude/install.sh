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

# statusline.toml (単価上書き設定)
cp "$SCRIPT_DIR/statusline.toml" "$TARGET_DIR/statusline.toml"
echo "  Copied statusline.toml"

# claude-statusline (Rust バイナリ)
if command -v cargo >/dev/null 2>&1; then
    echo "  Building claude-statusline (release)..."
    (cd "$SCRIPT_DIR/statusline" && cargo build --release --quiet)
    mkdir -p "$HOME/.local/bin"
    cp "$SCRIPT_DIR/statusline/target/release/claude-statusline" "$HOME/.local/bin/claude-statusline"
    echo "  Installed claude-statusline to ~/.local/bin"
else
    echo "  WARNING: cargo not found; skipped building claude-statusline"
fi

# statusLine 未設定なら貼り付け用スニペットを案内する (settings.json は編集しない)
if ! grep -q '"statusLine"' "$TARGET_DIR/settings.json" 2>/dev/null; then
    cat <<EOF

  statusLine is not configured yet.
  Add this to $TARGET_DIR/settings.json:

    "statusLine": {
      "type": "command",
      "command": "$HOME/.local/bin/claude-statusline"
    }
EOF
fi

echo ""
echo "Done. The following were NOT copied (device-specific):"
echo "  settings.json, projects/, sessions/, history.jsonl, cache/, plugins/, todos/"
