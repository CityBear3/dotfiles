# Python Review Hints

Language-specific idioms and pitfalls to inform reviewer hunts. Use these as starting points; do not treat them as an exhaustive checklist.

## Robustness

- Bare `except:` or `except Exception:` — catches too much, including `KeyboardInterrupt`, `SystemExit`; prefer specific types
- Mutable default arguments — `def f(x=[]):` shares the list across calls; canonical bug
- Missing `await` on a coroutine — `asyncio.run(f())` works but `f()` alone returns a coroutine object silently
- Type hints vs runtime — `def f(x: int) -> str:` is not enforced at runtime; consider `pydantic` / `mypy --strict` on boundaries
- File / connection leaks — use `with` statements; `open(...)` without `with` lasts until GC
- Context manager `__exit__` swallowing exceptions — `return True` from `__exit__` silently suppresses
- `dict.get(k)` returns `None` for missing keys — vs `dict[k]` raising `KeyError`; pick the right one
- Iterator exhaustion: `list(map(f, xs))` after `for x in map(f, xs):` — the second iteration is empty

## API

- Positional vs keyword arguments — `def f(a, b, c, d):` is fragile to reorder; `def f(*, a, b)` forces keyword-only
- Implicit `return None` — explicitly `return None` (or `return`) at function end for readability
- Exception hierarchy — define a module-specific base (`MyLibError(Exception)`) so callers can catch broadly without overcatching
- `**kwargs` explosion — losing the API contract; prefer explicit params or `TypedDict`
- `Protocol` (structural typing) for duck typing with type safety
- Pythonic naming — `snake_case` for functions/variables, `PascalCase` for classes, `_leading_underscore` for "private"
- Return type consistency — `-> Optional[X]` vs `-> X | None` (3.10+); pick one per project
- Async function naming — no enforced convention but suffix `_async` or prefix `a` clarifies for mixed APIs

## Performance

- List comprehension > `[].append` in a loop — comprehension is faster and clearer
- `dict.get(k, default)` vs `k in d and d[k]` — single lookup
- String concatenation in loops — `''.join(parts)` over `s += x`
- `asyncio.gather(*tasks)` vs sequential `await` — parallelize independent awaits
- Avoid `+` on large lists in loops — use `extend` or comprehension
- `functools.lru_cache` for pure functions called repeatedly
- NumPy / Pandas vectorization vs Python loops — order of magnitude for numeric workloads
- `pickle` / `json` deserialization in hot paths — consider `orjson` / `msgspec` for high QPS

## Tests

- `pytest` fixtures with `scope="function|class|module|session"` — wrong scope causes cross-test bleed or unnecessary setup cost
- `monkeypatch` over `unittest.mock.patch` for simpler scope-limited replacements
- `MagicMock` vs `Mock` — `MagicMock` supports magic methods; `Mock` does not
- `parametrize` for table-driven tests — `pytest.param(..., id="...")` for readable test names
- Async tests — `pytest-asyncio` with `@pytest.mark.asyncio`; confirm event loop policy
- Fixture teardown — `yield`-style fixtures with cleanup after `yield`
- Snapshot testing (`pytest-snapshot`, `syrupy`) — review diffs carefully; auto-accept hides regressions
- Shared state: `sys.modules`, `os.environ`, working directory — clean up explicitly
