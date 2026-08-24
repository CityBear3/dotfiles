# [Design Doc] Codex Task Orchestrator Subagents

- Owner: Repository owner
- Drafted by: Codex from owner-approved design decisions
- Date: 2026-08-25
- Status: Approved by repository owner on 2026-08-25
- Extends:
  - docs/design/2026-07-29-codex-agentic-engineering-workflow.md
  - docs/design/2026-08-13-codex-design-centered-contract-workflow.md
  - docs/design/2026-08-18-codex-pr-scoped-task-execution.md

## Context and scope

The PR-scoped Task execution design gives each planned Task Contract its own
branch, worktree, verification and review gate, correction loop, and Accepted
result. The current Codex runtime structure does not give that Task boundary its
own orchestration context. The root session runs execute-plan and every
execute-task loop, while implementers and reviewers are direct root subagents
that may not spawn descendants.

This keeps scheduling centralized, but it mixes the detailed histories of every
Task loop into the root context. Independent Tasks can have separate writers and
worktrees, yet their implementation, verification, review, triage, and
correction transitions are still advanced by one conversation. The mismatch
becomes more costly as PR-scoped Tasks run concurrently or re-enter after a
restack, stale result, or human review.

An experiment also showed that Codex collaboration subagents should not be
treated as sessions attached to a Herdr worktree. Directing the engineer to run
codex agents inside a Task worktree is therefore not a reliable way to observe
the subagent that is operating on that worktree. Git activity in the worktree is
the useful observable surface.

This design introduces one non-writing Task orchestrator subagent for each
active planned Task. The root remains the Feature coordinator and global
scheduler. The Task orchestrator owns one execute-task loop and may spawn only
the bounded implementer, verifier, and reviewer leaves selected by the existing
contracts. Planned Task worktrees are managed by Herdr, and their initial pane
shows lazygit for engineer observation. The pane is not an agent session and is
never acceptance evidence.

The lightweight path remains intentionally flatter. Because it has one coherent
Feature/Task Contract and no independent Task DAG, the root runs execute-task
directly and spawns the same bounded leaves without adding a Task orchestrator.

### Goals

- Give each planned Task Contract one isolated orchestration context matching
  its Task PR, branch, worktree, and acceptance loop.
- Keep Feature approval state, Task DAG readiness, PR topology, global capacity,
  stale-result propagation, and integration-only evidence with the root.
- Preserve exactly one source writer per Task worktree and keep the Task
  orchestrator itself non-writing.
- Allow a Task orchestrator to spawn only policy-selected leaf agents under an
  explicit lease granted by the root.
- Replace the fixed six-thread workflow limit with the installed and observed
  Codex subagent capacity.
- Bound each Task loop to one ordinary leaf slot and at most three concurrent
  leaves when the root grants spare capacity.
- Reuse an idle Task orchestrator for re-entry when available without making
  correctness depend on its conversational memory or identity.
- Manage every planned Task worktree through Herdr and show lazygit in its
  initial pane for interactive Git observation.
- Remove codex agents from the workflow's observation instructions.
- Preserve current Task Contract, verification, review, triage, Acceptance,
  publication, and cleanup semantics.

### Non-goals

- Add a Task orchestrator to the lightweight path.
- Turn the Task orchestrator into a source writer or allow more than one writer
  in a Task worktree.
- Let leaf agents spawn further descendants.
- Change Task decomposition, the Task dependency DAG, PR topology, Review modes,
  reviewer selection, the common Acceptance threshold, or stale-evidence rules.
- Treat Herdr, lazygit, pane state, or agent liveness as verification or
  acceptance evidence.
- Attach Codex collaboration subagents to Herdr panes or create independent
  Codex sessions for Task execution.
- Add session addressing, codex queue routing, a machine-readable runtime-state
  schema, or a persistent agent-state file.
- Require every ready Task or every selected reviewer to run concurrently.
- Change the installer's 4, 6, and 8 max_threads machine tiers.
- Automatically push, publish, merge, retarget, delete branches, remove
  worktrees, or perform another external or destructive action.
- Add a custom Git-monitor script when lazygit already provides the intended
  observation surface.

## Overview

The planned path gains one orchestration layer:

