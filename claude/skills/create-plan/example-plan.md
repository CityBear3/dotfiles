<!--
This file is a real plan from the Dyne compiler project, illustrating the
standard plan format described in SKILL.md. It is referenced as an example
only — do not edit unless updating the format itself. To regenerate, copy
a fresh plan from a real project and adjust this header.
-->

# Cross-cutting Parser Refactor (PR-A) Implementation Plan

> **Execution:** Use `/execute-plan` to dispatch this plan to agent-teams. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Reduce duplication and noise across the Stage 1+2 parser without changing the language's accepted surface form, except for fixing the Stage 1 `parse_block_until` span overshoot bug as a side effect of helper unification.

**Architecture:** 5 commits (one per task). Items: (3) aggregate `use` statements, (2) replace `peek().clone()` idiom site-by-site, (1) introduce `parse_comma_list` helper + migrate 11 sites, (4) introduce `parse_block_body` helper + migrate `parse_block_until` and `parse_match_arm_body` to wrap it (this fixes Stage 1 span bug), (5) pin Stage 1 block span behavior with new tests.

**Tech Stack:** Rust 2024 edition. Zero runtime deps. Cargo for build/test/lint/fmt.

**Working directory:** `.claude/worktrees/refactor-cross-cutting/compiler/` (run all `cargo` commands from there).
**Branch:** `refactor-cross-cutting`.
**Baseline before Task 1:** 183 tests passing, clippy `-D warnings` clean, `cargo fmt --check` clean.

**Per-task verification command** (mandatory before each commit):
```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

---

## Task 1: Aggregate function-internal `use` statements to top-of-file

**Why:** During Stage 2 agent-teams execution, several `use` statements were added inside functions where the imported items were used. They are now scattered. Aggregating to top-of-file improves readability.

**Files:**
- Modify: `compiler/src/parser/stmt.rs`
- Modify: `compiler/src/parser/expr.rs`

**Internal-`use` statements to remove and add to top-of-file:**

`stmt.rs` — remove function-local `use` lines:
- Line 54: `use crate::ast::{StructDef, StructField};` (inside `parse_struct_def`)
- Line 106: `use crate::ast::EnumDef;` (inside `parse_enum_def`)
- Line 146: `use crate::ast::EnumVariant;` (inside `parse_variant_decl`)

`expr.rs` — remove function-local `use` lines:
- Line 127: `use crate::ast::IfExpr;` (inside `parse_if_expr`)
- Line 128: `use crate::parser::stmt::{TokenKindKind, parse_block_until};` (inside `parse_if_expr`)
- Line 484: `use crate::ast::MatchArm;` (inside `parse_match_expr`)
- Line 529: `use crate::ast::Block;` (inside `parse_match_arm_body`)
- Line 530: `use crate::parser::stmt::parse_stmt;` (inside `parse_match_arm_body`)
- Line 371: `use crate::ast::{Pattern, PatternKind};` (mid-file module-level — also move to top-of-file)

### Steps

- [ ] **Step 1: Update top-of-file `use` block in `compiler/src/parser/stmt.rs`**

Replace the existing `use` block at the top (lines 3-11) with:

```rust
use crate::ast::{
    Block, EnumDef, EnumVariant, ExprKind, FunctionDef, Param, Program, Stmt, StmtKind,
    StructDef, StructField,
};
use crate::error::CompileError;
use crate::lexer::TokenKind;
use crate::parser::Parser;
use crate::parser::expr::parse_expr;
use crate::parser::types::parse_type;
use crate::source::Span;
```

Verify the existing top-of-file `use crate::ast::{ ... }` already imports the original Stage 1 names (Block, ExprKind, FunctionDef, Param, Program, Stmt, StmtKind). Only add the three new imports (StructDef, StructField, EnumDef, EnumVariant) to that single `use crate::ast::{...}` group.

- [ ] **Step 2: Remove function-local `use` lines in `compiler/src/parser/stmt.rs`**

Delete lines 54, 106, 146 (the three `use crate::ast::...;` lines inside `parse_struct_def`, `parse_enum_def`, `parse_variant_decl`).

- [ ] **Step 3: Update top-of-file `use` block in `compiler/src/parser/expr.rs`**

Replace lines 3-7 with:

```rust
use crate::ast::{BinOp, Block, Expr, ExprKind, IfExpr, MatchArm, Pattern, PatternKind, UnaryOp};
use crate::error::CompileError;
use crate::lexer::TokenKind;
use crate::parser::Parser;
use crate::parser::stmt::{TokenKindKind, parse_block_until, parse_stmt};
use crate::source::Span;
```

- [ ] **Step 4: Remove function-local and mid-file `use` lines in `compiler/src/parser/expr.rs`**

Delete:
- Lines 127, 128 (inside `parse_if_expr`)
- Line 371 (mid-file `use crate::ast::{Pattern, PatternKind};`)
- Line 484 (inside `parse_match_expr`)
- Lines 529, 530 (inside `parse_match_arm_body`)

After deletion, line numbers will shift; re-locate by content if needed.

- [ ] **Step 5: Verify**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 183 tests pass, clippy clean, fmt clean.

- [ ] **Step 6: Commit**

```sh
git add -A
git commit -m "Refactor: aggregate parser use statements to top-of-file

