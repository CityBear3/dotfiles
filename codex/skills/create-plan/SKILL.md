---
name: create-plan
description: Decompose an approved Feature Contract into self-contained, independently verifiable Task Contracts and an Implementation Plan. Use after the contract is separately approved and current.
---

# Create an implementation plan

Write a plan that a fresh implementer can execute without reconstructing design,
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
do not copy or mutate its meaning.

Include:

- goal, authorities and precedence, architecture summary, technologies, working
  directory, branch, and observed baseline;
- fixed decisions and explicit non-goals;
- shared interface contracts and their owners and consumers;
- complete Feature Contract coverage, including integration-only obligations;
- a Review context;
- a separate complete Review policy;
- Task Contracts ordered by dependency;
- re-entry impact and promotion reconciliation when either applies;
- final verification, review iteration, and publication policy.

For each Task Contract include:

- purpose and expected result;
- Feature Contract clauses it satisfies;
- responsibility and ownership boundaries;
- applicable shared interfaces and adjacent-task obligations;
- protected constraints and invariants;
- observable task-level verification obligations;
- dependencies;
- explicit non-goals;
- local decisions delegated to the implementation agent;
- discipline: TDD for production behavior, an existing green baseline for
  refactoring, or an explicit content/configuration migration discipline;
- a responsibility-scoped commit intent and whether the plan or writer selects
  its message;
- contractually significant files, signatures, ordering, commands, exact commit
  paths, or fixed commit message only when their identity is part of correctness.

## Choose contract detail, not procedure

Do not require exhaustive files, function-by-function steps, edit order, helper
structure, local algorithms, or exact commands by default. The writer may
discover private files inside its approved responsibility and must report the
actual changed files.

Fix exact detail when it defines a public or shared interface, writer ownership,
generated or manifest mapping, migration or compatibility sequence, safety
boundary, reproducible environment, authoritative coverage, or another
observable correctness condition. An unexpected private file is not a deviation;
a new owner, public seam, shared interface, invariant, or contract meaning is.

## Test planning

- Define observable verification by behavioral viewpoint.
- For behavior changes, specify the failing test and expected red result before
  implementation.
- Prefer unit tests for module or component behavior, including filesystem
  behavior.
- Use integration tests only for public-crate, multi-component, or real process
  journeys.
- Require Arrange, Act, Assert; DAMP setup; returned-result assertions; and
  relevant side-effect assertions.
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
breadth, independence, capacity, and Acceptance; it references the Review context
without repeating it.

Use `adaptive` as the default for planned work. Recommend `focused` or `deep`
only when repository evidence, approved decisions, and concrete risk surfaces
justify it. Never select a mode from file count, changed-line count, or apparent
diff size.

Apply these mode contracts:

- `focused`: one combined specification-and-quality per-task gate; final
  `code-reviewer`; `test-coverage-reviewer` when behavior or tests changed; plus
  only additional perspectives justified by recorded risk.
- `adaptive`: independent specification and quality per-task gates; final
  standard and adversarial perspectives selected for recorded risk.
- `deep`: independent specification and quality per-task gates; every final
  perspective applicable to the artifact and observed risks; adversarial
  integration whenever any adversarial perspective runs.

For every mode, name explicitly skipped perspectives and why they are
inapplicable. `Deep` means broad applicable coverage, not every configured
reviewer.

Record:

- **Mode and rationale**
- **Risk surfaces**
- **Per-task gate**
- **Final required reviewers and reasons**
- **Final conditional reviewers with exact triggers**
- **Explicitly skipped perspectives and reasons**
- **Residual risk**
- **Capacity and deterministic queue order**
- **Acceptance threshold**

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

Keep model and reasoning-effort choices in reviewer profiles, not in the plan.

## Agent capacity

When execution may use subagents, identify one writer and read-only reviewers.
Require every named reviewer to have a resolvable profile or complete fallback
prompt. Queue selected reviewers when capacity is lower; never reduce approved
scope or independence silently.

## Quality

- Do not hide design decisions inside implementation steps.
- Do not invent requirements.
- Do not use placeholders such as "implement as needed."
- Do not require a Design Doc when the settled task does not need one.
- Do not treat Review context as a source of requirements or Review policy as a
  feature contract.
- Make every Task Contract directly extractable for handoff while retaining
  cross-task coverage and interface ownership in the complete plan.
- Reference exact authority paths and approval evidence. Put applicable clauses
  in an extractable task handoff, but do not duplicate unrelated source prose
  that an agent can read directly when needed.
- Make destructive or external actions explicit approval gates.

Use [example-plan.md](example-plan.md) only when the output shape is unclear.

Return the complete plan to `agentic-engineering-workflow` for user approval. Do
not start implementation from this skill.
