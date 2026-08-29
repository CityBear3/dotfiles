# [Design Doc] Codex Task-loop optimization

- Owner: Repository owner
- Drafted by: Codex from owner-settled design decisions
- Date: 2026-08-28
- Revised: 2026-08-29
- Status: Minimal Task-executor Skill split approved by the repository owner on
  2026-08-29
- Approved draft SHA-256:
  `100d80bfda76377ca923fbaaa7f86b2983c4581c4dfe1432c412d3e2b1875069`
- Prior approved document SHA-256:
  `102f55e61810cdb4faef93a3d58c24f497807ebc47c3df6ff5cc4d54b24e9b8e`
- Repository baseline: `d32ec49957eb419dd12095b69c196eb0128619bb`
- Extends:
  - `docs/design/2026-08-25-codex-task-orchestrator-subagents.md`
  - `docs/design/2026-08-18-codex-pr-scoped-task-execution.md`

## Context and scope

The current planned workflow assigns one non-writing Task orchestrator to each
Task Contract. That orchestrator owns the Task-local implementation,
verification, policy-selected review, triage, and bounded correction loop while
the root owns Feature coordination and global capacity.

The topology is sound, but the internal loop repeats more work than each phase
needs. The implementer can run broad checks that the authoritative verifier
immediately repeats. The verifier mixes mechanical command evidence with
semantic diff inspection. Every role receives a large common handoff. A Task
may hold spare leaf capacity during phases that can use only one leaf. A small
correction causes every reviewer to rediscover the complete Task range from the
beginning even when exact prior review evidence is available.

The verifier profile also currently uses `gpt-5.6-sol` with `high` reasoning.
The repository owner has observed that `gpt-5.6-luna` with `max` effort can stop
responding in long verification workflows and wants verification to be a
shorter mechanical phase. The design therefore keeps Sol for reliability,
reduces its initial verifier effort to `medium`, and narrows the role so later
evaluation may safely compare `low`.

Operational use of the first optimization exposed another class of repeated
work. New subagents still inherit parent conversation by default unless dispatch
explicitly disables it. Waiting guidance permits frequent short polls. The
implementer has no explicit boundary for batching independent discovery, so
sequential tool turns can be spent on reads and searches whose results do not
affect one another. Multiple phases can repeat the same repository or external
search. Finally, immutable TDD execution history can be confused with the
current artifact's Acceptance evidence, causing a historical RED discrepancy to
block an otherwise correct current target without a reachable defect or a
material evidence gap.

This revision extends the original design with context-isolated dispatch,
event-responsive waiting, decision-aware implementer batching, a feature-local
search cache, explicit separation of historical TDD discipline from current
Acceptance, and a comparable operational evaluation boundary. It revises the
same durable authority rather than adding a precedence-bearing addendum.

Before implementation, the owner refined that evaluation boundary to reduce
benchmark-only work and make the observation more representative. The earlier
synthetic clean path and forced-correction path are replaced by one minimal
fixed product specification implemented independently in Rust, Go, and Kotlin.
Each language executes the real complete Task loop; findings and corrections
are observed rather than scripted. Three language repositories are reused only
as Git containers: frozen before and after branches start at the same base while
run state, evidence, sessions, and caches remain isolated by side.

Operational use then exposed an authority-boundary problem inside the shared
`execute-task` Skill. It accepts both a Task-orchestrator-owned planned variant
and a root-owned lightweight variant, so a consumer must repeatedly interpret
which Task-loop owner and lifecycle apply. The selected correction is deliberately
narrow: keep existing `execute-task` for planned and eligible legacy work, add
one `execute-lightweight-task` Skill for the root-owned lightweight loop, and
change only the lightweight calls in `agentic-engineering-workflow`. The
coordinator, planned topology, check-only phases, completion modes, and benchmark
design remain shared or unchanged.

This design preserves the Task orchestrator topology and acceptance gates while
optimizing the phases inside one Task loop. It supersedes the earlier generic
lease rule only where it permits spare Task leaves outside the reviewer wave,
and it refines the correction-review behavior without changing which reviewers
the approved Review policy selects.

### Goals

- Keep one Task orchestrator responsible for one complete planned Task loop.
- Prevent planned and lightweight Task-loop authority from sharing one executor
  contract by moving the lightweight variant to one dedicated Skill.
- Establish one concise current-head Verification Matrix as the authoritative
  verifier handoff and result spine.
- Limit writer-side checking to candidate-quality evidence and remove repeated
  full-suite verification from the writer phase.
- Make the verifier a fail-fast, mechanically ordered, check-only executor that
  does not perform semantic code review.
- Preserve only the Git identity and mutation checks the verifier must observe
  directly while keeping full Task Git ownership with the Task orchestrator.
- Give writers, verifiers, reviewers, and integrators compact role-specific
  handoffs and reports.
- Expand a Task's leaf lease only for the independent reviewer wave and revoke
  the expansion before integration, triage, or correction.
