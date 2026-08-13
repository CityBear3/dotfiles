---
name: finish-branch
description: Complete a feature branch only when its current head matches fresh verification and review evidence with no surviving finding, then carry out the user's publication or disposition choice.
---

# Finish a branch

Do not choose publication or branch disposition for the user.

## Require current-head completion evidence

Inspect the exact target, current evidence shared by all authority forms, and one
authority form. Shared evidence is:

- current branch, exact implementation base, current head, and full range;
- `git status --short`, full diff, and changed files;
- approved scope, non-goals, and Review context;
- fresh coordinator verification `PASS`, including observed commands and results,
  for the same current head and range;
- final review and triage for that same head and range, with no surviving finding;
- complete Review policy, reviewer and integrator outcomes, triggered
  conditionals, skipped perspectives with reasons, Acceptance result, residual
  risk, and every gap.

For new-format planned work also inspect:

- approved Design Doc when applicable, Feature Contract, complete Task Contract
  set, Implementation Plan, and their approval state;
- complete Feature Contract coverage, integration-only obligations, and explicit
  proof of every applicable Feature Contract observation;
- a current accepted result bound to the exact approved content of every Task
  Contract; no stale result may satisfy this evidence.

For lightweight work, accept the complete recoverable combined in-memory
Feature/Task Contract, its original request authority and design sources, exact
accepted-task evidence, and proof of every Feature Contract observation. Require
no unresolved material change or promotion condition. Do not require an
Implementation Plan, contract file, or separate artifact approval.

For a plan approved and already executing before the contract-centered format,
accept its exact approved plan and referenced design sources in place of Feature
and Task Contract artifacts only when unchanged approval and in-flight state,
absence of material ambiguity, and no owner migration choice are established.
Require the same current-head verification, review, triage, safety, and
publication evidence against its original completion criteria. Do not
manufacture new artifacts or weaken the gate.

Resolve base, head, range, diff, and changed files directly from Git. Require the
current head, verification head, and review head and range to match exactly.
Require no unexplained in-scope index, worktree, or untracked source change
outside the committed range.

An `adaptive` or `deep` policy is incomplete when required independent
perspectives were replaced by lead passes. Only an approved `focused` lead pass
can satisfy a no-agent path. Standalone verification or review never substitutes
for coordinator completion evidence.

If verification is stale, policy is incomplete, or a finding or gap remains, do
not present completion choices. Return `BLOCKED` with the observed state and exact
re-entry condition, or `Escalate` when resolution requires a user-owned policy,
authority, scope, or design decision.

For new-format work, also stop when a Task Contract lacks a current accepted
result for its exact approved content, Feature Contract coverage is incomplete,
an integration-only obligation is unproved, or a material artifact edit
invalidated approval. For any authority form, a successful task-local command or
aggregate task count is not feature completion.

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

Before a state-changing operation, re-resolve the selected refs, object IDs,
HEAD, status, relevant files, operation state, and completion evidence. For a
local merge, freeze the reviewed source object and approved destination ref and
object. If the choice evidence is stale or a safe prestate is uncertain, preserve
state and stop before the operation.

For keep, make no state change and report current refs and status. For discard,
revalidate again after explicit confirmation and delete only the exact confirmed
targets.

## Execute a safe local merge

Establish the approved destination checkout or worktree only from a clean
prestate with no conflicting Git operation. Revalidate the destination object and
unchanged frozen source object, then run the recorded non-interactive merge
command naming that source object. If any precondition is uncertain, return
`BLOCKED` without manufacturing a clean state.

On merge failure, inspect the resulting refs, operation state, index, and
worktree. Abort only when this skill started the same attributable merge from the
recorded clean prestate and abort is safe for unrelated data. Otherwise preserve
the partial state and report it; never reset, clean, retry, or discard to recover.

After success, run the required post-merge verification for the resulting
destination head. If it fails, preserve the result and report the failed command
and recovery limits; do not reset, clean, retry, or publish.

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
