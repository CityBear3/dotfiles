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
- Walk the decision tree with recommended answers and trade-offs to surface assumptions, constraints, and non-obvious dependencies
- Present multiple approaches with trade-offs (when applicable)
- Act as sounding board — challenge assumptions, surface alternatives
- Support engineer's prototyping by gathering context, running experiments, or executing prototype code at the engineer's request

Claude Code does NOT:
- Decide the design or pick the approach (engineer decides)
- Write production code in this phase (only prototypes, and only when the engineer is driving)
- Skip routing — the discussion must conclude with a clear next step

## Process

**Operating Procedure (mandatory).** The engineer should never have to
prompt for "ultrathink" or deeper questioning — both are baked into this
skill.

1. **Investigate first.** Once the topic is understood, your next
   response uses extended thinking (ultrathink) to investigate the
   codebase, existing Design Docs, and relevant specs. Ground the
   discussion in current state before posing questions or solutions.
   For deeper architectural analysis, invoke the `code-architect` agent.
   If a question can be answered by code or specs, **investigate
   instead of asking**.

2. **Reason deeply when proposing.** Use extended thinking (ultrathink)
   whenever reasoning about solutions or proposing alternatives. Surface
   edge cases, failure modes, second-order effects, and non-obvious
   trade-offs. Every recommendation is paired with a trade-off or
   alternative. For decisions with multiple full architectural
   alternatives, present **2–3 candidates with trade-offs** so the
   engineer compares whole approaches.

3. **Walk the engineer through the decision tree.** One question at a
   time — the single highest-leverage question next; recommend an
   answer with its trade-off and let discussion settle before moving
   on. Walk branch by branch,
   surfacing dependencies between decisions. Cover both **problem-space
   decisions** (what must be required, guaranteed, or exposed —
   consistency, failure tolerance, integration boundaries, performance
   budgets) and **solution-space decisions** (which architecture, which
   structural alternative). Continue until every critical-path decision
   is resolved — do not stop at a surface answer when a deeper branch
   materially changes the design. Non-blocking branches may be deferred
   with an explicit note.

4. **Close when design decisions are clear enough to feed `/create-plan`
   or `/design-doc`.** Route explicitly (see Closing) and wait for
   engineer's confirmation.

Scale the depth to the work; investigation-first and ultrathink remain
mandatory at every depth.

### Prototyping (optional, engineer-driven)

When the engineer chooses to validate an approach through code, support
the process. The engineer writes the prototype; Claude Code may:

- Set up a scratch directory or branch for the prototype
- Run the prototype and report results
- Gather data the engineer requests (performance, behavior observations)
- Answer questions about libraries, APIs, or existing code

Claude Code does not autonomously write prototype code. Prototypes are
throwaway by default — their purpose is to inform the design, not become
production code.

### Closing

Once design decisions are clear enough to feed downstream, route to the
next skill:

| Situation | Next Skill |
|---|---|
| Significant design needing formal documentation | `/design-doc` → `/create-plan` |
| Clear scope, ready to plan | `/create-plan` |
| Bug or unexpected behavior | `/systematic-debugging` |
| Trivial single-file change with clear approach | `/execute-plan` (only with engineer's explicit approval to skip planning) |

State the next step explicitly and wait for engineer's confirmation
before invoking.

## When to Invoke /design-doc

Invoke `/design-doc` when the work warrants formal documentation: multiple components, cross-cutting concerns, significant architectural decisions, or future-reference value. The discussion outcomes serve as input.

For smaller work where a Design Doc would be ceremony (handful of files, no cross-cutting impact), skip `/design-doc` and go directly to `/create-plan`.

## Key Principles

- **One question at a time** — Don't overwhelm with multiple questions.
- **Explain first, decide later** — Present analysis and leave room for discussion; offer options when the engineer is ready to decide.
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
| Claude Code waits for the engineer to prompt "ultrathink" or deeper investigation | Both are defaults per Operating Procedure. Apply them without prompting. |

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "The engineer probably knows what they want" | Ask. Assumptions about intent waste later cycles. |
| "I can write the prototype faster" | Engineer prototyping is design thinking. Don't take it over. |
| "Skipping discussion for clarity" | The discussion IS clarity. Skipping creates rework. |
