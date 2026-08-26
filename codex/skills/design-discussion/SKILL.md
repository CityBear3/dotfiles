---
name: design-discussion
description: Collaboratively clarify an engineering problem, investigate the current system, compare viable approaches, and settle user-owned design decisions. Use when beginning engineering work or when implementation exposes an unresolved design choice.
---

# Design discussion

Keep architecture, scope, algorithms, public contracts, and material trade-offs
under user control. Act as an investigator and sounding board.

## Establish the problem

Accept from `agentic-engineering-workflow` the planned route, confirmed
workspace, living-record location, exact existing authority and its currentness,
and unresolved evidence. Do not persist a material decision until the
coordinator has confirmed the checkout, branch, and starting ref.

Read relevant code, tests, documentation, and history before asking questions.
Summarize:

- current behavior;
- desired outcome;
- constraints and non-goals;
- decisions already made;
- remaining material ambiguity.

Ask only for choices that cannot be resolved from available evidence.

An exact, current, explicitly approved Design Doc or no-Design-Doc decision
record can satisfy the readiness dimensions it covers. Do not repeat those
decisions. Past conversation, an unapproved artifact, or an Agent-authored
summary is not approved authority. Reopen only a missing or changed branch and
the settled decisions whose meaning depends on it.

## Maintain the living decision record

For planned discussion, create the ignored workspace artifact
`docs/plans/YYYY-MM-DD-<feature>/decision-record.md` after workspace
confirmation and before recording the first material decision. Update it as
choices settle with:

- selected approaches and rationale;
- rejected alternatives and reasons;
- scope and non-goals;
- explicitly accepted deferrals with their intent and impact;
- unresolved material questions in a separate section.

Only explicitly settled choices enter the record as decisions. An Agent
recommendation is not a decision, and file existence is not approval. Do not
treat a question as deferred unless the user explicitly accepts both the
deferral and its impact.

## Explore

Resolve only one material decision at a time:

1. state the decision and why it is now reachable;
2. present the smallest set of viable options;
3. explain concrete trade-offs in this codebase;
4. recommend one option and why;
5. ask only that question and let the user decide;
6. record the answer and identify newly reachable branches and dependencies
   between decisions.

When a decision requires a prototype, visual artifact, benchmark, or another
discovery phase, record the explicit handoff and the evidence required to
re-enter that branch. The question remains unresolved until that evidence is
available. Support user-authored prototypes with research, diagnostics, or
review; do not take over implementation while the user is using code to explore
the design.

If the design scope is too broad to explore reliably, make scope decomposition
the next user-owned design decision.

## Assess Design Readiness

Report planned design as ready only when every applicable condition holds:

1. repository-discoverable facts have been investigated;
2. purpose and observable completion conditions are settled;
3. scope, non-goals, constraints, and invariants are settled;
4. applicable responsibility boundaries, dependency direction, and interfaces
   are settled;
5. expected behavior and its verification method are settled;
6. failure and recovery, migration, concurrency, authorization, performance,
   and comparable concerns are settled when applicable;
7. material design branches and dependencies between decisions are resolved;
8. questions requiring another discovery phase have explicit handoffs and
   evidence-based re-entry conditions;
9. no material question remains unresolved except an explicitly accepted
   deferral with recorded intent and impact; and
10. settled decisions are consolidated into the complete living record.

Treat these as applicability dimensions, not ten mandatory user questions. Do
not expand scope or ask about irrelevant dimensions to make the process appear
more rigorous. Report the exact missing branch or dimension when readiness does
not hold; do not fill it by inference. Exact current approved authority may
satisfy the dimensions it covers, including consolidation of unchanged
decisions, without copying them into a duplicate living record.

## Scale the process

- Route bugs through systematic debugging before planning a fix.
- Use a Design Doc for cross-cutting architecture, durable public contracts, or decisions worth preserving.
- Skip the Design Doc when the settled scope does not need a durable architecture
  artifact, but still construct a Feature Contract before planning.
- Do not implement from this skill.

## Construct a Feature Contract without a Design Doc

When Design Readiness holds and the coordinator confirms that no Design Doc is
warranted, present the complete decision record for explicit user approval as
the one holistic design approval. Retain that approved record as the design
authority throughout the active workspace lifecycle. Only then derive a
separate Feature Contract from that record and repository evidence. Include:

- context and goal;
- scope and non-goals;
- design sources, decisions, and precedence;
- observable behavior, preserved behavior, and material failure behavior;
- responsibilities, interfaces, and important unchanged boundaries;
- protected constraints and invariants;
- verification obligations;
- evidence-backed assumptions and explicitly approved deferrals.

Add conditional concerns such as state transitions, schema lifecycle, error and
recovery semantics, concurrency, authorization, performance, migration, or
rollback only when they apply. If completing the contract would decide
architecture, responsibility, a public or shared interface, schema, error
behavior, scope, or another material trade-off, return that exact ambiguity to
design discussion. Do not fill it in while drafting.

Return the complete Feature Contract to the coordinator for separate user
approval and workspace-only storage at
`docs/plans/YYYY-MM-DD-<feature>/feature-contract.md`. Keep it ignored and do not
force-add, stage, or commit it unless the user explicitly chooses archival. This
skill does not approve the contract, enter planning, or combine it with an
Implementation Plan.

## Handoff

Return the current decision record, Design Readiness result, unresolved branches
and dependencies, relevant evidence, existing-authority currentness, and whether
the settled decisions warrant a Design Doc to `agentic-engineering-workflow`.
When no Design Doc is warranted, also return the decision-record approval state
and draft Feature Contract. Let the coordinator own transitions and Feature
Contract approval.
