# [Design Doc] Codex Agentic Engineering Workflow

- Owner: Repository owner
- Drafted by: Codex from owner-approved design decisions
- Date: 2026-07-29
- Status: Approved; review-calibration amendment approved 2026-07-29

## Context and scope

The Codex assets already define focused skills for design discussion, planning,
implementation, verification, review, review-feedback triage, branch completion,
and session teardown. They do not define one authoritative workflow that selects
an entry path, owns phase transitions, or distinguishes user approval gates from
autonomous engineering loops. Those rules are currently implied by individual
skill entry and exit clauses.

The corresponding Claude configuration contains a central Core Flow, but copying
that flow verbatim into Codex global guidance would duplicate phase mechanics and
make the always-loaded prompt larger. Current
[GPT-5.6 guidance](https://developers.openai.com/api/docs/guides/model-guidance?model=gpt-5.6-sol#prompting-best-practices)
favors lean prompts that state each instruction once, while retaining explicit
autonomy, approval, success, and stopping boundaries. Codex skills provide the
appropriate progressive-disclosure layer for a durable workflow contract.

This design introduces a thin global trigger, one coordinating workflow skill,
and one path-neutral task-execution skill. The coordinator owns path selection and
cross-phase transitions. Planned execution owns plan-level sequencing, while the
shared task seam owns one task's implementation and acceptance mechanics. It also
changes Design Doc collaboration to match the owner's current practice: the owner
makes design decisions through detailed dialogue, and Codex may draft the
resulting document after those decisions are settled.

### Goals

- Give Codex one authoritative engineering workflow for change requests.
- Preserve a short, practical `AGENTS.global.md`.
- Keep architecture, scope, public contracts, and material trade-offs under user
  control.
- Allow safe in-scope implementation, verification, and review loops to proceed
  without unnecessary approval pauses.
- Provide a lightweight path for uniquely determined, low-risk changes.
- Let an implementation plan declare review intensity so review cost and breadth
  match the change's risk.
- Give every reviewer enough artifact-specific context to distinguish material
  defects from inapplicable code-oriented concerns.
- Keep the finding threshold proportional and evidence-based even when `deep`
  review broadens the perspectives consulted.
- Let Codex draft a Design Doc from an owner-approved decision record without
  inventing decisions.
- Give lightweight and planned work one task-execution contract without
  duplicating its acceptance mechanics.
- Preserve fresh verification and evidence-based review before branch completion.

### Non-goals

- Reproduce the Claude workflow or its wording exactly.
- Require a Design Doc or implementation plan for every change.
- Move phase-specific commands or detailed mechanics into global guidance.
- Let Codex make unresolved architecture, scope, algorithm, or public-contract
  decisions.
- Encode GPT-5.6 model names or reasoning effort in implementation plans.
- Treat reviewer count, maximal reasoning effort, or speculative findings as
  quality goals.
- Turn natural-language skill handoffs into machine-protocol schemas, content
  identities, or serialization contracts without a real runtime consumer that
  requires them.
- Lower the finding threshold merely because `deep` review was selected.
- Import or read `claude/` assets at Codex runtime.
- Change publication, destructive-action, or external-write approval boundaries.
- Execute multiple implementation-plan tasks in parallel. The task boundary may
  support future dependency-aware scheduling, but this scope remains sequential
  and single-writer.

## Overview

The workflow has five responsibility layers:

1. `codex/AGENTS.global.md` contains only the durable trigger, user-ownership
   invariant, approval boundary, and requirement to use the coordinator for
   engineering changes.
2. `agentic-engineering-workflow` is the single source of truth for entry
   classification, cross-phase transitions, lightweight eligibility, and
   escalation.
3. `execute-plan` owns approved-plan and policy validation, dependency ordering,
   task handoff and evidence aggregation, and plan-deviation detection.
4. `execute-task` owns the path-neutral discipline and acceptance contract for one
   lightweight or planned task.
5. `agent-teams-driven-development` owns agent scheduling and partial-state
   safety. Other phase skills own their verification, review, triage, and branch
   completion mechanics.

Read-only requests such as explanation, diagnosis, review, and planning do not
authorize implementation. A requested change enters one of two implementation
paths:

```text
                              +----------------------+
                              | answer / investigate |
                              +----------------------+
                                         ^
                                         |
request -> classify intent --------------+
          |
          | requested change
          v
   investigate current state
          |
          +-- all lightweight criteria hold --> workspace
          |                                      |
          |                                      v
          |                              materialize policy
          |                                      |
          |                                      v
          |                                 execute-task
          |
          +-- design decision remains --------> design discussion
                                                   |
                                                   +-- Design Doc when warranted
                                                   |       |
                                                   |       v
                                                   +--> create plan
                                                          |
                                                          v
                                                       workspace
                                                          |
                                                          v
                                                     execute-plan
                                                          |
                                                          | each task, in
                                                          | dependency order
                                                          v
                                                     execute-task

lightweight execute-task -----------+
                                    |
planned execute-plan aggregation ---+--> verify -> review -> triage
                                                   ^          |
                                                   |          +-- Fix --> bounded task
                                                   |          +-- Push back
                                                   |          +-- Escalate --> user
                                                   +----------------------+

clean review -> finish branch -> user chooses publication or disposition
```

The user's original change request is sufficient implementation authorization for
the lightweight path. The planned path requires approval of design decisions, any
Design Doc, and the implementation plan before execution.

## Detailed design

### Coordinator responsibility

The coordinator determines the current path, workflow phase, and next cross-phase
transition. It does not own task discipline, implementation handoff, plan
sequencing, agent scheduling, verification commands, review mechanics, or branch
publication.

For each transition it records or reports:

- the current path and phase;
- the approved scope and relevant decision source;
- the next automatic action or user-controlled gate;
- verification or review evidence required to leave the phase;
- any unresolved condition that prevents a safe transition.

An explicit user instruction to skip a phase or execute directly takes precedence
when it does not violate repository guidance or an approval boundary. Skipping a
phase does not permit Codex to invent missing design decisions or claim unobserved
verification.

### Task execution seam and component responsibilities

Both implementation paths use `execute-task` as the only task-execution seam. The
lightweight path invokes it directly after workspace confirmation and complete
policy materialization. The planned path invokes `execute-plan`, which invokes
`execute-task` once per task in dependency order.

`execute-plan` owns:

- validation of the approved plan and its complete approved review policy;
- dependency ordering and construction of each complete task handoff;
- aggregation of task commits, accepted ranges, verification, gate results,
  correction outcomes, and gaps;
- detection of a plan deviation or missing decision before handing control back
  to the coordinator.

`execute-task` owns one task's:

- declared discipline and implementation handoff to the actual writer;
- exact task verification and pre-commit working-tree diff inspection;
- task commit, new head, and exact base-to-head range;
- policy-selected per-task gate;
- bounded in-scope correction, fresh verification, re-review, and a clear stop
  condition.

The actual writer is either the lead or one `implementer`. This design does not
introduce multiple concurrent writers. `agent-teams-driven-development` owns only
agent scheduling, one-writer/read-only-reviewer enforcement, capacity and queue
management, and safe handling of agent failure or partial state. It refers to
`execute-task` for task and policy semantics rather than redefining them.

Each accepted task retains its own base-to-head range and gate evidence. For
planned work, `execute-plan` also reports the aggregate final HEAD and complete
implementation range after all tasks. The aggregate HEAD proves the cumulative
branch state; it does not replace or widen any task-specific accepted range.
Global verification and final review use the aggregate final HEAD and complete
range, while per-task acceptance remains attached to each task's exact range.

### Lightweight path

A change qualifies for the lightweight path only when all of the following are
true after inspecting the repository:

- the user explicitly requested a change;
- the objective, expected behavior, and scope are uniquely determined;
- no architecture, public contract, schema, or error-model decision changes;
- no material user-owned trade-off remains;
- the work is one coherent change;
- the work requires no external write, publication, destructive action, or
  material scope expansion.

File count and changed-line count are not eligibility criteria. A small diff may
still require design, while a mechanically broad but uniquely specified migration
may remain lightweight.

Before implementation, Codex confirms that the current workspace and branch are
suitable. Production-behavior changes use TDD; content, configuration, and
mechanical migrations use an explicit discipline appropriate to their contract.
Before invoking `execute-task`, the coordinator selects the lightweight
`focused` policy from the Design default, the original change authorization, and
observed risk and runtime capacity. It records the rationale, applicable review
perspectives, omissions, and Acceptance threshold together with the lightweight
Review context.

An explicit user-approved `adaptive` or `deep` mode replaces the default. If
materializing a complete policy would require a material choice, or observed risk
makes `focused` inappropriate, the coordinator returns to the planned path instead
of silently choosing or strengthening policy. The task then runs fresh global
verification and final review before branch completion.

If implementation reveals a disqualifying risk surface or a material decision,
Codex does not silently strengthen review and continue. It stops the lightweight
path and returns to design discussion, followed by planning when the revised scope
is settled.

Security or permission boundaries, persistent-data migration, concurrency or
recovery guarantees, and data-loss risk are disqualifying risk surfaces unless
investigation demonstrates that the requested change does not alter that
contract. Public API, schema, and error-model changes are already excluded by the
eligibility criteria.

### Planned path and approval gates

Design discussion investigates the current system and presents one material
decision at a time. The user owns the choice. A Design Doc is used when the
settled work has cross-cutting architecture, durable contracts, or significant
decisions worth preserving; otherwise the workflow may proceed directly to plan
creation after the user approves that transition.

The following remain user-controlled gates:

- unresolved design, architecture, scope, algorithm, and public-contract choices;
- approval of a drafted Design Doc;
- approval of an implementation plan, including its review policy;
- any plan deviation or material scope expansion;
- publication, push, pull-request creation, merge, discard, and destructive action;
- external writes and other actions outside the authority granted by the request.

The following transitions are automatic once their entry conditions are satisfied:

- an authorized lightweight task to `execute-task`;
- an approved plan to `execute-plan`, then to `execute-task` once per task;
- current lightweight task acceptance evidence to global verification;
- current planned nonfinal task acceptance evidence back to `execute-plan` for the
  next dependency-ordered task;
- acceptance evidence for every planned task to `execute-plan` aggregation, then
  the aggregate final HEAD and complete implementation range to global
  verification;
- passing verification to review;
- review findings to `receiving-code-review` triage;
- an in-scope `Fix` to implementation, verification, and fresh review;
- `Push back` to continued triage using the recorded decision and evidence;
- a clean review to `finish-branch`.

`finish-branch` always stops for the user's publication or disposition choice.
`Escalate` always stops for a user decision because it indicates that existing
authority is insufficient.

Verification failure is not itself a user gate. Codex diagnoses and fixes an
in-scope failure, then re-runs verification. It stops when resolution would change
the approved design or scope, required authority is missing, or the workflow's
bounded retry condition demonstrates that the current correction is not working.

During planned execution, `execute-plan` invokes `execute-task` sequentially in
dependency order and returns its aggregate evidence only after every task gate is
current. The boundary intentionally permits future scheduling of
dependency-independent tasks without moving task semantics, but parallel task
execution is outside the current scope.

### Collaborative Design Doc drafting

The Design Doc skill preserves user ownership of design without requiring the user
to write the prose.

Codex first investigates facts, exposes ambiguities, compares viable alternatives,
and asks detailed questions until no material design decision remains unresolved.
It maintains a decision record containing the selected approach, rationale,
rejected alternatives, scope, non-goals, and any explicitly deferred question.

After the user settles those decisions, Codex may draft the complete Design Doc.
The draft must be derivable from the decision record and repository evidence. It
must not hide a new design choice inside polished prose. If drafting reveals an
ambiguity that could materially change the result, Codex returns to discussion
instead of choosing silently.

The user reviews and approves the document before plan creation. When the user
prefers to author a draft, the skill continues to support structure, research,
critique, and targeted edits.

### Review context

Review policy controls breadth and independence. A separate, concise Review
context tells reviewers how to interpret the artifact being reviewed. Every plan
records one, and the lightweight path derives one from the approved request,
repository evidence, and any settled decisions before implementation.

The Review context describes in plain language:

- the artifact type and its purpose;
- its consumers and execution or interpretation model;
- the behavior and quality characteristics that matter;
- realistic failure modes with material impact;
- approved trade-offs and conditions that are not problems by themselves;
- perspectives or assumptions that do not apply to this artifact.

The Review context is not a versioned schema, content identity, or duplicated
evidence envelope. It is passed with the task or final-review request to every
reviewer and integrator. For standalone review, Codex derives the smallest
reasonable context from the requested files and repository evidence and reports
any material assumption.

Approved non-problems do not make an artifact immune from review. A reviewer may
revisit one only with materially new evidence showing a concrete reachable
failure or an approved-contract violation. A preference, a generic best practice,
or an imagined future consumer is not such evidence.

For Markdown skills and role prompts, reviewers treat the artifact as
natural-language instructions interpreted by Codex unless an actual machine
consumer is identified. They review responsibility boundaries, approval and
safety rules, contradictory or unusable instructions, prompt size, and realistic
tool-loop behavior. The absence of canonical serialization, content digests,
protocol identities, or a fully explicit state machine is not a defect by itself.

### Review policy

Every implementation plan contains a `Review policy` section. The planner proposes
the policy from concrete risk surfaces, and the user approves or adjusts it as part
of plan approval.

The policy records:

- mode: `focused`, `adaptive`, or `deep`;
- risk surfaces relevant to the change;
- the per-task review gate;
- required and conditional final reviewers;
- explicitly skipped review perspectives and their rationale;
- residual risk plus capacity and queue rules;
- the evidence and finding threshold for acceptance.

The policy references the Review context but does not repeat it.

The three modes have these contracts:

| Mode | Per-task review | Final review |
| --- | --- | --- |
| `focused` | One reviewer checks both specification compliance and code quality. | General code review, plus test coverage when behavior or tests changed. |
| `adaptive` | Specification and code quality are reviewed independently. | Standard and adversarial reviewers are selected only for identified risk surfaces. |
| `deep` | Specification and code quality are reviewed independently. | All perspectives applicable to the artifact and observed risks run, followed by adversarial integration when adversarial review runs. |

`adaptive` is the default for planned work. `focused` is the default for the
lightweight path.

Independent specification and quality review is part of the `adaptive` and `deep`
mode contracts, not an optional implementation detail. A direct/no-agent
instruction conflicts with either mode. The workflow must `Escalate` for agent
permission or a user-approved policy change, such as changing to `focused`; it
must not silently waive independence or treat sequential lead passes as
equivalent. `focused` permits a no-agent combined lead pass using the same complete
role, evidence, and output contract.

Risk surfaces include, when applicable, public APIs, persistence and migration,
security and permissions, concurrency, error and recovery behavior, performance
hot paths, cross-component responsibility changes, and tests whose doubles or
fixtures may diverge from production behavior. The planner recommends `adaptive`
or `deep` when these risks make `focused` inappropriate. If the user chooses a
lighter policy, the plan records omitted perspectives and residual risk rather
than silently dropping them.

Review strength controls breadth and independence, not the threshold for accepting
a finding. Model and reasoning-effort selection remain properties of reviewer
profiles so plans do not become coupled to a particular current model. In the
existing GPT-5.6 Sol setup, selecting `deep` may activate detailed specialist
profiles already configured for high or xhigh reasoning when their perspective is
applicable; the plan does not add generic instructions to “think harder” or run an
irrelevant profile merely to maximize reviewer count.

Every mode retains the same finding standard. A surviving finding must be
applicable to the artifact described by the Review context, identify a concrete
reachable behavior or approved-contract violation, cite evidence, state a
material consequence, and propose a proportionate correction. `Should Improve`
requires a concrete maintainability consequence or measurable repeated cost; it
is not a label for optional polish. Preference-only comments, speculative future
concerns, inapplicable code-oriented assumptions, and objections to an approved
decision without new evidence are removed.

A suggestion that introduces a new state machine, schema, identity system, or
other architectural mechanism is not an automatic `Fix` unless that mechanism is
the proportionate correction for a proven in-scope violation. Otherwise it is a
design proposal and therefore `Escalate`. The adversarial integrator applies the
same Review context and actively rejects unsupported, second-order, and
artifact-inapplicable findings instead of treating reviewer output volume as a
quality signal.

Capacity limits may queue reviewers but never silently reduce the approved
applicable scope or independence. An unavailable required gate remains a reported
gap and cannot be converted into an implicit waiver.

### Review feedback and loop completion

Every review item is classified as:

- `Fix`: verified, in scope, and compatible with the approved design;
- `Push back`: incorrect, unsupported, preference-only, or already decided;
- `Escalate`: requires a new design decision or material scope expansion.

For planned work, a `Fix` becomes a concrete plan step. For lightweight work, it
becomes a bounded fix step attached to the current change. Both routes return the
bounded task to `execute-task`, re-run relevant verification, create and inspect
the correction commit and exact range, and run fresh review; editing the code does
not resolve a finding by itself.

A workflow reaches `finish-branch` only when verification is current for the head
commit and the approved review policy has produced no remaining Must Fix or Should
Improve finding. Agent self-review or an earlier per-task approval does not replace
the final review required by that policy.

### Source-of-truth and compatibility rules

The coordinator is the sole source of truth for path selection and cross-phase
transitions. `execute-plan` is the source of truth for plan-level validation,
dependency order, handoff aggregation, and deviation detection. `execute-task` is
the source of truth for one task's discipline, commit/range evidence,
policy-selected gate, and bounded correction loop.

`agent-teams-driven-development` is the source of truth only for scheduling,
capacity, queueing, one-writer/read-only-reviewer enforcement, and agent
failure/partial-state safety. It and other phase skills may state their entry,
evidence, and handoff contracts, but do not reproduce task or cross-phase
semantics.

Repository-local `AGENTS.md` files remain authoritative for project-specific
commands and stricter constraints. The global guidance and coordinator do not
weaken them.

Codex assets remain self-contained. Claude assets may inform a deliberate port
during development but are never imported or read at runtime. The existing
installer's source inventory is expected to discover a new skill automatically;
implementation must still verify inventory, destination mapping, and installation
tests rather than assuming discovery succeeded.

## Cross-cutting concerns

### Prompt size and drift

The always-loaded global file contains only stable policy. The coordinator
contains path and cross-phase routing, `execute-plan` contains plan orchestration,
`execute-task` contains the shared task contract, and agent-team and phase skills
contain their specialized mechanics. Each rule has one authoritative home, with
short references and lazily loaded role prompts at boundaries. This limits prompt
repetition and makes drift detectable during review.

### Autonomy and safety

Autonomy applies only within approved local scope. Read-only requests do not become
implementation, and implementation authority does not imply publication or
destructive authority. The coordinator continues safe work without asking at every
step but stops when a new user-owned decision is required.

### Evidence and observability

Progress and completion reports identify the active phase, approved scope, Review
context, review policy, reviewers run or skipped, commands observed, and any
unverified gap. A successful tool call, edit, build, or subagent message is
intermediate evidence; completion requires the phase's behavioral contract.

### Failure and re-entry

The workflow may re-enter implementation after verification or review. Re-entry
retains the approved decisions, non-goals, Review context, review policy, exact
unresolved finding, task range, and observed correction attempts. It does not
reopen settled design without evidence. If the same concrete problem survives a
bounded correction attempt, the workflow stops and explains the remaining gap
instead of inventing a more elaborate protocol or creating an unbounded loop.

## Alternatives considered

### Put the full workflow in `AGENTS.global.md`

This would make the flow visible on every turn and resemble the Claude
configuration. It was rejected because it duplicates phase mechanics in the
always-loaded prompt and increases drift. A thin trigger plus a workflow skill
keeps the contract discoverable with less repeated context.

### Keep only independent phase skills

This preserves the smallest prompt footprint and lets Codex infer transitions. It
was rejected because approval gates, lightweight eligibility, automatic loops, and
branch-completion conditions would remain implicit and could vary across sessions.

### Require design discussion and a plan for every change

This produces a uniform audit trail. It was rejected because uniquely specified,
low-risk changes would pay disproportionate interaction and review cost. The
lightweight path retains verification and review while using explicit eligibility
criteria instead of file-size heuristics.

### Always run exhaustive review

This maximizes reviewer breadth. It was rejected because per-task and final review
already overlap, specialist reviewers can generate low-value noise on irrelevant
surfaces, and maximum effort is not automatically the best quality-cost trade-off.

### Let the reviewer choose all review depth at runtime

This exploits model judgment and avoids plan detail. It was rejected because the
expected cost, skipped perspectives, and residual risk would be invisible at plan
approval. The selected design lets the plan establish risk and policy while
reviewer profiles retain tactical judgment.

### Treat every handoff as a machine protocol

This can make every intermediate state mechanically distinguishable, but the
artifacts in this design are primarily natural-language skills interpreted by an
agent. It was rejected because canonical encodings, content identities, manifest
digests, and exhaustive lifecycle schemas add prompt cost and new consistency
failure modes without a matching runtime requirement. Exact Git ranges, observed
commands, concise state, and explicit gaps provide sufficient evidence here.

### Require the user to author all Design Doc prose

This makes authorship and decision ownership identical. It was rejected because
the owner's current workflow settles detailed decisions through dialogue and then
benefits from agent drafting. Separating decision ownership from prose production
preserves architectural control without imposing manual transcription.
