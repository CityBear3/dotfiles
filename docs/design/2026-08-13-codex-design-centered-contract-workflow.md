# [Design Doc] Codex Design-Centered Contract Workflow

- Owner: Repository owner
- Drafted by: Codex from owner-approved design decisions
- Date: 2026-08-13
- Status: Approved
- Extends: `docs/design/2026-07-29-codex-agentic-engineering-workflow.md`

## Context and scope

The current Codex engineering workflow already separates design discussion,
Design Doc drafting, planning, task execution, verification, review, and branch
completion. It preserves user ownership of architecture and material trade-offs,
but its implementation handoffs are centered on expected behavior, exact files,
concrete steps, and exact verification commands. The workflow does not yet define
one durable feature-level contract that connects an approved design to task
decomposition, or a task-level contract that gives an implementation agent
freedom over local implementation while preserving design intent.

This gap matters as implementation agents become more autonomous. An agent can
only run implementation, verification, and bounded correction without repeated
human intervention when the goal, responsibility boundaries, interfaces,
protected constraints, and observable completion conditions are explicit. Those
contracts must also survive context compaction, a fresh agent session, and
handoff to independent reviewers without relying on reconstruction from chat
history.

This design extends the existing workflow with a hierarchy of Design Doc,
Feature Contract, and Task Contract artifacts. It changes the planned path so
that these artifacts are approved at distinct abstraction levels before agents
enter an autonomous implementation loop. It also replaces unconditional
file-by-file procedures and exact-command planning with responsibility-centered
contracts and observable verification obligations, while retaining exact files,
ordering, signatures, and commands when their identity is part of correctness.

### Goals

- Establish shared human-Agent understanding before implementation through an
  explicit Feature Contract.
- Separate long-lived architectural decisions from feature-scoped behavioral
  commitments and task-scoped implementation obligations.
- Derive task boundaries from an approved Feature Contract rather than using
  tasks to discover or silently complete the design.
- Let implementation agents choose local files, helpers, algorithms, and focused
  checks when those choices stay within the approved contracts.
- Make contract-changing discoveries return to the owning design approval gate,
  while allowing autonomous implementation, verification, and correction inside
  the contract.
- Preserve exact approved contracts across context compaction, fresh sessions,
  and agent handoffs through workspace files while work remains active.
- Keep interface detail at the earliest layer where another consumer, task, or
  architectural boundary depends on it.
- Make verification obligations describe observable proof of correctness and use
  project skills or repository guidance for reusable verification procedures.
- Preserve the existing Review context, Review policy, evidence, safety, and
  branch-completion boundaries while grounding them in the new contracts.

### Non-goals

- Require a Design Doc for every change.
- Create a new `feature-contract` phase skill when existing design skills can own
  contract construction and the coordinator can own its approval state.
- Create one file per Task Contract.
- Prescribe private helpers, internal function signatures, file-edit order, or
  local syntax when no approved boundary depends on them.
- Remove TDD, task verification, independent review, exact Git range evidence,
  bounded correction, or final verification.
- Implement parallel plan-task scheduling, multiple concurrent writers, or
  per-task worktrees in this change.
- Expand sandbox permissions, external-write authority, publication authority,
  or destructive-action authority.
- Define a machine-readable contract schema or require a digest protocol for
  natural-language skill handoffs.
- Rewrite historical Design Docs or implementation plans to the new format.

## Overview

The workflow uses three design layers and one derived review view:

| Layer | Purpose | Approval and storage |
| --- | --- | --- |
| Design Doc | Durable architecture, component and context boundaries, public contracts, state transitions, schemas, and significant trade-offs | Required only when warranted; separately approved and stored under `docs/design/` |
| Feature Contract | The current feature's goal, observable behavior, applied design sources, responsibilities, protected constraints, and verification obligations | Required for every implementation; separately approved on the planned path and kept in the ignored feature plan directory while work is active |
| Task Contract | The projection of the Feature Contract onto one independently implementable and verifiable task | Approved together with all other Task Contracts as part of the workspace-only Implementation Plan |
| Review context | A concise artifact- and consumer-aware interpretation for reviewers | Derived from the approved contracts; not a competing source of requirements |

