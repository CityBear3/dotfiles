---
name: finish-branch
description: Finish an internally accepted Task PR branch or a Feature Accepted topology, preserving exact evidence and user control over publication, merge, cleanup, and disposition.
---

# Finish a Task PR or accepted feature

Do not choose publication, merge, cleanup, or branch disposition for the user.

## Select one completion mode

Use exactly one mode:

- **Task mode:** an exact Task PR is internally `Accepted` and may be published
  before the rest of the feature is accepted.
- **Lightweight mode:** one exact lightweight Task PR is internally `Accepted`
  and therefore also Feature Accepted under its recoverable combined contract.
- **Feature mode:** every Task PR and integration-only obligation is current and
  the coordinator returned Feature Accepted.
- **Eligible legacy mode:** follow the unchanged completion contract of a plan
  already executing before PR-scoped task execution.

Do not use planned task mode as feature completion or require Feature Accepted
before an individual planned Task PR may be published. Do not force lightweight
work into planned task or feature evidence forms.

## Require current Task PR evidence

For task mode inspect:

- approved Feature Contract, applicable Task Contract, Implementation Plan,
  Review context and policy, and their currentness;
- Task DAG and PR topology position, workspace, branch, planned base ref and
  exact commit, merge base, current head, exact range, status, diff, changed
  files, and commits;
- fresh task verification `PASS` and a policy-complete gate closed by review
  `CLEAN` or by integrated review `FINDINGS` whose every item has current `Push
  back` triage for that same unchanged range;
- current logical dependencies, shared interfaces, and ancestor evidence;
- publication state, human-feedback state, concerns, and every gap.

Require no unexplained in-scope state and no candidate or stale result. Resolve
the branch, base, head, merge base, range, status, and PR topology directly from
Git. A successful local command, writer report, or preliminary common-base check
is not task acceptance.

Task mode must not remove, archive, stage, or commit the active Feature Contract
or Implementation Plan. Those artifacts remain necessary for dependents,
staleness propagation, human-feedback re-entry, and feature acceptance.

## Require current lightweight evidence

For lightweight mode inspect:

- the complete recoverable combined in-memory Feature/Task Contract, original
  request authority and design sources, Review context, and Review policy;
- its exact workspace, branch, planned base ref and commit, merge base, current
  head, range, status, diff, changed files, and commits;
- fresh verification `PASS` and a policy-complete gate closed by `CLEAN` or by
  same-target integrated `FINDINGS` with every item currently classified `Push
  back`;
- the coordinator's Feature Accepted result for that unchanged exact Task PR;
- publication state, human-feedback state, concerns, and every gap.

Require no unresolved promotion condition, material contract change,
unexplained in-scope state, candidate, or stale result. Resolve the Git evidence
directly. Do not require a Design Doc, Feature Contract file, Task Contract file,
Implementation Plan, Task DAG, multi-PR topology, integration composition, or
separate artifact approval when the combined contract supplies the authority.

## Require current Feature Accepted evidence

For feature mode inspect:

- approved Design Doc when applicable, Feature Contract, complete Task Contract
  set, Implementation Plan, Review context and policy;
- both exact topologies and one current authoritative `Accepted` result for
  every Task PR;
- complete Feature Contract coverage and every integration-only verification
  and targeted review result;
- all task workspaces, branches, bases, heads, ranges, publication states,
  triage decisions, temporary integration workspaces or refs and their cleanup
  eligibility, concerns, and gaps;
- the coordinator's Feature Accepted result for those unchanged inputs.

Re-resolve every affected ref and workspace. Return `BLOCKED` if a task is a
candidate or stale, topology or status changed, coverage is incomplete, an
integration-only obligation is unproved, or a finding or gap survives. Do not
rerun an ordinary full-feature verification or review to manufacture feature
completion.

## Require eligible legacy evidence