~~~text
root — Feature coordinator
  agentic-engineering-workflow
  execute-plan
  global capacity and queue
  Task DAG and PR topology
  Herdr workspace lifecycle
  |
  +-- Task orchestrator A — one execute-task loop
  |     +-- implementer — sole source writer
  |     +-- verifier — check-only
  |     +-- policy-selected reviewers — read-only
  |
  +-- Task orchestrator B — one execute-task loop
        +-- implementer — sole source writer
        +-- verifier — check-only
        +-- policy-selected reviewers — read-only

Each Task worktree = one Herdr workspace
Initial Herdr pane = lazygit observation, not a Codex agent session
~~~

The lightweight path does not add the middle layer:

~~~text
root — Feature and Task coordinator
  execute-task
    +-- implementer
    +-- verifier
    +-- policy-selected reviewers
~~~

The responsibility boundary, rather than the number of files or expected task
duration, selects the topology. A planned path has durable, independently
schedulable Task units and uses Task orchestrators. An eligible lightweight
path has one coherent Task unit and lets the root fill that role.

## Detailed design

### Responsibility ownership

The root remains the only Feature coordinator. It owns:

- workflow path selection and every user approval gate;
- the approved Design Doc, Feature Contract, Implementation Plan, Review
  context, and Review policy identities and currentness;
- Task DAG readiness and the separate PR topology;
- creation and validation of coordination, Task, and integration workspaces;
- the deterministic ready-Task queue and global subagent capacity;
- assignment and revocation of per-Task leaf leases;
- the mapping among Task Contract, Task PR, Task orchestrator identity, Herdr
  workspace, worktree, branch, and current Git evidence;
- validation of returned Task evidence against directly observed Git state;
- dependency release, stale-result propagation, Feature Contract coverage, and
  integration-only verification or review;
- publication and branch-disposition transitions under existing authority
  gates.

One Task orchestrator is bound to exactly one planned Task Contract for its
lifetime and is never reassigned to another Task. It owns one execute-task loop
at a time:

- the task-local implementation, verification, review, triage, and bounded
  correction sequence already defined by execute-task;
- the task's candidate or authoritative mode;
- task-local writer and reviewer dispatch within the root's current lease;
- enforcement of one active writer and the absence of overlapping source
  mutation;
- direct inspection of the assigned worktree's branch, base, head, range,
  diff, status, and attributable state before each gate;
- preservation and reporting of writer, verifier, reviewer, correction, and
  interruption evidence;
- return of Candidate, Accepted, BLOCKED, or Escalate with the complete existing
  execute-task evidence contract.

The Task orchestrator does not edit source, choose a different Task Contract,
change review breadth, release a dependent, decide Feature acceptance, allocate
global capacity, publish work, or clean a workspace. A Task result is not
accepted by the root merely because the Task orchestrator reports it. The root
must resolve the reported branch, base, head, range, and status and reject a
mismatch.

Leaves retain their existing roles:

- the implementer is the only source writer for the Task worktree;
- the verifier is check-only and may create only ordinary test, build, or
  formatter artifacts permitted by its current contract;
- reviewers and adversarial integrators are read-only;
- every leaf is bounded to one selected role and may not spawn descendants;
- no leaf releases dependencies, changes the plan, or claims Feature
  acceptance.

The scheduling adapters are split by level. execute-plan and
dispatching-parallel-agents dispatch already-ready Task orchestrators.
execute-task and agent-teams-driven-development, running in the root for
lightweight work or in a Task orchestrator for planned work, dispatch only the
already-selected leaves permitted by that Task loop and its current lease.

### Task handoff and result

The root gives a fresh Task orchestrator the same complete plain-language Task
handoff currently passed from execute-plan to execute-task. It includes:

- Feature Contract identity, approval and currentness, and assigned clauses;
- exact Task Contract and applicable shared interfaces;
- Review context and complete Review policy;
- task worktree, Herdr workspace, branch, Task PR, planned base, starting head,
  and candidate or authoritative mode;
- responsibility, ownership, discipline, verification, and commit intent;
- contractually significant files, interfaces, ordering, or commands when the
  approved artifacts fix them;
- current capacity evidence, the granted leaf count, and any roles already
  selected for the current wave by execute-task or review;
- prior candidate, acceptance, interruption, correction, or stale evidence
  when the invocation is a re-entry.

The handoff refers to approved artifacts in the coordination workspace rather
than copying unrelated prose. Conversation history is not an authority source.

