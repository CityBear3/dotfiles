# [Design Doc] Codex PR-Scoped Task Execution

- Owner: Repository owner
- Drafted by: Codex from owner-approved design decisions
- Date: 2026-08-18
- Status: Approved
- Extends:
  - `docs/design/2026-07-29-codex-agentic-engineering-workflow.md`
  - `docs/design/2026-08-13-codex-design-centered-contract-workflow.md`

## Context and scope

The contract-centered workflow establishes Design Doc, Feature Contract, and
Task Contract layers before implementation. Its execution model is still a
single feature branch, however: `execute-plan` runs every Task Contract
sequentially against one advancing HEAD, and the coordinator performs one final
feature-wide verification and review before offering one pull request.

That model makes the approved task boundary weaker than the review and delivery
boundary. Several otherwise independent tasks become one large diff, an
architect must review unrelated responsibilities together, downstream work
cannot use isolated workspaces safely, and a correction to one responsibility
can invalidate evidence for the entire feature range. The larger loop also
reduces the implementation Agent's ability to make progress while a human is
reviewing an earlier pull request.

This design makes one Task Contract correspond to one pull-request-scoped
implementation loop by default. The engineer still owns feature design, task
decomposition, dependencies, shared interfaces, and the proposed PR topology.
Within those approved boundaries, Agents may implement independent tasks in
isolated workspaces and autonomously repeat implementation, verification,
review, triage, and bounded correction until the task is internally accepted.

The execution dependency graph and GitHub PR topology are related but distinct.
The plan preserves a dependency DAG for implementation. Because a GitHub pull
request has only one base, the plan separately projects that DAG onto sibling or
stacked PRs. A fan-in dependency is represented by a deterministic stack over
the required ancestor closure before the dependent task begins.

### Goals

- Make one approved Task Contract one independently reviewable PR by default.
- Run verification, review, triage, correction, and repeated verification and
  review against the exact Task PR base-to-head range.
- Let internally accepted prerequisite tasks release dependents without waiting
  for human PR review or merge.
- Allow independent tasks to be implemented concurrently in separate branches
  and worktrees with one writer per checkout.
- Record the implementation dependency DAG and PR topology separately in the
  approved Implementation Plan.
- Support fan-in by retaining parallel implementation where possible and
  deterministically stacking only the prerequisite closure needed by the
  dependent task.
- Preserve exact evidence across rebases, parent changes, interruptions, and
  context compaction by invalidating stale results rather than inferring that
  they still apply.
- Retain a feature acceptance boundary for contract coverage and
  integration-only obligations without repeating every task-level review over
  the combined feature.
- Keep PR publication, pushing, merging, force operations, and destructive
  cleanup under explicit user authority.

### Non-goals

- Change Design Doc, Feature Contract, or Implementation Plan approval
  ownership or merge their approval gates.
- Make every task run in parallel, or permit overlapping writers in one
  checkout.
- Force an artificial PR split when the behavior cannot remain buildable,
  verifiable, or contractually valid as separate tasks; inseparable work should
  instead be one Task Contract.
- Require `gh-stack` or another particular branch-management tool.
- Automatically publish, push, merge, force-push, retarget, or delete a branch
  or PR.
- Repeat a full feature-wide code review after every Task PR has already passed
  its approved review policy.
- Redesign review-mode strength, reviewer count, or the common Acceptance
  threshold. Review-intensity calibration is deferred to a separate task.
- Add a machine-readable workflow-state schema or a new tracked plan artifact.
- Add the workflow overview diagram to `codex/README.md` in this change.

## Overview

The approved Implementation Plan defines two topologies:

```text
Task dependency DAG                  PR topology

        +-- Task 1 --+               main
Base ---+            +-- Task 3        `-- PR 1: Task 1
        +-- Task 2 --+                     `-- PR 2: Task 2
                                               `-- PR 3: Task 3

Task 1 and Task 2 are logically independent.  The PR stack is a projection
needed because Task 3 consumes both and a PR can name only one base.
```

Independent tasks may begin from the same approved implementation base in
separate workspaces. When a downstream task has multiple parents, the planned
PR order linearizes those parent branches before the downstream task starts.
The linearization does not add a logical Task dependency: it only establishes a
reviewable single-base Git range.

Each Task PR follows this loop:

```text
approved Task Contract and final PR base
                  |
                  v
              implement
                  |
                  v
         task-scoped verification
                  |
             PASS | FAIL --> diagnose --> correct --+
                  v                                  |
           task-scoped review                        |
                  |                                  |
       CLEAN ------+------ FINDINGS --> triage ------+
                  |
                  v
        internally Accepted
                  |
          +-------+--------------------+
          |                            |
          v                            v
