# [Design Doc] Codex Task Orchestrator Subagents

- Owner: Repository owner
- Drafted by: Codex from owner-approved design decisions
- Date: 2026-08-25
- Amended: 2026-08-26
- Status: Original design approved by repository owner on 2026-08-25;
  2026-08-26 amendment approved by repository owner on 2026-08-26
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
the bounded implementer, verifier, reviewer, and findings-only integrator leaves
selected by the existing contracts and this amendment. Planned Task worktrees
are managed by Herdr, and their initial pane shows lazygit for engineer
observation. The pane is not an agent session and is never acceptance evidence.

The lightweight path remains intentionally flatter. Because it has one coherent
Feature/Task Contract and no independent Task DAG, the root runs execute-task
directly and spawns the same bounded leaves without adding a Task orchestrator.

Implementation and review of the original design exposed three shared-workflow
gaps. First, adding planned orchestration inputs to one undifferentiated Task
handoff can accidentally make planned-only evidence mandatory for lightweight
work. Second, standalone verification and review remain supported entry points
but need an explicit root-owned scheduling context under the new capacity model.
Third, a high-reasoning reviewer can discover valuable problems while also
proposing a correction that exceeds the current authority. Review findings must
therefore be integrated against the exact design and scope before any writer is
authorized to act.

The 2026-08-26 amendment defines context-discriminated handoffs, preserves a
bounded standalone context, and adds a findings-only review integrator. It uses
deep reasoning to narrow actionability rather than increase review breadth. A
confirmed Design Doc defect returns to the engineer as soon as it is identified;
an implementation mismatch whose answer is already fixed by approved authority
continues through the autonomous correction path.

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
- Separate the shared Task handoff core from planned and lightweight variants so
  one context cannot silently require another context's artifacts.
- Preserve standalone verification and review as root-owned, bounded,
  non-acceptance contexts under the same global capacity ceiling.
- Integrate every non-clean review result before correction, separating the
  observed problem from the reviewer's proposed remedy and attributing whether
  the current range introduced or worsened it.
- Return confirmed missing, contradictory, or materially ambiguous Design Doc
  authority to the engineer before correction or further queued review work.

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
- Run a general review integrator when all selected reviewers are clean.
- Increase Review mode breadth, add speculative perspectives, or treat the
  integrator as another reviewer that invents findings.
- Let a reviewer's proposed correction authorize scope expansion or a Design
  Doc change.
- Treat standalone verification or review as Task, coordinator, or Acceptance
  evidence.
- Add a persistent issue backlog or runtime-state schema for deferred concerns.

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
  |     +-- review integrator — findings-only, read-only
  |
  +-- Task orchestrator B — one execute-task loop
        +-- implementer — sole source writer
        +-- verifier — check-only
        +-- policy-selected reviewers — read-only
        +-- review integrator — findings-only, read-only

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
    +-- review integrator when findings exist
~~~

Standalone verification and review are also root-owned, but are not Task loops:

~~~text
root
  standalone target
    +-- verifier or policy-selected reviewers
    +-- review integrator when findings exist

result = standalone-only advisory evidence
~~~

Review integration is a transition between review and correction, not another
review perspective:

~~~text
reviewer reports
  |
  +-- all clean ------------------------------> CLEAN
  |
  +-- findings --> review integrator
                     |
                     v
                receiving-code-review
                     |
                     +-- Fix
                     +-- Push back
                     +-- Escalate
                           |
                           +-- confirmed design defect --> engineer
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
- reviewers, adversarial integrators, and general review integrators are
  read-only;
- every leaf is bounded to one selected role and may not spawn descendants;
- no leaf releases dependencies, changes the plan, or claims Feature
  acceptance.

The scheduling adapters are split by level. execute-plan and
dispatching-parallel-agents dispatch already-ready Task orchestrators.
execute-task and agent-teams-driven-development, running in the root for
lightweight work or in a Task orchestrator for planned work, dispatch only the
already-selected leaves permitted by that Task loop and its current lease.
The root also dispatches leaves for explicitly standalone targets and other
root-owned coordinator checks. Each request identifies exactly one execution
context; the scheduling adapter does not infer planned artifacts or Task-loop
authority from a generic root-owned request.

