---
name: create-plan
description: Decompose an approved design or otherwise settled scope into self-contained, independently verifiable implementation tasks. Use when the user asks for an implementation plan or approves transition from design discussion to planning.
---

# Create an implementation plan

Write a plan that a fresh implementer can execute without reconstructing
decisions from the conversation.

## Entry

Proceed when the coordinator supplies one of these entry conditions:

- the user approved a Design Doc;
- design discussion settled the scope and the user approved planning;
- the user explicitly requested decomposition of a clear scope.

Return unresolved architecture, public contracts, or scope to
`agentic-engineering-workflow` for design discussion.

## Investigate

Read the relevant design, current implementation, tests, repository guidance, and
recent history. Map files by responsibility before splitting tasks.

## Plan structure

Store the plan at `docs/plans/YYYY-MM-DD-<feature>.md` unless repository guidance
specifies another location.

Include:

- goal, architecture summary, technologies, working directory, branch, and
  observed baseline;
- fixed decisions and explicit non-goals;
- a Review context;
- a separate complete Review policy;
- tasks ordered by dependency;
- final verification, review iteration, and publication policy.

For each task include:

- why the task exists and whether behavior changes;
- discipline: TDD for behavior, existing green tests for refactoring, or an
  explicit content/configuration migration discipline;
- exact files created, modified, and tested;
- concrete steps, exact commands, and expected results;
- exact commit scope and message.

## Test planning

- Define tests by behavioral viewpoint.
- For behavior changes, specify the failing test and expected red result before
  implementation.
- Prefer unit tests for module or component behavior, including filesystem
  behavior.
- Use integration tests only for public-crate, multi-component, or real process
  journeys.
- Require Arrange, Act, Assert; DAMP setup; returned-result assertions; and
  relevant side-effect assertions.
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
- Make destructive or external actions explicit approval gates.

Use [example-plan.md](example-plan.md) only when the output shape is unclear.

Return the complete plan to `agentic-engineering-workflow` for user approval. Do
not start implementation from this skill.
