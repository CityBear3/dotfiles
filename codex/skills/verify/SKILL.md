---
name: verify
description: Verify a current implementation head with fresh project commands and return PASS, FAIL, or BLOCKED evidence. Use from the workflow coordinator after implementation or standalone for a read-only completion check.
---

# Verify the current implementation head

No completion claim without fresh observed evidence.

Remain check-only and read-only with respect to the index, tracked files, and
in-scope source files. Do not edit source, stage changes, create commits, run a
fix, or advance another workflow phase. Verification commands may create normal
ignored build or test artifacts, but must not mutate tracked or in-scope source
state.

## Resolve the requested target

Use one target form:

- a coordinator-managed committed range with exact base, current head, and
  `base..head` range;
- a standalone committed range;
- a standalone current index/worktree snapshot;
- a standalone bounded explicit fileset.

For any target, inspect applicable repository guidance and record current HEAD,
`git status --short`, changed files, requested scope, authoritative commands, and
known limitations before running checks.

## Coordinator-managed entry

Require:

- the original implementation base, current head, and exact full committed
  range;
- current status and changed files, with no unexplained in-scope index,
  worktree, or untracked source change outside that range;
- approved scope, decisions, non-goals, Review context, and Review policy;
- the implementation plan when present;
- every authoritative final verification command and expected result;
- task commits, task-review outcomes, concerns, and known gaps.

Resolve the base, head, range, changed files, and diff directly from Git. Require
repository HEAD to equal the supplied current head. Return `BLOCKED` before
checks when a ref or range is missing, status does not match, an in-scope change
exists outside the range, or another required input cannot be established.

Coordinator completion requires fresh evidence for this exact current head.
Standalone evidence never substitutes for this entry.

## Standalone read-only entry

Resolve the user's requested scope through local read-only investigation:

- For a committed range, record base, head, range, diff, changed files, and
  current status.
- For an index/worktree review, record HEAD, staged and unstaged status and
  diffs, relevant untracked paths, and bounded changed files.
- For an explicit fileset, record the exact bounded paths, current status, and
  what content was inspected.

Require applicable repository guidance and authoritative verification commands.
Use available plan, decision, Review context, and policy evidence when present.
Do not require implementation authorization or an approved policy for a
standalone check.

Return `BLOCKED` when the requested scope or authoritative commands cannot be
resolved safely. Report assumptions and limitations. A worktree or fileset result
may answer the direct request, but cannot satisfy the coordinator's committed
current-head gate.

## Select a compatible executor

Before selecting a named verifier, inspect its effective sandbox and complete
instructions. A compatible verifier must prohibit index, tracked-file, and
in-scope source mutation and must not permit formatter output into those files.

The current `implementation-verifier` profile uses a workspace-write sandbox and
permits formatter output, so it is incompatible with this check-only phase. Use
a compatible read-only route when available; otherwise the lead runs the checks
under this contract. Unavailability is not permission to weaken the boundary.

## Snapshot and run checks

Immediately before the first command, capture:

- current HEAD and target base/range or bounded standalone files;
- index entries, `git status --short`, and staged and unstaged diffs;
- relevant in-scope untracked paths and unrelated dirty state;
- pre-existing command artifacts that matter to the checks.

Run fresh, as applicable:

1. approved final verification commands;
2. focused tests for changed behavior;
3. owning package or workspace tests;
4. build or type check;
5. lint;
6. format check using only a documented non-mutating mode;
7. relevant smoke or snapshot checks;
8. `git diff --check`, diff inspection, and final status.

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

- verdict and coordinator-managed or standalone status;
- base, starting and ending head, exact range, snapshot, or bounded fileset;
- starting and ending `git status --short`, changed files, and unrelated state;
- Review context and approved criteria inspected when available;
- every command, expected result, observed result, and match status;
- checks not run and why;
- for `FAIL` or `BLOCKED`, the failed command or unmet guarantee, likely
  ownership, every gap, and the exact condition for safe re-entry.

For coordinator-managed verification, return evidence to the coordinator and do
not start review. For standalone verification, report directly to the requester.
Never diagnose or implement a fix from this skill.
