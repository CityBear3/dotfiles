<!--
This file is a real plan from the Dyne compiler project (PR-3c, generics +
match exhaustiveness), illustrating the standard plan format described in
SKILL.md. The plan demonstrates:

- Mixed Discipline tags (TDD for behavior-changing tasks, refactor for
  the binding_def_ids consolidation, mixed for the final cleanup task).
- A "Decisions reference" header recording sub-decisions settled during
  /design-discussion (Q1–Q6 here), separate from architectural decisions
  that live in the Design Doc.
- Embedded regression tests in the task that introduces the behavior, not
  split into a separate "tests-only" task.
- Forward references to upstream/downstream PRs (PR-3b carries bundled
  per Q6; PR-3d/3e items deferred).
- A "Post-/review iteration" section reserving budget for review-fix tasks
  per CLAUDE.md's autonomous Core Flow loop.
- An "Alternative Solutions Considered" section recording sub-decision
  alternatives that were rejected, with rationales.

Reference only. To regenerate, copy a fresh plan from a real project and
adjust this header.
-->

# Stage 3c Generics + Match Exhaustiveness Implementation Plan

> **Execution:** Use `/execute-plan` to dispatch this plan to agent-teams. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Land generic enums (annotation, constructor, match-pattern), built-in `Option<T>` / `Result<T, E>`, and match exhaustiveness checking, while closing PR-3b's silent variant-constructor soundness gap and bundling 3 documented PR-3b carry-overs.

**Architecture:** Eight sequential tasks. Task 1 introduces `Ty::Param(usize)` sentinel and extends `lower_type` for generic-type annotations. Task 2 adds `binding_def_ids` table and collapses 5 linear-scan helpers (refactor). Task 3 extends `signature_pass` to produce variant schemas with `Ty::Param` and adds outer-enum first-writer-wins gate. Tasks 4–5 wire generic instantiation through `synth_ident` / `synth_call` / `check_pattern` / `check_match_arm` using the unification table that PR-3b already plumbed. Task 6 lands `Option<T>` / `Result<T, E>` via `builtins.dy` embedded source. Task 7 adds `sema/exhaust.rs` with per-scrutinee table-driven exhaustiveness + 1-level payload recursion. Task 8 wires end-to-end (adapt `option_match.dy` back to `Option<T>`, e2e tests) and bundles 3 PR-3b cleanup carries.

**Tech Stack:** Rust 2024 edition. Zero runtime deps.

**Working directory:** `~/workspace/calculator/.claude/worktrees/stage3c-generics` (cargo CWD: `<worktree>/compiler`).
**Branch:** `stage3c-generics`.
**Baseline before Task 1:** 319 tests passing (294 lib + 7 bin + 17 e2e + 1 samples), clippy `-D warnings` clean, `cargo fmt -- --check` clean. Main HEAD `7f651e2`.

**Per-task verification command** (mandatory before each commit):
```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

**Decisions reference** (from `/design-discussion` 2026-05-08):

- **Q1 — Var-allowed in `Ty::Enum` args (β)**: `Ty::Enum(def_id, [Ty::Var(α)])` is a valid intermediate state during checking. `unify_or_diag`'s `resolve()` chain (PR-3b) transparently strips. Storage-time normalization removes any remaining Var.
- **Q2 — `Ty::Param(usize)` sentinel (B)**: New `Ty::Param(usize)` variant added to `Ty` enum. `def_types[variant_def_id]` stores HM-style schema (e.g. `Some` → `Ty::Function([Ty::Param(0)], Ty::Enum(option_def, [Ty::Param(0)]))`). `synth_ident` allocates fresh Vars per use, substitutes Param(i) → Var(α_i). Non-generic and generic share the same retrieval path. Closes PR-3b's silent variant-constructor soundness gap.
- **Q3 — Per-scrutinee table-driven exhaustiveness + 1-level payload recursion (I-extended)**: `exhaust.rs` dispatches per `Ty` kind (Enum / Bool / Int / Scalar / String / Vec / Mat / Array / Dict / Struct / Function / Var / Param / Error). 1-level recursion catches `case Some(Some(x))` / `case Some(None)` payload gaps. Maranget-arbitrary nesting deferred (future internal-only swap).
- **Q4 — Hybrid built-ins (Path 2)**: `Option<T>` / `Result<T, E>` declared in `compiler/builtins/builtins.dy` via `include_str!`. `compile()` loads built-ins → user source with shared `IdGenerator`. Built-ins compile failure is compile-time bug → panic.
- **Q5 — `binding_def_ids: BindingTable` new (r2)**: New table `HashMap<NodeId, DefId>` for intro→def mapping. `resolutions` remains use→def only (orthogonal). Resolver's `define_or_report` inserts intro NodeId → DefId. 5 linear-scan helpers collapse to O(1) lookups; Task 8 G3 corollary `(kind, name, span)` matching simplified.
- **Q6 — PR-3b carries bundled (s1)**: Three trivia (synth_pow `^` text, `mat_shape_mismatch` unused arg, `TypedProgram::new` 7-arg → struct literal inline) bundled in PR-3c with separate commits. Auto-resolved by PR-3c side effects: outer-enum first-writer-wins gate (signature_pass restructure), `Option<X>` cross-layer cascade (built-in registration). PR-3d carry: `synth_arith` Mat·Vec arm order.

**TDD discipline note** (applies to every TDD-tagged task below): each task follows the **red → green → refactor** cycle per `/test-driven-development`. The plan specifies WHAT (which tests, which behaviors); the skill drives HOW. Each TDD task ends with a **Refactor** step that consolidates duplicates introduced during green, polishes naming, and folds in any cleanup the green pass deferred. The refactor lands in the same commit as green when small, or as a separate commit when substantive (e.g., extracting a shared helper used by 3+ sites).

**Review-loop note** (per CLAUDE.md Core Flow): after Final verification → `/review` runs (4 parallel ultrathink reviewers). If Must Fix or Should Improve items surface, Claude Code applies `/receiving-code-review` (verify, push back, YAGNI) and **appends fix tasks to this plan**, then re-enters `/execute-plan` autonomously. The loop continues until `/review` reports no remaining items. Items that require **design changes** (architecture / DD contracts / scope expansion) are **escalated to the engineer**. The `Post-/review iteration` section near the end of this plan documents the expected flow and per the PR-3a/3b precedent reserves budget for ~2–6 fix tasks.

---

## Task 1: Add `Ty::Param(usize)` + extend `lower_type` for generic enum annotations

**Why:** `Ty::Param(usize)` is the type-parameter sentinel that Tasks 3–5 use to represent variant signatures (HM-style schema). `lower_type` currently rejects user-defined generic enum annotations (`let r: Result<Int, String>`) with a "PR-3c will land" diagnostic. PR-3c needs to actually lower these to `Ty::Enum(def_id, [Ty::Int, Ty::String])`. Variant signatures (with `Ty::Param`) are Task 3's work — Task 1 only handles use-site annotations.

**Behavior change:** yes (new diag for arity mismatch on generic types; `Result<Int, String>` etc. now produce a proper `Ty` instead of `Ty::Error`).
**Discipline:** TDD.

**Files:**
- Modify: `compiler/src/sema/ty.rs` — add `Ty::Param(usize)` variant; extend `lower_type` Generic-branch to handle user-defined generic enums; add helper `expected_type_param_count`.
- Modify: `compiler/src/sema/diag.rs` — add `wrong_type_arity(span, name, expected, actual)` helper.
- Modify: `compiler/src/sema/check.rs` — add `Ty::Param(_) => Ty::Error` defensive arms in synth/check matches (Param shouldn't appear at use-site after Tasks 4–5, but PR-3c is incremental).
- Modify: `compiler/src/sema.rs` — add `Ty::Param(_) =>` arm in any TyKind exhaustive match (none currently, but if signature_pass / type_eq exists, update accordingly).
- Test: `compiler/src/sema/ty.rs::tests` — 6 new tests (positive + negative).

**`Ty::Param` contract** (added to `compiler/src/sema/ty.rs`):

```rust
pub enum Ty {
    Int,
    Scalar(Dimension),
    Bool,
    String,
    Vec(usize, Dimension),
    Mat(usize, usize),
    Array(Box<Ty>),
    Dict(Box<Ty>, Box<Ty>),
    Function(Vec<Ty>, Box<Ty>),
    Struct(DefId),
    Enum(DefId, Vec<Ty>),
    Var(TypeVarId),
    /// Type-parameter sentinel. Indexed by the position in the parent
    /// definition's `type_params` list. Stored only in `def_types` /
    /// `variant_payloads` schemas; `synth_ident` substitutes with fresh
    /// `Var` at each use site (Task 4). Should never appear in expression
    /// types written to `TypedProgram.types` after PR-3c lands.
    Param(usize),
    Error,
}
```

**`lower_type` Generic branch update** (`compiler/src/sema/ty.rs`):

```rust
TypeKind::Generic(name, args) => match name.as_str() {
    "Scalar" => lower_scalar(args, ast_ty.span, diags),
    "Vec"    => lower_vec(args, ast_ty.span, resolutions, definitions, diags),
    "Mat"    => lower_mat(args, ast_ty.span, diags),
    "Array"  => lower_array(args, ast_ty.span, resolutions, definitions, diags),
    "Dict"   => lower_dict(args, ast_ty.span, resolutions, definitions, diags),
    _ => lower_user_generic(name, args, ast_ty, resolutions, definitions, diags),
},

/// Lower a user-defined generic enum instantiation, e.g. `Result<Int, String>`.
fn lower_user_generic(
    name: &str,
    args: &[TypeArg],
    ast_ty: &Type,
    resolutions: &ResolveTable,
    definitions: &DefinitionTable,
    diags: &mut Vec<Diagnostic>,
) -> Ty {
    let Some(def_id) = resolutions.get(&ast_ty.id).copied() else {
        return Ty::Error; // resolver already reported
    };
    let Some(info) = definitions.get(&def_id) else {
        return Ty::Error;
    };
    if !matches!(info.kind, DefKind::Enum) {
        diags.push(Diagnostic::type_error(
            ast_ty.span,
            format!("`{name}` is not a generic type"),
        ));
        return Ty::Error;
    }
    let expected = info.type_params.len();
    let actual = args.len();
    if expected != actual {
        diags.push(crate::sema::diag::wrong_type_arity(
            ast_ty.span, name, expected, actual,
        ));
        return Ty::Error;
    }
    let mut lowered_args = Vec::with_capacity(args.len());
    for arg in args {
        let ty = match arg {
            TypeArg::Type(t) => lower_type(t, resolutions, definitions, diags),
            TypeArg::Int(_) | TypeArg::Unit(_) => {
                diags.push(Diagnostic::type_error(
                    ast_ty.span,
                    format!("`{name}` type arguments must be types, not int/unit literals"),
                ));
                Ty::Error
            }
        };
        lowered_args.push(ty);
    }
    Ty::Enum(def_id, lowered_args)
}
```

**New `wrong_type_arity` helper** (`compiler/src/sema/diag.rs`):

```rust
pub fn wrong_type_arity(span: Span, name: &str, expected: usize, actual: usize) -> Diagnostic {
    Diagnostic::type_error(
        span,
        format!("`{name}` expects {expected} type argument(s), but {actual} were provided"),
    )
}
```

### Steps

- [ ] **Step 1: Write failing tests for `Ty::Param` + generic lowering (red phase)**

Add to `compiler/src/sema/ty.rs::tests`:

```rust
#[test]
fn ty_param_variant_compiles() {
    // Constructibility check; ensures Ty enum exposes Param.
    let _t = Ty::Param(0);
    let _u = Ty::Function(vec![Ty::Param(0)], Box::new(Ty::Enum(DefId(0), vec![Ty::Param(0)])));
}

#[test]
fn lower_user_generic_enum_concrete_args() {
    let src = "enum Result<T, E>\n  Ok(T)\n  Err(E)\nend\nlet r: Result<Int, String> = 0";
    let prog = parse(tokenize(src).unwrap()).unwrap();
    let (resolutions, defs, _) = resolve_program(&prog);
    let mut diags = Vec::new();
    let item = &prog.items[1];
    let ty = match item {
        crate::ast::Item::Let(l) => lower_type(&l.ty, &resolutions, &defs, &mut diags),
        _ => panic!(),
    };
    assert!(matches!(ty, Ty::Enum(_, ref args) if args.len() == 2 && args[0] == Ty::Int && args[1] == Ty::String));
    assert!(diags.is_empty(), "diags: {:?}", diags);
}

#[test]
fn lower_user_generic_arity_too_few() {
    let src = "enum Result<T, E>\n  Ok(T)\n  Err(E)\nend\nlet r: Result<Int> = 0";
    let prog = parse(tokenize(src).unwrap()).unwrap();
    let (resolutions, defs, _) = resolve_program(&prog);
    let mut diags = Vec::new();
    let item = &prog.items[1];
    let ty = match item {
        crate::ast::Item::Let(l) => lower_type(&l.ty, &resolutions, &defs, &mut diags),
        _ => panic!(),
    };
    assert_eq!(ty, Ty::Error);
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
    assert!(diags[0].message.contains("expects 2 type argument"));
}

#[test]
fn lower_user_generic_arity_too_many() {
    let src = "enum Maybe<T>\n  Just(T)\n  Nothing\nend\nlet m: Maybe<Int, String> = 0";
    let prog = parse(tokenize(src).unwrap()).unwrap();
    let (resolutions, defs, _) = resolve_program(&prog);
    let mut diags = Vec::new();
    let item = &prog.items[1];
    let ty = match item {
        crate::ast::Item::Let(l) => lower_type(&l.ty, &resolutions, &defs, &mut diags),
        _ => panic!(),
    };
    assert_eq!(ty, Ty::Error);
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
    assert!(diags[0].message.contains("expects 1 type argument"));
}

