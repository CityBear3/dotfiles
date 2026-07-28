---
name: create-plan
description: Decompose an approved design or otherwise settled scope into self-contained, independently verifiable implementation tasks. Use when the user asks for an implementation plan or approves transition from design discussion to planning.
---

# Create an implementation plan

Write a plan that a fresh implementer can execute without reconstructing decisions from the conversation.

## Entry

Proceed when the coordinator supplies one of these entry conditions:

- the user approved a Design Doc;
- design discussion settled the scope and the user approved planning;
- the user explicitly requested decomposition of a clear scope.

Return unresolved architecture, public contracts, or scope to
`agentic-engineering-workflow` for design discussion.

## Investigate

Read the relevant design, current implementation, tests, repository guidance, and recent history. Map files by responsibility before splitting tasks.

## Plan structure

Store the plan at `docs/plans/YYYY-MM-DD-<feature>.md` unless repository guidance specifies another location.

Include:

- goal, architecture summary, technologies, working directory, branch, and observed baseline;
- fixed decisions and explicit non-goals;
- a complete Review policy;
- one exact per-task verification command;
- tasks ordered by dependency;
- final verification, review iteration, and publication policy.

For each task include:

- why the task exists;
- whether behavior changes;
- discipline: TDD for behavior, existing green tests for refactoring, or an explicit content/configuration migration discipline;
- exact files created, modified, and tested;
- concrete steps, commands, and expected results;
- an exact commit scope and message.

## Test planning

- Define tests by behavioral viewpoint.
- For behavior changes, specify the failing test and expected red result before implementation.
- Prefer unit tests for module or component behavior, including filesystem behavior.
- Use Cargo integration tests only for public-crate, multi-component, or real process journeys.
- Require Arrange, Act, Assert; DAMP setup; returned-result assertions; and relevant side-effect assertions.
- Do not impose source-line or test-count quotas.

## Review policy

Include a `Review policy` section in every plan. Use `adaptive` as the default for
planned work. Recommend `focused` or `deep` only when repository evidence,
approved decisions, and concrete risk surfaces justify it; never select a mode
from file count, changed-line count, or apparent diff size. Leave the lightweight
path, including its `focused` default, and cross-phase policy application to
`agentic-engineering-workflow`.

Resolve each mode's review contract from `agentic-engineering-workflow`. Record
the concrete policy without reproducing cross-phase routing:

- **Mode:** `focused`, `adaptive`, or `deep`, with an evidence-based rationale.
- **Risk surfaces:** identify applicable public API, persistence or migration,
  security or permission, concurrency, error or recovery, performance hot-path,
  cross-component responsibility, and test-double or fixture risks.
- **Per-task gate:** name the combined or independent reviewers required by the
  selected mode, including a resolvable profile or complete fallback prompt.
- **Final required reviewers:** name every reviewer that must run and why.
- **Final conditional reviewers:** name each trigger that adds a reviewer.
- **Explicitly skipped perspectives:** name each omission and its reason.
- **Residual risk:** state what the selected breadth does not cover. When the user
  selects a lighter policy, preserve the omitted perspectives and resulting risk.
- **Capacity:** record the configured and observed limit, queue order, and how the
  approved scope remains intact when capacity is lower.
- **Acceptance threshold:** accept only Must Fix or Should Improve findings that
  cite a concrete reachable behavior or contract violation, evidence, impact,
  and a specific correction. Reject preference-only, speculative, and
  already-decided objections without new evidence.

Keep model and reasoning-effort choices in reviewer profiles, not in the plan.

## Agent capacity

When execution may use subagents, identify one writer and read-only reviewers.
Keep work independently assignable and ensure planned concurrency fits the
Review policy's capacity field.
Require every named reviewer to have a resolvable profile or a complete fallback
prompt.

## Quality

- Do not hide design decisions inside implementation steps.
- Do not invent requirements.
- Do not use placeholders such as "implement as needed."
- Do not require a Design Doc when the settled task does not need one.
- Make destructive or external actions explicit approval gates.

Use [example-plan.md](example-plan.md) only when the output shape is unclear.

Return the complete plan to `agentic-engineering-workflow` for user approval. Do
not start implementation from this skill.