- Review corrections delta-first while requiring the same reviewer set and a
  fresh full-current-head verdict.
- Keep verification on `gpt-5.6-sol`, initially at `medium` reasoning effort.
- Preserve fresh evidence, Review breadth, Acceptance, and correction authority.
- Start every new Task orchestrator and leaf without inherited parent
  conversation and give it one complete role-specific handoff.
- Replace routine short polling with normally five-to-ten-minute bounded waits
  that return early on mailbox or completion events.
- Batch independent implementer discovery and mechanical post-edit checks while
  preserving every judgment-dependent and TDD ordering boundary.
- Prevent repeated planned-lifecycle searches through an ignored,
  feature-local `search-cache.md` with source-aware invalidation.
- Keep historical TDD discipline evidence distinct from the evidence that
  establishes current-head Acceptance.
- Compare the complete before and after Task loops in minimal equivalent Rust,
  Go, and Kotlin workloads with dual-layer call counts, token use, wait
  behavior, elapsed time, repeated searches, and an unchanged
  completion-quality bar.

### Non-goals

- Replace the Task orchestrator with root-driven phase checkpoints.
- Change Task decomposition, the Task dependency DAG, PR topology, Review
  modes, reviewer selection, or the common Acceptance threshold.
- Skip a policy-selected reviewer after correction or reuse a prior verdict for
  a new head.
- Remove fresh verification after implementation or correction.
- Remove verifier target or mutation-invariant checks.
- Add a persistent Verification Matrix, runtime state file, finding identifier,
  or machine-readable coordination schema.
- Add a persistent telemetry service or hide underlying operations behind a
  single top-level call count.
- Change `agents.max_threads`, its installer tiers, `agents.max_depth`, or the
  existing maximum of three concurrent Task leaves.
- Add a Task orchestrator to the lightweight path.
- Split the complete coordinator, verification, review, workspace, or completion
  lifecycle into parallel planned and lightweight Skill families.
- Rename `execute-task` or redesign its planned and eligible-legacy callers.
- Treat conversation history, `search-cache.md`, agent identity, or liveness as
  correctness or Acceptance authority.
- Weaken the requirement to use TDD for applicable implementation work or claim
  TDD when an intended RED was not observed.
- Embed benchmark scenarios or run results as permanent runtime Skill prose.
- Force a review finding or correction solely to make benchmark paths match.
- Treat cross-language absolute duration as Task-loop overhead, or require a
  large sample application when a minimal equivalent workload exposes the same
  workflow behavior.
- Install the changed bundle before baseline measurement or make installation,
  publication, or branch disposition implicit in Design Doc approval.

### Explicit deferrals

None.

## Overview

The Task loop keeps its existing state transitions but each phase consumes and
returns evidence at its own boundary:

```text
Task orchestrator
  |
  +-- implementer
  |     red/focused green + necessary local type/build check
  |     pre-commit ownership and diff evidence
  |
  +-- derive current-head Verification Matrix
  |
  +-- verifier
  |     fail-fast mechanical checks + completed matrix
  |
  +-- reviewer wave
  |     semantic review, temporarily parallel when root capacity permits
  |
  +-- findings integration and triage when needed
  |
  +-- bounded correction
        new head -> fresh matrix/verify -> delta-first fresh review
```

The root still selects ready Tasks and controls global subagent capacity. The
Task orchestrator still validates complete Task identity and owns the Task-local
sequence. Leaves remain bounded, and only the implementer writes source.

The Skill boundary around those unchanged state transitions is:

```text
agentic-engineering-workflow
  +-- planned --------> execute-plan -> Task orchestrator -> execute-task
  +-- lightweight --------------------------------------> execute-lightweight-task

execute-task              planned and eligible legacy; Task-orchestrator-owned
execute-lightweight-task  lightweight only; root-owned
verify/review/triage      explicit-input check phases shared by both paths
finish-branch             existing completion modes shared by both paths
```

The router still owns classification and cross-phase transitions. The split is
at the Task executor only; it does not create a second coordinator family.

## Detailed design

### Minimal Task-executor Skill boundary

`execute-task` removes its lightweight handoff variant and rejects lightweight
authority. Its planned Task-orchestrator binding, planned cache input,
Verification Matrix, reviewer wave, bounded correction, and eligible-legacy
compatibility otherwise retain their existing meanings.

The new `execute-lightweight-task` accepts the recoverable combined
Feature/Task Contract, root-owned loop identity and capacity grant, exact Task
PR target, Review context and policy, discipline, verification obligations, and
prior attributable lightweight evidence. It rejects a Task orchestrator, Herdr
workspace requirement, Task DAG, PR topology, planned `search-cache.md`, or any
other planned-only authority. Its correction loop retains the same reviewer set
and fresh-current-head gate without passing through `execute-plan`.

