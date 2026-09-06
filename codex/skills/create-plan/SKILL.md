---
name: create-plan
description: Decompose an approved Feature Contract into PR-scoped Task Contracts, a dependency DAG, a PR topology, and an Implementation Plan. Use after the contract is separately approved and current.
---

# Create an implementation plan

Write a plan that a fresh independent Task Lead can execute without reconstructing design,
feature success, or task responsibility from the conversation.

## Entry

Require the coordinator to supply:

- the exact approved, current Feature Contract and its workspace path;
- its approved Design Doc or decision record sources;
- approval evidence for the exact contract content;
- working directory, feature workspace, and current repository state.
- when replanning, every prior accepted task with its exact contract content,
  dependencies, consumed interfaces, base, head, range, and gate evidence;
- when promoted from lightweight work, its original task base, current head,
  exact unaccepted range and commits, changed files, writer and gate evidence,
  ownership attribution, concerns, and gaps.

Reject a missing, draft, materially edited, contradicted, or source-incomplete
Feature Contract. Return unresolved architecture, responsibility, public or
shared interfaces, schemas, error semantics, scope, or material trade-offs to
`agentic-engineering-workflow`. Do not repair design while decomposing it.

## Investigate

Read the Feature Contract, its design sources, current implementation, tests,
repository guidance, and recent history. Map component and state ownership,
dependency direction, shared interfaces, verification routes, and likely writer
overlap before splitting tasks. Confirm that the contract still describes the
current repository.

## Decompose by responsibility

Derive tasks from responsibility and state ownership, dependency direction,
shared interfaces, independently observable completion, and integration
obligations. Use explicit dependencies when execution order alone resolves a
boundary.

Make one Task Contract one independently reviewable PR candidate by default.
When proposed tasks cannot remain buildable, verifiable, or contractually valid
as separate PRs, combine the inseparable responsibility into one Task Contract
or return the unresolved boundary to design. Do not keep nominally separate
tasks merely to place their mixed changes in one PR.

Treat difficult decomposition as design feedback. Return to the coordinator
when tasks would need conflicting owners, duplicated authority, an undefined
shared interface, missing failure semantics, or a new feature decision. Do not
hide the gap in an implementation step.

Record every cross-task interface once in a `Shared interface contracts` section
with one owner and named implementers and consumers. Include an exact signature
or representation only when another task's correctness depends on details such
as ownership, async behavior, thread guarantees, absence representation, error
type, schema, or ordering. Each Task Contract references this shared definition.

Build a Feature Contract coverage table. Map every contract obligation to one or
more Task Contracts or to an explicitly integration-only proof. Explain
deliberate overlap. Reject unexplained gaps or duplicated ownership.

Record two distinct graphs:

- a **Task dependency DAG** for semantic readiness and dependency release;
- a **PR topology** that gives each Task PR one planned base relationship.

Use sibling PRs for independent tasks and a linear stack for dependent chains.
For fan-in, choose an owner-visible deterministic order over the required parent
closure while preserving early independent implementation where ownership and
state permit it. Do not turn that Git order into a logical dependency. When only
a feature-level observation needs the combined tree, preserve independent
sibling PR relationships and use a temporary integration composition.

For every temporary integration composition, record its starting commit or
tree, exact accepted Task PR inputs and deterministic application order,
composition mechanism, workspace or temporary-ref strategy, identity checks,
and the point where it becomes eligible for separately authorized cleanup. The
approved composition must require no manual source edit or conflict resolution;
an unresolved composition conflict is a blocked integration input, not
delegated implementation work.

Record which tasks may implement before their final PR base exists. Such work
may produce a non-accepted candidate, but the task must be restacked onto its
planned final base and obtain fresh authoritative verification and review before
it can release a dependent.

For replanning, add a `Re-entry impact` section. Retain a prior accepted result
only when its exact Feature Contract authority, assigned Feature clauses, Task
Contract, dependencies, and relied-on shared interfaces remain semantically
unchanged. Mark every affected or transitively dependent result stale and map it
to the current Task Contract that must obtain fresh acceptance under both current
authorities.

For a lightweight promotion with preserved committed work, keep the original
lightweight base as the implementation base. Map every preserved change and file
to the complete new Task Contract set and define a first promotion-reconciliation
step that owns attribution and current-contract acceptance of the unaccepted
range. Distinguish later approved design and plan artifact state and include it
in the attributable reconciliation envelope at execution; do not absorb later
feature-source edits. Reject conflicting or incomplete attribution; do not make
the preserved current head an unreviewed baseline.

## Plan structure

Store the plan beside its Feature Contract at
`docs/plans/YYYY-MM-DD-<feature>/implementation-plan.md` unless stricter
repository guidance specifies another location. Reference the approved contract;
do not copy or mutate its meaning. Treat both files as ignored, workspace-only
execution artifacts. Do not force-add, stage, or commit either file unless the
user explicitly chooses archival.

