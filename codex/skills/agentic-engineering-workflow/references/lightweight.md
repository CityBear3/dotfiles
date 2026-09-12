# Lightweight work

## Use the lightweight path only when fully eligible

Require all of these conditions after investigation:

- the user explicitly requested a change;
- the objective, expected behavior, and scope are uniquely determined;
- no architecture, public API or other public contract, schema, or error-model
  decision changes;
- no material user-owned trade-off remains;
- the work is one coherent change;
- the work needs no external write, publication, destructive action, or material
  scope expansion.

Do not use file count or changed-line count as eligibility criteria. Treat
security or permission boundaries, persistent-data migration, concurrency or
recovery guarantees, and data-loss risk as disqualifying unless investigation
shows that the requested change does not alter that contract.

When the explicit request and repository evidence unambiguously establish the
goal, expected behavior, scope, and constraints, retain the request's exact
explicit content as the approved alignment source. Add no decision-record file
or separate alignment approval gate. If only non-material omissions remain,
derive one concise alignment record, present it, and ask once for confirmation;
an unpresented or unconfirmed Agent summary is not shared authority. A material
user-owned choice, durable coordination requirement, or unrecoverable in-memory
contract uses the planned path.

Treat the complete lightweight alignment source as implementation approval when
every eligibility criterion holds. Confirm the workspace with
`create-workspace`. Derive one concise in-memory Feature Contract from the
alignment source and repository evidence; because the route is one coherent
task, use the same contract as its Task Contract. Identify the material property
and reliable verification oracle, then apply the `test-driven-development`
applicability decision before selecting discipline. Record TDD as `applicable`,
`not applicable`, or `required but blocked`, with its reason; select a
contract-appropriate baseline and validation discipline when it is not
applicable.

The lightweight Feature/Task Contract must make the context and goal, scope and
non-goals, design sources and approved decisions with precedence, observable and
preserved behavior, compatibility and material failure behavior,
responsibilities and interfaces, protected constraints, verification
obligations, evidence-backed assumptions, and explicitly approved deferrals
unambiguous. Record that there are no approved deferrals when none exist. It adds
no contract file or separate approval gate. Keep it recoverable in the current
handoff and evidence for the duration of the task.

If implementation exposes a disqualifying risk or material decision, preserve
the evidence and stop the lightweight path. Return to `design-discussion`, then
planning after the user settles the revised scope. Do not silently broaden the
policy and continue.

Also promote to the planned path when the work no longer fits one coherent task,
needs durable cross-session coordination, or a material part of the in-memory
contract cannot be recovered after interruption or context compaction. Preserve
observed work and state; do not improvise another lightweight task. Record the
original lightweight task base, current head, exact unaccepted range and commits,
changed files, writer and gate evidence, ownership attribution, concerns, and
gaps. Never let preserved unaccepted work become the new plan's invisible
baseline.

## Prepare the lightweight task

Before invoking `execute-lightweight-task`, derive a concise Review context from
the approved request and repository evidence. State the artifact and purpose,
its consumers and interpretation or execution model, material quality criteria
and realistic failures, approved non-problems, and inapplicable assumptions.
Keep it separate from the Review policy.

Materialize the complete lightweight policy before implementation. If completing
that policy requires a material user-owned choice, or observed risk makes
`focused` inappropriate, return to the planned path before invoking
`execute-lightweight-task`. Do not silently select or strengthen policy to keep the
lightweight path.

Use `focused` as the lightweight default:

- one independent `focused-reviewer` covering specification, implementation
  and test quality, with the Feature Lead as direct sole writer;
- a mechanical `verification-runner` before review;
- fixed runner Luna/low, focused-reviewer Sol/high and conditional
  finding-integrator Sol/high bindings; Feature Lead retains session defaults;
- no second feature review when that one Task PR covers the complete contract;
- explicit reasons for skipped perspectives;
- direct root dispatch of phase-valid leaves with no descendants;
- runtime-managed thread admission, preserving selected roles as pending in
  policy order when admission is temporarily unavailable;
- conditional finding integration for overlap, conflict, authority defects,
  scope-sensitive remedies or non-trivial attribution; no integrator for
  all-clean reports or a simple clear finding eligible for direct triage;
- the common Acceptance threshold.

Acceptance keeps only artifact-applicable findings with an approved requirement,
concrete reachable evidence, material consequence, and proportionate correction.
Preference, speculation, generic best practice, optional polish, and objections
to approved decisions without new evidence are not findings. A proposed new
state machine, schema, identity system, or comparable mechanism is `Escalate`
unless it is necessary and proportionate to a proven in-scope violation.

When the required risk or independence makes focused lightweight review
inappropriate, return to planned discussion before editing. Planned adaptive
and deep policies retain separate specification and implementation-quality
review. If any required independent reviewer cannot be established, report
BLOCKED; do not substitute a lead pass. A no-agent instruction conflicting with
an approved gate requires engineer policy authority.

Give `execute-lightweight-task` one plain-language task handoff containing:

- the complete in-memory Feature/Task Contract, including its design sources and
  approved decisions, goal, observable and preserved behavior, compatibility,
  material failure behavior, responsibilities, interfaces, constraints,
  non-goals, verification obligations, assumptions, and approved deferrals;
- the Review context and complete Review policy;
- the discipline and applicable repository guidance;
- task workspace and branch, Task PR identity, planned base ref and exact
  commit, current head, merge base, exact range, inspected diff, and starting
  Git status including index, worktree, and relevant untracked state;
- responsibility and ownership boundaries;
- the responsibility-scoped commit intent and writer authority to select its
  message unless the request contractually fixes that message;
- the applicable verification route and expected observations, including the
  root-owned Task-loop owner's obligation to bind an in-memory current-target
  Verification Matrix before dispatch, including a pre-commit snapshot when
  appropriate, and apply the [evidence-applicability rule](evidence.md#reuse-evidence-by-applicability);
- attributable commits, prior verification and review, concerns, gaps, and
  re-entry evidence when applicable;
- the root-owned lightweight loop identity, selected or pending roles, and
  attributable runtime-rejection or interruption evidence when applicable;
- exact files, signatures, ordering, or commands only when their identity is
  contractually significant.

This is the common Task evidence plus exactly the lightweight variant. Do not
add a Herdr workspace, Task DAG, PR topology, or another planned-only field.

Do not dispatch roles, load reviewer prompts, implement, commit, or manage
corrections in this coordinator.

When an authorized lightweight correction re-enters `execute-lightweight-task`,
retain H1, reports/triage and the complete selected coverage. Require a bounded
H2 commit and a current matrix with fresh affected checks and explicitly carried
unaffected verification. The loop owner supplies
review's impact map; finding owners and affected reviewers rerun, while carried
coverage needs explicit non-invalidation evidence. Uncertainty means rerun.
`review` owns those rules. Ordinary planned corrections stay inside the
independent Task Lead's `execute-task`; Feature-level/integration feedback
routes to the owning Task through `execute-plan`.
