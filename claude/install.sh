#!/bin/bash
# Install the dotfiles Claude Code configuration into ~/.claude.
#
# Only names that come from this repository are managed. Skills and agents that
# other tools or the user installed into ~/.claude/skills and ~/.claude/agents
# are never touched. A manifest records which names the last install managed,
# so a skill or agent removed from the repository is removed from the
# destination on the next install, while everything else is preserved.
#
# Usage:
#   claude/install.sh            install
#   claude/install.sh --dry-run  print the planned actions without changing anything
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TARGET_DIR="${CLAUDE_HOME:-$HOME/.claude}"
MANIFEST="$TARGET_DIR/.dotfiles-managed"
DRY_RUN=0

for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=1 ;;
        -h|--help)
            sed -n '2,12p' "$0" | sed 's/^# \{0,1\}//'
            exit 0
            ;;
        *)
            echo "unknown option: $arg" >&2
            exit 2
            ;;
    esac
done

echo "Installing Claude Code configuration..."
echo "  Source: $SCRIPT_DIR"
echo "  Target: $TARGET_DIR"
[ "$DRY_RUN" -eq 1 ] && echo "  (dry run: no changes will be made)"

run() {
    if [ "$DRY_RUN" -eq 1 ]; then
        return 0
    fi
    "$@"
}

# --- Desired managed names (from the repository) -----------------------------

desired_skills=()
for dir in "$SCRIPT_DIR"/skills/*/; do
    [ -d "$dir" ] || continue
    desired_skills+=("skill:$(basename "$dir")")
done

desired_agents=()
for file in "$SCRIPT_DIR"/agents/*.md; do
    [ -f "$file" ] || continue
    desired_agents+=("agent:$(basename "$file")")
done

# --- Previously managed names (from the manifest) ----------------------------

previous=()
if [ -f "$MANIFEST" ]; then
    while IFS= read -r line; do
        [ -n "$line" ] && previous+=("$line")
    done < "$MANIFEST"
else
    echo "  No manifest at $MANIFEST; adopting repository names only, removing nothing."
fi

contains() {
    local needle="$1"; shift
    local item
    for item in "$@"; do
        [ "$item" = "$needle" ] && return 0
    done
    return 1
}

run mkdir -p "$TARGET_DIR" "$TARGET_DIR/skills" "$TARGET_DIR/agents"

# --- Global guidance ----------------------------------------------------------

# CLAUDE.md: the global guidance source is CLAUDE.global.md. The repo-level
# CLAUDE.md is repository-specific and is not distributed.
if [ ! -f "$TARGET_DIR/CLAUDE.md" ]; then
    echo "  CREATE  CLAUDE.md"
elif ! cmp -s "$SCRIPT_DIR/CLAUDE.global.md" "$TARGET_DIR/CLAUDE.md"; then
    echo "  UPDATE  CLAUDE.md"
else
    echo "  NO-OP   CLAUDE.md"
fi
run cp "$SCRIPT_DIR/CLAUDE.global.md" "$TARGET_DIR/CLAUDE.md"

# --- Skills: one rsync per managed skill directory ----------------------------

for entry in ${desired_skills[@]+"${desired_skills[@]}"}; do
    name="${entry#skill:}"
    src="$SCRIPT_DIR/skills/$name/"
    dst="$TARGET_DIR/skills/$name/"
    if [ ! -d "$dst" ]; then
        echo "  CREATE  skills/$name"
    elif [ -n "$(rsync -a --delete --dry-run --itemize-changes "$src" "$dst" | grep -v '^\.d' || true)" ]; then
        echo "  UPDATE  skills/$name"
    else
        echo "  NO-OP   skills/$name"
    fi
    run mkdir -p "$dst"
    # --delete is scoped to this one managed skill directory: it removes files
    # that disappeared from the repository's copy of this skill and nothing else.
    run rsync -a --delete "$src" "$dst"
done

# --- Agents: one file per managed agent ---------------------------------------

for entry in ${desired_agents[@]+"${desired_agents[@]}"}; do
    name="${entry#agent:}"
    src="$SCRIPT_DIR/agents/$name"
    dst="$TARGET_DIR/agents/$name"
    if [ ! -f "$dst" ]; then
        echo "  CREATE  agents/$name"
    elif ! cmp -s "$src" "$dst"; then
        echo "  UPDATE  agents/$name"
    else
        echo "  NO-OP   agents/$name"
    fi
    run install -m 0644 "$src" "$dst"
done

# --- Remove only names the previous install managed and the repo dropped -----

for entry in ${previous[@]+"${previous[@]}"}; do
    if contains "$entry" ${desired_skills[@]+"${desired_skills[@]}"} ${desired_agents[@]+"${desired_agents[@]}"}; then
        continue
    fi
    case "$entry" in
        skill:*)
            name="${entry#skill:}"
            if [ -d "$TARGET_DIR/skills/$name" ]; then
                echo "  REMOVE  skills/$name (no longer in repository)"
                run rm -rf "$TARGET_DIR/skills/$name"
            fi
            ;;
        agent:*)
            name="${entry#agent:}"
            if [ -f "$TARGET_DIR/agents/$name" ]; then
                echo "  REMOVE  agents/$name (no longer in repository)"
                run rm -f "$TARGET_DIR/agents/$name"
            fi
            ;;
    esac
done

# --- Report unmanaged entries (never touched) ---------------------------------

for dir in "$TARGET_DIR"/skills/*/; do
    [ -d "$dir" ] || continue
    name="$(basename "$dir")"
    contains "skill:$name" ${desired_skills[@]+"${desired_skills[@]}"} ${previous[@]+"${previous[@]}"} \
        || echo "  KEEP    skills/$name (not managed by dotfiles)"
done
for file in "$TARGET_DIR"/agents/*.md; do
    [ -f "$file" ] || continue
    name="$(basename "$file")"
    contains "agent:$name" ${desired_agents[@]+"${desired_agents[@]}"} ${previous[@]+"${previous[@]}"} \
        || echo "  KEEP    agents/$name (not managed by dotfiles)"
done

# --- Manifest -----------------------------------------------------------------

if [ "$DRY_RUN" -eq 0 ]; then
    {
        printf '%s\n' ${desired_skills[@]+"${desired_skills[@]}"}
        printf '%s\n' ${desired_agents[@]+"${desired_agents[@]}"}
    } > "$MANIFEST"
    echo "  Wrote manifest $MANIFEST"
fi

# --- statusline.toml (pricing overrides) --------------------------------------

if [ ! -f "$TARGET_DIR/statusline.toml" ]; then
    echo "  CREATE  statusline.toml"
elif ! cmp -s "$SCRIPT_DIR/statusline.toml" "$TARGET_DIR/statusline.toml"; then
    echo "  UPDATE  statusline.toml"
else
    echo "  NO-OP   statusline.toml"
fi
run cp "$SCRIPT_DIR/statusline.toml" "$TARGET_DIR/statusline.toml"

# --- claude-statusline (Rust binary) ------------------------------------------

if [ "$DRY_RUN" -eq 1 ]; then
    echo "  (dry run) would build claude-statusline and install it to ~/.local/bin"
elif command -v cargo >/dev/null 2>&1; then
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
echo "Done. Not managed by this script (never copied or removed):"
echo "  settings.json, hooks/, plugins/, rules/, projects/, sessions/, history.jsonl, cache/, todos/,"
echo "  and any skill or agent under $TARGET_DIR that is not part of this repository."
