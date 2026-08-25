---
name: execute-plan
description: Orchestrate an approved Implementation Plan across its Task dependency DAG, PR topology, isolated workspaces, and exact Task PR evidence.
---

# Execute an approved plan

Own approved-plan validation, dependency and PR-topology scheduling, per-task
Task-orchestrator handoff, root-granted capacity leases, workspace and agent
mapping, staleness propagation, and exact evidence aggregation. Do not edit
files, select implementer or reviewer roles, normalize findings, publish,
merge, or run verification or review itself.

## Validate plan entry

Before execution, require one approved authority form.

For new-format work, require:

- the approved, current Feature Contract and its design sources;
- an approved, current implementation plan;
- its complete Task Contract set, shared interface contracts, Feature Contract
  coverage, and integration-only obligations;
- its separate Review context and complete approved Review policy;
- a suitable non-default feature branch or approved workspace;
- understood working-tree state;
- an explicit discipline and observable verification obligations for every task;
- exact commands only where the plan marks their identity as contractually
  significant;
- settled dependencies, responsibility and ownership boundaries, shared
  interface owners and consumers, and non-goals.
- separate Task dependency DAG and PR topology, deterministic fan-in order,
  Task PR bases, task workspaces, concurrency eligibility, staleness rules, and
  exact integration-only starting identities, accepted inputs, order,
  mechanism, workspace strategy, identity checks, and cleanup eligibility.

For compatibility, accept an approved plan already executing before the
contract-centered format only when the coordinator supplies its exact approval
and in-flight evidence, referenced Design Doc or decision sources, unchanged
task specifications, Review context and policy, verification and completion
criteria, and confirmation that no material ambiguity exists and the owner did
not choose migration. Keep that legacy plan as the authority; do not manufacture
Feature or Task Contract files merely to satisfy the new shape.

Record the original plan implementation base, coordination workspace, and every
task branch, workspace, base, and head. Do not require all tasks to share one
advancing HEAD. On re-entry,
retain an already accepted task only when its exact Feature Contract authority,
assigned Feature clause meanings, Task Contract content, dependencies, and
relied-on shared-interface meanings remain unchanged. Preserve its exact base,
head, range, commit, verification, gate result, and gaps. Mark every affected or
transitively dependent result stale, exclude it from dependency release and
aggregation, and require fresh acceptance under both current approved
authorities. Do not widen an earlier task range when a later task adds commits.

When the approved plan follows a lightweight promotion, require the original
lightweight base, promotion head, execution-starting head, exact unaccepted range
and commits, later approved artifact state, changed files, attribution, writer
and gate evidence, and gaps. The plan's first ready step must be its approved
promotion reconciliation; never treat either later head as a clean
implementation base.

Stop and return a plan deviation when implementation would require a new
architecture, goal, scope, responsibility owner, public or shared interface
semantic, invariant, failure behavior, compatibility promise, verification
obligation, schema, error model, policy, or authority decision. The coordinator
owns the transition to the affected Design Doc, Feature Contract, or
Implementation Plan approval gate.

## Schedule Task PR work

Resolve both graphs before executing anything. Use the Task dependency DAG to
decide semantic readiness and the PR topology to decide the final review base.
A task is dependency-ready only when every logical predecessor is internally
`Accepted`. Human review and merge are not release conditions.

Permit multiple active tasks only when the approved plan marks them ready,
ownership-disjoint, free of conflicting shared state, and assigned to separate
branches and checkouts. Keep one writer per checkout and remain within approved
and observed capacity. Use `dispatching-parallel-agents` only as an adapter for
already bounded Task-orchestrator handoffs; queue deterministically rather than
weakening a gate.

Before every dispatch wave, resolve configured `agents.max_threads`, currently
observed runtime capacity, and all live descendants. Effective subagent capacity
is the lower configured or observed value. It excludes the root and counts each
Task orchestrator and leaf. The root alone grants leaf capacity. Normally grant
one leaf per schedulable active Task loop, never more than three or the smaller
current lease, and then distribute spare leaf slots in the plan's deterministic
queue order. Do not dispatch a Task orchestrator unless capacity is available
for both that orchestrator and its baseline leaf. Capacity rejection is
backpressure: retain queue and selected-role order without dropping,
substituting, or weakening work.

