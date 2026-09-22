# Plan-time role allocation

Resolve these allocations before execution. Keep the same required quality and
Acceptance bar for every Task Lead model.

| Role | Default model | Effort | Binding |
| --- | --- | --- | --- |
| Task Lead | `gpt-6-sol` | `xhigh` | Explicit independent Codex startup |
| Task Lead, justified plan override | `gpt-6-astra` | `high` | Same root contract, explicit startup |
| verification-runner | `gpt-6-luna` | `low` | Native profile |
| focused-reviewer | `gpt-6-sol` | `high` | Native profile; lightweight |
| spec-reviewer | `gpt-6-sol` | `high` | Native profile |
| implementation-quality-reviewer | `gpt-6-sol` | `high` | Native profile |
| risk-reviewer | `gpt-6-sol` | `xhigh` | Native profile; one perspective |
| finding-integrator | `gpt-6-sol` | `high` | Native profile; conditional |
| design-alignment-reviewer | `gpt-6-sol` | `xhigh` | Native profile |

The Feature Lead is intentionally absent: it already runs on the user's session
defaults. Do not pin it in the plan or modify global settings for Task startup.
Do not use max effort or runtime promotion/fallback.

Use Sol/xhigh as the default for new Task Leads. Assess the whole Task before
confirming that allocation: implementation, interpreting verification evidence,
and correction must all fit the model. Ground the choice in repository evidence
about the implementation approach, affected boundaries, likely failure modes,
and available verification. Extending an established pattern with understood
interactions and direct regression evidence is a typical Sol/xhigh fit.

Select Astra/high at planning time when the Task requires difficult judgment,
such as investigating unexplained behavior, reasoning across non-local
invariants, designing concurrency or recovery details, or preserving
compatibility, security or data integrity across shared boundaries. Long
refactors or ambiguous verification results can also make the work demanding.
Name the concrete difficulty or uncertainty and explain why Astra's deeper
reasoning and judgment are expected to help through implementation and correction.

The default is not a substitute for this assessment. File counts, a settled
specification, apparently simple code, or the existence of tests alone do not
establish Sol's fit. Routine private implementation choices do not alone require
Astra; when consequential implementation difficulty or evidence interpretation
remains uncertain after available investigation, select Astra/high. Resolve
missing user-owned design decisions through design discussion; a model choice
does not settle them.

Use the evidence available during planning and label quality/cost expectations
as estimates. Do not infer that Sol/xhigh and Astra/high are equivalent from
effort labels or aggregate benchmark scores.

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
