---
name: finish-branch
description: Complete a feature branch only when its current head matches fresh verification PASS and clean final review evidence, then carry out the user's publication or disposition choice.
---

# Finish a branch

Do not choose publication or branch disposition for the user.

## Require current-head completion evidence

Inspect:

- current branch, exact implementation base, current head, and full range;
- `git status --short`, full diff, and changed files;
- approved scope, decisions, non-goals, and Review context;
- fresh coordinator verification `PASS`, including observed commands and results,
  for the same current head and range;
- final review `CLEAN` for that same head and range;
- approved Review policy, reviewer and integrator outcomes, triggered
  conditionals, skipped perspectives with reasons, Acceptance result, residual
  risk, and every gap.

Resolve base, head, range, diff, and changed files directly from Git. Require the
current head, verification head, and review head and range to match exactly.
Require no unexplained in-scope index, worktree, or untracked source change
outside the committed range.

An `adaptive` or `deep` policy is incomplete when required independent
perspectives were replaced by lead passes. Only an approved `focused` lead pass
can satisfy a no-agent path. Standalone verification or review never substitutes
for coordinator completion evidence.

If verification is stale, review is not clean, policy is incomplete, or a
finding or gap remains, do not present completion choices. Return `BLOCKED` with
the observed state and exact re-entry condition, or `Escalate` when resolution
requires a user-owned policy, authority, scope, or design decision.

## Present choices

Only after the current-head gate passes, offer applicable user-controlled
choices:

1. publish or push the branch;
2. create a pull request;
3. merge locally into the base branch;
4. keep the branch/worktree as-is;
5. discard the branch/worktree.

Explain dirty-state, publication, and cleanup consequences. Wait for the user's
choice before every publication, pull request, local merge, keep, or discard
action. For discard, also require immediate destructive confirmation.

## Revalidate after the user choice

When the user selects a local merge, freeze the selection-time source ref and
object ID plus the selected base destination ref and object ID. Keep that source
ref and object ID fixed throughout the authorized merge attempt.

Immediately before a state-changing operation, recheck:

- current source and destination refs and exact object IDs;
- current branch, HEAD, base, range, diff, changed files, and
  `git status --short`;
- relevant untracked and ignored paths;
- absence or exact presence of merge, rebase, cherry-pick, or other operation
  state;
- fresh verification and final review head/range;
- approved-policy completion and unresolved findings or gaps.

Require these values to match the evidence shown when the user chose. If any
value differs, preserve state and return the stale evidence without starting the
operation.

For keep, make no state change and report current refs and status. For discard,
revalidate again after explicit confirmation and delete only the exact confirmed
targets.

## Execute a safe local merge

After revalidating the selection evidence and before merging, explicitly
establish the destination checkout. Use an existing worktree for the selected
destination ref when one is available and safe. Otherwise, as normal preparation
for the authorized merge, use the exact recorded non-interactive checkout or
switch command to move the clean current worktree to that destination ref. Do not
start this preparation unless the frozen source ref and object ID, destination
ref and object ID, clean state, and absence of active operations still match the
selection-time evidence.

If the destination checkout or worktree cannot be established safely, return
`BLOCKED` without starting the merge. Preserve the observed refs and files and
report the exact re-entry condition.

Once the destination is established, record:

- exact source and destination refs and object IDs;
- checked-out branch, HEAD, index entries, worktree status, and diff;
- relevant pre-existing untracked and ignored paths and their understood
  ownership;
- absence or presence of `MERGE_HEAD`, unmerged entries, and other active Git
  operations;
- exact non-interactive merge command and required post-merge verification.

Require the checked-out destination branch and HEAD to equal the selected
destination ref and object ID, the source ref to resolve to the unchanged frozen
source object ID, clean tracked, index, and worktree prestate, no active or
conflicting Git operation, and no unidentified relevant untracked or ignored
material that the merge, hooks, or checks could overwrite. If a precondition
cannot be established, return `BLOCKED` with observed state and the exact
re-entry condition. Do not reset, clean, move, or discard material to manufacture
a clean prestate.

From that destination worktree, run only the recorded exact non-interactive merge
command naming the frozen source ref. On conflict or merge failure, inspect refs,
HEAD, `MERGE_HEAD`, unmerged entries, index, worktree, relevant untracked and
ignored paths, and status before any recovery.

Run `git merge --abort` only when:

- this skill started the currently active merge from the recorded clean prestate;
- refs, operation state, and observed files show it is the same attributable
  attempt;
- abort is available and does not risk unrelated or pre-existing data.

After abort, recheck source and destination refs, checked-out branch, HEAD, index,
worktree status and diff, relevant untracked and ignored paths, and Git operation
state against the recorded prestate. If restoration cannot be established, do
not claim success, reset, clean, retry, or discard anything. Preserve and report
the partial state for the user's decision.

After a successful merge, run the required post-merge verification against the
resulting destination ref and head. If it fails, abort only when the same merge
attempt is still active and the safety conditions above hold. When a merge
commit, fast-forward, changed ref, or other state makes safe abort unavailable,
preserve every ref and file and report the failed command, status, partial
result, and recovery limitation. Do not reset, clean, retry, or publish.

## Execute the selected choice

- For publication or push, write only the exact authorized remote and ref; never
  infer force push.
- For a pull request, use `create-pr` only after that external write is
  authorized.
- For local merge, follow the safe merge procedure above. Do not fetch, pull, or
  push without separate authority.
- For keep, make no state change.
- For discard, remove only freshly revalidated and explicitly confirmed targets.

Never force-push, delete a branch, remove a worktree, reset, clean, or discard
uncommitted data from an implied choice. Never touch unrelated state.

Report resulting refs, branch/worktree, current head, range, status, changed
files, commands and observed results, verification, any abort or recovery action,
preserved partial state, concerns, and every gap. Do not choose or start another
workflow phase.