The planned path becomes:

```text
request and repository investigation
              |
              v
       design discussion
              |
              v
purpose and initial feature boundary are identifiable
              |
              v
     establish feature workspace
              |
              +------------------------------+
              | Design Doc warranted?        |
              +---------------+--------------+
                              |
                 yes          |          no
                  v           |           v
        draft Design Doc      |   use approved decision record
                  |           |           |
        approve Design Doc    |           |
                  +-----------+-----------+
                              |
                              v
                  draft Feature Contract
                              |
                   approve Feature Contract
                              |
                              v
                  decompose into tasks
                              |
          derive Task Contracts and integration proof
                              |
            approve complete Implementation Plan
                              |
                              v
              autonomous task execution loop
                              |
                              v
             feature-level verification and review
```

The approval gates are intentionally separate. A Design Doc approval establishes
the architecture. A Feature Contract approval establishes the shared definition
of feature success. Implementation Plan approval establishes the complete task
decomposition, Task Contracts, execution dependencies, and review policy.

## Detailed design

### Responsibility ownership

`agentic-engineering-workflow` owns:

- whether the request uses the lightweight or planned path;
- the required presence, source, approval state, and currentness of the Feature
  Contract;
- the ordered approval gates;
- the transition back to design when a contract meaning must change;
- propagation of Feature and Task Contract evidence into final verification and
  review.

`design-discussion` owns Feature Contract construction when no Design Doc is
warranted. It builds the contract from repository evidence and the owner-approved
decision record, and returns any material ambiguity instead of filling it in.

`design-doc` owns Feature Contract construction after a Design Doc is approved or
when an existing Design Doc is the applicable design source. Design Doc and
Feature Contract remain separate approval results. Failure to derive a complete
Feature Contract exposes a Design Doc ambiguity and returns to design rather than
advancing to planning.

`create-plan` owns decomposition of the approved Feature Contract into a complete
set of Task Contracts, task dependencies, shared interface ownership, execution
metadata, integration verification, Review context, and Review policy. It does
not introduce new architecture or feature semantics.

`execute-plan` owns exact propagation of the approved Feature Contract and the
applicable Task Contract to each task. `execute-task` owns autonomous work inside
that Task Contract and must stop when satisfying it requires changing a protected
contract.

`verify` checks the integrated result against Feature Contract obligations in
addition to task and project checks. `review` checks implementation quality and
alignment with both Feature and Task Contracts, using the derived Review context
and approved Review policy.

### Feature Contract

Every implementation has one Feature Contract. On the planned path it is a
separately presented and approved artifact. On the lightweight path Codex derives
it from the explicit request and repository evidence, and the same contract also
serves as the single Task Contract.

Every Feature Contract contains the following fixed core:

1. **Context and goal** — why the change is needed and what outcome constitutes
   success.
2. **Scope and non-goals** — affected consumers, systems, and behavior, plus what
   the change intentionally excludes.
3. **Design sources and decisions** — applicable Design Docs, ADRs, existing
   contracts, approved additional decisions, and precedence when sources differ.
4. **Behavioral contract** — externally observable success, preserved behavior,
   compatibility, and material failure behavior.
5. **Responsibilities and interfaces** — ownership of behavior and state,
   affected boundaries, and important unchanged boundaries.
6. **Protected constraints** — invariants and security, data integrity,
   performance, reliability, or other properties outside implementation-agent
   discretion.
7. **Verification obligations** — observations that prove the feature contract,
   including relevant boundaries, integration journeys, and failure behavior.
8. **Assumptions and approved deferrals** — evidence-backed assumptions and only
   those deferrals explicitly accepted by the owner.

The contract adds only applicable conditional sections, such as state
transitions, API or event semantics, schema and data lifecycle, errors and
recovery, concurrency and idempotency, authorization and confidentiality,
performance targets, migration coexistence, rollback, or cross-repository
ownership.

A missing applicable section is not filled with a placeholder. If the answer can
change architecture, responsibility, public contract, schema, error behavior,
scope, or a material trade-off, the workflow returns to design discussion.

