---
name: execute-plan
description: Coordinate Task execution and integration after the Implementation Plan is approved and execution is authorized.
---

# Execute an approved plan

The Feature Lead owns approved-plan validation, dependency/PR-topology readiness,
independent Task-session handoffs, workspace/session mapping, cross-Task
staleness and evidence aggregation. Task Leads own their local writer/check/
correction loops. Do not implement Task source, choose their private details,
proxy their ordinary leaf calls, publish, merge or run integration checks here.

## Validate plan entry

Before execution, require one approved authority form.
Resolve the workflow revision approved for that plan before applying the
new-session rules below. An older approved plan, including an older
contract-centered plan, does not inherit this topology merely because its
artifact format matches. Use its exact prior assets or stop for recovery or
explicit owner-selected migration.

For new-format work, require:

- the approved, current Feature Contract and its design sources;
- an approved, current implementation plan;
- explicit user authorization to start this exact approved plan, which grants
  creation or reuse of its exact non-destructive local Task workspaces and
  independent sessions at the engineer-confirmed allocations;
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
  Task PR bases, Task workspace modes, branch identities, exact or deterministic
  starting-ref resolution rules, concurrency eligibility, staleness rules, and
  exact integration-only starting identities, accepted inputs, order,
  mechanism, workspace strategy, identity checks, and cleanup eligibility.

Use an optional search-cache path and relevant reusable findings when supplied.
Their absence or staleness is not a missing-authority condition; no cache-miss
report is required. The Feature Lead remains its only writer.

For compatibility, accept an approved plan already executing before the
contract-centered format only when the coordinator supplies its exact approval
and in-flight evidence, referenced Design Doc or decision sources, unchanged
task specifications, Review context and policy, verification and completion
criteria, and confirmation that no material ambiguity exists and the owner did
not choose migration. Keep that legacy plan as the authority; do not manufacture
Feature or Task Contract files merely to satisfy the new shape.

Record the original plan implementation base, coordination workspace, every
planned Task workspace identity, and the branch, workspace, base, and head of
each already materialized Task. Do not require every workspace to exist before
execution or all tasks to share one advancing HEAD. On re-entry,
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

Apply the coordinator's continuation rules before declaring a plan deviation.
An ordinary planned procedure, including named RED/GREEN steps, does not by
itself make its execution history an independent Acceptance obligation. Recover
missing factual handoff/report evidence with its owning Task and preserve
history-only discrepancies without requesting an exception. Stop and return a
plan deviation when implementation would require a new
architecture, goal, scope, responsibility owner, public or shared interface
semantic, invariant, failure behavior, compatibility promise, verification
obligation, schema, error model, policy, or authority decision. The coordinator
owns the transition to the affected Design Doc, Feature Contract, or
Implementation Plan approval gate.

## Schedule independent Task sessions

Resolve the Task dependency DAG separately from PR topology. Release a Task
only when every logical predecessor is internally Accepted; publication/human
merge is not required. Concurrent Tasks must be plan-ready, ownership-disjoint
and isolated in separate worktrees without conflicting shared mutable state.
A PR stack edge alone is not a logical dependency.

Candidate mode is permitted only when the plan allows implementation before
its final PR base exists. A candidate cannot release dependents or satisfy
Feature Acceptance. Final-base materialization and any history changes need
their applicable authority and complete current exact-range gate coverage under
the coordinator's evidence-applicability rule.

For each ready Task, resolve the plan's exact/deterministic starting ref from
current Git and accepted predecessors, then use `create-workspace` to lazily
create/reuse that exact Herdr worktree and shell pane. Execution-start authority
covers those named non-destructive local workspaces and Task-session launches;
do not repeat per-Task approval. Missing identities, unavailable refs requiring
fetch, mismatched branches or pane occupants, or destructive/history operations
remain BLOCKED/Escalate. Never change the user's coordination checkout or
repair a mismatched workspace implicitly.

Resolve the engineer-confirmed model and effort for each Task and all selected
leaves. The plan must identify the shared Task Lead role source and explicit
Codex startup binding, including normal and Plan-mode effort. Feature Lead
uses session defaults and is not assigned by the plan. Unavailable allocations
are BLOCKED, not fallback or runtime promotion.

Use `dispatching-parallel-agents` for Herdr launch/resume of each selected Task
root. Supply the complete handoff defined by `execute-task`: relevant exact
authority and clauses, ownership, verification discipline, complete policy,
effective allocations, worktree/Git/Herdr routing identities, mode, commit
intent and attributable re-entry evidence. Add relevant reusable findings or
optional cache references only when useful. Keep sources directly readable and
omit unrelated conversation or unchanged unassigned prose. Each new Task
session starts without inherited Feature conversation.