`agentic-engineering-workflow` continues to classify requests and prepare both
routes. Its planned calls still enter `execute-plan` and `execute-task`; only
lightweight implementation, correction, and re-entry calls change to
`execute-lightweight-task`. Shared exact-target `verify`, `review`,
`receiving-code-review`, profiles, fallback prompts, `create-workspace`, and
`finish-branch` contracts are not split. Direct wording or inventory references
may change only where the new executor identity must be observable.

This boundary is complete when neither executor contains the other route's
handoff variant or claims the other route's loop owner. Shared check phases
remain safe because they receive an explicit target and authority rather than
selecting a lifecycle route.

### Evidence ownership by phase

The Task orchestrator owns complete Task identity and orchestration evidence:

- approved authority and Review policy;
- Task workspace and branch;
- planned base, merge base, head, exact range, diff, status, changed files, and
  attribution;
- writer isolation and task/correction commits;
- current global capacity, Task lease, queues, and phase transitions;
- assembly and currentness of the Verification Matrix;
- preservation of prior review and triage evidence for correction re-entry.

The implementer owns candidate production evidence only:

- the bounded implementation and changed files;
- an observed red failure and focused green result for behavior changes;
- focused tests for the owned responsibility;
- a local type or build check only when needed to commit a coherent candidate;
- pre-commit diff, ownership, authority coverage, and unrelated-state
  inspection;
- the responsibility-scoped task or correction commit.

The verifier owns mechanical current-target evidence:

- the supplied target head and range identity;
- completion of every applicable Verification Matrix row;
- exact command status and expected-versus-observed comparison;
- range and whitespace checks;
- pre/post tracked and in-scope source state and mutation attribution;
- a `PASS`, `FAIL`, or `BLOCKED` verdict for the unchanged target.

The verifier does not own semantic implementation judgment. It does not decide
whether the design is appropriate, whether code organization is maintainable,
whether tests prove the correct behavior beyond their recorded command result,
or whether the diff satisfies a contract not expressed in the current matrix.
Those questions remain with the applicable reviewers and findings integrators.

Each reviewer owns only its selected perspective. Review remains the semantic
gate for contract alignment, correctness, maintainability, scope, architecture,
and test quality. Integrators own reconciliation of complete finding reports,
not source discovery or correction authorization.

### Context-isolated dispatch and complete role handoffs

Every newly spawned Task orchestrator and leaf uses explicit
`fork_turns="none"`. Parent conversation is neither inherited execution context
nor correctness evidence. Re-entry through an existing idle identity still
receives a fresh complete handoff and revalidates every current input.

Complete means complete for the receiving role, not a copy of the Task owner's
entire state. Each handoff contains:

- the role's exact purpose, responsibility, allowed actions, and prohibited
  overlap;
- applicable authority identity, currentness, assigned clauses, constraints,
  and non-goals;
- workspace, Git target, source-state boundary, and direct source locations;
- required observations, output schema, stop conditions, and re-entry evidence;
  and
- capacity and descendant restrictions when the role schedules work.

The Task orchestrator additionally receives the complete Task-local authority,
Review context and policy, scheduling grant, phase state, and prior attributable
evidence needed to own the whole loop. Each leaf receives only the subset that
can change its bounded decision, while exact sources remain directly readable.
A leaf or replacement resolves Git and authority from those sources; it never
uses identity, prior conversation, or a plausible summary as proof.

### Current-head Verification Matrix

The Task-loop owner constructs one in-memory matrix after the candidate head and
exact Task range resolve and before dispatching the verifier. The matrix has one
row per observable obligation and records four concepts:

| Field | Meaning |
|---|---|
| Obligation | The exact Task, integration-only, lightweight, legacy, or standalone condition being proved |
| Command or check | The bounded observation that supplies evidence |
| Expected observation | The result required by the controlling authority |
| Non-match category | Whether a mismatch is an observed `FAIL` or an evidence/environment `BLOCKED` |

The matrix is plain-language handoff evidence rather than a stored schema. It
may group obligations observed by one command, but it must not hide an
unobserved requirement or duplicate the same obligation across competing
formats. Contractually fixed commands remain exact. When the authority leaves a
route open, the Task-loop owner or verifier may select a compatible standard
check and record the choice in the matrix.

The matrix is bound to the exact target head and controlling authority. A
commit, range change, contract change, or material command-route change makes it
stale and requires a rebuilt matrix. The completed matrix is the verifier's
compact report spine and is passed to review as verification evidence.

### Verifier target and Git boundary

The Task orchestrator resolves the full planned Task identity before verifier
dispatch. For a clean isolated planned Task PR, the verifier independently
confirms only the facts needed for valid command evidence:

- the expected workspace, current head, and exact committed range still match;
- the pre-check index, worktree, and relevant in-scope source state are clean as
  required by the supplied target;
- the changed-file inventory and `git diff --check` agree with that target;
- the same head and source state remain after verification, except for recorded
  normal ignored build or test artifacts.

It does not repeat branch selection, topology resolution, complete attribution,
or semantic diff inspection already owned by the Task orchestrator.

