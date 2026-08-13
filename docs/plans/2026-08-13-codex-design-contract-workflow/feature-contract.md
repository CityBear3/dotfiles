# Feature Contract: Codex Design-Centered Contract Workflow

- Owner: Repository owner
- Drafted by: Codex from the approved Design Doc and repository evidence
- Date: 2026-08-13
- Status: Approved

## Context and goal

The Codex engineering workflow must establish shared understanding of a change's
goal, observable behavior, responsibility boundaries, protected constraints,
and verification obligations before implementation agents enter an autonomous
execution loop.

Success means the installed Codex skills consistently route planned work through
separately approved Design Doc, Feature Contract, and Implementation Plan gates;
derive Task Contracts only after Feature Contract approval; preserve those
contracts across compaction and fresh sessions; and let agents choose local
implementation and verification details only inside the approved boundaries.

## Scope and non-goals

### In scope

- Planned- and lightweight-path classification and transition rules.
- Early feature-workspace establishment before planned-path design artifacts are
  persisted.
- Feature Contract construction, approval, storage, currentness, and invalidation.
- Task decomposition and Task Contract construction inside the Implementation
  Plan.
- Contract-centered handoff to task implementation, verification, and review.
- Responsibility-based file ownership and conditional placement of exact files,
  interface signatures, ordering, and verification commands.
- Recovery of the exact planned-path artifacts after context compaction,
  interruption, or a fresh session.
- Forward-compatible handling of historical plans created before this contract.

### Non-goals

- Parallel plan-task scheduling, multiple concurrent writers, or per-task
  worktrees.
- A machine-readable Feature or Task Contract schema.
- A new dedicated Feature Contract phase skill.
- Migration or rewriting of historical Design Docs and plans.
- Changes to sandbox permissions, external writes, publication, destructive
  actions, or branch-disposition authority.
- Changes to reviewer models, reasoning effort, or the common Acceptance
  threshold.
- Prescribing private helper structure, local syntax, or function-by-function
  implementation procedures.

## Design sources and decisions

### Authoritative sources

1. `docs/design/2026-08-13-codex-design-centered-contract-workflow.md`
2. `docs/design/2026-07-29-codex-agentic-engineering-workflow.md`
3. Repository and personal `AGENTS.md` guidance applicable to this workspace.

The 2026-08-13 Design Doc overrides the older workflow design only for contract
layers, approval order, artifact storage, task decomposition, interface-detail
placement, and autonomous verification detail. Existing safety, evidence,
single-writer, check-only review, and branch-completion decisions remain in
force.

### Approved decisions

- `Design Contract` is a common concept expressed through Feature and Task
  Contracts rather than one undifferentiated artifact.
- Design Doc, Feature Contract, and complete Implementation Plan have distinct
  approval gates.
- Planned-path Feature Contract and Implementation Plan are separate files under
  one feature plan directory.
- Task Contracts are normative sections inside the Implementation Plan and are
  approved as a complete set, not one at a time.
- Feature Contract precedes task decomposition. Decomposition that exposes a
  design gap returns to design instead of completing the design inside the plan.
- A suitable feature workspace is established before the first planned-path
  design artifact is written.
- Interface detail is recorded at the earliest layer where another consumer or
  task depends on it. Language constructs such as Rust traits are classified by
  semantics and consumers rather than syntax or size.
- Exact files, signatures, ordering, and commands are conditional contract
  details rather than universal plan requirements.
- Verification contracts state observable obligations. Reusable procedures come
  from project skills or repository guidance, and agents may select additional
  focused checks inside scope.
- Contract-preserving local decisions are autonomous. Contract-meaning changes
  invalidate the dependent approval and return to its owning gate.

## Behavioral contract

### Planned path

For a planned change, the coordinator must enforce this order:

1. inspect the repository and settle material design decisions;
2. once the purpose and initial feature boundary are identifiable, establish or
   confirm a suitable feature workspace;
3. create and separately approve a Design Doc when warranted;
4. construct the Feature Contract from the approved Design Doc or, when no
   Design Doc is warranted, from the approved decision record;
5. persist and separately approve the Feature Contract;
6. create the Implementation Plan by decomposing the approved Feature Contract
   into a complete Task Contract set;