#[test]
fn lower_user_generic_nested() {
    let src = "enum Result<T, E>\n  Ok(T)\n  Err(E)\nend\nenum Maybe<T>\n  Just(T)\n  Nothing\nend\nlet x: Result<Maybe<Int>, String> = 0";
    let prog = parse(tokenize(src).unwrap()).unwrap();
    let (resolutions, defs, _) = resolve_program(&prog);
    let mut diags = Vec::new();
    let item = &prog.items[2];
    let ty = match item {
        crate::ast::Item::Let(l) => lower_type(&l.ty, &resolutions, &defs, &mut diags),
        _ => panic!(),
    };
    assert!(diags.is_empty(), "diags: {:?}", diags);
    if let Ty::Enum(_, args) = ty {
        assert!(matches!(&args[0], Ty::Enum(_, inner) if inner.len() == 1 && inner[0] == Ty::Int));
        assert_eq!(args[1], Ty::String);
    } else {
        panic!("expected outer Enum");
    }
}

#[test]
fn lower_non_enum_used_with_args_diag() {
    let src = "struct Point\n  x: Scalar\n  y: Scalar\nend\nlet p: Point<Int> = 0";
    let prog = parse(tokenize(src).unwrap()).unwrap();
    let (resolutions, defs, _) = resolve_program(&prog);
    let mut diags = Vec::new();
    let item = &prog.items[1];
    let _ = match item {
        crate::ast::Item::Let(l) => lower_type(&l.ty, &resolutions, &defs, &mut diags),
        _ => panic!(),
    };
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
    assert!(diags[0].message.contains("not a generic type"));
}
```

- [ ] **Step 2: Verify red phase**

Run: `cd compiler && cargo test ty_param_variant_compiles lower_user_generic 2>&1 | head -30`
Expected: ALL 6 tests fail (5 with diag-presence assertion failure since current lower_type returns Ty::Error + "PR-3c" diag instead of the new shapes; 1 with missing variant `Ty::Param`).

- [ ] **Step 3: Add `Ty::Param(usize)` variant**

Add to `Ty` enum in `compiler/src/sema/ty.rs` (between `Var` and `Error`):

```rust
/// Type-parameter sentinel. Indexed by position in the parent definition's
/// `type_params` list. Stored only in `def_types` / `variant_payloads`
/// schemas; substituted with fresh `Var` at each use site by `synth_ident`.
/// Should not appear in expression types written to `TypedProgram.types`
/// after PR-3c lands.
Param(usize),
```

- [ ] **Step 4: Add defensive `Ty::Param(_)` arms**

Search `cargo build 2>&1 | grep "non-exhaustive"` for compiler-detected missing arms. Add:
- `compiler/src/sema/check.rs`: in any `match` over `Ty` that's currently exhaustive, add `Ty::Param(_) => Ty::Error` (synth_arith Vec/Mat dispatch, synth_unaryop Neg, synth_index, synth_field_access fall-throughs). Comment: "PR-3c: Param appears in def_types schemas; substituted at use site. Defensive Error here."
- `compiler/src/sema/diag.rs::format_ty`: add `Ty::Param(i) => format!("<param #{i}>")`.
- `compiler/src/sema/unify.rs::resolve`: add `Ty::Param(_) => ty.clone()` (Param is a constant; pass through).
- `compiler/src/sema/unify.rs::unify`: add `(Ty::Param(_), _) | (_, Ty::Param(_)) => Err((a, b))` — Param shouldn't unify with anything at runtime; it's only a schema sentinel.

- [ ] **Step 5: Add `wrong_type_arity` helper**

Add to `compiler/src/sema/diag.rs`:

```rust
pub fn wrong_type_arity(span: Span, name: &str, expected: usize, actual: usize) -> Diagnostic {
    Diagnostic::type_error(
        span,
        format!("`{name}` expects {expected} type argument(s), but {actual} were provided"),
    )
}
```

- [ ] **Step 6: Replace `lower_type` Generic branch user-defined arm**

In `compiler/src/sema/ty.rs`'s `lower_type`, replace the existing "PR-3c will land" diag arm:

```rust
// OLD:
_ => {
    diags.push(Diagnostic::type_error(
        ast_ty.span,
        format!("generic enum instantiation not yet supported (PR-3c will land `{name}<...>`)"),
    ));
    Ty::Error
}

// NEW:
_ => lower_user_generic(name, args, ast_ty, resolutions, definitions, diags),
```

Add the `lower_user_generic` function (full code in the contract above).

- [ ] **Step 7: Verify (green)**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 319 baseline + 6 new = **325 tests pass**. Clippy clean. Fmt clean.

Note: the existing test `lower_generic_enum_user_defined_is_deferred_to_3c` will now fail because it asserted "PR-3c" in the diag. Delete or repurpose that test as part of this Step (since PR-3c is *here*, the deferral is gone). Replace its assertion with something positive (e.g., that lowering succeeds for a defined generic).

- [ ] **Step 8: Refactor**

Refactor opportunities:
- `lower_user_generic` shares structure with the Vec/Mat/Array/Dict helpers (all take `args, span, ...` and dispatch). If a `lower_with_arity` helper falls out cleanly, extract it. Skip if YAGNI.
- The defensive `Ty::Param(_)` arms added in Step 4 should each have a comment explaining why Param shouldn't reach that point.
- The `lower_generic_enum_user_defined_is_deferred_to_3c` test is now meaningless. Replace its assertions or delete it.

Re-run verification after refactor.

- [ ] **Step 9: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Add Ty::Param sentinel + lower_type for user generic types

Introduces Ty::Param(usize) as the type-parameter sentinel. Variant
signatures (Task 3) and stdlib generic functions (PR-3e) will populate
schemas with Param(i); use sites (Task 4) substitute Param → fresh Var.

Extends lower_type's Generic branch to dispatch user-defined generic
enums into a new lower_user_generic helper. The "PR-3c will land"
diagnostic is replaced with proper lowering: Result<Int, String> →
Ty::Enum(result_def, [Int, String]). Arity mismatches now produce a
focused wrong_type_arity diag instead of silent Ty::Error.

Adds 6 regression tests (positive concrete args, arity-too-few,
arity-too-many, nested generic, non-enum-with-args). Replaces the
PR-3b "deferred-to-3c" test with a positive lowering case.

Defensive Ty::Param(_) arms added to synth_arith, synth_unaryop, format_ty,
unify::resolve, unify::unify so any accidental Param leakage at use sites
short-circuits cleanly to Error.
EOF
)"
```

---

## Task 2: Add `binding_def_ids` table + collapse 5 linear-scan helpers

**Why:** PR-3b accumulated 5 sites that linearly scan `definitions.iter().find(...)` to recover an intro-site DefId from a `(DefKind, name, span)` triple. The cross-task review and Task 8 G3 corollary made this O(n²) pattern visible. PR-3c's pattern code (Tasks 4–5) will add more bindings, multiplying the cost. A single intro→def table populated by `define_or_report` collapses all 5 sites to O(1) lookups and simplifies the dup-skip logic.

**Behavior change:** no (pure refactor — existing 325 tests are the green-bar safety net).
**Discipline:** refactor.

**Files:**
- Modify: `compiler/src/sema/resolve.rs` — extend `Resolver` with `binding_def_ids: BindingTable`, change `define_or_report` signature to take `intro_node_id: NodeId`, update all 9 call sites.
- Modify: `compiler/src/sema.rs` — add `pub binding_def_ids: BindingTable` to `TypedProgram`; thread through `resolve_program` return tuple.
- Modify: `compiler/src/sema/check.rs` — replace `local_let_def_id`, `loop_var_def_id`, `pattern_binding_def_id` with `binding_def_ids.get(&node.id).copied()`; replace `check_function`'s `(kind, name, span)` lookup with `binding_def_ids.get(&f.id).copied()`.
- Modify: `compiler/src/sema.rs::signature_pass` — replace param-DefId recovery with `binding_def_ids.get(&p.id).copied()`.

**`BindingTable` type definition** (added to `compiler/src/sema/resolve.rs`):

```rust
/// Maps a binding-introducing AST node's NodeId to the DefId allocated
/// for that binding. Orthogonal to `ResolveTable` (which maps use-site
/// NodeIds to definition DefIds). Populated by `define_or_report` for
/// Function, Struct, Enum, EnumVariant, Param, LocalLet, TopLevelLet,
/// LoopVar, and PatternBinding intro sites.
pub type BindingTable = HashMap<NodeId, DefId>;
```

**`define_or_report` signature change**:

```rust
// OLD:
fn define_or_report(&mut self, name: String, kind: DefKind, span: Span) -> Option<DefId>

// NEW:
fn define_or_report(
    &mut self,
    name: String,
    kind: DefKind,
    span: Span,
    intro_node_id: NodeId,
) -> Option<DefId> {
    // ... existing duplicate-name detection ...
    let def_id = self.fresh_def_id();
    self.definitions.insert(def_id, DefinitionInfo { kind, name, span, type_params: vec![] });
    self.binding_def_ids.insert(intro_node_id, def_id);   // NEW
    self.table.insert(/* ... */);
    Some(def_id)
}
```

**Call site migration table** — 9 sites need the `intro_node_id` arg:

| Site (file:line approx) | Source | Intro NodeId |
|------|--------|--------------|
| `resolve_function` Function intro | `f: &FunctionDef` | `f.id` |
| `resolve_function` per Param | `p: &Param` (loop) | `p.id` |
| `resolve_item Item::Struct` | `s: &StructDef` | `s.id` |
| `resolve_struct_fields` (no intro DefId — fields aren't named bindings; skip if no `define_or_report`) | — | — |
| `resolve_item Item::Enum` | `e: &EnumDef` | `e.id` |
| `resolve_item Item::Enum` per variant | `v: &EnumVariant` | `v.id` |
| `resolve_item Item::Let` | `l: &LetItem` | `l.id` |
| `resolve_stmt StmtKind::Let` | `s: &Stmt` (LocalLet) | `s.id` |
| `resolve_for_loop` Range/Iter | (loop var has no AST id today — see fallback below) | `outer_span`-derived **NEW** id |
| `resolve_pattern` PatternBinding | `p: &Pattern` | `p.id` |

**Loop-var caveat**: PR-3b's `ForStmt` AST may not carry per-binding NodeIds for the loop variable. Verify by reading `compiler/src/ast/stmt.rs`. If absent, defer this site to a follow-up (use the existing helper for now) and document with a TODO comment. The other 8 sites are sufficient to collapse 4 of the 5 helpers.

### Steps

- [ ] **Step 1: Add `BindingTable` type + Resolver field**

Modify `compiler/src/sema/resolve.rs`:
- Add `pub type BindingTable = HashMap<NodeId, DefId>;` near the `ResolveTable` type alias.
- Add `pub(crate) binding_def_ids: BindingTable` field to `Resolver` struct, initialized to `BindingTable::new()` in `Resolver::new()`.

- [ ] **Step 2: Extend `define_or_report` signature**

Change signature to take `intro_node_id: NodeId` parameter. Body inserts into `self.binding_def_ids` after `self.definitions.insert`.

- [ ] **Step 3: Update 8 call sites of `define_or_report`**

For each site listed in the migration table, add the `intro_node_id` argument. Verify by `grep -n "define_or_report" compiler/src/sema/resolve.rs`.

- [ ] **Step 4: Update `resolve_program` return tuple**

Change return from `(ResolveTable, DefinitionTable, Vec<Diagnostic>)` to `(ResolveTable, DefinitionTable, BindingTable, Vec<Diagnostic>)`. Threading update affects:
- `compiler/src/sema.rs::check()` — destructure 4-tuple, store binding_def_ids in TypedProgram.
- `compiler/src/sema.rs::signature_pass` — accept `&BindingTable`, replace param-DefId scan.
- `compiler/src/sema.rs::tests` (if any test calls `resolve_program` directly).
- `compiler/src/sema/ty.rs::tests` (~5 tests call `resolve_program`).
- `compiler/src/sema/resolve.rs::tests` (~20 tests).

Use a small mechanical search: `grep -rn "resolve_program" compiler/src/` and update each.

- [ ] **Step 5: Add `pub binding_def_ids: BindingTable` to `TypedProgram`**

Modify `compiler/src/sema.rs`:

```rust
pub struct TypedProgram {
    pub program: Program,
    pub types: TypeTable,
    pub resolutions: ResolveTable,
    pub definitions: DefinitionTable,
    pub binding_def_ids: BindingTable,    // NEW
    pub def_types: DefTypeMap,
    pub struct_fields: StructFieldMap,
    pub variant_payloads: VariantPayloadMap,
}
```

`TypedProgram::new` constructor adds the new field (becomes 8-arg; will be inlined in Task 8).

- [ ] **Step 6: Replace `signature_pass` param-DefId scan**

In `signature_pass`'s `Item::Function` arm, replace:

```rust
// OLD: O(params × definitions) scan
for (p, ty) in f.params.iter().zip(&param_tys) {
    if let Some(p_def_id) = definitions.iter()
        .find(|(_, info)| matches!(info.kind, DefKind::Param)
            && info.name == p.name && info.span == p.span)
        .map(|(id, _)| *id)
    {
        def_types.insert(p_def_id, ty.clone());
    }
}

// NEW: O(1) per param
for (p, ty) in f.params.iter().zip(&param_tys) {
    if let Some(p_def_id) = binding_def_ids.get(&p.id).copied() {
        def_types.insert(p_def_id, ty.clone());
    }
}
```

The function-arm initial DefId lookup can also use `binding_def_ids.get(&f.id)` instead of the existing `name_to_def` reverse index — but only if the existing pattern is also using the binding table; otherwise keep the `name_to_def` map for now (Task 3 may restructure).

- [ ] **Step 7: Replace `check_function` (kind, name, span) lookup**

In `compiler/src/sema/check.rs::check_function`, replace:

```rust
// OLD:
let info = self.definitions.iter()
    .find(|(_, info)| matches!(info.kind, DefKind::Function)
        && info.name == f.name && info.span == f.span)
    .map(|(id, _)| *id);

// NEW:
let info = self.binding_def_ids.get(&f.id).copied();
```

For duplicate functions, `binding_def_ids` only stored the first one's DefId (because `define_or_report` returns `None` on duplicate, no insert happens for the duplicate). So the lookup correctly returns `None` for the second function's `f.id`, naturally skipping the cascade. The Task 8 G3 corollary's `(kind, name, span)` matching becomes redundant — delete the corollary.

- [ ] **Step 8: Replace `local_let_def_id`, `loop_var_def_id`, `pattern_binding_def_id` helpers**

Replace each helper's body with:

```rust
fn local_let_def_id(&self, node_id: NodeId) -> Option<DefId> {
    self.binding_def_ids.get(&node_id).copied()
}
// Similarly for loop_var_def_id, pattern_binding_def_id
```

Then inline at call sites if the helper becomes a 1-line wrapper (judgment call — keep helpers if the call sites read more clearly with named methods). After all sites use binding_def_ids, the linear-scan helpers can be deleted entirely. Decide based on call-site count.

If the for-loop variable doesn't have a NodeId in the AST, the `loop_var_def_id` helper falls back to the existing scan with a TODO comment (handle in a future PR).

- [ ] **Step 9: Verify**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 325 tests still pass (no behavior change). Clippy clean. Fmt clean.

If any test fails, the failure indicates either:
- A missed call site of `define_or_report` (search again with grep).
- A test asserting on the structure of the linear-scan helper output (rewrite the test in terms of `binding_def_ids`).

- [ ] **Step 10: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Add binding_def_ids table; collapse 5 linear-scan helpers to O(1)

Resolver now records intro NodeId → DefId in a new BindingTable side-
table during define_or_report. The 5 sites that previously scanned
DefinitionTable linearly to recover a DefId by (kind, name, span) now
use binding_def_ids.get(&id).copied(): signature_pass param recovery,
check_function function recovery, local_let_def_id, loop_var_def_id,
pattern_binding_def_id.

resolutions remains use→def only (orthogonal to binding_def_ids which
is intro→def). The asymmetry is now explicit in TypedProgram's field
docs.

Task 8 G3 corollary's (kind, name, span) matching is removed —
binding_def_ids naturally skips duplicates because define_or_report
doesn't insert for them.

No behavior change. 325 existing tests unchanged.
EOF
)"
```

---

## Task 3: `signature_pass` produces variant schemas with `Ty::Param`; outer-enum gate

**Why:** Task 1 added `Ty::Param`; Task 2 made DefId lookups O(1). Task 3 populates `def_types[variant_def_id]` with HM-style schemas. Each variant's payload `T` (a type-parameter reference) lowers to `Ty::Param(i)` where `i` is its position in the parent enum's `type_params`. Variant signatures look like `Ty::Function([Ty::Param(0), Ty::Param(1)], Ty::Enum(parent_def, [Ty::Param(0), Ty::Param(1)]))`. Non-generic enums are the special case where `type_params` is empty and no `Ty::Param` appears.

This task also adds the **outer-enum first-writer-wins gate** that PR-3b deferred (per-variant gate already in place; outer enum was missing).

**Behavior change:** yes (variant constructor calls now type-check; PR-3b's silent variant gap closes; outer-enum duplicates now safely handled).
**Discipline:** TDD.

**Files:**
- Modify: `compiler/src/sema.rs::signature_pass` — `Item::Enum` arm builds variant signatures with `Ty::Param`; outer-enum gate `if struct_fields_or_enum_seen.contains(&def_id) { continue; }` added (using a new local set, since existing `variant_payloads.contains_key` is per-variant, not per-enum).
- Modify: `compiler/src/sema/resolve.rs` — extend `resolve_function` and similar to walk into generic-enum variant payloads when type-parameter scope is set up. Currently PR-3b skipped `e.type_params.is_empty()` cases; PR-3c needs to **enable** the walk inside a type-parameter scope.

**Type-parameter scope handling**: Resolver needs to know `T`, `E` etc. inside `enum Result<T, E>`'s variant payloads. Two implementation options:

- **(scope option a)**: Treat type parameters as **regular bindings** in a temporary scope. `resolve_item Item::Enum` opens a scope, defines each `type_params[i]` as a **synthetic binding** (DefKind::TypeParam? or reuse PatternBinding?), walks variant payloads (which now resolve `T` to that binding), closes the scope.
- **(scope option b)**: Lower variant payloads **without** putting type-params in scope; instead, signature_pass post-processes the AST and substitutes `Type::Named("T")` with `Ty::Param(0)` directly using the parent enum's `type_params` list as a substitution table.

**(scope option b) is simpler and matches the schema model** — type parameters aren't real bindings (no DefId allocated), they're positions in a list. Adopt (b).

`signature_pass` Item::Enum arm pseudocode:

```rust
Item::Enum(e) => {
    let Some(enum_def_id) = binding_def_ids.get(&e.id).copied() else { continue; };
    if struct_fields.contains_key(&enum_def_id) || enums_lowered.contains(&enum_def_id) {
        // outer-enum first-writer-wins: skip duplicate
        continue;
    }
    enums_lowered.insert(enum_def_id);   // mark before processing variants

    // Build a substitution map: type_param name → Ty::Param(i)
    let type_param_subst: HashMap<&str, usize> = e.type_params.iter()
        .enumerate()
        .map(|(i, name)| (name.as_str(), i))
        .collect();

    for variant in &e.variants {
        let Some(variant_def_id) = binding_def_ids.get(&variant.id).copied() else { continue; };
        if variant_payloads.contains_key(&variant_def_id) { continue; }   // existing per-variant gate

        let payload_tys: Vec<Ty> = variant.payload.iter()
            .map(|t| lower_type_with_subst(t, resolutions, definitions, &type_param_subst, diags))
            .collect();
        let return_ty = if e.type_params.is_empty() {
            Ty::Enum(enum_def_id, vec![])
        } else {
            Ty::Enum(enum_def_id, (0..e.type_params.len()).map(Ty::Param).collect())
        };
        variant_payloads.insert(variant_def_id, VariantPayload {
            parent_enum: enum_def_id,
            payload: payload_tys.clone(),
        });
        def_types.insert(variant_def_id, Ty::Function(payload_tys, Box::new(return_ty)));
    }
}
```

`lower_type_with_subst` is a new helper:

```rust
/// Like `lower_type` but with a substitution map for type-param references.
/// When a TypeKind::Named matches a key in `subst`, returns Ty::Param(value).
fn lower_type_with_subst(
    ast_ty: &Type,
    resolutions: &ResolveTable,
    definitions: &DefinitionTable,
    subst: &HashMap<&str, usize>,
    diags: &mut Vec<Diagnostic>,
) -> Ty {
    if let TypeKind::Named(name) = &ast_ty.kind {
        if let Some(&i) = subst.get(name.as_str()) {
            return Ty::Param(i);
        }
    }
    // Fall through to regular lower_type (which will recurse into Generic args, etc.)
    // But Generic args also need substitution → recursive-with-subst
    match &ast_ty.kind {
        TypeKind::Named(name) => {
            // builtin or user-defined
            lower_type(ast_ty, resolutions, definitions, diags)
        }
        TypeKind::Generic(name, args) => {
            // Recurse into args with substitution
            // ... mirror lower_user_generic but with subst threading ...
            let mut new_args = vec![];
            for arg in args {
                if let TypeArg::Type(t) = arg {
                    new_args.push(lower_type_with_subst(t, resolutions, definitions, subst, diags));
                } else {
                    // Int/Unit args: pass through to lower_type for handling
                }
            }
            // Build the result based on builtin or user-defined dispatch
            // ...
        }
        TypeKind::Function(args, ret) => {
            let arg_tys: Vec<_> = args.iter()
                .map(|a| lower_type_with_subst(a, resolutions, definitions, subst, diags))
                .collect();
            let ret_ty = lower_type_with_subst(ret, resolutions, definitions, subst, diags);
            Ty::Function(arg_tys, Box::new(ret_ty))
        }
    }
}
```

(Implementation detail: keep the recursion correct and DRY with existing `lower_type`. If the helper duplicates too much of `lower_type`, refactor `lower_type` to optionally accept a subst; or build `lower_type_with_subst` as a thin wrapper that pre-walks for Param substitutions and falls through.)

### Steps

- [ ] **Step 1: Write failing tests for variant signature schemas (red phase)**

Add to `compiler/src/sema.rs::tests`:

```rust
#[test]
fn signature_pass_populates_generic_variant_with_param() {
    let src = "enum Maybe<T>\n  Just(T)\n  Nothing\nend";
    let prog = parse(tokenize(src).unwrap()).unwrap();
    let typed = check(prog).unwrap();

    let just_def = typed.definitions.iter()
        .find(|(_, info)| info.name == "Just")
        .map(|(id, _)| *id).unwrap();

    let sig = typed.def_types.get(&just_def).cloned().unwrap();
    if let Ty::Function(params, ret) = sig {
        assert_eq!(params, vec![Ty::Param(0)]);
        if let Ty::Enum(_, args) = *ret {
            assert_eq!(args, vec![Ty::Param(0)]);
        } else {
            panic!("expected Ty::Enum return");
        }
    } else {
        panic!("expected Ty::Function");
    }
}

#[test]
fn signature_pass_populates_two_param_enum() {
    let src = "enum Result<T, E>\n  Ok(T)\n  Err(E)\nend";
    let prog = parse(tokenize(src).unwrap()).unwrap();
    let typed = check(prog).unwrap();

    let ok_def = typed.definitions.iter()
        .find(|(_, info)| info.name == "Ok")
        .map(|(id, _)| *id).unwrap();
    let sig = typed.def_types.get(&ok_def).cloned().unwrap();
    if let Ty::Function(params, ret) = sig {
        assert_eq!(params, vec![Ty::Param(0)]);
        if let Ty::Enum(_, args) = *ret {
            assert_eq!(args, vec![Ty::Param(0), Ty::Param(1)]);
        } else { panic!(); }
    } else { panic!(); }

    let err_def = typed.definitions.iter()
        .find(|(_, info)| info.name == "Err")
        .map(|(id, _)| *id).unwrap();
    let sig = typed.def_types.get(&err_def).cloned().unwrap();
    if let Ty::Function(params, _) = sig {
        assert_eq!(params, vec![Ty::Param(1)]);
    }
}

#[test]
fn signature_pass_populates_non_generic_variant_signature() {
    // Non-generic: type_params empty; payload uses concrete types, no Ty::Param.
    // Closes PR-3b's silent variant gap.
    let src = "enum Maybe\n  Just(Int)\n  Nothing\nend";
    let prog = parse(tokenize(src).unwrap()).unwrap();
    let typed = check(prog).unwrap();

    let just_def = typed.definitions.iter()
        .find(|(_, info)| info.name == "Just")
        .map(|(id, _)| *id).unwrap();
    let sig = typed.def_types.get(&just_def).cloned().unwrap();
    if let Ty::Function(params, ret) = sig {
        assert_eq!(params, vec![Ty::Int]);
        assert!(matches!(*ret, Ty::Enum(_, args) if args.is_empty()));
    } else { panic!(); }
}

#[test]
fn signature_pass_outer_enum_first_writer_wins() {
    let src = "enum E\n  A\nend\nenum E\n  B\nend";
    let prog = parse(tokenize(src).unwrap()).unwrap();
    let result = check(prog);
    let diags = result.unwrap_err();
    // Resolver fires duplicate-name once; signature_pass must NOT add cascading diags
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
    assert!(diags[0].message.contains("already defined") || diags[0].message.contains("duplicate"));
}

#[test]
fn signature_pass_nested_generic_payload() {
    let src = "enum Result<T, E>\n  Ok(T)\n  Err(E)\nend\nenum WrappedResult<T>\n  Wrap(Result<T, String>)\nend";
    let prog = parse(tokenize(src).unwrap()).unwrap();
    let typed = check(prog).unwrap();

    let wrap_def = typed.definitions.iter()
        .find(|(_, info)| info.name == "Wrap")
        .map(|(id, _)| *id).unwrap();
    let sig = typed.def_types.get(&wrap_def).cloned().unwrap();
    // Wrap's payload should be Result<Param(0), String>
    if let Ty::Function(params, _) = sig {
        if let [Ty::Enum(_, inner_args)] = params.as_slice() {
            assert_eq!(inner_args[0], Ty::Param(0));
            assert_eq!(inner_args[1], Ty::String);
        } else {
            panic!("expected nested Ty::Enum payload");
        }
    } else { panic!(); }
}

