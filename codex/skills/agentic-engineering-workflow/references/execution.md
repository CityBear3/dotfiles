# Authorized execution and acceptance

## Shared execution constraints

- Keep reusable language conventions in applicable guidance and Skills rather
  than copying them into Feature Contracts, Task Contracts, or Implementation
  Plans. Include exact language-specific detail there only when it defines a
  public or shared interface, compatibility, writer ownership, a reproducible
  environment, or another observable correctness condition.
- The Feature Lead owns Feature authority, Task readiness, dependency release,
  cross-Task effects, integration, and Feature Acceptance. Each new planned
  Task uses one independent Codex session in its Herdr worktree. Its Task Lead
  is the sole writer and local loop owner, and dispatches only bounded native
  check-only/read-only leaves. Lightweight work is written directly by the
  Feature Lead. Leaves never spawn descendants. Runtime-rejected checks stay
  pending and retry after progress without weakening selected gates.
- Fix Task-session and leaf model/effort allocations before execution; the
  engineer confirms planned allocations at Implementation Plan approval.
  Feature Lead uses its session defaults. No runtime promotion or fallback.
- Keep verification mechanical and independent review mandatory for Task
  Acceptance. Integrate findings only for overlap, conflict, authority defects,
  scope-sensitive remedies, or non-trivial attribution; simple clear findings
  may proceed directly to evidence-based triage. Reuse verification and review
  when their content, dependencies, requirements and execution conditions remain
  applicable; HEAD or history changes alone do not trigger reruns. Preserve
  original evidence and explicit applicability reasons. For actual corrections,
  rerun affected checks and finding-owning/affected reviewers.
- Treat Task `Accepted` as the completed Task-local quality decision. The
  Feature Lead checks result identity, applicability to current inputs, Task
  dependencies and assigned Feature/integration coverage; it does not routinely
  re-audit Task tests, raw verification reports or reviewer judgments. Reopen
  only the affected work for relevant changes or concrete contradictory evidence.
- Keep problem validity separate from remedy validity, return confirmed Design
  authority defects to the engineer early, and retain independent out-of-scope
  problems as non-blocking concerns rather than expanding the Task. Previously
  approved or in-flight work retains its exact topology and model authority;
  never silently migrate it onto changed workflow assets.

## Recover within the authorized loop

A phase gate holds its dependent transition until its obligations are met; a
leaf or phase returning `BLOCKED` does not by itself end the owning loop or
require engineer approval. During active approved work, the owning Task or
Feature Lead classifies the actual obstacle before returning control:

- **Historical procedure gap:** retain and disclose what happened. Apply
  `test-driven-development`'s history/current-evidence boundary and
  `receiving-code-review`'s history-only classification. An ordinary plan step,
  named RED/GREEN sequence, or mandatory procedure does not by itself make its
  past execution an independent Acceptance condition. Require concrete current
  impact or exact authority explicitly making the original history itself an
  Acceptance obligation before treating it as blocking. Do not reconstruct
  historical evidence, repeat completed work to simulate compliance, or request
  an exception solely to acknowledge the discrepancy.
- **Recoverable input or evidence gap:** inspect directly available authority,
  Git state and original reports, complete the missing handoff or matrix, and
  resume the pending phase. The responsible owner obtains missing observations
  or reruns only uncovered/invalidated checks and affected review under the
  existing policy. If the information belongs to another Task/Feature owner,
  return the precise gap to that owner for recovery; this is not automatically
  an engineer escalation. Keep valid evidence, failed observations and pending
  roles attributable. Do not change expectations, skip gates or infer approval
  to make a result pass.
