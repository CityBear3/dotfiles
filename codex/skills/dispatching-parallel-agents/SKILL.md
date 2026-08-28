---
name: dispatching-parallel-agents
description: Dispatch root-selected ready planned Tasks to dedicated Task orchestrators while respecting approved dependencies, isolated workspaces, root leases, and shared-state constraints.
---

# Dispatch parallel agents

Parallelize only independent planned Task loops. When invoked from
`execute-plan`, act as the root scheduling adapter for Tasks that it already
marked ready; do not reinterpret the Task DAG, PR topology, ownership, Review
policy, or selected leaf roles. Dispatch one exact `task-orchestrator` profile
per selected Task. This skill does not dispatch planned implementer, verifier,
reviewer, or integrator leaves; the bound Task orchestrator owns those
dispatches. This topology applies to new-format planned work; eligible legacy
plans retain their exact approved scheduling authority.

## Decide whether to dispatch

For each candidate task identify:

- exact scope and expected output;
- files or external state it may read or write;
- required context;
- dependencies on other tasks;
- approved branch and checkout, planned PR base, and candidate or authoritative
  mode when applicable;
- completion evidence;
- configured, observed, and effective subagent capacity, current live
  identities, the root-granted leaf count, and any already-selected roles for
  the wave.

Do not parallelize tasks that edit the same files, share a checkout or active
writer, mutate conflicting state, depend on one another's results, lack an
approved workspace, or require a single evolving judgment. A PR stack edge alone
is not a logical dependency when the approved plan permits an early candidate.

## Capacity

Use `list_agents` before every dispatch wave. Effective subagent capacity is the
lower of configured `agents.max_threads` and currently observed runtime
capacity. `max_threads` excludes the root and counts every live Task
orchestrator and leaf across the complete tree. Never infer that every
configured slot is currently available.

The root grants leases. A new Task orchestrator consumes one slot and must also
receive capacity for its baseline one-leaf grant. Do not dispatch it when those
two slots are unavailable. That baseline is the Task's only leaf outside the
source-reviewer wave. Only after fresh verifier `PASS` and selection of at least
two independent source reviewers may an orchestrator request temporary
expansion. The root may grant at most three total Task leaves or the smaller
current capacity, only to the selected source reviewers, in the approved
deterministic queue order. Revoke the expansion when the reviewer wave ends or
exits for priority authority assessment, before integration, triage, or
correction. A free slot is availability, not permission. Count writers and
reviewers from every active Task PR. Queue ready Tasks or selected roles rather
than dropping, reordering, substituting, or oversubscribing them.

## Dispatch

Call `spawn_agent` with the exact `task-orchestrator` profile once per concrete
selected Task. Use a stable, descriptive Task name, bind the returned identity
to that Task Contract for its lifetime, and include all task-local context
because conversation memory is not authority.

Each prompt states:

- goal and boundaries;
- working directory and relevant files;
- exact task branch, checkout, planned PR base, and mode;
- whether writes are allowed;
- prohibited overlap;
- required commands or evidence;
- exact return format;
- the current root-granted leaf count and selected-role queue;
- authority to spawn only policy-selected bounded leaves inside that grant,
  with every such leaf prohibited from spawning descendants, the baseline leaf
  used serially, and any temporary expansion reserved for the source-reviewer
  wave and revoked before later phases.

Continue useful lead work while agents run. Use bounded `wait_agent` calls and `list_agents` for regular status checks.

## Integrate

Validate each result against the requested output and its exact workspace. Check
for conflicting edits, branches, state, or assumptions. Return candidate and
authoritative results unchanged to the owning scheduler. The root directly
re-resolves branch, planned base, merge base, head, range, diff, and status
before dependency release or Feature aggregation. Agent identity, liveness,
Herdr, lazygit, and pane state are not acceptance evidence. Do not release a
dependency or claim feature completion here. If the same idle Task orchestrator
needs an attributable re-entry, use `followup_task`; use `send_message` for
information that should not start a new turn.

`Candidate`, `Accepted`, `BLOCKED`, and `Escalate` end the orchestrator's current
turn. Never reassign that identity to another Task. Prefer it for a fresh
handoff on candidate, stale, correction, or human-review re-entry when available.
If unavailable, dispatch a replacement only after the prior writer is inactive
and all Task state is attributable; otherwise preserve state and return
`BLOCKED`. A returned or idle identity reserves no leaf lease, but any identity
still observed live counts against capacity.

Synthesize results once. Do not repeat completed work merely because agents ran independently.
