---
name: create-workspace
description: Verify or establish an approved coordination, Task PR, or temporary integration-evidence workspace. Resolve work branches, starting refs, PR bases, and composition identities separately.
---

# Prepare a coordination, task, or integration workspace

Keep one writer in each checkout. Use one coordination workspace for active
Feature Contract and Implementation Plan artifacts. Use separate task branches
and checkouts when an approved plan permits concurrent Task PR work or when a
task's planned PR range must remain isolated. Use a temporary integration
workspace only for a plan-defined composed tree; it has no source writer or PR
identity.

On the planned path, invoke this skill after investigation makes the purpose and
initial feature boundary identifiable and before writing the first durable
Design Doc, Feature Contract, or Implementation Plan draft. This timing keeps
approved artifacts in the coordination workspace. After Implementation Plan
approval, reuse this skill to establish only the task workspaces and branch
relationships that plan defines, and later any exact temporary integration
workspace requested by `execute-plan`. For a dependency-ready Task, require the
approved plan identity and explicit execution-start authorization; that
authorization covers creation or reuse of its exact non-destructive local Task
branch and worktree without another approval. Other workspace and Git state
changes retain their applicable authority boundaries.

## Inspect

Run read-only checks:

- `git rev-parse --show-toplevel`
- `git rev-parse --path-format=absolute --git-common-dir`
- `git branch --show-current`
- `git worktree list --porcelain`
- `git status --short`

Read repository guidance for branch and worktree policy. Use the local
`refs/remotes/<remote>/HEAD`, when present, to identify a default branch without
contacting the remote.

## Resolve the intended state

Resolve these separately before changing Git state:

- **workspace purpose**: coordination, one named Task PR, or one named temporary
  integration-only composition;
- **workspace mode**: the current checkout, a Codex-managed worktree, or a
  herdr-managed worktree;
- **work branch**: an existing local branch, a new task branch, or the approved
  detached or temporary integration ref;
- **starting ref**: the commit used to create a new work branch;
- **planned PR base or composition identity**: the branch relationship against
  which a task receives authoritative verification and review, or the approved
  starting tree and ordered accepted inputs for integration evidence.

For every new planned Task PR governed by the current workflow, workspace mode
is contractually Herdr-managed. Require the repository root, exact Task branch,
explicit starting ref for a new branch, `--no-focus`, and `--json`. Herdr
creation or returned-identity validation failure is `BLOCKED`; do not silently
substitute a raw Git worktree, Codex-managed worktree, user-prepared checkout,
or another launch path. This requirement does not add planned Task artifacts or
a Herdr workspace to eligible lightweight work. Standalone is a read-only
authority form over an already resolved range, snapshot, or bounded fileset,
not a workspace mode; it never requires creating a Herdr workspace, Task branch,
or Task worktree.

Do not conflate the starting ref with the planned PR base. An independent task
may start from a common implementation base and later be restacked onto its
approved PR parent. Record that work as a candidate until the final base is
materialized; this skill never treats branch creation as task acceptance.

For an integration workspace, require the approved starting commit or tree,
ordered accepted Task PR inputs, composition mechanism, and retention boundary.
This skill establishes the empty workspace identity only; `execute-plan`
materializes and validates the composed tree without assigning a source writer.

If the current checkout already matches the intended workspace and work branch,
report its path and branch and continue. Do not create another workspace merely
because the current checkout is not a linked worktree.

Always isolate a temporary integration composition from coordination and Task PR
checkouts. Follow the plan's detached or temporary-ref strategy; never reuse an
active task branch merely because it already contains some required commits.

Unless the user or repository requires a worktree, prefer a coordination branch
in the current checkout. For a planned Task PR, follow the approved workspace
and PR topology rather than this default. Propose a short branch name and an
explicit starting ref; do not silently assume that the ref or PR base is
`main`. If a planned Task's starting-ref resolution rule is missing or cannot be
resolved exactly when it becomes dependency-ready, return `BLOCKED`; do not
substitute the current `HEAD`. For another new branch without an approved
starting ref, propose the current `HEAD`.

Resolve branch and base names against local refs. A remote branch means the
locally available remote-tracking ref such as `origin/develop`. Do not fetch
implicitly. If the requested ref is absent or freshness matters, ask before
running `git fetch`.

## Use the current checkout

Before switching or creating a branch, report:

- the current path and branch;
- dirty changes;
- the proposed work branch;
- for a new branch, the proposed starting ref;
- for a Task PR, its Task Contract, planned PR parent, and whether the final PR
  base is already materialized.

Ask for approval before changing branches. The exception is creation or reuse
of the exact planned Task branch in its separate Herdr worktree when
`execute-plan` supplies both the approved identity and explicit execution-start
authorization; do not ask again for that operation or use it to switch the
user's current checkout.

