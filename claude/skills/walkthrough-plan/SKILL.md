---
name: walkthrough-plan
description: |
  Walk through an implementation plan task by task, explaining each in a teaching
  format and advancing only when the engineer replies "次へ". A read-only,
  on-demand understanding tool — it does not edit the plan, gate execution, or make
  design decisions. Invoke with `/walkthrough-plan [plan-path]`.
---

# Walkthrough Plan

Walk the engineer through a `create-plan` plan one task at a time, re-presenting each task in a digestible teaching format. The engineer reads, asks questions, and replies "次へ" to advance. The purpose is **understanding** — helping the engineer maintain a clear mental model of work that agent-teams will execute autonomously (CLAUDE.md: "the engineer remains responsible for maintaining understanding of the codebase, including code delegated to Claude Code").

**Announce at start:** "I'm using the walkthrough-plan skill to walk through the plan task by task."

## When to Use

On demand, whenever the engineer wants to understand a plan task by task — typically after `/create-plan` produces a plan and before `/execute-plan`, but it can run any time against any plan.

This skill is **standalone and optional**. It is not part of the mandatory Core Flow, does not replace `create-plan`'s engineer review, and is not an approval gate for `execute-plan`. Skip it for trivial plans; reach for it when a plan is large or unfamiliar enough that a task-by-task walkthrough aids understanding (scale to the work / YAGNI).

## Boundaries

This skill is **read-only and explanatory**. It does NOT:

- **Edit the plan.** Change requests are captured as notes, never applied here (see "Handling change requests"). Plan authoring belongs to `create-plan`.
- **Gate execution.** Completing the walkthrough is not plan approval. It only offers a pointer to `/execute-plan` at the end.
- **Make or invent design decisions.** Explanations are grounded in the plan and Design Doc; Claude's own commentary is clearly labeled and never presented as a recorded decision (see "Grounding").
- **Implement anything.** No code is written, no tasks are executed.

## Locating the plan

1. **Argument given** (`/walkthrough-plan <path>`): use that plan.
2. **No argument**: list candidates in the project's plans directory (e.g. `docs/plans/` or `plans/`), present the most recent as the default, and confirm with the engineer before starting. If only one exists, confirm it. If invoked right after `/create-plan`, default to the plan just created.
3. **No plan found**: tell the engineer and suggest `/create-plan`. Do not invent a plan.

## Process

### Step 1: Load the plan and present the TOC

Read the full plan. Then present a **table of contents** — the plan's Goal / Architecture (1-2 lines) and a numbered list of every task with its title — so the engineer sees the overall shape and size before drilling in. State the total task count.

### Step 2: Walk task by task

Start at Task 1 (or the task the engineer names). Present the current task using the **Task Presentation Format** below, then stop and wait. Do not advance until the engineer replies "次へ".

### Step 3: Respond to navigation

Interpret the engineer's reply:

- **「次へ」 / "next"** — present the next task. After the last task, go to Step 4.
- **「前へ」 / "back"** — re-present the previous task.
- **「Task N」 / "N へ"** — jump to task N.
- **Any question** — answer it about the current task, then stay on the same task and wait again. A question does not advance the walkthrough.
- **A change request** — capture it (see "Handling change requests"), then stay on the current task.

### Step 4: Close

After the last task, summarize: number of tasks walked, and any captured change requests (see below). Then offer — **without gating** — the next step: "解説完了。`/execute-plan` に進みますか?" The engineer decides. This skill does not invoke the next skill itself.

## Task Presentation Format

Present each task in this structure. Pull content from the plan; do not restate the plan's raw checkboxes verbatim — re-frame them as a readable explanation.

**`Task N: <title>`**

1. **概要と why** — what the task does and why. Source the "why" from the task's **Why** field. (Cite: the task's `Why`.)

2. **コード / 差分** —
   - **New file** → show the key code the task creates.
   - **Modify** → show the change as a **hybrid diff**: prefer a real `current → planned` diff by reading the actual target file when the working tree is at the plan's baseline, presented as a fenced `diff` block. When the diff is reconstructed beyond the plan's literal text, label it: *「現コードと照合して Claude が構成した差分」*. If the file can't be read or the tree has drifted from the baseline, fall back to the plan's stated change and say so.

3. **必要なら説明 + why not** — explain non-obvious decisions only when it aids understanding. Source "why not" from the plan's **Decisions reference** (header) and **Alternative Solutions Considered** (footer); cite which. If the task has no recorded rationale, say *「プランに why-not の記載なし」* — do not invent one. Any commentary that is Claude's own (not from the plan / Design Doc) must be prefixed *「補足(Claude の解説)」*.

4. **テスト** — the tests the task pins (from the task's Test files / red-phase tests). For non-TDD tasks, state what the verification step checks instead.

5. **ステップの流れ (TDD)** — summarize the task's steps as a short narrative, not a verbatim dump:
   - `1. red:` the failing test that pins the behavior
   - `2. green:` the minimal implementation
   - `3. refactor:` consolidation / cleanup

   For `Discipline: refactor` tasks (no behavior change), say so and summarize the mechanical steps with the existing tests as the green bar, instead of a red→green→refactor cycle.

End with a separator and the advance prompt:

```
---
「次へ」で次のタスクに進みます。
```

## Grounding (no fabrication)

Every explanation traces to a source. The chain of custody:

| Element | Source in the plan |
|---|---|
| why | the task's **Why** |
| why not / alternatives | header **Decisions reference**, footer **Alternative Solutions Considered** |
| code / diff | the task's code blocks + (for diffs) the actual file |
| tests | the task's Test files / red-phase tests |

- Quote or cite the source. When the plan is silent, say it is silent — never fill the gap with an invented rationale.
- Claude's own analysis is allowed but must be labeled *「補足(Claude の解説)」* so it is never mistaken for a recorded decision.

## Handling change requests

If the engineer spots something to change during the walkthrough, **do not edit the plan**. Record the request (task number + what + why) in a running list. At Step 4 (Close), present the list and recommend: if changes are needed, re-enter `/create-plan` to revise the plan. Plan authoring and its self-review / consistency checks belong to `create-plan`, not here.
