---
name: agent-teams-driven-development
description: Schedule one writer and requested read-only reviewers for execute-task while enforcing live capacity, queues, and interruption safety.
---

# Agent-teams driven development

Act only as the scheduling adapter for `execute-task`. The lead schedules all
work, and subagents never spawn descendants. Do not select workflow paths,
Review context, review modes, role breadth, severity mappings, Acceptance,
correction semantics, task commits, or task acceptance here.

## Require a bounded scheduling request

Accept from `execute-task`:

- the already-selected implementer or reviewer role;
- exactly one resolved role contract: a selectable named profile, or a complete
  fallback contract when that named profile is unavailable;
- the complete writer or reviewer message already prepared by `execute-task`;
- whether the request is a fresh dispatch, follow-up, or replacement;
- any prior agent identity, interruption result, and observed Git state.

Reject a request that lacks exactly one resolved role contract, supplies both a
named profile and fallback, or would require this adapter to reinterpret task or
policy semantics. Do not load prompt files here. `execute-task` resolves named
profile availability first and loads a fallback only for an unavailable selected
profile. Unselected prompts remain unloaded.

Pass the resolved named profile and message, or the supplied fallback contract
and message, unchanged. `execute-task` owns completeness and freshness checks;
this adapter does not add another wrapper or field list.

## Enforce live capacity and a deterministic queue

Call `list_agents` before every dispatch wave. Set effective capacity to the
lower of configured and currently observed runtime capacity, count the lead, and
never exceed six total threads.

Queue already-selected roles in request order when available slots are
insufficient. Record configured, observed, and effective capacity, live agent
identities, queued roles, dispatch order, and every capacity gap. Do not reduce,
replace, or reorder selected roles to fit capacity.

Allow no more than one implementer and one active writer for the shared worktree.
Every reviewer is read-only. Independent reviewers may run concurrently when
capacity permits; otherwise queue them without changing their independence or
contracts.

If a required role cannot be instantiated or the queue cannot make progress,
return `BLOCKED` with observed availability evidence. Do not turn a runtime
shortage into policy `Escalate` or substitute the lead or another perspective.

## Schedule and observe

Dispatch only the already-selected role using its resolved named profile or
complete fallback contract. Tell every agent not to spawn descendants. Tell an
implementer its owned task and exact file responsibilities and that it is the
only writer. Tell a reviewer it is read-only and must inspect the supplied task
base, current head, range, and diff.

Use bounded waits, inspect live agents regularly, and return progress evidence to
the lead. Preserve reports, identities, completion state, and observed errors
without translating findings or deciding whether the task passed.

Return every writer response unchanged with the observed agent completion state.
`execute-task` validates status, report fields, commit, current head, range, and
verification. This adapter never claims task acceptance.

## Inspect interruption state before resuming

After an implementer interruption, failure, timeout, lost response, incomplete
report, partial edit, or partial commit:

1. inspect the interruption result and live-agent state;
2. confirm the prior writer is inactive;
3. inspect repository HEAD, status, commits, and task-base-to-current diff;
4. confirm the exact task base is an ancestor of the current head;
5. determine whether every in-scope edit and commit is attributable to the task.

Resume the same writer or dispatch a replacement only when no writer overlaps,
the exact task base remains an ancestor of the current head, the observed Git
state is attributable, and the unchanged task handoff still applies. If
inactivity, ancestry, or attribution is uncertain or fails, preserve all state
and return `BLOCKED` with the evidence and exact re-entry condition. Never guess,
clean, reset, rebase, amend, discard edits, or start another writer to force
progress.

For reviewer failure, preserve completed read-only reports, recheck live
capacity, and queue only the already-requested replacement role. If the required
independent role remains unavailable, return `BLOCKED`; do not substitute
another perspective.

## Return scheduling evidence

Return dispatch and queue order, agent identities, live and effective capacity,
completion or interruption states, reports, inspected Git state after writer
failure, and every availability or attribution gap.

Use `BLOCKED` whenever safe scheduling or writer-state attribution cannot be
established. Otherwise return scheduling evidence to `execute-task`, which alone
interprets results, applies the selected gate, manages corrections, and decides
task acceptance. Do not advance another task, global verification, final review,
publication, merge, or teardown.
