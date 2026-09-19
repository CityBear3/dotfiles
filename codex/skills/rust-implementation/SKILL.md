---
name: rust-implementation
description: Apply personal Rust conventions when editing production Rust code, module structure, documentation comments, error types, or Cargo settings.
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

## Separate definitions visibly

- Put one blank line between adjacent function, struct, enum, trait, and `impl`
  definitions, including between methods inside an `impl` or trait. Keep
  logically grouped imports, module declarations, fields, and variants compact.
- Keep a declaration's doc comments and attributes attached to it, with no blank
  line between them and the declaration. Place doc comments before attributes.
- Run the repository's scoped formatting command or `cargo fmt` with its selected
  toolchain, inspect the diff, and check definition spacing yourself. `rustfmt`
  does not necessarily insert these separators. Do not reformat unrelated code.

One blank line between definitions is this skill's readability default; the
[Rust Style Guide](https://doc.rust-lang.org/style-guide/) permits zero or one.
For example:

```rust
/// Owns a document's original text.
#[derive(Debug)]
pub struct Document {
    text: String,
}

impl Document {
    /// Stores text without normalizing whitespace or line endings.
    pub fn new(text: String) -> Self {
        Self { text }
    }

    /// Borrows the original text, including whitespace.
    pub fn text(&self) -> &str {
        &self.text
    }
}
```

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
- Choose a cohesive responsibility boundary before splitting files. Keep
  independent top-level responsibilities beside `main.rs` or `lib.rs`; group
  related implementation details under their parent module instead of adding
  every internal file at the crate root.
- For a new module `foo` with children, prefer `src/foo.rs` plus
  `src/foo/*.rs`. Do not introduce `src/foo/mod.rs` or mix both layouts in the
  same module tree unless repository guidance requires it.
- Let `foo.rs` own the module boundary: outward-facing types and operations,
  public errors, child declarations and re-exports, and high-level coordination.
- Let each `foo/*.rs` child own one cohesive internal responsibility with the
  narrowest useful visibility. Narrow local failures at the parent operation
  boundary when required by the selected error contract.
- Keep `main.rs` a thin entry adapter, including in binary-only crates. When a
  library exists, keep `lib.rs` focused on top-level declarations and necessary
  stable re-exports. This does not require extracting a library from a binary.
- A small cohesive module can stay in one file. Do not create directories by
  file-count thresholds or one file per type/function, and do not reorganize
  unrelated existing modules.

For example, a snapshot subsystem can own its reader and writer below a named
parent, while configuration remains an independent top-level responsibility:

```text
src/
  main.rs
  config.rs
  snapshot.rs
  snapshot/
    reader.rs
    writer.rs
```

See [Separating Modules into Different Files](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)
for how module declarations map to this hierarchy.

## Write documentation for readers of the code

Apply this coverage to new or changed production declarations within the task:

- Document crate and module responsibilities and boundaries with `//!`. Document
  items with `///`, using rustdoc Markdown and links where useful.
- Document types and traits by default regardless of visibility, including
  private and `pub(crate)` definitions. Explain their role and relevant
  invariants or usage constraints; describe fields and variants when their
  meaning needs explanation.
- Document public items, including functions and methods, with their observable
  behavior. Document non-public functions and methods when their purpose,
  assumptions, side effects, or constraints are not evident from the name and
  signature. A self-evident small helper may omit a doc comment.
- Describe the contract a reader needs, not a restatement of the name or types.
  Include applicable `# Errors`, `# Panics`, and, for unsafe APIs, `# Safety`
  obligations. Keep claims supported by the implementation and approved
  contract, and update documentation when behavior changes. Do not invent
  guarantees or add empty template sections.
- Use ordinary `//` comments for local implementation reasoning. Keep API and
  module documentation on the declaration it describes.
- For changed documentation, use scoped `cargo doc --no-deps` with the project's
  build settings; add `--document-private-items` when inspecting internal docs.
  Run scoped `cargo test --doc` for runnable examples on supported library
  targets. Do not add a library solely to enable doctests. Reuse equivalent
  repository checks and review prose as well as generated output.

See [How to write documentation](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html),
[API documentation guidelines](https://rust-lang.github.io/api-guidelines/documentation.html),
and [documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html).

## Preserve the planning boundary

Treat these conventions as implementation guidance. A plan should mention an
exact Rust type, signature, file, layout, or edition only when that identity is
part of a public or shared interface, compatibility, writer ownership, a
reproducible environment, or another observable correctness condition.
