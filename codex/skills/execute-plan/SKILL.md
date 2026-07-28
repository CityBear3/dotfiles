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
- retained stable-key and attempt history;
- plan context limited to the plan path, task-specific decisions, non-goals, and
  file responsibilities.

Do not copy the complete task specification or Review policy into the plan
context. Invoke `execute-task` for the task and let it own the writer,
verification, commit, exact range, selected gate, correction, and retry
semantics.

Do not start a dependent task until its predecessor returns `Accepted`. On
`BLOCKED`, `Escalate`, plan deviation, missing evidence, or a task head that is
not the current repository head, stop and return the exact gap to
`agentic-engineering-workflow`.

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

## Return orchestration status

Return:

- `Accepted` only with all ordered accepted task records, the separate aggregate
  final HEAD, the full implementation range, and complete policy provenance;
- `BLOCKED` with partial ordered evidence when an operational prerequisite or
  current-state guarantee cannot be established;
- `Escalate` with the exact plan deviation, missing decision, policy conflict, or
  task escalation.

Return control to `agentic-engineering-workflow` after acceptance or any stop
condition. Do not start global `verify`, final review, publication, merge, or
branch disposition.
