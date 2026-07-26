# Codex configuration

This directory is the source of truth for the personal Codex bundle.

## Rust installer

Use `install.sh` as the normal entry point. It resolves the installer manifest relative to its own location, so it can be invoked from any working directory.

```sh
# Preview the default install.
./codex/install.sh --dry-run

# Install. The explicit `install` subcommand is optional.
./codex/install.sh install
./codex/install.sh

# Restore the selected pre-install backup.
./codex/install.sh restore
```

Dry-run computes and renders an install plan without taking the operation lock or creating destination or state files. The launcher uses `cargo run --quiet --locked --release` on every invocation instead of installing or copying a standalone binary.

### Adopting existing assets

An existing same-name guidance file, skill, or agent is a conflict until the installer owns it. Review the dry-run and use `--adopt-existing` for the first intentional adoption:

```sh
./codex/install.sh --dry-run --adopt-existing
./codex/install.sh --adopt-existing
```

A successful install writes the ownership manifest. Later installs use that manifest to replace or remove only installer-owned names, so `--adopt-existing` is no longer needed for those assets.

### Agent concurrency

`--agent-threads` accepts `auto` or an integer from `2` through `32`; `auto` is the default. The selected value is written to `agents.max_threads`.

Automatic selection uses logical CPU count and physical memory:

- 4 threads when there are fewer than 8 logical CPUs or less than 16 GiB of memory.
- 8 threads when there are at least 12 logical CPUs and at least 32 GiB of memory.
- 6 threads otherwise.

### Roots and managed destinations

Command-line roots override these defaults:

| Root | Default |
|---|---|
| Codex home (`--codex-home`) | `$CODEX_HOME` when set and non-empty; otherwise `$HOME/.codex` |
| Personal skills (`--skills-home`) | `$HOME/.agents/skills` |
| Installer state (`--state-dir`) | `$XDG_STATE_HOME/dotfiles-codex-installer` when set and non-empty; otherwise `$HOME/.local/state/dotfiles-codex-installer` |

| Repository source | Personal destination |
|---|---|
| `AGENTS.global.md` | `<codex-home>/AGENTS.md` |
| `agents/<name>.toml` | `<codex-home>/agents/<name>.toml` |
| `skills/<name>/` | `<skills-home>/<name>/` |
| `config.toml` | Five managed values merged into `<codex-home>/config.toml` |

The five managed configuration values are `model`, `model_reasoning_effort`, `plan_mode_reasoning_effort`, `agents.max_threads`, and `agents.max_depth`. Other configuration bytes—including comments, statusline, context-window and auto-compact settings, MCP configuration, permissions, authentication, and providers—are preserved.

The installer manages only declared or manifest-owned names. Unrelated sibling skills and agents are preserved. `.system` cannot be installer-owned or pruned; in particular, `<codex-home>/skills/.system` is outside the destination mapping.

### Locking, state, backups, and recovery

Mutating `install` and `restore` commands serialize through `<codex-home>/codex-manifest-installer.lock`. This is a persistent empty lock file. Dry-run does not create or acquire it.

The state directory contains:

| Path | Meaning |
|---|---|
| `manifest-v1.json` | Names currently owned by the installer |
| `transaction/wal-v1.json` | Canonical write-ahead log for an unfinished mutation |
| `transaction/work/<operation-id>/` | Staged content and tombstones used by that transaction |
| `backups/latest` | The ID of the selected restore backup |
| `backups/<backup-id>/journal-v1.json` | Immutable backup metadata, roots, ownership, and fingerprints |
| `backups/<backup-id>/payload/` | Captured pre-install content |

At the start of a mutating command, the installer automatically recovers or finalizes an unfinished transaction before planning new work. There is no manual `recover` command.

An install with live mutations captures its pre-install managed state, commits the live changes, selects that backup through `backups/latest`, and removes older unselected backups during successful cleanup. Restore accepts only the selected latest backup; it does not accept an arbitrary backup path. After a successful restore, that same backup remains selected, no replacement backup is promoted, and successful cleanup retains only the selected backup directory. A completed transaction leaves no canonical WAL and no operation work tree.

## Manual installer fallback

Use this procedure only when the Rust installer cannot be built or run. It does not merge `config.toml`. Run the complete block from the dotfiles repository root.

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

The fallback intentionally does not copy `config.toml`, because replacing the live file would destroy unmanaged device-specific settings. When using the fallback, review the live `~/.codex/config.toml` and update only the five managed keys declared in this repository fragment. Never copy the fragment over the entire live file.