Move function-local and mid-file use statements in parser/stmt.rs and
parser/expr.rs to the top-of-file use block. No behavior change."
```

---

## Task 2: Replace `peek().clone()` idiom with span+kind pattern

**Why:** `let tok = p.peek().clone(); match &tok.kind { ... }` is a borrow-checker workaround that obscures intent. Since `Parser::peek_kind()` returns `&'t TokenKind` (lifetime tied to the token slice, not `&self`), we can match directly on `peek_kind()` without cloning the entire `Token`. We clone only the inner String when we need to keep it across the `advance` call.

**Pattern to apply at every site:**

Old:
```rust
let tok = p.peek().clone();
match &tok.kind {
    TokenKind::Ident(n) => {
        p.advance();
        // use n, tok.span
    }
    _ => Err(CompileError::parse(tok.span, "expected ...")),
}
```

New:
```rust
match p.peek_kind() {
    TokenKind::Ident(n) => {
        let n = n.clone();
        let span = p.advance().span;
        // use n, span
    }
    _ => {
        let span = p.current_span();
        return Err(CompileError::parse(span, "expected ..."));
    }
}
```

**Where the original code captured `tok.span` for error formatting with `format!(... {:?} ..., tok.kind)`**, change to use `p.peek().span` and `p.peek_kind()` (or capture both before generating the format) — borrow-checker permits because format! call evaluates before any subsequent `&mut p` use:

```rust
_ => {
    let span = p.current_span();
    return Err(CompileError::parse(
        span,
        format!("expected ..., found {:?}", p.peek_kind()),
    ));
}
```

**19 sites to update** (do them in this order; commit once at end):

`compiler/src/parser/types.rs`:
- Line 36: `let ident_tok = p.peek().clone();` (in `parse_type`)
- Line 53: `let end_tok = p.peek().clone();` (in `parse_type`) — used only for `end_tok.span`. Replace with `let end_span = p.current_span();` before `expect`.
- Line 87: `let tok = p.peek().clone();` (in `parse_type_param_name`)
- Line 156: `let tok = p.peek().clone();` (in `parse_unit_factor`)
- Line 172: `let exp_tok = p.peek().clone();` (in `parse_unit_factor`)

`compiler/src/parser/stmt.rs`:
- Line 57: `let name_tok = p.peek().clone();` (in `parse_struct_def` for struct name)
- Line 67: `let fname_tok = p.peek().clone();` (in `parse_struct_def` for field name)
- Line 109: `let name_tok = p.peek().clone();` (in `parse_enum_def`)
- Line 148: `let name_tok = p.peek().clone();` (in `parse_variant_decl`)
- Line 202: `let name_tok = p.peek().clone();`
- Line 285: `let name_tok = p.peek().clone();`
- Line 319: `let second_tok = p.peek().clone();`
- Line 367: `let name_tok = p.peek().clone();`
- Line 406: `let name_tok = p.peek().clone();`

`compiler/src/parser/expr.rs`:
- Line 14: `let tok = p.peek().clone();` (in `parse_primary`)
- Line 220: `let field_tok = p.peek().clone();`
- Line 273: `let name_tok = p.peek().clone();` (in `parse_struct_lit_field`)
- Line 377: `let tok = p.peek().clone();` (in `parse_pattern`)
- Line 431: `let next = p.peek().clone();`

