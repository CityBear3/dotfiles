# [Design Doc] Codex Task Lead execution loop

- Owner: Repository owner
- Drafted by: Codex from owner-settled design decisions
- Date: 2026-09-05
- Status: Approved by the repository owner on 2026-09-05
- Search-cache policy revised with owner approval on 2026-09-06: optional
  cross-session discovery sharing; other decisions are unchanged by this revision.
- Delivery: Feature Lead direct local revision on
  `codex/task-lead-execution-loop`; installation and publication are separate
- Extends:
  - `docs/design/2026-08-28-codex-task-loop-optimization.md`
  - `docs/design/2026-08-25-codex-task-orchestrator-subagents.md`
  - `docs/design/2026-08-18-codex-pr-scoped-task-execution.md`
- On approval, supersedes the extended documents only where this document
  explicitly changes Task-loop ownership, session and communication topology,
  role inventory, verification responsibility, correction-review invalidation,
  or model allocation.
  Unchanged authority, isolation, acceptance, publication, and cleanup
  decisions remain applicable.

## Current review state

The repository owner has accepted the lightweight Feature Lead writer and
requested clarification of Task Acceptance ownership. The owner also requires
the engineer to confirm proposed model allocations at Implementation Plan
approval, considering required quality as well as implementation complexity.
Simple code can still demand senior-to-staff-engineer-level quality; complexity
signals must not become an automatic model classifier.

The owner has clarified that each planned Task uses an independent Codex
session in its Herdr worktree and communicates through Herdr. The owner reports
that this communication flow worked in the previous benchmark and considers
the mechanism adequate. Accept that operating experience as the working
premise; do not require another communication experiment or benchmark before
cost discussion.

The owner has accepted proceeding with independent Task sessions, native
Task-local verification/review leaves, and explicit model/effort settings at
session startup. The original single native Feature Lead -> Task Lead -> leaf
tree is withdrawn: Herdr connects session roots; native dispatch stays inside
each session. Task-local transitions stay with the persistent Task Lead, and
Feature-level communication carries compact results with directly accessible
evidence. Existing one-writer and Acceptance requirements remain applicable.

For this workflow-asset revision, the owner requests direct Feature Lead
editing rather than executing the workflow being changed. This is the delivery
method for this revision, not a permanent exception for every Skill change.
The owner approved the consolidated document on 2026-09-05 and authorized the
direct revision. The prior decision record's settled choices are represented
here; authority transfer is complete.

## Context and scope

The current planned workflow makes the root/Feature lead the sole orchestrator
for every active Task. It dispatches a separate implementer, constructs each
current-head Verification Matrix, dispatches verification and every selected
reviewer, integrates findings, triages them, and re-enters the implementer for
correction. This removed the earlier non-writing Task-orchestrator layer, but it
also moved every detailed Task history back into the Feature-level context.

That topology scales poorly when a Feature contains dependency-ready parallel
Tasks, waiting Tasks, or corrections. The Feature lead must retain the state of
every Task and repeatedly cross the same Task boundary. A high-capability
Feature model is then charged for Task-local transitions that do not need
Feature-level reasoning. The problem becomes more important when the default
Feature model is more capable and more expensive.

Retained operational analysis provides directional evidence for changing this
structure. In one language comparison, total model invocation count differed by
2.53 times and cached input accounted for 98.03% of the total-token gap. One
correction loop accounted for 43.57% of the slower run's tokens and 68.22% of
the token gap. A separate direct-root comparison reduced model invocations by
68.6% and total tokens by 69%, although that run also avoided a correction and
therefore cannot attribute the entire improvement to topology. These
observations do not establish a general benchmark result. They do show that
repeated large-context invocations and broad correction waves are first-order
costs worth removing.

This design restores a Task-local coordination context without restoring the
old non-writing Task orchestrator. One `task-lead` is both the sole source
writer and the coordinator for one planned Task. It works in that Task's actual
Herdr worktree, owns its implementation-through-Acceptance loop, and dispatches
only bounded read-only or check-only leaves. The Feature lead retains Feature
authority, dependency release, PR topology, cross-Task invalidation,
integration, and Feature Acceptance.

The lightweight path remains intentionally flat. It has no Implementation Plan,
Task DAG, or dedicated Herdr Task worktree, so it does not create a Task Lead.
The Feature lead is its sole writer and loop owner and dispatches only a
mechanical verification runner and the selected focused reviewer. This removes
the old lightweight-only implementer invocation while preserving independent
gates.

The review inventory is consolidated around distinct decisions instead of one
profile per narrow perspective. Verification becomes command execution rather
than semantic judgment. Model and reasoning-effort allocation is fixed before
execution: in the Implementation Plan for planned Tasks and in the complete
combined contract for lightweight work. Runtime promotion, fallback, and
retry-based model changes are prohibited.

### Goals

- Keep the Feature lead focused on Feature authority, Task-DAG readiness, PR
  topology, cross-Task effects, integration, and Feature Acceptance.
- Give each new-format planned Task one Task Lead that owns its local state from
  implementation through Task Acceptance.
- Combine the planned Task's writer and coordinator in the Task Lead while
  preserving exactly one source writer per Task worktree.
- Run each planned Task Lead in the exact Herdr Task worktree selected by the
  approved plan and validated by `create-workspace`.
- Permit independent ready Tasks to execute concurrently without sharing a
  writer, branch, worktree, or mutable Task-local state.
- Reduce repeated context transfer, Feature-root re-entry, reviewer count, and
  correction-wave breadth without weakening observable Acceptance gates.
- Replace semantic verification with a mechanical `verification-runner` that
  executes only a predeclared matrix and reports observations.
- Retain independent specification and implementation-quality review for
  planned Tasks and one independent focused review for lightweight work.
- Represent exceptional review risk with one parameterized `risk-reviewer`
  contract rather than six permanently distinct profiles.
- Run one `finding-integrator` only when findings are multiple, conflicting,
  authority-sensitive, or otherwise unsafe to triage directly.
- Rerun verification for every new head, but rerun only the finding-owning and
  semantically affected reviewers after a bounded correction.
- Select every Task session and check-leaf model and reasoning effort before
  execution and prohibit runtime model promotion or fallback.
- Allow the Feature lead to inherit the session's default model and reasoning
  effort so a future default-model change does not require plan-schema churn.
- Preserve exact authority, Git identity, fresh evidence, review independence,
  correction bounds, and user-owned publication and disposition decisions.

### Non-goals

- Change how Design Docs, Feature Contracts, Task Contracts, or Implementation
  Plans receive owner approval.
- Let a model resolve a missing architecture, scope, contract, or other
  user-owned decision.
