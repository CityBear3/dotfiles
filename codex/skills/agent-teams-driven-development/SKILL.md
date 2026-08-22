---
name: agent-teams-driven-development
description: Schedule one Task PR writer or already-selected read-only reviewers while enforcing global capacity, queues, and interruption safety.
---

# Agent-teams driven development

Act only as the scheduling adapter for the writer selected by `execute-task` or
the reviewers selected by `review`. The lead schedules all work, and subagents
never spawn descendants. Do not select workflow paths, Review context, review
modes, role breadth, severity mappings, Acceptance, correction semantics, task
commits, or task acceptance here.

## Require a bounded scheduling request

Accept from the invoking `execute-task` or `review` phase:

- one already-selected named role or fallback contract;
- the complete contract-aware writer or reviewer message already prepared by
  that phase;
- whether the request is a fresh dispatch, follow-up, or replacement;
- any prior agent identity, interruption result, and observed Git state.

Reject an unresolved or ambiguous role, or a request that requires task or policy
interpretation. Pass the selected role and message unchanged; do not load prompts
or add another wrapper here.

## Enforce live capacity and a deterministic queue

Call `list_agents` before every dispatch wave. Set effective capacity to the
lower of configured and currently observed runtime capacity, count the lead, and
never exceed six total threads.

Queue already-selected roles in request order when available slots are
insufficient. Record configured, observed, and effective capacity, live agent
identities, queued roles, dispatch order, and every capacity gap. Do not reduce,
replace, or reorder selected roles to fit capacity.

Allow no more than one implementer and one active writer for the supplied task
workspace. Other `execute-task` calls may have writers only in separate approved
checkouts with ownership-disjoint tasks. Count every live task writer and
reviewer against the same effective capacity. Every reviewer is read-only.
Independent reviewers may run concurrently when capacity permits; otherwise
queue them without changing their independence or contracts.

If a required role cannot be instantiated or the queue cannot make progress,
return `BLOCKED` with observed availability evidence. Do not turn a runtime
shortage into policy `Escalate` or substitute the lead or another perspective.

## Schedule and observe

Dispatch only the already-selected role using its resolved named profile or
complete fallback contract. Tell every agent not to spawn descendants. Tell an
implementer the exact authority identity and currentness evidence plus one of:
assigned Feature clauses and Task Contract, exact eligible legacy authority and
owned responsibility, or approved promotion-reconciliation authority. Also pass
commit intent, any contractually fixed files, and that it is the only writer.
Tell a reviewer it is read-only and must inspect the supplied authority and
exact Task PR, integration-only composition, eligible legacy range, or
standalone target. Keep full sources directly available without copying
unrelated unchanged prose into each message.

After a successful dispatch for a task in a herdr-managed workspace, return the
mapping between the Task PR identity, returned agent identity, and exact task
workspace to the lead. When the user wants interactive visibility, tell the
lead to direct the user to open that herdr workspace and run `codex agents` to
search for and inspect the mapped task agent.

Treat `codex agents` as an observation surface by default. The lead remains the
source of truth for agent state and continues to own follow-up, interruption,
waiting, and closure. Do not start, rename, steer, or stop a task through the
dashboard unless the user explicitly requests manual intervention.

Use bounded waits, inspect live agents regularly, and return progress evidence to
the lead. Preserve reports, identities, completion state, and observed errors
without translating findings or deciding whether the task passed.

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

## Return scheduling evidence

Return dispatch and queue order, the Task-PR-to-agent-to-workspace mapping,
agent identities, live and effective capacity, completion or interruption
states, reports, inspected Git state after writer failure, and every
availability or attribution gap.

Use `BLOCKED` whenever safe scheduling or writer-state attribution cannot be
established. Otherwise return scheduling evidence to the invoking phase.
`execute-task` alone interprets writer results and manages corrections; `review`
alone integrates the requested reviewer results and returns its gate verdict.
Do not release a dependency, decide task or feature acceptance, publish, merge,
or tear down a workspace.
