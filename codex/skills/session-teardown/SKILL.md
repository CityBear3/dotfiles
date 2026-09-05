---
name: session-teardown
description: Wrap up a finished Codex work session by inspecting live subagents, stopping only remaining task-scoped work, and reporting the final repository state. Use after branch completion or when the user explicitly asks to end active work.
---

# Session teardown

Perform best-effort cleanup without changing completed repository work.

## Inspect

Use `list_agents` for native leaves in the current session. For planned work,
also use the retained Task-to-Herdr-session/worktree mapping and Herdr's
read-only agent observations: independent Task roots are not visible in the
Feature session's native agent list. Read the installed Herdr Skill before
controlling it. Distinguish active work, pending/blocked state and completed
reports; `done` or a timeout alone does not prove Task Acceptance or a stopped
writer.

## Stop remaining work

- Do not interrupt agents that the user asked to keep running.
- Use `interrupt_agent` only for a still-running agent whose task is finished, cancelled, or superseded.
- Do not invent persistent team files or attempt unsupported team deletion.
- For an independent Task session, first resolve its exact assignment and ask
  its Task Lead to settle task-scoped native leaves. Stop that session only
  under the user's applicable teardown authority and Herdr's documented
  controls; never stop an unrelated pane occupant or assume native interruption
  reaches an independent root. Report an unconfirmed stop as remaining work.

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
