---
name: session-teardown
description: Wrap up a finished Claude Code work session by inspecting live subagents, stopping only remaining task-scoped work, and reporting the final repository state. Use after branch completion or when the user explicitly asks to end active work.
---

# Session teardown

Perform best-effort cleanup without changing completed repository work.

## Inspect

Use the `ListAgents` tool to identify live subagents of this session and the other local Claude Code sessions it lists. Distinguish running work from already completed or failed agents. Task sessions (`<feature>-task-<n>`) appear there as separate local sessions, not as subagents of this one.

## Stop remaining work

- Do not interrupt agents the user asked to keep running.
- Use `TaskStop` only for a still-running subagent of this session whose task is finished, cancelled, or superseded.
- Do not stop a Task session. It is a separate Claude Code session in its own herdr workspace; list it and leave it for the engineer.

## Report

Inspect every task and temporary integration workspace known to the active plan,
not only the coordination checkout, and report:

- remaining live subagents of this session, if any;
- live Task sessions and their herdr workspaces, if any, left running for the engineer;
- coordination, Task PR, and temporary integration branches, worktrees, heads
  or trees, acceptance or stale state, cleanup eligibility, and disposition;
- uncommitted changes;
- publication or cleanup actions already completed;
- anything the user still needs to do.

Never run destructive cleanup implicitly. The engineer ends the session (`/exit`); Claude Code never runs it.
