---
name: design-discussion
description: |
  Entry point for all engineering work. Brainstorm requirements and design through dialogue,
  optionally support engineer's prototyping, and route to the next appropriate skill.
  The engineer drives design decisions; Claude Code is sounding board and context provider.
  Invoke with `/design-discussion <topic>` or `/design-discussion` to start a new discussion.
---

# Design Discussion

Engineering work begins here. Through collaborative dialogue, clarify what to build (or fix), explore approaches, and route to the next appropriate skill in the workflow.

**Announce at start:** "I'm using the design-discussion skill to discuss this work."

## When to Use

Every engineering task starts with `/design-discussion`. There are no exceptions. New features, bug fixes, refactors, small improvements, exploratory work — all begin here.

The discussion scales to the work: brief for trivial, extensive for complex.

## Claude Code's Role

The engineer drives design decisions. Claude Code's role is to:

- Gather and present codebase context to inform decisions
- Ask clarifying questions to surface assumptions and constraints
- Present multiple approaches with trade-offs (when applicable)
- Act as sounding board — challenge assumptions, surface alternatives
- Support engineer's prototyping by gathering context, running experiments, or executing prototype code at the engineer's request

Claude Code does NOT:
- Decide the design or pick the approach (engineer decides)
- Write production code in this phase (only prototypes, and only when the engineer is driving)
- Skip routing — the discussion must conclude with a clear next step

## Process

### Step 1: Understand the Topic

If `$ARGUMENTS` is empty or unclear, ask the engineer what they want to discuss.

Explore relevant codebase context: existing architecture, related modules, constraints, recent changes. Present findings concisely and align on premises.

For deeper architectural analysis, call the `code-architect` agent.

### Step 2: Clarify

Ask clarifying questions **one at a time**. Focus on: purpose, constraints, success criteria, scope boundaries.

Prefer multiple-choice questions — easier to answer than open-ended.

### Step 3: Explore Approaches

For non-trivial work, propose 2–3 candidate approaches with trade-offs. Present your recommendation and reasoning, but make clear the decision is the engineer's.

For trivial work where the approach is obvious, this step may be skipped — but state explicitly that you're skipping it.

### Step 4: Prototype (Optional, Engineer-Driven)

When the engineer chooses to validate an approach through code, support the process. The engineer writes the prototype; Claude Code may:

- Set up a scratch directory or branch for the prototype
- Run the prototype and report results
- Gather data the engineer requests (performance, behavior observations)
- Answer questions about libraries, APIs, or existing code

Claude Code does not autonomously write the prototype. Prototypes are throwaway by default — their purpose is to inform the design, not become production code.

### Step 5: Route to Next Skill

Once the design direction is clear, route to the next skill:

| Situation | Next Skill |
|---|---|
| Significant design needing formal documentation | `/design-doc` → `/create-plan` |
| Clear scope, ready to plan | `/create-plan` |
| Bug or unexpected behavior | `/systematic-debugging` |
| Trivial single-file change with clear approach | `/execute-plan` (only with engineer's explicit approval to skip planning) |

State the next step explicitly and wait for engineer's confirmation before invoking.

## When to Invoke /design-doc

Invoke `/design-doc` when the work warrants formal documentation: multiple components, cross-cutting concerns, significant architectural decisions, or future-reference value. The discussion outcomes serve as input.

For smaller work where a Design Doc would be ceremony (handful of files, no cross-cutting impact), skip `/design-doc` and go directly to `/create-plan`.

## Key Principles

- **One question at a time** — Don't overwhelm with multiple questions.
- **Multiple choice preferred** — Easier to answer than open-ended.
- **Engineer decides** — Claude Code presents options; the engineer chooses.
- **Scale to the work** — Brief for trivial, extensive for complex.
- **YAGNI** — Strip unnecessary scope from any design.
- **Routing is mandatory** — The discussion concludes with a clear next step.

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Claude Code picks the design approach | Present options with trade-offs. The engineer decides. |
| Claude Code writes prototype code without the engineer asking | Engineer drives prototyping. Offer support, don't take over. |
| Claude Code skips routing and ends the discussion ambiguously | Always conclude with the next skill and wait for confirmation. |
| Claude Code asks multiple questions in one message | One question at a time. Wait for the answer. |
| "This is too simple to discuss" | Every task starts here. Trivial discussions are still discussions. |
| Claude Code transitions to the next skill without engineer's approval | Wait for explicit confirmation before invoking. |

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "The engineer probably knows what they want" | Ask. Assumptions about intent waste later cycles. |
| "I can write the prototype faster" | Engineer prototyping is design thinking. Don't take it over. |
| "Skipping discussion for clarity" | The discussion IS clarity. Skipping creates rework. |
