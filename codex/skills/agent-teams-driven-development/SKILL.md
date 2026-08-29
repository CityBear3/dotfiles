---
name: agent-teams-driven-development
description: Schedule one Task PR writer or already-selected read-only check leaves while enforcing execution context, global capacity, queues, and interruption safety.
---

# Agent-teams driven development

Act only as the scheduling adapter for the writer selected by a Task executor
(`execute-task` or `execute-lightweight-task`), a verifier selected by `verify`,
or reviewers and integrators selected by `review`. For new-format planned work,
the bound Task orchestrator invokes this adapter and schedules only its leaves
under the current root-granted lease. For lightweight work, a standalone target,
or another root-owned coordinator check target, the root invokes it directly.
Every dispatched writer, verifier, reviewer, or integrator is a leaf and never
spawns descendants.
Eligible legacy work retains its exact approved invoking context. Do not select
workflow paths, Review context, review modes, role breadth, severity mappings,
Acceptance, correction semantics, task commits, or task acceptance here.

## Require a bounded scheduling request

Accept from the invoking Task executor, `verify`, or `review` phase:

- one already-selected named role or fallback contract;
- the complete contract-aware writer, verifier, reviewer, or integrator message
  already prepared by that phase;
- whether the request is a fresh dispatch, follow-up, or replacement;
- any prior agent identity, interruption result, and observed Git state;
- exactly one execution context: the bound planned Task orchestrator, the
  root-owned lightweight Task loop, a root-owned standalone target, another
  root-owned coordinator check target, or the exact eligible legacy context;
- configured, observed, and effective subagent capacity, all relevant live
  identities, the context-local root grant, and the ordered selected-role
  queue.

Reject an unresolved or ambiguous role, or a request that requires task or policy
interpretation. Pass the selected role and message unchanged; do not load prompts
or add another wrapper here.

## Enforce live capacity and a deterministic queue

Call `list_agents` before every dispatch wave. Set effective subagent capacity
to the lower of configured `agents.max_threads` and currently observed runtime
capacity. `max_threads` excludes the root and counts all live subagents in the
complete tree, including Task orchestrators and every leaf. Do not add a
separate hard-coded total-thread ceiling.

Only the root grants or expands a lease. A Task loop starts with one baseline
leaf and uses it serially for implementation, verification, findings
integration, triage, and correction. Temporary expansion is allowed only for a
policy-selected source-reviewer wave after fresh verification `PASS` and only
when at least two independent source reviewers were selected. For new-format
planned work, reject a self-inferred expansion by the Task orchestrator even
when another runtime slot appears free. For lightweight work, apply the same
phase rule to the root-owned loop. The root may grant at most three total Task
leaves or the smaller current capacity; only the selected source reviewers use
the expansion. Revoke it when that wave completes or exits for priority
authority assessment, before any findings integration, triage, or correction.
Free capacity is availability, not authority.

A standalone target has a separate root-granted target-local count of normally
one and at most three concurrent leaves; it has no Task lease or Task authority.
The global scheduler first grants one baseline leaf to each schedulable active
Task when possible, then considers eligible reviewer-wave requests in approved
deterministic queue order. A standalone request uses only its current
target-local grant and never infers permission to consume every globally free
slot.

Queue already-selected roles in request order when available slots are
insufficient. Record configured, observed, and effective capacity, live agent
identities, execution context, context-local grant, queued roles, dispatch
order, and every capacity gap. Do not reduce, replace, or reorder selected roles
to fit capacity.

Allow no more than one implementer and one active writer for the supplied task
workspace. Other Task executor calls may have writers only in separate approved
checkouts with ownership-disjoint tasks. Count every live Task orchestrator,
task writer, verifier, reviewer, and integrator against the same effective
capacity. Every reviewer is read-only. Only policy-selected source reviewers
may run concurrently under the temporary expansion; all other Task phases use
the baseline leaf. Otherwise queue them without changing their independence or
contracts.

If a required role cannot be instantiated or the queue cannot make progress,
return `BLOCKED` with observed availability evidence. Do not turn a runtime
shortage into policy `Escalate` or substitute the lead or another perspective.

## Schedule and observe