### Interface placement and language-level contracts

Interface information is placed at the earliest design layer where another
consumer or task depends on it. The rule is semantic ownership, not syntactic
size.

- A context boundary, public API, durable component contract, or architecturally
  significant interface belongs in the Design Doc, including an exact signature
  when consumers depend on it.
- Feature-specific operations, input and output meaning, error semantics,
  compatibility, and invariants belong in the Feature Contract.
- An interface shared by multiple implementation tasks is recorded once as a
  shared interface contract in the Implementation Plan, with one owner and named
  implementers and consumers. Each Task Contract references it.
- A private helper interface or local abstraction that does not alter an approved
  responsibility or observable result remains an implementation choice.

For Rust, a trait is therefore not automatically high-level design or private
implementation detail. A trait's responsibility, dependency direction, and
consumer guarantees appear in the Design Doc or Feature Contract when they are
architecturally significant. Its exact Rust signature is fixed before dependent
tasks begin when details such as `async`, `Send + Sync`, ownership, absence
representation, associated types, or the error type affect consumers. A private
trait introduced only to organize one task remains inside the agent's discretion.

### Task decomposition and Task Contracts

Task decomposition occurs only after Feature Contract approval. `create-plan`
maps repository responsibilities and derives tasks that are independently
implementable and verifiable, subject to explicit dependencies. Task boundaries
are evaluated by:

- responsibility and state ownership;
- dependency direction and shared interfaces;
- ability to observe completion without relying on an unfinished unrelated task;
- writer ownership and the risk of overlapping edits;
- integration obligations that cannot be proven by a single task.

Difficulty decomposing a feature is design feedback. A simple execution
dependency is recorded in the plan. Ambiguous responsibility, conflicting state
ownership, missing error semantics, or an undefined shared interface returns to
the Feature Contract or Design Doc approval gate.

Each Task Contract includes:

- task purpose and expected result;
- Feature Contract clauses it satisfies;
- responsibility and ownership boundaries;
- applicable shared interfaces and adjacent-task obligations;
- protected constraints and invariants;
- observable task-level verification obligations;
- dependencies;
- explicit non-goals;
- local decisions delegated to the implementation agent.

The complete Task Contract set must cover every Feature Contract obligation.
`create-plan` records deliberate overlap, task-external integration obligations,
and any Feature Contract clause that is verified only after aggregation. It
rejects unexplained gaps, conflicting ownership, or duplicated interface
authority.

### Implementation Plan and physical artifacts

Feature Contract and Implementation Plan are different artifacts because their
approval and mutation lifecycles differ. During active planned work, the feature
workspace uses:

```text
docs/plans/YYYY-MM-DD-<feature>/
├── feature-contract.md
└── implementation-plan.md
```

`feature-contract.md` contains the separately approved Feature Contract and
references any Design Doc source. Its meaning remains fixed while the plan is
created and executed.

`implementation-plan.md` is the orchestration artifact. It contains the complete
Task Contract set and the execution-level information that is not itself feature
design: dependencies, discipline, workspace and ownership information,
contractually necessary files or ordering, reusable verification routes, Review
context, Review policy, integration verification, commit scope, and final
completion policy.

Both files are ignored, workspace-only execution artifacts. They are not
force-added, staged, or committed by default. Their purpose is to survive
conversation compaction, fresh agent sessions, and task handoffs while the
feature is active—not to become permanent repository documentation. After the
final verification and review evidence has been derived from them,
`finish-branch` removes them before publication or final handoff. The repository
retains a Design Doc only when the architectural decisions are independently
worth preserving. Explicit user-requested archival is the exception.

Task Contracts remain distinct normative sections inside the Implementation
Plan; they are not separate files and are not synonymous with the entire plan.
This lets the plan validate cross-task coverage and ownership in one place while
keeping each task's contract directly extractable for handoff.

The workflow establishes a suitable feature branch or worktree after initial
investigation makes the purpose and feature boundary identifiable and before it
writes the first recoverable planned-path artifact. Existing user and repository
workspace rules still apply, and `create-workspace` retains its branch-change
approval boundary. An already suitable workspace requires no additional
transition.

