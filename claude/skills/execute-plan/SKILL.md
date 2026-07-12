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
- A feature workspace is set up (NOT main/master). If not, invoke `/create-workspace` first.

## Process

### Step 1: Verify Workspace

Confirm an isolated workspace (herdr worktree or feature branch) is set up. If not, invoke `/create-workspace`.

### Step 2: Dispatch to Agent-Teams

Invoke `/agent-teams-driven-development`. Pass the plan file path as context.

The agent-teams skill takes over: populates TaskList, spawns (or reuses) implementer + 2 reviewers, and runs per-task loops. Teammates persist across loop re-entries and are reclaimed automatically at session exit — there is no per-pass teardown.

### Step 3: Receive Completion Signal

When agent-teams reports completion, verify all TaskList entries are marked completed and all commits are on the feature branch.

### Step 4: Transition

→ Transition to `/verify` for formal verification (build, test, lint via implementation-verifier agent).

**Note**: After `/verify`, the flow transitions to `/review`. The review feedback loop may re-invoke `/execute-plan` (this skill) autonomously to execute fix tasks appended to the plan's "Post-/review iteration" section. This re-entry is part of the autonomous loop and does **not** require engineer confirmation — proceed directly with the new tasks. The loop terminates either by clean review (→ `/finish-branch`) or by escalation (→ engineer report).

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
| Executing on main/master | Stop. Set up the feature workspace via /create-workspace. |
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
- Engineer's mandatory review gate is at `/finish-branch` (before merge / PR / share). The autonomous loop (`/execute-plan` → `/verify` → `/review` → fix tasks) runs **without** engineer approval prompts; engineer involvement happens only on escalation or at `/finish-branch`
- If verification reveals issues not covered by the plan, stop and consult the engineer

## Integration

**Required:**
- `/create-plan` — provides the plan to execute
- `/create-workspace` — workspace verification/setup before execution
- `/agent-teams-driven-development` — autonomous execution
- `/verify` — formal verification after execution
