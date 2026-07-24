---
name: design-doc
description: Support a user-authored formal Design Doc through structure, research, ambiguity detection, and post-draft critique. Use when settled engineering work has cross-cutting architecture, durable contracts, or significant decisions worth preserving.
---

# Design Doc support

The user authors the design and its prose. Preserve ideation primacy: do not supply design-specific prose before the user has written a first draft.

## Allowed support

- investigate codebase facts and constraints;
- challenge assumptions and compare alternatives;
- provide the fixed template skeleton;
- identify ambiguity, missing rationale, duplicated concepts, and unresolved contracts;
- critique user-authored text;
- make targeted edits only when the user explicitly asks.

Do not invent components, name a design-specific decomposition, or ghostwrite unfinished sections.

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

## Review

After the user completes a draft, check:

- goals and non-goals are distinguishable;
- terms and assumptions are defined;
- requirements are measurable where necessary;
- public contracts, failure behavior, and invariants are explicit;
- alternatives include the reason they were rejected;
- diagrams or tables clarify real relationships instead of decorating prose;
- implementation details do not obscure design decisions.

When the user approves the document, recommend `create-plan` and wait for approval before transitioning.
