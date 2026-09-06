---
name: execute-task
description: Run one planned Task's implementation, independent checks, triage and bounded correction in its independent Task Lead session.
---

# Execute one planned Task

The independent Task Lead is both sole writer and Task-local loop owner.
Follow [the Task Lead contract](references/task-lead.md). Do not start another
implementer, restart Feature planning, schedule other Tasks, release
dependencies, issue Feature Acceptance, publish, merge or choose disposition.

## Require complete current authority

Require a compact, complete handoff from the Feature Lead:

- exact approved Design sources, Feature Contract, Implementation Plan and Task
  Contract identities/paths, approval/currentness, assigned clauses, purpose,
  expected behavior, constraints and non-goals;
- ownership, shared interfaces, adjacent-Task obligations and delegated private
  decisions;
- separate Review context and complete policy: selected/conditional roles and
  triggers, skips and reasons, pending order, finding-integration conditions,
  correction invalidation and Acceptance threshold;
- approved Task and leaf model/effort allocations, effective startup settings,
  and the shared Task Lead role source;
- material property, reliable oracle, development discipline and TDD
  applicability/reason, verification obligations and exact commands only when
  their identity is contractually significant;
- coordination directory, exact Herdr workspace/pane/agent routing identity,
  Task worktree/branch/PR, starting ref, planned base ref/commit, current head,
  merge base, range/diff, starting index/worktree/relevant untracked state;
- candidate or authoritative mode, dependency evidence and commit intent with
  fixed message or writer authority to select it;
- attributable existing commits, reports, correction attempts, carried-forward
  evidence, pending roles, interruptions, concerns and re-entry conditions.

When useful, the handoff may include reusable cross-session findings or a
relevant optional search-cache reference with its source and applicability
conditions. Cache absence or staleness never makes the handoff incomplete;
perform needed discovery without a cache-miss report. Only the Feature Lead
writes that cache.

Keep exact sources directly readable; do not copy unrelated Feature history or
unchanged unassigned prose. Reject lightweight authority: that loop belongs to
`execute-lightweight-task`. Reject a missing, contradictory, stale or
unrecoverable field rather than inventing a wrapper or decision.

This topology applies to plans approved for it. A previously approved or
in-flight plan retains its original topology, models and gates and must use
its exact prior coherent assets. If those are unavailable, preserve state and
return BLOCKED for recovery or owner-selected migration; do not reinterpret
legacy authority or manufacture new contracts.

For promotion reconciliation, also require original lightweight base, promotion
head, execution-starting head, exact unaccepted commits/range, attributable
approved artifact state, complete change-to-Task mapping and prior evidence.
This authorizes acceptance of attributable preserved work, not new semantics
or history rewriting.

## Validate identity and one writer

Before edits confirm effective model and both normal/Plan-mode effort against
the approved allocation. Never promote, fall back, or change effort mid-loop.
Confirm the exact workspace, branch, base, starting head and sole-writer
ownership directly. A fresh implementation starts at the supplied HEAD.
For authoritative review, the planned base must be an ancestor of the current
head and the merge-base-derived range must match the assigned PR.

Do not edit while old-head verification/review is running. Runtime admission
controls native leaves inside this session; selected rejected leaves remain
pending in order and retry after progress without losing a gate. Herdr connects
this session to the Feature Lead; native leaves do not get their own Herdr
sessions and never spawn descendants.

A missing model, input, safe writer state or target is BLOCKED. A new or changed
scope, responsibility, public/shared interface, contract meaning, invariant,
failure/compatibility guarantee, verification obligation or Review policy is
Escalate to the Feature Lead. A private file/helper inside delegated ownership
is not a deviation by itself. Astra provides no extra design authority.

## Implement and commit locally

Keep this Task Lead through ordinary implementation and correction turns.
Batch independent initial reads/searches/Git observations when every result is
attributable; stop before a result-dependent decision. Use supplied applicable
search-cache entries only as navigation. Return costly findings worth reusing
across sessions to the Feature Lead without editing the cache; routine searches
need no cache lookup or report.

Apply `test-driven-development` when applicable, preserving causal RED,
production edit and GREEN. For not-applicable work use the supplied baseline
and oracle; required-but-blocked discipline is BLOCKED. Preserve unrelated
changes and keep exploratory work outside Candidate evidence.

Choose private files, helpers, local types, algorithms and focused checks only
within delegated scope. Run required pre-commit commands and focused writer
checks; do not duplicate the full authoritative suite unless explicitly
required before commit. Inspect actual changes and authority coverage, then
use `commit` only for the authorized responsibility-scoped commit. Record
commands with expected/observed results, discipline evidence, diff inspection,
commit, new head, status, range, changed files and concerns.

A writer commit is candidate evidence, not Task Acceptance. Classify concerns
as bounded in-scope correction, operational BLOCKED or user-owned Escalate.

