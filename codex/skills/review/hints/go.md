# Go Review Hints

Language-specific idioms and pitfalls to inform reviewer hunts. Use these as starting points; do not treat them as an exhaustive checklist.

## Robustness

- `if err != nil` — confirm every error is handled or explicitly logged; `_ = someCall()` is a code smell
- Nil interface vs nil concrete — `var err error = (*MyErr)(nil); err != nil` is true; type assertions hide this
- Type switch / type assertion without `ok` — `x.(T)` panics on mismatch; prefer `x, ok := y.(T)`
- Goroutine leaks — every `go func()` needs a termination path; `context.Context` cancellation is the canonical mechanism
- Channel deadlock — unbuffered sends without a paired receiver, `select` without `default` in non-blocking contexts
- `defer` evaluates args at the defer site, executes at function return — `defer log.Println(time.Now())` captures the wrong time
- Range loop variable capture — `go func() { use(i) }()` in pre-1.22 Go captures the shared `i`; check Go version
- Slice append aliasing — `b := a[:n]; b = append(b, x)` may mutate `a` if cap allows

## API

- Accept interfaces, return structs — narrows the API surface
- Error wrapping: `fmt.Errorf("context: %w", err)` preserves the chain for `errors.Is` / `errors.As`
- Sentinel errors (`var ErrNotFound = errors.New(...)`) are stable contracts; ad-hoc `errors.New` inside functions is not
- Zero-value usability — exported types should be useful at their zero value where possible (sync.Mutex, bytes.Buffer)
- Avoid `interface{}` (`any`) in public APIs unless truly generic — kills type safety for callers
- Naming: short, lowercased package names; exported names start with capital; `Get` prefix usually unnecessary
- gRPC: field number changes break wire compatibility; reserved tags after removal

## Performance

- `append()` reallocation — pre-size slices with `make([]T, 0, n)` when n is known
- String concatenation in loops — `strings.Builder` is the idiom
- `map[K]V` allocation — pre-size with `make(map[K]V, n)`; iteration order is randomized
- JSON marshaling: reflection-heavy, expensive in hot paths — consider `easyjson` / generated codecs for high QPS
- Goroutine pool vs spawn-per-request — unbounded `go` calls are a memory cost
- `defer` has a small but non-zero cost in hot loops — measure if relevant
- Reflection (`reflect`) is slow; cache `reflect.Type` lookups outside hot paths

## Tests

- Table-driven tests with `t.Run(tc.name, ...)` — each subcase appears as its own line in output
- `t.Parallel()` — confirm tests don't share state; combine with `t.Cleanup` for ordering
- `testify` assertions are convenient but mask which assertion failed in a long test — prefer focused tests
- `httptest.Server` for HTTP client testing — actual transport, not mocks
- Mock interfaces, not concrete types — generated mocks (mockery, gomock) drift; regenerate on interface change
- Race detector (`go test -race`) catches data races at test time; CI should run with it
- Benchmark variance: `b.ResetTimer()`, `b.ReportAllocs()`, multiple runs with `-benchtime`