### Task handoff and result

Every new-format execute-task handoff has one common core and exactly one
context-specific variant. The common core contains:

- exact authority and currentness, assigned obligations, Review context, and
  complete Review policy;
- workspace, branch, planned base ref and commit, current head, merge base,
  exact range, inspected diff, and Git status;
- responsibility, ownership, discipline, verification, and commit intent;
- attributable commits, prior verification and review, concerns, gaps, and
  re-entry evidence when applicable;
- configured, observed, and effective subagent capacity, live identities, and
  selected or queued roles;
- contractually significant files, interfaces, ordering, or commands only when
  the approved authority fixes them.

The planned variant adds:

- Feature Contract identity, approval and currentness, and assigned clauses;
- exact Task Contract and applicable shared interfaces;
- task worktree, Herdr workspace, branch, Task PR, planned base, starting head,
  and candidate or authoritative mode;
- the bound Task orchestrator identity and current root-granted Task leaf lease;
- prior candidate, acceptance, interruption, correction, or stale evidence
  when the invocation is a re-entry.

The lightweight variant adds the recoverable combined Feature/Task Contract,
the root-owned loop identity, and its root-granted lightweight leaf count. It
does not require a Task orchestrator, Herdr workspace, Task DAG, PR topology, or
another planned-only artifact. A planned handoff without its bound Task
orchestrator or a lightweight handoff containing a pretend Task orchestrator is
rejected rather than normalized into the other variant.

The handoff refers to approved artifacts in the coordination workspace rather
than copying unrelated prose. Conversation history is not an authority source.

The Task orchestrator returns the existing execute-task result without weakening
or translating it. The result contains the exact authority, mode, writer state,
task and correction commits, workspace and branch, planned base, merge base,
head, range, changed files, commands and observations, verification, review,
triage, capacity, concerns, gaps, and re-entry condition. Writer state is
distinct from the verified starting and final Git status, including index,
worktree, and relevant untracked state.

Progress messages and agent liveness help the root observe work, but neither is
a completion signal. Candidate and Accepted retain their existing semantic
meaning. The root updates the DAG or Feature evidence only after the result and
directly observed Git state agree, including the reported final Git status.

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

### Standalone verification and review

A standalone target is a user-requested read-only check outside an approved
planned, lightweight, integration-only, or eligible legacy Task gate. It may be
an exact committed range, the current index and worktree snapshot, or an
explicit bounded fileset. Standalone describes the authority form, not a CLI
process, Codex session, branch type, or physical worktree.

The root owns every standalone target directly. It may dispatch verifier,
reviewer, adversarial-integrator, or review-integrator leaves, and each remains a
direct root descendant. A standalone target never creates a Task orchestrator
and never requires Herdr, a Task Contract, Task DAG, PR topology, or Task lease.

Standalone leaves count against the same configured and observed global
subagent capacity. The root gives one standalone target an explicit target-local
grant of normally one and at most three concurrent leaves. Selected roles beyond
the current grant remain queued in their original order. This prevents an ad hoc
review from monopolizing planned Task capacity without reducing its selected
read-only scope. When the user explicitly prohibits agents, the lead may run the
compatible standalone checks and perspectives sequentially under the same
read-only contract.

The result is labeled `standalone-only`. It may answer the direct user request
but cannot satisfy Task verification, Review policy completion, coordinator
evidence, or Acceptance.

### Review finding integration

Reviewers discover evidence; they do not authorize their own proposed remedies.
When every selected reviewer returns clean, no general integration agent runs.
When any reviewer returns findings, the Task-loop owner or standalone root
dispatches exactly one `review-integrator` before any correction writer is
authorized.