A standalone index/worktree snapshot or another explicitly dirty bounded target
retains a fuller pre/post fingerprint because staged, unstaged, and untracked
state is part of that target's identity. This exception does not broaden a clean
planned verifier's responsibilities.

### Fail-fast verification order

The verifier executes only applicable rows and checks in this order:

1. target identity and required clean-state precondition;
2. exact range, changed-file, and whitespace/diff checks;
3. documented non-mutating format check;
4. focused behavior tests;
5. build or type check;
6. lint;
7. owning package, workspace, or full tests;
8. integration, smoke, browser, API, or snapshot checks;
9. final head and mutation-invariant comparison.

A conclusive failure stops subsequent dependent or more expensive checks. The
verifier records the unrun matrix rows and why they are stale or unnecessary
rather than spending time on checks that cannot change the verdict.

Independent mechanical commands may be executed in one bounded batch when:

- each command's status and output remain separately attributable;
- the commands do not depend on one another's output;
- no result requires model judgment before the next command starts;
- failure reporting still identifies the first conclusive mismatch; and
- the final mutation-invariant check runs after the batch.

Batching is an execution optimization, not a weaker evidence form. If the tool
cannot preserve per-command results, commands remain separate.

### Decision-aware implementer batching

The implementer's first discovery stage may place independent authority reads,
repository searches, relevant file reads, and Git inspection into one bounded
programmatic tool batch. Each underlying operation remains separately
attributable. Before starting a new search, the role checks the applicable
`search-cache.md` entry supplied by or located through the handoff.

Batching stops whenever one result changes the choice, scope, authorization, or
input of the next action. In particular, approval and escalation decisions,
edits, semantic diagnosis, and any final validation that depends on earlier
output remain separate stages. A tool batch is not permission to collapse
several judgment points into one opaque operation.

For TDD work, RED execution, production editing, GREEN execution, and refactor
remain ordered stages. No GREEN check is launched speculatively before the edit
that it is meant to validate. After the edit and focused GREEN have completed,
independent mechanical post-edit checks may be grouped when their individual
results remain attributable. The verifier's fail-fast order and stronger
current-head boundary remain unchanged.

### TDD history and current Acceptance

TDD discipline evidence records an execution history: the test written for one
behavioral viewpoint, the observed pre-production RED and its reason, the
subsequent edit, focused GREEN, and any refactor while green. The implementer
must still attempt and report this sequence honestly and must never claim TDD
when the intended RED was not observed.

Acceptance evidence answers a different question: whether the current exact
head and range satisfy the approved contract with adequate current tests,
verification, and review. A later-discovered difference in the historical RED
failure category is immutable process evidence; rerunning a test after the
production change cannot repair or recreate that history.

An unrepairable historical discipline gap is disclosed explicitly. It does not
by itself become an Acceptance blocker when current evidence establishes no
reachable defect, no material coverage or evidence gap, and no material
contract deviation. It becomes `Escalate` when the controlling authority makes
that missing historical evidence material, or when it exposes a user-owned
scope, quality, or verification decision. A reachable current defect or missing
current evidence follows the ordinary finding, verification, and correction
path rather than being classified only as history.

### Feature-local search cache

Planned work stores reusable discovery at:

```text
docs/plans/YYYY-MM-DD-<feature>/search-cache.md
```

The file is ignored, workspace-only, and non-authoritative. It prevents the
Feature lead, Task orchestrators, and leaves from repeating the same repository,
Git, documentation, runtime, or external search, including useful searches that
found no result. The Feature lead is the only writer. Task orchestrators and
leaves read it and return new cache candidates in their reports. Within one
Task turn, an orchestrator may pass an attributable returned result directly to
the next role without waiting for file integration; cross-loop persistence
remains Feature-lead controlled.

Before searching, a role looks for an entry whose purpose, scope, and source
identity match the current question. Each entry records:

- search purpose and scope;
- source identity, including an applicable URL, version, Git ref, range, or
  path;
- observation date or repository identity;
- positive and useful negative results; and
- explicit reuse and invalidation conditions.

Repository entries become stale when their relevant ref, range, paths, search
scope, or controlling authority changes. Versioned documentation is keyed by
its stable identity. Mutable external information is rechecked when currentness
can materially affect a decision or obligation. An observed contradiction
invalidates the affected entry. The cache never replaces fresh Git and
authority resolution, required mechanical verification, or a policy-required
reviewer's independent assessment of the current target.

`search-cache.md` has the same lifecycle as `implementation-plan.md`. It stays
in the coordination worktree while publication, feedback re-entry, or
disposition evidence may need it and is retired when removal of that exact
worktree is authorized. It is not recoverable from Git unless the owner
separately chooses archival.

### Role-specific handoffs and reports

The Task-loop owner retains the complete durable evidence, but sends each leaf
only the subset that changes that role's decisions.

The implementer message contains owned responsibility, applicable authority
clauses, preserved boundaries, discipline, candidate target, commit intent, and
writer-side checks. Review scheduling, completed verification output, and
unrelated contract prose are omitted.

