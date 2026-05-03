<!--
This file is a real plan from the Dyne compiler project, illustrating the
standard plan format described in SKILL.md. Each task uses one of two Discipline
forms (TDD or refactor), and regression tests are embedded in the task that introduces
the behavior — not split into a separate "tests-only" task.

Reference only. To regenerate, copy a fresh plan from a real project and
adjust this header.
-->

# Cross-cutting Parser Refactor (PR-A) Implementation Plan

> **Execution:** Use `/execute-plan` to dispatch this plan to agent-teams. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Reduce duplication and noise across the Stage 1+2 parser without changing the language's accepted surface form, except for fixing the Stage 1 `parse_block_until` span overshoot bug as a side effect of helper unification.

**Architecture:** 4 commits (one per task). Items: (1) aggregate `use` statements, (2) replace `peek().clone()` idiom site-by-site, (3) introduce `parse_comma_list` helper + migrate 11 sites + pin newline-acceptance liberalization, (4) introduce `parse_block_body` helper + fix Stage 1 span overshoot bug + pin corrected span behavior.

**Tech Stack:** Rust 2024 edition. Zero runtime deps. Cargo for build/test/lint/fmt.

**Working directory:** `.claude/worktrees/refactor-cross-cutting/compiler/` (run all build/test commands from there).
**Branch:** `refactor-cross-cutting`.
**Baseline before Task 1:** 183 tests passing, clippy `-D warnings` clean, `cargo fmt -- --check` clean — engineer must verify before starting.

**Per-task verification command** (mandatory before each commit):
```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

---

## Task 1: Aggregate function-internal use statements to top-of-file

**Why:** During Stage 2 agent-teams execution, several `use` statements were added inside functions where the imported items were used. They are now scattered. Aggregating to top-of-file improves readability.

**Behavior change:** no (pure refactor)
**Discipline:** refactor — 183 existing tests are the green-bar safety net.

**Files:**
- Modify: `compiler/src/parser/stmt.rs`
- Modify: `compiler/src/parser/expr.rs`

**Internal-`use` statements to remove and re-add at top-of-file:**

`stmt.rs`: function-local `use` lines inside `parse_struct_def`, `parse_enum_def`, `parse_variant_decl` (3 lines).

`expr.rs`: function-local `use` lines inside `parse_if_expr` (2 lines), `parse_match_expr`, `parse_match_arm_body` (2 lines), plus a mid-file `use crate::ast::{Pattern, PatternKind};` near `parse_pattern`.

### Steps

- [ ] **Step 1: Identify and remove all function-internal / mid-file `use` lines in stmt.rs and expr.rs**

For each file, grep for `use` lines that appear after the top-of-file group. Verify each is inside (or near) a function. Remove them.

- [ ] **Step 2: Update top-of-file `use crate::ast::{...}` block to include the previously-local imports**

Read the actual current top-of-file block first (do NOT assume the plan's literal snippet is correct — Tasks 1+ may shift it). Add the new names alphabetically into the existing group.

- [ ] **Step 3: Update top-of-file `use crate::parser::stmt::{...}` block in expr.rs**

Add `parse_block_until`, `parse_stmt`, `TokenKindKind` (the names previously imported function-locally).

- [ ] **Step 4: Verify**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 183 existing tests still pass; clippy clean; fmt clean.

- [ ] **Step 5: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Refactor: aggregate parser use statements to top-of-file

Move function-local and mid-file use statements in parser/stmt.rs and
parser/expr.rs to the top-of-file use block. No behavior change.
EOF
)"
```

---

## Task 2: Replace peek().clone() with peek_kind() + span idiom

**Why:** `let tok = p.peek().clone(); match &tok.kind { ... }` is a borrow-checker workaround that obscures intent. `Parser::peek_kind()` returns `&'t TokenKind` (lifetime tied to the token slice, not `&self`), so we can match directly on `peek_kind()` without cloning the entire `Token`. We clone only the inner String when we need to keep it across `advance`.

**Behavior change:** no (pure refactor)
**Discipline:** refactor — 183 existing tests are the green-bar safety net.

**Files:**
- Modify: `compiler/src/parser/types.rs` (5 sites)
- Modify: `compiler/src/parser/stmt.rs` (9 sites)
- Modify: `compiler/src/parser/expr.rs` (5 sites)

