---
name: agent-teams-driven-development
description: Schedule one already-selected Task writer or read-only check leaf directly from the root while preserving role boundaries, runtime backpressure, and interruption safety.
---

# Agent-teams driven development

Act only as the scheduling adapter for a role already selected by
`execute-task`, `execute-lightweight-task`, `verify`, or `review`. The root is
the sole workflow orchestrator for planned, lightweight, integration-only, and
standalone targets. Do not select paths, roles, Review breadth, findings,
corrections, or acceptance here.

## Require one bounded request

Accept:

- one named role or complete fallback contract;
- one complete role-specific message prepared by the owning phase;
- the exact Task PR, integration, or standalone target and workspace;
- whether the request is fresh, follow-up, or replacement;
- prior identity and interruption evidence when applicable; and
- the selected-role order and required output contract.

Reject an ambiguous role or a request that requires policy interpretation. Pass
the selected role and message without adding another workflow wrapper.

## Dispatch under runtime admission

Every new implementer, verifier, reviewer, adversarial integrator, or review
integrator uses explicit `fork_turns="none"` and may not spawn descendants. A
compatible idle identity uses `followup_task` with a fresh complete handoff and
fresh validation. Parent conversation, identity, and liveness are never
correctness evidence.

Keep one active writer for a Task workspace. Other writers may run only in
approved separate checkouts with ownership-disjoint responsibilities. Reviewers
and integrators remain read-only; the verifier remains check-only and may create
only normal ignored test or build artifacts.

Attempt the selected spawn without workflow-level lease, grant, or capacity
arithmetic. When the runtime reports its thread limit, retain the role as
pending in the supplied order. After useful independent work is exhausted, wait
once for normally 300,000 to 600,000 milliseconds and retry after a mailbox or
completion event. Do not reduce Review breadth, substitute a role, or convert
temporary resource pressure into policy `Escalate`. Return operational
`BLOCKED` only after repeated non-progress prevents required work from
advancing.

Use `list_agents` only when duplicate prevention, liveness, failure,
interruption, replacement, or teardown is decision-relevant. Runtime admission
owns concurrency; the workflow still owns which roles and phase transitions are
valid.

## Preserve role boundaries

- Give an implementer its owned responsibility, applicable authority, material
  property and discipline, exact workspace and Git target, commit intent,
  focused writer checks, and one-writer boundary.
- Give a verifier the exact target and current-head Verification Matrix. It is
  check-only and returns `PASS`, `FAIL`, or `BLOCKED` without semantic review.
- Give a reviewer the unchanged verified target, selected perspective, Review
  context and policy, completed matrix, and relevant prior triage.
- Give an integrator the unchanged target, complete applicable source reports,
  authority, Review context and policy, and prior triage. It remains read-only
  and does not invent findings or issue final workflow classifications.

Keep exact sources directly readable. Do not copy unrelated topology,
scheduling, completed-gate, or authority prose into every message.

## Recover safely

After an implementer interruption, timeout, lost response, partial edit, or
partial commit:

1. inspect the interruption and live identity;
2. prove the prior writer is inactive;
3. inspect branch, planned base, head, status, commits, and exact diff; and
4. attribute every in-scope edit and commit to the Task.

Resume or replace only when writer isolation, attribution, and the handoff still
hold. Never clean, reset, recommit, or discard uncertain state to force
progress. For a failed read-only role, preserve completed reports and retry only
the already-selected compatible role.

## Return scheduling evidence

Return target, role, leaf identity, workspace, pending order, completion or
interruption state, report, and inspected Git evidence after writer failure.
Use `BLOCKED` when safe scheduling or attribution cannot be established.

The invoking phase interprets results and owns corrections and acceptance. Do
not release dependencies, publish, merge, or tear down a workspace.
