# [Design Doc] Codex Task-loop optimization

- Owner: Repository owner
- Drafted by: Codex from owner-settled design decisions
- Date: 2026-08-28
- Revised: 2026-08-31
- Status: Single-orchestrator revision approved by the repository owner on
  2026-08-31
- Approved single-orchestrator revision draft SHA-256:
  `0c74bb5dccd733a3153775b35e04f2120c0c66df87730ad10831844b1ba9a389`
- Retired single-orchestrator revision settled source:
  `docs/plans/2026-08-28-codex-task-loop-structural-optimization/decision-record.md`
- Prior approved operational revision SHA-256:
  `791530fa0bd64c49c59fe7d0b376be8e653d9102e6243bb46620d3b52f642875`
- Prior approved source/benchmark separation SHA-256:
  `71f46d3e6531a16bfe2e9611cd2ab351205c0d907ca73addeae8907e4ade879d`
- Previous PR candidate SHA-256:
  `e06a28b4b036c4df926567ef9892874b21f7221c5489286f01d3b08741fd6eca`
- Direct implementation reconciliation: verifier references reflect the
  separately approved `gpt-5.6-sol/low` state already committed at
  `caafd15bff5bc648d8bb27055f9e9cd818191542`.
- Prior approved source document SHA-256:
  `6fc0766f02c3cc40735d6b77ae12bed0df48c6f1795a3540f97a0928eb0c6cbb`
- Repository baseline: `d32ec49957eb419dd12095b69c196eb0128619bb`
- Extends:
  - `docs/design/2026-08-25-codex-task-orchestrator-subagents.md`
  - `docs/design/2026-08-18-codex-pr-scoped-task-execution.md`
- Supersedes the per-Task orchestrator ownership, root-to-Task scheduling
  handoff, and workflow capacity-lease decisions in the extended documents;
  their unchanged Task/PR isolation and acceptance decisions remain applicable.

## Context and scope

The previously approved planned workflow assigns one non-writing Task
orchestrator to each Task Contract. That orchestrator owns the Task-local
implementation, verification, policy-selected review, triage, and bounded
correction loop while the root owns Feature coordination and workflow-level
capacity grants.

Operational use showed that the extra Task coordination layer adds one context,
thread, handoff boundary, wait path, and root re-entry per active Task. The
observed runs did not show a corresponding quality difference from the earlier
root-owned loop. This is evidence for simplifying the current workflow, not a
statistical claim that the topology can never help another workload. The root
must absorb the necessary Task state, so removing the Task orchestrator also
does not imply that all of its measured token cost disappears.

This revision therefore uses the root/Feature lead as the only workflow
orchestrator. The root releases dependency-ready Tasks, dispatches each
Task-local role directly, owns phase transitions and exact evidence, and
coordinates isolated Task workspaces and PRs. One implementer remains the sole
writer for one Task responsibility. Independent ready Tasks and read-only
reviewers may run concurrently without allowing overlapping writers.

The verifier profile originally used `gpt-5.6-sol` with `high` reasoning. The
repository owner observed that `gpt-5.6-luna` with `max` effort could stop
responding in long verification workflows and wanted verification to be a
shorter mechanical phase. The initial design therefore kept Sol for
reliability and reduced verifier effort to `medium`. A later owner-approved
direct streamlining changed the already bounded mechanical role to `low`; this
operational follow-up preserves that current setting.

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
Acceptance. It revises the same durable authority rather than adding a
precedence-bearing addendum.

Operational use also exposed an authority-boundary problem inside the shared
`execute-task` Skill. The earlier correction split the root-owned lightweight
variant into `execute-lightweight-task` while retaining planned and eligible
legacy work in `execute-task`. This revision keeps that Skill boundary because
the two routes still consume different authority forms, but makes both loops
root-owned. Loop ownership is unified without recombining planned and
lightweight contracts.

The accepted phase optimizations remain: role-specific context, current-head
Verification Matrices, check-only verification, risk-directed TDD discipline,
early review finding admission, findings-only integration, targeted correction
review, event-responsive waiting, and Feature-local search caching. This
revision changes coordination ownership and removes workflow capacity leases;
it does not weaken acceptance gates or change which reviewers the approved
Review policy selects.

