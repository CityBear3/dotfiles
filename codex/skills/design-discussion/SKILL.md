---
name: design-discussion
description: Collaboratively clarify an engineering problem, investigate the current system, compare viable approaches, and settle user-owned design decisions. Use when beginning engineering work or when implementation exposes an unresolved design choice.
---

# Design discussion

Keep architecture, scope, algorithms, public contracts, and material trade-offs
under user control. Act as an investigator and sounding board.

## Establish the problem

Read relevant code, tests, documentation, and history before asking questions. Summarize:

- current behavior;
- desired outcome;
- constraints and non-goals;
- decisions already made;
- remaining material ambiguity.

Ask only for choices that cannot be resolved from available evidence.

## Explore

For each material decision:

1. state the decision to be made;
2. present the smallest set of viable options;
3. explain concrete trade-offs in this codebase;
4. recommend one option and why;
5. let the user decide.

Support user-authored prototypes with research, diagnostics, or review. Do not take over implementation while the user is using code to explore the design.

## Maintain the decision record

Update a compact decision record as choices settle:

- selected approach and rationale;
- rejected alternatives and reasons;
- scope;
- non-goals;
- explicitly deferred questions.

List unresolved material decisions separately. Do not treat a question as
deferred unless the user explicitly accepts that deferral.

## Scale the process

- Route bugs through systematic debugging before planning a fix.
- Use a Design Doc for cross-cutting architecture, durable public contracts, or decisions worth preserving.
- Skip the Design Doc when the settled scope is small enough for a self-contained plan.
- Do not implement from this skill.

## Handoff

Return the decision record, unresolved material decisions, and relevant evidence
to `agentic-engineering-workflow`. State whether the settled decisions warrant a
Design Doc. Let the coordinator select and gate the next phase.