A task whose logical inputs are ready but whose final PR base is not yet
materialized may run in candidate mode when the plan permits it. Candidate work
never releases a dependent and cannot contribute to feature acceptance. Before
authoritative acceptance, materialize the approved final base, perform any
authorized restack or retarget operation, and require fresh exact-range
verification and review.

For each ready new-format planned Task, dispatch the exact `task-orchestrator`
profile through `dispatching-parallel-agents` and bind that identity to only
that Task Contract. Give it one concise plain-language handoff containing:

- exact Feature Contract identity, path, approval/currentness evidence, and the
  clauses assigned to this task;
- the exact applicable Task Contract, including purpose, expected result,
  constraints, dependencies, non-goals, and delegated local decisions;
- applicable shared interface contracts and adjacent-task obligations;
- the Review context and complete Review policy;
- the declared discipline and applicable repository guidance;
- the coordination directory, exact Herdr workspace and initial pane identities,
  Task worktree, branch, and planned PR identity, plus direct Git validation and
  any non-blocking lazygit warning;
- the starting commit, planned PR base ref and commit, current head, and whether
  the handoff is candidate or authoritative;
- current merge base, exact base-to-head range, diff, status, attributable
  commits, prior verification and review, concerns, gaps, and re-entry evidence
  when applicable;
- configured, observed, and effective subagent capacity; all relevant live
  identities; the current granted leaf count; and any roles already selected by
  `execute-task` or `review` for this wave;
- for authoritative re-entry of a prior candidate, its candidate commit, head,
  preliminary evidence, and the authorized final-base materialization or
  restack evidence;
- responsibility and ownership boundaries;
- verification routes and observable obligations;
- the responsibility-scoped commit intent and its fixed message or the approved
  writer authority to select that message;
- contractually significant files, signatures, ordering, and exact commands
  only when the approved plan fixes them.

Do not inline or require an unconditional reread of unassigned, unchanged
Feature Contract or Design Doc prose. Keep the exact sources directly available
for lookup when an assigned clause, shared interface, finding, or changed
evidence requires more context.

The Task orchestrator runs `execute-task` for that Task and may dispatch only
its policy-selected implementer, verifier, reviewer, or adversarial-integrator
leaves through `agent-teams-driven-development`, within the current root grant.
It is non-writing, keeps one source writer, and tells every leaf not to spawn
descendants. The root does not dispatch planned Task leaves. The Task
orchestrator may request more capacity but may not grant or infer it, reorder
selected roles, release dependencies, or decide Feature acceptance.

For an eligible legacy task, pass the approved legacy task specification and its
referenced design sources as the explicit authority, plus the same workspace,
base, discipline, verification, review, commit, and evidence fields available in
that plan. Preserve its exact approved execution topology; do not retrofit a
Task orchestrator or relabel it as a new Feature or Task Contract. Stop if a
missing field creates material ambiguity; do not force migration or infer a
decision.

For a new-format planned Task, let the Task orchestrator's `execute-task` loop
own that workspace's writer, commit, exact PR range, verification,
policy-selected review, correction, and stop condition. Accept `Candidate` only
for a plan-authorized early implementation whose final PR base is still
unavailable. Re-enter the Task in authoritative mode after that base is current,
passing the attributable candidate and restack evidence so `execute-task` can
skip duplicate implementation and commit work.

`Candidate`, `Accepted`, `BLOCKED`, and `Escalate` end the Task orchestrator's
current turn. Record its stable identity with the Task Contract, Task PR, Herdr
workspace, branch, and returned Git evidence; never assign it to another Task.
Prefer the same idle identity for a fresh attributable re-entry, but give it a
complete new handoff and revalidate all authority, policy, Git, writer, and
capacity evidence. If it is unavailable, dispatch a replacement only after the
earlier writer is proven inactive and all state is attributable. Otherwise
preserve state and return `BLOCKED`. Accepted identities do not wait or poll
through Feature completion and reserve no leaf capacity while idle; any identity
still reported live continues to count against observed capacity.