The first controlled Rust operational comparison confirmed that the optimized
workflow preserved source quality but did not improve end-to-end efficiency for
one minimal planned Task. Wall time increased from 25m13.649s to 37m08.136s
while total model tokens increased only 1.76%. The implementer added 303.453s
and the general review integrator added 265.212s, together explaining about 80%
of the wall-time regression. Wait calls and timeouts decreased, so the bounded
wait design is not reopened. Spawn handoffs grew about 1.7--2.3 times, and the
test reviewer sent hypothetical mutation gaps into expensive late integration.
This single comparison is causal operational evidence for the observed run, not
a statistical performance claim across workloads.

The follow-up revision therefore broadens test-first discipline into a
risk-directed verification strategy without weakening RED causality when TDD is
selected, moves the existing Acceptance threshold to source-review admission,
and makes role-complete handoff and revalidation ownership explicit while
retaining context-isolated dispatch.

### Goals

- Use the root/Feature lead as the only workflow orchestrator for planned,
  lightweight, and eligible legacy Task loops.
- Remove the per-Task non-writing Task orchestrator, its root re-entry boundary,
  and workflow-level capacity grants without removing Task-local evidence or
  gates.
- Keep one isolated workspace, branch, PR, and active writer per Task while
  allowing ownership-disjoint dependency-ready Tasks to progress concurrently.
- Prevent planned and lightweight Task-loop authority from sharing one executor
  contract while making both executor loops root-owned.
- Establish one concise current-head Verification Matrix as the authoritative
  verifier handoff and result spine.
- Limit writer-side checking to candidate-quality evidence and remove repeated
  full-suite verification from the writer phase.
- Make the verifier a fail-fast, mechanically ordered, check-only executor that
  does not perform semantic code review.
- Preserve only the Git identity and mutation checks the verifier must observe
  directly while keeping full Task Git ownership with the root.
- Give writers, verifiers, reviewers, and integrators role-complete rather than
  workflow-complete handoffs and reports.
- Let the Codex runtime enforce session thread admission rather than maintaining
  Task leases, grants, reviewer expansions, or workflow capacity arithmetic.
- Review corrections against their bounded affected surface by default while
  retaining the same reviewer set and explicit full-traversal escalation.
- Keep verification on `gpt-5.6-sol` and preserve its current `low` reasoning
  effort.
- Preserve fresh evidence, Review breadth, Acceptance, and correction authority.
- Start every new leaf without inherited parent conversation and give it one
  complete role-specific handoff.
- Replace routine short polling with normally five-to-ten-minute bounded waits
  that return early on mailbox or completion events.
- Batch independent implementer discovery and mechanical post-edit checks while
  preserving judgment-dependent boundaries and causal test-first ordering.
- Select the verification discipline from the material property under
  development, including coherent RED/GREEN matrices and non-example oracles
  where appropriate, without manufacturing TDD evidence.
- Apply the common Acceptance threshold before a source reviewer emits a
  finding and avoid expensive integration probes used only to disprove an
  unsupported hypothesis.
- Prevent repeated planned-lifecycle searches through an ignored,
  feature-local `search-cache.md` with source-aware invalidation.
- Keep historical TDD discipline evidence distinct from the evidence that
  establishes current-head Acceptance.

### Non-goals

- Change Task decomposition, the Task dependency DAG, PR topology, Review
  modes, reviewer selection, or the common Acceptance threshold.
- Remove general findings integration or triage for a concrete admitted
  finding.
- Skip a policy-selected reviewer after correction or reuse a prior verdict for
  a new head.
- Remove fresh verification after implementation or correction.
- Remove verifier target or mutation-invariant checks.
- Add a persistent Verification Matrix, runtime queue or state file, finding
  identifier, telemetry service, machine-readable coordination schema, or
  another workflow scheduler.
