---
name: finish-branch
description: |
  Complete development work on a branch. Verifies tests pass, presents structured options
  (create PR, merge locally, keep as-is, discard), and handles cleanup.
  Invoke with `/finish-branch` after review is complete.
---

# Finish Branch

Guide completion of development work by presenting clear options and handling the chosen workflow.

**Announce at start:** "I'm using the finish-branch skill to complete this work."

**Core principle:** Verify tests → Present options → Execute choice → Clean up.

## Entry Conditions

- `/review` (the skill) has run to completion and reported clean — no Must Fix / Should Improve
- Or the engineer explicitly decides to finish the branch at any point (the entry guard below does not apply)

**Entry guard (automatic transition only):** when entered via the Core Flow's clean-review automatic transition, verify there is evidence of a clean `/review` completion since the latest commit: the most recent `/review` report in this session shows zero Must Fix / Should Improve, and no commit has been made after that report. If fixes were committed after the last `/review` report — or no `/review` report exists — do not proceed: return to `/verify` (the loop continues `/verify` → `/review` → back here). Agent-teams internal reviewer approval (spec-reviewer / code-quality-reviewer) is NOT such evidence.

## The Process

### Step 1: Verify Tests

Before presenting options, verify tests pass:

```bash
# Run project's test suite (check project CLAUDE.md for specific commands)
```

If tests fail:
```
Tests failing (<N> failures). Must fix before completing:
[Show failures]
Cannot proceed with merge/PR until tests pass.
```
Stop. Do not proceed to Step 2.

If tests pass: Continue to Step 2.

### Step 2: Determine Base Branch

```bash
git merge-base HEAD main 2>/dev/null || git merge-base HEAD master 2>/dev/null
```

Or ask: "This branch split from main — is that correct?"

### Step 3: Present Options

Present exactly these 4 options:

```
Implementation complete. What would you like to do?

1. Create a Pull Request
2. Merge back to <base-branch> locally
3. Keep the branch as-is (I'll handle it later)
4. Discard this work

Which option?
```

Do not add explanation — keep options concise.

### Step 4: Execute Choice

#### Option 1: Create a Pull Request

→ Invoke `create-pr` skill.

#### Option 2: Merge Locally

In a worktree session, `git checkout <base-branch>` is impossible — the base
branch is checked out in the main checkout. Operate on the main checkout via
`-C` instead:

```bash
main_root=$(dirname "$(git rev-parse --path-format=absolute --git-common-dir)")
git -C "$main_root" pull
git -C "$main_root" merge <feature-branch>
# Verify tests on the merged result (run them in $main_root)
```

The feature branch cannot be deleted while this worktree has it checked out.
Report instead:

```
Merged into <base-branch>. After you remove this workspace
(herdr worktree remove), delete the branch with `git branch -d <feature-branch>`.
```

(On a plain feature branch — no worktree — the classic sequence applies:
`git checkout <base-branch> && git pull && git merge <feature-branch>`,
verify tests, then `git branch -d <feature-branch>`.)

#### Option 3: Keep As-Is

Report: "Keeping branch `<name>`. You can return to it later."

#### Option 4: Discard

**Confirm first:**
```
This will permanently delete:
- Branch <name>
- All commits: <commit-list>

Type 'discard' to confirm.
```

Wait for exact confirmation. If confirmed, in a worktree session Claude
deletes nothing — the branch is checked out here. Report instead
(`$HERDR_WORKSPACE_ID` carries this session's workspace ID):

```
To discard: remove this workspace
  herdr worktree remove --workspace $HERDR_WORKSPACE_ID --force
then delete the branch from the main checkout:
  git branch -D <feature-branch>
```

(On a plain feature branch — no worktree:
`git checkout <base-branch> && git branch -D <feature-branch>` as before.)

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Proceeding with failing tests | Stop. Fix tests first. |
| Merging without verifying tests on the merged result | Run tests after merge before reporting success. |
| Deleting work without confirmation | Require typed "discard" confirmation for Option 4. |
| Auto-selecting an option | Always present the 4 options. The engineer chooses. |
| Force-pushing without explicit request | Never force-push unless the engineer explicitly asks. |

## Rules

- Always verify tests before offering options
- Present exactly 4 options — no more, no less
- Get typed confirmation for discard (Option 4)
- For Option 1, delegate to `create-pr` skill — do not duplicate its logic
- For Option 2, verify tests pass on the merged result before deleting the feature branch

## Transition

After the chosen option (1–4) completes — including after `create-pr` returns for Option 1 — hand off to wrap up the session:

→ Transition to `/session-teardown` to best-effort shut down the agent-teams team and prompt the engineer to end the session.

This runs after the option's git cleanup; it does not add a 5th menu option.