Dispatch only the already-selected leaf role using its resolved named profile
or complete fallback contract. Create every newly spawned implementer,
verifier, reviewer, adversarial-integrator, or review-integrator with explicit
`fork_turns="none"` on its `spawn_agent` call. If the runtime cannot establish no-history creation,
return `BLOCKED` instead of inheriting parent turns. Pass the selected role and
complete role-specific message unchanged, and tell every leaf not to spawn
descendants. Require each recipient to directly re-resolve current Git and
authority from the supplied exact sources; parent conversation, identity, and
liveness are never proof. An existing idle identity uses `followup_task` with a
fresh complete role message and fresh Git and authority validation; it is not a
new spawn. Tell an
implementer the exact authority identity and currentness evidence plus one of:
assigned Feature clauses and Task Contract, exact eligible legacy authority and
owned responsibility, or approved promotion-reconciliation authority. Also pass
commit intent, any contractually fixed files, and that it is the only writer.
Tell a verifier it is check-only, may write only normal ignored test or build
artifacts, must not mutate the index, tracked files, or in-scope source, and may
run only documented non-mutating format checks. Pass the exact target, the
completed-input current-head Verification Matrix, command-environment facts,
mutation boundary, and required completed-matrix `PASS`, `FAIL`, or `BLOCKED`
evidence. Do not add the complete Review policy unless an exact policy
constraint changes its route.
Tell a reviewer it is read-only and must inspect the supplied authority and
exact Task PR, integration-only composition, eligible legacy range, or
standalone target. Pass its perspective, Review context and policy, completed
Verification Matrix, and relevant prior triage. For correction review also pass
`H1`, `H2`, full `base..H2`, the `H1..H2` delta, corrected finding, and prior
report and triage. `review` owns correction-review scope and escalation; pass
that selected scope instead of reproducing its rules. Keep full sources directly
available without copying unrelated unchanged prose into each message.
Tell `adversarial-integrator` or `review-integrator` that it is read-only, must
use the supplied unchanged target and complete input reports, and may not invent
findings, classify final workflow authority, dispatch descendants, or mutate
source, Git, or authority artifacts.

After a successful dispatch, return the mapping between the Task PR,
standalone, or coordinator target identity; its Task-loop or root owner; the
returned leaf identity; and the exact workspace. The root remains the source of
truth for global agent identity, capacity, follow-up, interruption, waiting, and
closure. A Herdr workspace or lazygit pane may expose Git state to the engineer,
but it is not an agent session and supplies no scheduling, verification, or
acceptance evidence.

After useful independent work is exhausted, use one bounded `wait_agent` call
of normally 300,000 to 600,000 milliseconds. It returns early on mailbox,
completion, or steered user input; do not replace it with repeated short polls.
Use a shorter bound only for a nearer explicit deadline, teardown, or
interruption boundary and record the reason. Inspect live agents at dispatch
and phase boundaries and after an early return, then return progress evidence
to the invoking Task-loop or root owner. Preserve reports, identities,
completion state, and observed errors without translating findings or deciding
whether the task passed.

Return every response unchanged with the observed agent completion state.
The invoking Task executor validates writer status, mode, report fields, commit,
base, current head, range, and verification. `review` validates reviewer output
against its unchanged target and policy. This adapter never promotes a
candidate, integrates findings itself, or claims task or feature acceptance.

## Inspect interruption state before resuming

After an implementer interruption, failure, timeout, lost response, incomplete
report, partial edit, or partial commit:

1. inspect the interruption result and live-agent state;
2. confirm the prior writer is inactive;
3. inspect the task workspace branch, planned base, HEAD, status, commits, and
   exact PR diff;
4. determine whether every in-scope edit and commit is attributable to the task.

Return that evidence to the invoking Task executor before another writer is
dispatched.
Resume or replace only after a fresh request confirms that no writer overlaps,
the state is safe and attributable, and the handoff still applies. Otherwise
preserve state and return `BLOCKED`; never rewrite or discard state to force
progress.

For reviewer or integrator failure, preserve completed read-only reports,
recheck live capacity, and queue only the already-requested replacement role. If
the required independent role remains unavailable, return `BLOCKED`; do not
substitute another perspective.

For verifier failure, preserve its check-only report and target snapshot,
recheck live capacity, and queue only the same already-requested verifier when
`verify` requests replacement. If that compatible verifier remains unavailable,
return `BLOCKED`; do not substitute the root, Task orchestrator, or another
role.

## Return scheduling evidence

Return dispatch and queue order, the target-to-owner-to-leaf-to-workspace
mapping, execution context, agent identities, configured, observed, and
effective capacity, the context-local root grant, completion or interruption
states, reports, inspected Git state after writer failure, and every
availability or attribution gap.

Use `BLOCKED` whenever safe scheduling or writer-state attribution cannot be
established. Otherwise return scheduling evidence to the invoking phase.
The invoking Task executor alone interprets writer results and manages
corrections; `review` alone orchestrates integration of the requested reviewer
results and returns its gate verdict.
Do not release a dependency, decide task or feature acceptance, publish, merge,
or tear down a workspace.