The review-integrator is a read-only, no-descendant leaf using
`gpt-5.6-sol` with `xhigh` reasoning effort. Its deep reasoning is used to
narrow actionability, not to invent findings or expand review breadth. It
receives the exact unchanged target, all available reviewer reports, directly
accessible authority, repository guidance, Review context and policy, prior
triage, current Git evidence, and observed history needed to attribute the
problem. It does not rely on conversation memory or reviewer summaries as
authority.

For each finding, the integrator separately evaluates:

- the concrete observed or reachable problem and whether it reproduces;
- the requirement and exact authority that makes the problem material;
- whether the current range introduced, worsened, merely exposed, or did not
  cause it;
- whether it belongs to the current Task, another approved responsibility, an
  independent pre-existing concern, or an authority gap;
- whether the reviewer's proposed remedy is necessary, proportionate, and
  inside the current scope;
- whether approved authority already determines one correction or whether the
  Design Doc is missing, contradictory, or materially ambiguous;
- the evidence and smallest next transition without deciding implementation
  details.

The integrator deduplicates and reconciles findings but does not return final
workflow authorization. `receiving-code-review` consumes the integrated report
and remains the owner of `Fix`, `Push back`, and `Escalate` classification. A
`Fix` requires a verified current-target problem, an authority-owned and
proportionate correction, and no unresolved design choice. `Push back` covers a
false, unsupported, stale, immaterial, or already-decided objection, including
an excessive proposed remedy even when a smaller in-scope problem remains.

A valid problem outside the current authority is not converted into a current
Task fix. If it is independent of the current change and does not reveal an
authority defect, it is returned as a non-blocking concern and retained in the
current result without creating a persistent backlog. If it exposes missing,
contradictory, or materially ambiguous Design Doc authority, the result is an
`Escalate` whose reason is Design Escalation. This is a routing reason within the
existing Task states, not a fifth acceptance state.

Ordinary implementation findings are integrated once after every selected
reviewer has completed. A report that claims a Design Doc, Feature Contract, or
Task Contract defect receives priority integration: unstarted reviewers remain
queued while one integrator evaluates the authority claim. Already-running
read-only reviewers are not destructively interrupted and their completed
reports remain usable. If the design defect is confirmed, no queued review or
correction begins before returning to the engineer. If it is rejected or reduced
to an implementation problem, the selected reviewer queue resumes and one final
integration covers the complete reports.

An explicit no-agent instruction preserves the existing focused and standalone
lead fallback. The lead applies the same integration contract sequentially and
reports that no independent integrator ran; it may not claim an independent
integration result. A focused lead review does not violate the descendant rule
because no reviewer leaf exists in that fallback. Adaptive or deep independence
requirements remain unchanged.

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
  +-- Escalate  --> return to the owning authority gate
                       |
                       +-- Design Escalation --> engineer
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

Escalation is authority-aware rather than an automatic user interruption. The
coordinator first resolves whether existing approved authority already fixes one
answer. A clear implementation mismatch returns to an autonomous `Fix`; an
unsupported objection returns to `Push back`; and an independent pre-existing
concern may remain non-blocking. Only a missing, contradictory, or materially
ambiguous owner decision returns to the engineer. This preserves autonomy while
making confirmed Design Doc defects visible early.

After an approved Design Doc change, only Tasks whose assigned meaning,
responsibility, shared interface, verification obligation, or depended-on
authority changed become stale, together with their transitive dependents.
Unchanged Accepted Tasks are revalidated for currentness rather than repeated
unconditionally.

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
- a review finding cannot start correction until findings-only integration and
  `receiving-code-review` classification complete;
- a confirmed Design Doc defect stops unstarted review and correction work and
  returns exact evidence to the engineer;
- a missing standalone execution context is a scheduling-contract failure, not
  permission to reinterpret standalone evidence as a Task result.

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
read-only. The review-integrator is also read-only and may not stage, commit,
repair, rewrite authority, or dispatch descendants.

All agents inherit repository guidance and receive the exact worktree and
responsibility boundary. A lease grants concurrency only; it grants no new file,
Git, network, external-write, publication, or cleanup authority.

