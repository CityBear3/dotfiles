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

### Step 1: Verify Prerequisites

- Plan file exists and approved
- Workspace is isolated (worktree, not main)
- Read the plan once, extract all tasks with full text and context

### Step 2: Create Team

```
TeamCreate({ team_name: "<feature-name>", description: "Executing <plan-file>" })
```

### Step 3: Populate TaskList

For each plan task: `TaskCreate({ subject: "Task N: <component>", description: "<full task text from plan>" })`

### Step 4: Spawn Team Members

Specify the `model` parameter explicitly for each teammate. The role-to-model mapping is:

- **Implementer** → `opus` — production-ready code with minimal oversight; suited for multi-file refactors and complex logic
- **Spec Reviewer** → `sonnet` — matches diff against spec text (mechanical comparison). Sonnet is the multi-agent research system's recommended subagent model; Haiku previously caused agent-teams SendMessage to hang (reviewer reported "sent" but Leader never received, and the agent stopped responding to shutdown_request).
- **Code Quality Reviewer** → `sonnet` — nuanced judgment on naming/patterns/complexity; deep dive is delegated to `/review`

```
Agent({ team_name, name: "implementer", subagent_type: "general-purpose", model: "opus", prompt: <see ./implementer-prompt.md> })
Agent({ team_name, name: "spec-reviewer", subagent_type: "code-reviewer", model: "opus", prompt: <see ./spec-reviewer-prompt.md> })
Agent({ team_name, name: "code-quality-reviewer", subagent_type: "code-reviewer", model: "opus", prompt: <see ./code-quality-reviewer-prompt.md> })
```

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

1. Send `{type: "shutdown_request"}` to each teammate
2. Wait for shutdowns
3. `TeamDelete` to remove team and TaskList

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

- **Mechanical** (1-2 files, complete spec) → cheap model
- **Integration** (multi-file, patterns) → standard model
- **Architecture / design / review** → most capable model

Replace teammates with appropriate models based on task complexity.

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
- `./code-quality-reviewer-prompt.md` — Code quality reviewer onboarding + per-review / final-review message templates