The verifier message contains target identity, the Verification Matrix,
command-environment facts, required source-state boundary, and verdict schema.
The full Review policy is not copied unless one of its exact constraints changes
the verification route.

Each reviewer message contains the verified target, current diff, changed
files, its applicable authority clauses and selected perspective, Review context
and policy, the completed Verification Matrix, and relevant prior triage.
Unrelated authority remains directly readable but is not eagerly copied.

An adversarial or general findings integrator receives the unchanged target,
complete source reports relevant to that integration, applicable authority,
Review context and policy, and prior triage needed for origin and remedy
assessment. It does not receive writer procedure or verifier scheduling detail
unless a finding depends on that evidence.

Writer reports return candidate and commit evidence, verifier reports return the
completed matrix and verdict, reviewer reports return perspective-specific
findings or clean status, and integrators return reconciled evidence. The
scheduler preserves these reports without translating them into another wrapper.

### Phase-scoped Task leaf leases

Every active Task loop starts with one baseline leaf slot. That slot is enough
for the sole implementer, verifier, findings integrator, or correction writer,
which run as distinct phases.

After verification passes and review has selected at least two independent
source reviewers, the Task orchestrator may request a reviewer-wave expansion.
The root may temporarily grant up to three total concurrent Task leaf slots or
the smaller currently available capacity. The same rule applies to the
root-owned lightweight Task loop. Reviewers beyond the grant remain queued in
the approved deterministic order.

Only policy-selected source reviewers use the expansion. A Task orchestrator
cannot use it to overlap implementation with verification, run duplicate
verifiers, start an integrator early, or accelerate correction. The expansion
is revoked after the source-reviewer wave completes or when review exits early
for a priority authority assessment. Adversarial and general integrators then
run under the baseline one-leaf lease.

The root remains the only lease authority. A free runtime slot is availability,
not permission. An unavailable expansion increases review latency but does not
block while the baseline queue can still make progress.

### Event-responsive bounded waiting

An owner waiting for a Task orchestrator or leaf uses one bounded
`wait_agent` interval of normally five to ten minutes. Mailbox updates,
completion notifications, or steered user input return control before that
bound. The owner may perform independent useful work before entering the wait,
but does not replace the bounded wait with repeated short polls.

Live-agent inspection remains required at scheduling and phase boundaries,
after an early return, and before interruption or replacement. A shorter wait
is justified only by a nearer explicit deadline, teardown, or interruption
boundary and records that reason. Terminal `Candidate`, `Accepted`, `BLOCKED`,
or `Escalate` results end the current Task-orchestrator turn; neither the
orchestrator nor its scheduler waits or polls after returning one.

### Delta-first correction review

A correction changes the reviewed head from `H1` to `H2`. All earlier verdicts
are stale for `H2`, so the ordinary correction sequence still requires:

```text
bounded correction commit
  -> new current head H2
  -> rebuilt Verification Matrix
  -> fresh verifier PASS for H2
  -> same complete policy-selected reviewer set
  -> fresh H2 review verdict
```

Review traversal is optimized, not review authority. Each reviewer receives:

- the prior reviewed head `H1` and current head `H2`;
- the full current target `base..H2`;
- the correction delta `H1..H2`;
- the exact corrected finding, prior reviewer report, integrated assessment,
  and triage decision;
- the fresh completed Verification Matrix for `H2`;
- the same applicable authority, Review context, policy, and perspective.

The reviewer starts with the correction delta, confirms whether the finding is
resolved, and follows affected callers, tests, interfaces, responsibilities,
and obligations. It may use the earlier report only as navigation evidence.
It returns a new perspective result that covers the full current target and is
bound to `H2`.

The reviewer switches to ordinary full traversal when:

- the correction changes files or behavior outside its bounded authorization;
- a public or shared interface, responsibility boundary, schema, error model,
  concurrency, security, dependency, or test strategy changes;
- the planned base, controlling authority, or Review policy changes;
- prior reviewer or triage evidence is incomplete or was blocked;
- the correction exposes a new finding outside the expected surface; or
- the reviewer cannot establish that earlier inspected areas remain unaffected.

Reviewer selection is never recalculated from the delta. The same complete
policy-selected set reruns. Selective reviewer invalidation remains outside this
design.

### Failure and re-entry behavior

The existing `PASS`, `FAIL`, `BLOCKED`, `CLEAN`, `FINDINGS`, `Candidate`,
`Accepted`, and `Escalate` states remain unchanged.

- A stale or incomplete Verification Matrix is `BLOCKED`, not permission to
  improvise missing obligations.
- A mechanical mismatch with a required observation is `FAIL`.
- A missing environment, command, or target guarantee is `BLOCKED`.
- Verifier source mutation is `FAIL`; uncertain mutation ownership is
  `BLOCKED`.
- A missing reviewer-wave expansion queues reviewers under the baseline lease;
  it is `BLOCKED` only when the required queue cannot make progress.
