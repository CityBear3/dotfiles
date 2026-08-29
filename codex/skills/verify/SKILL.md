---
name: verify
description: Verify an exact Task PR range, an integration-only composed tree, or a standalone target with fresh checks and return PASS, FAIL, or BLOCKED evidence.
---

# Execute a current-head Verification Matrix

No completion claim without fresh observed evidence.

Remain check-only and read-only with respect to the index, tracked files, and
in-scope source files. Do not edit source, stage changes, create commits, run a
fix, or advance another workflow phase. Verification commands may create normal
ignored build or test artifacts, but must not mutate tracked or in-scope source
state.

Execute the supplied Verification Matrix mechanically. Do not perform semantic
review, contract-quality judgment, architecture or scope review,
maintainability review, or test-adequacy review. Those decisions belong to the
policy-selected reviewers after a fresh verification `PASS`.

For new-format planned work, look up a current matching cache entry before new discovery.
A cache hit never replaces fresh Git, authority, verification, or review evidence.
Use it only to navigate a source whose identity and invalidation conditions
still match, and return attributable cache candidates to the Feature lead
separately from the completed matrix. The verifier never edits
`search-cache.md`.

## Resolve the requested target

Use one target form:

- a coordinator-managed Task PR with exact planned base, branch, current head,
  merge base, and range;
- a coordinator-managed integration-only composed tree with its exact accepted
  Task PR heads and composition order;
- an eligible legacy coordinator-managed committed range;
- a standalone committed range;
- a standalone current index/worktree snapshot;
- a standalone bounded explicit fileset.

For any target, inspect applicable repository guidance and record current HEAD,
`git status --short`, changed files, requested scope, authoritative commands, and
known limitations before running checks.

## Coordinator-managed entry

Require one exact coordinator target and its authority.

For every coordinator-managed target, require one completed-input current-head
Verification Matrix mapping every applicable obligation to a bounded command or
check, expected observation, and `FAIL` or `BLOCKED` non-match category.

For a Task PR require:

- Task Contract and PR identities, task workspace and branch, planned base ref
  and exact commit, merge base, current head, and exact committed range;
- current status and changed files, with no unexplained in-scope index,
  worktree, or untracked source change outside that range;
- approved scope, non-goals, Review context, and Review policy;
- every authoritative task verification route and expected observation, with
  exact commands where their identity is contractually significant;
- task commits, current dependency and shared-interface evidence, concerns, and
  known gaps;
- for new-format planned work, the exact `search-cache.md` path and any current
  matching entry with its source identity and invalidation conditions;
- the approved Feature Contract and applicable Task Contract with their source
  and currentness evidence, or the complete lightweight combined contract.

Do not require an `Accepted` result or prior task review: this verification is
part of producing that result.

For a planned integration-only target require:

- the approved Design Doc when applicable, Feature Contract, complete
  Implementation Plan and Task Contract set, and their approval state;
- every current accepted Task PR result with exact base, head, range, and
  topology evidence;
- the exact temporary composition, starting and ending tree identity, and only
  the Feature Contract obligations classified as integration-only;
- approved Review context and policy, concerns, and known gaps.

Do not rerun task-scoped obligations merely because the composed tree contains
their changes. Task acceptance supports coverage but never substitutes for a
named integration-only observation.

For a lightweight integration-only target require:

- the complete recoverable combined in-memory Feature/Task Contract, original
  request authority and design sources, Review context, and current policy;
- the current exact accepted lightweight Task PR with base, head, tree, range,
  status, verification, review, and triage evidence;
- that accepted head and tree as the exact integration target and the named
  integration-only obligation and expected observation;
- no unresolved promotion condition, material contract change, or stale state.

Do not require a Design Doc, contract file, Implementation Plan, Task DAG,
multi-PR topology, or temporary multi-head composition for this authority form.
Verify only the named integration-only observation; do not rerun its task gate.