- Hide underlying operations behind a single top-level call count.
- Change `agents.max_threads`, its installer tiers, or `agents.max_depth`.
- Allow simultaneous writers in one worktree or overlapping responsibility.
- Split the complete coordinator, verification, review, workspace, or completion
  lifecycle into parallel planned and lightweight Skill families.
- Rename `execute-task` or redesign its planned and eligible-legacy callers.
- Treat conversation history, `search-cache.md`, agent identity, or liveness as
  correctness or Acceptance authority.
- Weaken RED-before-production causality after TDD is selected, accept an
  exploratory implementation as production evidence, or claim TDD when an
  intended RED was not observed.
- Re-enable parent conversation inheritance or impose a fixed handoff byte or
  token limit that could omit role-material authority.
- Make an external benchmark, installation, publication, or branch disposition
  implicit in source Feature Acceptance or Design Doc approval.

### Explicit deferrals

None.

## Overview

The Task loop keeps its existing state transitions but each phase consumes and
returns evidence at its own boundary:

```text
Root / Feature lead (sole orchestrator)
  |
  +-- Task A / isolated worktree / PR A
  |     implementer -> matrix -> verifier -> reviewer wave
  |                                  |
  |                                  +-> findings integration when needed
  |                                  +-> bounded correction
  |                                      -> fresh matrix/verify/review
  |
  +-- Task B / isolated worktree / PR B
  |     same Task-local state machine when dependency-ready
  |
  +-- Task C / isolated worktree / PR C
        same Task-local state machine when dependency-ready

Codex runtime owns session thread admission for direct leaves.
```

The root selects ready Tasks, validates complete Task and Git identity, builds
the current-head Verification Matrix, dispatches every policy-selected role,
and owns Task-local and Feature-level transitions. Leaves do not schedule
descendants. Only the active implementer or correction writer writes within one
Task responsibility.

The Skill boundary around those unchanged state transitions is:

```text
agentic-engineering-workflow
  +-- planned -----------------------> execute-plan -> execute-task
  +-- lightweight --------------------------------------> execute-lightweight-task

execute-task              planned and eligible legacy; root-owned
execute-lightweight-task  lightweight only; root-owned
verify/review/triage      explicit-input check phases shared by both paths
finish-branch             existing completion modes shared by both paths
```

The router still owns classification and cross-phase transitions. The split is
at the Task executor only; it does not create a second coordinator family.

## Detailed design

### Minimal Task-executor Skill boundary

`execute-task` removes its lightweight handoff variant and rejects lightweight
authority. It accepts planned and eligible-legacy Task identity from the root,
plus planned cache input, Verification Matrix obligations, reviewer policy,
bounded correction authority, and prior attributable evidence. It no longer
accepts or requires a Task orchestrator identity.

The new `execute-lightweight-task` accepts the recoverable combined
Feature/Task Contract, root-owned loop identity, exact Task PR target, Review
context and policy, discipline, verification obligations, and prior
attributable lightweight evidence. It rejects a Herdr workspace requirement,
Task DAG, PR topology, planned `search-cache.md`, or any other planned-only
authority. Its correction loop retains the same reviewer set and
fresh-current-head gate without passing through `execute-plan`.

`agentic-engineering-workflow` continues to classify requests and prepare both
routes. Its planned calls still enter `execute-plan` and `execute-task`; only
lightweight implementation, correction, and re-entry calls change to
`execute-lightweight-task`. Shared exact-target `verify`, `review`,
`receiving-code-review`, profiles, fallback prompts, `create-workspace`, and
`finish-branch` contracts are not split. Direct wording or inventory references
may change only where the new executor identity must be observable.

This boundary is complete when neither executor contains the other route's
handoff variant. Both claim the root as loop owner, while shared check phases
remain safe because they receive an explicit target and authority rather than
selecting a lifecycle route.

### Evidence ownership by phase

The root owns complete Feature and Task identity and orchestration evidence:

- approved authority and Review policy;
- Task dependency DAG, PR topology, readiness, and release state;
- Task workspace and branch;
- planned base, merge base, head, exact range, diff, status, changed files, and
  attribution;
