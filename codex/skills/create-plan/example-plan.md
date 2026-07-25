# Example implementation plan

> **Execution:** Run this plan only after user approval.

**Goal:** Add deterministic parsing for a new input form.

**Architecture:** Extend the parser at its existing component boundary. Keep CLI behavior in the binary layer and parsing behavior in the library.

**Working directory:** `.`
**Branch:** `feature/input-form`
**Baseline:** `cargo test` passes.

**Per-task verification:** `cargo test && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check`

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
