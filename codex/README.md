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