- Change Task decomposition, dependency meaning, PR bases, or dependency
  release from internal Task Acceptance to human merge.
- Create a dedicated Herdr worktree, Task Lead, or Implementation Plan solely
  for eligible lightweight work.
- Retrofit in-flight or approved legacy work to the new topology without new
  authority.
- Let writer self-checks substitute for independent verification or review.
  The Task-loop owner reports Task Acceptance only from the required independent
  evidence; the Feature Lead alone releases dependencies and accepts the Feature.
- Let the verification runner choose commands, assess adequacy, interpret
  semantics, diagnose failures, suggest remedies, or make Acceptance decisions.
- Restrict verification to only test, lint, and format commands when build,
  type-check, integration, smoke, or exact target-state checks are required.
- Reuse an old verification result for a changed head.
- Skip a reviewer whose perspective is affected by a correction.
- Add runtime model escalation, automatic fallback, speculative retry with a
  stronger model, or model choice based on a failed attempt.
- Add a persistent orchestration database, agent-state schema, telemetry
  service, queue, lease system, or benchmark protocol.
- Automatically publish, push, merge, retarget, delete branches, remove
  worktrees, or perform another external or destructive action.
- Make Herdr panes, agent liveness, conversation memory, or model identity part
  of correctness or Acceptance evidence.

### Explicit deferrals

- No benchmark suite or benchmark protocol is part of this Feature. Future
  operational comparisons, if any, remain separate from implementation
  authority and are not shown to benchmark subjects through this design.
- A no-model process-execution primitive is not assumed. The initial
  `verification-runner` remains a tightly bounded low-reasoning subagent; a
  later runtime-native command runner may replace it without changing its
  input, output, or decision boundary.

## Authority and supersession

This document deliberately creates a new durable design instead of rewriting
the approved 2026-08-28 document. The earlier document remains evidence for the
current implementation and for unchanged decisions. After approval, the
following new decisions take precedence:

| Earlier decision | New decision |
| --- | --- |
| Feature lead is the sole orchestrator for planned Task loops | Feature lead is the sole Feature coordinator; one Task Lead orchestrates each planned Task loop |
| Planned implementer is a leaf writer | Planned Task Lead is both sole writer and Task-local coordinator |
| Herdr provides worktree isolation and observation, not independent Task execution sessions | Each planned Task runs an independent Codex session in its Herdr worktree; Herdr carries handoffs and result observation |
| Root dispatches every planned leaf | Task Lead dispatches its check-only and read-only leaves |
| `implementation-verifier` receives a completed matrix and returns a verdict with bounded evaluation | `verification-runner` only executes declared rows and mechanically classifies observations |
| Correction reruns the complete selected reviewer set | Correction reruns the finding owner and every semantically affected reviewer |
| Reviewer specialization is represented by many fixed profiles | Eight logical roles cover execution, normal review, conditional risk, integration, and design alignment |
| Model and effort choices live only in profiles | The approved plan fixes each Task session and selected leaf allocation; Task startup explicitly binds its root allocation |
| Lightweight loop uses a separate implementer leaf | Feature lead is the lightweight writer and loop owner |

The following earlier decisions remain in force unless another approved source
changes them:

- exact Feature, Task, PR, branch, worktree, base, head, range, and status
  identity;
- one writer per Task responsibility and worktree;
- Herdr-owned planned Task workspaces and Git identity checks; Herdr additionally
  hosts Task sessions and carries communication under the owner's clarification;
- role-specific no-history handoffs with direct access to exact authority;
- event-responsive bounded waiting and runtime admission for native leaves
  within each session; Feature-level readiness and ownership remain separate;
- current-head Verification Matrices and source-mutation checks;
- the common finding-admission and Acceptance threshold;
- separation of TDD history from current artifact acceptance;
- Feature-local search caching as non-authoritative navigation;
- user authority over publication, merge, cleanup, and branch disposition.

## Overview

### Planned Feature topology

~~~text
User / repository owner
          |
          v
+---------------------------------------------------------------+
| Feature Lead                                                  |
| default session model + effort                                |
|                                                               |
| owns: Design/Feature authority, plan approval state, DAG,      |
| PR topology, Task release, cross-Task staleness, integration,  |
| Feature Acceptance, publication/disposition boundary           |
+-------------------+----------------------+--------------------+
                    |                      |
          Herdr: Task A ready       Herdr: Task B ready
                    |                      |
                    v                      v
       +------------------------+  +------------------------+
       | Task Lead A            |  | Task Lead B            |
       | independent session A  |  | independent session B  |
       | Herdr worktree A       |  | Herdr worktree B       |
       | sole writer + loop     |  | sole writer + loop     |
       +-----------+------------+  +-----------+------------+
                   |                           |
          +--------+---------+        +--------+---------+
          |                  |        |                  |
          v                  v        v                  v
 verification-runner    reviewers  verification-runner reviewers
    check-only          read-only      check-only       read-only
          |                  |        |                  |
          +--------+---------+        +--------+---------+
                   |                           |
                   v                           v
              Task Accepted               Task Accepted
                   |                           |
                   +------------+--------------+
                                |
                                v
              Feature integration-only verification
                 + design-alignment/risk if selected
                                |
                                v
                        Feature Accepted
~~~

The Feature-to-Task edges are Herdr communication between independent Codex
sessions, not native subagent edges. Each Task-to-check edge is native dispatch
inside that Task session. Feature integration checks are native leaves of the
Feature session. The Feature Lead does not proxy every leaf message. It gives
each Task Lead one complete Task-loop handoff and later consumes exact
attributable Task evidence. Task Leads may progress in parallel only when the
approved DAG and ownership boundaries permit it.

### Task-local loop

~~~text
Task handoff
    |
    v
Task Lead validates worktree, authority, base, head, and ownership
    |
    v
implement -> focused writer checks -> inspect -> commit H1
    |
    v
build current-head Verification Matrix
    |
    v
verification-runner --------------------------------------+
    | PASS                                                 |
    v                                                      | FAIL/BLOCKED
normal reviewers in parallel                              v
    |                                                Task Lead classifies
    +-- spec-reviewer                                     |
    +-- implementation-quality-reviewer                   +-- in-scope fix
    +-- risk-reviewer, only if planned/triggered           +-- operational block
    |                                                      +-- Feature/user escalate
    v
all clean -------------------------------> Task Accepted
    |
    v findings
finding-integrator only when required
    |
    v
Task Lead triage: Fix | Push back | Escalate
    |
    +-- Push back closes every finding -> Task Accepted
    +-- Escalate -> Feature Lead / user-owned decision
    +-- Fix -> bounded correction commit H2
                 |
                 v
          fresh verification on H2
                 |
                 v
 finding owner + semantically affected reviewers only
                 |
                 +--------------------------> repeat boundedly
