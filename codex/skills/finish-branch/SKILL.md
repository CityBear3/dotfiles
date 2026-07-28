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
- the exact current HEAD, coordinator-frozen immutable full implementation
  target identity, and coordinator target request passed verbatim;
- the strict coordinator-managed fresh verification `PASS` target, head, and
  exact full range;
- the coordinator-managed final review `CLEAN` target, head, and exact full
  range;
- the complete approved review policy, reviewers run, triggered conditionals,
  skipped perspectives with reasons, actual-risk reconciliation, Acceptance
  result, and unresolved findings or gaps.

Require the current head, fresh verification PASS head, and clean final review
head, target identity, and full range to exactly match. Require the complete
approved policy and actual-risk inventory to be fully satisfied with no unresolved
`Must Fix`, `Should Improve`, schema, policy, evidence, or runtime gap. A later
commit or uncovered in-scope index, worktree, or untracked source change makes
verification and clean review evidence stale. Standalone-only verification or
review evidence never satisfies this gate.

Validate the frozen identity against its bound base and head Git objects, range
and diff contents, changed files, and current repository state. Never rename,
regenerate, or substitute the identity or target request in this skill.

An `adaptive` or `deep` policy is unsatisfied when required independent
perspectives were replaced by sequential lead passes. Only the approved
`focused` lead-pass contract may satisfy a no-agent completion path. Never treat
an independence gap as completion evidence.

If any condition fails, do not present completion choices. Return the stale
evidence or policy gap to the coordinator as `BLOCKED`, or return `Escalate` when
resolution requires a user-owned policy, authority, or target decision. Include
the exact condition required for safe re-entry.

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
- the coordinator-frozen target identity and exact target request verbatim;
- exact source and target ref object IDs;
- `HEAD` and `git status --short`;
- index/worktree and untracked evidence;
- absence or exact presence of merge, rebase, cherry-pick, or other operation
  state;
- the fresh verification PASS head and exact range;
- the clean final review head and exact range;
- the completed approved-policy gate and unresolved-finding state.

Require them to match the completion evidence and target shown when the user made
the choice. If any value differs, do not perform the operation; return the stale
state to the coordinator.

For discard, obtain the existing explicit destructive confirmation first, then
revalidate immediately before deletion. For keep, make no state change; inspect
and report the current state.

## Execute a safe local merge

Immediately before a user-selected local merge, record an immutable pre-merge
snapshot containing:

- exact source branch/ref and object ID;
- exact target branch/ref and object ID;
- checked-out branch, HEAD, index entries, worktree status and diff, and
  tracked content identities;
- every preexisting untracked or ignored path that the exact merge command,
  configured hooks, or required post-merge checks can touch, with bounded path
  inventory and immutable content identity;
- a separate inventory of artifacts that explicit repository policy identifies
  as disposable, with the policy evidence; never infer disposability from an
  ignored status alone;
- absence or presence of `MERGE_HEAD`, unmerged entries, and every other active
  Git operation;
- the exact merge command and required post-merge verification.

Require the safe precondition to have clean tracked/index/worktree state,
attributable bounded untracked and ignored state, and no active merge or
conflicting operation. Relevant untracked or ignored content need not be absent,
but do not start a merge from unrelated or unidentified material state or when
exact refs and ownership cannot be established. Before starting the merge,
return `BLOCKED` when any relevant material untracked or ignored state cannot be
safely bounded and content-identified. Include the stable gap key, likely
ownership, observed state, and exact re-entry condition; never claim that
unrecorded content can be restored exactly.

Run only the exact authorized non-interactive local merge. On conflict or merge
failure, inspect `MERGE_HEAD`, unmerged entries, refs, HEAD, index, worktree,
the bounded untracked and ignored path/content identities, policy-declared
disposable artifacts, and status before taking recovery action.

Run `git merge --abort` only when this skill started the merge from the recorded
clean state, the repository still represents that same merge attempt, and abort
is safe for all recorded data. After abort, require exact restoration of the
pre-merge source and target refs, checked-out branch, HEAD, tracked contents,
index entries, worktree status and diff, every bounded untracked and ignored
path/content identity, and the exact absence or preexisting presence of each Git
operation state. Compare policy-declared disposable artifacts separately and
report any change without using them as proof for material-state restoration.
If exact restoration of recorded material state cannot be proven, never claim
it; do not reset, clean, discard, or retry. Preserve the partial state and return
`Escalate` for the exact user decision, or `BLOCKED` for an external/runtime
condition with an exact re-entry condition.

After a successful merge, run the required post-merge verification against the
resulting exact target ref and HEAD. If it fails and the same merge remains
safely abortable, abort and prove exact pre-state restoration as above. If a
merge commit, fast-forward, changed ref, or other resulting state means safe
abort is unavailable, preserve every resulting ref and file. Do not reset,
publish, retry, or clean up, including untracked or ignored state. Report the
failed command, refs, HEAD, tracked/index/worktree state, bounded untracked and
ignored content identities, merge state, and recovery limitation for the user's
decision.

## Execute the choice

- For publication or push, write only the exact authorized remote and ref; never
  infer a force push.
- For a PR, use the `create-pr` skill only after that exact external write is
  authorized.
- For a local merge, follow the safe local-merge transaction above. Do not fetch,
  pull, or push without separate authority.
- For keep, make no state change.
- For discard, delete only the confirmed and freshly revalidated exact targets.

Never force-push, delete a branch, remove a worktree, reset, clean, or discard
uncommitted data from an implied choice. Never touch unrelated dirty data.

Report the coordinator-frozen target identity and target request verbatim, the
resulting exact refs, branch/worktree, operation state, verification, recovery
action, and every gap to the coordinator. Do not choose or start another workflow
phase from this skill.