- writer isolation and task/correction commits;
- ready, pending, active, accepted, stale, and blocked Task states and phase
  transitions;
- the selected-but-not-started role queue required when runtime admission is
  temporarily unavailable;
- assembly and currentness of the Verification Matrix;
- preservation of prior review and triage evidence for correction re-entry;
  and
- integration-only composition, Feature acceptance, and user-escalation state.

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

Every newly spawned leaf uses explicit `fork_turns="none"`. Parent conversation
is neither inherited execution context nor correctness evidence. Re-entry
through an existing idle identity still receives a fresh complete handoff and
revalidates the current inputs inside that role's correctness boundary.

Complete means complete for the receiving role, not a copy of the Task owner's
entire state. Each handoff contains:

- the role's exact purpose, responsibility, allowed actions, and prohibited
  overlap;
- applicable authority identity, currentness, assigned clauses, constraints,
  and non-goals;
- workspace, Git target, source-state boundary, and direct source locations;
- required observations, output schema, stop conditions, and re-entry evidence;
  and
- the prohibition on descendant scheduling and overlapping write ownership.

The root retains complete Feature and Task authority, Review context and policy,
phase state, dependency and PR topology, pending roles, and prior attributable
evidence needed to own every loop. Each leaf receives only the subset that can
change its bounded decision, while exact sources remain directly readable. A
leaf or replacement resolves Git and authority from those sources; it never
uses identity, prior conversation, or a plausible summary as proof.

Authority prose is referenced by exact path or in-memory identity, approval and
currentness evidence, and stable applicable clause or interface identity rather
than copied into competing wrappers. The root revalidates Feature topology,
approval currentness, Task identity, gates, and phase state; the implementer
revalidates writer isolation and its edit target; the verifier revalidates
matrix target and mutation invariants; each reviewer revalidates the unchanged
reviewed target; and an integrator revalidates the unchanged finding target. A
role expands into additional directly available authority only when current
evidence makes that authority decision-relevant.

Role completeness is semantic, not a fixed byte or token budget. A compiler,
DBMS, operating-system, SIMD, or asynchronous-system Task may require detailed
invariants, failure semantics, concurrency guarantees, reference-oracle
identity, or performance obligations. Those inputs remain when they can change
the role's decision; unrelated topology, queue, completed-gate, and procedure
detail remains with its owner.

### Current-head Verification Matrix

The root constructs one in-memory matrix after the candidate head and
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
route open, the root or verifier may select a compatible standard
check and record the choice in the matrix.

The matrix is bound to the exact target head and controlling authority. A
commit, range change, contract change, or material command-route change makes it
stale and requires a rebuilt matrix. The completed matrix is the verifier's
compact report spine and is passed to review as verification evidence.

### Verifier target and Git boundary

The root resolves the full planned Task identity before verifier dispatch. For
a clean isolated planned Task PR, the verifier independently
confirms only the facts needed for valid command evidence:

- the expected workspace, current head, and exact committed range still match;
- the pre-check index, worktree, and relevant in-scope source state are clean as
  required by the supplied target;
- the changed-file inventory and `git diff --check` agree with that target;
- the same head and source state remain after verification, except for recorded
  normal ignored build or test artifacts.

It does not repeat branch selection, topology resolution, complete attribution,
or semantic diff inspection already owned by the root.

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

### Risk-directed verification discipline

Before editing, identify the material property and available oracle, then
classify TDD as `applicable`, `not applicable`, or `required but blocked` and
record the reason. TDD is applicable when observable production-code behavior
changes and a focused executable test or coherent test matrix can demonstrate
the missing behavior before the edit. Documentation, instructions, metadata,
formatting, generated output, and behavior-preserving mechanical changes use an
existing green baseline and proportionate validation. A required behavior or
verification route that cannot be established safely is blocked rather than
silently reclassified.

Applicable TDD uses the smallest causal behavioral slice:

- one independently decided viewpoint uses focused `RED -> production edit ->
  GREEN`;
- several cases may use `RED matrix -> one causal production edit -> GREEN
  matrix` when they exercise one missing capability, remain separately
  attributable, and no case result changes the design, test selection, or input
  of another; and