Do not require a search cache or cache-policy section in every plan. When
costly discoveries merit reuse across independent sessions, the Feature Lead
may keep an optional `search-cache.md` beside the plan, following
`agentic-engineering-workflow`'s sharing and retention rules. Reference its path
and relevant consumers only if it exists and is useful. One-off findings can go
directly in a Task handoff. Cache absence, a missing entry, or staleness is not a
plan gap and requires no miss report or empty placeholder.

Include:

- goal, authorities and precedence, architecture summary, technologies, working
  directory, branch, and observed baseline;
- fixed decisions and explicit non-goals;
- shared interface contracts and their owners and consumers;
- the Task dependency DAG, deterministic ready order, PR topology, planned
  bases, fan-in linearizations, and exact integration-only composition inputs,
  order, mechanism, workspace, identity checks, and retention;
- each Task workspace mode, branch identity, exact or deterministic starting-ref
  resolution rule, ownership, concurrency eligibility, and shared-state or
  write exclusions;
- lazy Task workspace materialization and the boundary that Plan approval fixes
  workspace identity while later authorization to start `execute-plan` grants
  creation or reuse authority for that exact non-destructive local state;
- complete Feature Contract coverage, including integration-only obligations;
- a Review context;
- a separate complete Review policy;
- a default model/effort allocation table, Task-specific overrides and their
  required-quality/risk/cost rationale for engineer confirmation at approval;
- independent Herdr Task-session startup bindings and the shared Task Lead role
  source, with native Task-local checks rather than a shared Feature agent tree;
- Task Contracts ordered by dependency;
- re-entry impact and promotion reconciliation when either applies;
- Task PR acceptance and staleness rules, feature acceptance, review iteration,
  workspace-artifact lifetime and cleanup, and publication policy.

For each Task Contract include:

- purpose and expected result;
- Feature Contract clauses it satisfies;
- responsibility and ownership boundaries;
- applicable shared interfaces and adjacent-task obligations;
- protected constraints and invariants;
- observable task-level verification obligations;
- the Task-loop owner's in-memory current-head Verification Matrix obligation,
  including one bounded command/check, expected observation, and `FAIL` or
  `BLOCKED` non-match category per observable obligation, plus invalidation on
  head, range, controlling-authority, or material-route change;
- dependencies;
- PR unit, planned parent or sibling relationship, and final-base readiness;
- whether implementation may produce a candidate before that base exists;
- workspace mode, branch identity, exact or deterministic starting-ref
  resolution rule, ownership, concurrency eligibility, and staleness triggers;
- explicit non-goals;
- local decisions delegated to the implementation agent;
- effective Task Lead and selected-leaf allocations, inherited defaults or exact
  overrides, and required-quality/risk/cost rationale;
- discipline: the material property, reliable verification oracle,
  `test-driven-development` applicability decision and reason, or the explicit
  non-TDD baseline and validation discipline;
- a responsibility-scoped commit intent and whether the plan or writer selects
  its message;
- contractually significant files, signatures, ordering, commands, exact commit
  paths, or fixed commit message only when their identity is part of correctness.

## Choose contract detail, not procedure

Do not require exhaustive files, function-by-function steps, edit order, helper
structure, local algorithms, or exact commands by default. The writer may
discover private files inside its approved responsibility and must report the
actual changed files.

Treat reusable language conventions supplied by repository guidance or an
applicable Skill as implementation guidance. Reference that guidance without
copying its rules into the plan. Restate or fix a language-specific choice only
when its exact identity satisfies the contract-significance criteria below.

Fix exact detail when it defines a public or shared interface, writer ownership,
generated or manifest mapping, migration or compatibility sequence, safety
boundary, reproducible environment, authoritative coverage, or another
observable correctness condition. An unexpected private file is not a deviation;
a new owner, public seam, shared interface, invariant, or contract meaning is.

## Test planning

- Identify each Task's material property and reliable verification oracle.
- Record the `test-driven-development` applicability decision and reason. When
  applicable, name the causal behavioral slice that must be absent before the
  production edit and whether one focused test or a coherent, separately
  attributable matrix proves it. Keep result-dependent cases sequential.
- When TDD is not applicable, name the baseline and proportionate property,
  model, differential, fault, integration, hardware, benchmark, or other
  validation. Mark exploratory implementation as non-Candidate work with an
  explicit productionization boundary.
- Name applicable repository or project verification routes. Require exact
  commands only when their identity or flags are needed for safety,
  reproducibility, coverage, environment selection, or migration correctness.
- Allow the writer to select and report additional focused, non-destructive
  checks inside the Task Contract.
- Do not impose source-line or test-count quotas.

## Review context

Record a concise `Review context` section before the Review policy. Describe in
plain language:

- the artifact type and its purpose;
- its consumers and execution or interpretation model;
- behavior and quality characteristics that materially matter;
- realistic failures with material consequences;
- approved trade-offs and conditions that are non-problems by themselves;
- assumptions or reviewer perspectives that are inapplicable.

Base the context on approved decisions and repository evidence. Do not turn it
into a machine-readable schema or repeat command results that belong to later
verification. An approved non-problem may be reconsidered only with materially
new evidence of a concrete reachable failure or approved-contract violation.

## Review policy

Include a separate `Review policy` section in every plan. The policy controls
breadth, independence, and Acceptance; it references the Review context without
repeating it.

