---
name: verify
description: Run a prepared Verification Matrix for Task acceptance, integration, or a requested standalone check.
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
  base, merge base, head/range or exact pre-commit snapshot, changed files and
  index/worktree/relevant untracked status;
- integration: exact composed tree, ordered accepted input heads/trees and
  named integration-only obligations, or the accepted lightweight head/tree;
- standalone: committed range, current staged/unstaged/untracked snapshot, or
  explicit bounded fileset with content fingerprints and known limitations.

Keep full applicable authority directly readable. Task verification needs no
prior Task Accepted result or review. Require no unexplained in-scope state
outside the bound range or snapshot. Integration verifies only its named remaining
obligations, not a replay of accepted Task checks. Lightweight authority is the
recoverable combined contract; do not add a plan, DAG, or contract file.
Standalone results retain the label `standalone-only`; later Task use requires
the coordinator's explicit applicability, obligation and independence mapping.
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
target/clean-state and source observations; for pre-commit or standalone snapshots,
include staged, unstaged, untracked and bounded-file content. This is an
in-memory current-target matrix, not a new persistent schema.

Order rows before dispatch. Include applicable format-check, focused tests,
build/type-check, lint, package/workspace tests, integration/smoke and Git checks
when required; there is no restriction to test/lint/format and no mandatory
suite regardless of the artifact. Exact contract commands must be preserved.
A formatter must have a documented non-mutating mode. Command choice,
sufficiency, ordering, environmental safety and diagnosis remain owner decisions.
A missing or ambiguous row prevents dispatch; never ask the runner to invent it.

Apply the coordinator's evidence-applicability rule before dispatch. Bind the
matrix to the current target and identify each required row as supported by
original PASS evidence and an explicit applicability comparison, or requiring
fresh execution. Content, dependencies, requirements or execution-condition
changes invalidate affected rows; missing or uncertain coverage requires fresh
checks during authorized work. HEAD/range identity changes alone do not.
Retain original commands, observations, pre/final state checks and provenance;
do not rewrite old observations to match a new target. Prepare current pre/final
checks for any fresh runner invocation. With complete applicable PASS coverage,
return the evidence mapping without dispatch. Writer checks and TDD history
cannot supply this independent gate.

Under the coordinator's reviewed-base/advancing-tip rule, a confirmed compatible
base-tip advance leaves Task verification current on its original target; do
not rebuild or execute its matrix for that alone. A latest-base integration
matrix requires fresh affected rows when their actual composition inputs change.

Required freshness does not authorize reopening a completed loop. Missing or
stale evidence found at completion/publication first returns to the Feature Lead
for the coordinator's engineer discussion boundary. Dispatch new checks only
within active approved work or the engineer-authorized response.

## Owner: dispatch once

Select `verification-runner` at the preapproved Luna/low allocation through
`dispatch-check-agents` in the owning session. Resolve its effective
check-only contract before dispatch. Send only the exact target, matrix,
environment, source-state comparisons and result contract. Send the ordered
fresh rows plus their pre/final checks; keep carried rows and their provenance
in the owner's complete coverage map. Omit unrelated
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