- Existing local branch: run `git switch <work-branch>`.
- New branch from a local branch, tag, commit, or remote-tracking ref: run
  `git switch -c <work-branch> <base-ref>`.
- Remote-only branch that should retain its upstream relationship: run
  `git switch --track -c <local-branch> <remote>/<branch>`.

Creating a branch in place keeps current uncommitted changes. Switching to an
existing branch may conflict with them. Never stash, move, copy, or discard
changes without explicit approval.

## Use a worktree

Choose the mechanism explicitly.

### Codex-managed worktree

Use this only in the ChatGPT desktop app. Ask the user to select the starting
branch through `/worktree` or Handoff. Codex creates the worktree at that
branch's commit in detached `HEAD`; creating or selecting the eventual work
branch is a later action in the Codex UI.

Do not describe a Codex-managed worktree as already checking out the requested
work branch.

### Herdr-managed worktree

Use this for every new planned Task worktree and for another persistent worktree
when the user selects Herdr and the CLI is available. Always pass the repository
root through `--cwd`, keep focus in the current session with `--no-focus`, and
request structured output with `--json`.

For a planned Task, accept creation only after `execute-plan` reports the Task
dependency-ready and supplies its approved workspace mode, branch, resolved
starting ref, planned PR base, and execution-start authorization. Treat that as
the applicable authority for the exact Herdr creation below, not as authority
for mismatch repair or any other Git operation.

If the work branch already exists locally, omit `--base`:

```sh
herdr worktree create \
  --cwd <repository-root> \
  --branch <existing-local-branch> \
  --no-focus \
  --json
```

If the work branch is new, pass its starting ref explicitly:

```sh
herdr worktree create \
  --cwd <repository-root> \
  --branch <new-work-branch> \
  --base <base-ref> \
  --no-focus \
  --json
```

For a branch that exists only as `origin/<name>`, use a local work-branch name
with `--base origin/<name>`. If that local work branch already exists, herdr
checks out the existing branch and does not recreate it from `--base`; stop and
resolve any mismatch instead of silently using the wrong commit.

Before branch-backed creation, check whether the local work branch is already
checked out in another worktree. After creation, report and verify:

- the coordination, Task PR, or integration composition identity;
- the returned worktree path, Herdr workspace ID, and initial pane ID;
- the checked-out branch and `HEAD`;
- the starting ref and planned PR base or composition inputs;
- the worktree status;
- the configured upstream, if any.

For a Task PR, perform those branch, `HEAD`, ancestry/base, status, and upstream
checks directly through Git in the returned path. Herdr JSON, workspace or pane
state, and later agent reports do not replace direct Git validation. If creation
or any required identity check disagrees with the approved Task PR, preserve the
workspace and return `BLOCKED` with the Task PR, attempted command, returned
path/workspace/pane identities when any, observed Git identity, error, and exact
re-entry condition.

For new planned Task sessions, leave the validated initial pane at an
interactive shell so `dispatching-parallel-agents` can start its independent
Codex Task Lead there. Do not launch lazygit in that Task pane or overwrite an
existing foreground process. Resolve and report the explicit pane occupant;
an unknown or conflicting occupant is BLOCKED rather than authority to kill it.
Read the installed Herdr Skill before controlling Herdr. Optional engineer
observation must not take over the Task's agent pane.

For a coordination workspace, ask the user to continue the session in the
returned path when the active writer must move there. For a planned Task PR,
return the validated Git/Herdr mapping and shell readiness to `execute-plan`.
Its dispatch adapter starts the independent Task session with the approved
model, both effort settings, exact working directory and shared Task Lead
contract; this skill does not submit a Task handoff or start the agent itself.
For an integration workspace, return its path and identity without assigning
an implementation writer or moving the user's session.

## Guardrails

- Explain that uncommitted changes in the current checkout do not follow into a
  herdr worktree.
- Never place two active writers in one checkout or reuse one task branch for
  another Task Contract.
- Reject a task workspace absent from the approved topology, overlapping writer
  ownership, or an unexplained branch already checked out elsewhere.
- Reject an integration workspace absent from the approved composition, and do
  not treat it as a Task PR, publication target, or implementation workspace.
- If Herdr is unavailable for a required planned Task workspace, return
  `BLOCKED`; do not offer or silently substitute another mechanism. For a
  non-planned workspace whose authority does not require Herdr, a Codex-managed
  or user-prepared worktree may be offered. Do not substitute raw
  `git worktree add` without approval.
- Do not invoke this skill solely to manufacture workspace evidence for a
  standalone verification or review target. Resolve that target read-only in
  `verify` or `review`.
- Do not remove worktrees or delete branches in this skill.