**Pattern (apply at every site):**

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
    other => {
        let span = p.current_span();
        return Err(CompileError::parse(
            span,
            format!("expected ..., found {other:?}"),
        ));
    }
}
```

### Steps

- [ ] **Step 1: Migrate the 5 sites in types.rs (parse_type ident, end-tok, parse_type_param_name, parse_unit_factor atom + exponent)**

Apply the pattern at each site. After this file is fully migrated, run `cargo build` to catch type errors early.

- [ ] **Step 2: Migrate the 9 sites in stmt.rs (parse_struct_def name + field, parse_enum_def, parse_variant_decl, parse_let_stmt, parse_for_stmt × 2, parse_function_def, parse_param)**

Same pattern. Run `cargo build`.

- [ ] **Step 3: Migrate the 5 sites in expr.rs (parse_primary, parse_postfix field-name, parse_struct_lit_field, parse_pattern × 2)**

Same pattern.

- [ ] **Step 4: Verify**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Sanity check: `grep -n "p\.peek\(\)\.clone\(\)" src/parser/*.rs` should return zero matches.

Expected: 183 existing tests still pass; clippy clean; fmt clean.

- [ ] **Step 5: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Refactor: replace peek().clone() with peek_kind() + span idiom

Token cloning at peek sites was a borrow-checker workaround. Since
peek_kind() returns a reference with lifetime tied to the token slice
(not &self), we can match directly on it and clone only the inner
String when needed. 19 sites simplified across types.rs, stmt.rs,
expr.rs. No behavior change.
EOF
)"
```

---

## Task 3: Introduce parse_comma_list helper, migrate 11 sites

**Why:** 11 comma-separated-list sites share the same loop pattern. After Stage 2's empty-list-rejection commits, three of these sites also share a "reject empty with custom message" branch. Centralize via a generic helper. Side effect: newlines around items in `Fn(...)` types and `<T, U>` type-param lists become accepted (consistent with Stage 2 §5.1 multi-line / trailing-comma conventions; existing valid code is unaffected).

**Behavior change:** yes (newline acceptance liberalized in `Fn(...)` types and `<T, U>` type-param lists)
**Discipline:** TDD — pin the new newline-acceptance with failing tests first, then introduce the helper and migrate sites.

**Files:**
- Modify: `compiler/src/parser/stmt.rs` (add helper + enum near `parse_block_until`)
- Modify: `compiler/src/parser/types.rs` (3 site migrations + 1 regression test)
- Modify: `compiler/src/parser/expr.rs` (6 site migrations)
- Modify: `compiler/src/parser/stmt.rs` (2 site migrations + 1 regression test)

**Helper definition** (insert into `compiler/src/parser/stmt.rs` near `parse_block_until`):

```rust
/// How `parse_comma_list` should treat an empty list (closing token immediately
/// after the opening boundary).
pub(crate) enum EmptyHandling {
    /// Empty list is allowed; return `Vec::new()`.
    Allow,
    /// Empty list is rejected with the given message.
    Reject(&'static str),
    /// Don't pre-check; let `parse_one` produce its own error if invoked at the close.
    /// Use when the original code did not have an explicit empty check.
    RequireOne,
}

/// Parse a comma-separated list of items terminated by `close`, with optional
/// newlines around items and an optional trailing comma. Caller is responsible
/// for consuming the opening and closing delimiters.
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
            EmptyHandling::RequireOne => Ok(vec![parse_one(p)?]),
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

**Per-site migration table:**

| Site | Caller | `close` | `empty` | `parse_one` |
|---|---|---|---|---|
| types.rs Fn type | parse_type (Fn) | RParen | Allow | parse_type |
| types.rs generic args | parse_type (Generic) | Gt | RequireOne | parse_type_arg |
| types.rs type-param list | parse_type_param_list | Gt | Reject("empty type parameter list `<>` is not allowed; omit the brackets entirely") | parse_type_param_name |
| stmt.rs variant payload | parse_variant_decl | RParen | Reject("empty payload list `()` is not allowed; omit the parentheses for a no-payload variant") | parse_type |
| stmt.rs param list | parse_param_list | RParen | Allow | parse_param |
| expr.rs matrix outer | parse_vec_or_mat_lit | RBracket | Allow | parse_row |
| expr.rs vec elements | parse_vec_or_mat_lit | RBracket | Allow | parse_expr |
| expr.rs call args | parse_postfix (LParen) | RParen | Allow | parse_expr |
| expr.rs struct lit | parse_postfix (LBrace) | RBrace | Allow | parse_struct_lit_field |
| expr.rs matrix row | parse_row | RBracket | Allow | parse_expr |
| expr.rs variant pattern payload | parse_pattern | RParen | Reject("empty payload list `()` is not allowed; use the variant name without parentheses") | parse_pattern |

### Steps

- [ ] **Step 1: Write failing regression tests for newline-liberalization (red phase)**

Add to `compiler/src/parser/types.rs::tests`:

```rust
#[test]
fn fn_type_params_accept_newlines_around_items() {
    let toks = tokenize("Fn(\n  Scalar,\n  Scalar,\n) -> Scalar").unwrap();
    let mut p = Parser::new(&toks);
    let t = parse_type(&mut p).unwrap();
    if let TypeKind::Function(params, _) = t.kind {
        assert_eq!(params.len(), 2);
    } else {
        panic!("expected Function");
    }
}
```

Add to `compiler/src/parser/stmt.rs::tests`:

```rust
#[test]
fn enum_def_type_params_accept_newlines() {
    let toks = tokenize("enum Foo<\n  T,\n  U,\n>\n  V\nend").unwrap();
    let mut p = Parser::new(&toks);
    let prog = parse_program(&mut p).unwrap();
    let Item::Enum(ref e) = prog.items[0] else {
        panic!("expected Enum");
    };
    assert_eq!(e.type_params, vec!["T".to_string(), "U".to_string()]);
}
```

- [ ] **Step 2: Verify red phase**

Run: `cargo test fn_type_params_accept_newlines enum_def_type_params_accept_newlines`
Expected: both FAIL (current parser rejects newlines in these positions).

- [ ] **Step 3: Add EmptyHandling enum + parse_comma_list helper**

Insert the helper code (above) into `compiler/src/parser/stmt.rs` near `parse_block_until`. Both `pub(crate)`. Update the top-of-file `use crate::parser::stmt::{...}` lines in expr.rs and types.rs to include the new names.

- [ ] **Step 4: Migrate all 11 call sites per the table**

For each site:
1. Read the existing comma-loop block.
2. Replace it with a single `parse_comma_list(...)` call producing the items vector.
3. The opening + closing delimiter consumption (`p.eat(...)`, `p.expect(...)`) stay at the call site.
4. Preserve any `end_span = p.current_span();` capture immediately before `p.expect`.

- [ ] **Step 5: Verify green phase**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
```

Expected: 185 tests pass (183 baseline + 2 new regression tests). All 5 pre-existing diagnostic tests pass with same error wording (the empty-`<>` and empty-`()` rejection messages should be preserved verbatim by the helper's `Reject` branch).

- [ ] **Step 6: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Refactor: introduce parse_comma_list helper

Eleven comma-separated-list parse sites across types.rs, stmt.rs, and
expr.rs implemented the same loop with subtle variants in empty
handling and trailing-comma support. Centralize via parse_comma_list
helper with an EmptyHandling enum (Allow / Reject(msg) / RequireOne).
Empty-rejection error messages are preserved verbatim.

Newlines around items in Fn(...) types and <T, U> type-parameter lists
are now accepted as a side effect of using parse_comma_list, consistent
with Stage 2 §5.1 multi-line / trailing-comma conventions. Regression tests
anchor this benign expansion. Existing valid code is unaffected.
EOF
)"
```

---

## Task 4: Introduce parse_block_body helper, fix Stage 1 span overshoot

**Why:** `parse_block_until` (Stage 1 control flow) and `parse_match_arm_body` (Stage 2 match arms) share the same loop algorithm. They differ only in (a) terminator predicate, (b) error-message context, and (c) end-span computation. Stage 2's M1 fix corrected the end-span computation in `parse_match_arm_body` only; `parse_block_until` still has the same bug. Unifying via `parse_block_body` fixes both at once and prevents future drift as more block-form constructs land.

**Behavior change:** yes (Stage 1 `Block.span` for function/while/for/if branches no longer overshoots into the closing keyword)
**Discipline:** TDD — write regression tests that pin the corrected span first (red), then introduce the helper to satisfy them (green), then refactor the existing wrappers.

**Files:**
- Modify: `compiler/src/parser/stmt.rs` (add `parse_block_body`; rewrite `parse_block_until` as wrapper; remove `require_stmt_terminator`; inline its logic into `parse_program`; add 3 regression tests)
- Modify: `compiler/src/parser/expr.rs` (rewrite `parse_match_arm_body` as wrapper)

**Helper definition** (replaces existing `parse_block_until` body and `require_stmt_terminator` function):

```rust
/// Parse a block body: leading newlines, then statements separated by
/// Newline / Eof / a caller-supplied terminator predicate, until the
/// terminator is at the parser's position.
///
/// The returned Block's span ends at the last consumed statement's span,
/// not at the terminator.
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

`parse_match_arm_body` becomes a 7-line wrapper:

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

### Steps

- [ ] **Step 1: Write failing regression tests for Stage 1 block span (red phase)**

Add to `compiler/src/parser/stmt.rs::tests`:

```rust
#[test]
fn function_body_span_does_not_include_end() {
    let src = "function f(): Int\n  return 1\nend\n";
    let toks = tokenize(src).unwrap();
    let mut p = Parser::new(&toks);
    let prog = parse_program(&mut p).unwrap();
    let Item::Function(ref func) = prog.items[0] else {
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
    let src = "function f(): Int\n  while true do\n    return 1\n  end\n  return 0\nend\n";
    let toks = tokenize(src).unwrap();
    let mut p = Parser::new(&toks);
    let prog = parse_program(&mut p).unwrap();
    let Item::Function(ref func) = prog.items[0] else { panic!("expected Function") };
    let while_stmt = func
        .body
        .stmts
        .iter()
        .find(|s| matches!(s.kind, StmtKind::While(_)))
        .expect("expected While stmt");
    let StmtKind::While(ref ws) = while_stmt.kind else { unreachable!() };
    let body_text = &src[ws.body.span.start..ws.body.span.end];
    assert!(
        !body_text.contains("end"),
        "while body span overshoots into 'end': {body_text:?}"
    );
}

#[test]
fn if_then_branch_span_does_not_include_else() {
    let src = "function f(): Int\n  if true then\n    return 1\n  else\n    return 2\n  end\nend\n";
    let toks = tokenize(src).unwrap();
    let mut p = Parser::new(&toks);
    let prog = parse_program(&mut p).unwrap();
    let Item::Function(ref func) = prog.items[0] else { panic!("expected Function") };
    let StmtKind::Expr(ref e) = func.body.stmts[0].kind else { panic!("expected ExprStmt") };
    let ExprKind::If(ref ifx) = e.kind else { panic!("expected If expression") };
    let then_text = &src[ifx.then_block.span.start..ifx.then_block.span.end];
    assert!(!then_text.contains("else"), "if then-branch overshoots into 'else': {then_text:?}");
    assert!(!then_text.contains("end"),  "if then-branch overshoots into 'end': {then_text:?}");
}
```

Read `compiler/src/ast/{item,expr,stmt}.rs` first to confirm field names (e.g. `then_block` vs `then_branch`, `Item::Function` vs `ItemKind::Function`).

- [ ] **Step 2: Verify red phase**

Run: `cargo test function_body_span while_body_span if_then_branch_span`
Expected: all 3 FAIL — current `parse_block_until` overshoots into the closing keyword.

- [ ] **Step 3: Add `parse_block_body` helper to stmt.rs**

Insert the helper code (above) above `parse_block_until`.

- [ ] **Step 4: Rewrite `parse_block_until` as a thin wrapper**

Replace its body with the wrapper code (above). `is_at_terminator` and `TokenKindKind` enum stay unchanged (still used by the wrapper).

- [ ] **Step 5: Rewrite `parse_match_arm_body` in expr.rs as a thin wrapper**

7 lines, calling `parse_block_body` with the `End | Case` predicate.

- [ ] **Step 6: Inline `require_stmt_terminator` into `parse_program`, delete the standalone helper**

`require_stmt_terminator` had two callers: `parse_block_until` (now via the helper) AND `parse_program` (top-level item terminator). Inline the equivalent Newline/Eof check into `parse_program` (where `terminators` was always `&[]`), then delete the helper function.

- [ ] **Step 7: Verify green phase**

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
cargo run --quiet -- ../samples/option_match.dy
```

Expected: 188 tests pass (185 + 3 new regression tests). All earlier regression tests preserved. Smoke test prints `parsed 5 item(s)`.

- [ ] **Step 8: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Refactor: unify block-body parsing via parse_block_body helper

parse_block_until (Stage 1 control flow) and parse_match_arm_body
(Stage 2 match arms) shared the same loop algorithm with three deltas:
terminator predicate, error-message context, and end-span computation.
The Stage 2 M1 fix corrected the end-span overshoot in
parse_match_arm_body; parse_block_until still had the same bug. This
commit extracts the shared algorithm into parse_block_body, with
caller-supplied terminator and error labels. The Stage 1 span overshoot
is fixed as a side effect: block spans now end at the last consumed
statement, not at the closing keyword. Three regression tests anchor the
corrected behavior.

require_stmt_terminator is removed; its logic is inlined into the
helper for parse_block_until's call site, and inlined directly into
parse_program (the only other caller).
EOF
)"
```

---

## Final verification (after all tasks)

```sh
cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check
cargo run --quiet -- ../samples/option_match.dy
```

Expected: 188 tests pass (183 baseline + 5 new regression tests across Tasks 3, 4); clippy clean; fmt clean; smoke test `parsed 5 item(s)`.

## Push and PR

```sh
git push -u origin refactor-cross-cutting
gh pr create --base main --title "Refactor: cross-cutting parser cleanup (PR-A)" --body "..."
```

PR description should explain the 4 commits, note the Stage 1 span behavior change as a bug fix (with link to Task 4 commit), note the newline-acceptance liberalization in Fn types and type-param lists (with link to Task 3 commit and a pointer to the regression tests), and confirm no other surface-language behavior change.

## Out of scope (PR-B)

- AST-wide span regression tests across all Expr/Stmt/Type/Pattern/Item variants (full-coverage extension).
- `parse_pattern` recursion-depth limit (Stage 3 timing — DoS hardening).

## Alternative Solutions Considered

Choices made during `/design-discussion` that shape this plan, with the alternatives that were rejected:

- **Block-body unification approach (Task 4): full enum extension (`α`)**: Add `Case` to `TokenKindKind` and merge into one `parse_block` function. **Rejected because**: `End/Else/Elseif` are surrounding-control-structure terminators, while `Case` is a next-arm marker — different semantic categories. Forcing them through the same enum + a single function with a string-context arg conflates concepts. β (loop-body extraction with caller-supplied predicate) keeps the call-site context naturally and is more extensible.

- **Block-body unification approach (Task 4): span fix only, no unification (`γ`)**: Just change `let end = p.current_span();` to track `stmt.span` in `parse_block_until`. **Rejected because**: doesn't address the duplication. The next block-helper (Stage 3) would copy whatever shape `parse_block_until` has and inherit any new bug. β extracts the invariant once.

- **Comma-list helper signature (Task 3): generic `is_done` predicate (`B`)**: Pass a `Fn(&Parser) -> bool` for the close detection. **Rejected because**: all 11 actual sites end at a single close token; predicate-form is overkill (premature generalization). Single-token form (`A`, chosen) is simpler and adequate.

- **Comma-list helper signature (Task 3): specialized helpers per close-token (`C`)**: One helper per `)`, `]`, `}`, `>`. **Rejected because**: 4× the helper definitions, duplicated empty-handling logic. The `EmptyHandling` enum + close parameter (`A`, chosen) achieves the same with one helper.

- **Helper placement (Task 3, 4): new `parser/block.rs` module (`b`)**: Move `parse_block_body` and friends into a dedicated module. **Rejected because**: only 1-2 helpers in scope; module split is premature. Reconsider when block-related helpers exceed 4-5.

- **Helper placement (Task 3, 4): method on `Parser` impl (`c`)**: Add `p.parse_block_body(...)` etc. **Rejected because**: `Parser` impl is currently a thin token-cursor abstraction. Adding parser-grammar logic to `impl Parser` would conflate the layers. Free functions in `parser/stmt.rs` keep responsibilities separate.

- **PR scope: single PR for all refactor + span tests + pattern depth limit**: Bundle everything. **Rejected because**: Tier C (AST-wide span regression tests) is purely additive and may surface new bugs that need their own focused PR; mixing with refactor would diffuse review attention. Pattern depth limit is Stage 3 timing. Splitting into PR-A (refactor, this plan) + PR-B (span regression-test coverage) is cleaner.

- **Index-arm migration (Task 3, expr.rs)**: Migrate the `parse_postfix` index arm via `parse_comma_list(.., RequireOne, parse_expr)`. **Rejected because**: the index AST is `Index(Box<Expr>, Box<Expr>)` — single expression. The current code parses one expr; forcing through `parse_comma_list` would either silently accept multi-arg `arr[i, j]` (surface change) or require post-helper length checks (worse error messages). Skipped intentionally; documented in Task 3 commit body.