#[test]
fn signature_pass_preserves_struct_and_let_handling() {
    // Negative regression: ensure adding generic enum logic didn't break struct/let.
    let src = "struct P\n  x: Int\nend\nlet pi: Scalar = 3.14";
    let prog = parse(tokenize(src).unwrap()).unwrap();
    let typed = check(prog).unwrap();
    assert!(!typed.struct_fields.is_empty());
    assert!(!typed.def_types.is_empty());
}
```

- [ ] **Step 2: Verify red phase**

Run: `cd compiler && cargo test signature_pass_populates_generic_variant 2>&1 | head`
Expected: tests fail (current signature_pass doesn't emit Ty::Param; variant signatures aren't in def_types either for generic OR non-generic).

- [ ] **Step 3: Add `lower_type_with_subst` helper**

Implement in `compiler/src/sema/ty.rs` (or a new `sema/lower.rs` if structure becomes cluttered). Recursion mirrors `lower_type` but pre-checks for Param substitutions on TypeKind::Named.

- [ ] **Step 4: Restructure `signature_pass` Item::Enum arm**

Per the pseudocode in the contract:
- Add `let mut enums_lowered: HashSet<DefId> = HashSet::new();` at the top of the function
- For each Item::Enum:
  1. Look up enum DefId via `binding_def_ids.get(&e.id)`
  2. Skip if `enums_lowered.contains(&enum_def_id)` (outer first-writer-wins)
  3. Build `type_param_subst` map
  4. For each variant, look up variant DefId via `binding_def_ids.get(&v.id)`
  5. Lower each payload via `lower_type_with_subst`
  6. Build return type as `Ty::Enum(enum_def_id, (0..N).map(Ty::Param).collect())` (or empty Vec for non-generic)
  7. Insert into `variant_payloads` and `def_types`

- [ ] **Step 5: Update Resolver to walk generic-enum variant payloads**

Currently `resolve_item Item::Enum` skips variant payload walks for `e.type_params.is_empty() == false` (PR-3b deferral). PR-3c needs the walk for `lower_type_with_subst` to find resolutions for non-type-param references (e.g. `Result<T, E>` in `Wrap(Result<T, String>)` — `Result` and `String` need name resolution; only `T` is a type-param).

Two-phase approach:
- The walk runs for ALL enums (generic and non-generic).
- The walk **does not** call `resolve_name_use` for names that match `e.type_params` (those are type-params, not external refs).
- This requires passing the type-param set through `resolve_type_annotation`.

Simplest approach: extend `resolve_type_annotation` to take an optional `&HashSet<&str>` of type-param names; if a `TypeKind::Named(name)` matches the set, skip the use resolution.

Update `resolve_item Item::Enum` to pass `&e.type_params.iter().collect::<HashSet<_>>()` into the variant payload walk.

Tests: `signature_pass_nested_generic_payload` (Step 1) verifies `Result` and `String` are resolved while `T` is not; `lower_type_with_subst` then maps `T → Ty::Param(0)`.

- [ ] **Step 6: Verify (green)**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 325 + 6 new = **331 tests pass**. The 3 baseline tests for non-generic enum (e.g. `signature_pass_populates_non_generic_variant_signature` — was previously skipping variant signatures) now pass with proper schemas. Clippy clean. Fmt clean.

If any prior test fails (e.g. PR-3b's `stage2_enum_with_generic_compiles`), the failure mode is "now produces signatures where it didn't before" — adapt the test if it asserted the absence.

- [ ] **Step 7: Refactor**

Refactor opportunities:
- If `lower_type_with_subst` duplicates >50% of `lower_type`, factor out a shared core that takes `Option<&HashMap>` or threads a `Subst` parameter.
- The `enums_lowered` HashSet is local to signature_pass; ensure it doesn't leak out.
- Consider a comment/test pinning the invariant "Ty::Param(i) only appears in def_types/variant_payloads schemas".

- [ ] **Step 8: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
signature_pass: variant schemas with Ty::Param; outer-enum gate

Variant signatures now stored in def_types as HM-style schemas:
- Generic: Some -> Ty::Function([Param(0)], Ty::Enum(opt, [Param(0)]))
- Non-generic: Just -> Ty::Function([Int], Ty::Enum(maybe, []))

Adds lower_type_with_subst helper that pre-checks TypeKind::Named
against the parent enum's type_params and emits Ty::Param(i) directly.
Resolver walk now handles generic enums (gated only on type-param
refs, not the entire enum).

Outer-enum first-writer-wins gate added (HashSet<DefId> tracking
processed enums in signature_pass). Closes PR-3b's per-variant-only
gap noted by the cross-task review.

Closes PR-3b's silent variant-constructor gap at the storage layer:
def_types now has signatures for ALL variants including non-generic.
synth_call (Task 4) will activate the actual call type-checking that
makes this surface.

6 regression tests pinning generic schemas, non-generic schemas,
outer-enum gate, and nested generic payloads.
EOF
)"
```

---

## Task 4: `synth_ident` + `synth_call` generic instantiation

**Why:** Task 3 stored variant signatures in `def_types` with `Ty::Param`. Task 4 wires the use-site retrieval: when `synth_ident` sees a variant DefId, it allocates fresh `Ty::Var` per type-param, substitutes Param(i) → Var(α_i), and returns the instantiated `Ty::Function` (or `Ty::Enum` for nullary variants). `synth_call` then proceeds normally (the per-arg `check_expr` activates Var-Int unification through the existing `unify_or_diag` plumbing).

This task **closes PR-3b's silent variant-constructor gap**: even non-generic `Just(1)` is now type-checked because the path is unified.

**Behavior change:** yes (variant constructor calls now type-check; arity errors, arg-type errors emerge; PR-3b silent gap closes).
**Discipline:** TDD.

**Files:**
- Modify: `compiler/src/sema/check.rs::synth_ident` — when DefKind is EnumVariant, retrieve the schema and instantiate fresh Vars; otherwise existing behavior. Remove the "PR-3c will populate" silent comment.
- Modify: `compiler/src/sema/check.rs` — add `instantiate_schema(ty, &mut Table) -> Ty` helper that walks a `Ty` and substitutes `Ty::Param(i)` → `Ty::Var(fresh)` (using a per-call substitution Vec).

**`instantiate_schema` helper**:

```rust
impl<'a> TypeChecker<'a> {
    /// Substitute every Ty::Param(i) in `ty` with a fresh Var.
    /// Param(i) at multiple positions resolves to the SAME fresh Var.
    /// Returns the instantiated type.
    fn instantiate_schema(&mut self, ty: &Ty) -> Ty {
        // First pass: count distinct Param indices, allocate fresh Vars
        let mut max_param = None;
        Self::scan_param_max(ty, &mut max_param);
        let Some(max) = max_param else { return ty.clone(); };  // no Params → identity
        let mut subst: Vec<TypeVarId> = (0..=max).map(|_| self.unify_table.fresh()).collect();
        // Second pass: substitute
        Self::subst_params(ty, &subst)
    }

    fn scan_param_max(ty: &Ty, max: &mut Option<usize>) {
        match ty {
            Ty::Param(i) => { *max = Some(max.map_or(*i, |m| m.max(*i))); }
            Ty::Vec(_, _) | Ty::Mat(_, _) | Ty::Int | Ty::Bool | Ty::String
            | Ty::Scalar(_) | Ty::Struct(_) | Ty::Var(_) | Ty::Error => {}
            Ty::Array(t) => Self::scan_param_max(t, max),
            Ty::Dict(k, v) => { Self::scan_param_max(k, max); Self::scan_param_max(v, max); }
            Ty::Function(args, ret) => {
                for a in args { Self::scan_param_max(a, max); }
                Self::scan_param_max(ret, max);
            }
            Ty::Enum(_, args) => { for a in args { Self::scan_param_max(a, max); } }
        }
    }

    fn subst_params(ty: &Ty, subst: &[TypeVarId]) -> Ty {
        match ty {
            Ty::Param(i) => Ty::Var(subst[*i]),
            Ty::Int | Ty::Bool | Ty::String | Ty::Scalar(_) | Ty::Mat(_, _)
            | Ty::Vec(_, _) | Ty::Struct(_) | Ty::Var(_) | Ty::Error => ty.clone(),
            Ty::Array(t) => Ty::Array(Box::new(Self::subst_params(t, subst))),
            Ty::Dict(k, v) => Ty::Dict(
                Box::new(Self::subst_params(k, subst)),
                Box::new(Self::subst_params(v, subst)),
            ),
            Ty::Function(args, ret) => Ty::Function(
                args.iter().map(|a| Self::subst_params(a, subst)).collect(),
                Box::new(Self::subst_params(ret, subst)),
            ),
            Ty::Enum(def, args) => Ty::Enum(*def, args.iter().map(|a| Self::subst_params(a, subst)).collect()),
        }
    }
}
```

**`synth_ident` update**:

```rust
fn synth_ident(&mut self, e: &Expr) -> Ty {
    let Some(def_id) = self.resolutions.get(&e.id).copied() else {
        return Ty::Error;
    };
    if let Some(ty) = self.def_types.get(&def_id).cloned() {
        // Schema may contain Ty::Param — instantiate with fresh Vars.
        return self.instantiate_schema(&ty);
    }
    // Existing fallback (Struct/Enum names → not_a_value diag, etc.)
    if let Some(info) = self.definitions.get(&def_id) {
        match info.kind {
            DefKind::Struct | DefKind::Enum => {
                self.diagnostics.push(crate::sema::diag::not_a_value(
                    e.span, info.kind, info.name.as_str(),
                ));
            }
            _ => {}
        }
    }
    Ty::Error
}
```

### Steps

- [ ] **Step 1: Write failing tests for generic + non-generic instantiation (red phase)**

Add to `compiler/src/sema/check.rs::tests`:

```rust
#[test]
fn variant_call_non_generic_typechecks() {
    // Closes PR-3b's silent gap. Just(1) is now type-checked.
    let diags = diags_for(
        "enum Maybe\n  Just(Int)\n  Nothing\nend\n\
         function f(): Maybe\n  return Just(1)\nend"
    );
    assert!(diags.is_empty(), "expected clean compile, got: {:?}", diags);
}

#[test]
fn variant_call_non_generic_wrong_arg_diag() {
    let diags = diags_for(
        "enum Maybe\n  Just(Int)\n  Nothing\nend\n\
         function f(): Maybe\n  return Just(\"oops\")\nend"
    );
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
    assert!(diags[0].message.contains("Int") && diags[0].message.contains("String"));
}

#[test]
fn variant_call_non_generic_wrong_arity_diag() {
    let diags = diags_for(
        "enum Maybe\n  Just(Int)\n  Nothing\nend\n\
         function f(): Maybe\n  return Just(1, 2)\nend"
    );
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
    assert!(diags[0].message.contains("expected 1") && diags[0].message.contains("found 2"));
}

#[test]
fn variant_call_generic_inferred_int() {
    let diags = diags_for(
        "enum Maybe<T>\n  Just(T)\n  Nothing\nend\n\
         function f(): Maybe<Int>\n  return Just(1)\nend"
    );
    assert!(diags.is_empty(), "diags: {:?}", diags);
}

#[test]
fn variant_call_generic_inferred_string_independent() {
    // Two Just calls in one function → independent fresh Vars; one Int, one String.
    let diags = diags_for(
        "enum Maybe<T>\n  Just(T)\n  Nothing\nend\n\
         function ints(): Maybe<Int>\n  return Just(1)\nend\n\
         function strs(): Maybe<String>\n  return Just(\"hi\")\nend"
    );
    assert!(diags.is_empty(), "diags: {:?}", diags);
}

#[test]
fn variant_nullary_value_in_context() {
    // None in context: T inferred from expected type.
    let diags = diags_for(
        "enum Maybe<T>\n  Just(T)\n  Nothing\nend\n\
         function f(): Maybe<Int>\n  return Nothing\nend"
    );
    assert!(diags.is_empty(), "diags: {:?}", diags);
}

#[test]
fn variant_call_two_param_enum_inferred() {
    let diags = diags_for(
        "enum Result<T, E>\n  Ok(T)\n  Err(E)\nend\n\
         function f(): Result<Int, String>\n  return Ok(42)\nend\n\
         function g(): Result<Int, String>\n  return Err(\"boom\")\nend"
    );
    assert!(diags.is_empty(), "diags: {:?}", diags);
}

#[test]
fn variant_call_generic_arg_int_to_scalar_widening() {
    // Implicit Int → Scalar(ZERO) at variant arg boundary should fire.
    let diags = diags_for(
        "enum Box<T>\n  Mk(T)\nend\n\
         function f(): Box<Scalar>\n  return Mk(1)\nend"
    );
    assert!(diags.is_empty(), "diags: {:?}", diags);
}
```

- [ ] **Step 2: Verify red phase**

Run: `cd compiler && cargo test variant_call 2>&1 | head -30`
Expected: ALL 8 new tests fail (current synth_ident silently returns Ty::Error for EnumVariant DefIds).

- [ ] **Step 3: Implement `instantiate_schema` + helpers**

Add to `compiler/src/sema/check.rs::TypeChecker` impl block (per the contract). Use `&mut self` for fresh-Var allocation.

- [ ] **Step 4: Update `synth_ident` to instantiate schemas**

Replace the existing synth_ident body per the contract. The existing `Ty::Error` early-return for missing def_types entry now happens AFTER the schema lookup (so EnumVariant DefIds with schemas get the new path; only orphan refs fall through).

- [ ] **Step 5: Verify (green)**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 331 + 8 new = **339 tests pass**. Clippy clean. Fmt clean.

- [ ] **Step 6: Refactor**

Refactor opportunities:
- `scan_param_max` and `subst_params` are recursive walkers that traverse `Ty`. If a third walker is added later (PR-3e?), extract a `TyWalker` trait. For PR-3c, two walkers don't justify abstraction yet.
- `instantiate_schema` allocates a Vec equal to `max_param + 1`. If type_params can have gaps (e.g. only Param(2) used but no Param(0)/Param(1)), we still allocate 3 fresh Vars but only use 1. Acceptable wastage; documenting in a comment.
- Storage normalization: ensure `synth_call`'s return value resolves any leftover Vars before storing in `types`. The `check_expr` and `unify_or_diag` paths should already do this via `resolve()`, but verify by checking the `types` HashMap entries in a test (e.g. add `assert!(matches!(typed.types[&id], Ty::Enum(_, args) if args[0] == Ty::Int))` for the explicit instantiation case).

- [ ] **Step 7: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Generic enum instantiation in synth_ident

synth_ident now retrieves variant signatures from def_types and
instantiates Ty::Param positions with fresh Var allocations via
the new instantiate_schema helper. Each variant reference gets
independent Vars (Some(1) and Some("x") in the same function infer
to different Maybe<Int> and Maybe<String>).

Non-generic variants share the same retrieval path: type_params is
empty, no Param appears in the schema, instantiate_schema is identity.
This closes PR-3b's silent variant-constructor gap: Just(1) etc.
now type-check normally.

8 new tests covering: non-generic clean compile / wrong-arg / wrong-
arity, generic with annotation, two independent calls, nullary value
in context, two-param Result, Int → Scalar widening at variant arg
boundary.

