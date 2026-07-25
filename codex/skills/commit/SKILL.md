---
name: commit
description: Create a focused Git commit from the intended working-tree changes, following repository conventions. Use when the user or an approved plan asks Codex to commit completed work.
---

# Commit

Create one reviewable commit for one coherent change.

## Inspect

Run:

1. `git status --short`
2. `git diff --cached`
3. `git diff`
4. `git log --oneline -5`

Stop when there is nothing to commit. Preserve unrelated changes and never stage likely secrets.

## Select the commit scope

- Follow an approved plan's explicit file list and message when present.
- Otherwise stage exact paths that form one coherent change.
- Do not use `git add .` or `git add -A` when unrelated changes exist.
- If ownership of a changed file cannot be determined from the request, plan, or history, ask before staging it.

## Verify

Run the validation required by the repository or approved plan before committing. Inspect the staged diff and run `git diff --cached --check`.

## Commit

Use the repository's established message style. Prefer an imperative subject that describes the outcome and a short body only when the reason is not obvious.

Do not amend, force, push, or publish unless the user explicitly requests that action.

Report the commit hash, subject, included scope, and fresh verification evidence.
