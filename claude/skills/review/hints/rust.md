# Rust Review Hints

Language-specific idioms and pitfalls to inform reviewer hunts. Use these as starting points; do not treat them as an exhaustive checklist. References: *Effective Rust* (David Drysdale), [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/), rust-lang/rust conventions.

## Robustness

- `unwrap()` / `expect()` outside tests — every call is a potential panic; ask whether the invariant is enforced upstream. Prefer `?` propagation or explicit handling
- Distinguish panic intent: `panic!` for caller-induced bugs / impossible states the caller could prevent; `unreachable!` for invariant violations within the function; `unimplemented!` (`todo!`) only as scaffolding — never in shipping code
- Array / slice indexing `[idx]` — prefer `.get()` unless bounds are statically guaranteed; assert bounds with `debug_assert!` when they should be invariant
- Arithmetic on untrusted inputs without `checked_*` / `wrapping_*` / `saturating_*` — debug panics, release wraps; especially `usize` arithmetic in length / capacity calculations
- `match` exhaustiveness when enums are likely to grow — combine `#[non_exhaustive]` on the producer with explicit `_ =>` arms (with intent comment) on the consumer
- Arm ordering when subtypes / supertypes interact (e.g., type checker arms, trait object downcasts)
- `Result` / `Option` discarded with `let _ =` / `.ok()` / `.unwrap_or_default()` — confirm the discard is intentional and documented
- Missing `#[must_use]` on `Result`-returning or builder-returning functions — callers can silently drop important values
- `&mut T` is held longer than needed — exclusive borrow serializes all other access; minimize the lifetime, especially across function calls that don't need mutation
- `RefCell` / `RefMut` runtime borrow panics — re-entrant access, callback invoking back into the borrowed value, two `borrow_mut()` regions overlapping; consider whether the design needs interior mutability at all
- `RefCell` used in multi-threaded context — `RefCell` is `!Sync`; concurrent use is a compile error, but mistakes appear when refactoring a single-threaded module into a multi-threaded one
- `Drop` order pitfalls — fields drop in declaration order; `MutexGuard` released earlier than expected can cause races
- `debug_assert!` for documented invariants — runtime check in debug, free in release
- Recursion depth on user inputs — risk of stack overflow without explicit bound; consider iterative or arena-based alternatives
- `Send` / `Sync` boundaries — adding interior mutability (`Cell`, `RefCell`) to a previously-`Sync` type silently breaks downstream

## API

- Public function signature changes break downstream — adding params, narrowing types, returning `Option` where `Result` was expected
- Naming conventions: `try_*` for fallible variants, `*_mut` for mutable access, `into_*` / `as_*` / `to_*` for conversions (move / borrow / clone respectively per API Guidelines)
- `&mut T` is **exclusive** (unique) access, not just "mutable" — taking `&mut T` excludes all other reads and writes. Choose by access exclusivity, not by intent to mutate. APIs taking `&mut self` for logically read-only operations over-restrict callers and prevent concurrent shared use
- `trait` is a **capability** (能力), not type identity. "Cat is-a Animal" inheritance modeling via traits is an anti-pattern; instead use composition and treat traits as "Cat also-implements Draw / Serialize / Debug". Marker traits (`Send`, `Sync`, `Copy`) and capability traits (`Read`, `Iterator`) follow this principle
- Newtype pattern (`pub struct UserId(String)`) over raw `String` / `u64` for semantically-distinct values — prevents accidental cross-domain mixing at compile time
- Conversion traits: `From<T> for U` (infallible) and `TryFrom<T> for U` (fallible); `Into` / `TryInto` are derived. Implement `From`, not `Into`
- Error types: `enum` with `#[non_exhaustive]` and `thiserror` / manual `Display`; avoid leaking internal types in public errors; implement `std::error::Error::source()` for chaining
- `#[must_use]` on builders, `Result`-returning functions, and pure computations whose result must not be silently dropped
- Builder pattern: prefer type-state (`Builder<HasName, HasEmail>`) for compile-time enforcement of required fields over runtime panics
- Accept `&str` / `&[T]` / `&Path` / `&impl AsRef<T>` for read-only inputs; take owned `String` / `Vec` / `PathBuf` only when the function will store them
- Exposing `RefCell` / `Cell` / `Mutex` in public types — consumers inherit the runtime panic risk (`borrow_mut`) or lock-ordering obligations; document the invariant or hide the interior mutability behind a method API
- Lifetimes in public APIs become caller obligations — minimize through `'static` bounds or elision where possible
- `pub` vs `pub(crate)` vs private — over-exposure widens the API surface; default to most-restricted
- Sealed traits (`pub trait Sealed: sealed::Sealed`) to prevent external implementations when invariants depend on internal logic
- Generic constraints on struct definitions tax every user; prefer constraints on `impl` blocks unless the type genuinely cannot exist without them

## Performance

