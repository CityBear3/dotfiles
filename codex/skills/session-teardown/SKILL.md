---
name: session-teardown
description: Wrap up a finished Codex work session by inspecting live subagents, stopping only remaining task-scoped work, and reporting the final repository state. Use after branch completion or when the user explicitly asks to end active work.
---

# Session teardown

Perform best-effort cleanup without changing completed repository work.

## Inspect

Use `list_agents` to identify live task-scoped agents. Distinguish running work from already completed or failed agents.

## Stop remaining work

- Do not interrupt agents that the user asked to keep running.
- Use `interrupt_agent` only for a still-running agent whose task is finished, cancelled, or superseded.
- Do not invent persistent team files or attempt unsupported team deletion.

## Report

Inspect every task and temporary integration workspace known to the active plan,
not only the coordination checkout, and report:

- remaining live agents, if any;
- coordination, Task PR, and temporary integration branches, worktrees, heads
  or trees, acceptance or stale state, cleanup eligibility, and disposition;
- uncommitted changes;
- publication or cleanup actions already completed;
- anything the user still needs to do.

Never end the user's client session or run destructive cleanup implicitly.
