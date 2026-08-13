---
name: create-workspace
description: Verify or establish feature work on the intended Git branch, a Codex-managed worktree, or a herdr-managed worktree. Resolve the work branch and its base ref separately. Use before the first durable planned-path design artifact or implementation when the current checkout may be unsuitable or isolation is required.
---

# Prepare a feature workspace

Keep one feature in one branch and one writer in each checkout. Use a worktree
when concurrent work or repository policy requires a separate checkout.

On the planned path, invoke this skill after investigation makes the purpose and
initial feature boundary identifiable and before writing the first durable
Design Doc, Feature Contract, or Implementation Plan draft. This timing keeps
approved artifacts and later implementation in the feature workspace. It does
not change the branch-selection approval or authorize a Git state change by
itself.

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

- **workspace mode**: the current checkout, a Codex-managed worktree, or a
  herdr-managed worktree;
- **work branch**: an existing local branch or a new branch for the task;
- **base ref**: the starting ref for a new branch only.

If the current checkout already matches the intended workspace and work branch,
report its path and branch and continue. Do not create another workspace merely
because the current checkout is not a linked worktree.

Unless the user or repository requires a worktree, prefer a feature branch in
the current checkout. Propose a short branch name and an explicit base ref; do
not silently assume that the base is `main`. If no base was requested for a new
branch, propose the current `HEAD`.

Resolve branch and base names against local refs. A remote branch means the
locally available remote-tracking ref such as `origin/develop`. Do not fetch
implicitly. If the requested ref is absent or freshness matters, ask before
running `git fetch`.

## Use the current checkout

Before switching or creating a branch, report:

- the current path and branch;
- dirty changes;
- the proposed work branch;
- for a new branch, the proposed base ref.

Ask for approval before changing branches.

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

Use this for a persistent worktree when the user selects herdr and the CLI is
available. Always pass the repository root through `--cwd`, keep focus in the
current session with `--no-focus`, and request structured output with `--json`.

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

Before creation, check whether the local work branch is already checked out in
another worktree. After creation, report and verify:

- the returned worktree path and herdr workspace ID;
- the checked-out branch and `HEAD`;
- the worktree status;
- the configured upstream, if any.

Ask the user to continue the session in the returned path. Do not run
`herdr agent start`.

## Guardrails

- Explain that uncommitted changes in the current checkout do not follow into a
  herdr worktree.
- If herdr is unavailable, offer a Codex-managed or user-prepared worktree. Do
  not substitute raw `git worktree add` without approval.
- Do not remove worktrees or delete branches in this skill.