- **No safe authorized continuation:** hold the affected operation when recovery
  needs a user-owned decision, new authority, or an unavailable prerequisite
  that the owners cannot resolve. Report the exact obstacle, relevant recovery
  attempts and results, remaining decision or external action, and re-entry
  condition. Continue independent ready work whose authority and inputs remain
  valid. A material design defect follows the [early Design Escalation rule](execution.md#return-authority-defects-early).

The phase/Task owner performs recovery within its existing responsibility;
check-only leaves still return evidence without repairing inputs or source.
Use the same approved allocations and preserve sole-writer and unchanged-target
guarantees. Temporary runtime rejection stays pending and retries after relevant
progress or availability evidence. Do not repeat an unchanged failed action,
busy-poll, or silently restart uncertain state. Exhausted safe recovery is a
reported blocker, not permission to weaken a gate.

Select and carry out the next authorized recovery action instead of ending the
turn merely to announce a procedural problem. These rules do not authorize
reopening completed work, publication, destructive changes, policy changes or
scope expansion; their existing boundaries still apply.

### Consume Accepted as a completed Task decision

The Task-loop owner issues `Accepted` only after its required independent
verification, review and triage are complete for the exact Task target and
authority. The Feature Lead consumes that Task-local quality decision rather
than issuing a second verdict on test adequacy, reviewer judgment, discipline
or gate completeness.

At receipt, dependency release, aggregation and publication, limit Feature-side
checks to the result's assigned Task/session and authority, correspondence to
the actual target, changes to relied-on inputs, and the approved Task dependency
and integration obligations. Use the existing handoff identities, Accepted
result and retained evidence references. Do not introduce another certificate,
result schema or approval gate. Feature Contract coverage means that each
approved clause is assigned to a current Accepted Task or a named remaining
integration obligation; it is not a second assessment of Task test coverage.

Retain Task-local raw reports and impact assessments with their owner and keep
them directly accessible by reference. Do not require the Feature Lead to read
or reconstruct every matrix, reviewer report or carry-forward argument on each
transition. Inspect the relevant source evidence only for a concrete mismatch,
contradiction or requested diagnosis, such as an Accepted result also reporting
a required reviewer still pending. An attributable Accepted result for unchanged
inputs needs no new verification, review or assurance merely because the
Feature Lead did not observe the Task's checks itself. Runtime `done`, a writer
self-check or an unassigned status string is not that result.

Recover an omitted result identity or report reference with the owning Task;
do not treat a missing pointer as proof that its check never ran. Relevant
content/authority/dependency changes or concrete contradictory evidence hold
only the affected use of Accepted. During active authorized work, the Task-loop
owner resolves the discrepancy and determines any affected checks/review;
the Feature Lead handles cross-Task invalidation and independent ready work.
Do not reopen acceptance for hypothetical mistrust or a history-only procedure
gap. The completion/publication discussion boundary still governs an actual
new problem requiring completed work to reopen.

## Prepare current task and feature evidence

For planned work retain:

- the original implementation base and coordination workspace;
- both approved topologies and every task workspace, branch, planned base,
  merge base, head, exact range, status, changed files, commits, dependency
  evidence, verification, review, triage, and publication state;
- accepted, candidate, stale, blocked, and in-flight results without conflating
  them;
- approved Design Doc when applicable, Feature Contract, complete Task Contract
  set, coverage, shared interfaces, integration-only obligations, Review
  context, and policy;
- an optional search-cache path and reusable cross-session findings when
  present, with the Feature Lead as sole writer;
- exact temporary integration compositions and their evidence;
- concerns, unresolved findings, and every gap.

For lightweight work retain its one exact Task PR and complete recoverable
combined contract. For eligible legacy work retain the original single-range
evidence required by its unchanged plan.

Task-local detail may remain in the owning Task's directly referenced reports;
retaining Feature evidence does not require copying or re-auditing those reports.
Require no unexplained in-scope state in any task checkout. Re-read affected
branches, bases, heads, ranges, worktrees, and status before every transition.
Standalone reports alone never constitute coordinator Acceptance.

## Advance only on current evidence

Advance automatically within approved local scope:

1. Accept from lightweight `execute-lightweight-task` only a current `Accepted`
   result for its exact Task PR range. When its combined contract has no integration-only
   obligation, that result is also Feature Accepted; do not repeat verification
   or review.
   When a named integration-only obligation remains, use that accepted head and
   tree as the exact single-task integration target and continue at steps 4–6
   with the recoverable combined contract and lightweight policy. Do not invoke
   `execute-plan` or require planned artifacts, a Task DAG, or a multi-PR
   topology for that target.
2. Accept from `execute-plan` only `TasksAccepted` with every Task Contract
   represented by a current authoritative result, both topologies resolved,
   complete coverage, and exact integration composition inputs.
3. Check Accepted applicability and map the approved Feature clauses to their
   assigned Accepted Tasks and named integration obligations, without re-auditing
   Task-local verification or review coverage. If no
   integration-only obligation remains, mark the feature accepted without a
   synthetic aggregate range or repeated review.
4. For each integration-only obligation, the Feature Lead builds the exact
   current-composition matrix, including commands, mechanical expectations and
   mutation checks. Pass only that matrix, target, environment and evidence to
   `verify`; dispatch its native runner in the Feature session. Accept only
   current `PASS` covering the exact composition under the applicability rule.
   Diagnose a `FAIL` before correction. Route an authorized planned correction
   through its owning Task Contract in `execute-plan`; route an authorized
   lightweight correction directly to its combined-contract Task in
   `execute-lightweight-task`. After the corrected Task has complete current
   gate coverage, refresh affected named integration evidence and explicitly
   carry unaffected evidence.
5. Invoke `review` only when the approved policy requires or conditionally
   triggers a targeted integration perspective. Pass the same exact integration
   authority and evidence to every selected reviewer. Do not invoke ordinary
   full-feature `$review`.
6. Require `review` to supply conditional finding-integrator evidence or
   explicit simple-finding direct-triage eligibility for integration FINDINGS.
   Send the unchanged target and complete evidence to `receiving-code-review`.
   Raw reviewer output never authorizes correction.
   For planned work, route an authorized `Fix` to its owning Task Contract
   through `execute-plan`, mark only semantically affected results and their
   transitive dependents stale, and rerun fresh affected task and integration
   evidence. For lightweight work, route the `Fix` directly to its
   combined-contract Task through `execute-lightweight-task`, then establish
   complete current task coverage and refresh affected named integration
   evidence. Preserve
   `Push back` while its target and controlling evidence remain unchanged.
   Retain an independent out-of-scope valid problem as a non-blocking concern
   without expanding the current Task or creating a backlog. Return a
   user-owned decision as `Escalate`.
7. Mark Feature Accepted only when every Task PR result and integration
   obligation is current and no finding, policy gap, design gap, or operational
   gap survives.

Diagnose failed verification before correction and recover blocked phases under
the [continuation rules](execution.md#recover-within-the-authorized-loop). Never advance failed or blocked
verification to review, blocked review to triage, unresolved triage to
correction, or incomplete evidence to completion. Stop repeated non-progress
with its observed attempts. Never discard uncertain state to force progress.

### Return authority defects early

When integrated evidence shows that the applicable Design Doc is missing,
contradictory, or materially ambiguous, accept only `Escalate` with reason
`Design Escalation`. Stop every unstarted reviewer and correction queue for the
affected target, preserve already-running read-only reports when they complete,
and return the exact defect and authority evidence to the engineer. Do not
silently repair or reinterpret the Design Doc.

After the engineer approves a Design Doc change, rederive and reapprove only
the Feature Contract, Task Contracts, Review policy, or plan content whose
meaning changed. Mark only Tasks assigned those changed meanings and their
transitive dependents stale. Retain an unchanged Accepted Task after directly
revalidating its exact authority, dependencies, relied-on interfaces, base,
head, range, and status; a Design amendment does not make every Task stale by
default.