~~~

### Lightweight topology

~~~text
Feature Lead
  sole writer + lightweight loop owner
        |
        +--> verification-runner
        |
        +--> focused-reviewer
        |
        +--> finding-integrator, only when required
        v
lightweight Task Accepted
~~~

The lightweight path does not use `task-lead`, `spec-reviewer`, or
`implementation-quality-reviewer` merely to resemble planned work. Promotion
to planned work follows the existing promotion and reconciliation authority.

## Logical role inventory

The design has eight logical roles: one independent Task-session root and seven
native leaf roles. A logical role is an authority and behavior contract, not
necessarily a one-to-one filename or runtime implementation. In particular,
`task-lead` is not selected through native `spawn_agent` from the Feature Lead.

| Role | Mutability | Normal owner/caller | Purpose |
| --- | --- | --- | --- |
| `task-lead` | Sole writer in one planned Task worktree | Feature Lead | Implement, commit, coordinate gates, triage, correct, and return exact Task evidence |
| `verification-runner` | Check-only; ignored build/test artifacts only | Task Lead or Feature Lead for lightweight/integration checks | Execute the supplied matrix and report raw observations plus mechanical status |
| `focused-reviewer` | Read-only | Lightweight Feature Lead | One combined lightweight specification and implementation-quality gate |
| `spec-reviewer` | Read-only | Task Lead | Check the exact Task result against approved authority and assigned obligations |
| `implementation-quality-reviewer` | Read-only | Task Lead | Check correctness, maintainability, error handling, tests, and implementation discipline |
| `risk-reviewer` | Read-only | Task Lead or Feature Lead | Apply one explicitly supplied exceptional-risk perspective |
| `finding-integrator` | Read-only | Task Lead or Feature Lead | Deduplicate and validate complex findings without inventing new ones |
| `design-alignment-reviewer` | Read-only | Feature Lead, exceptionally Task Lead | Check composed behavior and shared boundaries against approved design authority |

Leaves under a Task Lead are `verification-runner`, reviewers, and a conditional
`finding-integrator`. They may not spawn descendants. `task-lead` is the only
planned role allowed both to edit and to dispatch Task-local leaves.

The source inventory migrates by responsibility rather than preserving old
filenames:

| Current profile or profiles | Target logical role |
| --- | --- |
| `implementer` | `task-lead` for planned work; no separate lightweight writer profile |
| `implementation-verifier` | `verification-runner` |
| `code-reviewer` | `focused-reviewer` |
| `spec-reviewer` | `spec-reviewer` |
| `code-quality-reviewer`, `test-coverage-reviewer` | `implementation-quality-reviewer` |
| `scope-reviewer`, `code-architect`, `adversarial-api-reviewer`, `adversarial-performance-reviewer`, `adversarial-robustness-reviewer`, `adversarial-tests-reviewer` | parameterized `risk-reviewer` |
| `adversarial-integrator`, `review-integrator` | `finding-integrator` |
| `design-alignment-reviewer` | `design-alignment-reviewer` |

## Detailed design

### Feature Lead responsibilities

The Feature Lead remains the single authority for Feature-wide coordination.
It:

- owns the shared working model and exact approved Design Doc, Feature
  Contract, and Implementation Plan identities;
- resolves the dependency DAG separately from PR topology;
- releases only dependency-ready, ownership-disjoint Tasks;
- establishes or validates Task workspaces through `create-workspace`;
- starts or safely reuses the corresponding independent Codex session through
  Herdr, binding the approved Task model, effort, and Task Lead role contract;
- supplies one complete, compact Task-loop handoff to each Task Lead;
- records active, pending, Accepted, blocked, and stale Task states;
- propagates predecessor changes and other cross-Task invalidation;
- owns integration-only verification, composed-tree review, Feature Acceptance,
  and return to the user;
- may create the optional Feature-local search cache for costly discoveries
  worth reusing across independent sessions, remaining its only writer;
- retains all publication, merge, cleanup, and disposition gates.

The Feature Lead does not choose Task-local private implementation details,
construct routine Task matrices, dispatch routine Task reviewers, or retain
every correction transcript in its active context. It receives exact evidence
and may inspect its referenced sources when Feature-level decisions require it.

The Feature Lead's model and reasoning effort are not written into the
Implementation Plan. They are the session's effective defaults. This keeps the
Feature coordinator compatible with future default-model changes and avoids a
recursive requirement for the Feature Lead to plan itself.

### Planned Task Lead responsibilities

One Task Lead owns exactly one active Task responsibility. It:

- works only in the handoff's exact Herdr Task worktree and branch;
- is the sole process authorized to edit Task source, tests, configuration, and
  responsibility-scoped documentation;
- validates Task authority, base, head, range, status, and one-writer ownership
  before acting;
- applies the declared development discipline, including TDD when applicable;
- performs focused writer-side checks, inspects the diff, creates the authorized
  Task commit, and records the resulting head;
- constructs and invalidates the current-head Verification Matrix;
- dispatches only the plan-selected runner, reviewers, and conditional
  integrator with role-complete handoffs;
- retains Task-local prior reports, findings, triage, correction deltas, and
  affected-reviewer decisions;
- performs only authorized bounded corrections and creates separate correction
  commits unless exact authority says otherwise;
- returns `Candidate`, `Accepted`, `BLOCKED`, or `Escalate` with exact
  attributable evidence to the Feature Lead.

The Task Lead may not change Feature scope, Task responsibility, public or
shared interfaces, Design authority, PR topology, dependency meaning, Review
policy, publication state, or cleanup state unless the approved authority
already delegates that exact choice. A missing or contradictory material
decision returns `Escalate`; selecting Astra never grants design authority.

The Task Lead persists across ordinary verification, review, and correction
turns. Reusing that context avoids a fresh implementer invocation and repeated
authority reconstruction. Persistence is an efficiency mechanism, not
correctness evidence: every changed head and every re-entry still receives
direct Git and authority validation.

### Task Lead handoff

The Feature Lead supplies one complete handoff containing:

- Task identity, responsibility, purpose, expected result, constraints, and
  non-goals;
- exact current Design, Feature Contract, Task Contract, and Implementation Plan
  identities plus directly readable paths;
- assigned clauses, shared interfaces, adjacent-Task obligations, and delegated
  local decisions;
- material property, reliable verification oracle, and development discipline;
- exact coordination directory, Herdr workspace/pane/agent routing identity,
  worktree, branch, Task PR, starting ref, planned base, current head, merge
  base, range, and status;
- candidate or authoritative mode and applicable predecessor evidence;
- writer commit intent and any fixed message, files, signatures, ordering, or
  commands;
