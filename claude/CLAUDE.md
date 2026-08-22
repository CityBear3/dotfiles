# CLAUDE.md — dotfiles repo

This repository is the source of the user-global Claude Code configuration.
This file carries repo-specific rules only — the global behavior rules live in
`claude/CLAUDE.global.md`, distributed to `~/.claude/CLAUDE.md` by `claude/install.sh`.

- Edit configuration on the dotfiles side only, then run `claude/install.sh` to
  sync to `~/.claude/`. Never edit `~/.claude/` directly.
- `claude/install.sh` distributes: `CLAUDE.global.md` (as `~/.claude/CLAUDE.md`),
  `skills/`, `agents/`, `statusline.toml`, and builds/installs the Rust statusline.
  It manages only repository names (recorded in `~/.claude/.dotfiles-managed`);
  skills, agents, hooks, and plugins from other tools are never removed.
  `claude/install.sh --dry-run` prints the planned actions.
- The Rust statusline lives in `claude/statusline/` — `cargo test / clippy / fmt`
  apply there.
