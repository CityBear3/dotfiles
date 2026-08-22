# Example implementation plan

> **Execution:** Run this plan only after user approval.

## Goal and authorities

Add an approved library input form and expose it through the existing CLI.

1. `docs/plans/YYYY-MM-DD-input-form/feature-contract.md`
2. This approved Implementation Plan and its Task Contracts
3. Repository guidance

The Feature Contract owns accepted behavior. This plan owns decomposition,
topology, task acceptance, integration evidence, and publication boundaries.

## Working context

- Architecture: parsing and rendering remain library responsibilities; the CLI
  composes them.
- Working directory: `.`
- Coordination branch: `feature/input-form`
- Implementation base: the observed current `main` commit.
- Baseline: authoritative Rust checks pass.

## Fixed decisions and non-goals

- Preserve existing accepted forms and malformed-input behavior.
- Keep parser, renderer, and CLI responsibilities separate.
- Do not change persistence, permissions, or error ownership.
- Allow parser and renderer work to begin independently.

## Shared interface contracts

### Parsed input representation

- **Owner:** Task 1.
- **Implementer:** Task 1.
- **Consumers:** Tasks 2 and 3.
- **Contract:** The Feature Contract fixes the public representation and error
  meaning; private parser helpers remain delegated.

## Task dependency DAG

```text
        +-- Task 1: parser --+
Base ---+                    +-- Task 3: CLI journey
        +-- Task 2: output --+
```

Task 1 and Task 2 may implement concurrently because their owned files and state
do not overlap. Task 3 requires current internal acceptance from both.

## PR topology

```text
main
  `-- PR 1: Task 1
       `-- PR 2: Task 2
            `-- PR 3: Task 3
