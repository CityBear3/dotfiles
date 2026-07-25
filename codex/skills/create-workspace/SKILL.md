---
name: create-workspace
description: Verify or establish a feature workspace using a non-default Git branch or a herdr-managed worktree. Use before implementation when repository policy requires isolated feature work.
---

# Create or verify a workspace

Keep one feature in one branch/worktree and one Codex session.

## Inspect

Run read-only checks:

- `git rev-parse --show-toplevel`
- `git branch --show-current`
- `git worktree list --porcelain`
- `git status --short`

Read repository guidance for worktree policy.

## Already isolated

If the current checkout is on the intended non-default branch or already resides in the requested herdr worktree, report the path and branch and continue. Do not create a nested worktree.

## Main checkout

When implementation would start on `main`, `master`, or another default branch:

1. derive a short feature branch name from the approved scope;
2. report any dirty changes that would not follow into a new worktree;
3. ask the user before creating and switching the session to a new workspace;
4. when approved and available, run `herdr worktree create <branch>`;
5. report the created path and ask the user to reopen the session there.

Do not move, stash, discard, or copy dirty changes without explicit approval. Do not remove worktrees in this skill.