The unify_or_diag plumbing established in PR-3b now actively does
work: Var(α) introduced by instantiate_schema unifies with concrete
arg types via unify_or_diag's resolve() chain.
EOF
)"
```

---

## Task 5: `check_pattern` + `check_match_arm` generic substitution

**Why:** Match patterns over generic enums need the type-args from the scrutinee to bind sub-pattern variables correctly. `match opt` where `opt: Option<Int>` has `case Some(x) then ...` — `x` should bind as `Int`, not `Param(0)`. Task 5 wires this by substituting the scrutinee's resolved type-args into the variant's payload schema, then recursing into sub-patterns with the substituted payload types.

**Behavior change:** yes (generic match arms type-check correctly; sub-pattern bindings receive instantiated types).
**Discipline:** TDD.

**Files:**
- Modify: `compiler/src/sema/check.rs::check_match_arm` — when pattern is Variant, look up variant_payloads, substitute type-args from scrutinee type, recurse into sub-patterns with substituted payload types.
- Modify: `compiler/src/sema/check.rs::check_pattern` — Variant arm: use the new substitution, bind Ident sub-patterns with substituted types via `binding_def_ids`.

**`check_match_arm` Variant logic** (pseudocode):

```rust
fn check_match_arm(&mut self, arm: &MatchArm, scrut_ty: &Ty) -> Ty {
    self.check_pattern(&arm.pattern, scrut_ty);  // pattern uses scrut_ty as expected
    self.synth_block(&arm.body)
}

fn check_pattern(&mut self, p: &Pattern, expected: &Ty) {
    match &p.kind {
        PatternKind::Wildcard => {}
        PatternKind::Ident(name) => {
            if let Some(def_id) = self.binding_def_ids.get(&p.id).copied() {
                self.def_types.insert(def_id, expected.clone());
            }
        }
        PatternKind::Variant(name, sub_patterns) => {
            let Some(variant_def_id) = self.resolutions.get(&p.id).copied() else { return; };
            let Some(variant_info) = self.variant_payloads.get(&variant_def_id).cloned() else { return; };

            // Resolve expected (might contain Vars), expect Ty::Enum
            let resolved_expected = self.unify_table.resolve(expected);
            let (parent, type_args) = match &resolved_expected {
                Ty::Enum(p, args) => (*p, args.clone()),
                Ty::Error => return,  // no-cascade
                _ => {
                    // pattern type doesn't match scrutinee type
                    self.diagnostics.push(crate::sema::diag::pattern_type_mismatch(
                        p.span, &resolved_expected, "enum",
                    ));
                    return;
                }
            };

            // Verify variant belongs to this enum
            if variant_info.parent_enum != parent {
                self.diagnostics.push(crate::sema::diag::wrong_variant_for_enum(
                    p.span, name, &resolved_expected,
                ));
                return;
            }

            // Substitute Param(i) in payload with type_args[i]
            let substituted: Vec<Ty> = variant_info.payload.iter()
                .map(|t| Self::subst_with_args(t, &type_args))
                .collect();

            // Arity check
            if substituted.len() != sub_patterns.len() {
                self.diagnostics.push(crate::sema::diag::wrong_arity(
                    p.span, substituted.len(), sub_patterns.len(),
                ));
                return;
            }

            // Recurse with substituted types
            for (sub_p, sub_ty) in sub_patterns.iter().zip(substituted.iter()) {
                self.check_pattern(sub_p, sub_ty);
            }
        }
        // Literal patterns (IntLit, BoolLit, etc.) — check against expected, see Task 7 for exhaustiveness use
    }
}

/// Substitute Ty::Param(i) with type_args[i]. Used for variant payload
/// substitution where type_args come from the scrutinee.
/// Defined as a method on `Ty` in `compiler/src/sema/ty.rs` so Task 7's
/// exhaustiveness checker can also use it (DRY).
impl Ty {
    pub(crate) fn subst_with_args(&self, type_args: &[Ty]) -> Ty {
        match self {
            Ty::Param(i) => type_args.get(*i).cloned().unwrap_or(Ty::Error),
            Ty::Int | Ty::Bool | Ty::String | Ty::Scalar(_) | Ty::Mat(_, _)
            | Ty::Vec(_, _) | Ty::Struct(_) | Ty::Var(_) | Ty::Error => self.clone(),
            Ty::Array(t) => Ty::Array(Box::new(t.subst_with_args(type_args))),
            Ty::Dict(k, v) => Ty::Dict(
                Box::new(k.subst_with_args(type_args)),
                Box::new(v.subst_with_args(type_args)),
            ),
            Ty::Function(args, ret) => Ty::Function(
                args.iter().map(|a| a.subst_with_args(type_args)).collect(),
                Box::new(ret.subst_with_args(type_args)),
            ),
            Ty::Enum(def, args) => Ty::Enum(*def,
                args.iter().map(|a| a.subst_with_args(type_args)).collect(),
            ),
        }
    }
}
```

**New diag helpers** (`compiler/src/sema/diag.rs`):

```rust
pub fn pattern_type_mismatch(span: Span, actual: &Ty, expected_kind: &str) -> Diagnostic {
    Diagnostic::type_error(
        span,
        format!("pattern matches {expected_kind} but scrutinee is `{}`", format_ty(actual)),
    )
}

pub fn wrong_variant_for_enum(span: Span, variant_name: &str, scrut_ty: &Ty) -> Diagnostic {
    Diagnostic::type_error(
        span,
        format!("variant `{variant_name}` does not belong to scrutinee type `{}`", format_ty(scrut_ty)),
    )
}
```

### Steps

- [ ] **Step 1: Write failing tests for generic match patterns (red phase)**

Add to `compiler/src/sema/check.rs::tests`:

```rust
#[test]
fn match_generic_binds_payload_type() {
    // case Some(x) binds x: Int when scrutinee is Maybe<Int>
    let diags = diags_for(
        "enum Maybe<T>\n  Some(T)\n  Nothing\nend\n\
         function f(m: Maybe<Int>): Int\n  return match m\n    case Some(x) then x\n    case Nothing then 0\n  end\nend"
    );
    assert!(diags.is_empty(), "diags: {:?}", diags);
}

#[test]
fn match_generic_payload_type_mismatch() {
    // body returns wrong type for binding
    let diags = diags_for(
        "enum Maybe<T>\n  Some(T)\n  Nothing\nend\n\
         function f(m: Maybe<Int>): String\n  return match m\n    case Some(x) then x\n    case Nothing then \"none\"\n  end\nend"
    );
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
}

#[test]
fn match_two_param_enum_binding() {
    // case Ok(value), Err(e) binds value: Int, e: String for Result<Int, String>
    let diags = diags_for(
        "enum Result<T, E>\n  Ok(T)\n  Err(E)\nend\n\
         function f(r: Result<Int, String>): Int\n  return match r\n    case Ok(value) then value\n    case Err(_) then -1\n  end\nend"
    );
    assert!(diags.is_empty(), "diags: {:?}", diags);
}

#[test]
fn match_wrong_variant_for_enum_diag() {
    // Use Some on a Result scrutinee
    let diags = diags_for(
        "enum Maybe<T>\n  Some(T)\n  Nothing\nend\n\
         enum Result<T, E>\n  Ok(T)\n  Err(E)\nend\n\
         function f(r: Result<Int, String>): Int\n  return match r\n    case Some(x) then 0\n    case Ok(v) then v\n    case Err(_) then -1\n  end\nend"
    );
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
    assert!(diags[0].message.contains("Some") || diags[0].message.contains("Maybe"));
}

#[test]
fn match_pattern_arity_mismatch_diag() {
    // case Some() (no payload) on Maybe<Int> with payload of size 1
    let diags = diags_for(
        "enum Maybe<T>\n  Some(T)\n  Nothing\nend\n\
         function f(m: Maybe<Int>): Int\n  return match m\n    case Some() then 0\n    case Nothing then -1\n  end\nend"
    );
    // pattern arity 0 but payload size 1
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
}

#[test]
fn match_nested_variant_pattern() {
    // case Some(Some(x)) — 2-level nested binding
    let diags = diags_for(
        "enum Maybe<T>\n  Some(T)\n  Nothing\nend\n\
         function f(m: Maybe<Maybe<Int>>): Int\n  return match m\n    case Some(Some(x)) then x\n    case Some(Nothing) then 0\n    case Nothing then -1\n  end\nend"
    );
    assert!(diags.is_empty(), "diags: {:?}", diags);
}
```

- [ ] **Step 2: Verify red phase**

Run: `cd compiler && cargo test match_generic match_two_param match_wrong_variant match_pattern_arity match_nested_variant 2>&1 | head -40`
Expected: tests fail (current check_pattern silently does no substitution; bindings get Ty::Error).

- [ ] **Step 3: Add `subst_with_args` helper**

Add to `compiler/src/sema/check.rs::TypeChecker` impl. Static `Self::subst_with_args(ty, &type_args)` recursive walker.

- [ ] **Step 4: Add new diag helpers**

Add `pattern_type_mismatch` and `wrong_variant_for_enum` to `compiler/src/sema/diag.rs` per the contract.

- [ ] **Step 5: Update `check_pattern` Variant arm**

Replace existing logic per the contract pseudocode. Resolve expected via `unify_table`, validate enum kind and variant ownership, substitute payload, arity check, recurse.

- [ ] **Step 6: Update `check_pattern` Ident arm to use binding_def_ids**

```rust
PatternKind::Ident(name) => {
    if let Some(def_id) = self.binding_def_ids.get(&p.id).copied() {
        self.def_types.insert(def_id, expected.clone());
    }
}
```

(Replaces the old `pattern_binding_def_id` linear-scan helper.)

- [ ] **Step 7: Verify (green)**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 339 + 6 new = **345 tests pass**. Clippy clean. Fmt clean.

- [ ] **Step 8: Refactor**

Refactor opportunities:
- `subst_with_args` and `subst_params` (Task 4) are similar — both walk Ty replacing Params. Differ only in source: one uses `&[Ty]` directly (for use-site substitution), the other uses `&[TypeVarId]` (for fresh-Var instantiation). Consolidate into a single trait or higher-order function if natural; otherwise keep separate (different phases, different call patterns).
- The Variant pattern's "validate enum kind, validate variant ownership, substitute" logic is wide. If it grows another concern (e.g. exhaustiveness side-effects in Task 7), extract into `validate_variant_pattern` helper.

- [ ] **Step 9: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Generic match-pattern type substitution + binding

check_pattern's Variant arm now resolves the scrutinee's type via
unify_table, validates the variant belongs to the enum, substitutes
Ty::Param(i) -> scrutinee_args[i] in the payload schema, and recurses
into sub-patterns with substituted types.

Ident sub-patterns receive the substituted type via binding_def_ids
lookup -> def_types.insert (replacing the old pattern_binding_def_id
linear scan).

Two new diag helpers: pattern_type_mismatch (scrutinee not enum) and
wrong_variant_for_enum (variant from a different enum).

6 regression tests covering: generic payload binding, payload type
mismatch, two-param enum, wrong variant for scrutinee, arity
mismatch, 2-level nested patterns.
EOF
)"
```

---

## Task 6: Built-in `Option<T>` / `Result<T, E>` via `builtins.dy`

**Why:** Spec §4 line 22 requires `Result<T, E>` and `Option<T>` as built-in generic enums. Per Q4 (Hybrid / Path 2), they're declared in dyne syntax in `compiler/builtins/builtins.dy`, embedded via `include_str!`, and parsed → resolved → signature-passed alongside user source. Built-ins compile failure is a compile-time bug → panic.

**Behavior change:** yes (`Option<T>`, `Result<T, E>`, `Some`, `None`, `Ok`, `Err` are now valid names; the soft cross-layer cascade for `Option<X>` from PR-3b auto-resolves).
**Discipline:** TDD.

**Files:**
- Create: `compiler/builtins/builtins.dy` — declares Option<T> and Result<T, E>.
- Create: `compiler/src/sema/builtins.rs` — pipeline that loads builtins.dy, parses, resolves, integrates into the user-source ID space.
- Modify: `compiler/src/lib.rs::compile` — call builtins pipeline before user-source pipeline; thread shared IdGenerator.
- Modify: `compiler/src/parser.rs` (or wherever NodeId is allocated) — accept an optional `id_offset` so user-source NodeIds start after built-in NodeIds.
- Modify: `compiler/src/sema/resolve.rs` — accept built-in BindingTable as initial state; thread through resolve_program.
- Modify: `compiler/src/sema.rs::check` — merge built-in tables with user-source tables before signature_pass.

**`builtins.dy` content**:

```dyne
-- Built-in types provided by the dyne compiler.
-- Loaded via include_str! in compile() before user source.
-- Span positions in this file are reported as 'built-in' in diagnostics.

enum Option<T>
  Some(T)
  None
end

enum Result<T, E>
  Ok(T)
  Err(E)
end
```

**`sema/builtins.rs` content** (~80 LOC):

```rust
//! Built-in type registration via `compiler/builtins/builtins.dy`.
//!
//! Per /design-discussion 2026-05-08 (Q4), built-ins are declared in
//! dyne syntax for self-host friendliness. The compile pipeline loads
//! and processes builtins.dy ahead of user source, sharing the
//! IdGenerator so DefIds and NodeIds are unique across both sources.

use crate::ast::Program;
use crate::diag::Diagnostic;
use crate::ids::IdGenerator;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::sema::resolve::{BindingTable, DefinitionTable, ResolveTable, resolve_program};

const BUILTINS_SOURCE: &str = include_str!("../../builtins/builtins.dy");

pub(crate) struct BuiltinsContext {
    pub program: Program,
    pub resolutions: ResolveTable,
    pub definitions: DefinitionTable,
    pub binding_def_ids: BindingTable,
}

/// Load and process built-ins. On any failure, panics — built-ins should
/// be compile-time-validated and never depend on user input.
pub(crate) fn load_builtins(ids: &mut IdGenerator) -> BuiltinsContext {
    let tokens = tokenize(BUILTINS_SOURCE)
        .unwrap_or_else(|e| panic!("built-ins lex failed: {:?}", e));
    let program = parse_with_ids(tokens, ids)
        .unwrap_or_else(|e| panic!("built-ins parse failed: {:?}", e));
    let (resolutions, definitions, binding_def_ids, diags) = resolve_program(&program);
    if !diags.is_empty() {
        panic!("built-ins resolve failed: {:?}", diags);
    }
    BuiltinsContext {
        program,
        resolutions,
        definitions,
        binding_def_ids,
    }
}
```