- preselected logical roles, risk perspectives, correction invalidation rules,
  and effective model/effort allocation;
- existing attributable commits, reports, concerns, and re-entry state.

When useful, include reusable findings directly or relevant entries from an
existing optional search cache, with their sources and applicability conditions.
Use the handoff directly for one-off sharing. An absent or stale cache is not
a handoff gap and requires no miss report. The Feature Lead creates a cache
only when further cross-session reuse justifies it, not as a routine artifact.

The handoff omits unrelated conversation history, completed work from other
Tasks, and unassigned unchanged Design prose. Exact sources remain directly
available. Every new Task Lead starts without inherited parent conversation.

### Task Lead result

The Task Lead returns one status and evidence envelope:

- `Candidate`: plan-authorized early implementation exists, but the final PR
  base or authoritative gates are not yet current;
- `Accepted`: exact current head passed fresh verification and all required
  reviewer evidence is present, with findings resolved by verified corrections
  or current justified pushbacks;
- `BLOCKED`: an operational or evidence condition prevents safe progress and
  has an exact re-entry condition;
- `Escalate`: progress requires a user-owned decision, new authority, material
  scope expansion, or plan deviation.

The envelope includes Task and workspace identity, base/head/range/status,
commits and changed files, discipline evidence, completed Verification Matrix,
runner report, every applicable reviewer report, integration and triage when
present, correction history, carried-forward reviewer evidence with reasons,
concerns, any useful cross-session discovery candidates, and gaps. The Feature
Lead consumes the envelope directly and does not replace it with an
unverifiable prose summary.

Reporting Task `Accepted` belongs to the Task-loop owner, who is also its
writer: the Task Lead for planned work and the Feature Lead for lightweight
work. This is an evidence-backed gate result, not unilateral writer
self-approval. It requires the independent runner and applicable reviewer
reports, including explicit non-invalidation evidence for carried-forward
review coverage. For planned work the Feature Lead validates the returned
evidence against the current Task and dependency state before releasing a
dependent. Only the Feature Lead issues Feature `Accepted`.

### Lightweight loop

The Feature Lead owns lightweight implementation directly. Before editing, it
must hold the complete recoverable combined Feature/Task Contract and the fully
selected lightweight Review policy. It remains the sole source writer, creates
the responsibility-scoped commit, constructs the matrix, dispatches the runner
and focused reviewer, and handles bounded correction.

The lightweight route fixes its leaf model and effort allocation when the
combined contract and Review policy become complete, before the first edit. It
does not use runtime promotion or fallback. If implementation exposes planned
work, the current lightweight state is preserved and promoted through the
existing design, planning, and reconciliation path.

### Verification runner

The Task-loop owner constructs the Verification Matrix. Each row contains:

- stable row ID;
- observable obligation and exact authority;
- exact non-mutating command or target-state check;
- working directory and required environment facts;
- expected observable result;
- mechanical mismatch classification of `FAIL` or `BLOCKED`.

Rows may include tests, non-mutating format checks, lint, type-check, build,
package/workspace checks, integration or smoke commands, and exact Git or
target-state observations. The runner may not add, delete, reorder, repair, or
reinterpret a row. Missing or contradictory inputs are `BLOCKED`.

Immediately before execution, the runner records the exact head, base, range,
status, changed files, and mutation boundary. It executes rows fail-fast in
declared order and records command/check, exit code, bounded stdout/stderr,
expected result, observed result, and row result. It then confirms that the
index, tracked source, and in-scope files were not mutated. Normal ignored
build/test artifacts are allowed only within the declared boundary.

The final status is mechanical:

- `PASS`: every required row matched and the source-mutation invariant holds;
- `FAIL`: a row observably mismatched or source/index mutation occurred;
- `BLOCKED`: a required input, tool, environment, or target could not be
  resolved safely, and no mismatch conclusion is justified.

The runner does not judge whether the matrix is sufficient, whether behavior is
correct, or what should be changed. The Task Lead interprets a failed
observation against authority. Reviewers independently inspect verification
adequacy within their perspectives.

### Normal review

Planned Tasks normally run `spec-reviewer` and
`implementation-quality-reviewer` concurrently after fresh runner `PASS` on an
unchanged head.

`spec-reviewer` owns exact approved behavior, scope, non-goals, assigned shared
interfaces, and observable contract coverage. It does not enforce private
implementation preferences delegated to the writer.

`implementation-quality-reviewer` combines the former code-quality and
test-coverage responsibilities. It owns concrete correctness risks,
maintainability consequences, responsibility placement, error handling,
readability, test adequacy, test isolation, and implementation discipline. It
must keep behavior/test evidence and maintainability claims distinct, but a
single report prevents two reviewers from rereading the same diff and tests.

Lightweight work uses one `focused-reviewer` that combines the applicable
specification and implementation-quality checks against the complete combined
contract. It remains independent from the Feature Lead writer.

Every reviewer uses the common finding threshold. A finding must be applicable
to the actual artifact and consumer, identify an approved requirement or
realistic reachable behavior, cite concrete evidence, state a material
consequence, and propose a proportionate remedy. Preference-only, speculative,
second-order, artifact-inapplicable, and optional-polish claims are dropped at
the source.

### Conditional risk review

`risk-reviewer` replaces the fixed scope, architecture, API, performance,
robustness, and adversarial-test profiles. Each invocation receives exactly one
named perspective and its trigger, authority, threat or failure model,
applicable surface, expected evidence, and stop condition. Separate perspectives
remain separate invocations when independent scrutiny matters.

Supported perspective classes include:

- scope and responsibility boundary;
- architecture and dependency direction;
- API misuse and error-model safety;
- measurable repeated cost or resource behavior;
- robustness, partial failure, concurrency, and recovery;
- test strength, mock divergence, fault coverage, and shared state.

The Implementation Plan selects required and conditional perspectives from the
Task's real risk. The Task Lead may activate only a predeclared conditional
perspective when its exact trigger becomes true. A new unplanned risk that
would materially expand review returns to the Feature Lead for plan or design
authority; it is not silently added.

`design-alignment-reviewer` remains separate because it compares the composed
implementation with durable Design, Feature, and Task authority across
boundaries. It normally runs at Feature integration or for a Task explicitly
marked as changing a shared boundary. It is not a routine reviewer for every
local Task.

### Finding integration and triage

`finding-integrator` replaces both the adversarial integrator and general review
integrator. It runs only when at least one of these conditions holds:

- multiple reviewers report potentially overlapping findings;
- reports conflict about facts, authority, origin, severity, or remedy;
- a finding claims that Design, Feature, or Task authority is defective;
- a proposed remedy may cross Task scope or change a shared interface;
- reproduction or origin attribution is materially non-trivial.

