---
name: test-driven-development
description: Select whether test-first development applies, then preserve causal red, green, and refactor evidence for a focused behavior or coherent matrix. Use before editing when production behavior, a defect, or an approved plan may require TDD.
---

# Test-driven development

Select the reliable oracle for the material property. When TDD applies,
establish evidence that its test or coherent matrix detects the missing behavior
before writing production code.

For Rust work, read [references/rust.md](references/rust.md) before choosing test placement or structure.

## Decide whether TDD applies

Before editing, identify the material property and available oracle, then
classify TDD as `applicable`, `not applicable`, or `required but blocked` and
record the reason and selected validation route.

- `applicable`: the task changes observable production-code behavior or fixes a
  defect, and a focused executable test or coherent test matrix can demonstrate
  the missing behavior before the production edit.
- `not applicable`: the task does not add or change observable production
  behavior, approved authority selects non-test-first evidence because it is the
  reliable oracle for the material property, or the task is explicitly
  authorized exploratory work that cannot become a production Candidate.
  Preserve a relevant baseline and run the selected proportionate validation;
  do not manufacture a failing test or claim TDD.
- `required but blocked`: behavior changes or approved authority requires TDD,
  but the intended RED cannot be observed safely because the test seam,
  environment, or authority is missing. Stop and report the missing condition
  instead of silently reclassifying the work.

Updating an existing test whose old expectation would contradict an approved
content or contract change is maintenance, not by itself a TDD RED. Difficulty
alone does not make a production behavior change non-applicable; select a
property, model, differential, fault, integration, or other executable oracle
when that is what proves the behavior.

Independent initial authority reads, repository searches, relevant file reads,
and Git inspection may run before the first TDD stage in one bounded
programmatic batch only when every result remains separately attributable. End
that batch and stop before a result-dependent judgment, test selection, edit,
or dependent validation.

Only when TDD is `applicable`, choose the smallest causal behavioral slice and
keep this order explicit:

```text
focused RED -> one causal production edit -> focused GREEN -> refactor while green
```

Several cases may form one RED/GREEN matrix only when they express the same
missing capability, remain separately attributable, and no result changes
another case's design, selection, or input. Otherwise keep the cases sequential.
Never batch across the selected slice's RED, production edit, or GREEN boundary.
Run independent mechanical post-edit checks only after focused GREEN.

## Red

1. Select one causal behavioral slice.
2. Write the smallest test or coherent matrix that expresses the expected
   result; each test still covers one behavioral viewpoint.
3. Structure each test as Arrange, Act, Assert.
4. Run the focused test or matrix and confirm every RED fails for the intended
   missing behavior.

A compile error caused by an intentionally missing API can be a valid initial red result. An unrelated setup, fixture, or syntax failure is not.

## Green

Implement the smallest production change that satisfies the test. Do not add speculative abstractions, unrelated cleanup, or extra behavior.

Run the focused test or coherent matrix until it passes, then run relevant
neighboring checks.

## Refactor

Improve names, responsibilities, and duplication while the tests remain green. Prefer readable DAMP tests over shared helpers that hide each case's meaning.

## History and current evidence

When TDD applies, record the causal slice, actual pre-production RED results and
reasons, subsequent production edit, focused GREEN results, and any refactor,
and never recreate or repair historical RED evidence after the production edit.
Otherwise record the applicability decision, reason, baseline, and validation
used. Disclose an
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

## Exploration and non-example evidence

A time-bounded exploratory implementation may establish feasibility, expose an
environment constraint, or help derive an oracle. Record its hypothesis and
result, but do not call it TDD or treat it as a production Candidate. Before
productionization, make the intended behavior and contract-appropriate current
verification explicit.

Example tests do not replace property, model, differential, fault-injection,
stress, integration, emulator or hardware, or benchmark evidence when one of
those is the reliable oracle for a material state-space, equivalence,
concurrency, failure, environment, or performance obligation. Keep correctness
and performance evidence distinct.

Never claim TDD if the intended RED result was not observed.