- `clone()` / `to_owned()` / `to_string()` in hot paths — verify reference / borrow alternatives; use `Cow<'_, str>` for sometimes-owned strings
- `&[T]` vs `Vec<T>` in parameters: slice for read-only access, `Vec` only when ownership transfer is needed
- `&str` vs `String` in parameters: same principle; `impl AsRef<str>` for flexibility but watch monomorphization cost
- `Vec::new()` followed by repeated `push()` in known-size loops — use `Vec::with_capacity(n)`
- `.collect::<Vec<_>>()` when the next operation is another iterator chain — keep the iterator lazy
- Iterator short-circuit: `try_fold` / `try_for_each` / `find` / `any` / `all` over manual loops; they stop early on first match or error
- `String` formatting in hot paths (`format!`, `to_string`) — prefer `write!` into a reusable buffer or `itoa` / `ryu` for numeric formatting
- `Box<dyn Trait>` carries a vtable indirection and prevents inlining; static dispatch (`impl Trait` / generic) is faster but increases binary size
- `Arc` / `Rc` cloning is cheap but not free; avoid in tight loops; consider `Arc::clone(&x)` for explicitness
- Drop cost — types with non-trivial `Drop` (e.g., `String`, `Vec<String>`) called in tight loops; consider arena allocation
- `HashMap` for small N — `Vec<(K, V)>` linear search may win below ~16 entries; `FxHashMap` / `AHashMap` for non-cryptographic hashing
- Allocation profiling: `dhat-rs` or `heaptrack` before optimizing — measured bottleneck only

## Async

- `MutexGuard` / `RefMut` / `RwLockGuard` held across `.await` — the future may be cancelled, leaking the lock; worse, this can deadlock when the awaited future tries to acquire the same lock. Use `tokio::sync::Mutex` for cross-`.await` locks, or scope the guard with `drop(guard)` before the `.await`
- `tokio::spawn` requires `Send + 'static` on the future — captures of `Rc`, `RefCell`, or non-`'static` borrows fail to compile. The anti-pattern is wrapping everything in `Arc<Mutex<_>>` to satisfy the constraint; consider whether the work should be spawned at all, or run on a `LocalSet`
- Holding non-`Send` types (`Rc`, `RefCell`, raw pointers) across `.await` makes the future non-`Send` — affects what executor / scheduler can run it
- Async fn return type ties output lifetimes to input lifetimes — `async fn f(&self) -> T` returns `impl Future + '_`; callers cannot outlive `self`. Use `async move` blocks or owned arguments when the future needs to be detached
- `Pin` and self-referential futures — usually invisible thanks to `async fn`, but manual `Future` impls or `Box::pin` wrappers expose `Pin`; do not move pinned futures
- `tokio::select!` branch cancellation — when one branch completes, others are dropped mid-execution. Side effects already initiated (e.g., a write that was acknowledged but not awaited) may not roll back. Ensure each branch is cancellation-safe or use `tokio::pin!` + manual selection
- Bounded vs unbounded channels (`mpsc::channel(n)` vs `mpsc::unbounded_channel`) — unbounded loses backpressure and can OOM under producer overrun. Default to bounded with a measured capacity
- `async fn` in traits (stabilized in 1.75) — `Send` bounds may not propagate automatically; use `trait_variant` crate or document the `Send` story for spawn callers
- Long-running async work without explicit `.await` yield points — starves other tasks on the same executor thread; insert `tokio::task::yield_now()` in CPU-heavy loops or move to `spawn_blocking`
- `spawn_blocking` for CPU-bound or blocking I/O — the async executor's worker threads must not block; running blocking code on them stalls every other task on that thread

## Tests

- `#[should_panic]` — verify the panic message via `expected = "..."` to catch wrong-panic regressions
- Property-based testing (`proptest`, `quickcheck`) for parser / serializer round-trips, arithmetic invariants, ordering laws
- Snapshot testing (`insta`) — diff carefully on review; `cargo insta accept` can silently rubber-stamp regressions
- Doc tests (`/// # Examples ... `) — they run as part of `cargo test`; keep them realistic and current
- `cargo nextest` for faster parallel test execution and better output; preferred over `cargo test` in CI
- `cargo test --release` for performance-sensitive tests where optimization changes behavior (e.g., recursion limits)
- `mockall` mocks — confirm mock behavior matches real implementation invariants (mock divergence is a common bug source)
- Test isolation: shared `static` state, `OnceLock`, environment variables, working directory — use `serial_test` if unavoidable
- `#[cfg(test)]` discipline — test-only helpers should be gated; visible-only-in-tests modules avoid polluting the public API
- Async tests — `#[tokio::test]` (or equivalent); avoid `block_on` inside tests as it interacts oddly with nested runtimes
- Compiler / parser tests: assert on both AST and diagnostic span ranges, not just "no error"; snapshot the span explicitly
