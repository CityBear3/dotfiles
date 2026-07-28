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

## Require canonical task context

Before implementation, require one canonical context containing:

- the complete task specification;
- the original request, Design Doc, or other approved decision source, including
  explicit non-goals;
- the required implementation discipline;
- the approved workspace and working directory;
- the exact task base commit;
- every exact task verification command and expected result;
- the complete active Review policy and its provenance;
- configured and observed capacity plus queue rules;
- prior stable gate keys and complete attempt history;
- when present, approved-plan task context limited to the plan path,
  task-specific decisions, non-goals, and file responsibilities.

Do not duplicate the complete task specification or Review policy inside optional
plan context. Reject missing, stale, contradictory, or mode-inconsistent input.
Return a named gap through the invoking skill to
`agentic-engineering-workflow`; do not infer a decision, expand scope, or weaken
evidence.

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

## Choose one writer and hand off implementation

Keep exactly one writer: either the lead or one `implementer`. Use the lead when
direct execution is authorized. When dispatching an implementer, load
[implementer-prompt.md](../agent-teams-driven-development/implementer-prompt.md)
only at that point, combine its complete role contract with the canonical task
context, and pass that already-selected contract to
`agent-teams-driven-development` for scheduling.

The implementation handoff must give the writer the complete task, decisions and
non-goals, discipline, file responsibilities, workspace, base commit, exact
verification, repository guidance, and output contract. Require the writer to
preserve unrelated changes and report files, commands, observed results,
concerns, and every commit it creates.

For production behavior, use a red-green-refactor loop and record the observed
red failure. For content, configuration, refactoring, or mechanical migrations,
apply the declared discipline and preserve the relevant green baseline.

## Produce task evidence

Perform this sequence for the current task:

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

Do not replace a task's exact range with a later aggregate plan range. Approval
is current only for the exact head and range actually reviewed.

## Assemble current reviewer evidence

Before any reviewer dispatch, assemble one current evidence bundle containing:

- the candidate current HEAD, which becomes the accepted/current head only when
  this gate approves it;
- the exact task base, head, base-to-head range, and inspected diff contents;
- the writer report, including its identity, changed files, commits, commands,
  observed results, and concerns;
- every fresh task verification command and observed result for this same code
  state;
- applicable repository guidance and working directory;
- the authoritative changed-file list;
- the canonical task context, complete active Review policy, and provenance.

Confirm that repository HEAD still equals the bundle head, the diff resolves
exactly from the recorded base to that head, the changed-file list matches that
diff, verification ran after the last content edit, the writer report identifies
the same commit, and no current state makes the evidence stale. A lead writer
must provide the same report fields as an implementer.

Pass the complete bundle unchanged with each selected reviewer role contract to
`agent-teams-driven-development`. Do not dispatch a reviewer with a missing,
partial, contradictory, or stale bundle. Return `BLOCKED` with the exact evidence
gap instead.

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

Pass the canonical task context, current evidence bundle, and only the selected
complete role contracts to `agent-teams-driven-development`. Never load all
reviewer prompts speculatively. Reviewers remain read-only and review the exact
current base-to-head range. Runtime shortage is an operational `BLOCKED` state,
not a policy `Escalate`; explicit no-agent authority conflicts with
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

Include the writer identity, task and fix commits, exact task base, new head,
exact base-to-head range, verification evidence, gate and normalization results,
policy provenance, capacity or queue evidence, stable-key attempt history, and
every remaining gap. Return this task record to the invoking coordinator or
`execute-plan`; do not advance to another task or cross-phase gate.
