---
name: verify
description: Verify an exact Task PR range, an integration-only composed tree, or a standalone target with fresh checks and return PASS, FAIL, or BLOCKED evidence.
---

# Verify a Task PR or integration target

No completion claim without fresh observed evidence.

Remain check-only and read-only with respect to the index, tracked files, and
in-scope source files. Do not edit source, stage changes, create commits, run a
fix, or advance another workflow phase. Verification commands may create normal
ignored build or test artifacts, but must not mutate tracked or in-scope source
state.

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
composition input, or required authority is missing or stale. Standalone
evidence never substitutes for this entry.

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

## Snapshot and run checks

Immediately before the first command, capture:

- target kind, workspace and branch when applicable, current HEAD, planned base,
  merge base, exact range, composed tree, or bounded standalone files;
- index entries, `git status --short`, and staged and unstaged diffs;
- relevant in-scope untracked paths and unrelated dirty state;
- pre-existing command artifacts that matter to the checks.

Run fresh, as applicable:

1. every contractually fixed target verification command;
2. checks selected to observe each assigned Task Contract or integration-only
   Feature Contract obligation;
3. focused tests for changed behavior;
4. owning package or workspace tests;
5. build or type check;
6. lint;
7. format check using only a documented non-mutating mode;
8. relevant integration, smoke, browser, API, or snapshot checks;
9. `git diff --check`, diff inspection, and final status.

For a Task PR, map every assigned Task Contract and Feature Contract obligation
to fresh observed evidence. For integration-only verification, map only the
named remaining obligations and preserve accepted task evidence separately. An
unobserved required obligation is `FAIL` when the current result violates or
omits the contract and `BLOCKED` when its environment or evidence cannot be
established.

For eligible legacy work, map every original approved completion criterion to
fresh evidence instead. A material ambiguity stops verification and returns to
the coordinator; it does not force migration or infer a replacement contract.

Do not replace repository wrappers with broader commands that change semantics.
If a required formatter has no check-only form, return `BLOCKED` without running
it.

After the final command, capture the same HEAD, status, diffs, untracked paths,
and relevant source state. Attribute every change. Normal ignored build artifacts
are allowed when recorded. A tracked or in-scope source mutation caused by
verification is `FAIL`; uncertain ownership is `BLOCKED`. Do not restore, stage,
commit, reset, or clean either state.

A commit or target-content change makes earlier command evidence stale. Do not
return `PASS` unless every required result applies to the unchanged target.

## Evaluate and report

Return exactly one verdict:

- `PASS` — every required command and inspection succeeded for the unchanged
  target;
- `FAIL` — a required command or contract check produced an observed failure;
- `BLOCKED` — a command, dependency, permission, input, range, or current-head
  guarantee could not be established.

Report:

- verdict and Task PR, integration-only, eligible legacy, or standalone status;
- workspace, branch, planned base, merge base, starting and ending head, exact
  range, composed tree, snapshot, or bounded fileset;
- starting and ending `git status --short`, changed files, and unrelated state;
- for dispatched checks, the owning execution context, configured, observed,
  and effective capacity, root grant, live identities, and queue order;
- Review context and approved criteria inspected when available;
- approved Design Doc, Feature Contract, applicable Task Contract and dependency
  evidence, integration-only obligation and accepted task set, complete
  lightweight contract, or eligible legacy authority inspected;
- each assigned task, integration-only, lightweight, or legacy criterion, its
  evidence, and pass, fail, or blocked result;
- every command, expected result, observed result, and match status;
- checks not run and why;
- for `FAIL` or `BLOCKED`, the failed command or unmet guarantee, likely
  ownership, every gap, and the exact condition for safe re-entry.

For coordinator-managed verification, return evidence to the coordinator and do
not start review. For standalone verification, report directly to the requester.
Never diagnose or implement a fix from this skill.