Drafts and approved execution artifacts are written in that workspace so a
fresh session can recover them without chat history while work remains active.
The coordinator records which exact artifact is approved and does not treat
later edits as approved automatically. An implementation plan may reference but
must not silently rewrite an approved Feature Contract.

### Contract detail versus implementation procedure

Plans specify responsibility and ownership boundaries rather than unconditional
exact file lists. Exact files are included only when file identity is part of a
public or shared contract, a task ownership boundary, a generated or manifest
mapping, a migration, or another correctness requirement. The implementation
agent may discover and edit additional private implementation files within its
approved ownership boundary and must report the actual changed files.

Plans do not normally prescribe edit order, helper structure, local algorithm,
or function-by-function steps. They specify exact ordering when an intermediate
state, compatibility window, migration sequence, rollback boundary, code
generation step, or other ordering guarantee is part of correctness.

The implementation agent must stop rather than reinterpret an unexpected file or
responsibility overlap. A newly discovered private file inside the existing
boundary is not a plan deviation by itself; a new component owner, shared
interface, public seam, protected constraint, or contract meaning is.

### Verification obligations and autonomous feedback

Feature and Task Contracts define what must be observed, not an unconditional
list of shell commands. Reusable procedures for starting an application,
operating a browser, exercising an API, or running standard project checks belong
in repository guidance or project-specific skills. The plan names the applicable
verification route and expected observations.

Exact commands are fixed when their identity or flags are required for safety,
reproducibility, coverage, environment selection, migration correctness, or an
authoritative repository contract. Otherwise the implementation agent selects
the concrete commands from current repository evidence and the named project
skills. It may add focused checks when they remain in scope and non-destructive.

The task loop remains bounded:

1. inspect the current task base and relevant implementation;
2. implement within the Task Contract using the declared discipline;
3. obtain fresh feedback through applicable tests, static checks, and real
   behavior observations;
4. inspect the resulting diff and contract coverage;
5. correct concrete in-scope failures;
6. stop on repeated non-progress, an operational blocker, or a required contract
   change;
7. pass current evidence to the policy-selected independent task gate.

Global verification uses the Feature Contract to prove the integrated result.
Passing each Task Contract is necessary but not sufficient when a Feature
Contract obligation exists only at an integration boundary.

### Contract changes and approval invalidation

The implementation agent may clarify wording or add evidence only when the
meaning of the approved contract is unchanged. A change to the goal, scope,
responsibility boundary, interface semantics, invariant, material failure
behavior, compatibility promise, or verification obligation invalidates the
dependent approval.

On the planned path, a Feature Contract meaning change returns to the Feature
Contract approval gate and, when its design source is insufficient or changed,
to Design Doc or design discussion approval first. After reapproval,
`create-plan` revalidates all Task Contracts and the complete plan. A Task
Contract meaning change that stays within the Feature Contract still invalidates
Implementation Plan approval and requires the plan to be re-presented.

On the lightweight path, any material contract change disqualifies the route and
promotes the work to the planned path. The workflow preserves observed work and
does not silently broaden authority.

### Review context and review alignment

Review context remains separate from the contracts and Review policy. It is a
concise reviewer-facing projection of the approved Feature Contract, Task
Contracts, repository evidence, and approved non-problems. It does not introduce
requirements or replace the contract sources.

Per-task specification review checks the exact Task Contract and applicable
Feature Contract clauses. Final design-alignment and scope review check the
integrated range against the approved Feature Contract, applicable Design Doc,
complete Task Contract coverage, and approved Implementation Plan. Review policy
continues to select breadth, independence, capacity, and the common Acceptance
threshold.

### Lightweight path

The lightweight path still requires a Feature Contract, but Codex derives it
from the explicit request and repository evidence without creating a contract
file or adding a separate approval gate. Because the route is one coherent task,
the Feature Contract and Task Contract are the same handoff.

