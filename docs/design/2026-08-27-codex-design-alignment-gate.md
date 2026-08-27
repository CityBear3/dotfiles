# [Design Doc] Codex Design Alignment Gate

- Owner: Repository owner
- Drafted by: Codex from settled owner decisions
- Date: 2026-08-27
- Status: Approved
- Extends: `docs/design/2026-07-29-codex-agentic-engineering-workflow.md`
- Refines: `docs/design/2026-08-13-codex-design-centered-contract-workflow.md`

## Context and scope

The Codex engineering workflow assigns architecture, scope, public contracts,
and material trade-offs to the user. It also requires Design Docs to derive from
an owner-approved decision record. These declarative ownership rules do not yet
make shared understanding a procedural prerequisite. The coordinator may decide
that a change is already settled and advance directly to Design Doc drafting,
while `design-discussion` does not require one-question exploration of material
design branches or an explicit readiness check.

That gap allowed a Design Doc to be presented before the user and Agent had
worked through a complete decision record. A polished document can appear
coherent even when it contains assumptions the user has not considered. Once
implementation is autonomous or distributed across agents, those assumptions
can propagate through contracts, plans, and tasks before the mismatch becomes
visible.

This design adds an explicit alignment state to every change and a Design
Readiness Gate to planned work. It treats discussion as an interactive search of
the applicable design tree, not as approval of a complete Agent-authored design.
It also keeps the process proportional: a small, uniquely determined change does
not receive the same interview as a cross-cutting design.

