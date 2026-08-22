---
name: design-discussion
description: Collaboratively clarify an engineering problem, investigate the current system, compare viable approaches, and settle user-owned design decisions. Use when beginning engineering work or when implementation exposes an unresolved design choice.
argument-hint: "[topic]"
---

# Design discussion

Keep architecture, scope, algorithms, public contracts, and material trade-offs
under user control. Act as an investigator and sounding board.

Invoke phase skills through the Skill tool (`/systematic-debugging`); never
perform another phase's work inline.

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

- Investigate the codebase with extended thinking before posing questions.
  When structural context is needed, launch `code-architect` with the Agent
  tool: `Agent({ subagent_type: "code-architect", model: "sonnet", prompt:
  <question> })`. Pass no `name`, run it in the foreground so its report
  returns as the tool result, and pass `model: "sonnet"` explicitly even
  though the definition pins it.
- Ask one question at a time; recommend an answer with its trade-off and leave
  room for discussion before offering a decision.

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

- Route bugs through `/systematic-debugging` before planning a fix.
- Use a Design Doc for cross-cutting architecture, durable public contracts, or decisions worth preserving.
- Skip the Design Doc when the settled scope does not need a durable architecture
  artifact, but still construct a Feature Contract before planning.
- Do not implement from this skill.

## Construct a Feature Contract without a Design Doc

When the coordinator confirms that no Design Doc is warranted and the decision
record is owner-approved, derive a separate Feature Contract from that record
and repository evidence. Include:

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

Return the decision record, unresolved material decisions, relevant evidence,
and whether the settled decisions warrant a Design Doc to
`agentic-engineering-workflow`. When no Design Doc is warranted, also return the
draft Feature Contract. Let the coordinator own its approval and select the next
phase.
