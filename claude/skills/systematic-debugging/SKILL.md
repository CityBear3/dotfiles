---
name: systematic-debugging
description: |
  Systematic root cause investigation before attempting fixes. Use when encountering any bug,
  test failure, or unexpected behavior. Requires completing investigation before proposing fixes.
  Cross-cutting skill — can be invoked at any stage.
---

# Systematic Debugging

Find root cause before attempting fixes. Random fixes waste time and create new bugs.

**Announce at start:** "I'm using the systematic-debugging skill to investigate this issue."

**Core principle:** ALWAYS find root cause before attempting fixes. Symptom fixes are failure.

## The Iron Law

```
NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST
```

If you haven't completed Phase 1, you cannot propose fixes.

## When to Use

Use for ANY technical issue:
- Test failures
- Bugs in production
- Unexpected behavior
- Performance problems
- Build failures
- Integration issues

Use this ESPECIALLY when:
- Under time pressure (emergencies make guessing tempting)
- "Just one quick fix" seems obvious
- You've already tried multiple fixes
- You don't fully understand the issue

## The Four Phases

Complete each phase before proceeding to the next.

### Phase 1: Root Cause Investigation

BEFORE attempting ANY fix:

1. **Read Error Messages Carefully**
   - Don't skip past errors or warnings
   - Read stack traces completely
   - Note line numbers, file paths, error codes

2. **Reproduce Consistently**
   - Can you trigger it reliably?
   - What are the exact steps?
   - If not reproducible → gather more data, don't guess

3. **Check Recent Changes**
   - What changed that could cause this?
   - Git diff, recent commits
   - New dependencies, config changes

4. **Trace Data Flow**
   - Where does the bad value originate?
   - What called this with the bad value?
   - Keep tracing up until you find the source
   - Fix at source, not at symptom

### Phase 2: Pattern Analysis

1. **Find Working Examples** — Locate similar working code in the same codebase
2. **Compare Against References** — Read reference implementations completely, don't skim
3. **Identify Differences** — List every difference between working and broken, however small
4. **Understand Dependencies** — What settings, config, or environment does this need?

### Phase 3: Hypothesis and Testing

1. **Form Single Hypothesis** — State clearly: "I think X is the root cause because Y"
2. **Test Minimally** — Make the SMALLEST possible change to test the hypothesis. One variable at a time.
3. **Verify Before Continuing** — Did it work? Yes → Phase 4. No → form NEW hypothesis. Don't add more fixes on top.

### Phase 4: Implementation

1. **Create Failing Test Case** — Use the `test-driven-development` skill
2. **Implement Single Fix** — Address the root cause. ONE change at a time. No "while I'm here" improvements.
3. **Verify Fix** — Test passes? No other tests broken? Issue actually resolved?
4. **If 3+ Fixes Failed: Question Architecture** — Each fix reveals new problems in different places → this is an architectural issue, not a bug. Stop and discuss with the engineer.

## Transition

After root cause is found and fix scope is understood:

| Fix Scope | Transition |
|-----------|-----------|
| Any fix | → `/create-plan` → `/execute-plan` → `/verify` |
| Design change required | → `/design-discussion` → full flow |

The engineer decides the transition.

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Proposing fixes before completing Phase 1 | Finish investigation first. |
| "Quick fix for now, investigate later" | Investigate now. Quick fixes become permanent. |
| "Just try changing X and see if it works" | Form a hypothesis first. Test one variable. |
| Adding multiple changes at once | One change at a time. Can't isolate what worked otherwise. |
| "It's probably X, let me fix that" | "Probably" means you haven't verified. Verify first. |
| Proposing solutions before tracing data flow | Trace the flow. Then propose. |
| 3+ fix attempts without questioning architecture | Stop. This might be a design problem, not a bug. |

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "Issue is simple, don't need process" | Simple issues have root causes too. Process is fast for simple bugs. |
| "Emergency, no time for process" | Systematic debugging is FASTER than guess-and-check thrashing. |
| "Just try this first, then investigate" | First fix sets the pattern. Do it right from the start. |
| "I see the problem, let me fix it" | Seeing symptoms ≠ understanding root cause. |
| "One more fix attempt" (after 2+ failures) | 3+ failures = question the architecture. Don't fix again. |
| "Reference too long, I'll adapt the pattern" | Partial understanding guarantees bugs. Read it completely. |

## Rules

- Phase 1 is mandatory. No exceptions.
- Report findings to the engineer before proposing fixes.
- If the engineer is the executor of the fix, shift to support role after investigation.
- When 3+ fixes fail, escalate to the engineer for architectural discussion.