For a lightweight Task PR target, accept the complete combined in-memory
Feature/Task Contract, its original request authority and design sources, and
the exact Task PR target. Require that the contract remains completely
recoverable and no promotion condition or material change is unresolved. This
Task PR verification also provides feature evidence when no integration-only
obligation exists. Do not require an Implementation Plan, contract file, or
separate artifact approval.

For a plan approved and already executing before the contract-centered format,
accept its exact approved plan and referenced design sources in place of Feature
and Task Contract artifacts only when the coordinator supplies unchanged
approval and in-flight evidence, no material ambiguity, and no owner migration
choice. Use its original scope, task specifications, verification and completion
criteria, Review context, and Review policy. Do not manufacture new artifacts or
weaken current-head evidence.

Resolve the applicable workspace, branch, base, head, merge base, range, tree,
changed files, and diff directly from Git. Require that workspace HEAD and status
match the supplied target. Return `BLOCKED` before checks when any identity,
composition input, required authority, or matrix row is missing, contradictory,
incomplete, or stale. The matrix is valid only for its supplied head, range,
controlling authority, and material verification route. Standalone evidence
never substitutes for this entry.

## Standalone read-only entry

Resolve the user's requested scope through local read-only investigation:

- For a committed range, record base, head, range, diff, changed files, and
  current status.
- For an index/worktree review, record HEAD, staged and unstaged status and
  diffs, relevant untracked paths, and bounded changed files.
- For an explicit fileset, record the exact bounded paths, current status, and
  what content was inspected.

Require applicable repository guidance and authoritative verification routes,
including exact commands only when their identity is part of the requested or
repository contract. Use available Design Doc, Feature Contract, Task Contracts,
plan, Review context, and policy evidence when present.
Do not require implementation authorization or an approved policy for a
standalone check.

Before dispatch, have the root bind every applicable standalone obligation to a
bounded command or check, expected observation, and `FAIL` or `BLOCKED`
non-match category. This in-memory matrix is evidence for the bounded snapshot,
not a persistent coordination schema.

Return `BLOCKED` when the requested scope or authoritative commands cannot be
resolved safely. Report assumptions and limitations. A worktree or fileset result
may answer the direct request, but cannot satisfy the coordinator's committed
current-head gate.

## Select a compatible executor

Before selecting a named verifier, inspect its effective sandbox and complete
instructions. A compatible verifier must prohibit index, tracked-file, and
in-scope source mutation, allow writes only for normal ignored test or build
artifacts, prohibit formatter output, and require documented non-mutating format
checks. The named `implementation-verifier` profile is the compatible verifier
when its effective instructions retain those boundaries; its workspace-write
sandbox exists only for the bounded ignored artifacts and does not weaken the
check-only contract.

For a new-format planned Task PR, `execute-task` selects the exact
`implementation-verifier` role and the bound Task orchestrator dispatches that
leaf through `agent-teams-driven-development` under the current root-granted
lease. For a lightweight Task PR, the root dispatches the same selected verifier
leaf through that adapter. For an eligible legacy Task, preserve its exact
approved invoking context. The verifier is always a leaf and may not spawn
descendants.

For another coordinator-managed target, the root dispatches the compatible
named verifier through the same adapter under the applicable capacity policy.
If a required compatible verifier cannot be instantiated, return `BLOCKED` with
the role, capacity, queue, and exact re-entry condition. Do not substitute the
root, Task orchestrator, or another role, and do not weaken a planned or
lightweight gate.

For a standalone target, the root normally dispatches the compatible named
verifier as its direct leaf through `agent-teams-driven-development`. Record the
standalone execution context, configured, observed, and effective global
subagent capacity, live identities, a root-granted target-local count of
normally one and at most three concurrent leaves, and the selected-role queue.
The target has no Task lease and may not consume capacity beyond that grant.
Only an explicitly requested no-agent execution may let the lead run these
checks under this complete check-only contract. Report either form as
`standalone-only`, never as coordinator or Acceptance evidence.

## Execute the matrix in mechanical fail-fast order

