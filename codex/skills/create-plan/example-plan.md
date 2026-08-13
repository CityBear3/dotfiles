# Example implementation plan

> **Execution:** Run this plan only after user approval.

## Goal and authorities

Add deterministic parsing for one approved input form through the existing
library boundary.

1. `docs/plans/YYYY-MM-DD-input-form/feature-contract.md`
2. This Implementation Plan and its Task Contract
3. Repository guidance

The Feature Contract owns accepted and preserved behavior. This plan owns the
single-task decomposition and execution evidence.

## Working context

- Architecture: Keep CLI ownership in the binary and parsing ownership in the
  library.
- Working directory: `.`
- Branch: `feature/input-form`
- Baseline: the repository's standard Rust test route passes.

## Fixed decisions and non-goals

- Accept the new form through the existing parser API.
- Preserve existing forms and malformed-input behavior.
- Do not change persistence, permissions, concurrency, or CLI ownership.

## Shared interface contracts

No new cross-task interface is introduced. The existing public parser API is a
Feature Contract boundary and remains unchanged.

## Feature Contract coverage

| Feature obligation | Owning proof |
| --- | --- |
| Parse the complete new representation | Task Contract 1 |
| Preserve existing and malformed forms | Task Contract 1 |
| Prove one real CLI-to-library journey | Integration verification |

## Review context

- **Artifact and purpose:** A Rust library parser plus its existing CLI journey;
  accept one representation without changing ownership.
- **Consumers:** Library callers receive parsed values and the CLI delegates to
  that library API.
- **Material criteria:** Deterministic parsing, unchanged existing syntax and
  error behavior, and one real CLI path.
- **Material failures:** Accepting malformed input, regressing an existing form,
  returning an incomplete value, or bypassing the library boundary.
- **Approved non-problems:** Exhaustive grammar fuzzing and performance tuning are
  outside this feature.
- **Inapplicable assumptions:** Persistence, database identity, permissions, and
  concurrency do not apply unless implementation adds such a path.
- **New-evidence rule:** Revisit a non-problem only with new evidence of a
  concrete reachable regression or approved-contract violation.

## Review policy

- **Mode:** `adaptive`.
- **Rationale:** Public parsing behavior and a CLI journey change, so independent
  task review and API-focused final review are required.
- **Risk surfaces:** Parser compatibility and library-to-CLI integration.
- **Per-task gate:** Independent `spec-reviewer` and `code-quality-reviewer`.
- **Final required reviewers:** `code-reviewer` for general correctness,
  `test-coverage-reviewer` for changed behavior, and
  `adversarial-api-reviewer` for parser API compatibility, then
  `adversarial-integrator`.
- **Conditional reviewers:** Add `adversarial-robustness-reviewer` if error or
  recovery behavior changes.
- **Skipped perspectives:** Skip design alignment because no Design Doc exists;
  scope review because this single Task Contract and its coverage table exhaust
  the approved scope; architecture and performance because ownership and
  measured hot paths do not change; adversarial tests unless fixtures or test
  infrastructure change.
- **Residual risk:** No exhaustive grammar fuzzing or performance measurement.
- **Capacity:** At most four threads including the lead; queue without reducing
  approved scope.
- **Queue order:** Run the two per-task reviewers together when capacity permits,
  then final code and test coverage, API review, triggered robustness review,
  and adversarial integration last.
- **Acceptance:** Keep only artifact-applicable `Must Fix` or `Should Improve`
  findings with an approved requirement, concrete reachable evidence, material
  consequence, and proportionate correction. `Should Improve` requires a
  concrete maintainability consequence or measurable repeated cost. Drop
  preference, speculation, second-order concerns, generic best practice,
  optional polish, inapplicable assumptions, and objections without materially
  new evidence. Treat an unproven architectural mechanism as `Escalate`.

## Task Contract 1: Parse the approved form

### Purpose and expected result

The parser returns the complete approved value for the new form while preserving
all current successful and malformed-input behavior.

### Feature Contract clauses satisfied

- Parse the approved representation deterministically.
- Preserve existing representations and malformed-input behavior.

### Responsibility and ownership boundaries

- Own parsing behavior inside the existing library parser component.
- Do not move parsing into the CLI or change the public parser API.

### Applicable shared interfaces

Consume the unchanged public parser API defined by the Feature Contract.

### Protected constraints

- Return the complete value; do not silently drop fields.
- Preserve existing errors and accepted forms.

### Verification obligations

- Observe the focused test fail because the new form is currently rejected.
- Observe the complete new value after implementation.
- Observe representative existing and malformed forms remain unchanged.

### Dependencies

None.

### Explicit non-goals

- No parser API redesign, persistence change, or grammar-wide optimization.

### Delegated local decisions

The writer chooses private helpers, local types, internal file placement, edit
order, and additional focused tests within the parser responsibility.

### Discipline

TDD: record the expected red failure, make the smallest contract-compliant
change pass, then refactor without changing behavior.

### Contractually significant detail

Use the repository's authoritative Rust test, lint, and non-mutating format-check
routes. The exact private files are deliberately delegated. The commit owns only
parser responsibility and its behavioral tests; the plan fixes its message as
`parser: accept input form`.

## Integration verification

Run the authoritative project checks and exercise one real CLI input through the
existing library boundary. Observe the complete output and unchanged malformed
behavior.

## Post-review iteration

Route a concrete in-scope `Fix` through this Task Contract, fresh verification,
and fresh review. Return any parser API or feature-behavior change to approval.

## Publication

Do not publish without explicit user approval.
