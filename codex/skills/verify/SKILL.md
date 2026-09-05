---
name: verify
description: Execute a predeclared Verification Matrix mechanically against an exact Task, integration, or standalone target and return raw PASS, FAIL, or BLOCKED evidence.
---

# Mechanical verification

Separate owner-side preparation from runner execution. The Task Lead owns
planned Task verification; the Feature Lead owns lightweight, integration-only
and standalone verification. The dispatched runner executes observations only.
No source/index writes, semantic diagnosis, remedies, review or phase advance
are authorized here. Normal ignored command artifacts require a declared
boundary.

## Owner: bind an exact target and complete matrix

Resolve directly through Git one target:

- Task PR: authority identity/currentness, workspace, branch, exact planned
  base, merge base, committed head/range, changed files and status;
- integration: exact composed tree, ordered accepted input heads/trees and
  named integration-only obligations, or the accepted lightweight head/tree;
- standalone: committed range, current staged/unstaged/untracked snapshot, or
  explicit bounded fileset with content fingerprints and known limitations.

Keep full applicable authority directly readable. Task verification needs no
prior Task Accepted result or review. Require no unexplained in-scope state
outside a committed Task range. Integration verifies only its named remaining
obligations, not a replay of accepted Task checks. Lightweight authority is the
recoverable combined contract; do not add a plan, DAG, or contract file.
Standalone results are labeled `standalone-only` and cannot satisfy Task gates.
Eligible legacy inputs require their exact unchanged approved execution
authority; no silent migration or new contract artifacts.

The loop owner, not the runner, maps each required observable obligation to:

- stable row ID and exact controlling authority;
- exact non-mutating command or fully specified mechanical target-state check;
- working directory, command environment and artifact boundary;
- expected observable result and mechanical comparison;
- `FAIL` or `BLOCKED` for a specified mismatch.

Define the precondition and final mutation checks, including their commands,
expected values and mechanically comparable fingerprints. Include source/index
and relevant untracked state, not just HEAD. For clean Task ranges, use bounded
target/clean-state and source observations; for dirty standalone snapshots,
include staged, unstaged, untracked and bounded-file content. This is an
in-memory current-target matrix, not a new persistent schema.

Order rows before dispatch. Include applicable format-check, focused tests,
build/type-check, lint, package/workspace tests, integration/smoke and Git checks
when required; there is no restriction to test/lint/format and no mandatory
suite regardless of the artifact. Exact contract commands must be preserved.
A formatter must have a documented non-mutating mode. Command choice,
sufficiency, ordering, environmental safety and diagnosis remain owner decisions.
A missing or ambiguous row prevents dispatch; never ask the runner to invent it.

Any change to head, range, source snapshot, controlling authority or material
verification route invalidates the matrix. Fresh verification is required on
every corrected head. Writer-side focused checks and TDD history are not this
independent gate.

## Owner: dispatch once

Select `verification-runner` at the preapproved Luna/low allocation through
`agent-teams-driven-development` in the owning session. Resolve its effective
check-only contract before dispatch. Send only the exact target, matrix,
environment, source-state comparisons and result contract; omit unrelated
Review policy, topology, previous transcripts and discovery-cache duties.

No runner may dispatch another runner. A rejected spawn remains pending until
safe retry; unavailable allocation is BLOCKED without fallback. A required
Task gate cannot use a lead pass instead. Only an explicitly authorized no-agent
standalone request permits the lead to execute the same mechanical rows and
label the result `standalone-only`.

## Runner: execute supplied rows only

You are already the leaf. Do not perform owner-side preparation or dispatch.
Return BLOCKED for missing, stale, contradictory, unsafe or unresolvable inputs.
Do not add, delete, reorder, choose, repair or reinterpret rows.

Run predeclared target checks immediately before ordinary commands. Execute
ordinary rows in input order, fail-fast on the first FAIL or BLOCKED. For each
row retain ID, exact command/check and directory, exit code, bounded attributable
stdout/stderr, expected result, observed result and mechanical classification.
Preserve access to full output when the bounded excerpt is insufficient; do not
turn truncation into success. Record every unrun row and its blocking reason.

After success or an early stop, always attempt the supplied final mutation
check. Compare against the starting source/index state and record allowed
artifacts. Never restore, repair, stage, commit, clean or run a write-mode
formatter. A conclusive mismatch or source/index mutation is FAIL. An unavailable
input/tool/permission/environment or unresolvable final guarantee is BLOCKED;
preserve a previously observed FAIL even when the final check is also blocked.

Do not judge behavior, coverage, adequacy, architecture, scope or remedies.
Do not perform new discovery, edit a search cache, or interpret an ambiguous
expectation. Return it to the owner.

## Runner: report evidence, not acceptance

Return exact target and start state; one row table in input order with
`ID | command/check | expected | observed | result` and associated raw command
observations; final state and mutation result; allowed artifacts; unrun rows
and reasons; gaps; and exactly one result:

- `PASS`: all required rows matched on the unchanged target and final source
  invariants hold.
- `FAIL`: at least one conclusive declared mismatch or prohibited mutation.
- `BLOCKED`: no conclusive failure, but a required observation or guarantee
  could not be established.

Return to the owning loop (or standalone requester). PASS proves only that the
mechanical observations matched, not semantic correctness or Task Acceptance.