### Cost and throughput

The design adds one active orchestration context for each active planned Task.
It trades that token and slot cost for Task-local context, concurrent Task
loops, and simpler root aggregation. The per-Task maximum of three leaves and
the configured global capacity bound the burst. The scheduler may leave slots
unused; maximizing concurrency is not a completion criterion.

The general review integrator consumes one leaf slot only when findings exist.
Clean review gates add no integration-agent cost. Priority design integration may
delay queued reviewers briefly, trading one bounded read-only pass for earlier
avoidance of correction work based on defective authority. Standalone targets
use the same maximum of three concurrent leaves so ad hoc work cannot consume
the entire configured ceiling.

The fixed six-total workflow limit is not retained as a separate cost control.
Review policy, ready-Task selection, one-writer isolation, per-Task leases, and
deterministic queueing control actual concurrency under the configured ceiling.

### Compatibility and rollout

The existing Task Contract, Task PR, verification, review, triage, Candidate,
Accepted, Feature acceptance, publication, and stale-result contracts remain
compatible. The change is in who owns the planned execute-task loop and how
agents are scheduled.

The Codex bundle adds Task orchestrator and review-integrator agent profiles,
permits the Task orchestrator to spawn bounded Task leaves, changes the tracked
agents.max_depth to 2, and updates installer inventory and expectations for the
managed assets. The root dispatches lightweight, standalone, and other
root-owned leaves directly. Existing custom leaves continue to prohibit
descendant spawning. The tracked max_threads value remains the standard-tier
input and the installer may still replace it with 4, 6, or 8 for the target
machine; this design does not hard-code 8 into the source fragment.

agentic-engineering-workflow, execute-plan, execute-task,
dispatching-parallel-agents, agent-teams-driven-development, create-workspace,
review, receiving-code-review, and related prompts must agree on handoff
variants, standalone context, hierarchy, leases, Herdr mapping, findings-only
integration, and evidence boundaries. Documentation must remove the fixed
six-total rule and codex agents observation guidance without changing the
installer's max_threads tier selection.

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

### Use one superset handoff for every execution context

Rejected because planned-only Task orchestrator, Herdr, and topology fields can
silently become mandatory for lightweight work, while standalone targets do not
have Task authority at all. A common evidence core plus explicit variants keeps
shared Git and capacity fields aligned without leaking one context's artifacts
into another.

### Duplicate complete planned and lightweight handoff definitions

Rejected because independent copies make common Git, verification, and capacity
evidence drift over time. Only genuinely context-specific authority belongs in a
variant.

### Make standalone checks lead-only

Rejected because standalone review already supports named read-only perspectives
and benefits from independent inspection. A bounded root-owned target preserves
that capability without granting Task or Acceptance authority.

### Let standalone review use every globally free slot

Rejected because an ad hoc deep review could monopolize capacity and starve
ready planned or lightweight work. A target-local maximum of three leaves
preserves deterministic queueing and the existing per-loop fairness principle.

### Apply reviewer findings directly as correction instructions

Rejected because finding evidence and proposed remediation have different
validity. A reviewer may identify a real compatibility problem while proposing a
scope-expanding fix, or may expose an authority defect that no implementation
writer is allowed to resolve. Findings-only integration separates those cases
before mutation.

### Reuse adversarial-integrator for every review result

Rejected because adversarial integration reconciles adversarial perspectives
inside the review gate. General finding integration instead evaluates all review
outcomes against authority, scope, origin, and actionability at the transition
from review to triage. The two roles have distinct inputs and responsibilities.

### Run the general review integrator after clean reviews

Rejected because there is no finding to reconcile or authorize. It would add
latency and capacity cost without improving evidence.

### Let the coordinator silently repair an incomplete Design Doc

Rejected because missing, contradictory, or materially ambiguous design is an
engineer-owned decision. Returning early avoids implementing and reviewing work
against invented authority. Clear implementation divergence from an already
settled design remains autonomous.
