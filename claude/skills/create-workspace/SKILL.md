---
name: create-workspace
description: |
  Ensure feature work runs in its own herdr workspace (a git worktree under
  ~/.herdr/worktrees). Verifies and sets up the workspace when the session is
  already inside one; when the session started in the main checkout, asks the
  engineer, creates the workspace via `herdr worktree create`, and guides the
  engineer to reopen there. Invoke with `/create-workspace`
  (called from /design-discussion and /execute-plan).
---

# Create Workspace

AE feature work runs in per-feature workspaces: one feature = one git worktree
under `~/.herdr/worktrees/<repo>/<branch>` = one herdr workspace = one Claude
session, from `/design-discussion` through `/finish-branch`. This skill ensures
that state.

**Core principle:** Claude runs the herdr commands; the engineer opens and
closes sessions.

**Announce at start:** "I'm using the create-workspace skill to set up the feature workspace."

## Scope

- Covers AE feature-work workspaces only. Worktrees Claude spawns autonomously
  (subagent `isolation: "worktree"`, EnterWorktree's managed location) stay
  harness-managed under `.claude/worktrees` and are not affected by this skill.
- Workspace removal is the engineer's manual operation
  (`herdr worktree remove`). No skill removes workspaces or deletes branches.

## Step 1: Detect Session Location

```bash
common=$(git rev-parse --path-format=absolute --git-common-dir)
toplevel=$(git rev-parse --show-toplevel)
```

| Result | Meaning | Next |
|---|---|---|
| `$common` ≠ `$toplevel/.git` | Linked worktree — feature workspace | Step 2 (verify & set up) |
| `$common` = `$toplevel/.git` | Main checkout — launchpad session | Step 3 (ask, then create) |

## Step 2: In a Worktree — Verify and Set Up

1. Confirm the branch: `git branch --show-current` must not be main/master.
2. Run project setup (auto-detect):

```bash
if [ -f package.json ]; then npm install; fi
if [ -f Cargo.toml ]; then cargo build; fi
if [ -f requirements.txt ]; then pip install -r requirements.txt; fi
if [ -f pyproject.toml ]; then poetry install; fi
if [ -f go.mod ]; then go mod download; fi
```

3. Verify clean baseline: run the project's test suite. If tests fail, report
   the failures and ask whether to proceed or investigate first.
4. Report:

```
Workspace ready at <path> (branch <branch>)
Tests passing (<N> tests, 0 failures)
```

## Step 3: In the Main Checkout — Ask, Then Create

Ask the engineer one question, with a proposed branch name derived from the
feature under discussion:

```
This session is in the main checkout. Feature work belongs in its own herdr
workspace.

1. Create workspace `<branch-name>` — I run `herdr worktree create`; you open
   a new session there and restart from /design-discussion. (recommended)
2. Continue here via EnterWorktree — keeps this conversation, but the feature
   does not get its own herdr workspace.

Which?
```

**Option 1 — create the workspace:**

```bash
herdr worktree create --cwd "$(git rev-parse --show-toplevel)" --branch <branch-name> --no-focus --json
```

- The JSON result carries `.result.workspace.workspace_id` and
  `.result.worktree.path` (`~/.herdr/worktrees/<repo>/<branch>`).
- Always `--no-focus`: never yank the engineer out of the current session.
- Do NOT run `herdr agent start` — starting the session is the engineer's act.

Then report and stop:

```
Workspace '<branch>' created at <path> (herdr workspace <id>).
Switch to it in herdr, run `claude`, and start with /design-discussion <topic>.
```

**Option 2 — EnterWorktree:** use the EnterWorktree tool and continue the flow
in this session.

## Fallback: herdr Unreachable

If the `herdr` CLI or its socket is unavailable, report that and ask the
engineer how to proceed (work on a feature branch in place, or the engineer
prepares a worktree manually). Do not reimplement worktree management with raw
git commands.

## Red Flags

| Violation | Correct Behavior |
|---|---|
| Creating AE worktrees with raw `git worktree add` | herdr owns AE workspaces. Use `herdr worktree create` (or the fallback question). |
| Running `herdr agent start` to launch the new session | The engineer opens sessions. Report and stop. |
| Creating the workspace with `--focus` | Never steal focus from the running session. |
| Removing workspaces or deleting worktree-checked-out branches | Removal is the engineer's manual operation. |
| Proceeding on main/master because "it's a small change" | Feature work gets a workspace. The engineer decides exceptions. |
| Skipping setup/baseline verification in a fresh worktree | Always verify before implementation starts. |

## Integration

**Called by:**
- `/design-discussion` — workspace check when the discussion reveals feature work (launchpad detection)
- `/execute-plan` — workspace precondition before dispatching to agent-teams
- `/agent-teams-driven-development` — workspace prerequisite

**Pairs with:**
- `/finish-branch` — completion; merge/discard are worktree-aware
- `/session-teardown` — session end; workspace removal stays with the engineer
