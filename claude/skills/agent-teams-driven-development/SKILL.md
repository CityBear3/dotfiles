---
name: agent-teams-driven-development
description: |
  Execute an implementation plan via a coordinated team of agents (implementer + two reviewers).
  Each task flows through implementation → spec compliance review → code quality review,
  with re-review loops on issues. The lead orchestrates the team via Claude Code's team feature
  (TaskList + SendMessage). Invoked from /execute-plan.
---

# Agent-Teams-Driven Development

Execute a plan by coordinating a team: one implementer and two reviewers (spec compliance + code quality). Each task flows through the team with re-review loops, ensuring issues are caught before moving on.

**Why teams:** Persistent team members reduce spawn overhead. The lead orchestrates via TaskList (work tracking) and SendMessage (review coordination). Each member has isolated context — the lead provides exactly what they need.

**Core principle:** Per-task implementation + two-stage review + re-review loops = high quality, autonomous progress

## When to Use

Invoked by `/execute-plan`. Not invoked directly by the engineer.

**Prerequisites:**
- An approved plan exists (from `/create-plan`)
- An isolated workspace is set up (via `/using-git-worktrees`)
- The current branch is NOT main/master

## Team Composition

| Role | Agent Type | Purpose |
|---|---|---|
| **Lead** | (main session) | Orchestrates, assigns tasks, runs review loops |
| **Implementer** | general-purpose | Implements via TDD, commits, self-reviews |
| **Spec Reviewer** | code-reviewer | Verifies code matches spec (nothing missing/extra) |
| **Code Quality Reviewer** | code-reviewer | Verifies code quality (clean, maintainable, patterns) |

## Setup

**No explicit team creation (v2.1.178+).** Teammates implicitly form a team on the first `Agent()` spawn — there is no `TeamCreate` step, and `Agent()` no longer takes a `team_name`. Cleanup is automatic on session exit; `TeamDelete` no longer exists. The lead still coordinates via TaskList (work tracking) and SendMessage (review coordination).

### Step 1: Verify Prerequisites

- Plan file exists and approved
- Workspace is isolated (worktree, not main)
- Read the plan once, extract all tasks with full text and context

### Step 2: Populate TaskList

For each plan task: `TaskCreate({ subject: "Task N: <component>", description: "<full task text from plan>" })`

### Step 3: Spawn Team Members

**Model rule: omit the `model` parameter** — teammates inherit the lead's session model. This tracks the most capable model the engineer is currently running without hardcoding a model name in this skill. Omission resolves to the session model via two distinct mechanisms (verified 2026-06-10):

- `code-reviewer` (both reviewers): the agent definition has `model: inherit`.
- `general-purpose` (implementer): teammates do NOT inherit the lead's model by default — inheritance requires `/config` → "Default teammate model" = "Default (leader's model)" (`teammateDefaultModel: null` in `~/.claude.json`). This is a device-local setting not synced by install.sh; on a machine without it, the implementer silently resolves to opus, which equals the floor below — a safe failure mode.

**Floor: if the lead session is running below opus tier (sonnet / haiku), specify `model: "opus"` explicitly on all three spawns.** Rationale:

- Reviewers were once on sonnet and were promoted to opus (commit 7464841) because the cost/latency trade-off didn't hold — subtle issues (span overrun / arm ordering / type-inference edges) were missed. Inheriting a sonnet session would silently undo that decision.
- Haiku previously caused agent-teams SendMessage to hang: reviewer reported "sent" but Leader never received, and the agent stopped responding to shutdown_request. Teammates must never run on haiku.

```
Agent({ name: "implementer", subagent_type: "general-purpose", prompt: <see ./implementer-prompt.md> })
Agent({ name: "spec-reviewer", subagent_type: "code-reviewer", prompt: <see ./spec-reviewer-prompt.md> })
Agent({ name: "code-quality-reviewer", subagent_type: "code-reviewer", prompt: <see ./code-quality-reviewer-prompt.md> })
// Lead on sonnet/haiku → add model: "opus" to each call
```

### Step 4: Report Spawned Models

Immediately after the three spawns, display each teammate's resolved model to the engineer — the resolution above depends on device-local settings (`teammateDefaultModel`) and agent definitions, so an unintended configuration must surface before any task runs:

```
Team spawned (lead session: <model>):
- implementer:           <model>
- spec-reviewer:         <model>
- code-quality-reviewer: <model>
```

Take each model from the spawn's tool result. If a result does not state the model, derive it from the Step 3 resolution rules and mark it `(derived)`. If any teammate resolved to haiku or below the opus floor unexpectedly, do not proceed — shut it down and respawn with `model: "opus"` per the floor rule, then report the corrected lineup.

## Per-Task Loop

For each task in TaskList order:

### 1. Assign to Implementer

```
TaskUpdate({ taskId, owner: "implementer", status: "in_progress" })
SendMessage({ to: "implementer", message: <full task text + context> })
```

### 2. Handle Questions