**`compile()` integration** (`compiler/src/lib.rs` or wherever the entry point lives):

```rust
pub fn compile(source: &str) -> Result<TypedProgram, Vec<Diagnostic>> {
    let mut ids = IdGenerator::new();

    // Phase 1: built-ins (panics on failure)
    let builtins = sema::builtins::load_builtins(&mut ids);

    // Phase 2: user source
    let user_tokens = tokenize(source)?;
    let user_program = parse_with_ids(user_tokens, &mut ids)?;
    let (user_resolutions, user_definitions, user_binding_def_ids, mut diags) =
        resolve_program_with_initial_state(&user_program, &builtins);

    // Phase 3: merge tables
    let mut all_definitions = builtins.definitions.clone();
    all_definitions.extend(user_definitions);
    let mut all_resolutions = builtins.resolutions.clone();
    all_resolutions.extend(user_resolutions);
    let mut all_binding_def_ids = builtins.binding_def_ids.clone();
    all_binding_def_ids.extend(user_binding_def_ids);

    // Phase 4: signature_pass on combined program
    let combined_items: Vec<&Item> = builtins.program.items.iter().chain(user_program.items.iter()).collect();
    let (def_types, struct_fields, variant_payloads) =
        signature_pass(&combined_items, &all_resolutions, &all_definitions, &all_binding_def_ids, &mut diags);

    // Phase 5: type-check user bodies (built-in bodies are empty/non-existent)
    let (types, type_diags) = check::run(...);
    diags.extend(type_diags);

    if !diags.is_empty() {
        return Err(diags);
    }
    Ok(TypedProgram { ... })
}
```

(Implementation detail: the actual approach for "merging" built-ins and user source needs to handle the resolver's symbol-table pre-population. Easier approach: parse two programs but treat the user-source resolver as starting from a non-empty SymbolTable seeded with built-in names.)

### Steps

- [ ] **Step 1: Write failing tests for built-in availability (red phase)**

Add to `compiler/tests/end_to_end.rs`:

```rust
#[test]
fn builtin_option_resolves_in_type_annotation() {
    let result = dyne::compile("function f(): Option<Int>\n  return Some(1)\nend");
    assert!(result.is_ok(), "expected clean compile, got: {:?}", result.err());
}

#[test]
fn builtin_result_resolves_in_type_annotation() {
    let result = dyne::compile(
        "function f(): Result<Int, String>\n  return Ok(42)\nend"
    );
    assert!(result.is_ok(), "expected clean compile, got: {:?}", result.err());
}

#[test]
fn builtin_some_none_visible() {
    let result = dyne::compile(
        "function f(): Option<Int>\n  return None\nend\n\
         function g(): Option<Int>\n  return Some(1)\nend"
    );
    assert!(result.is_ok(), "expected clean compile, got: {:?}", result.err());
}

#[test]
fn builtin_ok_err_visible() {
    let result = dyne::compile(
        "function f(): Result<Int, String>\n  return Ok(1)\nend\n\
         function g(): Result<Int, String>\n  return Err(\"x\")\nend"
    );
    assert!(result.is_ok(), "expected clean compile, got: {:?}", result.err());
}

#[test]
fn builtin_match_option_compiles() {
    let result = dyne::compile(
        "function f(o: Option<Int>): Int\n  return match o\n    case Some(x) then x\n    case None then 0\n  end\nend"
    );
    assert!(result.is_ok(), "expected clean compile, got: {:?}", result.err());
}
```

- [ ] **Step 2: Verify red phase**

Run: `cd compiler && cargo test builtin_option_resolves builtin_result builtin_some builtin_ok 2>&1 | head -30`
Expected: tests fail with "undefined name `Option`" / "undefined name `Result`" diagnostics (built-ins not yet registered).

- [ ] **Step 3: Create `builtins.dy`**

Write `compiler/builtins/builtins.dy` per the contract content.

- [ ] **Step 4: Add `IdGenerator` to parser**

Modify `compiler/src/parser.rs` (or wherever `next_node_id` lives):

```rust
pub fn parse(tokens: Vec<Token>) -> Result<Program, Vec<Diagnostic>> {
    let mut id_gen = IdGenerator::new();
    parse_with_ids(tokens, &mut id_gen)
}

pub fn parse_with_ids(tokens: Vec<Token>, ids: &mut IdGenerator) -> Result<Program, Vec<Diagnostic>> {
    // ... existing logic but use `ids.next_node_id()` instead of internal counter ...
}
```

`IdGenerator` lives in `compiler/src/ids.rs` (per PR-3a):

```rust
pub struct IdGenerator {
    next_node: u32,
    next_def: u32,
}

impl IdGenerator {
    pub fn new() -> Self { Self { next_node: 0, next_def: 0 } }
    pub fn next_node_id(&mut self) -> NodeId { let id = NodeId(self.next_node); self.next_node += 1; id }
    pub fn next_def_id(&mut self) -> DefId { let id = DefId(self.next_def); self.next_def += 1; id }
}
```

(Likely IdGenerator already exists; verify and reuse.)

- [ ] **Step 5: Create `sema/builtins.rs`**

Write per the contract. Embed built-ins source via `include_str!`. `load_builtins` panics on failure.

Add `pub mod builtins;` to `compiler/src/sema.rs`.

- [ ] **Step 6: Update `resolve_program` to accept built-in initial state**

```rust
pub fn resolve_program_with_initial_state(
    prog: &Program,
    builtins: &BuiltinsContext,
) -> (ResolveTable, DefinitionTable, BindingTable, Vec<Diagnostic>) {
    let mut resolver = Resolver::new();
    // Seed resolver's table with built-in names
    for (def_id, info) in &builtins.definitions {
        if matches!(info.kind, DefKind::Enum | DefKind::EnumVariant) {
            resolver.table.insert_global(info.name.clone(), *def_id);
        }
    }
    // Seed resolver's resolutions, definitions, binding_def_ids with built-ins
    resolver.resolutions = builtins.resolutions.clone();
    resolver.definitions = builtins.definitions.clone();
    resolver.binding_def_ids = builtins.binding_def_ids.clone();
    resolver.next_def_id = compute_next_after(&builtins.definitions);
    
    // Walk user program
    resolver.walk(prog);
    
    (resolver.resolutions, resolver.definitions, resolver.binding_def_ids, resolver.diagnostics)
}

pub fn resolve_program(prog: &Program) -> (ResolveTable, DefinitionTable, BindingTable, Vec<Diagnostic>) {
    // No built-ins seeding — kept for tests that use bare resolve_program.
    let mut resolver = Resolver::new();
    resolver.walk(prog);
    (resolver.resolutions, resolver.definitions, resolver.binding_def_ids, resolver.diagnostics)
}
```

The non-built-in `resolve_program` remains for unit tests in `sema/ty.rs` etc. that don't need built-ins. Top-level `compile()` uses `resolve_program_with_initial_state`.

- [ ] **Step 7: Update `compile()` entry point**

Modify `compiler/src/lib.rs` per the contract pseudocode. Ensure:
- `IdGenerator` is shared between built-ins parse and user-source parse
- `resolve_program_with_initial_state` is called for user source
- Combined items vector is passed to `signature_pass`

- [ ] **Step 8: Update `signature_pass` to accept combined items**

Most of the signature_pass logic already iterates over `&Program.items`. Adapt to accept `&[&Item]` or restructure to take two programs (built-ins + user). Built-ins items go through the same logic, populating the same tables.

Note: if the IdGenerator was correctly shared, built-in DefIds and user DefIds are non-overlapping; the merged tables work correctly without remapping.

- [ ] **Step 9: Verify (green)**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 345 + 5 new = **350 tests pass**. Clippy clean. Fmt clean.

If any pre-existing test fails:
- Tests that defined their own `Option<T>` or `Result<T, E>` will now hit a "duplicate name" error. Adapt those tests to use different names (e.g. `MyOption<T>`).
- Tests that called `resolve_program` directly (without built-ins) and relied on `Option`/`Result` being undefined now have an ambiguous state — verify their assertions still hold.

- [ ] **Step 10: Refactor**

Refactor opportunities:
- The compile() function is now multi-phase. Consider breaking into named helpers (`parse_phase`, `resolve_phase`, `signature_phase`, `check_phase`) for readability.
- `resolve_program_with_initial_state` and `resolve_program` share most logic — extract a shared inner that takes optional initial state.
- The "panic on built-in failure" path is intentional but worth documenting at module level in `sema/builtins.rs`.

- [ ] **Step 11: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Built-in Option<T> and Result<T, E> via builtins.dy

Adds compiler/builtins/builtins.dy declaring the two built-in generic
enums in dyne syntax. compile() now embeds this source via include_str!
and processes it before user code, sharing an IdGenerator so DefIds
and NodeIds are unique across both. Per Q4 (/design-discussion
2026-05-08, Path 2), this hybrid approach favors self-host
friendliness.

The compile pipeline now has a sema/builtins.rs module with
load_builtins(): tokenize → parse → resolve. Resolver gains a
resolve_program_with_initial_state variant that seeds tables from
the built-ins context. signature_pass and check::run iterate over
the combined items.

PR-3b's Option<X> cross-layer cascade is auto-resolved: user code
referring to Option / Result no longer hits resolver "undefined name"
followed by lower_type "PR-3c will land".

5 e2e tests covering: Option in type annotation, Result in type
annotation, Some/None visible, Ok/Err visible, match on Option<Int>
compiles.

Built-ins compile failure is a compile-time bug -> panic.
EOF
)"
```

---

## Task 7: Match exhaustiveness in `sema/exhaust.rs`

**Why:** Spec §4 line 286 requires exhaustive match. Per Q3, exhaust.rs uses per-scrutinee table-driven dispatch with 1-level payload recursion. Each scrutinee type kind has a "completeness rule" plug-in; new types (e.g. future Tuple) add a new arm without restructuring.

**Behavior change:** yes (non-exhaustive matches now produce compile errors).
**Discipline:** TDD.

**Files:**
- Create: `compiler/src/sema/exhaust.rs` (~200 LOC) — per-scrutinee dispatch with 1-level payload recursion.
- Modify: `compiler/src/sema.rs` — `pub mod exhaust;`.
- Modify: `compiler/src/sema/check.rs::synth_match` — call exhaust check after arm checking.
- Modify: `compiler/src/sema/diag.rs` — add `non_exhaustive_enum`, `non_exhaustive_bool`, `requires_wildcard`, `not_matchable`.

**`exhaust.rs` skeleton**:

```rust
//! Match exhaustiveness checker.
//!
//! Per /design-discussion 2026-05-08 (Q3 I-extended), uses per-scrutinee
//! table-driven dispatch with 1-level payload recursion. Each Ty kind
//! has a coverage rule. New scrutinee kinds plug in by adding a match
//! arm. Maranget-style arbitrary nesting deferred (future replacement
//! is internal-only swap of this module).

use crate::ast::{MatchArm, Pattern, PatternKind};
use crate::diag::Diagnostic;
use crate::source::Span;
use crate::sema::resolve::DefinitionTable;
use crate::sema::ty::Ty;
use crate::sema::VariantPayloadMap;
use std::collections::HashSet;

pub(crate) fn check_exhaustive(
    scrut_ty: &Ty,
    arms: &[MatchArm],
    span: Span,
    definitions: &DefinitionTable,
    variant_payloads: &VariantPayloadMap,
) -> Vec<Diagnostic> {
    match scrut_ty {
        Ty::Enum(def_id, type_args) => check_enum_coverage(*def_id, type_args, arms, span, definitions, variant_payloads),
        Ty::Bool => check_bool_coverage(arms, span),
        Ty::Int | Ty::Scalar(_) | Ty::String => require_catchall(arms, span, kind_name(scrut_ty)),
        Ty::Vec(_, _) | Ty::Mat(_, _) | Ty::Array(_) | Ty::Dict(_, _) => require_catchall(arms, span, kind_name(scrut_ty)),
        Ty::Struct(_) => check_struct_coverage(arms, span),
        Ty::Function(_, _) => vec![Diagnostic::type_error(span, "match on function value is not allowed".into())],
        Ty::Var(_) | Ty::Param(_) | Ty::Error => vec![],  // skip
    }
}

fn check_enum_coverage(
    def_id: DefId,
    type_args: &[Ty],
    arms: &[MatchArm],
    span: Span,
    definitions: &DefinitionTable,
    variant_payloads: &VariantPayloadMap,
) -> Vec<Diagnostic> {
    // Collect all variant DefIds belonging to this enum
    let all_variants: Vec<DefId> = variant_payloads.iter()
        .filter_map(|(vid, info)| if info.parent_enum == def_id { Some(*vid) } else { None })
        .collect();

    let mut covered_variants: HashSet<DefId> = HashSet::new();
    let mut variant_payload_arms: HashMap<DefId, Vec<&Pattern>> = HashMap::new();

    for arm in arms {
        match &arm.pattern.kind {
            PatternKind::Variant(name, sub_patterns) => {
                // Resolve variant DefId via resolutions (in caller)
                // ... lookup variant DefId via name match in all_variants ...
                let Some(variant_def) = lookup_variant(definitions, &all_variants, name) else { continue; };
                covered_variants.insert(variant_def);
                variant_payload_arms.entry(variant_def).or_default().extend(sub_patterns);
            }
            PatternKind::Ident(_) | PatternKind::Wildcard => return vec![],  // catch-all
            _ => {}  // type-error reported elsewhere
        }
    }

    let mut diags = vec![];

    // Top-level missing variants
    let missing: Vec<&str> = all_variants.iter()
        .filter(|vid| !covered_variants.contains(vid))
        .filter_map(|vid| definitions.get(vid).map(|info| info.name.as_str()))
        .collect();
    if !missing.is_empty() {
        diags.push(crate::sema::diag::non_exhaustive_enum(span, &missing));
    }

    // 1-level payload recursion: for each covered variant, check sub-pattern coverage
    for (variant_def, sub_patterns) in &variant_payload_arms {
        let Some(payload) = variant_payloads.get(variant_def) else { continue; };
        let substituted_payload: Vec<Ty> = payload.payload.iter()
            .map(|t| substitute_param(t, type_args))
            .collect();
        // For each payload position, gather the patterns at that column
        for (col, sub_ty) in substituted_payload.iter().enumerate() {
            let column_patterns: Vec<&Pattern> = sub_patterns.iter()
                .filter_map(|p| match &p.kind {
                    PatternKind::Variant(_, inners) => inners.get(col),
                    _ => None,
                })
                .collect();
            // Recursively check exhaustiveness of column patterns against sub_ty
            // (with synthetic arms — wraps each column pattern into a MatchArm)
            // ... recursive call ...
        }
    }

    diags
}

