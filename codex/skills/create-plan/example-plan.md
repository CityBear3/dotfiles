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

Task workspaces use separate Herdr-managed worktrees. They are materialized
lazily when each Task becomes dependency-ready. Plan approval fixes their
identity; authorization to start `execute-plan` grants creation or reuse of that
exact non-destructive local state.

## Fixed decisions and non-goals

- Preserve existing accepted forms and malformed-input behavior.
- Keep parser, renderer, and CLI responsibilities separate.
- Do not change persistence, permissions, or error ownership.
- Allow parser and renderer work to begin independently.
- Feature Lead owns readiness, dependency release and Feature Acceptance.
  Each Task Lead is sole writer and local loop owner in an independent Herdr
  session; it dispatches native checks, not another implementer.

## Shared interface contracts

### Parsed input representation

- **Owner:** Task 1.
- **Implementer:** Task 1.
- **Consumers:** Tasks 2 and 3.
- **Contract:** The Feature Contract fixes the public representation and error
  meaning; private parser helpers remain delegated.

### Planned discovery cache

- Path: `docs/plans/YYYY-MM-DD-<feature>/search-cache.md`.
- Owner: The Feature lead is the only writer.
- Consumers: Feature Lead, independent Task Leads and relevant read-only leaves.
- Entry: Record purpose and scope, source identity, observation date or
  repository identity, positive and useful negative results, reuse conditions
  and source-aware invalidation conditions.
- Boundary: The cache never replaces current Git, authority, verification, or
  review evidence.
- Lifecycle: It has the same lifecycle as the ignored Implementation Plan.

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
  `implementation-quality-reviewer` after mechanical verification against each
  exact Task PR range.
- **Integration required reviewers:** `design-alignment-reviewer` for the
  composed compatibility journey against the named Feature obligation.
- **Integration conditional reviewers:** `risk-reviewer`, API perspective,
  only if a changed public parser seam admits ambiguous caller usage. Its
  authority is the approved parser contract; surface is that seam, failure
  model is realistic caller misuse, evidence is a concrete failing call, and
  stop condition is a verified issue or bounded inspection with no finding.
- **Conditional integration:** Use `finding-integrator` for overlap/conflict,
  authority defects, scope-sensitive remedies or non-trivial attribution. No
  integrator for all-clean or a single clear bounded finding eligible for
  direct triage. Prioritize authority-defect evidence before unstarted review.
- **Skipped perspectives:** Skip architecture and performance because approved
  ownership and measured hot paths do not change; skip adversarial robustness
  unless recovery behavior changes; skip scope because each Task gate receives
  exact clauses and ownership.
- **Residual risk:** No exhaustive grammar fuzzing.
- **Runtime admission and order:** Schedule ready Task 1 and Task 2 candidates
  in deterministic order in separate worktrees, then their Task gates, Task 3,
  and integration review. Keep a runtime-rejected role pending in that order and
  retry after progress without reducing reviewer breadth. Phase gates keep
  implementation, verification, findings integration, triage, and correction
  ordered; independent reviewers may run only after verifier `PASS`.
- **Acceptance:** Keep only artifact-applicable findings with an approved
  requirement, reachable evidence, material consequence, and proportionate
  correction. Drop preference, speculation, optional polish, and objections to
  approved decisions without materially new evidence.

## Model and session allocations

These are proposals until the engineer approves this plan, including the quality
and cost rationale. Feature Lead is not assigned; it uses its session defaults.

| Default role | Model | Effort |
| --- | --- | --- |
| Task Lead | gpt-5.6-sol | high |
| verification-runner | gpt-5.6-luna | low |
| spec-reviewer | gpt-5.6-sol | high |
| implementation-quality-reviewer | gpt-5.6-sol | high |
| risk-reviewer | gpt-5.6-sol | xhigh |
| finding-integrator | gpt-5.6-sol | high |
| design-alignment-reviewer | gpt-5.6-sol | xhigh |

| Task | Task Lead override / effective allocation | Quality and cost rationale |
| --- | --- | --- |
| 1 | Astra/high | Propose extra reasoning capacity for senior/staff-level public compatibility judgment across parsing and errors; engineer confirms whether the benefit justifies cost |
| 2 | None: Sol/high | Same high correctness/maintainability bar; exact rendering oracle and settled public contract bound the reasoning surface |
| 3 | None: Sol/high | High integration quality remains required; settled composition and real-process evidence provide a direct oracle |

