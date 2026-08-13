---
name: execute-plan
description: Orchestrate an approved implementation plan by validating it, invoking execute-task in dependency order, and aggregating exact task evidence.
---

# Execute an approved plan

Own approved-plan validation, dependency ordering, per-task handoff, ordered
evidence aggregation, and plan-deviation detection. Do not edit files, dispatch
implementers or reviewers, select prompts, normalize findings, or run global
verification or final review from this skill.

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

For compatibility, accept an approved plan already executing before the
contract-centered format only when the coordinator supplies its exact approval
and in-flight evidence, referenced Design Doc or decision sources, unchanged
task specifications, Review context and policy, verification and completion
criteria, and confirmation that no material ambiguity exists and the owner did
not choose migration. Keep that legacy plan as the authority; do not manufacture
Feature or Task Contract files merely to satisfy the new shape.

Record the original plan implementation base and the current head. On re-entry,
also retain every already accepted task with its exact base, head, range, commit,
verification, gate result, and gaps. Do not widen an earlier task range when a
later task adds commits.

Stop and return a plan deviation when implementation would require a new
architecture, goal, scope, responsibility owner, public or shared interface
semantic, invariant, failure behavior, compatibility promise, verification
obligation, schema, error model, policy, or authority decision. The coordinator
owns the transition to the affected Design Doc, Feature Contract, or
Implementation Plan approval gate.

## Materialize ordered task handoffs

Resolve the dependency graph before executing anything. Use its deterministic
order and run tasks sequentially; this contract does not authorize parallel plan
tasks or more than one active writer.

For each ready task, give `execute-task` one concise plain-language handoff:

- the approved Feature Contract and clauses assigned to this task;
- the exact applicable Task Contract, including purpose, expected result,
  constraints, dependencies, non-goals, and delegated local decisions;
- applicable shared interface contracts and adjacent-task obligations;
- the Review context and complete Review policy;
- the declared discipline and applicable repository guidance;
- the working directory and approved workspace;
- the exact task base, which is the current head before this task;
- responsibility and ownership boundaries;
- verification routes and observable obligations;
- the responsibility-scoped commit intent and its fixed message or the approved
  writer authority to select that message;
- contractually significant files, signatures, ordering, and exact commands
  only when the approved plan fixes them.

For an eligible legacy task, pass the approved legacy task specification and its
referenced design sources as the explicit authority, plus the same workspace,
base, discipline, verification, review, commit, and evidence fields available in
that plan. Do not relabel it as a new Feature or Task Contract. Stop if a missing
field creates material ambiguity; do not force migration or infer a decision.

Invoke `execute-task` once for the task and let it own the writer, verification,
commit, exact range, policy-selected gate, correction, and stop condition.

Do not start a dependent task until its predecessor returns `Accepted`. On
`BLOCKED`, `Escalate`, plan deviation, missing evidence, or a returned head that
is not the current repository head, preserve the observed state and return the
exact gap to `agentic-engineering-workflow`.

## Resume only attributable work

After an interrupted or incomplete task, retain the last accepted aggregate
separately from the observed in-flight work. Before resuming:

1. confirm through the scheduling result that the prior writer is inactive and
   no writer overlaps;
2. inspect current HEAD, status, commits, and the task-base-to-current diff;
3. confirm the observed edits and commits are attributable to that task and
   descend from its task base;
4. confirm the unchanged task handoff still applies.

Resume the unfinished work or pending read-only gate only when all four checks
pass. If state is uncertain, mismatched, or unattributable, do not clean, reset,
recommit, or dispatch a replacement. Return `BLOCKED` with the observed agent and
Git state plus the exact condition required for re-entry. Use `Escalate` only
when resumption needs a material decision, scope, policy, or authority change.

An interrupted task remains unaccepted. Do not add its commits or range to the
accepted task results, recalculate the aggregate, or release a dependent task
until `execute-task` returns current acceptance evidence.

## Re-enter for a planned correction

Treat an authorized correction as one concrete plan step after the previously
accepted tasks. Preserve the original implementation base and prior task ranges.
Give `execute-task` the exact finding or failed observation, approved correction,
observed attempts and results, unchanged Feature and Task Contracts with shared
interfaces or unchanged eligible legacy authority, Review context, Review
policy, last accepted head as the correction task base, responsibility
boundaries, verification obligations, and a correction commit intent bounded to
the finding with its fixed message or approved writer message-selection
authority.

When the same concrete problem repeats without progress, or the next action would
repeat an observed failed correction, stop and return the attempt evidence. Do
not invent another tracking protocol or silently expand the correction.

After `Accepted`, append the correction once with its commit, exact base, current
head, range, fresh verification, gate result, and gaps. Recalculate the aggregate
from the original implementation base without rewriting prior task ranges.

## Aggregate accepted tasks

After each accepted task, append an ordered result containing:

- task name and dependency position;
- Feature Contract clauses and Task Contract obligations, or eligible legacy
  completion criteria, proved;
- exact task base, accepted current head, and base-to-head range;
- task and correction commits;
- fresh verification obligations, commands selected or required, and observed
  results;
- per-task gate result;
- changed files, concerns, and gaps.

After every planned task is accepted:

1. retain the complete ordered task results;
2. record the distinct aggregate final current head;
3. calculate the full implementation range from the original plan base;
4. confirm the aggregate head is current and each task still identifies its own
   exact accepted range;
5. report plan deviations, correction attempts, and residual gaps separately
   from successful evidence.

The aggregate range supports global verification and final review. It never
replaces a task-specific reviewed range.

## Return orchestration status

Return:

- `Accepted` only with every ordered accepted task result, aggregate current
  head, full implementation range, and either the Feature Contract with complete
  Task Contract coverage and integration-only obligations or the exact eligible
  legacy authority, plus Review context and complete Review policy;
- `BLOCKED` with the last accepted aggregate, observed in-flight agent and Git
  state, gaps, and exact re-entry condition;
- `Escalate` with the exact plan deviation, missing decision, policy conflict, or
  task escalation.

Return control to `agentic-engineering-workflow` after acceptance or any stop
condition. Do not start global verification, final review, publication, merge, or
branch disposition.
