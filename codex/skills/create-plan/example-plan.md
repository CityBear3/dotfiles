# Example implementation plan

> **Execution:** Run this plan only after user approval.

**Goal:** Add deterministic parsing for a new input form.

**Architecture:** Extend the parser at its existing component boundary. Keep CLI
behavior in the binary layer and parsing behavior in the library.

**Working directory:** `.`
**Branch:** `feature/input-form`
**Baseline:** `cargo test` passes.
**Task verification:** `cargo test && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check`

## Fixed decisions and non-goals

- Accept the new form through the existing parser API.
- Preserve existing forms and malformed-input behavior.
- Do not change persistence, permissions, concurrency, or CLI ownership.

## Review context

- **Artifact and purpose:** A Rust library parser plus its existing CLI journey;
  accept one approved representation without changing component ownership.
- **Consumers:** Library callers receive parsed values and the CLI delegates to
  that library API.
- **Material criteria:** Deterministic parsing, unchanged existing syntax and
  error behavior, and one real CLI path.
- **Material failures:** Accepting malformed input, regressing an existing form,
  returning an incomplete value, or bypassing the library boundary.
- **Approved non-problems:** Exhaustive grammar fuzzing and performance tuning are
  outside this task; their absence is not a defect by itself.
- **Inapplicable assumptions:** Persistence, database identity, permissions, and
  concurrency do not apply unless the implementation adds such a path.
- **New-evidence rule:** Revisit a non-problem only with new evidence of a
  concrete reachable regression or approved-contract violation.

## Review policy

- **Mode:** `adaptive`.
- **Rationale:** Observable library parsing and its CLI journey change.
  Independent task review plus API-focused final review cover that contract.
- **Risk surfaces:** Public parsing behavior and library-to-CLI integration.
  Error and recovery behavior are explicitly unchanged.
- **Per-task gate:** Independent read-only `spec-reviewer` and
  `code-quality-reviewer`; rerun both after a correction.
- **Final required reviewers:** `code-reviewer`, `test-coverage-reviewer`, and
  `adversarial-api-reviewer`, followed by `adversarial-integrator`.
- **Final conditional reviewers:** Add `adversarial-robustness-reviewer` if the
  diff changes malformed-input handling, returned errors, or recovery.
- **Explicitly skipped perspectives:** Skip `design-alignment-reviewer` because
  no Design Doc is needed; `scope-reviewer` because the exact task and per-task
  specification gate cover scope; `code-architect` because ownership stays
  fixed; `adversarial-performance-reviewer` because no measurable hot path
  changes; and `adversarial-tests-reviewer` because no doubles, fixtures, or test
  infrastructure change.
- **Residual risk:** No fuzzing, exhaustive grammar coverage, or performance
  measurement. Acceptance covers the specified form, existing forms, and one
  real CLI journey.
- **Capacity:** Use at most four total threads including the lead, or lower
  observed capacity. Queue without reducing approved scope.
- **Acceptance:** Keep only artifact-applicable Must Fix or Should Improve
  findings with a reachable input, approved requirement, concrete impact, and
  proportionate correction. Should Improve requires a concrete maintainability
  consequence or measurable repeated cost. Drop preference, speculation,
  second-order concerns, inapplicable assumptions, and objections without new
  evidence. Treat an unproven architectural mechanism as `Escalate`.

## Task 1: Parse the new form

**Why:** The library currently rejects a supported representation.

**Behavior change:** yes
**Discipline:** TDD

**Files:**

- Modify: `src/parser.rs`
- Create: `src/parser_tests.rs`

### Steps

- [ ] Add a unit test that supplies the new form and asserts the complete parsed
      value.
- [ ] Run the focused test and observe the expected assertion failure.
- [ ] Implement the smallest parser change through the existing component API.
- [ ] Run the exact task verification command; expect all checks to pass.
- [ ] Commit only the parser and its unit test.

## Final verification

Run the task verification and one representative CLI smoke test.

## Post-review iteration

Route a concrete in-scope `Fix` through bounded implementation, fresh
verification, and fresh review.

## Push and PR

Do not publish without explicit user approval.
