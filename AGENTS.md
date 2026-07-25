# Dotfiles repository guidance

対話と作業報告は日本語で行います。

## Ownership

- `claude/` is the source of truth for Claude Code assets.
- `codex/` is the source of truth for Codex assets.
- Do not make a Codex asset import or read from `claude/` at runtime. Claude assets may be consulted only while deliberately porting behavior.
- Personal Codex skills install under `~/.agents/skills`; custom agents and global guidance install under `~/.codex`.
- Never manage or prune `~/.codex/skills/.system`.

## Engineering workflow

- The engineer owns architecture, scope, and design decisions. Implement only an approved plan or an explicitly requested change.
- Use test-driven development for production-code behavior changes: establish a failing test, make the smallest implementation pass, then refactor.
- Prefer one writer for a shared worktree. Reviewers inspect and report without editing unless explicitly assigned a fix.
- Keep unit tests focused on one behavioral viewpoint. Structure them as Arrange, Act, Assert; prefer DAMP clarity over DRY test abstraction.
- Use integration tests for public-crate, multi-component, or real process journeys, not merely because a unit touches the filesystem.

## Completion

- Review changed assets against their declared inventory, schema, references, and destination mapping.
- Do not claim completion without fresh, observed verification evidence.
- Preserve unrelated user files and configuration.
