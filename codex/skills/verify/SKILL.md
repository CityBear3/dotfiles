---
name: verify
description: Verify a current implementation head with fresh project commands and return PASS, FAIL, or BLOCKED evidence. Use from the workflow coordinator after implementation or standalone for a read-only completion check.
---

# Verify the current implementation head

No completion claim without fresh observed evidence.

Remain read-only. Do not edit files, create commits, or repair failures from this
skill.

## Coordinator-managed entry

When the workflow coordinator invokes this skill, require:

- the current head commit and exact base-to-head range;
- the approved scope, decision source, and active review policy;
- the approved implementation plan when present;
- authoritative final verification commands;
- prior stable gate keys and bounded retry history on re-entry.

Read repository guidance, approved scope and decision source, active review
policy, changed files, and the current diff. Read the approved implementation
plan when present. Record the current head immediately before verification.
Resolve authoritative project commands before running generic defaults. Return
`BLOCKED` with the missing evidence as an unverified gap when an entry input
cannot be established.

## Standalone read-only entry

When the user invokes this skill outside the coordinator, resolve through local
read-only investigation:

- the requested scope;
- the current head and exact base-to-head range;
- applicable repository guidance;
- authoritative verification commands;
- available plan, decision, and review-policy evidence when present.

Do not require an active review policy, implementation authorization, or
coordinator-owned retry history for standalone verification. Return `BLOCKED`
with the exact missing input when the requested scope, range, or authoritative
commands cannot be resolved safely.

## Shared preparation

Record unrelated dirty state separately. Treat any uncommitted change that can
affect the required commands or inspected files as an unverified current-head
gap.

If agents are allowed and the `implementation-verifier` profile is selectable,
use it as a read-only verifier. If it is not selectable, give a generic read-only
subagent the complete verification contract. When the user prohibits agents,
perform the same checks directly.

## Checks

Run fresh, as applicable:

1. the approved final verification commands, using the plan when present;
2. focused tests for changed behavior;
3. owning package or workspace tests;
4. build or type check;
5. lint with warnings treated according to project policy;
6. format check;
7. relevant smoke or snapshot checks;
8. `git diff --check`, diff inspection, and final status.

Do not replace repository wrappers with broader commands that change semantics. Ask before unusually expensive full-workspace checks when repository policy requires it.

Read the current head again after the checks. A commit added after a command makes
that command's evidence stale for the new head. Do not return `PASS` unless every
required result applies to the unchanged current head and exact range.

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
- the exact command, output, and affected range;
- likely ownership: requested or approved implementation scope, unrelated
  existing state, or scope, design, or authority outside the approval;
- every unverified gap.

Do not mark a failure acceptable without evidence that it is unrelated and
outside scope.

## Report

Return:

- verdict: PASS, FAIL, or BLOCKED;
- starting and ending current head, exact range, and files inspected;
- every command and observed result;
- approved criteria and review policy inspected when available;
- stable gate keys, failures, and likely ownership;
- checks not run and every unverified gap.

For a coordinator-managed entry, return all evidence to the coordinator. On
`PASS`, do not start review. On `FAIL`, let the coordinator use the bounded retry
contract to diagnose, fix, and reverify only when ownership is within approved
scope; return scope, design, or new-authority needs for `Escalate`.

For a standalone read-only entry, report the verdict and evidence directly to the
requester. Do not fix failures, start review, or advance another workflow phase.