fn check_bool_coverage(arms: &[MatchArm], span: Span) -> Vec<Diagnostic> {
    let mut seen_true = false;
    let mut seen_false = false;
    for arm in arms {
        match &arm.pattern.kind {
            PatternKind::BoolLit(true) => seen_true = true,
            PatternKind::BoolLit(false) => seen_false = true,
            PatternKind::Ident(_) | PatternKind::Wildcard => return vec![],
            _ => {}
        }
    }
    let mut missing = vec![];
    if !seen_true { missing.push("true"); }
    if !seen_false { missing.push("false"); }
    if missing.is_empty() {
        vec![]
    } else {
        vec![crate::sema::diag::non_exhaustive_bool(span, &missing)]
    }
}

fn require_catchall(arms: &[MatchArm], span: Span, kind: &str) -> Vec<Diagnostic> {
    if arms.iter().any(|a| matches!(a.pattern.kind, PatternKind::Ident(_) | PatternKind::Wildcard)) {
        vec![]
    } else {
        vec![crate::sema::diag::requires_wildcard(span, kind)]
    }
}

fn check_struct_coverage(arms: &[MatchArm], span: Span) -> Vec<Diagnostic> {
    if arms.iter().any(|a| matches!(a.pattern.kind, PatternKind::Ident(_) | PatternKind::Wildcard)) {
        vec![]
    } else {
        vec![]   // struct destructure pattern (if introduced) handled here later
    }
}

// Substitute Param(i) with type_args[i] (for nested payload recursion).
// Defined as a method on Ty (in compiler/src/sema/ty.rs) so Task 5's
// check_pattern and Task 7's exhaust both share one implementation.
//
// pub(crate) impl Ty {
//     pub(crate) fn subst_with_args(&self, type_args: &[Ty]) -> Ty {
//         match self {
//             Ty::Param(i) => type_args.get(*i).cloned().unwrap_or(Ty::Error),
//             Ty::Int | Ty::Bool | Ty::String | Ty::Scalar(_) | Ty::Mat(_, _)
//             | Ty::Vec(_, _) | Ty::Struct(_) | Ty::Var(_) | Ty::Error => self.clone(),
//             Ty::Array(t) => Ty::Array(Box::new(t.subst_with_args(type_args))),
//             Ty::Dict(k, v) => Ty::Dict(
//                 Box::new(k.subst_with_args(type_args)),
//                 Box::new(v.subst_with_args(type_args)),
//             ),
//             Ty::Function(args, ret) => Ty::Function(
//                 args.iter().map(|a| a.subst_with_args(type_args)).collect(),
//                 Box::new(ret.subst_with_args(type_args)),
//             ),
//             Ty::Enum(def, args) => Ty::Enum(*def,
//                 args.iter().map(|a| a.subst_with_args(type_args)).collect(),
//             ),
//         }
//     }
// }
//
// Then call as: payload.subst_with_args(type_args)

fn kind_name(ty: &Ty) -> &'static str {
    match ty {
        Ty::Int => "Int",
        Ty::Scalar(_) => "Scalar",
        Ty::String => "String",
        Ty::Vec(_, _) => "Vec",
        Ty::Mat(_, _) => "Mat",
        Ty::Array(_) => "Array",
        Ty::Dict(_, _) => "Dict",
        _ => "type",
    }
}

fn lookup_variant(definitions: &DefinitionTable, candidates: &[DefId], name: &str) -> Option<DefId> {
    candidates.iter()
        .find(|vid| definitions.get(vid).map(|info| info.name == name).unwrap_or(false))
        .copied()
}
```

**New diag helpers**:

```rust
pub fn non_exhaustive_enum(span: Span, missing_variants: &[&str]) -> Diagnostic {
    Diagnostic::type_error(
        span,
        format!("non-exhaustive match: missing variant(s) {}", missing_variants.join(", ")),
    )
}

pub fn non_exhaustive_bool(span: Span, missing: &[&str]) -> Diagnostic {
    Diagnostic::type_error(
        span,
        format!("non-exhaustive match on Bool: missing {}", missing.join(", ")),
    )
}

pub fn requires_wildcard(span: Span, kind: &str) -> Diagnostic {
    Diagnostic::type_error(
        span,
        format!("non-exhaustive match on `{kind}`: a wildcard pattern (`_` or binding) is required"),
    )
}
```

### Steps

- [ ] **Step 1: Write failing tests for exhaustiveness (red phase)**

Add to `compiler/src/sema/check.rs::tests` or new `compiler/src/sema/exhaust.rs::tests`:

```rust
#[test]
fn match_enum_missing_variant_diag() {
    let diags = diags_for(
        "enum Maybe<T>\n  Some(T)\n  None\nend\n\
         function f(m: Maybe<Int>): Int\n  return match m\n    case Some(x) then x\n  end\nend"
    );
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
    assert!(diags[0].message.contains("None") || diags[0].message.contains("missing"));
}

#[test]
fn match_enum_with_wildcard_passes() {
    let diags = diags_for(
        "enum Maybe<T>\n  Some(T)\n  None\nend\n\
         function f(m: Maybe<Int>): Int\n  return match m\n    case Some(x) then x\n    case _ then 0\n  end\nend"
    );
    assert!(diags.is_empty(), "diags: {:?}", diags);
}

#[test]
fn match_bool_missing_false_diag() {
    let diags = diags_for(
        "function f(b: Bool): Int\n  return match b\n    case true then 1\n  end\nend"
    );
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
    assert!(diags[0].message.contains("false") || diags[0].message.contains("Bool"));
}

#[test]
fn match_int_requires_wildcard_diag() {
    let diags = diags_for(
        "function f(i: Int): Int\n  return match i\n    case 0 then 0\n  end\nend"
    );
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
    assert!(diags[0].message.contains("wildcard"));
}

#[test]
fn match_struct_with_ident_passes() {
    let diags = diags_for(
        "struct P\n  x: Int\n  y: Int\nend\n\
         function f(p: P): Int\n  return match p\n    case s then s.x\n  end\nend"
    );
    assert!(diags.is_empty(), "diags: {:?}", diags);
}

#[test]
fn match_function_value_not_matchable_diag() {
    // Match on a Ty::Function value (e.g. let f = some_fn; match f { ... })
    // This is contrived but verifies the not_matchable path.
    let diags = diags_for(
        "function g(): Int\n  return 0\nend\n\
         function f(): Int\n  return match g\n    case _ then 0\n  end\nend"
    );
    assert!(diags.iter().any(|d| d.message.contains("function") && d.message.contains("not allowed")),
            "diags: {:?}", diags);
}

#[test]
fn match_nested_payload_some_some_some_none_diag() {
    let diags = diags_for(
        "function f(oo: Option<Option<Int>>): Int\n\
           return match oo\n    case Some(Some(x)) then x\n    case None then -1\n  end\nend"
    );
    // Missing case Some(None) at the inner level.
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
    assert!(diags[0].message.contains("None"));
}

#[test]
fn match_nested_payload_complete_passes() {
    let diags = diags_for(
        "function f(oo: Option<Option<Int>>): Int\n\
           return match oo\n    case Some(Some(x)) then x\n    case Some(None) then 0\n    case None then -1\n  end\nend"
    );
    assert!(diags.is_empty(), "diags: {:?}", diags);
}

#[test]
fn match_int_with_wildcard_passes() {
    let diags = diags_for(
        "function f(i: Int): Int\n  return match i\n    case 0 then 0\n    case _ then 1\n  end\nend"
    );
    assert!(diags.is_empty(), "diags: {:?}", diags);
}

#[test]
fn match_array_requires_wildcard() {
    let diags = diags_for(
        "function f(xs: Array<Int>): Int\n  return match xs\n    case _ then 0\n  end\nend"
    );
    assert!(diags.is_empty(), "diags: {:?}", diags);
}

#[test]
fn match_dict_requires_wildcard_diag() {
    let diags = diags_for(
        "function f(d: Dict<Int, String>): Int\n  return match d\n    case s then 0\n  end\nend"
    );
    // Ident binding is catch-all, should pass
    assert!(diags.is_empty(), "diags: {:?}", diags);
}

#[test]
fn match_two_param_enum_missing_variant_diag() {
    let diags = diags_for(
        "function f(r: Result<Int, String>): Int\n  return match r\n    case Ok(v) then v\n  end\nend"
    );
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
    assert!(diags[0].message.contains("Err"));
}
```

- [ ] **Step 2: Verify red phase**

Run: `cd compiler && cargo test match_enum_missing match_bool_missing match_int_requires match_nested 2>&1 | head -40`
Expected: tests fail (no exhaustiveness checking yet — non-exhaustive matches compile silently).

- [ ] **Step 3: Create `sema/exhaust.rs`**

Write per the contract. Implement `check_exhaustive`, `check_enum_coverage`, `check_bool_coverage`, `require_catchall`, `check_struct_coverage`, `substitute_param`, `kind_name`, `lookup_variant`.

For 1-level payload recursion: when processing covered variants, gather payload-position patterns and recursively call `check_exhaustive_simple` (a non-recursive variant that handles only flat patterns) for each payload column.

- [ ] **Step 4: Add `pub mod exhaust;` to `sema.rs`**

```rust
pub mod exhaust;
```

- [ ] **Step 5: Add new diag helpers**

Add `non_exhaustive_enum`, `non_exhaustive_bool`, `requires_wildcard` to `compiler/src/sema/diag.rs` per the contract.

`not_matchable` for Ty::Function is built inline in exhaust.rs (one-off, no helper).

- [ ] **Step 6: Wire `synth_match` to call exhaust**

In `compiler/src/sema/check.rs::synth_match`:

```rust
fn synth_match(&mut self, scrutinee: &Expr, arms: &[MatchArm]) -> Ty {
    let scrut_ty = self.synth_expr(scrutinee);
    let resolved_scrut = self.unify_table.resolve(&scrut_ty);

    // Existing arm walking + unification
    let mut arm_ty = ...;
    // (keep existing logic)

    // NEW: exhaustiveness check
    let exhaust_diags = exhaust::check_exhaustive(
        &resolved_scrut,
        arms,
        scrutinee.span,
        self.definitions,
        self.variant_payloads,
    );
    self.diagnostics.extend(exhaust_diags);

    arm_ty
}
```

- [ ] **Step 7: Verify (green)**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 350 + 12 new = **362 tests pass**. Clippy clean. Fmt clean.

If pre-existing tests break (e.g., existing `synth_match` tests that wrote non-exhaustive matches), update those tests to add catch-all or full coverage.

- [ ] **Step 8: Refactor**

Refactor opportunities:
- `check_enum_coverage`'s 1-level payload recursion is the most complex part. Consider extracting `check_payload_column_coverage` as a separate helper.
- The `require_catchall` and `check_struct_coverage` are very similar — the only difference is the diag message. Could parameterize.
- Compile-time check that the dispatch in `check_exhaustive` covers all `Ty` variants — Rust's exhaustive `match` does this automatically. Verify no `_` arms.

- [ ] **Step 9: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Match exhaustiveness in sema/exhaust.rs

Per /design-discussion 2026-05-08 (Q3 I-extended): per-scrutinee
table-driven dispatch with 1-level payload recursion.

Coverage rules:
- Enum: all variants required (catch-all `_` or Ident satisfies)
- Bool: true and false both required
- Int/Scalar/String/Vec/Mat/Array/Dict: catch-all required
- Struct: single shape, always exhaustive (Ident satisfies)
- Function: not-matchable diag
- Var/Param/Error: skip (no-cascade)

1-level payload recursion catches `case Some(Some(x))` /
`case Some(None)` gaps at the inner level. Maranget-style arbitrary
nesting deferred (Q3 YAGNI; future internal-only swap).

3 new diag helpers: non_exhaustive_enum (lists missing variants),
non_exhaustive_bool, requires_wildcard.

12 new tests covering each scrutinee kind, with positive and
negative paths plus the nested payload case.

synth_match now invokes exhaust::check_exhaustive after arm
unification. Resolved scrutinee type is used (Vars get unwound).
EOF
)"
```

---

## Task 8: End-to-end: adapt option_match.dy + bundle PR-3b carries