### Steps

- [ ] **Step 1: Apply the pattern site-by-site to all 19 sites listed above**

For each site:
1. Read the surrounding ~15 lines to understand what the cloned token is used for.
2. Replace `let tok = p.peek().clone();` and the subsequent `match &tok.kind { ... }` with the new pattern.
3. If `tok.span` is used post-`advance`, capture it via `let span = p.advance().span;` (the advance returns the consumed token by reference; `.span` is `Copy`).
4. If `tok.kind` is used in an error format string and the error path doesn't advance, use `p.peek_kind()` directly in `format!`.
5. If only `tok.span` is needed for an error and `tok.kind` is dead, simplify: `let span = p.current_span(); return Err(CompileError::parse(span, "..."));`.

After each file is fully migrated, run `cargo build` to catch type errors early. Don't commit yet.

- [ ] **Step 2: Verify**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 183 tests pass, clippy clean, fmt clean.

- [ ] **Step 3: Commit**

```sh
git add -A
git commit -m "Refactor: replace peek().clone() with peek_kind() + span idiom

Token cloning at peek sites was a borrow-checker workaround. Since
peek_kind() returns a reference with lifetime tied to the token slice
(not &self), we can match directly on it and clone only the inner
String when needed. 19 sites simplified across types.rs, stmt.rs,
expr.rs. No behavior change."
```

---

## Task 3: Introduce `parse_comma_list` helper

**Why:** 11 sites implement the same pattern: optional newlines around comma-separated items inside a closing delimiter, with optional trailing comma. After Stage 2's empty-list-rejection commits, three of these sites also share a "reject empty with custom message" branch. Centralize.

**Files:**
- Modify: `compiler/src/parser/stmt.rs` (add helper + enum near `parse_block_until`)
- Modify: `compiler/src/parser/types.rs` (3 site migrations)
- Modify: `compiler/src/parser/expr.rs` (6 site migrations)
- Modify: `compiler/src/parser/stmt.rs` (2 site migrations)

**Helper definition** (add to `compiler/src/parser/stmt.rs`, near the other block helpers around line 420):

```rust
/// How `parse_comma_list` should treat an empty list (closing token immediately
/// after the opening boundary).
pub(crate) enum EmptyHandling {
    /// Empty list is allowed; return `Vec::new()`.
    Allow,
    /// Empty list is rejected; emit `CompileError::parse` with this message.
    Reject(&'static str),
    /// Don't pre-check; let `parse_one` produce its own error if it fails.
    /// Use when the original code did not have an explicit empty check.
    RequireOne,
}

/// Parse a comma-separated list of items terminated by `close`, with optional
/// newlines around items and an optional trailing comma. Caller is responsible
/// for consuming the opening delimiter and the closing delimiter.
pub(crate) fn parse_comma_list<T, F>(
    p: &mut Parser,
    close: &TokenKind,
    empty: EmptyHandling,
    mut parse_one: F,
) -> Result<Vec<T>, CompileError>
where
    F: FnMut(&mut Parser) -> Result<T, CompileError>,
{
    p.consume_newlines();
    if p.at(close) {
        return match empty {
            EmptyHandling::Allow => Ok(Vec::new()),
            EmptyHandling::Reject(msg) => Err(CompileError::parse(p.current_span(), msg)),
            EmptyHandling::RequireOne => {
                // Fall through to parse_one which will produce its own error.
                let mut items = Vec::new();
                items.push(parse_one(p)?);
                Ok(items)
            }
        };
    }
    let mut items = vec![parse_one(p)?];
    p.consume_newlines();
    while p.eat(&TokenKind::Comma) {
        p.consume_newlines();
        if p.at(close) {
            break;
        }
        items.push(parse_one(p)?);
        p.consume_newlines();
    }
    Ok(items)
}
```

Note: the `RequireOne` branch's "fall through to parse_one" runs when the close is already at position; it intentionally calls `parse_one` to surface the natural error from that function (e.g. "expected type name, found Gt"). This preserves the pre-refactor behavior at types.rs:50 (generic type-arg list).

**Per-site migration table:**

