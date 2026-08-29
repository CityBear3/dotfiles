---
name: test-driven-development
description: Decide whether TDD applies, then use a focused red, green, refactor loop for testable production-code behavior changes and bug fixes. Use before editing when the task may require TDD or an approved plan specifies it.
---

# Test-driven development

Establish evidence that the test detects the missing behavior before writing production code.

For Rust work, read [references/rust.md](references/rust.md) before choosing test placement or structure.

## Decide whether TDD applies

Before editing, classify TDD as `applicable`, `not applicable`, or
`required but blocked` and record the reason.

- `applicable`: the task changes observable production-code behavior or fixes a
  defect, and a focused executable test can demonstrate the missing behavior
  before the production edit.
- `not applicable`: the task is limited to documentation, instructions,
  metadata, formatting, generated output, or a behavior-preserving mechanical
  change. Preserve an existing green baseline and run proportionate validation;
  do not manufacture a failing test or claim TDD.
- `required but blocked`: behavior changes or approved authority requires TDD,
  but the intended RED cannot be observed safely because the test seam,
  environment, or authority is missing. Stop and report the missing condition
  instead of silently reclassifying the work.

Updating an existing test whose old expectation would contradict an approved
content or contract change is maintenance, not by itself a TDD RED.

Independent initial authority reads, repository searches, relevant file reads,
and Git inspection may run before the first TDD stage in one bounded
programmatic batch only when every result remains separately attributable. End
that batch and stop before a result-dependent judgment, test selection, edit,
or dependent validation.

Only when TDD is `applicable`, keep this order explicit and never batch across
it:

```text
focused RED -> production edit -> focused GREEN -> refactor while green
```

Run independent mechanical post-edit checks only after focused GREEN.

## Red

1. Select one observable behavioral viewpoint.
2. Write the smallest test that expresses the expected result.
3. Structure the test as Arrange, Act, Assert.
4. Run the focused test and confirm it fails for the intended reason.

A compile error caused by an intentionally missing API can be a valid initial red result. An unrelated setup, fixture, or syntax failure is not.

## Green

Implement the smallest production change that satisfies the test. Do not add speculative abstractions, unrelated cleanup, or extra behavior.

Run the focused test until it passes, then run relevant neighboring tests.

## Refactor

Improve names, responsibilities, and duplication while the tests remain green. Prefer readable DAMP tests over shared helpers that hide each case's meaning.

## History and current evidence

When TDD applies, record the actual pre-production RED and its reason, the
subsequent production edit, focused GREEN, and any refactor, and never recreate
or repair historical RED evidence after the production edit. Otherwise record
the applicability decision, reason, baseline, and validation used. Disclose an
unrepairable historical discipline gap. It is not an Acceptance blocker by
itself unless it exposes a reachable current defect, material current evidence
gap, material contract deviation, or controlling authority that makes the
history material. Current Acceptance still requires adequate tests, fresh
verification, and selected review for the exact current target.

## Test contract

- One test covers one behavioral viewpoint, not necessarily one function call.
- Assert returned values and errors directly.
- Assert side effects when they are part of that test's behavior.
- Prefer complete value comparisons over a sequence of weak field assertions.
- Keep tests deterministic, independent, and parallel-safe.
- Use real collaborators when inexpensive; fake only at a meaningful boundary.
- Do not change production visibility solely to reach internals.

Never claim TDD if the red result was not observed.