**Why:** With Tasks 1–7 complete, the sample `option_match.dy` (currently using non-generic `MaybeMeasurement` per PR-3b's Task 7 adaptation) can be reverted to use built-in `Option<T>`. This is the canonical end-to-end test of generics + match exhaustiveness + built-ins. This task also bundles 3 PR-3b carry-overs per Q6: `synth_pow` `^` text fix, `mat_shape_mismatch` unused arg, `TypedProgram::new` inline.

**Behavior change:** yes for the carries (cosmetic/error-message correctness improvements); no for the sample (the sample doesn't run, only its compile success matters).
**Discipline:** Mixed — sample adaptation is no-test-needed (uses existing every_sample_parses), carries are Refactor + small cleanup, e2e tests are TDD.

**Files:**
- Modify: `samples/option_match.dy` — restore `Option<T>` usage.
- Modify: `compiler/src/sema/check.rs::synth_pow` — fix diag text from `**` to `^`.
- Modify: `compiler/src/sema/diag.rs::mat_shape_mismatch` — drop unused `actual.0` arg.
- Modify: `compiler/src/sema.rs::TypedProgram::new` — inline at call site (struct literal); remove `#[allow(clippy::too_many_arguments)]`.
- Add: `compiler/tests/end_to_end.rs` — e2e tests for generic match exhaustiveness, Option / Result usage.

**`samples/option_match.dy` restored** (~30 lines):

```dyne
-- Demonstrates generic enum usage and match exhaustiveness.

struct Measurement
  value: Scalar
  uncertainty: Scalar
end

function value_or_default(known: Option<Measurement>, default: Scalar): Scalar
  return match known
    case Some(m) then m.value
    case None then default
  end
end

let known: Option<Measurement> = Some(Measurement { value: 9.81, uncertainty: 0.01 })
let unknown: Option<Measurement> = None
```

### Steps

- [ ] **Step 1: Write failing e2e tests for full generic + match flow (red phase)**

Add to `compiler/tests/end_to_end.rs`:

```rust
#[test]
fn compile_generic_enum_with_exhaustive_match() {
    let result = dyne::compile(
        "function f(o: Option<Int>): Int\n\
           return match o\n    case Some(x) then x\n    case None then 0\n  end\nend"
    );
    assert!(result.is_ok(), "expected clean compile, got: {:?}", result.err());
}

#[test]
fn compile_generic_match_non_exhaustive_yields_diagnostic() {
    let result = dyne::compile(
        "function f(o: Option<Int>): Int\n\
           return match o\n    case Some(x) then x\n  end\nend"
    );
    let diags = result.unwrap_err();
    assert_eq!(diags.len(), 1, "diags: {:?}", diags);
    assert!(diags[0].message.contains("None"));
}

#[test]
fn compile_result_with_pattern_binding() {
    let result = dyne::compile(
        "function f(r: Result<Int, String>): Int\n\
           return match r\n    case Ok(v) then v\n    case Err(_) then -1\n  end\nend"
    );
    assert!(result.is_ok(), "expected clean compile, got: {:?}", result.err());
}

#[test]
fn compile_user_defined_generic_enum_e2e() {
    let result = dyne::compile(
        "enum Maybe<T>\n  Just(T)\n  Nothing\nend\n\
         function f(m: Maybe<Int>): Int\n\
           return match m\n    case Just(x) then x\n    case Nothing then 0\n  end\nend"
    );
    assert!(result.is_ok(), "expected clean compile, got: {:?}", result.err());
}

#[test]
fn compile_pow_diag_uses_caret_syntax() {
    let result = dyne::compile(
        "function f(): Int\n  return true ^ 2\nend"
    );
    let diags = result.unwrap_err();
    assert!(diags.iter().any(|d| d.message.contains("`^`")),
            "expected `^` in diag, got: {:?}", diags);
    assert!(diags.iter().all(|d| !d.message.contains("`**`")),
            "no diag should reference `**`, got: {:?}", diags);
}
```

- [ ] **Step 2: Verify red phase**

Run: `cd compiler && cargo test compile_generic compile_result compile_user_defined compile_pow_diag 2>&1 | head -30`
Expected: tests fail or partially pass depending on Tasks 4–7 completion. After Tasks 4–7 are complete, generic_enum_with_exhaustive_match should already pass — but the `pow_diag_uses_caret_syntax` test will fail (synth_pow text not yet fixed).

- [ ] **Step 3: Adapt `samples/option_match.dy`**

Replace contents per the contract. Verify no `MaybeMeasurement` references remain. The header comment from PR-3b ("future revision restores `Option<T>`") should also be removed.

- [ ] **Step 4: Fix `synth_pow` diag text**

In `compiler/src/sema/check.rs::synth_pow`:

```rust
// OLD:
self.diagnostics.push(crate::sema::diag::op_type_error(
    e.span, "`**` base", &xt,
));

// NEW:
self.diagnostics.push(crate::sema::diag::op_type_error(
    e.span, "`^` base", &xt,
));
```

Same for the exponent diag (and any other `**` reference in the file).

- [ ] **Step 5: Drop `mat_shape_mismatch` unused arg**

In `compiler/src/sema/diag.rs::mat_shape_mismatch`, change signature:

```rust
// OLD:
pub fn mat_shape_mismatch(span: Span, expected: (usize, usize), actual: (usize, usize)) -> Diagnostic

// NEW:
pub fn mat_shape_mismatch(span: Span, expected: (usize, usize), actual_cols: usize) -> Diagnostic {
    Diagnostic::type_error(
        span,
        format!("matrix shape mismatch: expected {} columns, found {}", expected.1, actual_cols),
    )
}
```

Update the call site in `synth_mat_lit` to pass only the columns count.

- [ ] **Step 6: Inline `TypedProgram::new` at call site**

In `compiler/src/sema.rs::check`, replace:

```rust
TypedProgram::new(program, types, resolutions, definitions, binding_def_ids, def_types, struct_fields, variant_payloads)
```

with a struct literal:

```rust
TypedProgram {
    program,
    types,
    resolutions,
    definitions,
    binding_def_ids,
    def_types,
    struct_fields,
    variant_payloads,
}
```

Delete the `TypedProgram::new` function and its `#[allow(clippy::too_many_arguments)]` attribute.

- [ ] **Step 7: Verify (green)**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
cargo run --quiet -- ../samples/option_match.dy
```

Expected: 362 + 5 new = **367 tests pass**. Clippy clean. Fmt clean. `option_match.dy` smoke test prints "parsed N item(s)" successfully.

If `every_sample_parses` was relying on the PR-3b `MaybeMeasurement` adaptation, it's now restored to test the canonical Option<T> form.

- [ ] **Step 8: Commit (split into 4 commits per Q6 s1)**

Commit 1 — sample adaptation:
```sh
git add samples/option_match.dy
git commit -m "$(cat <<'EOF'
Restore Option<T> in option_match.dy now that PR-3c lands generics

The PR-3b adaptation that replaced Option<T> with non-generic
MaybeMeasurement is no longer needed. Sample exercises the canonical
generic enum + match form, providing an end-to-end test of generics +
exhaustiveness + built-ins.
EOF
)"
```

Commit 2 — synth_pow text:
```sh
git add compiler/src/sema/check.rs
git commit -m "$(cat <<'EOF'
Fix synth_pow diagnostic text: `**` -> `^`

dyne uses `^` for the power operator (token Caret in the parser).
synth_pow's diagnostics referenced `**` from an early prototype.
Cosmetic correctness; no behavior change.
EOF
)"
```

Commit 3 — mat_shape_mismatch arg cleanup:
```sh
git add compiler/src/sema/diag.rs compiler/src/sema/check.rs
git commit -m "$(cat <<'EOF'
mat_shape_mismatch: drop unused expected-rows tuple position

The diag helper accepted a `(usize, usize)` for actual-shape but the
call site always passed `(rows.len(), row.len())` where rows.len()
matched expected.0 by construction. Simplifies signature to take only
the offending column count. Pre-existing tech debt called out in
PR-3b's /review.
EOF
)"
```

Commit 4 — TypedProgram::new inline + e2e tests:
```sh
git add -A
git commit -m "$(cat <<'EOF'
Inline TypedProgram::new; e2e tests for PR-3c

The vestigial 7-arg constructor (now 8-arg with binding_def_ids) had a
single private call site and #[allow(clippy::too_many_arguments)].
Replaced with a struct literal; #[allow] removed. Pre-existing
tech debt called out in PR-3b's /review.

Adds 5 e2e tests in tests/end_to_end.rs:
- compile_generic_enum_with_exhaustive_match
- compile_generic_match_non_exhaustive_yields_diagnostic
- compile_result_with_pattern_binding
- compile_user_defined_generic_enum_e2e
- compile_pow_diag_uses_caret_syntax (regression for synth_pow text fix)
EOF
)"
```

---

## Final verification (after all tasks)

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
cargo run --quiet -- ../samples/option_match.dy
```

Expected:
- **~367-380 tests pass** (final number depends on review-fix carryover; absolute count less important than 0 failures)
- Clippy `-D warnings` clean
- `cargo fmt -- --check` clean
- Smoke test on `option_match.dy` prints "parsed N item(s)" successfully

## Post-/review iteration (autonomous loop per CLAUDE.md Core Flow)

After Final verification passes, invoke `/review`. The skill launches four parallel aspect-specific reviewers:

- **design-alignment** — verifies the implementation matches `docs/design/typechecker.md` § 3c
- **code-quality** (with `ultrathink`) — surfaces non-obvious correctness, performance, idiom issues; exercises cross-task coherence
- **test-coverage** — audits behaviors that should be tested but aren't
- **scope-completeness** — verifies in-scope items done, out-of-scope items not crept in

If `/review` surfaces Must Fix or Should Improve items, Claude Code applies `/receiving-code-review` (verify, push back, YAGNI) and **appends fix tasks to this plan**, then re-enters `/execute-plan` autonomously. Loop continues until `/review` reports no remaining items.

Items requiring **design changes** (architecture, DD contracts, scope expansion) are **escalated to the engineer**.

PR-3a/3b precedent: ~2–6 fix tasks emerged from `/review`. Reserve budget accordingly.

## Push and PR

```sh
git push -u origin stage3c-generics
gh pr create --base main --title "Stage 3c: generics + match exhaustiveness" --body "..."
```

PR description should explain:
- Generics: variant signatures with Ty::Param schema, fresh-Var instantiation, type-arg substitution in match patterns
- Match exhaustiveness: per-scrutinee table-driven + 1-level payload recursion (Q3 I-extended)
- Built-ins: Option<T> / Result<T, E> via builtins.dy (Q4 hybrid)
- Resolver refactor: binding_def_ids replaces 5 linear-scan helpers (Q5)
- Bundled trivia: synth_pow ^ text, mat_shape_mismatch arg, TypedProgram::new inline (Q6)
- Closes PR-3b's silent variant-constructor soundness gap (side effect of Q2)
- Ref: docs/design/typechecker.md § PR-3c, docs/plans/2026-05-08-stage3c-generics.md

## Out of scope (deferred to later PRs)

- User-defined generic functions — non-goal per spec; spec §6 says "generics are enum type parameters only"
- Maranget-style arbitrary nested pattern exhaustiveness — Q3 YAGNI; 1-level recursion suffices for current dyne usage. Future replacement is internal-only swap of `sema/exhaust.rs`.
- Stdlib generic functions (`kahan_sum<T>`, etc.) — PR-3e
- Unit propagation through operators — PR-3d (replaces ZERO placeholders + activates `Scalar<kg>` semantics)
- Spec §6.1 precision warnings — PR-3e
- Lambda body type checking — parser-gated (parser doesn't currently construct `ExprKind::Lambda`)
- `synth_arith` Mat·Vec arm order misleading diag — PR-3d (replaced by real shape rule)
- Open-sum interfaces / sealed traits — speculative; not on Stage 3 roadmap

## Alternative Solutions Considered

The high-level design space is captured in `docs/design/typechecker.md` § Alternatives. PR-3c sub-decisions settled during /design-discussion (2026-05-08), recorded in the relevant sections of the plan above:

- **Option α (Q1: Var only as wrapping, not in Ty::Enum args)**. **Rejected because**: dyne's generic enum semantics require Var inside `Ty::Enum(def, [Var(α)])`. There is no design-coherent way to wrap-only.
- **Option A (Q2: regenerate variant signature on each synth_ident call by re-walking AST)**. **Rejected because**: O(constructor-calls × ast-walk) is wasteful when a compiled schema can be cached in def_types. Option B (`Ty::Param` sentinel) is the standard HM approach.
- **Option C (Q2: TypeVarId sentinel for both bound params and unsolved holes)**. **Rejected because**: conflates "schema parameter" and "fresh hole" semantics. Bug-prone — every traversal of `Ty::Var` would need to ask "is this bound or fresh?" Option B's distinct `Ty::Param` keeps semantics orthogonal.
- **Option II (Q3: Maranget pattern matrix, full nested coverage)**. **Rejected because**: ~300-500 LOC for handling that dyne's domain (computational physics) doesn't need. Option I-extended (1-level recursion) covers `Some(Ok(_))` etc. with ~200 LOC. Future swap is internal-only.
- **Option p (Q4: pure compiler-side `register_builtins`)**. **Rejected because**: engineer judgment that dyne's roadmap targets self-host friendliness; declarations expressible in dyne syntax should be in dyne source. Option q (Path 2) hybrid takes the best of both — Option/Result in dyne syntax, future generic stdlib functions remain compiler-side.
- **Option q (Q4: pure dyne source, including stdlib functions in dyne syntax)**. **Rejected because**: spec §6 line 207 designates `kahan_sum<T>` and similar as built-in generic functions, not user-expressible (user-generic-functions are non-goal). Pure q would force a user-generic-function feature.
- **Option r1 (Q5: keep linear-scan helper)**. **Rejected because**: O(definitions) scans accumulate as Tasks 4–7 add more bindings. Resolver-side `binding_def_ids` is ~50 LOC additive and collapses 5 sites to O(1). Net positive.
- **Option r3 (Q5: extend `resolutions` to include intro NodeIds)**. **Rejected because**: muddies use-site vs intro-site semantics. Orthogonal table preserves clarity.
- **Option s2 (Q6: separate cleanup PR for trivia)**. **Rejected because**: engineer time tax of an extra worktree + verify + review + merge cycle for 3 trivial fixes. PR-3c's `binding_def_ids` addition naturally motivates `TypedProgram::new` inline (would otherwise become 8-arg). Bundling with split commits keeps PR review readable.
- **Option s3 (Q6: defer trivia indefinitely)**. **Rejected because**: forgetting risk; trivial fixes accumulate as tech debt.
