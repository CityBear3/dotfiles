---
name: execute-plan
description: |
  Execute an approved implementation plan via the agent-teams autonomous loop.
  Verifies workspace setup, dispatches to agent-teams, and transitions to verification.
  Invoke with `/execute-plan` after /create-plan completes.
---

# Execute Plan

Execute the approved implementation plan via the agent-teams autonomous loop.

**Announce at start:** "I'm using the execute-plan skill to execute this plan."

## Entry Conditions

- An approved plan exists (from `/create-plan`)
- A feature branch or worktree is set up (NOT main/master). If not, invoke `/using-git-worktrees` first.

## Process

### Step 1: Verify Workspace

Confirm an isolated workspace (worktree or feature branch) is set up. If not, invoke `/using-git-worktrees`.

### Step 2: Dispatch to Agent-Teams

Invoke `/agent-teams-driven-development`. Pass the plan file path as context.

The agent-teams skill takes over: creates the team, populates TaskList, spawns implementer + 2 reviewers, runs per-task loops, runs final review, tears down team.

### Step 3: Receive Completion Signal

When agent-teams reports completion, verify all TaskList entries are marked completed, final review approved, all commits on feature branch.

### Step 4: Transition

→ Transition to `/verify` for formal verification (build, test, lint via implementation-verifier agent).

## Discipline

- The plan must be followed faithfully. No ad-hoc design changes during execution.
- Never start execution on main/master without explicit engineer consent.
- If verification reveals issues outside the plan's scope, stop and consult the engineer.

## Failure Handling

- Always work on a feature branch / worktree, never on main.
- Rely on CI as a safety net if configured.
- If a change turns out to be wrong, prefer reverting to patching. Atomic commits enable clean reverts.

## Red Flags

| Violation | Correct Behavior |
|---|---|
| Executing without an approved plan | Stop. Get plan approval first via /create-plan. |
| Executing on main/master | Stop. Set up worktree or feature branch via /using-git-worktrees. |
| Ad-hoc design changes during execution | Flag. Return to `/design-discussion` if design must change. |
| Skipping /verify after agent-teams completes | /verify is the formal gate. Run it. |
| Reporting completion with known test failures | Fix or note explicitly. |
| Bypassing /agent-teams-driven-development to execute inline | Agent-teams is the autonomous loop. Don't bypass. |

## Rationalization Prevention

| Excuse | Reality |
|---|---|
| "Just a small design tweak during execution" | Design changes go through /design-discussion. No exceptions. |
| "The plan is small, I can execute inline" | Agent-teams is the execution mechanism. Don't bypass. |
| "Tests pass, no need for /verify" | /verify is the formal gate. Run it. |

## Rules

- Never start execution on main/master without explicit engineer consent
- Agent-teams is the execution mechanism — do not bypass
- The engineer's review is a mandatory gate before any change is merged or shared
- If verification reveals issues not covered by the plan, stop and consult the engineer

## Integration

**Required:**
- `/create-plan` — provides the plan to execute
- `/using-git-worktrees` — workspace setup before execution
- `/agent-teams-driven-development` — autonomous execution
- `/verify` — formal verification after execution
