---
name: design-doc
description: Draft or support a formal Design Doc from user-owned decisions through research, ambiguity detection, structured writing, and critique. Use when settled engineering work has cross-cutting architecture, durable contracts, or significant decisions worth preserving.
---

# Design Doc support

Keep every material design decision under user control. Treat prose authorship as
separate from decision ownership.

## Validate decisions before drafting

Accept from `agentic-engineering-workflow` any one of these inputs:

- an owner-approved decision record;
- an existing approved design artifact that establishes an equivalent record;
- a user-authored draft from which an equivalent owner-approved record can be
  verified.

Require the record to identify:

- the selected approach and rationale;
- rejected alternatives and reasons;
- scope and non-goals;
- explicitly deferred questions.

Investigate relevant code, tests, documentation, and history. Challenge
assumptions and compare viable alternatives to validate the supplied record. Ask
detailed questions for factual or other non-material document clarification only
when the answer cannot change architecture, responsibilities, public contracts,
schemas, error models, or scope.

Do not settle a missing material decision from this skill. When a missing or
unapproved choice could change those contracts, return the ambiguity and evidence
to `agentic-engineering-workflow` for design discussion. Do not choose silently.

## Draft from settled decisions

After all material decisions are settled, Codex may draft the complete Design
Doc. Derive its design-specific prose from the decision record and repository
evidence. Do not introduce a component, responsibility, contract, or trade-off
that those sources do not establish.

If drafting reveals a material ambiguity, stop and return it to design
discussion through the coordinator. Do not hide a new decision in polished
prose.

When the user prefers to author a draft, provide the same research, fixed
template, ambiguity detection, critique, and explicitly requested targeted
edits. Preserve the user's prose unless an edit is requested.

## Template

```markdown
# [Design Doc] Title

## Context and scope

### Goals

### Non-goals

## Overview

## Detailed design

## Cross-cutting concerns

## Alternatives considered
```

The detailed design should explain architecture, state transitions, invariants, public interfaces, schemas, error models, and trade-offs when those are part of the contract. Leave private helper structure and step-by-step implementation to code and the implementation plan.

For API-heavy documents, read [references/api-section-format.md](references/api-section-format.md). For detailed-design structure and review criteria, read [references/detailed-design-guide.md](references/detailed-design-guide.md).

## Review and approval

Review every draft, whether Codex-authored or user-authored. Check:

- goals and non-goals are distinguishable;
- terms and assumptions are defined;
- requirements are measurable where necessary;
- public contracts, failure behavior, and invariants are explicit;
- alternatives include the reason they were rejected;
- diagrams or tables clarify real relationships instead of decorating prose;
- implementation details do not obscure design decisions;
- every material claim follows from the decision record and repository evidence.

Present the complete document for user approval. Return the approval state,
decision record, and evidence to `agentic-engineering-workflow`; do not begin
planning from this skill.

## Derive the Feature Contract after approval

Only after the coordinator confirms approval of the exact Design Doc, derive a
separate Feature Contract for the current feature. Use the approved document as
the durable design source and repository evidence for its current application.
The Feature Contract must state:

- context and goal;
- scope and non-goals;
- design sources, applied decisions, and precedence;
- observable behavior, compatibility, and material failure behavior;
- responsibilities, interfaces, and important unchanged boundaries;
- protected constraints and invariants;
- verification obligations;
- evidence-backed assumptions and explicitly approved deferrals.

Include applicable conditional concerns such as state transitions, API or event
semantics, schema lifecycle, errors and recovery, concurrency, authorization,
performance, migration, or rollback. Place interface detail at the earliest
layer where an independent consumer or implementation task depends on it; do not
promote private helper structure into the contract.

If a complete Feature Contract cannot be derived without changing or adding a
material architecture, responsibility, public contract, schema, error model,
scope, or trade-off, return the gap to the coordinator so the Design Doc or
design discussion can be reopened. Do not silently supplement an approved Design
Doc during contract drafting.

Return the complete Feature Contract for separate user approval and
workspace-only storage at
`docs/plans/YYYY-MM-DD-<feature>/feature-contract.md`. Keep it ignored and do not
force-add, stage, or commit it unless the user explicitly chooses archival.
Design Doc approval does not approve this artifact, and this skill does not
enter planning.
