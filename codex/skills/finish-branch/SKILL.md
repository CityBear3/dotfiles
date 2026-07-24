---
name: finish-branch
description: Complete verified work on a feature branch by presenting publication, local merge, keep, or discard choices and carrying out the selected option. Use after review is clean or when the user explicitly asks to finish the branch.
---

# Finish a branch

Do not choose the disposition of the branch for the user.

## Verify the state

Inspect:

- current branch and worktree;
- `git status --short`;
- commits and diff against the base branch;
- fresh test, lint, format, and review evidence required by the repository.

If commits were added after the latest clean review, report that verification/review is stale before offering completion.

## Present choices

Offer only applicable choices:

1. create a pull request;
2. merge locally into the base branch;
3. keep the branch/worktree as-is;
4. discard the branch/worktree.

Explain dirty-state, publication, and cleanup consequences. Wait for the user's choice.

## Execute the choice

- For a PR, use the `create-pr` skill.
- For a local merge, update the base safely, merge non-interactively, and run the required post-merge verification.
- For keep, make no state change.
- For discard, resolve exact targets and obtain explicit confirmation immediately before deletion.

Never force-push, delete a branch, remove a worktree, or discard uncommitted data from an implied choice.

Report the resulting branch/worktree state and then use session teardown if the work is finished.
