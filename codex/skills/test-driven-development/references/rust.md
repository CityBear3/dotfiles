# Rust testing guidance

Use this reference when writing or restructuring Rust tests.

## Choose the test boundary

Follow repository guidance and the approved design before these personal
defaults. Choose placement by the behavior being verified:

| Test boundary | Placement |
| --- | --- |
| Behavior through a module's public API | Corresponding `<module>_test.rs` |
| Internal implementation responsibilities and detailed conditions | Inline in the implementation file, or corresponding `<file_name>_test.rs` |
| Composed components or a real binary/process journey | Cargo `tests/` targets |

Public API tests verify the module's public contract without depending on its
internal helpers or representation. Keep internal unit tests near the
implementation that owns their responsibility. Filesystem use or calling a
public API alone does not make a test an integration test. Do not widen
production visibility solely for tests or migrate unrelated existing layouts.

When using a separate sibling test file, connect it from the owning module:

```rust
#[cfg(test)]
#[path = "parser_test.rs"]
mod tests;
```

- Use doctests for concise public API examples. Use `examples/` for representative whole-API usage, and ensure CI runs them when they carry behavior expectations.

## Write the case

Use the [shared TDD guidance](../SKILL.md) for Arrange/Act/Assert, one viewpoint
per test and DAMP setup.

- Give each case a behavioral name stating the condition and expected result.
- Compare the complete returned value when practical. Add side-effect assertions when the side effect is the viewpoint.
- Do not test facts already guaranteed by Rust's type system.

## Filesystem and process state

- Give every test isolated paths and cleanup ownership.
- Never read or modify the user's real home, Codex configuration, or unrelated project files.
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