A single concrete finding with clear authority, evidence, origin, and bounded
remedy may proceed directly to Task Lead triage. The integrator is read-only,
receives every relevant source report, validates problem and remedy separately,
deduplicates, resolves contradictions, and never invents findings.

The Task Lead classifies an admitted finding as `Fix`, `Push back`, or
`Escalate`. It may authorize only an in-scope correction whose contract meaning
is already settled. A confirmed authority defect or missing user-owned decision
returns immediately to the Feature Lead and then to the user as required.

### Correction invalidation

Every correction creates a new head and therefore always requires:

1. exact current base/head/range/status validation;
2. a rebuilt current-head Verification Matrix;
3. fresh `verification-runner` execution;
4. fresh review from the finding-owning reviewer; and
5. fresh review from every other semantically affected perspective.

The Task Lead records an impact map from each correction file and behavior to
reviewer perspectives. A prior clean report may be carried forward only when:

- its exact prior head and report are available;
- the correction does not change its owned behavior, tests, interface,
  responsibility, authority, verification strategy, or evidence;
- affected callers and shared boundaries are bounded and unchanged; and
- the Task Lead records concrete evidence for non-invalidation.

Uncertainty requires rerun. Public/shared interface, schema, error model,
concurrency, security, data integrity, migration, dependency, test strategy, or
authority changes invalidate every perspective that can observe that surface.
The Feature Lead receives the impact map and carried-forward evidence in the
Task result.

This replaces the prior rule that every correction reruns the complete selected
reviewer set. It does not reuse an old verdict as the verdict for a new head;
it carries forward only explicitly non-invalidated perspective evidence while
fresh runner and affected-perspective evidence bind the new head.

### Model and reasoning-effort allocation

The Feature Lead uses the session's default model and effort. Every independent
Task session and native leaf allocation is resolved before implementation
starts and remains fixed for that Task or lightweight loop.