The Task Lead implements and owns native runner/reviewer/integrator dispatch,
triage and bounded correction. Do not start a separate implementer, build
routine Task matrices, or relay each local transition through the Feature Lead.
Native admission is local to each session; preserve selected pending work and
do not infer unlimited service capacity across independent sessions. Do not
calculate leases or build another scheduler.

Keep each Task session through ordinary correction. Record Task-to-Herdr-agent/
pane-to-worktree mapping and exact allocation. Herdr waits/reads are routing
observations, not prompt-specific receipts or acceptance. Use bounded responsive
waiting within tool limits, avoiding unchanged busy polling. A timeout does not
mean a stopped writer. Follow the adapter's safe re-entry rules before resending
work or replacing a session.

Consume compact Candidate/Accepted/BLOCKED/Escalate reports with directly
available attributable evidence references, not every local transcript. Consume
Accepted as the completed Task-local quality decision under the coordinator's
Accepted boundary. Match the result to its assigned Task/session and approved
authority; directly confirm workspace, branch, reviewed base, head/range/status
and correspondence to the intended target. Compare relevant changed inputs and
consumed dependencies. Do not rejudge Task test adequacy, rebuild its matrix,
re-audit raw runner/reviewer reports, or re-prove its gate completeness.

The Task Lead retains original verification/reviewer reports and carry-forward
arguments. Read that detail only for a concrete mismatch, contradiction or
requested diagnosis. Recover missing result metadata or references from the
existing owner; an omitted pointer does not prove the check was omitted. For an
actual contradiction, such as Accepted with a required review still pending,
hold the affected dependency release and return the exact discrepancy to that
Task Lead for authorized recovery. Do not repair Task source, reinterpret
authority, assume an unperformed check passed, or reset uncertain state. Return
BLOCKED only when no safe authorized recovery is available.

After confirming that Accepted applies, record it and immediately recalculate
readiness; dispatch newly ready Tasks unless the user requested a boundary stop.
Only Feature Lead releases dependencies. Preserve other exact Task results on a
blocked Task and continue independent dependency-ready work; report an unresolved
owning gap and attempted recovery to `agentic-engineering-workflow`.
A Task Design Escalation returns its authority evidence early; do not dispatch
affected queued work or silently repair design.

Previously approved/in-flight topologies require their exact prior coherent
assets, models and policy. If these cannot be used safely, preserve the state
and request recovery or owner-selected migration; never retrofit independent
Task sessions or new review rules onto legacy authority.

## Propagate stale results

These rules govern active approved execution. For a concern found after the
loop has completed and control has moved to completion/publication, return to
the coordinator's engineer discussion boundary before reopening Task work.

Before every scheduling wave and feature aggregation, re-resolve the Task DAG,
PR topology, contract authorities, shared interfaces, task branches, bases,
heads, merge bases, diffs, statuses, and decision-relevant live agent state.
Distinguish the fixed reviewed base from the latest observed base-branch tip
using the coordinator's reviewed-base/advancing-tip rule. A confirmed compatible
fast-forward alone preserves Accepted and does not stale descendants, block
dependency release, or re-enter Task loops. Refresh a named latest-base
integration obligation only when its composition inputs changed.
Traverse both graphs when an ancestor, topology edge, contract meaning, logical
dependency, or consumed interface changes. Compare actual inputs and obligations
under the coordinator's evidence-applicability rule; identity-only changes do
not stale evidence. Mark every result with invalidated required coverage stale,
remove it from dependency release and feature coverage, and re-enter its
Task Lead's authoritative `execute-task` loop after the approved final base is
restored.

Do not mark the complete accepted set stale merely because a Design Doc,
Feature Contract, Task Contract, or plan artifact changed. Retain and directly
revalidate a result whose assigned authority meaning, dependencies, consumed
interfaces, base, head, range, and status are unchanged; mark only semantically
affected Tasks and their transitive dependents stale.

Rebase, restack, retarget, force operations, or other history changes require
their applicable explicit authority. Reapproval of prose does not revive stale
Git evidence. Preliminary common-base checks are not acceptance; map their
actual coverage and final-base applicability, supplement gaps, and establish
the complete approved gate before accepting a restacked Task.

## Reconcile promoted lightweight work

