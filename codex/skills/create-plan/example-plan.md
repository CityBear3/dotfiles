# Example implementation plan

> **Execution:** Run this plan only after user approval.

**Goal:** Add deterministic parsing for a new input form.

**Architecture:** Extend the parser at its existing component boundary. Keep CLI behavior in the binary layer and parsing behavior in the library.

**Working directory:** `.`
**Branch:** `feature/input-form`
**Baseline:** `cargo test` passes.

**Per-task verification:** `cargo test && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check`

## Review policy

- **Mode:** `adaptive`.
- **Rationale:** The new accepted syntax changes observable library parsing
  behavior and its CLI journey. Independent specification and quality checks plus
  API-focused final review address that contract without unrelated specialist
  review.
- **Risk surfaces:** Public parsing behavior and library-to-CLI integration
  change. Error and recovery behavior are explicitly unchanged; no persistence,
  permission, concurrency, performance-hot-path, cross-component ownership, or
  test-double risk is identified.
- **Per-task gate:** Run independent read-only `spec-reviewer` and
  `code-quality-reviewer` checks against the task commit. Re-run both after any
  correction.
- **Final required reviewers:** Run `code-reviewer`,
  `test-coverage-reviewer`, and `adversarial-api-reviewer`; then run
  `adversarial-integrator` over the adversarial result.
- **Final conditional reviewers:** Add `adversarial-robustness-reviewer` if the
  diff changes malformed-input handling, returned errors, or recovery behavior.
- **Explicitly skipped perspectives:** Skip `design-alignment-reviewer` because
  no Design Doc is needed; `scope-reviewer` because one exact task and its
  per-task specification gate cover the declared scope; `code-architect` because
  component responsibilities stay fixed; `adversarial-performance-reviewer`
  because no performance-sensitive path or objective changes; and
  `adversarial-tests-reviewer` because no doubles, fixtures, or test
  infrastructure change.
- **Residual risk:** This plan does not add fuzzing, exhaustive grammar coverage,
  or performance measurement. Acceptance covers the specified input form,
  existing forms, and one real CLI journey.
- **Capacity:** Use at most four total threads including the lead, or the lower
  observed runtime capacity. Queue the per-task reviewers, final reviewers, and
  adversarial integration rather than reducing the approved scope.
- **Acceptance threshold:** Keep only Must Fix or Should Improve findings with a
  reachable input, cited requirement or code evidence, concrete impact, and a
  specific correction. Drop preference-only, speculative, and already-decided
  objections without new evidence.

### Task 1: Parse the new form

**Why:** The library currently rejects a supported representation.

**Behavior change:** yes
**Discipline:** TDD

**Files:**

- Modify: `src/parser.rs`
- Create: `src/parser_tests.rs`

### Steps

- [ ] Add a unit test that supplies the new form and asserts the complete parsed value.
- [ ] Run the focused test and observe the expected assertion failure.
- [ ] Implement the smallest parser change through the existing component API.
- [ ] Run the per-task verification command.
- [ ] Commit only the parser and its unit test.

## Final verification

Run the per-task verification command and one representative CLI smoke test.

## Post-review iteration

Reserved for verified review findings.

## Push and PR

Do not publish without explicit user approval.
