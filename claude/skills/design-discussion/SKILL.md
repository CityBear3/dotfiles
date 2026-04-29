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
- Grill through the decision tree with recommended answers and trade-offs to surface assumptions, constraints, and non-obvious dependencies
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

### Step 2: Clarify and Grill

Walk the engineer through the problem space relentlessly until intent and
constraints are fully resolved. This is not surface-level clarification — it
is a guided interrogation of the decision tree, branch by branch, that
produces a complete picture of *what* must be true.

**Process:**

- Ask **one question at a time**. Multiple-choice or recommend-an-answer
  formats are preferred over open-ended.
- For each question, provide **your recommended answer paired with a short
  trade-off or alternative**, so the engineer reacts to a concrete proposal
  rather than generating from scratch.
- **Walk the decision tree branch by branch.** When one decision constrains
  the next, surface the dependency and resolve them in order.
- Cover purpose, constraints, success criteria, scope boundaries, failure
  tolerance, integration scope, and other intent-level concerns. Continue
  until shared understanding is reached on every critical-path branch.
- If a question can be answered by **exploring the codebase, explore instead
  of asking**.

**Scope (problem space):** decisions about user-facing requirements and
contracts — what the system must guarantee, tolerate, or expose. Examples:
consistency requirements, load profile, failure tolerance, integration
boundaries, performance budgets.

**Out of scope (defer to Step 3):** decisions about internal structure or
implementation strategy — *how* the system is built. These are explored as
full alternatives in Step 3.

**Termination:** Stop when the engineer signals shared understanding
("let's move on") or when all critical-path branches are resolved.
Non-blocking branches may be deferred with an explicit note.

**Recommendation discipline:** Recommendations are reactions, not answers.
Always pair a recommendation with its trade-off or an alternative so the
engineer engages with the rationale, not the answer alone. The engineer
decides.

**Scale to work:** the depth of grilling matches the work. Trivial tasks
may resolve in two or three exchanges; complex designs walk a deeper tree.
Do not over-question simple work.

### Step 3: Explore Approaches

Once the problem space is settled in Step 2, explore the **solution space**:
how to build something that satisfies the constraints established there.

For non-trivial work, propose 2–3 candidate architectures or implementation
strategies with trade-offs. Present your recommendation and reasoning, but
make clear the decision is the engineer's.

For trivial work where the approach is obvious, this step may be skipped —
but state explicitly that you're skipping it.

**Distinguishing Step 3 from Step 2:**

- Step 2 asks discrete questions about *what is required*; each question
  has a single answer that constrains the design.
- Step 3 presents *whole alternatives for how to build it*; each candidate
  is a complete approach to be compared against the others.

A question that asks the engineer to choose between full structural
alternatives belongs here. A question with a single discrete answer that
constrains the design belongs in Step 2.

**Think deeply at this step.** Use extended thinking (ultrathink) to reason
about edge cases, failure modes, second-order effects, and long-term
implications of each candidate approach. Surface non-obvious trade-offs and
constraints the engineer may not have considered. The quality of design
judgment downstream depends on the quality of the options presented here.

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
- **Walk the decision tree** — Branch by branch, until critical-path decisions are resolved. Don't accept surface answers when a deeper branch matters.
- **Recommend with trade-off** — Pair every recommended answer with the cost or alternative. Recommendations are reactions, not answers.
- **Codebase before questions** — If code can answer, read code instead of asking.

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Claude Code picks the design approach | Present options with trade-offs. The engineer decides. |
| Claude Code writes prototype code without the engineer asking | Engineer drives prototyping. Offer support, don't take over. |
| Claude Code skips routing and ends the discussion ambiguously | Always conclude with the next skill and wait for confirmation. |
| Claude Code asks multiple questions in one message | One question at a time. Wait for the answer. |
| "This is too simple to discuss" | Every task starts here. Trivial discussions are still discussions. |
| Claude Code transitions to the next skill without engineer's approval | Wait for explicit confirmation before invoking. |
| Claude Code asks a question and accepts the engineer's first reply without checking dependent branches | Walk the decision tree. If the answer constrains a downstream decision, surface the dependency. |
| Claude Code recommends an answer without a trade-off, and the engineer rubber-stamps it | Always pair recommendation with trade-off or alternative. Recommendations are reactions to engage with, not defaults to accept. |
| Claude Code asks the engineer something the codebase already answers | Explore the codebase first. Ask only if code can't answer. |

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "The engineer probably knows what they want" | Ask. Assumptions about intent waste later cycles. |
| "I can write the prototype faster" | Engineer prototyping is design thinking. Don't take it over. |
| "Skipping discussion for clarity" | The discussion IS clarity. Skipping creates rework. |