Immediately before the first command, confirm the exact target identity and
matrix binding. For a clean isolated planned Task PR, capture only the expected
workspace and branch, current HEAD, planned base, merge base, exact committed
range, required clean `git status --short`, changed-file inventory, diff check,
and pre-existing command artifacts that matter to the checks. Complete Task
topology, attribution, and semantic diff ownership remain with the Task-loop
owner.

For a standalone dirty index/worktree snapshot, staged, unstaged, untracked, and
bounded-file state is part of the target. Capture the fuller index entries,
status, staged and unstaged diffs, relevant untracked paths, bounded-file
fingerprint, unrelated dirty state, and pre-existing artifacts before checking.

Run applicable matrix rows fresh in exactly this order:

1. target identity and required clean-state precondition;
2. exact range, changed-file inventory, `git diff --check`, and bounded diff
   consistency;
3. format check using only the documented non-mutating mode;
4. focused behavior tests;
5. build or type check;
6. lint;
7. owning package, workspace, or full tests;
8. integration, smoke, browser, API, or snapshot checks;
9. final head and mutation-invariant comparison.

Run every contractually fixed command in its applicable ordered row. A
conclusive failure stops later dependent or more expensive rows. Record each
unrun matrix row and the failure or blocked prerequisite that prevented it.
Batch independent mechanical commands only when their individual status and
output remain attributable, no semantic decision is needed between them, the
first conclusive mismatch remains visible, and the final mutation check still
runs.

For a Task PR, complete every supplied matrix row with fresh observed evidence.
For integration-only verification, complete only rows for the named remaining
obligations and preserve accepted task evidence separately. An observed
mechanical mismatch is `FAIL`; an unavailable command, dependency, permission,
environment, evidence input, or target guarantee is `BLOCKED`. Do not infer or
semantically invent a missing row.

For eligible legacy work, map every original approved completion criterion to
fresh evidence instead. A material ambiguity stops verification and returns to
the coordinator; it does not force migration or infer a replacement contract.

Do not replace repository wrappers with broader commands that change semantics.
If a required formatter has no check-only form, return `BLOCKED` without running
it.

After the final command or an earlier conclusive stop, run the final head and
mutation-invariant comparison. For a clean planned target, compare the head,
clean status, and tracked or in-scope source state, recording normal ignored
build artifacts. For a dirty standalone target, repeat the full bounded
fingerprint. A tracked or in-scope source mutation caused by verification is
`FAIL`; uncertain ownership is `BLOCKED`. Do not restore, stage, commit, reset,
or clean either state.

A commit or target-content change makes earlier command evidence stale. Do not
return `PASS` unless every required result applies to the unchanged target.

## Evaluate and report

Return the completed Verification Matrix and exactly one verdict:

- `PASS` — every required command and inspection succeeded for the unchanged
  target;
- `FAIL` — a required command or contract check produced an observed failure;
- `BLOCKED` — a command, dependency, permission, input, range, or current-head
  guarantee could not be established.

Keep the report compact and do not repeat unchanged authority, Review policy,
capacity, queue, or contract prose already held by the owner. Return:

1. one target block containing the target form and only its applicable identity
   fields: workspace, branch, base, merge base, range, composed tree, snapshot,
   or bounded fileset; starting head and `git status --short`; changed files and
   unrelated state only when they are part of the target guarantee;
2. one row table in input order with exactly
   `ID | command/check | expected | observed | result`; use supplied row IDs or
   assign report-local `V1`, `V2`, and so on without changing matrix meaning;
3. one final-state block containing the ending head, mutation-invariant result,
   and allowed artifacts actually observed;
4. unrun row IDs with one reason each;
5. a consulted cache entry or attributable cache candidate only when present;
6. gaps only when present and, for `FAIL` or `BLOCKED`, the failed row or unmet
   guarantee, likely ownership when known, and exact safe re-entry condition;
7. exactly `PASS`, `FAIL`, or `BLOCKED`.

For coordinator-managed verification, return evidence to the coordinator and do
not start review. For standalone verification, report directly to the requester.
Never diagnose or implement a fix from this skill. A verifier `PASS` proves only
that every mechanical matrix observation matched on the unchanged target; it is
not semantic review or Task acceptance.
