---
name: execute-task
description: Execute and accept one path-neutral implementation task with one writer, exact evidence, a policy-selected gate, and bounded retries.
---

# Execute one task

Own the complete lifecycle and acceptance of exactly one implementation task.
Use the same contract for a lightweight task, one task from an approved plan, or
an authorized bounded correction. Do not select a workflow path, schedule plan
dependencies, run global verification or final review, publish, merge, or choose
branch disposition from this skill.

## Require one immutable canonical task context

Before implementation, require exactly one immutable canonical task context
containing:

- the complete task specification;
- the original request, Design Doc, or other approved decision source, including
  explicit non-goals;
- the required implementation discipline;
- the approved workspace and working directory;
- the exact task base commit;
- every exact task verification command and expected result;
- the complete active Review policy and its provenance;
- configured and observed capacity plus queue rules;
- when present, approved-plan task context limited to the plan path,
  task-specific decisions, non-goals, and file responsibilities.

Assign the complete serialized context one stable identity, such as a content
digest plus schema version. The identity changes whenever any context field
changes. Pass either this one complete payload or one resolvable immutable
reference to it; never inline another copy of the complete task or policy in an
evidence bundle, scheduler envelope, reviewer message, or optional plan context.

An `execute-task` invocation may additionally carry one validated partial task
record and lifecycle phase for the same context identity. Retry history and
mutable execution evidence belong to that partial record, not the immutable
canonical context.

Reject missing, stale, contradictory, or mode-inconsistent input. Return a named
gap through the invoking skill to `agentic-engineering-workflow`; do not infer a
decision, expand scope, or weaken evidence.

The complete Review policy must record:

- mode: `focused`, `adaptive`, or `deep`, with rationale and risk surfaces;
- the mode-consistent per-task gate;
- final required reviewers with reasons and conditional reviewers with triggers
  and reasons;
- skipped perspectives with reasons;
- residual risk;
- configured and observed capacity plus queue rules;
- the Acceptance threshold;
- provenance for each material field, including the Design default, original
  request authorization, and observed risk or capacity evidence.

## Enter or resume the task lifecycle

Record exactly one current lifecycle phase:

- `pre-implementation`: no task-owned edit or commit exists;
- `implementation/working-tree pending`: attributable task edits exist and no
  accepted task commit/evidence bundle is ready;
- `committed evidence ready/gate pending`: task commits, fresh verification,
  pre-commit inspection, exact range, and the current evidence bundle are
  validated, but one or more selected gate roles remain pending;
- `correction pending`: an authorized stable-key correction is ready or has
  attributable partial writer state.

A partial task record contains:

- canonical context identity and lifecycle phase;
- exact task base, current HEAD, task/fix commits, and exact ranges;
- writer identity, writer status, confirmed inactivity, and ownership attribution
  for HEAD, status, working-tree diff, and commits;
- completed verification commands, expected and observed results, match status,
  pre-commit evidence, and current evidence-bundle identity;
- exact pending gate and roles, stable-key attempt history, gaps, and the
  condition required for safe re-entry.

On re-entry, validate that the context identity and payload, active policy,
repository HEAD and status, commit ancestry, recorded range and diff, evidence
bundle, and writer ownership remain unchanged. For
`committed evidence ready/gate pending`, reuse the same validated committed
evidence, skip implementation and commit, and resume only the pending gate. Never
create a duplicate commit or discard partial work.

For `implementation/working-tree pending` or `correction pending`, resume the
existing attributable state only after confirming the prior writer is inactive
and no writer overlaps. If a safely resolvable operational or evidence mismatch
exists, return `BLOCKED` with a new exact re-entry condition. If the mismatch
requires a material decision, scope, policy, or authority change, return
`Escalate`. Never silently restart from `pre-implementation`.

## Choose one writer and hand off implementation

Keep exactly one writer: either the lead or one `implementer`. Use the lead when
direct execution is authorized. When dispatching an implementer, load
[implementer-prompt.md](../agent-teams-driven-development/implementer-prompt.md)
only at that point, pass its complete role contract with the canonical task
context exactly once, and send that already-selected contract to
`agent-teams-driven-development` for scheduling.

