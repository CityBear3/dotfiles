---
name: design-discussion
description: Collaboratively clarify an engineering problem, investigate the current system, compare viable approaches, and settle user-owned design decisions. Use when beginning engineering work or when implementation exposes an unresolved design choice.
---

# Design discussion

The user owns architecture, scope, algorithms, and public contracts. Act as an investigator and sounding board.

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

## Scale the process

- Route bugs through systematic debugging before planning a fix.
- Use a Design Doc for cross-cutting architecture, durable public contracts, or decisions worth preserving.
- Skip the Design Doc when the settled scope is small enough for a self-contained plan.
- Do not implement from this skill.

## Exit

End with a compact decision record:

- selected approach;
- rejected alternatives and reasons;
- scope and non-goals;
- unresolved questions;
- recommended next phase.

Wait for the user to approve the next phase.