Use `adaptive` as the default for new planned work, or `deep` when approved
risk justifies broader applicable perspectives. `focused` belongs to eligible
lightweight work, not a way to remove independent planned specification and
quality gates. Never select breadth from file count or apparent diff size.

Apply these mode contracts:

- `adaptive`: independent `spec-reviewer` and
  `implementation-quality-reviewer` for each Task PR, including test quality;
  add only recorded required/triggered Task and integration perspectives.
- `deep`: the same independent pair plus every perspective applicable to the
  approved Task/integration surface and concrete risk. Each `risk-reviewer`
  invocation gets one perspective, exact trigger, authority, failure model,
  surface, expected evidence and stop condition. Separate perspectives remain
  independent invocations when needed. Use `design-alignment-reviewer` mainly
  for composed/shared-boundary authority, not routine local changes.

For every mode, name explicitly skipped perspectives and why they are
inapplicable. `Deep` means broad applicable coverage, not every configured
reviewer.

Record conditional `finding-integrator` use for overlapping/conflicting
findings, authority defects, scope-sensitive remedies or non-trivial
reproduction/origin. All-clean reports and a single clear, bounded finding with
no trigger need no integrator; the latter still requires evidence-based triage.
Ordinary required integration uses complete reports after reviewer completion.
An authority-defect claim pauses unstarted reviewers for priority integration;
a confirmed Design Doc defect returns to the engineer before queued review or
correction. A risk perspective alone does not trigger integration.

Record:

- **Mode and rationale**
- **Risk surfaces**
- **Per-task gate**
- **Integration required reviewers and reasons**, using `none` when task gates
  fully cover the feature
- **Integration conditional reviewers with exact triggers**
- **Explicitly skipped perspectives and reasons**
- **Residual risk**
- **Deterministic reviewer order under runtime admission**
- **Findings-only integration and Design Escalation priority**
- **Acceptance threshold**

Runtime admission does not change Review selection. Record the deterministic
reviewer order used when a selected spawn is temporarily rejected. Keep phase
gates explicit so implementation, verification, findings integration, triage,
and correction remain ordered while independent source reviewers may run after
fresh verifier `PASS`. Do not encode leases, grants, or thread arithmetic in the
plan.

Correction policy must retain prior head `H1`, create one bounded correction
commit to `H2`, rebuild the Verification Matrix and run fresh `H2` verification.
Keep policy coverage fixed, rerun finding-owning and affected reviewers, and
carry prior clean evidence only with explicit non-invalidation reasons.
Uncertainty requires rerun. Record that `review` owns the impact map, evidence
coverage and correction traversal rules; supply prior reports, exact delta,
current target and fresh matrix without copying those rules.

Use the same proportional Acceptance threshold in every mode. A finding survives
only when it applies to the artifact and consumer model, cites an approved
requirement, identifies concrete reachable evidence, states a material
consequence, and proposes a proportionate correction. `Should Improve` requires
a concrete maintainability consequence or measurable repeated cost.

Drop preference-only, speculative, second-order, artifact-inapplicable, optional
polish, and objections to approved decisions without new evidence. A proposed
state machine, schema, identity system, or other architectural mechanism is
`Escalate` unless it is necessary and proportionate to a proven in-scope
violation.

## Fix model allocations at plan approval

Read [model-allocation.md](references/model-allocation.md). Record defaults once
and only Task-specific overrides, while showing every Task's effective
allocation and required-quality/risk/cost rationale. The engineer confirms
these with this plan's approval; do not add a startup approval question or a
runtime promotion/fallback mechanism. Feature Lead uses session defaults and
is never assigned by this plan. Resolve native profile bindings and independent
Task root launch settings separately.

## Execution concurrency

Each Task Lead is the sole writer/root in an independent Herdr session and
owns native read/check-only leaves. Permit multiple active writers only for dependency-ready,
ownership-disjoint tasks in separate checkouts without conflicting shared
state. Require every named reviewer to have a resolvable profile or complete
equivalent role contract with enforceable approved allocation. Record ready-Task
and reviewer order. Native admission is local to each session; it does not
establish aggregate service capacity. Herdr startup failure or native rejection
retains pending work without duplicate writers, weakened gates or model changes.

## Quality

- Do not hide design decisions inside implementation steps.
- Do not invent requirements.
- Do not use placeholders such as "implement as needed."
- Do not require a Design Doc when the settled task does not need one.
- Do not treat Review context as a source of requirements or Review policy as a
  feature contract.
- Make every Task Contract directly extractable for handoff while retaining
  cross-task coverage and interface ownership in the complete plan.
- Keep logical dependencies distinct from PR base relationships and explain
  every deliberate fan-in linearization.
- Reference exact authority paths and approval evidence. Put applicable clauses
  in an extractable task handoff, but do not duplicate unrelated source prose
  that an agent can read directly when needed.
- Make destructive or external actions explicit approval gates.

Use [example-plan.md](example-plan.md) only when the output shape is unclear.

Return the complete plan to `agentic-engineering-workflow` for user approval. Do
not start implementation from this skill.
