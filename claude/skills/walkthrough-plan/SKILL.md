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