As of this document's date, official OpenAI documentation describes
[GPT-6 Astra](https://developers.openai.com/api/docs/models/gpt-6-astra) as the
most capable model for the hardest end-to-end work,
[GPT-5.6 Sol](https://developers.openai.com/api/docs/models/gpt-5.6-sol) as the
flagship model for complex professional work, and
[GPT-5.6 Luna](https://developers.openai.com/api/docs/models/gpt-5.6-luna) as
optimized for cost-sensitive, high-volume workloads. The allocation below uses
those current product roles but remains a repository-owned workflow decision.
Runtime availability is validated separately and never inferred from the
documentation.

| Logical role | Default allocation | Selection rule |
| --- | --- | --- |
| `task-lead` | `gpt-5.6-sol` / `high` | Default proposal; engineer confirms suitability for the required quality and risk at plan approval |
| `task-lead` Astra allocation | `gpt-6-astra` / `high` | Propose when required quality, reasoning demands, or risk justify it; engineer confirms at plan approval |
| `verification-runner` | `gpt-5.6-luna` / `low` | Mechanical matrix execution only |
| `focused-reviewer` | `gpt-5.6-sol` / `high` | Lightweight independent gate |
| `spec-reviewer` | `gpt-5.6-sol` / `high` | Planned specification gate |
| `implementation-quality-reviewer` | `gpt-5.6-sol` / `high` | Planned implementation and test-quality gate |
| `risk-reviewer` | `gpt-5.6-sol` / `xhigh` | One preselected high-risk perspective per invocation |
| `finding-integrator` | `gpt-5.6-sol` / `high` | Conditional evidence integration |
| `design-alignment-reviewer` | `gpt-5.6-sol` / `xhigh` | Cross-authority and composed-boundary review |

The following are signals for proposing a Task Lead allocation, not automatic
selection rules or an exhaustive checklist:

- the Task introduces or changes a public or shared interface consumed by more
  than one Task or subsystem;
- correctness depends on concurrency, ordering, recovery, idempotency,
  migration, security, data integrity, or compatibility reasoning;
- the owned change spans subsystems with non-local invariants or significant
  emergent interactions;
- the reliable oracle is non-deterministic, model-based, fault-based,
  property-based, hardware-dependent, or otherwise requires substantial
  interpretation;
- the implementation must preserve several interacting constraints for which a
  local edit cannot be evaluated independently.

Task size, file count, expected token count, or a testing technique alone does
not decide the model. Small or straightforward code may still require
senior-to-staff-engineer-level judgment and quality in interface design,
maintainability, correctness, or integration. The proposal must account for
those demands. Sol selection does not lower the required quality or Acceptance
threshold; Astra selection does not guarantee that they are met. A missing
architecture or contract decision returns to the engineer rather than being
delegated to a stronger model. `max` effort is not used initially.

The plan records a Feature-wide default allocation table and only Task-specific
overrides, including selected `risk-reviewer` perspectives. Alongside each Task's
effective allocation, present the required quality, relevant complexity/risk,
and the quality/cost rationale. The engineer explicitly confirms these
allocations as part of Implementation Plan approval, before execution. Do not
add a second approval gate or repeat the question at Task startup. Until that
approval, allocations are proposals; afterward, they remain fixed for execution.
The plan also records the effective profile or runtime binding for the model and
effort. Because a Task Lead is an independent Codex session root, its startup
must apply the approved Task model and effort explicitly; the Feature Lead's
global Astra/xhigh default must not silently become every Task's allocation.
Use explicit `--model`, `model_reasoning_effort`, and
`plan_mode_reasoning_effort` startup overrides. Both effort settings use the
approved Task effort so entering Plan mode cannot silently select another
allocation. The latter is a distinct override, as documented in the
[Codex configuration reference](https://learn.chatgpt.com/docs/config-file/config-reference).
Herdr passes these Codex arguments after `--`; the launch also resolves the
exact Task working directory without changing the global configuration.
A custom subagent profile alone does not bind a separately launched root
session. The launch handoff must also supply the same reviewed Task Lead role
contract and its directly readable source, independent of model choice. The
Task Lead confirms its workspace, role, and effective allocation before edits;
missing, conflicting, or unobservable bindings do not permit implementation.
If the selected model or effort is unavailable at dispatch,
the Task is `BLOCKED`; no runtime substitute, promotion, or fallback is allowed.

For lightweight work, the complete combined contract fixes the runner and
focused-reviewer allocations before editing. There is no Task Lead allocation.

### Context and token boundaries

Every new leaf starts with `fork_turns="none"` or the runtime-equivalent empty
history. Its handoff contains only role-owned evidence and direct authority
references. The Task Lead keeps the Task-local state that previously returned
to the Feature Lead after every phase.

The following controls are mandatory:

- send exact authority identities and relevant clauses, not the Feature
  conversation transcript;
- keep complete sources readable instead of copying unrelated prose into every
  handoff;
- batch independent reads and searches while preserving judgment-dependent
  ordering;
- treat any supplied applicable search-cache entries only as navigation;
- retain one Task Lead across bounded correction when safe;
- run normal reviewers concurrently after fresh verification;
- avoid `finding-integrator` for all-clean or simple single-finding review;
- rerun only correction-invalidated reviewers;
- report exact evidence once and reference it rather than paraphrasing it into
  competing formats.

An optional search cache does not require routine lookups, empty files, or miss
reports. Changed sources or assumptions require rechecking; its absence or
staleness never blocks progress. If created, retain it under the existing
ignored plan-artifact lifecycle without using it as acceptance evidence.

No fixed token limit may cause material authority or evidence to be omitted.
When the Task Lead context becomes unsafe or incomplete, re-entry uses the
exact worktree, Git, authority, and report envelope rather than conversational
memory.

### Independent Task sessions and Herdr communication

Each planned Task Lead runs as the root of an independent Codex session in its
Task worktree. The Feature Lead coordinates those sessions through Herdr. A
Task Lead is not a native subagent of the Feature Lead. Herdr also provides
worktree and pane observation:

~~~text
Herdr-managed environment
  Coordination workspace / pane
    Codex session F: Feature Lead
          |
          | Herdr-mediated handoff and result observation
          +--> Task worktree A / pane
          |      Codex session A: Task Lead A
          |        +-- native verification-runner
          |        `-- native reviewers / conditional integrator
          |
          `--> Task worktree B / pane
                 Codex session B: Task Lead B
                   +-- native verification-runner
                   `-- native reviewers / conditional integrator
~~~

An independent Codex session and a named persistent Herdr session are distinct
concepts. This architecture does not require a separate Herdr server per Task.

The installed Herdr 0.8.2 CLI and its bundled Skill establish that an existing
shell pane can start Codex through `agent start --kind codex`; `agent prompt`
submits input, `agent read` reads terminal output, and `agent wait` observes
lifecycle states. The owner confirms that Herdr-mediated communication worked
during the previous benchmark. That prior operating experience is sufficient
for selecting the mechanism here. The current documentation pass did not
create an agent or worktree, and does not claim a new live validation.

Herdr lifecycle status assists routing and completion observation; Task
Acceptance still comes from the Task Lead's attributable Git, verification,
and review evidence. Resolve explicit pane and agent identities before sending
a handoff. A successful start means readiness, not Task completion. Wait
results are not prompt-specific receipts, and `done`, a timeout, or terminal
silence cannot establish completion or a stopped writer. Before accepting a
result, match its Task, authority, base, and head to the expected assignment.
If terminal reads omit required evidence, retrieve the exact source or request
a complete report from the same Task session before proceeding. Do not resend
implementation blindly or introduce another queue or state schema.

Task-local runner, reviewer, and integrator invocations use native subagents
with empty inherited history and no descendants. They do not receive separate
Herdr sessions. The Task Lead owns their dispatch and waits locally; the
Feature Lead observes Task-level results, cross-Task effects, and escalations,
not every local phase. Feature integration and lightweight checks use native
leaves directly under the Feature Lead.

Runtime admission applies within the relevant native session tree. Task roots
are not children in the Feature session's `max_threads` or `max_depth` tree;
this does not imply unlimited aggregate service capacity. A rejected local
spawn stays pending and is retried after relevant progress or availability
evidence without dropping a selected gate. Herdr startup failure stays pending
or blocked with its cause; first inspect the target to avoid duplicate writers.
Bounded, event-responsive waiting preserves user interruption and progress by
other ready Tasks. No new global scheduler, fixed concurrency quota, or lease
mechanism is introduced.

### Failure, interruption, and re-entry

Task correctness never depends on a Task Lead's identity or memory. An
observation timeout does not terminate the independent session or authorize
another writer. On an interruption, lost response, or incomplete commit, the
Feature Lead first resolves whether the assigned session and its leaves are
still active and whether the current Task result is merely unobserved. Before
resuming or replacing an uncertain writer, it:

1. confirms the prior writer is inactive before another writer can start, and
   resolves active checks against the old head before permitting new edits;
2. directly inspects the worktree, branch, head, status, commits, and exact
   range;
3. attributes every edit and commit to the Task responsibility;
4. validates current authority, model allocation, pending gates, and reports;
5. gives the safely idle session a current re-entry handoff, or launches a
   replacement at the same approved allocation with one complete no-history
   handoff.

A replacement never cleans, resets, rebases, amends, discards, or silently
restarts work. Ambiguous or unattributable state is `BLOCKED` or `Escalate`.

A runner or reviewer interruption leaves that role pending. The Task Lead may
retry only after confirming the prior leaf is inactive. A model availability
failure is not a reason to change the model. A correction that repeats the same
problem without progress stops with the observed attempts and an exact
escalation.

## State model

### Planned Task states

~~~text
Pending
  |
  | dependencies accepted
  v
Ready -> WorkspaceReady -> Implementing -> CandidateHead
                                           |
                                           v
                                       Verifying
                                      /    |     \
                                  PASS    FAIL   BLOCKED
                                    |       |       |
                                    v       |       +--> Blocked
                                Reviewing   +--> Correcting
                               /    |    \
                           CLEAN FINDINGS BLOCKED
                             |      |         |
                             v      v         +--> Blocked
                         Accepted Triage
                                    |
                        +-----------+-----------+
                        |           |           |
                       Fix      Push back    Escalate
                        |           |           |
                        v           v           +--> Feature/user
                    Correcting   Accepted
                        |
                        +--> new head -> Verifying
~~~

`Candidate` mode remains available only when the plan permits work before the
final PR base is materialized. Candidate evidence cannot release a dependent or
satisfy Feature Acceptance.

### Feature states

The Feature Lead releases a dependent only from an internally `Accepted`
predecessor. Any accepted predecessor head change marks dependent evidence
stale according to the approved plan. After every required Task is current and
Accepted, the Feature Lead composes integration evidence, invokes selected
integration reviewers, and alone issues Feature `Accepted`.

## Cross-cutting concerns

### Quality and independence

Combining writer and Task coordinator reduces handoffs but creates a risk that
the writer controls its own gates. The following boundaries preserve
independence:

- reviewer selection, risk triggers, models, and effort are fixed before
  implementation;
- the Task Lead cannot weaken or omit a selected gate;
- runner and reviewers are separate no-history subagents with no source/index
  write authority; only declared ignored check artifacts may be generated;
- reviewers inspect the exact diff, surrounding code, tests, authority, and
  raw runner evidence directly;
- finding integration is read-only and separate from Task Lead triage;
- the Feature Lead receives raw attributable reports and invalidation decisions,
  not only the Task Lead's verdict;
- only the Feature Lead can release dependencies and accept the Feature.

### Permissions and external effects

The Task Lead has workspace-write authority only within its assigned Task
worktree and responsibility. Runner write access, when technically required,
is limited to ordinary ignored build/test artifacts; reviewers and integrators
are read-only. No role gains publication, network, credential, merge, cleanup,
or destructive authority from this topology.

Dependency downloads and external host access continue to require the
applicable repository authority and runtime permission. Model choice does not
change filesystem or network permission.

### Observability and recoverability

Each Task result exposes:

- exact Task/workspace/branch/base/head/range/status identity;
- writer commits and changed files;
- matrix rows and raw runner observations;
- reviewer selection, reports, and effective model/effort bindings;
- integration and triage decisions;
- correction impact maps and carried-forward evidence;
- pending roles, interruptions, concerns, and re-entry conditions.

Herdr hosts Task sessions and provides communication. Observed agent identity
and lifecycle can support routing, but pane state and
`done` do not establish Task correctness or Acceptance. Exact Git and gate
evidence remain required. Existing Task handoffs and result evidence retain
the routing identities; no new persistent runtime state schema is introduced.

### Cost and throughput

The Feature Lead uses the session default for cross-Task synthesis without
copying that allocation into every Task. Sol/high is the default Task proposal;
the engineer confirms Sol/high or Astra/high from the required quality,
reasoning demands, risk, and cost at plan approval. Mechanical verification uses
Luna/low; risk and design-alignment review use Sol/xhigh. No expected percentage
of Sol or Astra Tasks is imposed.

Invocation reduction comes from removing the planned implementer/Task-loop
split, keeping corrections in one Task Lead context, consolidating ordinary
quality/test review, using one parameterized risk role only when triggered,
making findings integration conditional, and narrowing correction reruns.
Parallel ready Tasks and reviewers improve wall time without changing semantic
readiness or gate order.

These are structural expectations, not a promised percentage improvement. The
source implementation is accepted through contract and test evidence rather
than a benchmark target.

Compare cost over one accepted Task or Feature, including initial context,
implementation, verification, review, communication, and correction. Total
tokens alone are not a monetary measure: separate uncached input, cached input,
and output, then apply the relevant model and service-tier rates. Keep quality
and elapsed time alongside that cost; neither a cheaper model nor fewer
reviewers proves better efficiency at the required quality.

Independent Task sessions add initial instructions, authority discovery, and
handoff processing. They can reduce the Feature Lead's repeated Task-local
reasoning and keep unrelated Task history out of other sessions. Worktree or
session creation alone does not prove a token saving. Herdr terminal operations
do not themselves require model reasoning, but model invocations around input
submission, repeated status reads, and result interpretation contribute cost.

The selected cost controls are:

- keep the Feature Lead involved at Task dispatch, accepted-result evaluation,
  cross-Task effects, and genuine escalation; do not relay every implementation,
  verification, or correction transition through it;
- retain each Task Lead through bounded correction, avoiding repeated session
  startup and reconstruction of the same authority;
- use Task-local native leaves, without an independent Herdr session for every
  check role;
- keep full evidence attributable and directly available, while ordinary
  Feature-level reports carry the result, relevant concerns, and evidence
  references instead of repeated terminal transcripts;
- preserve the approved reviewer consolidation, conditional integration, and
  correction invalidation rules instead of lowering the quality threshold.

These controls do not set fixed token quotas or predict exact savings. Use
existing normal-run usage evidence when available; a new benchmark or telemetry
system is not required to proceed with this design discussion.

### Compatibility and rollout

Rollout is atomic at the workflow-asset level. New profile names and new Skill
references must be installed together so no workflow can select a missing or
retired role.

For this revision, the Feature Lead directly edits the repository's workflow
assets and performs their local validation. Do not dispatch this migration
through `execute-plan`, independent Task sessions, or a separate implementer;
the workflow being changed is not its own execution harness. The agreed design
and an explicit bounded edit/validation sequence remain the basis for the
change, but separate workflow-generated Feature/Task Contracts and execution
sessions are not prerequisites for this direct revision. This owner-selected
delivery method does not change the normal approval or execution rules that
the resulting Skills define for future work. Direct lead validation is not
represented as independent reviewer evidence or workflow Task Acceptance.

Expected implementation surfaces include:

- `codex/AGENTS.global.md` Feature/Task orchestration guidance;
- `codex/agents/` native leaf profiles;
- reusable Task Lead instructions supplied to independent roots, with their
  installation mapping;
- `codex/skills/create-plan/` model-allocation and reviewer-selection contract;
- `codex/skills/create-workspace/` Herdr Task pane/session startup boundary;
- `codex/skills/execute-plan/` Feature/Task ownership and dispatch;
- `codex/skills/execute-task/` Task Lead-owned planned loop;
- `codex/skills/execute-lightweight-task/` direct Feature Lead writer loop;
- `codex/skills/verify/` command-runner boundary;
- `codex/skills/review/` consolidated selection, integration, and invalidation;
- dispatch, receiving-review, README, installer inventory, and tests that name
  the affected roles.

The installer already derives managed agent inventory from safe
`codex/agents/*.toml` sources. Removed profiles therefore use the installer's
normal managed-removal plan, while unrelated destination agents remain
untouched.

An Implementation Plan approved before rollout keeps its exact approved
topology and model allocation rules. It is not silently migrated. New-format
plans approved after rollout use this design. Eligible legacy work keeps its
exact authority. A Task already running under one topology finishes or stops
under that topology; it never changes coordinator or model mid-loop.

Rollback restores one previously coherent asset revision and applies it only to
new work. Active Tasks are not moved between topologies. Publication and local
installation remain separately authorized operations.

## Validation strategy

Implementation must provide fresh evidence for these properties:

- inventory/reference checks prove every selected logical role resolves and
  every retired role reference is removed or intentionally legacy-scoped;
- role and startup checks prove Task Lead write authority, leaf read/check-only
  authority, no descendant permission for leaves, shared Task Lead instructions
  for both models, exact Task working directory, and explicit model/effort
  bindings including Plan mode;
- plan validation establishes model proposals with required-quality and risk
  rationale, engineer confirmation during plan approval, default-plus-override
  recording, no Feature Lead binding, and fail-closed unavailable allocations;
- orchestration validation covers Herdr dispatch between independent roots,
  native Task-local leaves, local pending-spawn retention, one writer,
  ready-Task progress, and dependency release only from Accepted evidence;
- verification tests prove declared-row-only execution, fail-fast order, raw
  output capture, mutation detection, and no semantic diagnosis;
- review tests prove planned normal reviewer selection, lightweight focused
  review, parameterized risk perspectives, and conditional integration;
- correction tests prove fresh runner execution for every new head, mandatory
  finding-owner rerun, affected-reviewer rerun, evidence-backed carry-forward,
  and uncertainty-to-rerun behavior;
- interruption tests prove safe writer replacement and attributable re-entry,
  including timeout/lifecycle observations that do not imply Task completion or
  authorize duplicate writers;
- installer end-to-end tests prove the new inventory is installed and retired
  managed profiles are removed without touching unrelated agents;
- documentation checks prove README, Skill, agent, and Design references agree.

Production behavior changes should use focused causal TDD when an executable
oracle exists. Schema, inventory, reference, and documentation-only properties
may use the more reliable structural or integration checks. A benchmark is not
an Acceptance requirement for this Feature.

## Risks and mitigations

### Task Lead self-coordination weakens independence

The Task Lead writes and triages its own Task. This is mitigated by plan-fixed
gates, independent no-history reviewers, raw evidence return, read-only finding
integration, and Feature-level dependency release. The Task Lead cannot issue
Feature Acceptance.

### Selective correction review misses a regression

The affected set can be underestimated. The design therefore always reruns the
runner and finding owner, defines broad invalidators for shared or risky
surfaces, requires evidence for every carried-forward perspective, and treats
uncertainty as invalidation.

### Parameterized risk review loses specialist clarity

One generic profile can become vague. Each invocation therefore carries exactly
one named perspective, trigger, threat/failure model, surface, evidence shape,
and stop condition. Independent perspectives remain separate invocations.

### Task session startup uses the wrong defaults

Independent Codex sessions can otherwise load the global model and effort.
Bind each session to its engineer-approved allocation at startup and validate
that effective allocation with the Task identity. Both Sol and Astra launches
must receive one consistent Task Lead role contract. No Task switches models
or effort mid-loop.

### Task Lead context grows during a difficult correction loop

Persistent context can itself become costly or stale. Reports remain compact
and attributable, correction is bounded, repeated non-progress escalates, and a
safe replacement can reconstruct state from Git, authority, and exact reports
without inheriting conversation.

### Feature Lead becomes a costly lightweight writer

The lightweight path uses the default Feature model for implementation. It also
removes one writer-agent invocation and is limited to work that passes the
strict lightweight eligibility gate. Any work whose risk, ambiguity, or
coordination requirements make this inappropriate must promote to planned work
before further edits.

### Selected models are unavailable

Availability can differ across runtimes. The plan records the exact allocation,
and dispatch fails closed with a re-entry condition. Automatic fallback would
make cost, behavior, and evidence differ from the approved plan and is therefore
not allowed.

## Alternatives considered

### Keep the Feature Lead as the sole planned Task orchestrator

Rejected because parallel and waiting Tasks force detailed Task state and
correction histories into the most expensive shared context. It also requires
the Feature Lead to re-enter every local phase.

### Restore a non-writing Task orchestrator plus implementer

Rejected because it recreates two Task-level contexts, an extra handoff, and an
extra wait path. Making the Task Lead the sole writer removes that duplication
while retaining Task-local coordination.

### Make Task Leads native children of the Feature session

Rejected after the owner clarified the established Herdr operating model.
Independent Task roots align session boundaries with isolated Task worktrees
and allow an explicit plan-approved allocation per root. Native dispatch is
retained inside each session for its bounded check roles. This choice is not
an assertion that separate sessions alone reduce total cost.

### Give every runner and reviewer an independent Herdr session

Rejected because these bounded read/check-only roles need neither another Task
writer nor another persistent coordination context. Native leaves retain their
independence and fixed allocation with less startup and transport overhead.

### Use Task Leads for lightweight work

Rejected because lightweight work has no Task DAG, Implementation Plan, or
dedicated Herdr Task worktree. The Feature Lead can implement it directly and
retain independent runner/reviewer gates with fewer model invocations.

### Keep every current specialist reviewer profile

Rejected because overlapping diff, authority, and test reads multiply cached
input and correction-wave cost. The consolidated roles preserve independent
decisions while parameterizing exceptional risk.

### Use only one general reviewer

Rejected because planned specification compliance and implementation quality
are materially different decisions. Keeping them independent gives better
fault isolation and permits concurrent review.

### Always run the finding integrator

Rejected because clean reviews and a single clear finding do not need another
full-context model invocation. Integration is reserved for ambiguity,
conflict, authority questions, or non-trivial attribution.

### Rerun every reviewer after every correction

Rejected because a bounded test-only or local correction need not invalidate
every perspective. Fresh verification plus explicit semantic invalidation
preserves quality while avoiding a complete second wave.

### Trust every old reviewer verdict after a bounded correction

Rejected because a changed head may invalidate a perspective. Only exact prior
evidence accompanied by a concrete non-invalidation argument can carry forward,
and uncertainty requires rerun.

### Let the verification runner choose or diagnose commands

Rejected because command selection, adequacy, diagnosis, and remedy are
semantic decisions. Mixing them into the runner spends reasoning tokens and
blurs accountability. The Task-loop owner and reviewers retain those decisions.

### Remove the verification subagent entirely

Deferred rather than rejected. A deterministic process runner is preferable
when the runtime provides one with equivalent isolation and evidence capture.
Until then, Luna/low provides a bounded adapter without granting semantic
authority.

### Promote Sol to Astra after failure

Rejected because runtime promotion makes cost and behavior depend on an
implementation attempt, complicates recovery, and can hide an authority or
planning defect. The engineer confirms the allocation once at plan approval
from required quality, reasoning demands, risk, and cost.

### Pin the Feature Lead in every Implementation Plan

Rejected because the root session exists before plan creation and is governed
by the user's effective defaults. Plan entries apply only to dispatched
Task sessions and selected native leaves, not the existing Feature root.

### Use Astra/xhigh or max for every difficult role

Rejected because capability and reasoning effort have separate costs. Task
allocations are justified by required quality and risk and confirmed by the
engineer at plan approval; narrower risk and design review use Sol/xhigh.
`max` is left unused until separate evidence justifies it.

## Unresolved questions

No material design question remains open in the discussed session, hosting,
startup-allocation, or cost-control branches. Herdr communication feasibility
is accepted from the owner's previous operating experience; no new benchmark
is required. Exact monetary savings are not established or an Acceptance gate.

The full Design Doc is approved. The Feature Lead performs the bounded asset
revision directly as requested; local installation, publication, live session
launch, and cleanup are not part of that revision's execution authority.