7. separately approve the complete Implementation Plan, including Review context
   and Review policy;
8. execute tasks within their approved contracts;
9. verify and review the integrated result against the Feature Contract.

The workflow must not transition directly from Design Doc approval or settled
design discussion to `create-plan` without an approved, current Feature Contract.

### Feature Contract construction

Every Feature Contract must contain:

- context and goal;
- scope and non-goals;
- design sources and approved decisions;
- externally observable behavior and material failure behavior;
- responsibilities and interfaces;
- protected constraints;
- verification obligations;
- assumptions and explicitly approved deferrals.

Only applicable conditional concerns are added. A missing material decision is
returned to design discussion. A Design Doc that cannot support a complete
Feature Contract is reopened rather than supplemented silently during planning.

### Task decomposition and Implementation Plan

`create-plan` must derive tasks only from an approved, unchanged Feature
Contract. It must:

- map tasks by responsibility, state ownership, dependency direction, shared
  interfaces, and independently observable completion;
- give each task a Task Contract covering purpose, Feature Contract clauses,
  ownership, shared interfaces, constraints, verification obligations,
  dependencies, non-goals, and delegated local decisions;
- record shared interfaces once with one owner and named implementers and
  consumers;
- show that the complete Task Contract set covers every Feature Contract
  obligation;
- identify integration-only obligations;
- reject unexplained gaps, conflicting ownership, duplicated authority, and
  design decisions invented during decomposition.

The Implementation Plan is an orchestration artifact containing Task Contracts,
dependencies, execution metadata, Review context, Review policy, integration
verification, and completion policy. Task Contracts are not separate files and
are not synonymous with the whole plan.

### Autonomous task execution

Each task handoff must include the approved Feature Contract, the applicable
Task Contract, relevant shared interfaces, the derived Review context, active
Review policy, task base and workspace evidence, declared discipline, and the
verification route and obligations.

Inside the Task Contract, the writer may choose private files, helpers, local
types and interfaces, algorithms, edit order, concrete standard commands, and
additional focused checks. The writer must preserve unrelated changes and report
actual changed files and observed verification evidence.

The writer must stop and return the exact gap when implementation needs a new or
changed goal, scope, responsibility owner, shared or public interface semantic,
invariant, material failure behavior, compatibility promise, verification
obligation, authority, or Review policy.

### Integrated verification and review

Task acceptance proves the current Task Contract but does not prove an
integration-only Feature Contract obligation. Final verification must inspect
the current aggregate head and demonstrate all applicable Feature Contract
observations. Final review must check contract alignment in addition to local
implementation quality.

Review context remains a concise projection for reviewer interpretation. It must
be derived from, and must not add to or replace, the approved contracts. Review
policy continues to control breadth, independence, capacity, and Acceptance.

### Lightweight path

A lightweight implementation still has a Feature Contract, automatically
derived from the explicit request and repository evidence. Because the route is
one coherent task, the same handoff is also its Task Contract. It creates no
contract file and adds no separate contract approval gate.

If the task ceases to be one coherent change, needs durable cross-session
coordination, loses a material contract from recoverable context, or exposes a
user-owned decision, the workflow preserves observed state and promotes the work
to the planned path.

## Responsibilities and interfaces

### Skill ownership

- `agentic-engineering-workflow`: path selection, workspace transition timing,
  contract presence and approval state, cross-phase transitions, and contract
  invalidation routing.
- `create-workspace`: suitable branch or worktree establishment before durable
  planned-path artifacts, retaining its explicit branch-change approval.
- `design-discussion`: decision record and Feature Contract construction when no
  Design Doc is warranted.
- `design-doc`: Design Doc drafting and, only after Design Doc approval, Feature
  Contract construction from that source.
- `create-plan`: Feature Contract decomposition, Task Contract set, shared
  interface ownership, coverage, integration proof, Review context, and Review
  policy.
- `execute-plan`: dependency order and exact contract propagation to tasks.
- `execute-task`: autonomous implementation and bounded feedback loop inside one
  Task Contract.
- `verify`: fresh feature-level proof against the Feature Contract.
- `review`: read-only Feature and Task Contract alignment review under the
  approved Review policy.

### Artifact interface

Planned work uses:

```text
docs/plans/YYYY-MM-DD-<feature>/
├── feature-contract.md
└── implementation-plan.md
```

The Feature Contract references its Design Doc when applicable. The
Implementation Plan references the approved Feature Contract and must not copy
or mutate its meaning. A resumed session re-reads both files and repository state
before advancing.

### Interface-detail rule

An interface's semantic contract and exact form are recorded at the earliest
layer needed by an independent consumer, task, or durable architectural
boundary. Shared interfaces have one owner. Private interfaces with no
contractual consumer remain implementation choices.

## Protected constraints

- The repository owner retains architecture, scope, public-contract, algorithm,
  and material trade-off decisions.
- No unapproved Design Doc, Feature Contract, or Implementation Plan may
  authorize dependent planned-path work.
- A material edit invalidates the dependent approval; approval is never inferred
  from a stale version or conversation summary.
- Existing read-only verification, review, and review-triage boundaries remain.
- Existing one-writer, exact task-base/head/range, unrelated-change preservation,
  and fresh-evidence requirements remain.
- Existing external-write, publication, destructive-action, merge, and branch
  disposition gates remain.
- Runtime Codex assets must not depend on `claude/` assets.
- Historical plans remain valid historical artifacts and are not rewritten by
  this feature.
- Contract and plan files must not contain credentials, personal data, private
  tokens, or unnecessary sensitive runtime values.

## Verification obligations

Completion must demonstrate the following observations:

1. The planned route in the coordinator contains the early workspace,
   Design-Doc-when-warranted, Feature Contract, and Implementation Plan gates in
   the approved order.
2. Design discussion and Design Doc flows construct Feature Contracts from their
   respective approved sources without collapsing their approval gates.
3. New plan structure persists separate Feature Contract and Implementation Plan
   files and keeps Task Contracts inside the latter.
4. Planning derives Task Contracts only after Feature Contract approval, checks
   full clause coverage and interface ownership, and returns design gaps rather
   than inventing decisions.
5. Task handoffs and task acceptance use Feature and Task Contracts while
   preserving the existing writer, Git evidence, verification, review, and
   correction guarantees.
6. Plans require responsibility and observable-result detail; exact files,
   procedures, signatures, ordering, and commands are required only when they are
   contractually significant.
7. Verification and review receive the approved contract evidence and check the
   integrated current head against Feature Contract obligations.
8. Lightweight work derives a single in-memory Feature/Task Contract and returns
   to the planned path on material change or unrecoverable long-lived context.
9. A fresh session can reconstruct planned-path design and task intent from the
   workspace files without relying on the original chat transcript.
10. Existing installer inventory and regression checks continue to accept the
    modified skill assets, and changed Markdown has no whitespace or sensitive
    data violation.

Project-standard installer tests, repository diff checks, semantic route
inspection, and the applicable review policy are the expected verification
routes. The Implementation Plan will select concrete commands where their exact
form is required and leave additional contract-preserving focused checks to the
implementer.

## Assumptions and approved deferrals

- Codex skills and plans are natural-language contracts interpreted by agents;
  no machine-readable schema is required.
- The existing installer generically inventories skill directories, so new
  installer production behavior is not expected unless implementation evidence
  proves otherwise.
- Existing Review modes and Acceptance semantics remain applicable.
- Parallel task scheduling, task worktrees, multiple writers, and broader
  approval automation are explicitly deferred.
- Improvements learned from later autonomous-loop operation may update project
  skills or guidance through a separately approved change.

## Failure and recovery behavior

- Missing or materially stale contract content is `Escalate` when it requires a
  user-owned decision or approval and `BLOCKED` when the required artifact or
  operational state cannot be established.
- A decomposition or implementation discovery that contradicts an approved
  contract is returned as a design or plan deviation with evidence.
- Meaning-preserving clarification may proceed only when the approved contract
  remains semantically unchanged; a meaning change returns to its approval gate.
- Interrupted work preserves observable files, commits, agent state, and gaps.
  It is never reset or reconstructed from an uncertain summary to force progress.

## Compatibility

New planned work uses the contract-centered directory structure. Historical
plans stay unchanged. An already executing legacy plan may finish under its
approved contract unless a material ambiguity prevents safe continuation or the
owner explicitly chooses migration.