Before releasing a dependency or aggregating Feature evidence, directly resolve
the reported workspace, branch, planned base, merge base, head, range, diff, and
status through Git. Agent identity, memory, liveness, progress messages, Herdr,
lazygit, and pane state are operational observations only. A mismatch with the
Task result is `BLOCKED`, not authority to repair or reinterpret the state.

Do not start a logical dependent until every predecessor returns current
`Accepted`. On `BLOCKED`, `Escalate`, plan deviation, missing evidence, a
workspace mismatch, or a returned branch, base, or head that does not match the
observed task workspace, preserve all task states and return the exact gap to
`agentic-engineering-workflow`.

## Propagate stale results

Before every scheduling wave and feature aggregation, re-resolve the Task DAG,
PR topology, contract authorities, shared interfaces, task branches, bases,
heads, merge bases, diffs, statuses, live agent identities, and current capacity.
Traverse both graphs when an ancestor, topology edge, contract meaning, logical
dependency, or consumed interface changes. Mark every affected result stale,
remove it from dependency release and feature coverage, and re-enter its Task
orchestrator for authoritative `execute-task` after the approved final base is
restored.

Rebase, restack, retarget, force operations, or other history changes require
their applicable explicit authority. Reapproval of prose does not revive stale
Git evidence, and preliminary common-base checks do not survive restacking as
acceptance.

## Reconcile promoted lightweight work

Before ordinary planned tasks, give the bound Task orchestrator the approved
promotion-reconciliation Task Contract for its `execute-task` loop, original
lightweight base, promotion head,
execution-starting head, exact unaccepted range and commits,
attributable approved artifact state, complete change-to-Task-Contract mapping,
and prior writer and gate evidence. This special handoff authorizes acceptance
work on the attributable envelope; it does not authorize history rewriting or
new feature semantics.

Require fresh verification and the complete policy-selected task gate against
the current approved contracts. The preserved commits satisfy the reconciliation
commit intent when no correction is needed. If approved design or plan artifacts
remain uncommitted, the reconciliation Task Contract must declare their bounded
commit and one writer creates it before the gate. If correction is authorized,
use one writer and record a new bounded commit. Accept reconciliation only when every
preserved change has unambiguous ownership and current evidence; otherwise return
`BLOCKED` or a material plan deviation. Include the original lightweight base in
that task's accepted range and feature evidence, and do not release dependent
tasks before reconciliation is accepted.

## Resume only attributable work

After an interrupted or incomplete task, retain accepted and candidate results
for every other workspace separately from the observed in-flight work. Before
resuming one task:

1. confirm through the scheduling result that the prior Task orchestrator and
   leaf identities are known, the prior writer is inactive, and no writer
   overlaps;
2. inspect that workspace's branch, HEAD, status, commits, planned base, and
   exact base-to-head diff;
3. confirm the observed edits and commits are attributable to that task and
   descend from its task base;
4. confirm the unchanged task handoff still applies.

Resume the unfinished work or pending read-only gate only when all four checks
pass. If state is uncertain, mismatched, or unattributable, do not clean, reset,
recommit, or dispatch a replacement. Return `BLOCKED` with the observed agent and
Git state plus the exact condition required for re-entry. Use `Escalate` only
when resumption needs a material decision, scope, policy, or authority change.

An interrupted task remains unaccepted. Preserve an attributable candidate but
do not add it to accepted results, feature coverage, or dependency release until
`execute-task` returns current authoritative acceptance evidence.

## Re-enter for a planned correction

