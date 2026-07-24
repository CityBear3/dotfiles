# Rust review hints

Use these prompts selectively. Repository guidance and the approved design take precedence.

## Robustness

- Check `unwrap`, `expect`, indexing, arithmetic, and asserted unreachable states against actual input and invariant boundaries.
- Verify errors are propagated or deliberately transformed rather than discarded with `.ok()`, `let _ =`, or broad defaults.
- Inspect recursion, loops, and retries for termination on user-controlled input.
- Check lock or borrow guards held across callbacks or `.await`.
- Verify async branches are cancellation-safe and blocking work stays off executor workers.
- Review persisted formats, rename/write protocols, and cleanup ordering for observable partial states when those behaviors changed.

## API

- Use `as_`, `to_`, and `into_` consistently with borrow, copy, and ownership transfer.
- Prefer borrowed inputs such as `&str`, `&[T]`, and `&Path` when ownership is not retained.
- Implement `From` for infallible conversion and `TryFrom` for fallible conversion.
- Keep visibility as narrow as the component contract allows.
- Check public error types, enums, serialized shapes, and function signatures for downstream compatibility.
- Treat traits as capabilities and avoid using them only to mimic class inheritance.

## Performance

- Investigate cloning, allocation, collection, formatting, and repeated lookup only on a demonstrated repeated or hot path.
- Preserve iterator laziness when intermediate materialization has no purpose.
- Check sequential independent I/O or awaits for avoidable waterfalls.
- Require measurement or a clear input-size/caller-frequency argument before reporting an optimization finding.

## Tests

- Confirm a bug fix has a regression test that would fail before the fix.
- Select test placement by boundary, not by filesystem use: component behavior belongs in unit tests; Cargo `tests/` is for public-crate, multi-component, or real process journeys.
- For a new unit-test module, prefer a descriptive sibling such as `<module>_tests.rs` connected with `#[cfg(test)]` and explicit `#[path]`; do not require a generic `test.rs`.
- Review tests through meaningful public or `pub(crate)` component APIs; production internals should not be exposed solely for tests.
- Require visible Arrange, Act, Assert and one behavioral viewpoint per test.
- Prefer DAMP case-local setup when a shared fixture would hide why values matter.
- Assert the complete returned domain value when practical. Assert a side effect when it is the behavior under test.
- Reject tests that only prove the mock, only check success, or duplicate guarantees already enforced by the type system.
- Check isolation of paths, static state, environment, current directory, clocks, and ports under parallel execution.
- Never permit tests to touch the user's real home or configuration.
- For binary journeys, use the Cargo-built binary rather than a behavioral shim.
- Review snapshots and generated expected files as behavior changes; do not accept them mechanically.

These hints do not impose module-length, test-count, or coverage-percentage quotas.