- A stale prior correction report disables delta-first optimization but does not
  remove the required fresh review; reviewers use ordinary full traversal.
- A correction that changes contract meaning returns to the existing authority
  or plan gate rather than being treated as a larger delta.

No optimization authorizes cleaning, resetting, amending, rebasing, discarding,
publishing, or installing live assets merely because an intermediate phase
passes. Installation occurs only at the explicit rollout boundary after
baseline evidence is complete.

## Cross-cutting concerns

### Context and token use

The Verification Matrix removes repeated prose and gives verification one
current-head evidence format. Role-specific messages stop every leaf from
receiving the complete Task orchestration state. Delta-first review reuses
earlier evidence without reusing its verdict. Context-isolated dispatch prevents
the parent transcript from being copied into every new role. Decision-aware
batching removes unnecessary model turns, and `search-cache.md` prevents the
same discovery from being repeated across the planned lifecycle.

These are context reductions, not permission to omit exact authority. Every
role keeps direct access to the source artifacts and expands its inspection when
current evidence requires it. A cache hit or compact handoff remains operational
context, never proof that the current Git target or authority still matches.

### Reliability and model configuration

The verifier profile remains on `gpt-5.6-sol` and changes from `high` to
`medium` reasoning effort. The role's narrower decision surface and explicit
matrix reduce the need for exploratory reasoning. `low` is not part of the
initial rollout and may replace `medium` only after representative verification
tasks show unchanged target correctness, matrix completeness, failure
classification, and mutation detection.

