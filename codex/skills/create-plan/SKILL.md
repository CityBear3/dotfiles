---
name: create-plan
description: Decompose an approved design or otherwise settled scope into self-contained, independently verifiable implementation tasks. Use when the user asks for an implementation plan or approves transition from design discussion to planning.
---

# Create an implementation plan

Write a plan that a fresh implementer can execute without reconstructing decisions from the conversation.

## Entry

Proceed when one of these is true:

- the user approved a Design Doc;
- design discussion settled the scope and the user approved planning;
- the user explicitly requested decomposition of a clear scope.

Return to design discussion if architecture, public contracts, or scope remain undecided.

## Investigate

Read the relevant design, current implementation, tests, repository guidance, and recent history. Map files by responsibility before splitting tasks.

## Plan structure

Store the plan at `docs/plans/YYYY-MM-DD-<feature>.md` unless repository guidance specifies another location.

Include:

- goal, architecture summary, technologies, working directory, branch, and observed baseline;
- fixed decisions and explicit non-goals;
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

## Agent capacity

When execution may use subagents, identify one writer and read-only reviewers. Keep work independently assignable and ensure planned concurrency fits the configured and observed capacity. Additional reviewers requested by the plan must have either a resolvable profile or a complete fallback prompt.

## Quality

- Do not hide design decisions inside implementation steps.
- Do not invent requirements.
- Do not use placeholders such as "implement as needed."
- Do not require a Design Doc when the settled task does not need one.
- Make destructive or external actions explicit approval gates.

Use [example-plan.md](example-plan.md) only when the output shape is unclear.

Present the complete plan for user approval. Do not start implementation until it is approved.
