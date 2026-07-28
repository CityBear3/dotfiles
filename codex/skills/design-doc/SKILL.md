---
name: design-doc
description: Draft or support a formal Design Doc from user-owned decisions through research, ambiguity detection, structured writing, and critique. Use when settled engineering work has cross-cutting architecture, durable contracts, or significant decisions worth preserving.
---

# Design Doc support

Keep every material design decision under user control. Treat prose authorship as
separate from decision ownership.

## Settle decisions before drafting

Load the decision record from design discussion. Investigate relevant code,
tests, documentation, and history. Challenge assumptions, compare viable
alternatives, and ask detailed questions until no material decision remains
unresolved.

Require the record to identify:

- the selected approach and rationale;
- rejected alternatives and reasons;
- scope and non-goals;
- explicitly deferred questions.

When a missing choice could change architecture, responsibilities, public
contracts, schemas, error models, or scope, return the ambiguity and evidence to
`agentic-engineering-workflow` for design discussion. Do not choose silently.

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