For eligible legacy mode, require its exact approved plan and referenced design
sources, unchanged approval and in-flight status, original completion criteria,
current branch, base, head, range, status, verification, review, triage, and
publication evidence. Require no material ambiguity or owner migration choice.
Do not manufacture new contract artifacts, Task PR topology, or weaker evidence.
Apply artifact retention or retirement only when and as its unchanged completion
contract requires; do not impose the new planned-feature lifecycle.

## Keep workspace-only artifacts with their worktree

Do not remove the planned feature's ignored `feature-contract.md` or
`implementation-plan.md` as a separate Feature Accepted action. Confirm that
both remain ignored, untracked, unstaged, and inside the current feature plan
directory. Keep them in the coordination worktree while they may still be
needed for publication, human-feedback re-entry, or disposition evidence.

When the user later authorizes removal of that exact coordination worktree and
its retained evidence is no longer required, let removal of the worktree clean
up these ignored files with the workspace. Warn that they are not recoverable
from Git. If either artifact is tracked, staged, outside the expected directory,
or the user requests preservation beyond the worktree lifecycle, return
`Escalate` for an explicit retention or archival decision. Preserve every
Design Doc.

Lightweight mode has no workspace-only contract or plan files. Proceed directly
from its current completion evidence to user-controlled choices.

## Present applicable choices

In task mode present only choices applicable to that exact Task PR:

1. push its current branch;
2. create its PR against the planned base;
3. keep it local and continue eligible plan work;
4. merge it only when the user explicitly requests that disposition and current
   PR topology permits the merge;
5. discard its branch or worktree with separate destructive confirmation.

In lightweight mode present the same exact-branch publication, PR, merge, keep,
and separately confirmed discard choices, but do not refer to continued plan
work or planned workspace-artifact lifecycle.

In feature mode present the remaining choices for the complete topology:

1. publish any still-local accepted Task PRs;
2. merge or land current PRs in topology order;
3. keep branches and worktrees as-is;
4. clean up exact task or integration branches and worktrees only after their
   retention is no longer required and the user explicitly confirms destructive
   targets.

In eligible legacy mode present only the publication and disposition choices
defined by its unchanged approved completion contract. Do not add new topology
or workspace-artifact requirements.

Explain dirty state, stack dependencies, human-review invalidation, and cleanup
consequences. Wait for the user's choice before every external write, merge,
keep, or destructive action. One bounded authorization may cover only the exact
refs and operations it names.

## Revalidate before a state change

Before any selected operation, re-resolve:

- mode, Task and Feature authority;
- exact local and remote refs, planned PR base, branch and head object IDs;
- task and descendant acceptance state;
- status, changed files, worktrees, and active Git operation state;
- prior publication and merge state.

If the evidence changed, preserve state and stop. A parent update may make
descendant evidence stale; return the topology to the coordinator instead of
publishing or merging it as current.

## Execute a safe local merge

Freeze the reviewed source object and approved destination ref and object.
Establish the destination checkout only from a clean prestate with no conflicting
Git operation. Revalidate the unchanged frozen source, then run the recorded
non-interactive merge naming that source object.

On failure, inspect refs, operation state, index, and worktree. Abort only when
this skill started the same attributable merge from the recorded clean prestate
and abort is safe for unrelated data. Otherwise preserve partial state; never
reset, clean, retry, or discard to recover.

After success, run the required post-merge verification for the destination
head. If it fails, preserve and report the result; do not reset, publish, or
continue landing descendants.

## Execute the selected choice

- For publication or push, write only the exact authorized remote and ref;
  never infer force push or retargeting.
- For a pull request, use `create-pr` only after its exact external write is
  authorized.
- For local merge, follow the safe merge procedure and current topology order.
- For keep, make no state change.
- For discard, revalidate and remove only freshly confirmed exact targets.

Never force-push, retarget a PR, delete a branch, remove a worktree, reset,
clean, or discard data from an implied choice.

Report mode, resulting topology and refs, affected branches and worktrees,
current heads and ranges, status, commands and observed results, verification,
publication or merge state, preserved partial state, concerns, and every gap.
Do not choose or start another workflow phase.
