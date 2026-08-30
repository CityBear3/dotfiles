---
name: dispatching-parallel-agents
description: Dispatch root-selected ready planned Task roles directly while preserving dependency readiness, isolated workspaces, one-writer ownership, and runtime backpressure.
---

# Dispatch parallel agents

Act as the root's scheduling adapter for Task phases already selected by
`execute-plan`, `execute-task`, `verify`, or `review`. Do not reinterpret the
Task DAG, PR topology, Task Contract, Review policy, selected role, or phase
transition. The root remains the sole orchestrator.

## Require ready work

For each selected Task role require:

- the exact Task and PR identity, current phase, purpose, and expected result;
- applicable authority and a complete role-specific message;
- isolated workspace, branch, planned base, current head, and source-state
  boundary;
- dependency readiness and ownership-disjoint concurrency evidence;
- whether this is a fresh dispatch, compatible follow-up, or replacement; and
- required result fields and stop conditions.

Do not run Tasks concurrently when they share a checkout or active writer,
overlap write ownership, mutate conflicting external state, depend on one
another's unfinished result, or require one evolving judgment. A PR stack edge
alone is not a logical dependency when the approved plan permits an early
candidate.

## Dispatch direct leaves

Pass each already-selected role to `agent-teams-driven-development`. Every new
implementer, verifier, reviewer, adversarial integrator, or review integrator
uses explicit `fork_turns="none"`, receives one complete role-specific handoff,
and may not spawn descendants. The root retains Task state, phase transitions,
Review selection, and acceptance authority.

Keep exactly one active writer in each Task workspace. Independent readers may
run concurrently only after their owning phase gate permits it. Removal of
workflow leases does not authorize overlapping implementation and verification,
duplicate verifiers, early findings integration, or correction before triage.

Runtime admission determines which spawn requests start. When a request is
rejected because the thread limit is reached, retain that selected role as
pending in approved ready-Task or reviewer order. After useful independent work
is exhausted, wait once for normally 300,000 to 600,000 milliseconds; mailbox,
completion, or user-input events may return earlier. Retry pending admission
after progress. Do not drop, replace, reorder, or weaken selected work merely to
fit current runtime availability, and do not classify thread pressure as a
Design escalation.

Use live-agent inspection when duplicate prevention, liveness, failure,
interruption, replacement, or teardown is decision-relevant. Do not require a
capacity snapshot before every ordinary dispatch.

## Integrate scheduling results

Validate each returned result against the exact Task workspace and requested
role. Re-resolve branch, planned base, merge base, head, range, diff, status,
writer activity, and attribution before a phase transition or dependency
release. Agent identity, memory, liveness, Herdr, lazygit, and pane state are
operational observations rather than acceptance evidence.

Use a compatible idle identity only with a fresh complete handoff and current
Git and authority validation. Start a replacement only after the prior writer
is inactive and all in-scope state is attributable; otherwise return `BLOCKED`
without cleaning or rewriting state.

Return the Task-to-role-to-leaf-to-workspace mapping, pending order, completion
or interruption state, role reports, directly observed Git evidence, and every
gap to the owning root phase. Do not release a dependency, decide Task or
Feature acceptance, publish, merge, or dispose of a workspace here.
