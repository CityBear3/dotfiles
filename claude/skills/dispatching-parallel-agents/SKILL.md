---
name: dispatching-parallel-agents
description: Dispatch already-bounded independent tasks to Claude Code subagents while respecting approved dependency readiness, isolated workspaces, runtime capacity, and shared-state constraints.
---

# Dispatch parallel agents

Parallelize only independent work. When invoked from `execute-plan`, act as a
scheduling adapter for tasks that it already marked ready; do not reinterpret
the Task DAG, PR topology, ownership, or review policy.

## Decide whether to dispatch

For each candidate task identify:

- exact scope and expected output;
- files or external state it may read or write;
- required context;
- dependencies on other tasks;
- approved branch and checkout, planned PR base, and candidate or authoritative
  mode when applicable;
- completion evidence.

Do not parallelize tasks that edit the same files, share a checkout or active
writer, mutate conflicting state, depend on one another's results, lack an
approved workspace, or require a single evolving judgment. A PR stack edge alone
is not a logical dependency when the approved plan permits an early candidate.

## Capacity

Use the capacity recorded in the approved Review policy (default: at most 4
concurrent subagents per session) and queue the rest in deterministic policy
order.

Count writers and reviewers from every active Task PR. When ready tasks or
reviewers exceed free slots, use the plan's deterministic queue instead of
dropping, reordering, or oversubscribing them.

## Dispatch

Call the Agent tool once per concrete task, passing `model: "sonnet"`
explicitly and no `name` parameter — a named spawn becomes a persistent
teammate, and reviewers, verifiers, and implementers must be one-shot
subagents. Send every independent task's call in a single message so they run
concurrently, and never `run_in_background: true`, so each result returns
inline as that message's tool result. Include all task-local context in the
prompt because agents should not depend on the conversation.

Each prompt states:

- goal and boundaries;
- working directory and relevant files;
- exact task branch, checkout, planned PR base, and mode;
- whether writes are allowed;
- prohibited overlap;
- required commands or evidence;
- exact return format;
- no descendant spawning.

There is no lead work to interleave while foreground agents run: batching the
independent calls into one message is what runs them concurrently, and every
result becomes available together once that message's calls complete.

## Integrate

Validate each result against the requested output and its exact workspace. Check
for conflicting edits, branches, state, or assumptions. Return candidate and
authoritative results unchanged to the owning scheduler; do not release a
dependency or claim feature completion here. If an existing agent needs a
correction, use `SendMessage` to the agent id the Agent tool returned, which
resumes it with its context intact.

Synthesize results once. Do not repeat completed work merely because agents ran independently.
