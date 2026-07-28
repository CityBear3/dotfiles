---
name: agent-teams-driven-development
description: Execute an approved implementation plan through one writer and policy-selected per-task review loops, using bounded Codex subagents and runtime-aware capacity. Use from execute-plan unless the user explicitly requests direct execution without agents.
---

# Agent-teams driven development

The lead schedules all work. Subagents do not spawn descendants.

## Inputs

Require:

- an approved plan path;
- the complete approved Review policy and selected mode;
- a non-default feature branch or approved workspace;
- repository guidance and working directory;
- configured maximum capacity and currently observed live agents.

Require the policy context to include risk surfaces, per-task gate, final
required reviewers, conditional reviewers with triggers, skipped perspectives
with reasons, residual risk, capacity and queue rules, and Acceptance threshold.

Read the four task templates beside this file before dispatch:

- [implementer-prompt.md](implementer-prompt.md)
- [focused-reviewer-prompt.md](focused-reviewer-prompt.md)
- [spec-reviewer-prompt.md](spec-reviewer-prompt.md)
- [code-quality-reviewer-prompt.md](code-quality-reviewer-prompt.md)

Use the `implementer` profile when selectable; otherwise include its complete
fallback prompt in the task message.

## Select the per-task gate

Resolve the approved mode before reviewer dispatch:

- `focused`: dispatch one read-only `code-reviewer` with an explicit combined
  specification-compliance and code-quality contract. When that profile is
  unavailable, use the complete focused reviewer fallback prompt.
- `adaptive`: dispatch an independent read-only `spec-reviewer` and
  `code-quality-reviewer`.
- `deep`: dispatch the same independent read-only `spec-reviewer` and
  `code-quality-reviewer`.

When either independent profile is unavailable, use its complete fallback prompt.
Reject a missing, unknown, or mode-inconsistent per-task gate. Carry the final
reviewer fields in the execution context, but do not dispatch final review from
this skill.

Treat [spec-reviewer-prompt.md](spec-reviewer-prompt.md) as the complete
path-independent specification output contract. Require the named
`spec-reviewer` profile message, fallback prompt, and no-agent/direct
specification pass to use its complete output schema: every finding is exactly
`Must Fix` or `Should Improve`. Apply this contract on planned and lightweight
paths. Treat a missing or unknown specification severity as a schema gap. Do not
infer, normalize, or translate the severity; require corrected schema-compliant
output before applying Acceptance.

## Capacity

Use `list_agents` before each dispatch wave. The effective limit is the lower of
the approved Review policy's configured capacity and capacity observed from the
current runtime. Count the lead. Never exceed six total threads.

Keep one implementer as the only writer and every reviewer read-only. For
`focused`, run or queue the one combined reviewer. For `adaptive` and `deep`, run
the two independent reviewers concurrently when two slots are available;
otherwise queue them without reducing the approved gate.

## Per-task loop

1. Record the task's base commit.
2. Give the implementer the full task, approved decisions, dependencies, working
   directory, exact verification command, and output contract.
3. Use `wait_agent` in bounded intervals and inspect `list_agents` regularly. Send the user a progress update at least every 60 seconds during long work.
4. Require the implementer to report changed files, exact verification,
   pre-commit working-tree diff inspection, task commit, results, and concerns.
5. Record the new head commit and inspect the exact base-to-head diff range.
   Include the complete task specification, relevant Design Doc and plan
   sections, implementer report, and observed verification evidence in every
   reviewer message.
6. Dispatch the approved focused, adaptive, or deep per-task gate against that
   exact range.
7. Normalize profile-native severity at the orchestration boundary. The named
   `code-quality-reviewer` profile and its fallback use `Critical` and
   `Important`: map an evidence-qualified `Critical` finding to `Must Fix` and an
   evidence-qualified `Important` finding to `Should Improve`. Preserve and
   report both the original and normalized labels. Do not silently raise a lower
   native severity or non-finding to `Should Improve`.
8. Require APPROVED or evidence-based findings that cite file and line, violated
   requirement or quality consequence, reachable evidence, impact, and a concrete
   correction. Drop preference-only and speculative findings under the policy's
   Acceptance threshold.
9. Send verified in-scope findings to the existing implementer with
   `followup_task` when idle or `send_message` when already running.
10. After fixes, require fresh task verification, inspect the pre-commit
    working-tree fix diff, require the declared fix commit, record the new head,
    inspect the updated exact base-to-head range, and rerun the same complete gate
    against that range.

On plan re-entry, reload the approved Review policy and rerun the same gate; do
not reuse a prior approval or silently change mode. Stop and return evidence to
the coordinator when a design decision is missing, the plan must change, or its
retry contract stops the same gate. Do not let reviewers edit or spawn
descendants.

## Completion

Complete only when every plan task has:

- an implementation commit;
- observed task verification evidence;
- approval from the focused combined reviewer, or both adaptive/deep independent
  reviewers, as selected by the approved policy.

Return task commits, exact ranges, implementer evidence, verification, gate
approvals, and the complete Review policy to `agentic-engineering-workflow`. Do
not start global `verify`, final review, publication, merge, or teardown from
this skill.