Treat an authorized correction as work on its owning Task PR. Preserve every
other task's exact result and the original implementation base. For new-format
planned work, give `execute-task` the exact finding or failed observation and
approved correction through the Task's retained or safely replaced orchestrator
identity. For eligible legacy work, preserve its approved invoking context.
Supply the observed attempts and results, unchanged Feature and Task Contracts
with shared interfaces or unchanged eligible legacy authority, Review context,
Review policy, current planned PR base and accepted head, responsibility
boundaries, verification obligations, fresh capacity grant when applicable, and
a correction commit intent bounded to the finding with its fixed message or
approved writer message-selection authority.

When the same concrete problem repeats without progress, or the next action would
repeat an observed failed correction, stop and return the attempt evidence. Do
not invent another tracking protocol or silently expand the correction.

After `Accepted`, append the correction once with its commit, exact PR base,
current head, range, fresh verification, gate result, and gaps. Traverse both
graphs, mark affected descendants stale, and recalculate feature coverage
without widening any unchanged task range.

## Materialize integration-only evidence

Only after every input Task PR is current and `Accepted`, materialize each
approved integration-only composition before returning `TasksAccepted`. Use
`create-workspace` to establish its plan-defined temporary workspace, then apply
the exact starting commit or tree and accepted Task PR inputs with the approved
deterministic mechanism and order. This is a Git composition operation, not a
source-writing task; assign no implementation writer and make no manual conflict
fix.

Record the workspace, starting identity, ordered input commits and trees,
commands, ending HEAD and tree, status, and diff. Require the observed tree to
match the plan's identity checks. A conflict, missing input, unexplained change,
or authority-required workspace or history operation returns `BLOCKED` or
`Escalate` as applicable. Never publish the temporary ref or treat it as another
Task PR. Return its exact identity and retain it through current integration
verification and targeted review. When the plan-defined retention boundary is
reached, report that it is cleanup-eligible; never remove its ref or workspace
without the applicable user-controlled disposition.

## Aggregate accepted tasks

After each accepted task, append a result keyed by Task Contract and PR identity
containing:

- task name and dependency position;
- Task orchestrator identity, Herdr workspace and pane mapping, configured,
  observed, and effective capacity, granted leaves, selected-role queue, and
  dispatch or replacement evidence;
- exact authority and Task Contract content/currentness accepted;
- Feature Contract clauses and Task Contract obligations, eligible legacy
  completion criteria, or promotion mappings proved;
- exact task workspace, branch, planned base ref and commit, merge base,
  accepted current head, and base-to-head range;
- verified starting Git status and verified final Git status, including index,
  worktree, and relevant untracked state, matched to direct root re-observation;
- task and correction commits;
- fresh verification obligations, commands selected or required, and observed
  results;
- per-task gate result;
- changed files, concerns, and gaps.

After every planned task is accepted and current:

1. retain the complete result set in deterministic Task DAG and PR-topology
   order;
2. re-resolve every task branch, base, head, range, Git status, dependency, and
   shared interface;
3. prove complete Feature Contract coverage and identify only the obligations
   that remain integration-only;
4. materialize and record the exact temporary tree for each integration-only
   obligation from its approved accepted heads and deterministic composition,
   without treating the composition as a PR;
5. report task publication eligibility, plan deviations, correction attempts,
   stale results, and residual gaps separately
   from successful evidence.

There is no synthetic full-feature review range. A stacked descendant head may
contain its ancestors, but it never replaces their task-specific accepted
ranges. A temporary composed tree exists only to prove named integration-only
obligations.

## Return orchestration status

Return:

- `TasksAccepted` only with every current Task PR result, both resolved
  topologies, complete Feature Contract coverage, any required integration
  composition, or the exact eligible legacy authority, plus Review context and
  complete Review policy; no candidate, stale result, or unreconciled promoted
  range may contribute;
- `BLOCKED` with all accepted and candidate results, observed in-flight agents
  and per-workspace Git state, configured/observed/effective capacity, current
  leases and queues, gaps, and exact re-entry condition;
- `Escalate` with the exact plan deviation, missing decision, policy conflict, or
  task escalation.

Return control to `agentic-engineering-workflow` after `TasksAccepted` or any
stop condition. Do not run feature integration verification or targeted review,
publish, merge, or choose branch disposition.
