---
name: execute-plan
description: Execute an approved implementation plan with its approved review policy on a feature branch, normally through bounded agent teams. Use after plan approval; honor an explicit request to execute directly without subagents.
---

# Execute an approved plan

## Entry

Confirm:

- the plan is approved and current;
- its complete Review policy is approved;
- the working tree is on the intended non-default feature branch or workspace;
- unresolved dirty changes are understood;
- baseline and required tools are available.

If the current checkout is unsuitable, use `create-workspace`. Stop for missing design decisions rather than choosing them during execution.

## Load the review policy

Before any implementation or reviewer dispatch, read the complete approved Review
policy and validate:

- mode: `focused`, `adaptive`, or `deep`;
- risk surfaces and rationale;
- the mode-consistent per-task gate;
- named final required reviewers and named conditional reviewers with triggers;
- named skipped perspectives with reasons;
- residual risk;
- configured and observed capacity plus queue rules;
- the Acceptance threshold.

Keep the complete policy, approved task scope, Design Doc or decision source,
non-goals, and current evidence in the execution context. Preserve that context
across tasks, fixes, and plan re-entry. Do not infer a missing field, substitute a
different gate, or dispatch until the coordinator resolves an invalid or
incomplete policy.

## Choose execution mode

- Default: use `agent-teams-driven-development`.
- If the user explicitly says not to use agents, execute each task directly in
  this session without dispatching reviewers.

In direct execution, preserve the same task, commit, exact-diff, evidence,
Acceptance, fix, fresh re-review, and plan-re-entry contracts. Do not weaken or
replace the approved gate because subagents are unavailable. Do not spawn agents
contrary to the user's current instruction.

For direct/no-agent execution, read these complete prompts directly:

- [focused-reviewer-prompt.md](../agent-teams-driven-development/focused-reviewer-prompt.md);
- [spec-reviewer-prompt.md](../agent-teams-driven-development/spec-reviewer-prompt.md);
- [code-quality-reviewer-prompt.md](../agent-teams-driven-development/code-quality-reviewer-prompt.md).

Apply their complete role, context, and output contracts without dispatch:

- for `focused`, perform one combined specification-and-quality pass using the
  focused reviewer prompt;
- for `adaptive` and `deep`, perform distinct sequential specification and
  quality passes using the spec and code-quality reviewer prompts, and evaluate
  each pass independently.

Before applying Acceptance in direct execution:

- require every specification finding to be exactly `Must Fix` or
  `Should Improve`; treat a missing or unknown specification severity as a schema
  gap, do not infer, normalize, or translate it, and obtain schema-compliant
  re-output;
- for quality findings, map an evidence-qualified `Critical` to `Must Fix` and an
  evidence-qualified `Important` to `Should Improve`; preserve both the original
  and normalized labels, and do not raise a lower native severity or non-finding.

Keep this direct/no-agent gate contract equivalent to the agent-team path.

## Execute

Follow tasks in dependency order. Apply the discipline declared by each task. Preserve unrelated changes and do not add speculative work.

For every task:

1. record the task's base commit, specification, and policy context;
2. implement only its declared scope;
3. run its exact verification;
4. inspect the pre-commit working-tree diff;
5. create the declared task commit;
6. record the new head commit;
7. inspect the exact base-to-head diff range;
8. apply the approved per-task gate to that exact range;
9. record implementer evidence, verification, gate result, and every gap.

After an in-scope fix, run fresh task verification, inspect the working-tree fix
diff, create the declared fix commit, record the new head, inspect the updated
exact base-to-head range, and rerun the same complete approved gate against that
range. Use this sequence in both direct and agent-team execution. On plan
re-entry, reload the approved policy and reapply the same gate; do not reuse
approval for a stale head. Return a plan deviation, missing decision, or
persistent gate history to the coordinator under its escalation and retry
contract.

## Handoff

After every task satisfies its approved gate, return control to
`agentic-engineering-workflow` with task commits, exact diff ranges, verification
evidence, gate results, the complete Review policy, and unresolved gaps. Do not
start global `verify`, final review, publication, or branch disposition from
this skill.