This exception is bounded by the existing eligibility criteria. If the task can
no longer complete as one coherent change, loses a material contract from
context, requires durable cross-session coordination, or exposes a user-owned
decision, it returns to design discussion and the planned path. It does not
create an improvised second task under lightweight authority.

## Cross-cutting concerns

### Compaction and fresh-session recovery

Conversation summaries and in-memory coordinator state are not the sole source
of an approved planned-path contract. The feature workspace contains the current
Design Doc when applicable, the Feature Contract, and the Implementation Plan.
A resumed coordinator re-reads these files, repository status, and the recorded
approval evidence before advancing. If it cannot establish which content was
approved or finds a material edit after approval, it returns to the relevant
gate instead of inferring approval.

This recovery guarantee applies while planned work is active. After completion,
the ignored Feature Contract and Implementation Plan are retired; durable design
decisions remain in the Design Doc, and implemented behavior and verification
remain in code, tests, Git history, and completion or pull-request evidence.

### Duplication and drift

The Design Doc remains the source of durable architectural decisions. During
active work, the Feature Contract references those decisions and adds only
feature-scoped application and success criteria. The Implementation Plan
references the Feature Contract and contains Task Contracts plus orchestration.
Review context is derived and concise. Each layer avoids copying prose that can
be referenced without losing the meaning needed by its consumer.

### Existing plans and compatibility

Previously tracked historical plans are not migrated automatically. New planned
work uses the ignored workspace directory format and retires it after completion.
A currently executing approved legacy plan may finish under its approved
contract unless a user explicitly asks to migrate it or a material ambiguity
prevents safe continuation.

### Sensitive information

Contracts and plans must not persist credentials, personal data, private tokens,
or unnecessary environment-specific values. They record stable semantics and
redacted evidence references rather than sensitive runtime data.

### Failure behavior

The workflow returns `Escalate` when a missing or changed contract requires a
user-owned decision, `BLOCKED` when the artifact, workspace, permission, or
operational state cannot be established safely, and a plan deviation when task
decomposition or implementation contradicts the approved artifacts. It never
uses a successful local command or an agent's self-review as evidence that a
missing contract is satisfied.

## Alternatives considered

### Keep one undifferentiated Design Contract

Rejected because architectural decisions, feature success, and task obligations
have different consumers, abstraction levels, approval gates, and lifecycles. A
single contract would either be too broad for task handoff or too detailed for
durable design.

### Treat Task Contracts as standalone detailed-design files

Rejected as the default because complete cross-task coverage, ownership overlap,
shared interfaces, dependencies, and integration proof are best reviewed as one
Implementation Plan. Separate task files would add synchronization and approval
cost without improving the current sequential single-writer execution model.

### Combine Design Doc and Feature Contract into one approval package

Rejected because a large architecture artifact can hide whether the human and
Agent share the same concrete definition of feature success. Separate approval
turns Feature Contract derivation into a check that the Design Doc is actionable.

### Embed Feature Contract in the Implementation Plan

Rejected because the Feature Contract is approved before task decomposition and
must remain stable while the plan is created and revised. Appending Task
Contracts to the same file would mutate the approved artifact and require
section-level approval identity. A separate file makes the lifecycle boundary
structural and improves compaction recovery.

### Keep the approved Feature Contract only in coordinator memory

Rejected because compaction, fresh sessions, and independent agents must recover
the exact contract without reconstructing it from conversation history.

### Require exact files, implementation steps, and verification commands for every task

Rejected because it converts detailed design into prescribed procedure and
unnecessarily removes implementation-agent judgment. Exact detail remains
required when it is itself part of an interface, ownership, ordering, safety,
reproducibility, or coverage contract.

### Delegate all verification choices to the implementation agent

Rejected because verification quality and completion evidence would vary without
an approved observation contract. The chosen design fixes observable obligations
and reusable routes while allowing the agent to select appropriate concrete and
additional focused checks.

### Introduce a dedicated Feature Contract skill

Rejected for now because Feature Contract construction belongs to the current
design source: `design-discussion` without a Design Doc and `design-doc` with one.
The coordinator owns approval and transition state, while `create-plan` consumes
the result. A new phase skill would duplicate these responsibility boundaries.