In plan-authorized candidate mode, return Candidate after attributable commit
and preliminary evidence; no authoritative gate or dependency release occurs
until the final PR base exists. On authoritative re-entry, validate candidate
and separately authorized restack/final-base evidence. Skip duplicate
implementation and commits when preserved work is already attributable.
A promotion reconciliation likewise inspects the original-base envelope and
uses existing commits; create only an explicitly declared artifact/correction
commit when necessary.

## Run independent current-head gates

After the committed head and exact range resolve, construct the complete
in-memory Verification Matrix defined by `verify`. Map every observable
obligation to stable row ID, authority, exact non-mutating command/check,
directory/environment, expected observation and mechanical mismatch status.
Include source-state pre/final checks and allowed ignored artifacts.
Head/range/authority/material-route changes invalidate the matrix.

Invoke `verify` with only its exact target, matrix, environment and mutation
boundary. Dispatch the preapproved native verification-runner once through
`agent-teams-driven-development`. Never ask it to choose checks, diagnose,
judge adequacy, or make Acceptance decisions. A FAIL is interpreted by this
Task Lead against authority; diagnose before correction. BLOCKED is not clean.

Only after fresh PASS on the unchanged target invoke `review`. Normally
dispatch independent spec-reviewer and implementation-quality-reviewer
concurrently, plus only plan-selected required or triggered perspectives.
Send each its own authority/target evidence, raw matrix observations and
relevant prior reports; do not proxy this through the Feature Lead.
Gate independence cannot be replaced with writer self-checks or standalone
results. No-agent conflicts require owner policy authority, not a silent lead
substitute.

`review` returns CLEAN, FINDINGS or BLOCKED and owns perspective coverage and
conditional finding integration. For FINDINGS require complete source reports
and either the required finding-integrator result or an explicit simple-finding
direct-triage eligibility record. Use `receiving-code-review` to verify and
classify Fix, Push back or Escalate; raw findings alone do not authorize edits.

Keep problem and remedy validity separate. A confirmed authority defect returns
immediately as Design Escalation to the Feature Lead; do not start queued review
or correction for that target. Preserve completed and already-running read-only
reports. Independent out-of-scope valid problems remain non-blocking concerns,
not an implicit backlog or scope expansion.

## Correct without reopening the whole loop

Retain prior reviewed head H1, full reports/triage, correction attempts and the
complete policy-selected coverage set. This same Task Lead applies only the
authorized bounded correction and creates a separate correction commit H2.
No new implementer session or repeated writer handoff is needed.

Revalidate base/head/range/status, rebuild the matrix and run fresh verification
on H2. Prepare the `review` impact map: correction files/behaviors, finding
owners, affected perspectives and concrete evidence for each proposed
carry-forward. Always rerun finding-owning and semantically affected reviewers;
uncertainty means rerun. The selected policy coverage is unchanged even when
only a subset needs a fresh invocation. Missing prior evidence cannot carry.

Give rerun reviewers H1/H2, correction delta, full current range, prior
report/triage, exact authority and fresh matrix. `review` owns targeted/full
traversal and carry-forward validation; do not duplicate its rules here.
Reintegrate only when its triggers hold, then triage the current target.
A changed base, authority or shared surface invalidates all affected evidence
and may require Feature-level re-entry. Never carry old verification forward.

Stop repeated correction without progress with attempts and the exact gap.
Do not invent another tracking schema, broaden the Task or raise the model.

## Recover and return exact evidence

A Herdr timeout or lost response does not stop this writer. Before a resumed or
replacement writer can edit, the Feature Lead resolves prior session/leaf
activity, confirms no competing writer or old-head checks, attributes all
in-scope state and revalidates the handoff. A safe replacement receives one
complete no-history handoff at the same approved allocation. Never clean,
reset, rebase, amend, discard or silently restart uncertain state.

Resume only pending gates when committed work and their existing evidence are
still current. Re-entry relies on Git, authority and reports, not memory.

Return Candidate, Accepted, BLOCKED or Escalate with exact Task/authority and
workspace/Herdr routing identity, allocation, branch/base/merge base/head/range,
starting/final source state, commits, changes, discipline and command evidence,
completed matrix/raw observations, every selected perspective's report or
explicit non-invalidation evidence, integration/triage, pending roles, attempts,
concerns, any useful cross-session discovery candidates, and gaps. Keep
evidence directly accessible; send compact result plus references, not repeated
local transcripts.

Accepted requires the exact authoritative range, all obligations proved by
fresh verification and independent review coverage, and every finding closed
by verified correction or current justified Push back. Task Lead reports this
evidence-backed result despite being its writer; it is not self-approval.
Only the Feature Lead validates cross-Task currentness, releases dependents and
issues Feature Acceptance. A terminal Task result ends this turn without
another polling loop; new work requires a current handoff.
