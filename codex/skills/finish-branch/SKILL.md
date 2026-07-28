---
name: finish-branch
description: Complete a feature branch only when its current head matches fresh verification PASS and clean final review evidence, then carry out the user's publication or disposition choice.
---

# Finish a branch

Do not choose the disposition of the branch for the user.

## Require current-head completion evidence

Inspect:

- current branch and worktree;
- `git status --short`;
- commits and diff against the base branch;
- the current head;
- the fresh verification `PASS` head and exact range;
- the clean final review head and exact range;
- the complete approved review policy, reviewers run, triggered conditionals,
  skipped perspectives with reasons, Acceptance result, and unresolved findings.

Require the current head, fresh verification PASS head, and clean final review
head to exactly match. Also require the approved policy to be fully satisfied and
no unresolved Must Fix or Should Improve finding. A later commit or uncovered
in-scope working-tree change makes verification and clean review evidence stale.

An accepted direct/no-agent independence limitation recorded under the approved
policy is residual evidence and risk, not an unresolved finding or policy gap. It
does not block completion when the policy-selected sequential passes ran and
approved. Preserve the limitation in the completion evidence and final report.

If the approved policy contains a non-waivable independence requirement and the
current evidence used direct/no-agent passes without that independence, the
policy is not satisfied. Do not present completion choices; return the conflict
to the coordinator for `Escalate` with the exact policy or user decision
required.

If any condition fails, do not present completion choices. Return the stale
evidence or policy gap to the coordinator for the appropriate workflow phase.

## Present choices

Only after the current-head completion gate passes, offer the applicable
user-controlled publication or branch disposition choices:

1. publish or push the branch;
2. create a pull request;
3. merge locally into the base branch;
4. keep the branch/worktree as-is;
5. discard the branch/worktree.

Explain dirty-state, publication, and cleanup consequences. Wait for the user's
choice before every publication, pull request, local merge, keep, or discard
disposition. Do not repeat an approval prompt once the exact choice and target are
recorded, except for the immediate destructive confirmation required below.

## Revalidate after the user choice

For publication, push, pull-request creation, local merge, or discard, freshly
reinspect all of these immediately before the state change:

- the current branch and ref plus the exact selected target;
- `HEAD` and `git status --short`;
- the fresh verification PASS head and exact range;
- the clean final review head and exact range;
- the completed approved-policy gate and unresolved-finding state.

Require them to match the completion evidence and target shown when the user made
the choice. If any value differs, do not perform the operation; return the stale
state to the coordinator.

For discard, obtain the existing explicit destructive confirmation first, then
revalidate immediately before deletion. For keep, make no state change; inspect
and report the current state.

## Execute the choice

- For publication or push, write only the exact authorized remote and ref; never
  infer a force push.
- For a PR, use the `create-pr` skill only after that exact external write is
  authorized.
- For a local merge, use the approved local refs, merge non-interactively, and run
  the required post-merge verification. Do not fetch, pull, or push without
  separate authority.
- For keep, make no state change.
- For discard, delete only the confirmed and freshly revalidated exact targets.

Never force-push, delete a branch, remove a worktree, or discard uncommitted data from an implied choice.

Report the resulting branch/worktree state to the coordinator. Do not choose or
start another workflow phase from this skill.