[Official OpenAI subagent guidance](https://learn.chatgpt.com/docs/agent-configuration/subagents)
describes `medium` as a balanced default, `low` as appropriate for
straightforward latency-sensitive work, and higher effort as more costly but
potentially useful for complex reasoning. It also confirms that different
roles can select different model and reasoning settings and that each subagent
performs its own model and tool work. The benchmark therefore measures the
actual role mix rather than inferring savings from profile declarations alone.

[Official OpenAI non-interactive-mode guidance](https://learn.chatgpt.com/docs/non-interactive-mode)
defines `--ephemeral` as avoiding persisted session rollout files and `--json`
as a JSONL event stream containing command, file-change, tool, search, and turn
usage evidence. Those interfaces supply the fresh-session and dual-layer raw
measurement boundary; they do not replace repository manifests or
Acceptance evidence.

### Compatibility and rollout

Existing approved plans and eligible legacy work retain their authority form.
Existing planned callers continue to use `execute-task`. Existing lightweight
entry points move to `execute-lightweight-task` without acquiring a plan,
Task-orchestrator, or workspace-only planned artifact. The new boundary applies
after the updated bundle is installed and a new Codex session loads it.

`search-cache.md` is a per-feature workspace artifact, not an installed managed
asset. The installer inventory nevertheless adds exactly one managed Skill,
`execute-lightweight-task`, while retaining `execute-task` at its existing
destination. The implementation must keep both executor contracts, their direct
callers and references, fallback prompts, README guidance, and asset-contract
tests semantically aligned.

Local implementation and verification do not install into the live Codex home.
After candidate verification, the rollout fingerprints the still-installed old
bundle, performs the complete before measurement, and only then crosses the
owner-controlled installation boundary. Publication and branch disposition
remain separate owner-controlled actions.

### Evaluation

Completion requires fresh repository validation of the changed assets and
focused contract tests. Operational evaluation then runs one minimal canonical
product specification through the real complete Task loop in Rust, Go, and
Kotlin. The product is a deterministic `task-filter` CLI:

- it accepts one file whose non-empty records have the exact TSV form
  `id<TAB>priority<TAB>status`;
- identifiers match `[a-z0-9-]+`; non-ASCII identifiers are invalid;
- priorities are integers from 0 through 9 and status is `open` or `done`;
- it emits only open records as `id<TAB>priority`, ordered by descending
  priority and then ascending ASCII identifier;
- it ignores empty lines and treats duplicate identifiers as independent
  records;
- an invalid record produces no standard output, a line-numbered diagnostic,
  and exit status 2; and
- an input read failure produces no standard output, a stable diagnostic, and
  exit status 1.

Equivalent native tests cover filtering, deterministic ordering, invalid input,
and read failure. The implementation stays deliberately small and uses no
third-party Rust crate or Go module. Kotlin keeps the prepared project's fixed
Kotlin JVM plugin 2.3.20, Gradle 9.2.1, JVM 25, standard runtime, and
`kotlin("test")` declarations. Gradle resolution may use the network, but the
benchmark adds no plugin, application or test library, repository, dependency
declaration, or version.

The benchmark root is
`/Users/sakumatomoya/workspace/codex-task-loop-benchmark`. It contains exactly
three language repositories:

- `task-filter-rust`;
- `task-filter-go`; and
- `koltin`, retaining the owner-selected project name.

Each repository records one base commit and creates `benchmark/before` and
`benchmark/after` from that exact commit before the first observed run. The
before and after sides execute sequentially by switching branches in the same
language repository. This is repository reuse, not execution-state reuse. The
before result remains committed on its branch, and the controller preserves
raw run evidence outside the language repositories at:

```text
evidence/{before,after}/{rust,go,kotlin}/
```

Mutable compiler, package, build, and temporary state is redirected outside the
language repositories from the start and separated by side at:

```text
run-state/{before,after}/{rust,go,kotlin}/
```

Rust redirects Cargo home and target state. Go redirects build, module, GOPATH,
and temporary state and disables network module paths. Kotlin gives each side a
separate mutable Gradle home, project cache, build output, and temporary state.
Before observation, the controller resolves only the fixed declared Kotlin
graph into a fingerprinted input snapshot. Both sides start from equivalent
copies of that snapshot; later network and download events are recorded
observations rather than automatic failures. Environment preparation is
excluded from observed Task-loop time; the global Gradle home, build outputs,
compiler daemon state, and mutable Gradle state are never shared or mutated by
the measured runs.

Before switching to an after branch, the controller records the before result
head, raw JSONL, metrics, current-head evidence, tracked/untracked/ignored status
manifests, and the per-run `search-cache.md` when the installed bundle creates
one. Absence is recorded rather than synthesized. The controller then proves
that the frozen after branch still equals the recorded base and that the
language workspace has no residual before search cache, build output,
conversation state, verdict, or other run-only artifact. Unexpected residual
state blocks the pair instead of being silently reused.

All three before workloads run against the fingerprinted installed baseline
before candidate installation. All three after workloads run only after the
candidate is installed and fingerprinted. Every side uses a new
`codex exec --ephemeral --json` process and its own empty run-state directory.
The exact prompt, canonical product contract, starting Git tree, Review context,
Review policy, root capacity, top-level model settings, and expected quality bar
remain fixed within each language pair. Leaf conversation inheritance,
role-specific handoff content, search-cache behavior, waiting, batching, and
role settings follow the installed Task-loop bundle and are observed rather
than imposed by the benchmark controller.

The complete Task loop includes implementation, fresh verification,
policy-selected review, findings integration and triage when findings occur,
bounded authorized correction when needed, fresh post-correction gates, and the
final Task outcome. No finding or correction is injected. Naturally occurring
findings, correction cycles, convergence, and final quality are measurements.
The exact repository setup, fixtures, prompt bytes, status-manifest procedure,
language dependency and state boundaries, and run commands live in the
Implementation Plan rather than reusable Skill prose.

Measurement reports two layers:

- model-facing tool calls and turns; and
- the underlying operations, commands, and checks inside direct or
  programmatic calls.

Waits are reported separately by call count, requested bound, observed elapsed
time, and early-return event. The report also includes phase and end-to-end
elapsed time, repeated searches, required-evidence completeness, verdict and
finding agreement, mutation and target-identity detection, and correction-loop
convergence. No persistent telemetry or event-log service is introduced.

Absolute values and before-to-after percentage changes are reported within each
language pair. Cross-language absolute durations remain descriptive because
compiler, build-tool, and ecosystem costs differ. The initial evaluation runs
each side once to bound time and token use; it preserves raw values so a later
repeat can investigate a noisy or surprising result without changing this
feature's benchmark contract.

Lower calls, turns, operations, waits, searches, tokens, or elapsed time count
as an improvement only when both sides satisfy the same quality bar: complete
contract coverage, fresh verification, every policy-selected review, complete
handoff evidence, and no unresolved Acceptance blocker.

## Alternatives considered

### Split every planned and lightweight lifecycle Skill

Rejected because the observed collision is inside the mixed Task executor.
Duplicating or renaming the coordinator, check-only phases, workspace handling,
and completion workflow would enlarge migration and policy-drift risk without
strengthening the selected authority boundary.

### Keep lightweight as an `execute-task` variant

Rejected because additional conditional guards preserve the same mixed executor
contract that caused planned and lightweight loop ownership to be confused. One
dedicated lightweight executor removes that ambiguity with the smallest
installed Skill and caller change.

### Root-driven phase orchestration

Rejected because the Task orchestrator already matches the Task PR boundary.
Moving every phase transition back to the root would restore the context and
coordination pressure the current topology was designed to remove.

### Let the implementer run the complete authoritative suite

Rejected because the verifier must rerun current-head checks independently.
Keeping the same broad suite in both phases adds latency without strengthening
the authoritative gate. Writer-side evidence remains focused on producing a
coherent candidate.

### Keep semantic diff inspection in verification

Rejected because it blurs mechanical evidence with reviewer judgment and makes
the verifier require broader context and reasoning. Review already owns semantic
correctness, scope, quality, architecture, and test adequacy.

### Remove verifier Git inspection

Rejected because a verifier must prove that command results apply to the
requested unchanged target and that it did not mutate source. The selected
design removes duplicated topology and semantic work while preserving those
invariants.

### Keep spare Task leaves available for every phase

Rejected because implementation, verification, integration, and correction use
one active leaf by contract. Spare Task slots have useful parallel work only in
the independent reviewer wave and otherwise reduce global scheduling fairness.

### Selectively rerun reviewers after correction

Rejected because reviewer invalidation is a new Review-policy mechanism with a
higher risk of missed cross-perspective regressions. Delta-first traversal gives
most of the context benefit while preserving the existing selected set and a
fresh verdict.

### Reuse the previous review verdict

Rejected because review evidence is bound to an exact head and range. Prior
reports can guide inspection but cannot make a new head accepted.

### Use Luna/Max for verification

Rejected because the owner observed non-response in long verification turns and
the redesigned role is bounded and mechanical. Sol/medium is the conservative
initial reliability setting; Sol/low remains an evaluation option.

### Add a persistent matrix or runtime state schema

Rejected because it introduces lifecycle, identity, and recovery obligations
without being needed for one Task turn. Durable contracts, compact handoffs,
reports, and direct Git evidence remain the recovery sources.

### Inherit parent conversation and send only a delta prompt

Rejected because inherited turns are implicit, oversized, and stale-prone. A
replacement or compacted session cannot prove which parts remain current. One
complete role-specific handoff plus direct Git and authority lookup gives every
new subagent an explicit recoverable execution contract.

### Poll agents frequently with short waits

Rejected because repeated polling adds turns and tool calls without improving
responsiveness when mailbox and completion events already return a bounded wait
early. Five-to-ten-minute bounds retain liveness without busy waiting.

### Batch every implementer action

Rejected because edits, approvals, diagnosis, and TDD transitions depend on
earlier observations. Only independent discovery or mechanical post-edit checks
are batched; judgment-dependent stages remain ordered.

### Make every historical RED discrepancy an Acceptance blocker

Rejected because a past sequence cannot be repaired after production behavior
exists, and the RED category alone does not prove a current defect. The workflow
discloses the discipline gap and blocks or escalates only for a reachable defect,
material evidence gap, or material contract deviation. It never fabricates a
replacement RED or claims unobserved TDD.

### Use general research notes or a design-only cache

`research-notes.md` was rejected because it suggests free-form prose rather than
lookup-before-search behavior. `discovery-cache.md` was rejected because it
communicates the primary purpose less directly than `search-cache.md`. Limiting
the cache to design and planning was rejected because Task and correction loops
would still repeat discovery.

### Give every cached source one TTL

Rejected because Git refs, versioned documentation, mutable external pages, and
negative search results have different currentness semantics. Source-aware
identity and invalidation avoid both unnecessary local searches and unsafe
reuse of mutable information.

### Count only top-level calls or add persistent telemetry

Top-level-only counting was rejected because programmatic batching could hide
unchanged or increased underlying work. A persistent event log or telemetry
service was rejected as unnecessary new operational state. Dual counters and
bounded run evidence expose both layers without adding a service.

### Script one clean path and one forced correction path

Rejected after operational use because a mandated finding or correction tests a
benchmark script more than the real Review loop. The selected evaluation runs
the full loop for every fixed workload and records naturally occurring findings
and corrections while holding the final quality bar constant.

### Use six before/after language checkouts

Rejected because two frozen branches from the same recorded base preserve the
source starting point without doubling the number of project directories.
Separate controller-owned evidence and run-state trees plus an exact residual-
state check provide the required execution isolation.

### Keep Kotlin strictly offline

Rejected after E1 setup proved that the local Kotlin 2.3.20 cache lacks part of
the compiler/build-tool artifact graph. The repository owner selected fixed
dependency declarations, not offline execution, as the material boundary.
Network resolution therefore remains limited to the already-declared graph and
does not authorize another plugin, library, repository, or version.

### Keep Python or use Java for the JVM workload

Python was rejected because it is not representative of the owner's primary or
workplace workloads. Java was superseded by the locally prepared Kotlin/JVM
project, which better represents the workplace environment while retaining a
fixed Kotlin 2.3.20, Gradle 9.2.1, and JVM 25 toolchain. Rust, Go, and Kotlin
provide three relevant paired workloads while keeping the benchmark bounded.

### Use a larger application or third-party dependencies

Rejected because the benchmark compares workflow overhead, convergence, and
quality on the same fixed task within each language. Additional product scope
or dependency declarations would spend more time and tokens without improving
that comparison. Kotlin's fixed build-tool graph may resolve over the network,
but that does not authorize another application or test dependency.

### Compare unrelated live Tasks after rollout

Rejected as the sole evaluation because Task difficulty, findings, and runtime
conditions would confound the result. Fixed language-local pairs establish a
controlled before/after comparison; later live observations may inform future
work but are not this revision's acceptance evidence.

### Publish the revision as a separate addendum

Rejected because future consumers would need to resolve two precedence-bearing
documents for one Task-loop contract. This revision keeps one current durable
authority while Git history preserves the previously approved text.
