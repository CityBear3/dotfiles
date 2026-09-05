# Plan-time role allocation

Resolve these allocations before execution. Models are not a proxy for the
required quality bar, and complexity signals are not an automatic classifier.

| Role | Default model | Effort | Binding |
| --- | --- | --- | --- |
| Task Lead | `gpt-5.6-sol` | `high` | Explicit independent Codex startup |
| Task Lead, engineer-approved override | `gpt-6-astra` | `high` | Same root contract, explicit startup |
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

Propose Sol/high by default, and Astra/high when required judgment, quality,
reasoning demands or risk justify its cost. Consider public/shared boundaries,
non-local invariants, concurrency/recovery, compatibility, security, data
integrity and demanding evidence interpretation, but do not treat any signal
as an automatic rule. Even small, straightforward code may demand senior/staff
engineering quality. File counts, apparent simplicity, property tests or
expected token counts alone cannot decide the model. Sol does not lower the
Acceptance bar; Astra does not guarantee it or resolve missing design authority.

Record a default table once and Task-specific overrides. For each Task show
effective allocations, required quality, relevant risks/reasoning demands and
quality/cost rationale. Engineer confirmation happens with Implementation Plan
approval, not an additional per-Task startup gate. Allocations remain fixed
through correction and replacement. Unavailable model/effort is BLOCKED.

The Task root uses explicit model, normal effort and Plan-mode effort startup
settings, exact worktree and the shared
[Task Lead contract](../../execute-task/references/task-lead.md). A native
profile alone cannot configure an independent root. Check leaves use their
effective profile allocation, not implicit root inheritance; a profile/runtime
conflict prevents dispatch. Model choice never changes sandbox, network or
publication authority.

Use accepted-Task/Feature cost when interpreting normal-run usage: distinguish
uncached input, cached input, output, model/service tier and correction overhead.
Keep quality and elapsed time alongside monetary cost. No token quota, fixed
Sol/Astra percentage, savings promise, benchmark or new telemetry is required.
