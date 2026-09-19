# Plan-time role allocation

Resolve these allocations before execution. Keep the same required quality and
Acceptance bar for every Task Lead model.

| Role | Default model | Effort | Binding |
| --- | --- | --- | --- |
| Task Lead | `gpt-6-astra` | `high` | Explicit independent Codex startup |
| Task Lead, justified plan override | `gpt-5.6-sol` | `high` | Same root contract, explicit startup |
| verification-runner | `gpt-5.6-luna` | `low` | Native profile |
| focused-reviewer | `gpt-5.6-sol` | `high` | Native profile; lightweight |
| spec-reviewer | `gpt-5.6-sol` | `high` | Native profile |
| implementation-quality-reviewer | `gpt-5.6-sol` | `high` | Native profile |
| risk-reviewer | `gpt-5.6-sol` | `xhigh` | Native profile; one perspective |
| finding-integrator | `gpt-5.6-sol` | `high` | Native profile; conditional |
| design-alignment-reviewer | `gpt-5.6-sol` | `xhigh` | Native profile |

The Feature Lead is intentionally absent: it already runs on the user's session
defaults. Do not pin it in the plan or modify global settings for Task startup.
Do not use max effort or runtime promotion/fallback.

Use Astra/high for new Task Leads, including when implementation difficulty or
the judgment needed through verification and correction remains uncertain.
Assess the whole Task: public/shared boundaries, non-local invariants,
concurrency/recovery, compatibility, security, data integrity, and interpreting
evidence can make implementation demanding even when the requested edit is small.

Consider Sol/high when concrete repository evidence makes the implementation
approach, affected boundaries, likely failure modes, and verification clear
enough to expect the same required quality at lower total completion cost.
Explain that fit briefly using the Task's actual work, such as extending an
established implementation pattern with understood interactions and direct
regression evidence. File counts, a settled specification, apparently simple
code, or the existence of tests alone do not establish that fit.

Use the evidence available during planning and label cost expectations as
estimates. When model suitability remains unclear, select Astra/high and
continue preparing the plan. Resolve missing user-owned design decisions
through design discussion; a model choice does not settle them.

Record a default table once and Task-specific overrides. For each Task show
effective allocations, required quality, relevant risks/reasoning demands and
quality/cost rationale. Engineer confirmation happens with Implementation Plan
approval, not an additional per-Task startup gate. Allocations remain fixed
through correction and replacement. Unavailable model/effort is BLOCKED.
Already-approved plans retain their approved allocations.

The Task root uses explicit model, normal effort and Plan-mode effort startup
settings, exact worktree and the shared
[Task Lead contract](../../execute-task/references/task-lead.md). A native
profile alone cannot configure an independent root. Check leaves use their
effective profile allocation, not implicit root inheritance; a profile/runtime
conflict prevents dispatch. Model choice never changes sandbox, network or
publication authority.

Judge cost through Task/Feature Acceptance, including investigation,
implementation, verification, correction and re-review. When usage is available,
distinguish uncached input, cached input, output and model/service tier. Consider
elapsed time and human intervention alongside monetary cost and quality. A lower
token price does not establish a lower completion cost, and neither model
guarantees savings. No token quota, fixed Sol/Astra percentage, mandatory
benchmark or new telemetry is required for selection.