Before ordinary planned tasks, give the Task Lead's `execute-task` loop the
approved promotion-reconciliation Task Contract, original
lightweight base, promotion head,
execution-starting head, exact unaccepted range and commits,
attributable approved artifact state, complete change-to-Task-Contract mapping,
and prior writer and gate evidence. This special handoff authorizes acceptance
work on the attributable envelope; it does not authorize history rewriting or
new feature semantics.

Require current verification and the complete policy-selected task gate against
the current approved contracts, mapping prior applicable independent evidence
and supplementing uncovered obligations. The preserved commits satisfy the reconciliation
commit intent when no correction is needed. If approved design or plan artifacts
remain uncommitted, the reconciliation Task Contract must declare their bounded
commit and one writer creates it before the gate. If correction is authorized,
use one writer and record a new bounded commit. Accept reconciliation only when
every preserved change has unambiguous ownership and current evidence;
otherwise return `BLOCKED` or a material plan deviation. Include the original
lightweight base in that task's accepted range and feature evidence, and do not
release dependent tasks before reconciliation is accepted.

## Resume only attributable work

After an interrupted or incomplete task, retain accepted and candidate results
for every other workspace separately from the observed in-flight work. Before
resuming one task:

1. resolve the prior Herdr session and native leaf identities, confirm the
   prior writer is inactive before replacement, and ensure no competing writer
   or old-head check overlaps;
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

For completed-work feedback, require the coordinator's established shared
problem understanding and engineer-agreed response and execution authority.
An old implementation or publication approval alone does not reopen the loop.

Treat an authorized correction as work on its owning Task PR. Preserve every
other task's exact result and the original implementation base. For new-format
planned work, give the Task Lead's `execute-task` loop the exact finding or
failed observation and approved correction. For eligible legacy work, preserve
its approved invoking context.
Supply the observed attempts and results, unchanged Feature and Task Contracts
with shared interfaces or unchanged eligible legacy authority, Review context,
Review policy, current planned PR base and accepted head, responsibility
boundaries, verification obligations, selected or pending roles, and a
correction commit intent bounded to the finding with its fixed message or
approved writer message-selection authority. Also supply prior reviewed head
`H1`, prior reviewer reports/triage and the unchanged complete policy coverage.
Task-local corrections stay in the existing Task Lead session; only integration,
feedback re-entry and cross-Task effects need Feature coordination. Require a
bounded correction commit H2, current matrix and fresh affected verification
with explicitly carried unaffected rows under the coordinator's applicability
rule, then
finding-owner/affected-perspective reruns with explicit non-invalidation
evidence for any carried coverage. `review` owns
correction-review scope and escalation; pass its required correction evidence
without restating traversal rules here.

When the same concrete problem repeats without progress, or the next action would
repeat an observed failed correction, stop and return the attempt evidence. Do
not invent another tracking protocol or silently expand the correction.

After `Accepted`, append the correction once with its commit, exact PR base,
current head, range, current verification, gate result, and gaps. Traverse both
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
- Herdr workspace/pane/session mapping and references to Task-owned dispatch,
  leaf and recovery evidence;
- exact authority and Task Contract content/currentness accepted;
- Feature Contract clauses and Task Contract obligations, eligible legacy
  completion criteria, or promotion mappings proved;
- exact task workspace, branch, planned base ref and commit, merge base,
  accepted current head, and base-to-head range;
- verified starting Git status and verified final Git status, including index,
  worktree, and relevant untracked state, matched to direct root re-observation;
- task and correction commits;
- the Accepted result and references to its Task-owned matrix, original runner
  and reviewer reports, triage and carry-forward evidence;
- non-blocking concerns, changed files, any useful cross-session discovery
  candidates, and gaps.

The Task Lead retains the detailed gate record. Aggregate its verdict and
references without reading every row or reconsidering local quality judgments.

After every planned task is accepted and current:

1. retain the complete result set in deterministic Task DAG and PR-topology
   order;
2. re-resolve every task branch, base, head, range, Git status, dependency, and
   shared interface;
3. map every approved Feature clause to its assigned current Accepted Task or
   named integration-only obligation; do not reinterpret Task test coverage or
   add a new verification requirement while aggregating;
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
- `BLOCKED` with all accepted and candidate results, observed in-flight agents,
  per-workspace Git state, pending roles, runtime-rejection evidence, gaps, and
  exact re-entry condition;
- `Escalate` with the exact plan deviation, missing decision, policy conflict,
  task escalation, or `Design Escalation` authority defect and its integrated
  evidence.

Return control to `agentic-engineering-workflow` after `TasksAccepted` or any
stop condition. Do not run feature integration verification or targeted review,
publish, merge, or choose branch disposition.