- cases whose observations change a subsequent design, edit, or test remain
  sequential.

The order is strict at the selected slice boundary. No GREEN check is launched
before the production edit it validates, and a broad matrix is not permission
to combine unrelated behavior or hide an incorrect RED reason. After focused
GREEN, independent mechanical post-edit checks may be grouped when their
individual results remain attributable. The verifier's fail-fast order and
stronger current-head boundary remain unchanged.

Example-based TDD is not the only valid oracle. Property, model, differential,
fault-injection, stress, integration, emulator or hardware, and benchmark
evidence are selected when state space, semantic equivalence, concurrency,
failure, environment, or performance is material. Correctness and performance
obligations remain distinct: a noisy performance threshold is not manufactured
as a unit-test RED, and a benchmark does not replace correctness evidence.

A time-bounded exploratory implementation may establish feasibility, expose a
hardware or runtime constraint, or help derive an oracle. It records its
hypothesis and result but is not TDD, a production Candidate, or Acceptance
evidence. Productionization requires explicit intended behavior and the
contract-appropriate current verification route; exploratory code is discarded
or enters that ordinary implementation boundary under current authority.

### TDD history and current Acceptance

When applicable, TDD discipline evidence records an execution history: the
selected causal behavioral slice, its test or coherent matrix, the observed
pre-production RED results and reasons, the subsequent edit, focused GREEN
results, and any refactor while green. The implementer must report this sequence
honestly and never claim TDD when the intended RED was not observed. A
non-applicable change reports its reason, baseline, and validation instead; an
exploration reports its hypothesis and result without claiming Candidate or
Acceptance evidence.

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

### Early review finding admission

The common Acceptance threshold applies inside every source-review perspective
before it emits a finding. A source finding must connect the inspected artifact
to an approved requirement or realistic reachable consumer behavior, concrete
evidence, a material consequence, and a proportionate correction. Preference,
generic best practice, optional hardening, an imagined future consumer, and an
unsupported second-order scenario return the perspective's clean result rather
than becoming work for a later integrator.

For test coverage, constructing an alternative implementation that passes the
current tests is an investigation technique, not sufficient finding evidence by
itself. The reviewer must also show that the escaping implementation violates
an approved behavior or material quality obligation, or produces a realistic
reachable failure. Missing property, model, differential, concurrency, failure,
hardware, or performance evidence remains a valid finding when the applicable
contract or observed risk requires it.

When any source report remains non-clean, the existing general findings
integration and triage path remains mandatory. The integrator evaluates
authority, artifact applicability, reachability, and materiality before runtime
reproduction or remedy analysis. It drops an item missing those prerequisites
without running probes solely to disprove speculation. An evidence-grounded
item receives the existing separate problem-validity and remedy-validity
assessment and proceeds to `Fix`, `Push back`, or `Escalate` classification by
`receiving-code-review`.

This moves rejection earlier without lowering Acceptance, letting raw reviewer
output authorize correction, or removing independent integration for concrete
findings.

### Feature-local search cache

Planned work stores reusable discovery at:

```text
docs/plans/YYYY-MM-DD-<feature>/search-cache.md
```

The file is ignored, workspace-only, and non-authoritative. It prevents the root
and leaves from repeating the same repository, Git, documentation, runtime, or
external search, including useful searches that found no result. The root is
the only writer. Leaves read it and return new cache candidates in their
reports. Within one Task turn, the root may pass an attributable returned
result directly to the next role before integrating it into the file;
cross-loop persistence remains root-controlled.

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

The root retains the complete durable evidence, but sends each leaf
only the subset that changes that role's decisions. Exact authority is passed
as directly readable source identity plus applicable clauses rather than copied
in full, and the same fact is not repeated in a second wrapper schema.

The implementer message contains owned responsibility, applicable authority
clauses, preserved boundaries, discipline, candidate target, commit intent, and
writer-side checks. Review scheduling, completed verification output, and
unrelated contract prose, Feature topology, and other Task state are omitted.

