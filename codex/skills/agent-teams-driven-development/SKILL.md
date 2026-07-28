---
name: agent-teams-driven-development
description: Schedule one writer and requested read-only reviewers for execute-task while enforcing live capacity, queues, and partial-state safety.
---

# Agent-teams driven development

Act only as the scheduling adapter for `execute-task`. The lead schedules all
work, and subagents never spawn descendants. Do not select workflow paths,
review modes, role breadth, severity mappings, Acceptance, retry semantics, task
commits, or task acceptance here.

## Require a scheduling request

Accept from `execute-task`:

- one canonical task context;
- the already-selected implementer or reviewer roles;
- each selected role's complete prompt contract;
- the working directory and exact task base;
- configured capacity, observed capacity evidence, and queue rules;
- whether the request is a fresh dispatch, follow-up, or replacement;
- any prior agent identity, termination evidence, and partial state.

Reject a request that lacks a selected role contract or would require this
adapter to reinterpret the Review policy. Do not load or resolve prompt files in
this adapter. The incoming implementer contract is loaded by `execute-task` only
when an implementer is actually being dispatched, and incoming reviewer
contracts only after it has selected the active gate; unselected prompts remain
unloaded.

## Enforce live capacity and queueing

Call `list_agents` before every dispatch wave. Set effective capacity to the
lower of configured capacity and currently observed runtime capacity, count the
lead, and never exceed six total threads.

Maintain a deterministic queue when selected roles exceed available slots. Do
not reduce or replace requested roles to fit capacity. Record configured,
observed, and effective capacity, live identities, queued roles, dispatch order,
and every capacity gap.

Allow no more than one implementer and one active writer for the shared worktree.
Every reviewer is read-only. Independent reviewers may run concurrently only
when capacity permits; otherwise queue them without changing their independence
or contracts.

## Schedule and observe

Dispatch only the already-selected role with its canonical context and complete
role contract. Explicitly tell every agent it must not spawn descendants. For an
implementer, state its owned task and file responsibilities and that it is the
only writer. For a reviewer, state that it is read-only and must inspect the
supplied exact range.

Use bounded waits, inspect live agents regularly, and return progress evidence to
the lead. Preserve each agent's report, identity, completion state, and observed
errors without translating findings or deciding whether the task passed.

## Protect failure and partial state

Never dispatch a replacement writer while another writer may still be active.
After an implementer interruption, failure, timeout, or lost response:

1. inspect the termination result and live-agent state;
2. inspect the repository HEAD, working-tree status, and task-base-to-current
   diff;
3. determine whether partial edits or commits can be attributed and whether the
   previous writer is conclusively inactive;
4. dispatch a replacement only when no writer overlaps and the current state is
   understood well enough for the canonical task context to remain valid.

If writer termination, ownership, HEAD, status, diff, or partial state is
uncertain, return `BLOCKED` with all partial evidence. Do not guess, clean the
worktree, discard edits, or start another writer.

For reviewer failure, preserve completed read-only reports, recheck live capacity,
and queue only the already-requested replacement role. If the selected
independent role cannot be provided, return the availability gap to
`execute-task`; do not substitute the lead or another perspective.

## Return scheduling evidence

Return the dispatch and queue record, agent identities, live and effective
capacity, completion or termination states, reports, repository-state evidence
collected after writer failure, and every partial-state or availability gap.

Use `BLOCKED` when safe scheduling or writer-state attribution cannot be
established. Otherwise return scheduling evidence to `execute-task`, which alone
interprets task results, applies the selected gate, manages retries, and decides
task acceptance. Do not advance to another task, global verification, final
review, publication, merge, or teardown.