release ready dependents       user-controlled publication
                                       |
                              human review feedback
                                       |
                         same task correction loop
```

Internal acceptance is an Agent-side execution state, not human approval and
not merge authority. It is sufficient to release a dependent task because its
base, contract, verification, and review evidence are exact and current. Human
review may arrive later. A change to an accepted parent invalidates affected
descendant evidence and returns those descendants to the appropriate point in
their own loop.

After all Task Contracts have current accepted results, feature acceptance
checks coverage and integration-only obligations:

```text
all current Task PR results
             |
             v
  validate contracts, topology,
  exact heads, ranges, and coverage
             |
             v
integration-only obligations present?
        | no                 | yes
        |                    v
        |        materialize exact integration tree
        |                    |
        |        run only integration obligations
        +--------------------+
             |
             v
       Feature Accepted
```

A full repeated feature review is not part of this default path. The plan may
require a targeted integration review only when an approved Feature Contract
obligation or a concrete cross-task risk cannot be assessed from the individual
PR ranges.

## Detailed design

### Task Contract and PR boundary

One Task Contract maps to one PR candidate by default. The task owns one
responsibility-scoped diff, its tests and other verification evidence, and the
corrections required by its review. The PR boundary is therefore chosen during
task decomposition rather than after implementation.

If two proposed tasks cannot be independently built, verified, reviewed, or
kept contractually valid, `create-plan` does not place them in one PR as
nominally separate tasks. It either returns an unresolved design boundary or
defines them as one Task Contract with one responsibility and one acceptance
loop. A plan may depart from the one-to-one default only through an explicit,
owner-approved exception with a concrete atomicity or compatibility reason.

Each Task Contract continues to define its purpose, Feature Contract coverage,
ownership, shared interfaces, invariants, verification obligations,
dependencies, non-goals, delegated local decisions, discipline, and commit
intent. It additionally identifies:

- its PR unit and intended review responsibility;
- its logical Task dependencies;
- its planned PR parent or sibling relationship;
- whether its implementation may start before its final PR base is
  materialized;
- conditions that make its accepted evidence stale;
- any publication or landing constraint relevant to adjacent tasks.

### Dependency DAG and PR topology

`create-plan` records the Task dependency DAG and PR topology as separate
normative sections.

The Task dependency DAG determines when implementation inputs are semantically
available. A task becomes dependency-ready only after every logical predecessor
has a current internal `Accepted` result. Human review and merge are not
dependency-release conditions.

The PR topology determines the Git base against which each task is reviewed.
It uses:

- sibling PRs for independent tasks without a downstream fan-in that requires
  their combined tree;
- a linear stack for ordinary dependent tasks;
- a deterministic linearization of the required parent closure when a task has
  multiple logical parents;
- a temporary integration branch or worktree only for feature obligations that
  require a combined tree but do not justify changing otherwise independent PR
  relationships.

For a fan-in such as Task 3 depending on independent Task 1 and Task 2, the plan
chooses and exposes one review order, for example `Task 1 -> Task 2 -> Task 3`.
Task 1 and Task 2 may still be implemented concurrently from a common base.
Before Task 2 receives authoritative acceptance, its work is restacked onto the
accepted Task 1 head and its task loop runs against that final base. Task 3
starts only after both parent results are current and the planned stack has been
materialized.

The stack order is part of Implementation Plan approval because it affects diff
identity, rebase cost, review sequence, and invalidation propagation. It does
not change the logical ownership or Feature Contract coverage of the tasks.

### Isolated scheduling and writers

The planned path may have more than one active task writer only when all of the
following hold:

- the tasks are implementation-ready under the dependency DAG;
- the plan declares that their ownership and likely writes do not overlap;
- each task has its own branch and checkout or worktree;
- each checkout has exactly one writer;
- required execution and reviewer capacity is available;
- neither task mutates shared external state that makes concurrent evidence
  ambiguous.

Tasks with overlapping ownership, unresolved shared interfaces, shared mutable
state, or a dependency remain sequential. Parallelism is an execution option,
not a completion requirement.

`create-workspace` establishes the coordination workspace and may establish or
validate task workspaces described by the approved topology. It no longer
assumes that one planned feature necessarily has one implementation branch.
Branch creation, worktree creation, and any tool-specific operation retain their
existing authority and safety boundaries.

`execute-plan` owns readiness calculation, deterministic scheduling, task and
workspace mapping, and exact evidence aggregation. It must never use parallel
execution to weaken a selected reviewer gate, exceed capacity, or place two
writers in one checkout.

### Authoritative Task PR loop

`execute-task` remains the bounded implementation-and-correction owner for one
Task Contract, but its accepted target is the planned PR range instead of the
next segment of one global feature HEAD. It records the exact base commit, head,
merge-base-derived diff, branch, workspace, status, commits, verification, and
review evidence.

`verify` and `review` accept a coordinator-managed Task PR entry. Verification
proves the Task Contract's observable obligations on the exact PR range. Review
applies the plan's approved Review context and policy to that same range. Review
intensity does not increase merely because the workflow uses more PRs; the
existing policy and Acceptance threshold remain controlling.

A failing observation is diagnosed before correction. Concrete review findings
go through `receiving-code-review` and are classified as `Fix`, `Push back`, or
`Escalate`. An authorized `Fix` returns to the same task branch and repeats
fresh verification and review. A contract, topology, responsibility, or scope
change returns to the owning approval gate rather than being hidden in a fix.

A task becomes internally `Accepted` only when:

- its final planned PR base and current head resolve exactly;
- fresh verification passes for that range;
- its approved task review is `CLEAN` after triage;
- the branch and worktree still match the reviewed evidence;
- its dependencies and consumed shared-interface meanings remain current;
- no unexplained or out-of-scope change remains.

### Publication and human review

Internal acceptance makes a branch eligible for publication; it does not
authorize publication. `finish-branch` applies task-scoped completion evidence
when presenting branch publication or disposition choices, and `create-pr`
publishes only the exact approved head and planned base after the user grants the
external write.

Feature Contract and Implementation Plan artifacts remain active after an
individual Task PR is published. Feature acceptance ends their execution role,
but does not trigger individual deletion: they remain ignored and workspace-
local until the containing coordination worktree is explicitly removed. Archive
them only when the user asks to preserve them beyond that worktree lifecycle.

Human review feedback on a published PR is evaluated against the same Task
Contract and current PR range. An accepted `Fix` uses the same task loop, and a
push or PR-base update remains an external write requiring applicable authority.
A requested design or plan change returns to the corresponding approval gate.

Merge order follows the current PR topology and remains user-controlled. The
workflow does not infer merge, retarget, force-push, or branch-deletion authority
from implementation approval or PR creation.

### Fan-in, rebases, and stale evidence

Acceptance evidence is bound to exact authority and exact Git identity. Any of
the following makes a Task PR result stale when it can affect that identity or
meaning:

- its base, head, merge base, diff, or in-scope status changes;
- an ancestor branch is amended, rebased, restacked, or corrected;
- the planned PR parent or stack order changes;
- an assigned Feature or Task Contract clause changes meaning;
- a consumed shared interface or logical dependency changes meaning;
- human review causes an upstream change included in the task's effective tree.

The workflow determines the affected descendants from both the Task dependency
DAG and PR topology. It rebases or restacks only through an authorized,
attributable operation, then requires fresh verification and review for every
changed PR range. Reapproval of prose alone never revives stale Git evidence.

This rule deliberately accepts some repeated checking in exchange for starting
independent implementation early. The alternative—waiting for every parent PR
to be reviewed and merged—would keep bases simpler but would suspend Agent work
during potentially long human review delays.

### Feature acceptance

Feature acceptance is distinct from a repeated feature-wide task loop. It
requires:

- one current internal `Accepted` result for every approved Task Contract;
- exact agreement between accepted heads, the dependency DAG, and PR topology;
- complete Feature Contract coverage with no stale or unexplained task result;
- proof of every integration-only verification obligation;
- current triage state with no surviving finding or design gap.

When an integration-only obligation exists, `execute-plan` identifies the exact
accepted heads and deterministic composition order. A temporary integration
branch or worktree may materialize that tree without publishing it or changing
the approved PR topology. The applicable integration commands and observations
run against that exact tree. The temporary tree is evidence, not an additional
feature PR.

The coordinator does not run the ordinary full `$review` again over all combined
task changes. It requests a targeted integration review only when the approved
plan names an obligation that cannot be reviewed in one Task PR, or fresh
integration evidence exposes a concrete cross-task risk covered by the current
policy. A newly discovered policy or design choice is escalated rather than
silently adding review scope.

After feature acceptance, `finish-branch` leaves the ignored Feature Contract
and Implementation Plan in the coordination worktree and presents the remaining
user-controlled topology disposition choices. Their cleanup follows an explicit
worktree-removal decision and occurs with that worktree rather than as a
separate artifact-deletion step. Published PRs, branches, and worktrees are not
deleted as an implied cleanup action.

### Lightweight path

The lightweight path remains one coherent Feature/Task Contract and therefore
one PR candidate. Its task verification and review also constitute its feature
acceptance when no separate integration-only obligation exists. It does not add
parallel task scheduling, a PR topology section, or a second duplicate final
review.

If lightweight work becomes more than one coherent PR responsibility or needs a
fan-in, it promotes to the planned path under the existing preservation and
approval rules.

## Cross-cutting concerns

### Compaction, interruption, and recovery

The ignored Feature Contract and Implementation Plan remain the source of the
approved goal, Task Contracts, dependency DAG, PR topology, integration
obligations, and policy while work is active. Git branches, worktrees, commits,
and published PR metadata provide observable execution state.

Every task handoff and result records its exact workspace, base, head, range,
commands, observations, reviewer result, and dependency evidence. On resume,
the coordinator re-reads the approved artifacts and reconstructs the topology
from Git before accepting retained evidence. If an earlier acceptance result
cannot be recovered after compaction or a fresh session, the safe recovery is
to rerun the applicable verification and review—not to infer acceptance or
create another tracked runtime-state file.

An interrupted writer is not replaced until its liveness and workspace state
are known. Unattributable commits, overlapping worktrees, ambiguous ancestry,
or mismatched PR metadata return `BLOCKED` without reset, cleanup, or history
rewriting.

### Review proportionality

Moving review to each Task PR changes the target and timing, not the approved
severity threshold or reviewer breadth. Each reviewer sees only the Feature
clauses, Task Contract, shared interfaces, Review context, policy, diff, and
evidence applicable to that PR. Findings still require concrete reachable
evidence, an approved requirement or material quality consequence, and a
proportionate correction.

General reconsideration of `focused`, `adaptive`, or `deep` defaults and reviewer
counts is explicitly outside this design.

### Failure behavior

The workflow returns:

- `Escalate` for a missing user-owned design, topology, task-boundary, policy,
  publication, merge, or scope decision;
- `BLOCKED` when exact branch, worktree, ancestry, evidence, capacity, or
  operational state cannot be established safely;
- a plan deviation when implementation requires a different Task Contract,
  dependency DAG, PR topology, shared interface, or integration obligation;
- stale evidence when an exact accepted range or controlling meaning changes.

It does not convert preliminary checks on a common-base branch into final
acceptance after restacking, or treat a clean child PR as current after its
parent changes.

## Alternatives considered

### Keep one feature branch and one feature PR

Rejected because unrelated Task Contracts remain mixed in one architectural
review surface, independent implementation cannot use isolated writers safely,
and correction and evidence invalidation operate over an unnecessarily large
range.

### Stack every task from the start

Rejected because it turns logical independence into execution dependence,
delays parallel implementation, and increases avoidable rebase and review
churn. The chosen design stacks dependent chains and fan-in closures only where
a single PR base requires it.

### Keep fan-in parents as sibling PRs and wait for both to merge

Rejected because the dependent Agent would remain idle throughout human review
and merge latency. Internal acceptance provides a precise release condition
without granting merge authority.

### Use an unpublished synthetic integration branch as the permanent PR base

Rejected because it hides multiple parent changes behind a base that is not a
normal review or landing unit, complicates GitHub comparisons, and makes branch
ownership and retargeting less clear. Temporary integration trees remain useful
only for integration-only evidence.

### Publish a task before its local verification and review are clean

Rejected as the default because it externalizes avoidable Agent feedback noise
and gives reviewers an unstable target. Publication begins only after internal
acceptance, while later human findings reopen the same loop.

### Repeat full feature verification and review after all Task PRs pass

Rejected because task-scoped obligations and review findings would be evaluated
twice without new evidence. Feature acceptance retains coverage, currentness,
and integration-only proof, with targeted integration review when the approved
contract actually requires it.

### Remove feature acceptance entirely

Rejected because individually clean tasks do not prove complete Feature
Contract coverage, current dependency evidence, or behavior observable only in
the combined tree.

### Require a particular stacked-PR tool

Rejected because the dependency and PR topology contracts are tool-independent.
A compatible tool may implement approved operations, but it is not part of the
workflow's semantic contract.