The verifier message contains target identity, the Verification Matrix,
command-environment facts, required source-state boundary, and verdict schema.
The full Review policy is not copied unless one of its exact constraints changes
the verification route. It does not receive Task scheduling or approval history
that the root has already resolved.

Each reviewer message contains the verified target, current diff, changed
files, its applicable authority clauses and selected perspective, Review context
and policy, the completed Verification Matrix, and relevant prior triage.
Unrelated authority remains directly readable but is not eagerly copied. Prior
reports are included only for correction re-entry or another explicit decision
that depends on them.

An adversarial or general findings integrator receives the unchanged target,
complete source reports relevant to that integration, applicable authority,
Review context and policy, and prior triage needed for origin and remedy
assessment. It does not receive writer procedure or verifier scheduling detail
unless a finding depends on that evidence.

Writer reports return candidate and commit evidence, verifier reports return the
completed matrix and verdict, reviewer reports return perspective-specific
findings or clean status, and integrators return reconciled evidence. The root
preserves these reports without translating them into another wrapper.
Handoff size is observed during operational evaluation but has no fixed limit;
missing role-material evidence is never traded for a smaller prompt.

### Runtime-governed direct dispatch

The root dispatches each Task role directly after its preceding evidence gate
passes. Implementation, verification, findings integration, triage, and
correction remain distinct phases; removal of leases does not authorize phase
overlap, duplicate verification, early integration, or multiple writers.

After verification passes, the root may start the complete policy-selected
independent reviewer set without calculating a Task-local grant. The Codex
runtime decides which spawn requests the session thread limit admits. The
workflow does not maintain baseline leases, reviewer expansions, a fixed Task
leaf ceiling, or configured/observed/effective capacity arithmetic.

A role selected by policy but rejected at spawn because the runtime thread
limit is reached remains pending in the approved ready-Task or selected-reviewer
order. The root waits for a mailbox or completion event and retries admission;
it does not drop the role, weaken Review breadth, or classify resource pressure
as a Design escalation. Because a rejected spawn is not assumed to be
runtime-queued, the root retains only the minimal pending-role state needed to
retry it.

Live-agent inspection is used to prevent duplicate dispatch and to establish
liveness, failure, interruption, recovery, and teardown state. It is not a
mandatory capacity probe before every normal dispatch. Leaves do not schedule
descendants. Runtime admission controls concurrency while workflow authority
continues to control which roles and phase transitions are valid.

### Event-responsive bounded waiting

The root waiting for one or more direct leaves uses one bounded `wait_agent`
interval of normally five to ten minutes. Mailbox updates, completion
notifications, or steered user input return control before that bound. The root
may perform independent useful work before entering the wait, but does not
replace the bounded wait with repeated short polls.

Live-agent inspection remains required when duplicate, liveness, failure,
interruption, replacement, or teardown state is decision-relevant. A shorter
wait is justified only by a nearer explicit deadline, teardown, or interruption
boundary and records that reason. A terminal role result returns control to the
root, which validates its target and advances, retries, corrects, or escalates
the owning Task state.

### Targeted correction review

A correction changes the reviewed head from `H1` to `H2`. An earlier verdict
cannot authorize `H2` by itself, so the correction sequence still requires:

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

`review` owns correction-review scope and escalation. Targeted re-review is the
default: the reviewer confirms the corrected finding, inspects `H1..H2`, follows
affected callers, tests, interfaces, responsibilities, and obligations, and
establishes which prior clean conclusions remain unaffected. The exact prior
report supplies that unchanged coverage; the new result is bound to `H2` and
the bounded correction. Together they form current evidence for that
perspective.

The reviewer escalates to full current-target traversal when:

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
- Runtime thread-limit rejection leaves the selected role pending and causes a
  bounded wait and retry. It is operationally `BLOCKED` only after repeated
  non-progress prevents a required queue from advancing; it is not a Design
  escalation by itself.
- A stale prior correction report prevents bounded unaffected coverage and
  therefore escalates that reviewer to full current-target traversal.
