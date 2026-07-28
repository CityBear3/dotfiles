# [Design Doc] Codex Agentic Engineering Workflow

- Owner: Repository owner
- Drafted by: Codex from owner-approved design decisions
- Date: 2026-07-29
- Status: Approved

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

This design introduces a thin global trigger and one coordinating workflow skill.
The coordinator owns the state machine and delegates phase mechanics to the
existing skills. It also changes Design Doc collaboration to match the owner's
current practice: the owner makes design decisions through detailed dialogue, and
Codex may draft the resulting document after those decisions are settled.

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
- Let Codex draft a Design Doc from an owner-approved decision record without
  inventing decisions.
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
- Import or read `claude/` assets at Codex runtime.
- Change publication, destructive-action, or external-write approval boundaries.

## Overview

The workflow has three layers:

1. `codex/AGENTS.global.md` contains only the durable trigger, user-ownership
   invariant, approval boundary, and requirement to use the coordinator for
   engineering changes.
2. `agentic-engineering-workflow` is the single source of truth for entry
   classification, state transitions, lightweight eligibility, review-policy
   application, and escalation.
3. Existing phase skills own their current mechanics. Their entry and transition
   wording points back to the coordinator rather than defining competing flows.

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
          +-- all lightweight criteria hold --> workspace --> implement
          |                                      (focused review policy)
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
                                                        execute

implement / execute -> verify -> review -> triage
                         ^          |
                         |          +-- Fix ------> bounded fix
                         |          +-- Push back -> continue triage
                         |          +-- Escalate --> user decision
                         |
                         +-------------------------------+

clean review -> finish branch -> user chooses publication or disposition
```

The user's original change request is sufficient implementation authorization for
the lightweight path. The planned path requires approval of design decisions, any
Design Doc, and the implementation plan before execution.

## Detailed design

### Coordinator responsibility

The coordinator determines the current workflow state and the next phase. It does
not duplicate how a phase investigates, writes tests, edits files, dispatches
agents, verifies commands, or publishes a branch.

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
The path then runs fresh verification and a `focused` review before branch
completion.

The user may explicitly request `adaptive` or `deep` review for a lightweight
change. If implementation reveals a disqualifying risk surface or a material
decision, Codex does not silently strengthen review and continue. It stops the
lightweight path and returns to design discussion, followed by planning when the
revised scope is settled.

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

- approved implementation or an approved plan to verification;
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
- the evidence and finding threshold for acceptance.

The three modes have these contracts:

| Mode | Per-task review | Final review |
| --- | --- | --- |
| `focused` | One reviewer checks both specification compliance and code quality. | General code review, plus test coverage when behavior or tests changed. |
| `adaptive` | Specification and code quality are reviewed independently. | Standard and adversarial reviewers are selected only for identified risk surfaces. |
| `deep` | Specification and code quality are reviewed independently. | All applicable standard and adversarial perspectives run, followed by adversarial integration. |

`adaptive` is the default for planned work. `focused` is the default for the
lightweight path.

Risk surfaces include, when applicable, public APIs, persistence and migration,
security and permissions, concurrency, error and recovery behavior, performance
hot paths, cross-component responsibility changes, and tests whose doubles or
fixtures may diverge from production behavior. The planner recommends `adaptive`
or `deep` when these risks make `focused` inappropriate. If the user chooses a
lighter policy, the plan records omitted perspectives and residual risk rather
than silently dropping them.

Review strength controls breadth, independence, and required evidence. Model and
reasoning-effort selection remain properties of reviewer profiles so plans do not
become coupled to a particular current model. In the existing GPT-5.6 Sol setup,
selecting `deep` activates the detailed specialist profiles already configured for
high or xhigh reasoning; the plan does not add generic instructions to “think
harder.”

Every mode retains the same finding standard. A surviving finding must identify a
concrete reachable behavior or contract violation, cite evidence, state impact,
and propose a specific correction. Preference-only comments, speculative future
concerns, and objections to an approved decision without new evidence are removed.
Capacity limits may queue reviewers but never silently reduce the approved scope.

### Review feedback and loop completion

Every review item is classified as:

- `Fix`: verified, in scope, and compatible with the approved design;
- `Push back`: incorrect, unsupported, preference-only, or already decided;
- `Escalate`: requires a new design decision or material scope expansion.

For planned work, a `Fix` becomes a concrete plan step. For lightweight work, it
becomes a bounded fix step attached to the current change. Both routes re-run
relevant verification and fresh review; editing the code does not resolve a
finding by itself.

A workflow reaches `finish-branch` only when verification is current for the head
commit and the approved review policy has produced no remaining Must Fix or Should
Improve finding. Agent self-review or an earlier per-task approval does not replace
the final review required by that policy.

### Source-of-truth and compatibility rules

The coordinator is the sole source of truth for cross-phase transitions. Phase
skills may state their entry requirements, work contract, output evidence, and
handoff, but do not reproduce the complete state machine.

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

The always-loaded global file contains only stable policy. The coordinator contains
the workflow state machine, and phase skills contain mechanics. Each rule has one
authoritative home, with short references at boundaries. This limits prompt
repetition and makes drift detectable during review.

### Autonomy and safety

Autonomy applies only within approved local scope. Read-only requests do not become
implementation, and implementation authority does not imply publication or
destructive authority. The coordinator continues safe work without asking at every
step but stops when a new user-owned decision is required.

### Evidence and observability

Progress and completion reports identify the active phase, approved scope, review
policy, reviewers run or skipped, commands observed, and any unverified gap. A
successful tool call, edit, build, or subagent message is intermediate evidence;
completion requires the phase's behavioral contract.

### Failure and re-entry

The workflow may re-enter implementation after verification or review. Re-entry
retains the approved decisions, non-goals, review policy, and exact unresolved
finding. It does not reopen settled design without evidence. Repeated failure of
the same gate triggers diagnosis and then escalation when further progress would
require a changed design or unbounded retries.

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

### Require the user to author all Design Doc prose

This makes authorship and decision ownership identical. It was rejected because
the owner's current workflow settles detailed decisions through dialogue and then
benefits from agent drafting. Separating decision ownership from prose production
preserves architectural control without imposing manual transcription.
