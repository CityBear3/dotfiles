---
name: rust-implementation
description: Apply the user's reusable Rust conventions when creating or changing Rust production code, module layout, error types, or Cargo package settings. Use before editing Rust code or configuration; do not use for read-only discussion or language-neutral planning.
---

# Rust implementation guidance

Apply these defaults to Rust implementation work without copying them into a
Feature Contract, Task Contract, or Implementation Plan.

## Resolve authority first

- Follow the closest repository `AGENTS.md`, approved design and contracts, and
  established public API before these personal defaults.
- If project authority intentionally selects a consolidated error abstraction,
  including a stable library error type analogous to `std::io::Error`, preserve
  that abstraction instead of forcing operation-local error types.
- When existing code uses a different edition, module layout, or public error
  model, do not migrate unrelated code unless the approved task includes that
  migration.
- Stop for clarification when applicable authorities conflict and the choice
  would change a public or shared interface, compatibility, or failure behavior.

## Keep error contracts accurate

When project authority does not select another model:

- Give each fallible operation a closed error contract containing only failures
  that the operation can actually return.
- Reuse an error type when every returning operation can produce its complete
  variant set. Do not create a nominal type per function when the reachable sets
  are genuinely identical.
- Avoid module-wide or application-wide umbrella errors that admit unrelated
  failures. Keep an error with the operation or narrow responsibility whose
  failure set it describes.
- Map or narrow a lower-layer error at an operation boundary when propagating it
  would expose variants that the operation cannot return.
- Use `From`, `#[from]`, and `?` only when the conversion preserves the intended
  caller error surface. A convenient conversion must not widen that surface.
- Use `thiserror` when it is already available or its addition is authorized.
  Its availability does not authorize adding or changing a dependency.

Treat an intentionally consolidated project or library error as a chosen API
contract, not as an umbrella-error mistake. Judge it by that contract's
abstraction and compatibility requirements.

## Use named-parent modules for new structure

- For a new crate, use Rust edition 2024 unless project authority selects
  another edition. Do not upgrade an existing crate implicitly.
- For a new module `foo` with children, prefer `src/foo.rs` plus
  `src/foo/*.rs`. Do not introduce `src/foo/mod.rs` or mix both layouts in the
  same module tree unless repository guidance requires it.
- Let `foo.rs` own the module boundary: outward-facing types and operations,
  public errors, child declarations and re-exports, and high-level coordination.
- Let each `foo/*.rs` child own one cohesive internal responsibility with the
  narrowest useful visibility. Narrow local failures at the parent operation
  boundary when required by the selected error contract.
- When a crate contains both a library and binary, keep `lib.rs` focused on
  top-level declarations and necessary stable re-exports, and keep `main.rs` as
  a thin adapter unless the repository architecture says otherwise.

## Preserve the planning boundary

Treat these conventions as implementation guidance. A plan should mention an
exact Rust type, signature, file, layout, or edition only when that identity is
part of a public or shared interface, compatibility, writer ownership, a
reproducible environment, or another observable correctness condition.
