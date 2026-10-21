# Implementation quality within delegated scope

Use this guidance when choosing or revising internal implementation in planned
or lightweight work. Repository guidance, approved design and contracts, and
established compatibility requirements take precedence.

## Ground consequential implementation choices

Read applicable language implementation Skills and use relevant
[language review hints](../../review/hints/) as questions about the actual
implementation. Hints are investigation starting points, not mandatory
conventions, an exhaustive checklist, or a reason to adopt every suggested
pattern. Apply their concrete concerns within the owned scope.

Use the project's toolchain documentation and version-appropriate official
language, standard-library or dependency documentation to resolve consequential
API behavior, idioms and constraints. Verify uncertain or version-sensitive
claims before relying on them. Select documentation relevant to the decision;
do not require a fresh web search or a survey of every source for each edit.
Documentation examples supply options and constraints, not an automatically
optimal implementation for this project.

For meaningful choices, compare reasonable approaches against cohesive
responsibilities, dependency direction, invariants and state ownership,
reachable error paths, readable control flow, and proof of behavior. Prefer
idiomatic code and the smallest sufficient mechanism whose concrete benefit
justifies its complexity and verification cost. Inspect existing patterns as
evidence; justify both following and departing from them using the current
constraints. Avoid speculative abstractions, unrelated cleanup and generic
hardening.

Keep consequential, non-obvious rationale available in the implementation or
Task evidence when it helps assess the result. Do not require a fixed number of
alternatives, a separate comparison document, or a justification for every
helper. The quality of the implementation and its evidence matters more than
the amount of recorded deliberation.

## Preserve authority and development discipline

Choose private helpers, local types and algorithms within delegated ownership.
A change to responsibility ownership, public/shared contracts, compatibility or
failure guarantees returns to the owning workflow's design/scope decision;
neither a hint nor official documentation grants that authority. Do not
implicitly upgrade toolchains or dependencies to adopt a documented example.

Investigate implementation options and choose the test boundary before the
applicable causal RED/production-edit/GREEN slice. Refactor while green and use
the [test contract](../../test-driven-development/SKILL.md#test-contract) to
check responsibility ownership, defect detection and unnecessary coupling.
Follow the selected non-TDD oracle when TDD is not applicable. This guidance
does not authorize an extra improvement pass after Acceptance.
