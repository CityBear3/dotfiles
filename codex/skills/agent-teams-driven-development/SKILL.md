---
name: agent-teams-driven-development
description: Execute an approved implementation plan through one writer and independent spec and quality review loops, using bounded Codex subagents and runtime-aware capacity. Use from execute-plan unless the user explicitly requests direct execution without agents.
---

# Agent-teams driven development

The lead schedules all work. Subagents do not spawn descendants.

## Inputs

Require:

- an approved plan path;
- a non-default feature branch or approved workspace;
- repository guidance and working directory;
- configured maximum capacity and currently observed live agents.

Read the three task templates beside this file before dispatch:

- [implementer-prompt.md](implementer-prompt.md)
- [spec-reviewer-prompt.md](spec-reviewer-prompt.md)
- [code-quality-reviewer-prompt.md](code-quality-reviewer-prompt.md)

If the runtime can select named profiles, use `implementer`, `spec-reviewer`, and `code-quality-reviewer`. Otherwise include the complete corresponding fallback prompt in the `spawn_agent` message.

## Capacity

Use `list_agents` before each dispatch wave. The effective limit is the lower of configured capacity and capacity observed from the current runtime. Count the lead. Never exceed six total threads.

Keep one implementer as the only writer. Run the two read-only reviewers concurrently after implementation when two slots are available; otherwise queue them.

## Per-task loop

1. Record the task's base commit.
2. Give the implementer the full task, dependencies, working directory, verification command, and output contract.
3. Use `wait_agent` in bounded intervals and inspect `list_agents` regularly. Send the user a progress update at least every 60 seconds during long work.
4. Require the implementer to report changed files, commit, commands, results, and concerns.
5. Record the head commit and dispatch spec and quality review against the exact range.
6. Require reviewers to inspect the diff independently and return verified file/line findings or approval.
7. Send valid findings to the existing implementer with `followup_task` when idle or `send_message` when already running.
8. Re-run both reviews after fixes.

Stop and report when a design decision is missing, the plan must change, or the same gate fails twice after attempted correction. Do not let reviewers edit.

## Completion

Complete only when every plan task has:

- an implementation commit;
- observed task verification evidence;
- spec approval;
- quality approval.

Report task commits and evidence, then transition to `verify`. Do not push, merge, or tear down the session from this skill.