- A correction that changes contract meaning returns to the existing authority
  or plan gate rather than being treated as a larger delta.
- A required verification oracle or safe test seam that cannot be established
  is `BLOCKED`; exploratory implementation does not bypass that evidence gap.
- A source-review candidate that lacks approved or reachable behavior and a
  material consequence is a clean source result, not a deferred integration
  problem.
- An implementation detail not fixed by approved authority remains delegated
  to the writer inside its responsibility. Only a material missing or ambiguous
  architecture, responsibility, public contract, schema, error model, scope,
  or verification obligation returns to the user-owned design branch.
- An unexpected leaf failure or interruption makes the root inspect liveness,
  exact Git state, mutation attribution, and returned evidence before re-entry.
  It may reuse a compatible idle identity or start a replacement with a fresh
  complete handoff; identity and prior conversation never establish recovery
  correctness.

No optimization authorizes cleaning, resetting, amending, rebasing, discarding,
publishing, or installing live assets merely because an intermediate phase
passes. Installation occurs only at the explicit rollout boundary after
baseline evidence is complete.

## Cross-cutting concerns

### Context and token use

The Verification Matrix removes repeated prose and gives verification one
current-head evidence format. Role-specific messages stop every leaf from
receiving the complete Task orchestration state. Targeted correction review
reuses exact unaffected coverage without letting an earlier verdict authorize
the new head. Removing the Task orchestrator eliminates one context, handoff,
wait path, and root re-entry per active Task. Context-isolated dispatch prevents
the parent transcript from being copied into every new role. Decision-aware
batching and coherent causal test matrices remove unnecessary model turns,
source-review admission prevents speculative late integration, and
`search-cache.md` prevents the same discovery from being repeated across the
planned lifecycle.

These are context reductions, not permission to omit exact authority. Every
role keeps direct access to the source artifacts and expands its inspection when
current evidence requires it. A cache hit or compact handoff remains operational
context, never proof that the current Git target or authority still matches.
The operational target is less irrelevant copied context and repeated semantic
work, not minimum prompt length or minimum test count. The root necessarily
absorbs Task state that used to be held by Task orchestrators. Exact artifact
references, compact reports, bounded per-Task state, and isolated workspaces
limit that central context without introducing another coordinator.

### Reliability and model configuration

The verifier profile remains on `gpt-5.6-sol`. The initial rollout changed
reasoning effort from `high` to `medium`; the later owner-approved direct
streamlining changed the already bounded mechanical role to `low`. This
follow-up leaves that setting unchanged. The role's narrow decision surface and
explicit matrix reduce the need for exploratory reasoning, while target
correctness, matrix completeness, failure classification, and mutation
detection remain Acceptance obligations rather than assumptions inferred from
the effort level.