Do not repeat canonical fields in the scheduling envelope. Require the writer to
preserve unrelated changes and report status, files, commands, expected results,
observed results, whether each matched, pre-commit inspection, concerns, and
every commit it creates.

For production behavior, use a red-green-refactor loop and record the observed
red failure. For content, configuration, refactoring, or mechanical migrations,
apply the declared discipline and preserve the relevant green baseline.

## Consume writer status

Require the lead or implementer to return exactly one status and map it as
follows:

- `DONE` advances only after the requested commit exists and current-state,
  ownership, verification, expected-result matching, pre-commit, report, and
  evidence checks all pass. `DONE` alone is not task acceptance.
- `DONE_WITH_CONCERNS` never advances directly. Classify every concern: an
  authorized in-scope correction enters the stable-key loop; an operational or
  evidence gap returns `BLOCKED`; a material decision, scope, policy, or
  authority issue returns `Escalate`.
- `BLOCKED` returns an operational gap only after writer inactivity and exact
  HEAD, status, diff, commit, and ownership attribution are established.
- `NEEDS_CONTEXT` becomes `BLOCKED` when the missing input is safely resolvable
  within approved authority, or `Escalate` when it is a user-owned decision or
  missing authority.

After any non-`DONE` response, or any response accompanied by partial edits or a
partial commit, require the scheduling adapter or direct lead to establish writer
inactivity and repository-state ownership before resume or replacement. Never
allow overlapping writers.

## Produce task evidence

From `pre-implementation`, perform this sequence for the current task:

1. Record and validate the exact task base commit and starting status.
2. Implement only the declared scope with the selected discipline.
3. Run every exact task verification command and record observed results.
4. Inspect the pre-commit working-tree diff, including unrelated state.
5. Create only the declared task commit.
6. Record the new head commit.
7. Inspect the exact base-to-head range from the preserved task base through the
   new head.
8. Assemble and validate the current reviewer evidence bundle.
9. Run the policy-selected per-task gate against that exact range.
10. Apply severity normalization and the active Acceptance threshold.
11. Record verification, commit, range, gate, normalization, and unresolved-gap
    evidence.

From another validated lifecycle phase, perform only its unfinished suffix. In
particular, a committed gate-pending record resumes at step 9 with the same
commit, range, and evidence bundle.

After a task commit and validated evidence bundle, transition to
`committed evidence ready/gate pending` before dispatching the gate. A surviving
authorized finding transitions to `correction pending`; attributable unfinished
edits remain `implementation/working-tree pending`. Only a completed selected
gate can produce `Accepted`.

Do not replace a task's exact range with a later aggregate plan range. Approval
is current only for the exact head and range actually reviewed.

## Assemble current reviewer evidence

Before any reviewer dispatch, assemble one current evidence bundle containing:

- the canonical task context identity/reference, without another inline task or
  policy copy;
- writer identity, writer status, and the complete writer report;
- the candidate current HEAD, which becomes the accepted/current head only when
  this gate approves it;
- the exact task base, head, base-to-head range, and inspected diff contents;
- every fresh task verification command, expected result, observed result, and
  whether it matched for this same code state;
- the authoritative changed-file list;
- repository-guidance identity/reference or an immutable snapshot;
- task/fix commits, pre-commit inspection evidence, unresolved gaps, and evidence
  timestamps or sequence identity needed to establish freshness.

Confirm that the bundle context reference resolves to the one canonical context,
repository HEAD still equals the bundle head, the diff resolves exactly from the
recorded base to that head, the changed-file list matches that diff, verification
ran after the last content edit and matched every expected result, the writer
report identifies the same commit, and no current state makes the evidence stale.
A lead writer must provide the same report fields as an implementer.

Pass the canonical context once and complete evidence bundle once with each
selected reviewer role contract to `agent-teams-driven-development`. Do not
dispatch a reviewer with a missing, partial, contradictory, duplicated, or stale
bundle. Return `BLOCKED` with the exact evidence gap instead.

## Select and run only the active gate

Resolve the mode before loading any reviewer prompt:

- For `focused`, load only
  [focused-reviewer-prompt.md](../agent-teams-driven-development/focused-reviewer-prompt.md)
  and run one combined specification-and-quality gate. When the user explicitly
  prohibits agents, or the approved policy explicitly selects a lead combined
  pass, the lead may run that complete pass. Otherwise request the selected
  read-only combined reviewer, queue it deterministically while runtime capacity
  is constrained, and return `BLOCKED` if the required reviewer cannot be
  established. Do not substitute the lead solely because a reviewer or slot is
  unavailable.
- For `adaptive` and `deep`, load only
  [spec-reviewer-prompt.md](../agent-teams-driven-development/spec-reviewer-prompt.md)
  and
  [code-quality-reviewer-prompt.md](../agent-teams-driven-development/code-quality-reviewer-prompt.md),
  then require independent read-only specification and quality agents. An
  explicit no-agent instruction conflicts with the approved mode: return
  `Escalate` for agent permission or a complete approved policy change. Otherwise
  queue both selected roles under the policy. If runtime capacity or role
  availability cannot establish either required independent role, return
  `BLOCKED` with the exact operational gap.

Pass the canonical task context once, current evidence bundle once, and only the
selected complete role contracts to `agent-teams-driven-development`. Never load
all reviewer prompts speculatively. Reviewers remain read-only and review the
exact current base-to-head range. Runtime shortage is an operational `BLOCKED`
state, not a policy `Escalate`; explicit no-agent authority conflicts with
`adaptive`/`deep` and is not treated as runtime unavailability.

## Normalize findings and apply Acceptance

Before applying Acceptance:

- require every specification finding to use exactly `Must Fix` or
  `Should Improve`; missing or unknown severity is a schema gap that requires
  schema-compliant reviewer re-output, not inference;
- for adaptive and deep quality findings, map an evidence-qualified `Critical`
  to `Must Fix` and an evidence-qualified `Important` to `Should Improve`, while
  preserving both original and normalized labels;
- do not promote a lower native severity or a non-finding;
- preserve the focused gate's native `Must Fix` and `Should Improve` labels
  without remapping.

Apply the complete policy's Acceptance threshold only after normalization. Keep
only concrete reachable behavior or contract violations with cited evidence,
impact, and a specific correction. Exclude preference-only, speculative,
unsupported, or already-decided objections that have no new evidence.

## Fix and re-review with bounded retries

For each verification failure or surviving finding, record a stable gate key
before any implementer follow-up or lead correction. Base the key on the failed
command or review requirement and concrete behavior, not a transient line number.
For each key, record the attempt number, causal hypothesis, planned action, and
fresh verification or review evidence.

Permit attempt 1 and, if the same stable key remains, one materially informed
attempt 2. Assign a new key only when the failed contract or behavior is
materially different. If the same key survives attempt 2, or another action would
repeat a correction without new evidence, do not attempt a third correction;
return `Escalate` with the key and complete attempt history.

For every authorized in-scope correction:

1. give the existing writer the stable key, attempt record, and correction;
2. run fresh exact task verification;
3. inspect the pre-commit fix diff;
4. create only the declared fix commit;
5. record the new head;
6. inspect the updated exact task base-to-head range;
7. rerun the same complete policy-selected gate against that current range.

Do not reuse a stale verification result, gate approval, head, or range.

## Return task acceptance

Return exactly one task status:

- `Accepted` when exact verification passes and the complete selected gate
  approves the current head;
- `BLOCKED` when required evidence, a safe writer state, a command, permission,
  range, reviewer, or other operational prerequisite cannot be established;
- `Escalate` for a material decision, scope or policy change, explicit
  adaptive/deep no-agent independence conflict, plan deviation, or exhausted
  stable-key retry.

Include the canonical context identity, lifecycle phase, writer identity and
status, task and fix commits, exact task base, new head, exact base-to-head range,
verification and pre-commit evidence, evidence-bundle identity, gate and
normalization results, policy provenance, capacity or queue evidence, stable-key
attempt history, every remaining gap, and the exact re-entry condition.

For post-commit reviewer or capacity `BLOCKED`, return at least the task base,
committed current head, commits, validated verification/pre-commit/range/evidence
bundle, writer status and confirmed inactivity, exact pending gate and roles,
operational gaps, and re-entry condition under
`committed evidence ready/gate pending`. Return this partial task record to the
invoking coordinator or `execute-plan`; do not advance to another task or
cross-phase gate.