Native checks use the fixed profiles above. Each independent Task root receives
the installed `execute-task/references/task-lead.md` contract and explicit model,
normal effort, Plan-mode effort and exact worktree through Herdr startup.
Both effort settings match its approved effort. No separate native Task Lead
profile, implicit global default, runtime promotion/fallback or startup approval.
Unavailable allocations are BLOCKED. Each Task keeps its session through
bounded correction and reports Task-level results with attributable evidence.

## Task Contract 1: Parse the approved form

- **Purpose:** Return the complete approved representation while preserving
  current successful and malformed-input behavior.
- **Coverage:** Parsing and parser-side compatibility clauses.
- **Ownership:** Existing library parser and its focused tests.
- **Shared interface:** Own the parsed input representation.
- **Constraints:** Preserve public errors and accepted forms.
- **Verification:** Observe the complete value and representative compatibility
  behavior.
- **Verification Matrix:** After the commit, map every obligation to one bounded
  check, expected observation, and `FAIL` or `BLOCKED` non-match category;
  rebuild after a head, range, authority, or material route change.
- **Dependencies:** None.
- **PR relationship:** PR 1 against `main`; final base exists immediately.
- **Workspace:** Herdr-managed branch `feature/input-form-parser`, starting from
  the observed implementation-base commit.
- **Concurrency:** May run with Task 2 in a separate checkout.
- **Non-goals:** No CLI, persistence, or API redesign.
- **Delegation:** Private helpers, local types, and focused tests.
- **Discipline:** TDD applicable because the parser behavior is executable before
  production editing.
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
- **Verification:** Observe exact new and representative existing output.
- **Verification Matrix:** After the commit, map every obligation to one bounded
  check, expected observation, and `FAIL` or `BLOCKED` non-match category;
  rebuild after a head, range, authority, or material route change.
- **Dependencies:** None logically.
- **PR relationship:** PR 2 stacked on PR 1. A common-base candidate is allowed;
  final acceptance requires restacking and fresh verification and review.
- **Workspace:** Herdr-managed branch `feature/input-form-renderer`, starting
  from the observed implementation-base commit for candidate execution.
- **Concurrency:** May implement with Task 1 in a separate checkout.
- **Non-goals:** No parser or CLI edits.
- **Delegation:** Private rendering helpers and test arrangement.
- **Discipline:** TDD applicable because renderer output is directly observable.
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
- **Verification Matrix:** After the commit, map every obligation to one bounded
  check, expected observation, and `FAIL` or `BLOCKED` non-match category;
  rebuild after a head, range, authority, or material route change.
- **Dependencies:** Current internal acceptance of Tasks 1 and 2.
- **PR relationship:** PR 3 stacked on PR 2.
- **Workspace:** Herdr-managed branch `feature/input-form-cli`, starting from
  the current accepted Task 2 head after the planned stack is materialized.
- **Concurrency:** Not ready before both dependencies and the fan-in stack are
  current.
- **Non-goals:** No new CLI ownership or persistence.
- **Delegation:** Test fixtures and private CLI wiring.
- **Discipline:** TDD applicable because the real process journey is executable.
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
topology changes to approval. For a bounded correction from `H1` to `H2`, create
one correction commit, rebuild the matrix and run fresh `H2` verification.
Rerun finding-owning and affected reviewers; supply explicit non-invalidation
evidence for other carried coverage, with uncertainty requiring rerun. `review`
owns coverage and traversal; supply its impact map, prior reports/triage, exact
delta, current target and fresh matrix without duplicating the rules here.

## Publication

An internally accepted Task PR may be published with explicit user authority
before feature acceptance. Keep Feature Contract, Implementation Plan, and
`search-cache.md` files ignored in the coordination worktree until that worktree
is explicitly removed; do not delete them as a separate Feature Accepted
action. The Feature lead is the cache's only writer, and consumers use it only
as source-aware discovery navigation. Do not infer push, merge, retarget,
force, or cleanup authority.
