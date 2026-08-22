# Rust testing guidance

Use this reference when writing or restructuring Rust tests.

## Choose the test boundary

- Keep module or component behavior in unit tests, including behavior that creates files or inspects filesystem side effects.
- Put a new unit-test module in a descriptive sibling file such as `parser_tests.rs`:

```rust
#[cfg(test)]
#[path = "parser_tests.rs"]
mod tests;
```

- Exercise the unit through its meaningful public or `pub(crate)` component API. Do not expose private implementation solely for tests.
- Use Cargo `tests/` targets for a crate's public API, multiple composed components, or a real binary/process journey.
- Use doctests for concise public API examples. Use `examples/` for representative whole-API usage, and ensure CI runs them when they carry behavior expectations.

## Write the case

- Give the test a behavioral name that states the condition and expected result.
- Make Arrange, Act, and Assert visible in the test body.
- Keep one behavioral viewpoint per test.
- Prefer DAMP setup in the case over generic fixtures that hide why values matter.
- Compare the complete returned value when practical. Add side-effect assertions when the side effect is the viewpoint.
- For bug fixes, first observe the regression test failing against the unfixed behavior.
- Do not test facts already guaranteed by Rust's type system.

## Filesystem and process state

- Give every test isolated paths and cleanup ownership.
- Never read or modify the user's real home, Claude Code configuration, or unrelated project files.
- Follow the repository's declared test-temp root; use RAII cleanup.
- Pass paths, environment-derived values, clocks, and dependencies into the component where practical.
- Avoid mutating process-global environment because Rust tests run concurrently.
- When a real binary is the subject, launch the Cargo-built binary rather than a behavioral shim.

## Assertions and failures

- Prefer equality on whole domain values and errors so diffs expose the actual mismatch.
- Assert structured output rather than reparsing strings when a structured API exists.
- Use `unwrap` or `expect` for test setup only when failure means the test cannot proceed; make the message explain the setup contract.
- Avoid timing sleeps. Synchronize on observable events or bounded deterministic conditions.

## Verification

Run the narrow test during red/green iterations, then the owning crate's test, format, and lint commands. Use repository wrappers such as `just test` when local guidance requires them.

These conventions do not impose file-length, test-count, or coverage-percentage quotas.
