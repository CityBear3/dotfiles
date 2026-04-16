---
name: test-driven-development
description: |
  Test-Driven Development skill. Write the test first, watch it fail, write minimal code to pass.
  Cross-cutting skill — invoked during implement when writing new functionality or fixing bugs.
---

# Test-Driven Development (TDD)

Write the test first. Watch it fail. Write minimal code to pass.

**Core principle:** If you didn't watch the test fail, you don't know if it tests the right thing.

## The Iron Law

```
NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST
```

Write code before the test? Delete it. Start over. No exceptions.

## When to Use

**Always** during `execute-plan` for:
- New features
- Bug fixes
- Behavior changes

**Exceptions (ask the engineer):**
- Throwaway prototypes
- Generated code
- Configuration files

## Red-Green-Refactor

### RED — Write Failing Test

Write one minimal test showing what should happen.

Requirements:
- One behavior per test
- Clear name that describes the behavior
- Real code (no mocks unless unavoidable)

### Verify RED — Watch It Fail

**MANDATORY. Never skip.**

Run the test. Confirm:
- Test fails (not errors)
- Failure message is expected
- Fails because feature is missing (not typos)

Test passes? You're testing existing behavior. Fix the test.

### GREEN — Minimal Code

Write the simplest code to pass the test. Don't add features, refactor other code, or "improve" beyond what the test requires.

### Verify GREEN — Watch It Pass

**MANDATORY.**

Confirm:
- Test passes
- Other tests still pass
- Output is clean (no errors, warnings)

Test fails? Fix code, not the test. Other tests fail? Fix now.

### REFACTOR — Clean Up

After green only:
- Remove duplication
- Improve names
- Extract helpers

Keep tests green. Don't add behavior.

### Repeat

Next failing test for next behavior.

## Bug Fix Pattern

1. Write a failing test that reproduces the bug
2. Verify it fails (RED)
3. Fix the bug (GREEN)
4. Verify the regression test now passes
5. Verify no other tests broke

Never fix bugs without a test.

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Writing production code before the test | Delete the code. Write the test first. |
| Test passes immediately (never saw it fail) | You're testing existing behavior or the wrong thing. Fix the test. |
| "I'll write tests after" | Tests written after pass immediately, proving nothing. Test first. |
| Keeping pre-written code "as reference" | That's testing after with extra steps. Delete and start from tests. |
| Adding features beyond what the test requires | Minimal code only. Next feature needs its own test first. |
| Mocking everything | Code is too coupled. Use dependency injection or simplify design. |

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "Too simple to test" | Simple code breaks. A test takes 30 seconds. |
| "I'll test after" | Tests-after answer "what does this do?" Tests-first answer "what should this do?" |
| "TDD will slow me down" | TDD is faster than debugging. Pragmatic = test-first. |
| "Deleting X hours of work is wasteful" | Sunk cost fallacy. Keeping unverified code is technical debt. |
| "Need to explore first" | Fine. Throw away the exploration. Start with TDD. |
| "Test is hard to write = skip it" | Hard to test = hard to use. Listen to the test. Simplify the design. |
| "Existing code has no tests" | You're improving it. Add tests for what you touch. |
| "Manual testing is faster" | Manual testing doesn't prove edge cases and can't be re-run automatically. |

## Rules

- This is a cross-cutting skill invoked during `execute-plan`. It does not define its own transitions.
- The engineer can explicitly exempt specific code from TDD. Claude Code cannot self-exempt.
- When stuck: write the assertion first, then work backward to the test setup.