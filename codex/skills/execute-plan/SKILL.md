---
name: execute-plan
description: Orchestrate an approved implementation plan by validating it, invoking execute-task in dependency order, and aggregating exact task evidence.
---

# Execute an approved plan

Own approved-plan validation, dependency ordering, canonical per-task handoff,
ordered evidence aggregation, and plan-deviation detection. Do not edit files,
dispatch implementers or reviewers, select prompts, normalize findings, or run
global verification or final review from this skill.

## Validate plan entry

Before execution, require:

- an approved, current implementation plan;
- its complete approved Review policy and provenance;
- a suitable non-default feature branch or approved workspace;
- understood working-tree state;
- an explicit implementation discipline and exact verification for every task;
- settled dependencies, file responsibilities, decisions, and non-goals.

For initial execution or any re-entry, preserve separately:

- the original plan implementation base;
- the last-accepted ordered task map with every immutable exact range;
- the last-accepted aggregate final HEAD and full implementation range;
- at most one in-flight partial task record with its canonical context identity,
  lifecycle phase, task base, partial head, commits, evidence, writer state,
  pending gate, gaps, and exact re-entry condition.

Without an in-flight partial record, require repository HEAD to equal the
last-accepted aggregate head. With one, permit repository HEAD to equal the
attributable validated partial head only when its task base equals the
last-accepted aggregate head, its commits descend from that base, and its
HEAD/status/diff ownership is established. An unattributed, non-descendant,
duplicated, or mismatched partial state returns `BLOCKED`.

For correction re-entry, additionally require:

- the concrete authorized correction step tied to its failed command or review
  finding;
- retained stable-key and attempt history.

Validate that the complete policy records mode, rationale, risk surfaces, its
mode-consistent per-task gate, final required reviewers with reasons, conditional
reviewers with triggers and reasons, skipped perspectives with reasons, residual
risk, configured and observed capacity plus queue rules, Acceptance, and
provenance. Reject missing, stale, contradictory, or mode-inconsistent fields
rather than inferring them.

Stop and return a plan deviation when implementation would require a new
architecture, scope, public-contract, schema, error-model, policy, authority, or
file-responsibility decision. The coordinator owns the transition needed to
resolve it.

## Materialize ordered task handoffs

Resolve the plan's dependency graph before executing anything. Run tasks
sequentially in dependency order; this contract does not authorize parallel task
execution. Record the original plan implementation base before the first task.

For each ready task, build one canonical `execute-task` context containing:

- the complete task specification;
- the original decision source and non-goals;
- task discipline, workspace, and working directory;
- the exact task base commit, which is the current head before that task;
- exact verification commands and expected results;
- the complete active Review policy and provenance;
- configured and observed capacity plus queue rules;
- plan context limited to the plan path, task-specific decisions, non-goals, and
  file responsibilities.

Do not copy the complete task specification or Review policy into the plan
context. Carry retained stable-key and attempt history only in the mutable task
record alongside the canonical context. Invoke `execute-task` for the task and
let it own the writer, verification, commit, exact range, selected gate,
correction, and retry semantics.

Do not start a dependent task until its predecessor returns `Accepted`. On
`BLOCKED`, `Escalate`, plan deviation, missing evidence, or a task head that is
not the current repository head, stop and return the exact gap to
`agentic-engineering-workflow`.

## Preserve one in-flight partial task

When an `execute-task` invocation first returns `BLOCKED` with attributable task
state, validate and retain its returned partial task record as the sole in-flight
partial. Keep the last-accepted task map and aggregate unchanged.

Pass the in-flight partial record and lifecycle phase back to `execute-task` with
the same canonical context identity. Do not add its commits or range to the
accepted task map, recalculate the accepted aggregate, or advance a dependency
while its status is `BLOCKED` or otherwise not `Accepted`.

When `execute-task` returns `Accepted`, validate the returned head, range,
context identity, and completion of the pending gate. Append that task record
exactly once, clear the in-flight partial, and only then recalculate the aggregate
and release dependent tasks. Reject an already-appended task identity or commit
instead of duplicating it.

On a partial-state mismatch, retain the last-accepted aggregate unchanged and
return `BLOCKED` with the partial record, observed HEAD/status/diff, ownership
evidence, and exact condition required for re-entry. Return `Escalate` only when
resumption requires a material decision, scope, policy, or authority change.

## Re-enter for a planned correction

Treat an authorized planned correction as a new concrete plan step after the
previously accepted ordered tasks, not as a standalone lightweight task. Preserve
the original implementation base and prior records. Build the correction's
canonical task context with the complete correction task, causal hypothesis,
authorized action, exact verification, last-accepted aggregate head as task base,
and the unchanged approved policy. Carry its stable key and complete attempt
history in the mutable task record outside the canonical context. When a partial
correction record already exists, pass that same record and lifecycle phase back
to `execute-task` rather than rebuilding or recommitting it.

Invoke `execute-task` for the correction. When it returns `Accepted`, append its
task identifier, correction commits, exact correction base/head/range,
verification, gate, normalization, and retry evidence to the ordered task map.
Append once, then recalculate the aggregate. Do not rewrite prior task ranges or
discard prior retry history.

When the correction returns `BLOCKED` or `Escalate`, retain the previous
last-accepted aggregate separately from the one in-flight partial correction
record, including its lifecycle phase and exact re-entry condition. Do not append
the partial record, advance dependencies, or report the last-accepted aggregate
as acceptance of the current correction.

## Preserve task records and aggregate separately

After each accepted task, append an ordered immutable record containing:

- task identifier and dependency position;
- exact task base and accepted head;
- exact task base-to-head range;
- task and fix commits;
- exact verification evidence;
- per-task gate result and normalization evidence;
- stable-key retry history;
- complete policy provenance and any recorded operational gaps.

Never widen or replace an accepted task range when later tasks add commits. The
next task uses the previous accepted head as its new base, while the earlier
record remains unchanged.

After every planned task is accepted:

1. retain the complete ordered task-range map;
2. record the distinct aggregate final HEAD;
3. calculate the full implementation range from the original plan
   implementation base to that aggregate final HEAD;
4. confirm the aggregate head is current and each task record still identifies
   its own exact accepted range;
5. report plan deviations, policy gaps, retries, and residual gaps separately
   from successful evidence.

The aggregate range supports the next cross-phase check; it does not become a
replacement per-task review range.

After an accepted correction step, repeat the same aggregation from the original
plan implementation base through the correction head. Return the updated ordered
task map, aggregate final HEAD, and full implementation range. Global
verification and final review must target that full updated range; the
correction's exact task range remains separate task evidence and never shrinks
the global target.

## Return orchestration status

Return:

- `Accepted` only with all ordered accepted task records, the separate aggregate
  final HEAD, the full implementation range, and complete policy provenance;
- `BLOCKED` with partial ordered evidence when an operational prerequisite or
  current-state guarantee cannot be established, including the separate
  last-accepted aggregate, one in-flight partial record, lifecycle phase, and
  exact re-entry condition;
- `Escalate` with the exact plan deviation, missing decision, policy conflict, or
  task escalation.

Return control to `agentic-engineering-workflow` after acceptance or any stop
condition. Do not start global `verify`, final review, publication, merge, or
branch disposition.