| Site (file:line) | Caller | `close` | `empty` | `parse_one` |
|---|---|---|---|---|
| types.rs:21 | `parse_type` (Fn) | `&TokenKind::RParen` | `Allow` | `parse_type` |
| types.rs:50 | `parse_type` (generic args) | `&TokenKind::Gt` | `RequireOne` | `parse_type_arg` |
| types.rs:79 | `parse_type_param_list` | `&TokenKind::Gt` | `Reject("empty type parameter list `<>` is not allowed; omit the brackets entirely")` | `parse_type_param_name` |
| stmt.rs:167 | `parse_variant_decl` | `&TokenKind::RParen` | `Reject("empty payload list `()` is not allowed; omit the parentheses for a no-payload variant")` | `crate::parser::types::parse_type` |
| stmt.rs:381 | `parse_param_list` | `&TokenKind::RParen` | `Allow` | `parse_param` |
| expr.rs:89 | `parse_postfix` (call) | `&TokenKind::RParen` | `Allow` | `parse_expr` |
| expr.rs:109 | `parse_vec_or_mat_lit` | `&TokenKind::RBracket` | `Allow` | `parse_expr` |
| expr.rs:190 | `parse_postfix` (index) | `&TokenKind::RBracket` | `RequireOne` | `parse_expr` |
| expr.rs:249 | `parse_postfix` (struct lit) | `&TokenKind::RBrace` | `Allow` | `parse_struct_lit_field` |
| expr.rs:291 | `parse_row` (matrix row) | `&TokenKind::RBracket` | `Allow` | `parse_expr` |
| expr.rs:400 | `parse_pattern` (variant payload) | `&TokenKind::RParen` | `Reject("empty payload list `()` is not allowed; use the variant name without parentheses")` | `parse_pattern` |

**Migration template** (apply at each site):

Old (representative):
```rust
if p.eat(&TokenKind::LParen) {
    p.consume_newlines();
    if p.at(&TokenKind::RParen) {
        return Err(CompileError::parse(
            p.current_span(),
            "empty payload list `()` is not allowed; ...",
        ));
    }
    let mut payload = Vec::new();
    payload.push(parse_type(p)?);
    p.consume_newlines();
    while p.eat(&TokenKind::Comma) {
        p.consume_newlines();
        if p.at(&TokenKind::RParen) { break; }
        payload.push(parse_type(p)?);
        p.consume_newlines();
    }
    end_span = p.current_span();
    p.expect(&TokenKind::RParen, "')'")?;
}
```

New:
```rust
if p.eat(&TokenKind::LParen) {
    let payload = parse_comma_list(
        p,
        &TokenKind::RParen,
        EmptyHandling::Reject("empty payload list `()` is not allowed; ..."),
        crate::parser::types::parse_type,
    )?;
    end_span = p.current_span();
    p.expect(&TokenKind::RParen, "')'")?;
    payload  // assign to outer variable as appropriate
}
```

For caller-side `parse_one` that is a free function pointer (e.g. `parse_type`), pass it directly. For closures over `parse_pattern` (recursive call) or methods needing path qualification, use `|p| parse_pattern(p)` or `crate::parser::types::parse_type`.

### Steps

- [ ] **Step 1: Add `EmptyHandling` enum and `parse_comma_list` helper to `compiler/src/parser/stmt.rs`**

Insert the helper code (shown above) just before `parse_block_until` (currently around line 423). Both `EmptyHandling` and `parse_comma_list` should be `pub(crate)` so they are reachable from `parser/expr.rs` and `parser/types.rs`.

- [ ] **Step 2: Add `use` lines for `EmptyHandling` and `parse_comma_list` to `parser/types.rs` and `parser/expr.rs`**

Update top-of-file `use crate::parser::stmt::...` lines to include `EmptyHandling` and `parse_comma_list` (alongside `TokenKindKind`, `parse_block_until`, `parse_stmt` already there from Task 1).

- [ ] **Step 3: Migrate all 11 call sites per the table above**

