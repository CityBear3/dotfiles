# Codex configuration

This directory is the source of truth for the personal Codex bundle.

## Installation mapping

| Repository source | Personal destination |
|---|---|
| `AGENTS.global.md` | `~/.codex/AGENTS.md` |
| `agents/<name>.toml` | `~/.codex/agents/<name>.toml` |
| `skills/<name>/` | `~/.agents/skills/<name>/` |
| `config.toml` | Partially merged into `~/.codex/config.toml` by the Rust installer |

The bootstrap and installer manage only declared names. They do not prune unrelated personal skills or agents.

`~/.codex/skills/.system` is owned by Codex and is never a destination, inventory root, or deletion target for this repository.

The repository `config.toml` intentionally contains only the managed model and agent-capacity values. Context Window, Auto Compact, statusline, MCP, permissions, authentication, providers, and other device-specific settings remain untouched.

## Temporary bootstrap and installer fallback

Use this procedure until the Rust installer is complete, or when the installer cannot be built or run. Run the complete block from the dotfiles repository root.

Prerequisites:

- `~/.codex` already exists because Codex is installed.
- `install` and `rsync` are available.
- Any local edits inside same-name managed skills or agents have been reviewed or backed up.

```sh
set -euo pipefail

codex_source="$PWD/codex"
codex_home="${CODEX_HOME:-$HOME/.codex}"

command -v install >/dev/null
command -v rsync >/dev/null
test -d "$codex_home"
test -f "$codex_source/AGENTS.global.md"
test -d "$codex_source/agents"
test -d "$codex_source/skills"

mkdir -p "$codex_home/agents" "$HOME/.agents/skills"

install -m 0644 \
  "$codex_source/AGENTS.global.md" \
  "$codex_home/AGENTS.md"

for source in "$codex_source"/agents/*.toml; do
  install -m 0644 \
    "$source" \
    "$codex_home/agents/$(basename "$source")"
done

for source in "$codex_source"/skills/*; do
  destination="$HOME/.agents/skills/$(basename "$source")"
  mkdir -p "$destination"
  rsync -a --delete "$source/" "$destination/"
done
```

The `rsync --delete` boundary is each individual same-name skill directory. It removes files that disappeared from a managed skill, but it does not delete unrelated sibling skills. Agent files are installed one by one, so unrelated agent profiles are also preserved.

This procedure never targets `~/.codex/skills/.system`, the customized statusline, or `~/.codex/config.toml`.

### Verify the applied assets

Run from the repository root:

```sh
set -euo pipefail

codex_source="$PWD/codex"
codex_home="${CODEX_HOME:-$HOME/.codex}"

cmp "$codex_source/AGENTS.global.md" "$codex_home/AGENTS.md"

for source in "$codex_source"/agents/*.toml; do
  cmp "$source" "$codex_home/agents/$(basename "$source")"
done

for source in "$codex_source"/skills/*; do
  diff -qr \
    "$source" \
    "$HOME/.agents/skills/$(basename "$source")"
done

test -d "$codex_home/skills/.system"
```

No output from `cmp` and `diff` means the managed installed assets match the repository sources.

### Configuration limitation

The temporary procedure intentionally does not copy `config.toml`, because replacing the live file would destroy unmanaged device-specific settings. Until the Rust installer can perform a partial TOML merge, review the live `~/.codex/config.toml` and update only the keys declared in this repository fragment when a configuration change is required. Never copy the fragment over the entire live file.
