---
name: commit
description: Create a git commit with a properly formatted message. Analyzes staged and unstaged changes, then creates a commit following the project's conventions.
argument-hint: "[optional message hint]"
---

# Commit

Create a git commit with a properly formatted message.

## Input

`$ARGUMENTS` optionally provides a hint about the commit's intent. If omitted, infer from the changes.

## Execution

### Step 1: Analyze Changes

Run the following in parallel:
1. `git status` — check for staged and unstaged changes
2. `git diff --cached` and `git diff` — see what will be committed
3. `git log --oneline -5` — see recent commit style for consistency

If there are no changes to commit, inform the user and stop.

### Step 2: Stage Files

If there are unstaged changes that should be included:
- Ask the user which files to stage, or stage all if the intent is clear
- Prefer adding specific files by name over `git add -A`
- Never stage files that likely contain secrets (.env, credentials, etc.)

### Step 3: Draft Commit Message

Format:

```
<Prefix>: <Title>

<Description>
```

**Prefix** — one of the following:
- `Feat` — new feature or capability
- `Add` — adding a new file, dependency, or resource (not a feature)
- `Fix` — bug fix
- `Refactor` — code restructuring without behavior change
- `Update` — enhancement or modification to existing feature
- `Remove` — deletion of code, files, or features
- `Docs` — documentation only
- `Test` — adding or modifying tests
- `Chore` — build, CI, tooling, or other maintenance

**Title** — concise summary (under 50 characters if possible), imperative mood, no period

**Description** — one or more lines explaining the "why" behind the change, not the "what". Wrap at 72 characters. May be omitted for trivial changes.

### Step 4: Confirm and Commit

Present the draft message to the user. After approval, create the commit.

Use a HEREDOC to pass the message:

```bash
git commit -m "$(cat <<'EOF'
<Prefix>: <Title>

<Description>

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>
EOF
)"
```

Always append the Co-Authored-By trailer.

## Rules

- Never commit without showing the message to the user first
- Never use `git add -A` or `git add .` without user confirmation
- Never commit files that likely contain secrets
- If a pre-commit hook fails, fix the issue and create a NEW commit (do not amend)
- Language of the commit message: English