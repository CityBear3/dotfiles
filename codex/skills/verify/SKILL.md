---
name: verify
description: Perform fresh post-implementation verification of an approved change, including build, tests, lint, formatting, diff, and readability checks. Use after execute-plan or when the user asks for a completion check.
---

# Verify implementation

No completion claim without fresh observed evidence.

## Scope

Read repository guidance, the approved plan, changed files, and the current diff. Resolve authoritative project commands before running generic defaults.

If agents are allowed and a named profile is selectable, use `implementation-verifier`. If it is not selectable, give a generic subagent the complete verification contract. When the user prohibits agents, perform the same checks directly.

## Checks

Run, as applicable:

1. the plan's final verification command;
2. focused tests for changed behavior;
3. owning package or workspace tests;
4. build or type check;
5. lint with warnings treated according to project policy;
6. format check;
7. relevant smoke or snapshot checks;
8. `git diff --check`, diff inspection, and final status.

Do not replace repository wrappers with broader commands that change semantics. Ask before unusually expensive full-workspace checks when repository policy requires it.

## Evaluate

Distinguish:

- implementation failure;
- test or tooling failure;
- unavailable dependency or permission;
- unrelated pre-existing failure;
- skipped or unverified check.

Do not mark a failure as acceptable without evidence that it is unrelated and outside scope.

## Report

Return:

- verdict: PASS, FAIL, or BLOCKED;
- commit/range and files inspected;
- every command and observed result;
- plan criteria satisfied;
- failures and likely ownership;
- checks not run and why.

Transition to review only on PASS.
