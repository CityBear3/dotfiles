---
name: investigate
description: |
  Codebase investigation and understanding skill. Use before design-doc, plan, or systematic-debugging
  when the problem space, existing architecture, or scope needs to be understood first.
  Invoke with `/investigate <topic or question>`.
---

# Investigate

Explore and understand the codebase, problem space, or existing behavior before making decisions.

**Announce at start:** "I'm using the investigate skill to explore this topic."

## When to Use

- Before writing a Design Doc — understand existing architecture and constraints
- Before debugging — understand what the expected behavior should be
- Before planning — understand scope and affected areas
- When the engineer asks "how does X work?" or "what would be affected by Y?"

## The Process

### Step 1: Scope the Investigation

Clarify what the engineer wants to understand:
- A specific component or module?
- The impact of a proposed change?
- How a particular behavior works end-to-end?
- What constraints or patterns exist in the codebase?

If `$ARGUMENTS` is unclear, ask one focused question to narrow scope.

### Step 2: Gather Evidence

Use the `code-architect` agent or direct codebase exploration:

1. **Structure**: Identify relevant files, modules, and their responsibilities
2. **Dependencies**: Map what depends on what in the area of investigation
3. **Patterns**: Identify conventions and patterns already in use
4. **History**: Check git history for recent changes and their rationale if relevant
5. **Constraints**: Identify external dependencies, API contracts, or invariants

Do not speculate. Report what you find, supported by file paths and code references.

### Step 3: Present Findings

Present a structured summary:

```
## Investigation: <topic>

### Relevant Components
<files and modules, with their responsibilities>

### How It Works
<factual description of current behavior or architecture>

### Constraints and Dependencies
<what must be preserved or considered>

### Open Questions
<anything unclear that needs the engineer's input>
```

### Step 4: Transition

After the engineer reviews findings, ask how to proceed:

| Finding | Suggested Next Skill |
|---------|---------------------|
| Design change needed | → `design-doc` |
| Bug or unexpected behavior found | → `systematic-debugging` |
| Scope is clear, ready to plan | → `plan` |
| Small, well-understood fix | → `implement` (with engineer approval to skip planning) |

The engineer decides the transition. Do not proceed autonomously.

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Making design recommendations during investigation | Present facts. The engineer decides. |
| Investigating broadly without focus | Scope first, then investigate. |
| Reporting assumptions as findings | Only report what you verified in the code. |
| Skipping investigation because "it seems simple" | The engineer decides when investigation is unnecessary. |
| Proposing fixes before the engineer has reviewed findings | Present findings → wait for the engineer's decision → then transition. |

## Rules

- Present findings as factual observations, not recommendations
- Use file paths and code references to support every claim
- When multiple patterns exist in the codebase, report all of them
- Clearly distinguish between what the code does and what it appears to intend
- Do not write or modify code during investigation