The interaction model is informed by
[the design-centered development workflow](https://azukiazusa.dev/blog/recent-ai-coding-development-process-centered-on-design/)
and its
[`/grill-me` design-interview discussion](https://azukiazusa.dev/blog/before-implementation-interview-design-requirements-grill-me/).

### Goals

- Establish shared user-Agent understanding before planned work produces a
  Design Doc, Feature Contract, Implementation Plan, or implementation handoff.
- Keep the user in control of every material design branch while letting the
  Agent investigate facts, compare options, and recommend an answer.
- Ask one material question at a time and follow dependencies between decisions
  until the applicable design tree is resolved.
- Make readiness observable through an applicability-driven gate rather than an
  informal Agent judgment or a fixed exhaustive interview.
- Preserve planned decisions across compaction and sessions in a temporary,
  ignored living record.
- Reuse exact current approved design authority and reopen only changed or
  missing branches.
- Avoid duplicate holistic approval when a Design Doc will provide the final
  whole-design confirmation.
- Preserve implementation-agent freedom over private files, helpers, syntax, and
  other local realization choices.

### Non-goals

- Require a full design interview for every engineering change.
- Require a Design Doc for every planned change.
- Require a new branch or worktree solely because design discussion occurs.
- Add a separate alignment or `grill-me` skill.
- Turn natural-language design discussion into a machine-readable state machine
  or fixed questionnaire.
- Prescribe file-by-file implementation steps, private helper interfaces, or
  local algorithms.
- Change Task execution, review-role selection, publication, merge, destructive
  action, or branch-disposition policy except where those phases consume design
  authority.
- Modify global `AGENTS.md` guidance; the workflow and phase skills own this
  behavior.

### Explicit deferrals

None.

## Overview

Every explicit engineering change has an alignment state. Read-only requests
remain read-only and do not enter this state machine.

```text
explicit change request
         |
         v
investigate repository facts
         |
         v
classify alignment route
         |
         +-- lightweight eligible ------------------------------+
         |                                                       |
         |  request is complete --> use exact request authority  |
         |                                                       |
         |  only non-material gaps --> present one concise       |
         |                             record for confirmation    |
         |                                                       v
         |                                               execute-task path
         |
         +-- material choice or planned coordination required
                                 |
                                 v
                    confirm coordination workspace
                                 |
                                 v
                    create living decision record
                                 |
                                 v
                    resolve one material branch at a time
                                 |
                                 v
                       Design Readiness Gate
                                 |
                    +------------+-------------+
                    |                          |
             Design Doc warranted       no Design Doc
                    |                          |
             draft Design Doc           approve complete
                    |                    decision record
             approve exact doc                 |
                    |                          |
             verify authority transfer         |
                    |                          |
             retire decision record            |
                    +------------+-------------+
                                 |
                                 v
                         Feature Contract
```

The key distinction is between settling decisions and approving a design
authority. Each material choice is settled interactively. When a Design Doc is
warranted, the exact Design Doc receives the one holistic approval. When there
is no Design Doc, the decision record itself becomes the design authority and
therefore receives that approval.

## Detailed design

### Responsibility boundaries

`agentic-engineering-workflow` owns:

- classifying every explicit change as lightweight or planned after repository
  investigation;
- retaining the exact alignment source and its currentness;
- preventing a planned-path transition until Design Readiness is established;
- establishing or confirming the coordination workspace before the first
  recoverable planned decision artifact is written;
- selecting the Design Doc or no-Design-Doc transition without treating artifact
  existence as approval;
- reopening the correct design source when later work exposes a material gap.

`design-discussion` owns:

- investigating code, tests, documentation, and history before asking the user;
- distinguishing repository facts from user-owned choices;
- exploring one material decision at a time with the smallest viable option set,
  concrete codebase-specific trade-offs, and a recommendation;
- following branches created by each decision and dependencies between settled
  decisions;
- maintaining settled, rejected, deferred, and unresolved information in the
  living decision record;
- reporting whether Design Readiness is satisfied and whether a durable Design
  Doc is warranted.

`design-doc` owns:

- validating that a consolidated decision record has no unresolved material
  question before drafting;
- deriving prose only from settled decisions and repository evidence;
- returning a newly discovered material ambiguity to `design-discussion` rather
  than hiding a choice in prose;
- presenting the exact document for the one holistic design approval;
- verifying that the approved document received every durable decision before
  the temporary record is retired.

The coordinator owns transitions; phase skills own their mechanics. None of
these skills takes ownership of the user's material design decisions.

### Lightweight alignment

The existing lightweight eligibility contract remains the safety boundary. A
request qualifies only when investigation shows that its objective, observable
behavior, scope, constraints, and lack of material trade-offs are uniquely
determined.

If the request explicitly contains that information, its exact content is the
approved alignment source. No extra alignment file or approval is added. If the
request has only non-material omissions that repository evidence can resolve,
the Agent derives a concise alignment record. When that derivation requires user
confirmation but not a material design choice, the Agent presents the record and
asks once. An unpresented or unconfirmed Agent summary is not shared authority.

A material choice, durable coordination requirement, or unrecoverable
in-memory contract promotes the work to the planned path. The lightweight path
does not stretch its eligibility criteria to avoid design discussion.

### Planned discussion and living decision record

Initial investigation may begin in the current context. As soon as purpose and
the initial feature boundary are identifiable, the coordinator confirms the
checkout, branch, and starting ref through `create-workspace`. Confirmation does
not require creating a new branch or worktree when the current checkout is
already approved and suitable.

Before persisting the first material decision, `design-discussion` creates:

```text
docs/plans/YYYY-MM-DD-<feature>/decision-record.md
```

The file is ignored and workspace-only. It contains:

- selected approaches and rationale;
- rejected alternatives and reasons;
- scope and non-goals;
- explicitly accepted deferrals, including their intent and impact;
- unresolved material questions in a separate section.

The record is updated only as choices settle. An Agent recommendation is not a
decision. A question is not deferred unless the user explicitly accepts the
deferral. The record is a recovery aid and drafting source; it does not become
approved design merely because it exists.

### One-question design-tree exploration

For each unresolved material branch, the Agent:

1. states the decision and why it is now reachable;
2. presents the smallest viable option set;
3. explains concrete trade-offs against the current system;
4. recommends one answer and gives its rationale;
5. asks only that question and lets the user decide;
6. records the settled choice and identifies newly reachable dependent branches.

The Agent does not ask for facts it can obtain from the repository. It also does
not present the whole completed design before the user has participated in its
material branches. The user may reject the option set, supply domain constraints,
challenge the recommendation, or redirect the discussion.

When a material decision requires a prototype, visual artifact, benchmark, or
other high-fidelity evidence, the discussion records an explicit handoff and the
evidence required to re-enter that branch. It does not guess or classify the
question as harmless deferral. If the scope is too broad to explore reliably,
scope decomposition itself becomes the next design decision.

### Design Readiness Gate

The planned path may leave design discussion only when every applicable
condition is satisfied:

1. Facts discoverable from the codebase have been investigated.
2. Purpose and observable completion conditions are settled.
3. Scope, non-goals, constraints, and invariants are settled.
4. Applicable responsibility boundaries, dependency direction, and interfaces
   are settled.
5. Expected behavior and its verification method are settled.
6. Failure and recovery, migration, concurrency, authorization, performance,
   and comparable concerns are settled when applicable.
7. Material branches created by decisions, and dependencies between decisions,
   are resolved.
8. Questions requiring another discovery phase have explicit handoffs and
   re-entry conditions.
9. No material question remains unresolved, except a deferral whose intent and
   impact the user explicitly accepted.
10. The settled decisions have been consolidated into the complete decision
    record.

These are readiness dimensions, not ten mandatory questions. The Agent selects
only applicable branches and must not expand scope to manufacture answers for
irrelevant dimensions.

### Approval and authority lifecycle

Approval is evidence of shared understanding; accumulating approval events is
not the objective.

For work that warrants a Design Doc:

1. every material choice is settled and Design Readiness is established;
2. `design-doc` drafts from the complete decision record without requiring a
   separate holistic approval of that temporary record;
3. the user reviews and approves the exact Design Doc as the single holistic
   design confirmation;
4. `design-doc` checks that the approved document contains the selected approach
   and rationale, rejected alternatives and reasons, scope and non-goals, and
   explicit deferrals;
5. only after that transfer check is the temporary decision record deleted.

For work that does not warrant a Design Doc, the complete decision record is
presented and explicitly approved before Feature Contract drafting because it
is the design authority. It remains available through the Feature Contract and
workspace lifecycle and is retired with the other temporary planned artifacts.

Mechanical consolidation or wording clarification does not add a new approval
gate. A new decision or semantic change reopens the affected branch and every
dependent decision whose meaning may change.

### Existing authority and re-entry

An exact, current, explicitly approved Design Doc or no-Design-Doc decision
record may satisfy the applicable readiness dimensions without repeating the
interview. The coordinator verifies the source, exact content, approval state,
and currentness. A past conversation, unapproved document, or plausible summary
does not qualify.

When only part of the authority is missing, `design-discussion` reopens only the
missing branch. When a settled choice changes, it also rechecks downstream
decisions that depend on that choice. The revised authority is re-presented at
the approval layer it invalidated; unchanged independent decisions remain valid.

If Design Doc drafting, Feature Contract construction, planning, or
implementation exposes a material ambiguity, that phase stops and returns the
evidence to the coordinator. The coordinator re-enters `design-discussion` at
the affected branch and does not continue under an inferred answer.

## Cross-cutting concerns

### Proportionality and interaction cost

The lightweight path avoids a full interview for explicit, uniquely determined
work. The planned path avoids a fixed exhaustive questionnaire by testing
applicability. Questions remain one at a time so the user can reason about each
branch without approving an apparently complete design wholesale.

The process does not measure quality by question count. Repetitive questions,
low-impact local implementation choices, and speculative future requirements
are scope expansion rather than evidence of rigor.

### Compaction and recovery

The living decision record prevents planned design from depending only on chat
history. On resume, the coordinator reads the current record and repository
state, establishes which decisions are settled, and reopens only unresolved or
stale branches. It does not infer approval from file presence.

The Design Doc becomes the durable source only after exact approval and a
successful transfer check. Until then, the decision record remains available so
that drafting errors or omissions can be corrected against the settled source.

### Compatibility with the contract workflow

This design adds a prerequisite to, rather than replacing, the existing Design
Doc, Feature Contract, and Implementation Plan layers. After design authority is
established, Feature Contract derivation and approval continue to define feature
success, and Implementation Plan approval continues to define task topology and
execution policy.

The Feature Contract may clarify the application of approved design but cannot
silently complete a missing design branch. Difficulty deriving the contract is
evidence that design must be reopened.

### Failure behavior

The workflow stops at the current phase when:

- a material branch lacks a user decision;
- an applicable readiness dimension is missing;
- a prototype or other evidence handoff has not satisfied its re-entry condition;
- the decision record, Design Doc, or approval/currentness evidence cannot be
  recovered;
- downstream work exposes a semantic change to its design authority.

Operational inability to establish a required workspace or artifact is
`BLOCKED`. A missing user-owned choice or requested semantic expansion is an
approval/design re-entry condition, not an implementation failure.

### Verification obligations

Skill validation must demonstrate the workflow behavior, not merely the
presence of prescribed wording. Verification should establish that:

- an unambiguous lightweight request does not receive a full interview or a
  decision-record file;
- a planned change cannot reach Design Doc drafting with an unresolved material
  branch;
- design discussion asks one material question at a time after repository
  investigation;
- a prototype-dependent question creates a handoff and re-entry condition;
- the Design Doc path uses one holistic approval and retires the record only
  after transfer is checked;
- the no-Design-Doc path approves and retains its decision record as authority;
- existing current authority skips redundant discussion, while a changed
  decision reopens dependent branches;
- each changed skill remains structurally valid and its responsibility agrees
  with the other two skills.

Exact validation commands and scenario fixtures belong in the Feature Contract
and Implementation Plan unless repository tooling makes them contractually
significant.

## Alternatives considered

### Require a full interview for every change

Rejected because trivial work with a complete explicit request does not benefit
from planned-path ceremony. Alignment is universal, but its representation and
depth are proportional.

### Let the Agent decide informally that design is settled

Rejected because an informal judgment allowed the workflow to advance before
shared understanding was established. Readiness must be supported by applicable
dimensions, resolved branches, and a recoverable source.

### Use a fixed exhaustive questionnaire

Rejected because relevance depends on the change. It would encourage irrelevant
questions, fatigue, and speculative scope expansion without proving shared
understanding.

### Add a separate alignment or `grill-me` skill

Rejected because `design-discussion` already owns investigation, alternatives,
trade-offs, and user-owned decisions. A second skill would split one
responsibility and create ambiguous routing.

### Keep planned decisions only in conversation

Rejected because planned work must survive compaction, fresh sessions, artifact
drafting, and potentially parallel task execution without reconstructing design
authority from memory.

### Approve both the decision record and derived Design Doc holistically

Rejected because the same complete design would receive two consecutive
approvals. Individual choices still require explicit settlement, while approval
of the exact Design Doc provides the whole-design confirmation.

### Delete the record when a Design Doc draft is created

Rejected because a draft is not approved authority and may omit or alter a
settled decision. Retirement occurs only after exact Design Doc approval and a
successful transfer check.

### Repeat the complete design discussion whenever work resumes

Rejected because exact, current approved authority remains valid. Re-entry is
limited to missing, changed, and dependent branches.

### Persist planned decisions before workspace confirmation

Rejected because a recoverable artifact could be written into the wrong
checkout or branch. Read-only investigation may start immediately, but the
coordination workspace is confirmed before the first material decision is
persisted.
