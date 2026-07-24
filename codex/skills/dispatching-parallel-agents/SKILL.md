---
name: dispatching-parallel-agents
description: Dispatch two or more independent, bounded tasks to Codex subagents while respecting runtime capacity and shared-state constraints. Use when work can proceed concurrently without overlapping writes or sequential dependencies.
---

# Dispatch parallel agents

Parallelize only independent work.

## Decide whether to dispatch

For each candidate task identify:

- exact scope and expected output;
- files or external state it may read or write;
- required context;
- dependencies on other tasks;
- completion evidence.

Do not parallelize tasks that edit the same files, mutate shared state, depend on one another's results, or require a single evolving judgment.

## Capacity

Use `list_agents` before dispatch. Count the lead and all live agents. Respect the lower of configured capacity and runtime capacity; never assume all configured slots are available.

When a plan needs more reviewers than free slots, queue them by priority instead of dropping or oversubscribing them.

## Dispatch

Call `spawn_agent` once per concrete task. Use a stable, descriptive task name and include all task-local context because agents should not depend on the conversation.

Each prompt states:

- goal and boundaries;
- working directory and relevant files;
- whether writes are allowed;
- prohibited overlap;
- required commands or evidence;
- exact return format;
- no descendant spawning.

Continue useful lead work while agents run. Use bounded `wait_agent` calls and `list_agents` for regular status checks.

## Integrate

Validate each result against the requested output and check for conflicting edits or assumptions. If an existing agent needs a correction, use `followup_task`; use `send_message` for information that should not start a new turn.

Synthesize results once. Do not repeat completed work merely because agents ran independently.