For each site:
1. Read the existing comma-loop block.
2. Replace it with a single `parse_comma_list(...)` call producing the items vector.
3. The opening delimiter consumption (`p.eat(&TokenKind::LParen)` etc.) and the closing delimiter consumption (`p.expect(...)`) stay at the call site — the helper does NOT touch them.
4. Preserve any `end_span = p.current_span();` capture immediately before `p.expect` (used by callers that compute the construct's span).

After each file (types.rs, then stmt.rs, then expr.rs) is migrated, run `cargo build`. Don't commit yet.

- [ ] **Step 4: Verify**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 183 tests pass, clippy clean, fmt clean. Particular tests to watch:
- `enum_def_empty_type_params_rejected`, `enum_def_empty_payload_rejected`, `pattern_variant_empty_payload_rejected` — confirm error messages preserved exactly.

- [ ] **Step 5: Commit**

```sh
git add -A
git commit -m "Refactor: introduce parse_comma_list helper

Eleven comma-separated-list parse sites across types.rs, stmt.rs, and
expr.rs implemented the same loop with subtle variants in empty
handling and trailing-comma support. Centralize via parse_comma_list
helper with an EmptyHandling enum (Allow / Reject(msg) / RequireOne).
Empty-rejection error messages are preserved verbatim. No behavior
change."
```

---

## Task 4: Introduce `parse_block_body` helper, fix Stage 1 span overshoot

**Why:** `parse_block_until` (used by `if`/`while`/`for`/`function`) and `parse_match_arm_body` share the same loop algorithm. They differ only in (a) terminator predicate, (b) error-message context, and (c) how the body's end span is computed. The Stage 2 M1 fix corrected the end-span computation in `parse_match_arm_body` only; `parse_block_until` still has the same overshoot bug. Unifying via a generic `parse_block_body` helper fixes both at once and prevents future drift.

**Behavior change:** After this task, `Block.span` for Stage 1 constructs (function bodies, if/elseif/else branches, while body, for body) will end at the last consumed statement's span (rather than at the closing keyword). This is a bug fix — the previous span included trailing newlines and the terminator keyword.

**Files:**
- Modify: `compiler/src/parser/stmt.rs` (add `parse_block_body`; rewrite `parse_block_until` as a wrapper; remove now-unused `require_stmt_terminator`)
- Modify: `compiler/src/parser/expr.rs` (rewrite `parse_match_arm_body` as a wrapper)

**Helper definition** (place in `compiler/src/parser/stmt.rs`, replacing the existing `parse_block_until` function and `require_stmt_terminator` helper around lines 423-463):

```rust
/// Parse a block body: leading newlines, then statements separated by
/// Newline / Eof / a caller-supplied terminator predicate, until the
/// terminator is at the parser's position.
///
/// The returned Block's span ends at the last consumed statement's span,
/// not at the terminator. Callers compute their own outer span (incl.
/// the terminating keyword) at the call site if needed.
pub(crate) fn parse_block_body<F>(
    p: &mut Parser,
    is_terminator: F,
    eof_msg: &'static str,
    after_stmt_label: &'static str,
) -> Result<Block, CompileError>
where
    F: Fn(&Parser) -> bool,
{
    let start = p.current_span();
    p.consume_newlines();
    let mut stmts = Vec::new();
    let mut end_span = start;
    while !is_terminator(p) {
        if matches!(p.peek_kind(), TokenKind::Eof) {
            return Err(CompileError::parse(p.current_span(), eof_msg));
        }
        let stmt = parse_stmt(p)?;
        end_span = stmt.span;
        stmts.push(stmt);
        if !matches!(p.peek_kind(), TokenKind::Newline | TokenKind::Eof) && !is_terminator(p) {
            return Err(CompileError::parse(
                p.current_span(),
                format!("{}, found {:?}", after_stmt_label, p.peek_kind()),
            ));
        }
        p.consume_newlines();
    }
    Ok(Block {
        stmts,
        span: Span::merge(start, end_span),
    })
}

/// Parse a block that ends at any of the supplied block terminators
/// (End, Else, Elseif). Used by Stage 1 control-flow forms.
pub(crate) fn parse_block_until(
    p: &mut Parser,
    terminators: &[TokenKindKind],
) -> Result<Block, CompileError> {
    parse_block_body(
        p,
        |p| is_at_terminator(p, terminators),
        "unexpected end of input inside block",
        "expected newline after statement",
    )
}
```

**`parse_match_arm_body` rewrite** (in `compiler/src/parser/expr.rs`, replacing the existing function around lines 528-562):

```rust
fn parse_match_arm_body(p: &mut Parser) -> Result<Block, CompileError> {
    crate::parser::stmt::parse_block_body(
        p,
        |p| matches!(p.peek_kind(), TokenKind::End | TokenKind::Case),
        "unexpected end of input inside match arm body",
        "expected newline after match arm statement",
    )
}
```

**`require_stmt_terminator` removal:** the existing function at `parser/stmt.rs:452-463` is now dead code (its logic is inlined into `parse_block_body`). Verify no other caller via `grep require_stmt_terminator` and delete it.

`is_at_terminator` (around line 473) is still used by the new `parse_block_until` wrapper — keep it.

### Steps

- [ ] **Step 1: Confirm no other callers of `require_stmt_terminator`**

```sh
cd compiler && grep -n require_stmt_terminator src/parser/*.rs
```

Expected: only the definition (at stmt.rs:452 area) and the call in `parse_block_until`. After this task, both go away.

- [ ] **Step 2: Replace `parse_block_until` and remove `require_stmt_terminator` in `compiler/src/parser/stmt.rs`**

Replace the existing `parse_block_until` (lines 423-448), the doc comment above it, and the `require_stmt_terminator` function (lines 452-463) with the new code shown above (`parse_block_body` + new wrapper `parse_block_until`).

`is_at_terminator` (line 473) and `TokenKindKind` enum stay unchanged.

- [ ] **Step 3: Replace `parse_match_arm_body` in `compiler/src/parser/expr.rs`**

Replace the existing `parse_match_arm_body` function (lines 528-562) with the new 7-line wrapper shown above.

- [ ] **Step 4: Verify build**

```sh
cd compiler && cargo build 2>&1 | tail -10
```

Expected: build succeeds. No new warnings.

- [ ] **Step 5: Run tests, expect some span-related test breakage and fix**

```sh
cd compiler && cargo test 2>&1 | tail -30
```

Most existing tests pass. Some may fail because they assert on `Block.span` ranges that previously included the closing keyword. For each failing test:
1. Read the assertion.
2. If it asserts on a span end position that included the closing keyword (`end`/`else`/`elseif`), update the assertion to the new (correct) end position — the last consumed statement's end.
3. If it asserts on body content (statements vector, expressions inside), it should still pass.

Tests likely affected (to check first):
- Any test that does `let body_text = &src[block.span.start..block.span.end]; assert!(body_text.contains("end"))`.
- Tests in `parser/stmt.rs::tests` and `parser/expr.rs::tests` referencing function/if/while/for body spans.

Update tests to reflect the new (correct) span behavior. The goal is to eliminate assertions of the buggy span — not to preserve the bug.

- [ ] **Step 6: Verify**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 183 tests pass (or 183 minus any genuinely-removed assertions, plus zero new failures), clippy clean, fmt clean.

- [ ] **Step 7: Commit**

```sh
git add -A
git commit -m "Refactor: unify block-body parsing via parse_block_body helper

parse_block_until (Stage 1 control flow) and parse_match_arm_body
(Stage 2 match arms) shared the same loop algorithm with three deltas:
terminator predicate, error-message context, and end-span computation.
The Stage 2 M1 fix corrected the end-span overshoot in
parse_match_arm_body; parse_block_until still had the same bug. This
commit extracts the shared algorithm into parse_block_body, with
caller-supplied terminator and error labels. The Stage 1 span overshoot
is fixed as a side effect: block spans now end at the last consumed
statement, not at the closing keyword. require_stmt_terminator is
removed (its logic is inlined into the helper)."
```

---

## Task 5: Pin Stage 1 block span behavior

**Why:** Task 4 changed the span of all Stage 1 block constructs as a bug fix. Add pin tests so the corrected behavior is anchored against future regressions and so the contract is visible.

**Files:**
- Modify: `compiler/src/parser/stmt.rs` (add tests inside `mod tests`)

**Tests to add** (insert at end of `mod tests` in `parser/stmt.rs`, before the closing `}`):

```rust
#[test]
fn function_body_span_does_not_include_end() {
    let src = "function f(): Int\n  return 1\nend\n";
    let toks = tokenize(src).unwrap();
    let mut p = Parser::new(&toks);
    let prog = parse_program(&mut p).unwrap();
    let crate::ast::ItemKind::Function(ref func) = prog.items[0].kind else {
        panic!("expected Function");
    };
    let body_text = &src[func.body.span.start..func.body.span.end];
    assert!(
        !body_text.contains("end"),
        "function body span overshoots into 'end': {body_text:?}"
    );
}

#[test]
fn while_body_span_does_not_include_end() {
    let src = "function f(): Int\n  while true\n    return 1\n  end\n  return 0\nend\n";
    let toks = tokenize(src).unwrap();
    let mut p = Parser::new(&toks);
    let prog = parse_program(&mut p).unwrap();
    let crate::ast::ItemKind::Function(ref func) = prog.items[0].kind else {
        panic!("expected Function");
    };
    // Find the While statement inside the function body
    let while_stmt = func
        .body
        .stmts
        .iter()
        .find(|s| matches!(s.kind, crate::ast::StmtKind::While(_, _)))
        .expect("expected While stmt");
    let crate::ast::StmtKind::While(_, ref body) = while_stmt.kind else { unreachable!() };
    let body_text = &src[body.span.start..body.span.end];
    assert!(
        !body_text.contains("end"),
        "while body span overshoots into 'end': {body_text:?}"
    );
}

#[test]
fn if_then_branch_span_does_not_include_else() {
    let src = "function f(): Int\n  if true\n    return 1\n  else\n    return 2\n  end\nend\n";
    let toks = tokenize(src).unwrap();
    let mut p = Parser::new(&toks);
    let prog = parse_program(&mut p).unwrap();
    let crate::ast::ItemKind::Function(ref func) = prog.items[0].kind else {
        panic!("expected Function");
    };
    // Find the If expression in the body's first stmt (should be ExprStmt(If(...)))
    let first_stmt = &func.body.stmts[0];
    let crate::ast::StmtKind::Expr(ref e) = first_stmt.kind else {
        panic!("expected ExprStmt");
    };
    let crate::ast::ExprKind::If(ref ifx) = e.kind else {
        panic!("expected If expression");
    };
    let then_text = &src[ifx.then_branch.span.start..ifx.then_branch.span.end];
    assert!(
        !then_text.contains("else"),
        "if then-branch span overshoots into 'else': {then_text:?}"
    );
    assert!(
        !then_text.contains("end"),
        "if then-branch span overshoots into 'end': {then_text:?}"
    );
}
```

**Note:** Adjust struct/enum field paths if the AST exposes them differently. The `IfExpr` struct (from `crate::ast::IfExpr`) is expected to have `then_branch: Block`, `else_branch: Option<Block>` (verify by reading `compiler/src/ast/expr.rs`).

### Steps

- [ ] **Step 1: Add the three pin tests to `compiler/src/parser/stmt.rs::tests`**

Insert the test code shown above before the closing `}` of the `mod tests` block. Adjust AST field accessors if needed (read `compiler/src/ast/expr.rs::IfExpr` and `compiler/src/ast/stmt.rs::FunctionDef` first to confirm field names).

- [ ] **Step 2: Run new tests to verify they pass under the post-Task-4 behavior**

```sh
cd compiler && cargo test --quiet function_body_span while_body_span if_then_branch_span 2>&1 | tail -10
```

Expected: 3 new tests pass.

- [ ] **Step 3: Verify**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 186 tests pass, clippy clean, fmt clean.

- [ ] **Step 4: Commit**

```sh
git add -A
git commit -m "Test: pin Stage 1 block span (function/while/if branches)

Anchors the Task-4 bug fix: Block.span for Stage 1 constructs ends at
the last consumed statement, not at the closing keyword. Without these
tests the corrected behavior could regress unnoticed."
```

---

## Final verification (after all 5 tasks)

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
cargo run --quiet -- ../samples/option_match.dy
```

Expected: 186 tests pass, clippy clean, fmt clean, smoke test "parsed 5 item(s)".

## Push and PR

```sh
git push -u origin refactor-cross-cutting
gh pr create --base main --title "Refactor: cross-cutting parser cleanup (PR-A)" --body "..."
```

PR description should explain the 5 commits, note the Stage 1 span behavior change as a bug fix (with link to Task 4 commit), and confirm no surface-language behavior change.

## Out of scope (PR-B)

- AST-wide span pin tests across all Expr/Stmt/Type/Pattern/Item variants — separate PR.
- `parse_pattern` recursion-depth limit (Stage 3 timing).
