---
name: verify
description: Verify a current implementation head with fresh project commands and return PASS, FAIL, or BLOCKED evidence. Use from the workflow coordinator after implementation or standalone for a read-only completion check.
---

# Verify the current implementation head

No completion claim without fresh observed evidence.

Remain check-only and read-only with respect to the index, tracked files, and
in-scope source files. Do not edit source, stage changes, create commits, run a
fix, or advance another workflow phase. Verification commands may create their
normal ignored build or test artifacts, but they must not mutate tracked or
in-scope source state.

## Resolve one verification target

Use exactly one target form:

- an exact committed range identified by base commit, head commit, range, and
  diff;
- a captured current index/worktree snapshot identified by HEAD, index state,
  staged diff, worktree diff, and path/content identities for in-scope untracked
  files;
- a bounded explicit fileset identified by path inventory and immutable content
  identities for every inspected file.

Identity ownership depends on the entry route. For a coordinator-managed entry,
the coordinator supplies an exact target request and this skill resolves the
content-bound immutable identity exactly once as defined below. For a standalone
entry, this skill creates its own standalone-only stable identity under the
selected target schema. In both routes, record HEAD, index/worktree and in-scope
untracked evidence, unrelated dirty state, applicable repository guidance, every
authoritative command and expected result, and known limitations before running
any command.

## Coordinator-managed entry

When the workflow coordinator invokes this skill, require:

- one exact coordinator target request containing the implementation base Git
  object, current HEAD Git object, full base-to-HEAD range and authoritative diff
  contents, changed-file inventory, and strict entry HEAD, index, worktree, and
  in-scope untracked path/content state;
- no in-scope index, worktree, or untracked source state outside that committed
  range;
- the approved scope, decision source, non-goals, and complete active Review
  policy with provenance;
- the approved implementation plan when present;
- every authoritative final verification command and expected result;
- prior stable gate keys and bounded retry history on re-entry.

At entry, resolve the request's base and head as Git objects and validate its
range, diff contents, changed-file inventory, current HEAD, index, worktree, and
in-scope untracked path/content evidence. From those exact content and
strict-state fields, create one content-bound immutable target identity exactly
once. Return that identity with the unchanged request fields, and use it for
every check and report in this invocation. Never accept a coordinator-supplied
identity, rename or regenerate the resolved identity, or substitute another
identity later in the phase.

Read repository guidance, approved scope and decision source, active review
policy, changed files, and the current diff. Read the approved implementation
plan when present. Resolve authoritative project commands before generic
defaults. Standalone snapshot or fileset evidence never satisfies this entry and
cannot authorize current-head completion. Return `BLOCKED` without running checks
when the exact request, strict current state, complete policy, commands, or
another entry input cannot be established.

## Standalone read-only entry

When the user invokes this skill outside the coordinator, resolve the explicitly
requested target as one of the three target forms above through local read-only
investigation. Require:

- the requested scope;
- applicable repository guidance;
- authoritative verification commands;
- available plan, decision, and review-policy evidence when present.

Do not require an active review policy, implementation authorization, or
coordinator-owned retry history for standalone verification. Return `BLOCKED`
with the exact missing input when the target identity, requested scope, or
authoritative commands cannot be resolved safely.

Create the standalone target identity once from every immutable field required
by its selected target schema. Label it `standalone-only`; it is separate from
and cannot be promoted to a coordinator-resolved identity.

Label an index/worktree snapshot or explicit fileset result `standalone-only`.
It may answer the requested verification question, but it cannot satisfy the
coordinator's immutable current-HEAD completion gate. Report this limitation
even when all commands pass.

## Select only a compatible verification executor

Before selecting any named verifier, inspect its effective sandbox and complete
instructions. A compatible verifier must prohibit index, tracked-file, and
in-scope source mutation and must not permit formatter output into those files.
Do not select or dispatch a named profile whose effective contract permits such
mutation.

The current `implementation-verifier` profile uses a workspace-write sandbox and
permits formatter output, so it is incompatible with this check-only phase. Do
not rely on or dispatch it. Use a compatible read-only route when one is
available; otherwise the lead runs the checks directly under this complete
check-only contract. An unavailable compatible route is not permission to weaken
the contract.

## Snapshot and run checks

Immediately before the first command, capture:

- HEAD, the target identity, and, for coordinator-managed verification, the
  unchanged coordinator target request;
- index entries plus the complete staged and unstaged status and diffs;
- immutable identities for tracked and in-scope source contents;
- path/content identities for in-scope untracked files and unrelated dirty
  state;
- absence or presence of pre-existing command artifacts that matter to the
  checks.

Run fresh, as applicable:

1. the approved final verification commands, using the plan when present;
2. focused tests for changed behavior;
3. owning package or workspace tests;
4. build or type check;
5. lint with warnings treated according to project policy;
6. format check;
7. relevant smoke or snapshot checks;
8. `git diff --check`, diff inspection, and final status.

Use only a formatter's documented check-only form, such as `--check` or an
equivalent no-write mode. If the required formatter offers only a mutating form,
return `BLOCKED` with that exact tool gap without running it. Do not replace
repository wrappers with broader commands that change semantics. Ask before
unusually expensive full-workspace checks when repository policy requires it.

After the final command, capture the same HEAD, index, status, diff, tracked
content, in-scope source, and untracked evidence again. Compare the snapshots and
attribute every change. Normal newly created ignored build artifacts are allowed
when recorded. An attributable tracked or in-scope source mutation is a
verification contract failure; uncertain ownership is `BLOCKED`. Do not repair,
restore, stage, commit, or clean either state from this skill.

A commit or target-content change after a command makes that command's evidence
stale. Do not return `PASS` unless every required result applies to the unchanged
target and no verification-caused tracked or in-scope source mutation occurred.

## Evaluate

Return exactly one verdict:

- `PASS` — every required command and inspection succeeded with fresh evidence
  for the unchanged current head;
- `FAIL` — a required command or contract check produced an observed failure;
- `BLOCKED` — a required command, dependency, permission, input, or current-head
  guarantee was unavailable, so the result cannot be established.

For every `FAIL` or `BLOCKED`, record:

- a stable gate key based on the failed command or contract and concrete behavior,
  not a transient line number;
- the exact command, output, target identity, and affected range or snapshot;
- likely ownership: requested or approved implementation scope, unrelated
  existing state, or scope, design, or authority outside the approval;
- every unverified gap and the exact condition required for safe re-entry.

Do not mark a failure acceptable without evidence that it is unrelated and
outside scope.

## Report

Return:

- verdict: PASS, FAIL, or BLOCKED;
- target form and identity; for coordinator-managed verification, the exact
  unchanged coordinator target request and confirmation that the identity was
  resolved once at entry; starting and ending HEAD; exact range, snapshot, or
  fileset; and files inspected;
- starting and ending index/worktree, tracked/source, in-scope untracked
  path/content, and unrelated dirty-state evidence;
- every command, expected result, observed result, and match status;
- approved criteria and review policy inspected when available;
- stable gate keys, failures, and likely ownership;
- checks not run, standalone-only limitations, every unverified gap, and exact
  re-entry conditions.

For a coordinator-managed entry, return all evidence to the coordinator. On
`PASS`, do not start review or advance phases. On `FAIL` or `BLOCKED`, do not
diagnose or fix; return the stable key, ownership, target, gap, and re-entry
condition for coordinator classification.

For a standalone read-only entry, report the verdict and evidence directly to the
requester. Do not fix failures, start review, or advance another workflow phase.
