# TypeScript Review Hints

Language-specific idioms and pitfalls to inform reviewer hunts. Use these as starting points; do not treat them as an exhaustive checklist.

## Robustness

- Type narrowing gaps — `if (x)` does not narrow `x: string | undefined` to `string` if `x` could be `""` and the empty case matters
- `any` leakage from third-party types or `JSON.parse` — every `any` is a hole in type safety
- Unhandled promise rejection — `async` function called without `await` and without `.catch()` propagates uncaught
- Discriminated union exhaustiveness — `switch` over a tagged union without `default: assertNever(x)` silently misses new variants
- Optional chaining vs falsy: `obj?.field` returns `undefined` for `obj === null`, but `obj?.field ?? default` differs from `obj?.field || default` on falsy values
- `Object.freeze` is shallow; nested mutation slips through
- Array methods that mutate (`sort`, `reverse`, `splice`) vs non-mutating — easy to confuse
- Floating-point arithmetic — `0.1 + 0.2 !== 0.3`; relevant for money, time, coords

## API

- Public function signature evolution — adding required params is breaking; default-valued params or function overloads are safer
- Naming: `get*` for sync, `fetch*` / `load*` for async, `is*` / `has*` for boolean
- `Promise<Result<T, E>>` vs throwing — pick one error model and apply consistently within a module
- Structural typing pitfall — `interface User { id: string }` accepts any object with `id`; nominal-style branding (`type UserId = string & { __brand: 'UserId' }`) prevents accidental mixing
- Public types should be explicit, not inferred — `export const foo = ...` infers a possibly-too-narrow type
- React props / hooks: stable references matter for memoization; passing inline objects defeats `useMemo`
- Generic constraint over-engineering — overly clever generics make APIs unteachable

## Performance

- Large object spread (`{...big, x: 1}`) — full shallow copy; expensive in hot paths
- `Map` vs `Object` — `Map` is faster for frequent additions/deletions, `Object` for small fixed shapes
- Deep cloning: `structuredClone` for built-in types, `JSON.parse(JSON.stringify(...))` loses functions/Dates, libraries (lodash cloneDeep) cost
- Async waterfalls — sequential `await` where `Promise.all` would parallelize
- Bundle size: importing entire libraries vs named imports (`import { fn } from 'lib'` may or may not tree-shake)
- React re-renders: missing `useMemo` / `useCallback` on dependencies of memoized children
- Array methods chained 5+ times — single `for` may be clearer and faster

## Tests

- `jest.mock` / `vi.mock` resets — confirm `beforeEach` clears mocks to prevent cross-test pollution
- Async tests: `await expect(promise).resolves.toEqual(...)` not `expect(await promise).toEqual(...)` for clearer errors
- Snapshot stability — snapshot of a Date / random / order-dependent structure is fragile; normalize before snapshotting
- Test isolation: shared module state, `global.X = ...`, module-level caches survive across tests
- Mock dynamic imports carefully — `jest.doMock` must execute before the import being mocked
- Type-level tests (`tsd`, `expectTypeOf`) for generic APIs — runtime tests miss type regressions
- E2E vs integration vs unit: integration tests over a real DB / real HTTP catch what mocked unit tests miss