The Task orchestrator returns the existing execute-task result without weakening
or translating it. The result contains the exact authority, mode, writer state,
task and correction commits, workspace and branch, planned base, merge base,
head, range, changed files, commands and observations, verification, review,
triage, capacity, concerns, gaps, and re-entry condition.

Progress messages and agent liveness help the root observe work, but neither is
a completion signal. Candidate and Accepted retain their existing semantic
meaning. The root updates the DAG or Feature evidence only after the result and
directly observed Git state agree.

### Hierarchy and capacity

Codex agents.max_threads limits concurrent subagents across the complete
descendant tree and does not include the root, as described by the official
[OpenAI Multi-agent guide](https://developers.openai.com/api/docs/guides/responses-multi-agent).
The workflow therefore defines:

~~~text
effective subagent capacity =
  min(configured agents.max_threads, currently observed runtime capacity)

effective total threads =
  root + effective subagent capacity
~~~

The current installed configuration has max_threads set to 8, so its maximum is
eight concurrent subagents plus the root, or nine total threads. This is a
ceiling, not a utilization target. Other installed machines continue to use
their configured tier, and runtime backpressure may lower the effective value.

The workflow removes every independent hard-coded limit of six total threads.
All descendants count against the same effective subagent capacity. A Task
orchestrator consumes one of those slots and each active leaf consumes another.

The approved hierarchy is root to Task orchestrator to leaf. The tracked and
installed agents.max_depth must therefore be 2 rather than 1. Workflow policy,
not a larger platform depth, prohibits leaves from spawning further
descendants.

The root grants leaf capacity rather than permanently reserving it:

- an active Task loop normally receives one leaf slot;
- one Task loop may receive at most three concurrent leaf slots;
- the same per-Task-loop limit applies when the root directly owns a lightweight
  execute-task loop;
- the Task orchestrator may not exceed the current lease even when list_agents
  shows additional free capacity;
- execute-task or review selects the required roles under the approved policy;
  the Task orchestrator requests capacity for those roles without changing
  their order or breadth, and the root grants only the count that the current
  global wave permits;
- the root first gives one slot to each schedulable active Task when capacity
  permits, then grants spare slots in the approved deterministic queue order;
- implementation and correction retain one writer; additional concurrent slots
  are used only for independent check-only or read-only roles;
- insufficient capacity queues work and never drops, substitutes, or weakens a
  selected verification or review gate.

With max_threads set to 8, two fully leased planned Tasks consume all subagent
capacity:

~~~text
Task orchestrator A       1
  leaf lease              3
Task orchestrator B       1
  leaf lease              3
                         --
subagents                 8
root                      1
                         --
total                     9
~~~

Three Tasks may instead make progress with one leaf each and leave two spare
subagent slots:

~~~text
Task orchestrators        3
baseline leaves           3
spare subagent slots      2
                         --
subagents                 8
root                      1
                         --
total                     9
~~~

The root does not dispatch an additional Task orchestrator merely to leave it
waiting when no baseline leaf slot can be granted. A returned or idle Task
orchestrator has no reserved leaf capacity. Any agent the runtime still reports
as live counts against the observed capacity.

### Planned Task worktrees and observation

Every Task worktree created for this planned workflow is Herdr-managed. The root
uses create-workspace with the repository root, exact task branch, explicit
starting ref for a new branch, no focus change, and structured output. It
records the returned Herdr workspace and pane identities and then verifies the
worktree branch, head, base relationship, and status directly through Git.

Failure to create or validate the required Herdr worktree is BLOCKED because the
approved Task execution location does not exist. This workflow does not
silently substitute a raw Git worktree, a Codex-managed worktree, or another
launch mechanism for that Task.

After Task workspace creation, the root starts lazygit in the initial Herdr pane.
The pane exists for the engineer to inspect changes, diffs, and commits in the
actual Task worktree. It is not the Task orchestrator's process, does not carry
the Task handoff, and does not show authoritative agent state.

A lazygit launch failure is non-blocking. The root reports a warning containing
the Task PR identity, worktree path, Herdr workspace and pane identity, attempted
launch, observed error, and the fact that Task execution continues. The pane
remains available as a shell. No monitor script or retry loop is required.

lazygit is interactive and can change Git state if the engineer chooses an
action. Such a change has the same treatment as any other observed worktree
change: the Task orchestrator and root re-resolve the exact state, and
unexplained or stale evidence blocks advancement. The observation pane never
overrides Git inspection.

The workflow removes its instruction to open a Task worktree and run codex
agents. The root remains the source of truth for collaboration-agent identities,
liveness, follow-up, interruption, and closure. The Task-PR-to-agent-to-workspace
mapping remains required scheduling evidence, but is reported in the
coordination session rather than discovered through a worktree dashboard.

### Task orchestrator lifecycle and re-entry

A Task orchestrator runs only while it has a Task turn to advance. Candidate,
Accepted, BLOCKED, and Escalate end that turn and return the exact result to the
root. In particular, Accepted does not start a wait or polling loop through the
remainder of the Feature.

The root retains the Task orchestrator identity together with the durable Task
and Git evidence. When a candidate receives its final base, an accepted Task
becomes stale, a bounded correction is authorized, or human review reopens the
Task, the root prefers to send a fresh re-entry handoff to the same idle Task
orchestrator when that identity remains available. Reactivation is scheduled
under the same global capacity and per-Task lease as fresh work.

Reuse is an optimization, not a correctness requirement. Before every re-entry,
the root and Task orchestrator revalidate:

- current Feature and Task authority and Review policy;
- Task worktree, branch, base, head, merge base, range, diff, and status;
- prior candidate or acceptance evidence and the reason it became incomplete or
  stale;
- the exact bounded correction or authoritative re-entry authority;
- that the earlier writer is inactive and no writer overlaps.

If the previous Task orchestrator identity is unavailable, the root may dispatch
a replacement with the same complete durable handoff after it establishes safe,
attributable workspace state. The replacement does not infer acceptance from
Git history or the earlier agent's memory. If attribution, liveness, or
authority cannot be established, the result is BLOCKED and the workspace is
preserved unchanged.

### State transitions and failure behavior

The new orchestration layer does not add a new Task acceptance state:

~~~text
ready
  |
  v
Task orchestrator active
  |
  +-- Candidate --> idle --> authoritative re-entry
  +-- Accepted  --> idle --> stale/correction re-entry when needed
  +-- BLOCKED   --> preserve state and report exact re-entry condition
  +-- Escalate  --> return to the owning user approval gate
~~~

Existing failure classifications remain controlling:

- a missing design, scope, topology, policy, publication, or other user-owned
  decision is Escalate;
- unavailable or mismatched worktree, branch, ancestry, evidence, capacity,
  agent liveness, or attributable writer state is BLOCKED;
- a changed contract, Task boundary, dependency, PR topology, shared interface,
  or integration obligation is a plan deviation or returns to its approval
  gate;
- a changed base, head, range, controlling authority, dependency, or consumed
  interface makes the affected result stale.

The added layer introduces these concrete operational checks:

- a Task orchestrator result that disagrees with the root's Git observation is
  BLOCKED;
- a Task orchestrator that exceeds or attempts to self-expand beyond its lease
  is a scheduling contract violation and cannot advance the Task;
- a lost Task orchestrator or leaf is not replaced until the prior writer is
  inactive and all in-scope state is attributable;
- capacity rejection is backpressure and retains deterministic queue order;
- Herdr worktree creation or identity failure is BLOCKED;
- lazygit launch failure is only a non-blocking warning.

No failure authorizes reset, clean, stash, amend, rebase, branch deletion,
worktree removal, or another destructive or history-changing action.

## Cross-cutting concerns

### Context, compaction, and identity

The hierarchy reduces root-context pressure, but agent context is never durable
authority. Approved artifacts in the coordination worktree, exact Task
handoffs and results, and directly observed Git state remain the recovery
sources. Context compaction, an unavailable prior Task orchestrator, or a fresh
replacement may increase rereading and repeated checking, but does not weaken a
gate or require a runtime-state file.

Task orchestrator names should be stable and descriptive within a Feature so
the root can report the Task mapping and prefer the same identity for re-entry.
Names are operational labels rather than tracked identifiers or contract
authority.

### Permissions and writer isolation

The Task orchestrator needs collaboration and read-only inspection capability
but does not need source-edit authority. The implementer remains the only agent
authorized to change and commit Task source. Verifier build and test artifacts
remain bounded by its existing check-only contract, and reviewers remain
read-only.

All agents inherit repository guidance and receive the exact worktree and
responsibility boundary. A lease grants concurrency only; it grants no new file,
Git, network, external-write, publication, or cleanup authority.

### Cost and throughput

The design adds one active orchestration context for each active planned Task.
It trades that token and slot cost for Task-local context, concurrent Task
loops, and simpler root aggregation. The per-Task maximum of three leaves and
the configured global capacity bound the burst. The scheduler may leave slots
unused; maximizing concurrency is not a completion criterion.

The fixed six-total workflow limit is not retained as a separate cost control.
Review policy, ready-Task selection, one-writer isolation, per-Task leases, and
deterministic queueing control actual concurrency under the configured ceiling.

### Compatibility and rollout

The existing Task Contract, Task PR, verification, review, triage, Candidate,
Accepted, Feature acceptance, publication, and stale-result contracts remain
compatible. The change is in who owns the planned execute-task loop and how
agents are scheduled.

The Codex bundle adds a Task orchestrator agent profile, permits that role alone
to spawn bounded leaves, changes the tracked agents.max_depth to 2, and updates
the installer expectations for that managed value. Existing custom leaves
continue to prohibit descendant spawning. The tracked max_threads value remains
the standard-tier input and the installer may still replace it with 4, 6, or 8
for the target machine; this design does not hard-code 8 into the source
fragment.

agentic-engineering-workflow, execute-plan, execute-task,
dispatching-parallel-agents, agent-teams-driven-development, create-workspace,
review, and related prompts must agree on the hierarchy, lease, Herdr mapping,
and evidence boundaries. Documentation must remove the fixed six-total rule and
codex agents observation guidance without changing the installer's max_threads
tier selection.

New planned work uses this hierarchy after the updated bundle is installed.
An eligible legacy plan already executing under an approved earlier mechanism
may continue under its preserved legacy authority; the workflow does not
retroactively reinterpret completed or in-flight agent turns.

## Alternatives considered

### Keep every planned Task loop in the root

Rejected for the planned path because it keeps detailed implementation, review,
and correction histories from independent Task PRs in one context and makes the
root advance every Task-local transition. It remains the selected lightweight
topology because there is only one coherent Task and no Task-level parallelism
or isolation benefit to offset another agent layer.

### Add Task orchestrators to both planned and lightweight paths

Rejected because the lightweight path has no separate Task DAG, Task PR set, or
durable cross-Task coordination. The extra handoff, slot, context, failure, and
latency costs would not isolate another workstream.

### Use independent Codex sessions in Herdr worktrees

Rejected because this workflow does not have or need a durable session-address
registry or cross-session messaging contract. Codex collaboration subagents
already provide the required parent-child communication and hierarchy. Herdr
remains the worktree and observation manager rather than the Task
orchestration transport.

### Observe Task subagents through codex agents

Rejected because collaboration subagents are not reliably discoverable as
worktree-local Codex sessions. A dashboard result is also weaker than exact Git
and coordinator evidence. lazygit displays the worktree state the engineer
actually needs to inspect.

### Keep an active Task orchestrator waiting until Feature completion

Rejected because a waiting turn consumes live capacity without advancing the
Task. An idle identity may be reused when available, while durable artifacts and
Git evidence permit safe replacement.

### Always replace the Task orchestrator after Accepted

Rejected because the same available identity can retain useful Task-local
context for human feedback, restacking, stale evidence, and bounded correction.
Reuse remains optional so correctness never depends on identity retention.

### Give each Task a fixed leaf reservation

Rejected because implementation normally needs one leaf while review breadth
varies. Fixed reservations waste capacity and cannot fit two Tasks symmetrically
under smaller configured ceilings. Wave-based leases preserve fairness and use
spare capacity only when selected work exists.

### Allow one Task to use every free leaf slot

Rejected because one deep review could monopolize global capacity and stall
another ready Task. A maximum of three concurrent leaves per Task loop permits
ordinary independent review while keeping cross-Task scheduling bounded.

### Keep a fixed six-total workflow ceiling

Rejected because it duplicates and conflicts with installed max_threads,
changes units by counting the root, and ignores the installer's current
machine-tier selection. Configured and observed subagent capacity already
provide the correct ceiling.

### Block Task execution when lazygit cannot start

Rejected because lazygit is an optional observation surface and not part of the
Task authority, implementation, verification, or acceptance path. Herdr
worktree establishment remains blocking; its optional initial UI does not.

### Add a custom read-only Git monitor

Rejected because it requires a script or code solely to duplicate existing Git
observation. lazygit is already available in the intended environment, and its
absence has a defined non-blocking fallback to the pane's shell.
