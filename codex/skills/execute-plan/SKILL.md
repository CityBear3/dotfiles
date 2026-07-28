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
  this session. For `focused`, perform one combined specification-and-quality
  pass. For `adaptive` or `deep`, perform distinct specification and quality
  passes and evaluate each independently.

In direct execution, preserve the same task, commit, exact-diff, evidence,
Acceptance, fix, fresh re-review, and plan-re-entry contracts. Do not weaken or
replace the approved gate because subagents are unavailable. Do not spawn agents
contrary to the user's current instruction.

## Execute

Follow tasks in dependency order. Apply the discipline declared by each task. Preserve unrelated changes and do not add speculative work.

For every task:

1. record the task's base commit, specification, and policy context;
2. implement only its declared scope;
3. run its exact verification;
4. inspect the exact base-to-head diff;
5. create the declared task commit;
6. apply the approved per-task gate to that exact range;
7. record implementer evidence, verification, gate result, and every gap.

After an in-scope fix, run fresh task verification and the same approved gate
against the updated exact range. On plan re-entry, reload the approved policy and
reapply the same gate; do not reuse approval for a stale head. Return a plan
deviation, missing decision, or persistent gate history to the coordinator under
its escalation and retry contract.

## Handoff

After every task satisfies its approved gate, return control to
`agentic-engineering-workflow` with task commits, exact diff ranges, verification
evidence, gate results, the complete Review policy, and unresolved gaps. Do not
start global `verify`, final review, publication, or branch disposition from
this skill.