[Official OpenAI subagent guidance](https://learn.chatgpt.com/docs/agent-configuration/subagents)
describes `medium` as a balanced default, `low` as appropriate for
straightforward latency-sensitive work, and higher effort as more costly but
potentially useful for complex reasoning. It also confirms that different
roles can select different model and reasoning settings and that each subagent
performs its own model and tool work. Any external evaluation must observe the
actual role mix rather than infer savings from profile declarations alone.

### Compatibility and rollout

Existing approved plans and eligible legacy work retain their authority form.
Existing planned callers continue to use `execute-task`. Existing lightweight
entry points use `execute-lightweight-task` without acquiring a plan or
workspace-only planned artifact. Both executors become root-owned. The new
coordination topology applies after the updated bundle is installed and a new
Codex session loads it; an in-flight Task is not hot-swapped between owners.

`search-cache.md` is a per-feature workspace artifact, not an installed managed
asset. The installed Task-orchestrator agent profile is retired, while
`execute-task` and `execute-lightweight-task` remain at their existing
destinations. The implementation must keep both executor contracts, their
root-owned callers and references, fallback prompts, README guidance, and
installer inventory mapping semantically aligned. Rust installer tests use
only test-owned source fixtures and do not read or assert the semantics of
tracked Skills, agent profiles, or Codex configuration. Those assets are
validated and reviewed directly, so an instruction-only change does not require
installer tests.

Local implementation and verification do not install into the live Codex home.
Installation, publication, and branch disposition remain separate
owner-controlled actions.

### Independent operational evaluation

Operational comparison is intentionally outside this source Feature's Design,
Contract, Implementation Plan, and Acceptance. A standalone `benchmark-plan.md`
under `/Users/sakumatomoya/workspace/codex-task-loop-benchmark` may consume an
exact accepted candidate and compare it with an installed baseline. The owner
may wait for that evidence before merging, but benchmark completion or an
efficiency threshold does not make the source Feature accepted or rejected.

A concrete source correctness defect discovered during the benchmark returns to
the ordinary source workflow. Only measurements that depend on a changed
candidate or behavior identity become stale. Benchmark procedure, fixtures,
toolchains, evidence, and lifecycle remain outside this repository and are not
installed runtime guidance.

The next same-workload comparison records quality, elapsed time, tokens by
role, wait calls, tool calls, correction waves, and false escalations. It tests
whether direct root ownership reduces coordination overhead without assuming
that all former Task-orchestrator work disappears. A hierarchical coordinator
is reconsidered only if measured root context or coordination pressure becomes
material while quality gates remain fixed.

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

### Keep one Task orchestrator per Task

Rejected because it retains a context, thread, message hop, wait path, and split
phase ownership for every Task without an observed quality benefit. The root
already owns dependency release and Feature acceptance and can own the same
Task state through exact artifacts and compact role reports.

### Keep a thin Task orchestrator

Rejected because a reduced prompt does not remove the extra context, thread,
root re-entry, or divided phase ownership. Reintroducing a hierarchical Task
coordinator remains an evidence-driven fallback if later workloads demonstrate
a material root context or coordination limit.

### Use a decentralized agent swarm

Rejected because Task and PR ownership, dependency release, exact Git targets,
correction authority, and Feature acceptance require one consistent
coordinator. Runtime-managed concurrency does not transfer those design
responsibilities to arbitrary leaves.

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

### Keep workflow-level Task leases and reviewer grants

Rejected because runtime already owns session thread admission. A second
scheduler requires capacity snapshots, grants, revocation, and per-Task ceilings
without strengthening Task authority. The root retains only selected pending
roles and retries runtime-rejected spawns; policy and phase gates still prevent
invalid overlap.

### Selectively rerun reviewers after correction

Rejected because reviewer invalidation is a new Review-policy mechanism with a
higher risk of missed cross-perspective regressions. Targeted traversal gives
most of the context benefit while preserving the existing selected set and
fresh current correction evidence.

### Reuse the previous review verdict

Rejected because a prior verdict alone cannot make a new head accepted. An exact
prior report may establish unaffected coverage only when targeted inspection of
the current correction confirms that its surface remains bounded.

### Use Luna/Max for verification

Rejected because the owner observed non-response in long verification turns and
the redesigned role is bounded and mechanical. Sol was retained for reliability;
the current mechanical verifier uses the separately approved `low` effort.

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

### Require one RED/edit/GREEN cycle per example

Rejected because several independently observable cases may express one missing
capability and require the same causal edit. A coherent RED/GREEN matrix keeps
the test-first boundary while avoiding repeated edits. Complex state,
equivalence, concurrency, failure, hardware, and performance properties may
also require non-example oracles that small example cycles cannot prove.

### Let the general integrator reject every speculative review item

Rejected because it preserves a costly late validation path for candidates that
never met the common Acceptance threshold. Source reviewers apply the same
threshold before output; integration remains mandatory for concrete admitted
findings.

### Set a fixed maximum handoff length

Rejected because relevance is role- and risk-dependent. A fixed size could omit
material invariants from a difficult system while still allowing duplicated
irrelevant prose. Role ownership, exact source references, and
decision-relevant source inspection provide the useful boundary.

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

### Publish the revision as a separate addendum

Rejected because future consumers would need to resolve two precedence-bearing
documents for one Task-loop contract. This revision keeps one current durable
authority while Git history preserves the previously approved text.
