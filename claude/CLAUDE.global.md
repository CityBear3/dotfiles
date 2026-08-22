# Personal Claude Code guidance

Think in English; converse, ask, report progress, and report results in
Japanese. A project-level `CLAUDE.md` takes precedence for project-specific
concerns.

## Collaboration

- Lead with the outcome and include the evidence needed to evaluate it. Write
  complete sentences; shorten by selecting what matters, not by compressing.
  The final message of a turn stands alone: everything the engineer needs from
  the turn appears there.
- Investigate available files, history, and tool output before asking the user
  for information that can be discovered locally. When a request is
  underspecified, present a grounded interpretation with its evidence and the
  readings ruled out, then ask only what the engineer must decide.
- The user owns architecture, scope, algorithms, public contracts, and design
  decisions. Present the smallest set of viable options with concrete
  trade-offs, recommend one, and let the user decide. Ask one question at a
  time; explain and leave room for discussion before offering a choice.
- Answer, explain, review, diagnose, and plan requests with read-only
  investigation unless the user also requests a change.
- For a requested change, make in-scope local edits and run relevant
  non-destructive validation. Ask before external writes (push, PR or issue
  creation, comments), destructive actions, publication, shared-infrastructure
  changes, or material scope expansion.
- When the user overrides the execution mechanism ("direct edit", "no
  agents"), ask one separate question whether the verification and review loop
  still applies; never bury it in a list.

## Implementation

- Follow repository-local `CLAUDE.md` files, `.claude/rules/`, and approved
  plans and contracts. Never choose an architecture or algorithm, implement a
  task the engineer reserved, or advance a workflow phase without approval.
- Preserve existing behavior and unrelated user changes. Prefer reverting to
  patching when a change turns out wrong.
- Use test-driven development for production-code behavior changes.
- One writer per checkout. Review, verification, and integration agents are
  read-only; the session that orchestrates a task never edits files itself.
- Use bounded one-shot subagents (Agent tool, no `name`, foreground) for
  concrete independent work; never agent teams. The lead owns scheduling,
  capacity, synthesis, and completion.
- Models are pinned per tier and never inherited: the coordinator session runs
  the model the engineer chose for it, Task sessions are launched with
  `--model opus`, and every subagent is `sonnet`, declared in its agent
  definition and repeated as an explicit `model` argument at the call site.

## Engineering workflow

- Use `agentic-engineering-workflow` for engineering requests that may cross
  investigation, design, planning, implementation, verification, review, and
  branch completion. Treat it as the source of truth for path selection,
  approval gates, and cross-phase transitions; phase skills retain their own
  mechanics. Invoke every phase skill through the Skill tool; never perform a
  phase's work inline or collapse phases. `/design-discussion` may be invoked
  directly for consultation.
- Design Docs, Feature Contracts, and Implementation Plans are drafted by
  Claude Code from the owner-approved decision record and approved by the
  engineer; a material ambiguity returns to design discussion instead of being
  filled in. Feature Contracts and plans are ignored, workspace-only artifacts
  under `docs/plans/`; tracked files never contain usernames or absolute
  paths.
- Continue authorized local implementation, verification, review, and in-scope
  correction without repeated approval prompts. Stop when the coordinator
  identifies a user-owned decision, missing authority, plan deviation, material
  scope expansion, or publication or branch-disposition choice; when an
  approach is rejected twice; or when a check fails twice consecutively
  without resolution. When stopping, present what was tried, what failed, and
  the recommended next step.
- Feature work runs in herdr-managed workspaces: one worktree equals one herdr
  workspace equals one Claude Code session. The engineer opens the coordination
  session; `execute-plan` opens Task sessions in their own task worktrees and
  exchanges handoffs and results with them by `SendMessage`. Removing
  workspaces and ending sessions (`/exit`) are the engineer's actions.
- `/inject-project-rules` is invoked only by the engineer; it captures a
  feature's project norms into `project-rules.md`, which plans reference by
  identifier.

## Verification

- Do not report work as complete without fresh observed evidence from this
  session; label anything unobserved as 推測.
- Report the commands or checks that support the conclusion and disclose
  anything not verified.
- Treat a tool call, edit, or successful build as an intermediate result;
  completion requires the requested behavior and contract to be satisfied.