If implementer asks questions, answer clearly and completely before they proceed.

### 3. Receive Implementer Status

Implementer reports one of four statuses (see Handling Status below).

### 4. Parallel Review (Spec Compliance + Code Quality)

Spec compliance and code quality are independent review aspects — send both reviewers in parallel:

```
SendMessage({ to: "spec-reviewer", message: "Review task <N>. Diff: <BASE_SHA>..<HEAD_SHA>. Spec: <task text>" })
SendMessage({ to: "code-quality-reviewer", message: "Review task <N>. Diff: <BASE_SHA>..<HEAD_SHA>" })
```

Wait for both responses, then aggregate issues from both reviewers.

If issues from either: send **a single combined fix request** to implementer that covers all issues from both reviewers → re-trigger both reviewers in parallel against the fixed diff. Loop until both approve.

### 5. Mark Task Complete

```
TaskUpdate({ taskId, status: "completed" })
```

Move to next task.

## Completion

After all tasks complete, proceed to Teardown. **Do not run a final whole-implementation review here** — the parent flow (`/execute-plan`) transitions to `/verify` then `/review`, where the verification reviewers (`design-alignment-reviewer`, `scope-reviewer`, `test-coverage-reviewer`) and adversarial personas (`adversarial-robustness-reviewer`, `adversarial-api-reviewer`, `adversarial-performance-reviewer`, `adversarial-tests-reviewer`) run the deep-dive review with extended thinking, integrated via `adversarial-integrator`. Duplicating that review here adds latency without catching additional issues.

## Teardown

1. Send a graceful shutdown to each teammate: `SendMessage({ to: <name>, message: { type: "shutdown_request" } })`. (A plain-text "shut down" request also works; `shutdown_request` is the explicit form and is still accepted.)
2. Wait for each teammate's `shutdown_response`.

There is no `TeamDelete` (removed in v2.1.178) and no manual team/TaskList teardown — the implicit team and its TaskList are cleaned up automatically when the session exits. **Still shut the teammates down explicitly here**: implicit teams are not cleaned up between skills, so in the autonomous loop they would otherwise linger into `/verify` and `/review`.

## Handling Status

Implementer reports one of four statuses:

- **DONE**: Proceed to spec review
- **DONE_WITH_CONCERNS**: Read concerns. If correctness/scope → address. If observations → note and proceed.
- **NEEDS_CONTEXT**: Provide missing context, re-prompt
- **BLOCKED**: Assess:
  1. Context problem → more context, same model
  2. Needs more reasoning → spawn replacement with more capable model
  3. Too large → escalate (break down)
  4. Plan flawed → escalate

**Never** ignore an escalation or force same model to retry without changes.

## Model Selection

Default is the Step 3 rule: inherit the lead's session model, floored at opus. The only deviation is upward — when handling a BLOCKED escalation that needs more reasoning, spawn a replacement on a more capable model. Never downgrade teammates below opus based on task simplicity; that trade-off was already rejected in practice (commit 7464841).

## Escalation

Escalate to engineer when:
- A task fails twice after fix attempts
- A plan deviation is required
- The plan itself appears flawed
- A teammate reports BLOCKED with no clear resolution

Present what was tried, what failed, teammate's analysis, recommended next step.

## Red Flags

| Violation | Correct Behavior |
|---|---|
| Start execution on main/master without engineer consent | Stop. Verify worktree / feature branch. |
| Skip spec OR code quality review | Both required for every task. |
| Move to next task while either review has open issues | Loop until both approve. |
| Self-review replaces actual review | Both needed — different scopes. |
| Fix implementer issues manually in lead session | Send to implementer (avoids context pollution). |
| Dispatch multiple implementers in parallel for same files | Sequential per file. |
| Ignore implementer questions | Answer fully before they proceed. |
| Accept "close enough" on spec compliance | Reviewer found issues = not done. |
| Skip re-review after fixes | Verify fixes actually work. |
| Continue after escalation without engineer | Stop. Wait. |

## Rationalization Prevention

| Excuse | Reality |
|---|---|
| "Self-review is enough" | Outside view catches what self misses. |
| "Small task, skip code quality" | Quality issues compound. Always review. |
| "Fix is minor, skip re-review" | Minor fixes introduce new issues. |
| "I'll fix it in the lead session" | Pollutes lead context. Send to implementer. |
| "Team can self-organize" | Lead orchestrates. Teammates execute. |

## Integration

**Required:**
- `/using-git-worktrees` — isolated workspace before starting
- `/create-plan` — creates the plan
- `/finish-branch` — after all tasks complete

**Teammates follow:**
- `/test-driven-development` — TDD for each task
- `/commit` — commit conventions

## Prompt Templates

Sub-files in this skill directory:
- `./implementer-prompt.md` — Implementer onboarding + per-task / per-fix message templates
- `./spec-reviewer-prompt.md` — Spec compliance reviewer onboarding + per-review message template
- `./code-quality-reviewer-prompt.md` — Code quality reviewer onboarding + per-review message template
