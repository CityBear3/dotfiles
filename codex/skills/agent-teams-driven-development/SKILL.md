---
name: agent-teams-driven-development
description: Schedule one Task PR writer, an already-selected verifier, or already-selected read-only reviewers while enforcing global capacity, queues, and interruption safety.
---

# Agent-teams driven development

Act only as the scheduling adapter for the writer or verifier selected by
`execute-task`, a verifier selected by `verify`, or the reviewers selected by
`review`. For new-format planned work, the bound Task orchestrator invokes this
adapter and schedules only its leaves under the current root-granted lease. For
lightweight work or another root-owned coordinator check target, the root
invokes it directly. Every dispatched writer, verifier, reviewer, or integrator
is a leaf and never spawns descendants.
Eligible legacy work retains its exact approved invoking context. Do not select
workflow paths, Review context, review modes, role breadth, severity mappings,
Acceptance, correction semantics, task commits, or task acceptance here.

## Require a bounded scheduling request

Accept from the invoking `execute-task`, `verify`, or `review` phase:

- one already-selected named role or fallback contract;
- the complete contract-aware writer, verifier, or reviewer message already
  prepared by that phase;
- whether the request is a fresh dispatch, follow-up, or replacement;
- any prior agent identity, interruption result, and observed Git state;
- execution context: planned Task orchestrator, lightweight root, or another
  root-owned coordinator check target;
- configured, observed, and effective subagent capacity, all relevant live
  identities, the root-granted leaf count for this Task loop, and the ordered
  selected-role queue.

Reject an unresolved or ambiguous role, or a request that requires task or policy
interpretation. Pass the selected role and message unchanged; do not load prompts
or add another wrapper here.

## Enforce live capacity and a deterministic queue

Call `list_agents` before every dispatch wave. Set effective subagent capacity
to the lower of configured `agents.max_threads` and currently observed runtime
capacity. `max_threads` excludes the root and counts all live subagents in the
complete tree, including Task orchestrators and every leaf. Do not add a
separate hard-coded total-thread ceiling.

Only the root grants or expands a lease. A Task loop normally receives one leaf
slot and may use at most three concurrent leaves or its smaller current grant.
For new-format planned work, reject a self-inferred expansion by the Task
orchestrator even when another runtime slot appears free. For lightweight work,
apply the same per-loop leaf limit to the root-owned loop. The global scheduler
first grants one leaf to each schedulable active Task when possible, then
allocates spare slots in the approved deterministic queue order.

Queue already-selected roles in request order when available slots are
insufficient. Record configured, observed, and effective capacity, live agent
identities, invoking Task-loop context, granted leaf count, queued roles,
dispatch order, and every capacity gap. Do not reduce, replace, or reorder
selected roles to fit capacity.

Allow no more than one implementer and one active writer for the supplied task
workspace. Other `execute-task` calls may have writers only in separate approved
checkouts with ownership-disjoint tasks. Count every live Task orchestrator,
task writer, verifier, reviewer, and integrator against the same effective
capacity. Every reviewer is read-only. Independent check-only or read-only
leaves may run concurrently when the grant permits; implementation and
correction retain one writer. Otherwise queue them without changing their
independence or contracts.

If a required role cannot be instantiated or the queue cannot make progress,
return `BLOCKED` with observed availability evidence. Do not turn a runtime
shortage into policy `Escalate` or substitute the lead or another perspective.

## Schedule and observe

Dispatch only the already-selected leaf role using its resolved named profile
or complete fallback contract. Tell every leaf not to spawn descendants. Tell an
implementer the exact authority identity and currentness evidence plus one of:
assigned Feature clauses and Task Contract, exact eligible legacy authority and
owned responsibility, or approved promotion-reconciliation authority. Also pass
commit intent, any contractually fixed files, and that it is the only writer.
Tell a verifier it is check-only, may write only normal ignored test or build
artifacts, must not mutate the index, tracked files, or in-scope source, and may
run only documented non-mutating format checks. Pass the exact target,
authority, required commands and expected observations, current Git snapshot,
and required `PASS`, `FAIL`, or `BLOCKED` evidence.
Tell a reviewer it is read-only and must inspect the supplied authority and
exact Task PR, integration-only composition, eligible legacy range, or
standalone target. Keep full sources directly available without copying
unrelated unchanged prose into each message.

After a successful dispatch, return the mapping between Task PR identity,
Task-loop owner, returned leaf identity, and exact Task workspace. The root
remains the source of truth for global agent identity, capacity, follow-up,
interruption, waiting, and closure. A Herdr workspace or lazygit pane may expose
Git state to the engineer, but it is not an agent session and supplies no
scheduling, verification, or acceptance evidence.

Use bounded waits, inspect live agents regularly, and return progress evidence
to the invoking Task-loop owner. Preserve reports, identities, completion state,
and observed errors without translating findings or deciding whether the task
passed.

Return every response unchanged with the observed agent completion state.
`execute-task` validates writer status, mode, report fields, commit, planned
base, current head, range, and verification. `review` validates reviewer output
against its unchanged target and policy. This adapter never promotes a
candidate, integrates findings, or claims task or feature acceptance.

## Inspect interruption state before resuming

After an implementer interruption, failure, timeout, lost response, incomplete
report, partial edit, or partial commit:

1. inspect the interruption result and live-agent state;
2. confirm the prior writer is inactive;
3. inspect the task workspace branch, planned base, HEAD, status, commits, and
   exact PR diff;
4. determine whether every in-scope edit and commit is attributable to the task.

Return that evidence to `execute-task` before another writer is dispatched.
Resume or replace only after a fresh request confirms that no writer overlaps,
the state is safe and attributable, and the handoff still applies. Otherwise
preserve state and return `BLOCKED`; never rewrite or discard state to force
progress.

For reviewer failure, preserve completed read-only reports, recheck live
capacity, and queue only the already-requested replacement role. If the required
independent role remains unavailable, return `BLOCKED`; do not substitute
another perspective.

For verifier failure, preserve its check-only report and target snapshot,
recheck live capacity, and queue only the same already-requested verifier when
`verify` requests replacement. If that compatible verifier remains unavailable,
return `BLOCKED`; do not substitute the root, Task orchestrator, or another
role.

## Return scheduling evidence

Return dispatch and queue order, the
Task-PR-to-Task-loop-owner-to-leaf-to-workspace mapping, agent identities,
configured, observed, and effective capacity, the root grant, completion or
interruption states, reports, inspected Git state after writer failure, and
every availability or attribution gap.

Use `BLOCKED` whenever safe scheduling or writer-state attribution cannot be
established. Otherwise return scheduling evidence to the invoking phase.
`execute-task` alone interprets writer results and manages corrections; `review`
alone integrates the requested reviewer results and returns its gate verdict.
Do not release a dependency, decide task or feature acceptance, publish, merge,
or tear down a workspace.
