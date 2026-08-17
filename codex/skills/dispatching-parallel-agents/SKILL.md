---
name: dispatching-parallel-agents
description: Dispatch already-bounded independent tasks to Codex subagents while respecting approved dependency readiness, isolated workspaces, runtime capacity, and shared-state constraints.
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

Use `list_agents` before dispatch. Count the lead and all live agents. Respect the lower of configured capacity and runtime capacity; never assume all configured slots are available.

Count writers and reviewers from every active Task PR. When ready tasks or
reviewers exceed free slots, use the plan's deterministic queue instead of
dropping, reordering, or oversubscribing them.

## Dispatch

Call `spawn_agent` once per concrete task. Use a stable, descriptive task name and include all task-local context because agents should not depend on the conversation.

Each prompt states:

- goal and boundaries;
- working directory and relevant files;
- exact task branch, checkout, planned PR base, and mode;
- whether writes are allowed;
- prohibited overlap;
- required commands or evidence;
- exact return format;
- no descendant spawning.

Continue useful lead work while agents run. Use bounded `wait_agent` calls and `list_agents` for regular status checks.

## Integrate

Validate each result against the requested output and its exact workspace. Check
for conflicting edits, branches, state, or assumptions. Return candidate and
authoritative results unchanged to the owning scheduler; do not release a
dependency or claim feature completion here. If an existing agent needs a
correction, use `followup_task`; use `send_message` for information that should
not start a new turn.

Synthesize results once. Do not repeat completed work merely because agents ran independently.