```

Task 2 may produce a candidate from the common base. Before authoritative Task
2 verification and review, restack it onto the accepted Task 1 head. The Git
edge does not create a logical dependency. Task 3 starts only after both parent
results and this stack are current.

## Feature Contract coverage

| Feature obligation | Owning proof |
| --- | --- |
| Parse the approved representation | Task 1 |
| Render the approved value without changing existing output | Task 2 |
| Preserve malformed-input behavior | Tasks 1 and 3, with deliberate overlap |
| Prove the real CLI journey | Task 3 |
| Prove combined compatibility route | Integration-only verification |

## Review context

- **Artifact and purpose:** A Rust parser, renderer, and CLI composition.
- **Consumers:** Library callers and CLI users.
- **Material criteria:** Exact value preservation, compatibility, stable errors,
  and one real process journey.
- **Material failures:** Field loss, accepting malformed input, stale stacked
  evidence, or bypassing the library boundary.
- **Approved non-problems:** Exhaustive fuzzing and performance tuning.
- **Inapplicable assumptions:** Persistence, identity, and permissions.

## Review policy

- **Mode:** `adaptive`.
- **Rationale:** Public parsing behavior and a cross-component CLI journey
  require independent task gates; only the combined compatibility route needs
  integration review.
- **Risk surfaces:** Parser compatibility, exact output, stacked-range
  currentness, and library-to-CLI composition.
- **Per-task gate:** Independent `spec-reviewer` and
  `code-quality-reviewer` against each exact Task PR range.
- **Integration required reviewers:** `test-coverage-reviewer` for the
  integration-only compatibility journey.
- **Integration conditional reviewers:** `adversarial-api-reviewer` only if an
  implementation changes or exposes a public parser seam, followed by
  `adversarial-integrator`.
- **Skipped perspectives:** Skip architecture and performance because approved
  ownership and measured hot paths do not change; skip adversarial robustness
  unless recovery behavior changes; skip scope because each Task gate receives
  exact clauses and ownership.
- **Residual risk:** No exhaustive grammar fuzzing.
- **Capacity and queue:** At most 4 concurrent subagents per session (the
  approved default); schedule ready Task 1 and Task 2 candidates in separate
  worktrees, then their task gates, Task 3, and integration review.
- **Acceptance:** Keep only artifact-applicable findings with an approved
  requirement, reachable evidence, material consequence, and proportionate
  correction. Drop preference, speculation, optional polish, and objections to
  approved decisions without materially new evidence.

## Task Contract 1: Parse the approved form

- **Purpose:** Return the complete approved representation while preserving
  current successful and malformed-input behavior.
- **Coverage:** Parsing and parser-side compatibility clauses.
- **Ownership:** Existing library parser and its focused tests.
- **Shared interface:** Own the parsed input representation.
- **Constraints:** Preserve public errors and accepted forms.
- **Verification:** Record the focused red failure, then observe the complete
  value and representative compatibility behavior.
- **Dependencies:** None.
- **PR relationship:** PR 1 against `main`; final base exists immediately.
- **Concurrency:** May run with Task 2 in a separate checkout.
- **Non-goals:** No CLI, persistence, or API redesign.
- **Delegation:** Private helpers, local types, and focused tests.
- **Discipline:** TDD.
- **Commit intent:** Parser responsibility and tests; writer chooses the
  message.

## Task Contract 2: Render the approved value

- **Purpose:** Render the Feature Contract representation exactly while
  preserving existing output.
- **Coverage:** Rendering and output-compatibility clauses.
- **Ownership:** Existing renderer and its focused tests.
- **Shared interface:** Consume the parsed input representation without changing
  its ownership.
- **Constraints:** No field loss or new formatting contract.
- **Verification:** Record the focused red failure, then observe exact new and
  representative existing output.
- **Dependencies:** None logically.
- **PR relationship:** PR 2 stacked on PR 1. A common-base candidate is allowed;
  final acceptance requires restacking and fresh verification and review.
- **Concurrency:** May implement with Task 1 in a separate checkout.
- **Non-goals:** No parser or CLI edits.
- **Delegation:** Private rendering helpers and test arrangement.
- **Discipline:** TDD.
- **Commit intent:** Renderer responsibility and tests; writer chooses the
  message.

## Task Contract 3: Complete the CLI journey

- **Purpose:** Compose the accepted parser and renderer through the existing CLI
  boundary.
- **Coverage:** CLI journey and end-to-end malformed behavior.
- **Ownership:** CLI composition and real-process tests.
- **Shared interface:** Consume the accepted parser and renderer contracts.
- **Constraints:** Keep parsing and rendering in the library.
- **Verification:** Exercise new, existing, and malformed forms through the real
  process.
- **Dependencies:** Current internal acceptance of Tasks 1 and 2.
- **PR relationship:** PR 3 stacked on PR 2.
- **Concurrency:** Not ready before both dependencies and the fan-in stack are
  current.
- **Non-goals:** No new CLI ownership or persistence.
- **Delegation:** Test fixtures and private CLI wiring.
- **Discipline:** TDD.
- **Commit intent:** CLI composition and process tests; writer chooses the
  message.

## Feature acceptance

Confirm all three exact Task PR results remain current. Materialize the approved
stack and run only the integration-only compatibility route. Do not rerun the
ordinary full task reviews. Run the approved targeted integration reviewer, then
report Feature Accepted only when coverage, topology, and integration evidence
have no gap.

## Staleness and correction

A changed parent, restack, retarget, contract, or consumed interface makes every
affected descendant range stale. Re-materialize the approved topology and rerun
fresh verification and the complete task gate for changed ranges. Route a
concrete in-scope finding through its owning Task Contract; return semantic or
topology changes to approval.

## Publication

An internally accepted Task PR may be published with explicit user authority
before feature acceptance. Keep Feature Contract and Implementation Plan files
ignored in the coordination worktree until that worktree is explicitly removed;
do not delete them as a separate Feature Accepted action. Do not infer push,
merge, retarget, force, or cleanup authority.
