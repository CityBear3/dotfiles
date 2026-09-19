---
name: dispatching-parallel-agents
description: Start or resume dependency-ready Task Lead sessions when an approved Implementation Plan authorizes execution.
---

# Dispatch independent Task sessions

This is the Feature Lead's Task-session adapter for `execute-plan`.
Task Leads are independent Codex roots, not native Feature subagents. Native
check dispatch belongs to `agent-teams-driven-development` inside each session.
Do not select Task readiness, policy, models, acceptance, or publication here.

## Require ready, isolated work

For each assignment require the approved Task/PR identity and complete handoff,
execution-start authority, ready-order and dependency evidence, exact validated
Herdr workspace/pane/worktree and Git identity, sole-writer ownership, and the
engineer-approved Task model/effort. Require fresh/resume/replacement intent and
prior routing identities where applicable. The Task Lead role source is
[task-lead.md](../execute-task/references/task-lead.md); supply its resolved
absolute installed path in every launch handoff.

Only dependency-ready, ownership-disjoint Tasks in separate checkouts may run
concurrently. Do not create artificial DAG edges from PR stack order; a
plan-authorized early candidate cannot release dependents. Missing authority,
ambiguous ownership, or overlapping writers prevents dispatch.

## Launch through Herdr

Read the installed Herdr Skill before controlling it and respect its environment,
pane identity, readiness and permission rules. `create-workspace` returns a
Git-validated Task worktree with separate editor and Task Lead pane identities.
For a new Task, launch in the verified right Task Lead shell; the initial left
pane belongs to the engineer's Neovim. Use the returned Task Lead pane ID, not
the initial or UI-focused pane. An editor startup failure does not block this
available shell. Do not start Codex over an editor, lazygit, another agent, an
approval dialog, or an unidentified pane occupant. Resolve the explicit pane
and agent mapping; do not use focus or names as guessed identity.

Pass native Codex arguments after Herdr's `--`. For an approved Astra/high Task
the argument shape is:

~~~sh
herdr agent start <task-agent-name> --kind codex --pane <validated-task-lead-pane> -- \
  --cd <absolute-task-worktree> --model gpt-6-astra \
  -c 'model_reasoning_effort="high"' \
  -c 'plan_mode_reasoning_effort="high"'
~~~

Substitute the exact approved allocation, including both effort settings; an
approved Sol/high Task uses `gpt-5.6-sol`. Do not edit global defaults, inherit the
Feature model implicitly, lower sandbox/approval settings, or use a native
subagent profile as if it configured this root. Supply the complete Task
handoff through `herdr agent prompt` only after startup readiness, instructing
the root to read the shared Task Lead contract and run `execute-task`.
Before edits, require confirmation of Task, role source, working directory,
branch/base/head, effective allocation, and ownership. Missing or contradictory
bindings are `BLOCKED`, never automatic fallback.

Startup readiness is not Task completion. Retain the returned Herdr routing
identity with the Task assignment. On startup failure or unknown state inspect
that exact target before retrying; do not create a duplicate writer. Temporary
unavailability stays pending or blocked with evidence, without new concurrency
quotas, leases, or a global scheduler.

## Observe without proxying the local loop

Retain the Task session through implementation, verification, review and bounded
correction. It dispatches and waits for its native leaves itself. Feature-level
messages are dispatch, Task result, cross-Task effect, or genuine escalation.

Use Herdr's bounded, event-responsive wait/read facilities within the active
tool and responsiveness limits. Avoid repeated unchanged polling. A lifecycle
state, timeout, or terminal silence is neither a prompt-correlated receipt nor
Task Acceptance. Match result Task, authority and exact Git target to the
assignment. If required output is truncated, retrieve the exact evidence source
or request a complete report from that same session; do not preemptively force
a new file-report protocol or replay the implementation prompt.

## Resume and return

Do not replace a possibly active writer. Resolve the existing session and its
recorded Task Lead pane separately from the editor pane; preserve existing
running layouts and resume the matching session without creating another pane.
Resolve its leaves, attribute branch/base/head/status/commits/diff, confirm no
competing writer or old-head checks, and revalidate authority and allocation. Resume a
safely idle matching session with a current handoff; a replacement starts with
no inherited conversation at the same approved model/effort. Never clean,
reset, rebase, amend, discard, or silently restart uncertain work.

Return Task-to-Herdr-session-to-workspace mapping with distinct Task Lead and
editor pane identities, allocation, pending order, startup/resume observations,
result and evidence references, and gaps to
`execute-plan`. Feature Lead directly validates Git and gate evidence before
dependency release. This adapter does not issue acceptance or remove sessions,
worktrees, or branches.
