# [Design Doc] Codex Task-loop optimization

- Owner: Repository owner
- Drafted by: Codex from owner-settled design decisions
- Date: 2026-08-28
- Status: Approved by the repository owner on 2026-08-28
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

This design preserves the Task orchestrator topology and acceptance gates while
optimizing the phases inside one Task loop. It supersedes the earlier generic
lease rule only where it permits spare Task leaves outside the reviewer wave,
and it refines the correction-review behavior without changing which reviewers
the approved Review policy selects.

### Goals

- Keep one Task orchestrator responsible for one complete planned Task loop.
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
- Change `agents.max_threads`, its installer tiers, `agents.max_depth`, or the
  existing maximum of three concurrent Task leaves.
- Add a Task orchestrator to the lightweight path.
- Install, publish, or activate the changed bundle in a live Codex home.

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

## Detailed design

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
publishing, or installing live assets.

## Cross-cutting concerns

### Context and token use

The Verification Matrix removes repeated prose and gives verification one
current-head evidence format. Role-specific messages stop every leaf from
receiving the complete Task orchestration state. Delta-first review reuses
earlier evidence without reusing its verdict.

These are context reductions, not permission to omit exact authority. Every
role keeps direct access to the source artifacts and expands its inspection when
current evidence requires it.

### Reliability and model configuration

The verifier profile remains on `gpt-5.6-sol` and changes from `high` to
`medium` reasoning effort. The role's narrower decision surface and explicit
matrix reduce the need for exploratory reasoning. `low` is not part of the
initial rollout and may replace `medium` only after representative verification
tasks show unchanged target correctness, matrix completeness, failure
classification, and mutation detection.

Official OpenAI model guidance describes `medium` as a balanced starting point,
`low` as latency-oriented, and recommends comparing lower settings on
representative workloads rather than assuming the highest effort is required.

### Compatibility and rollout

Existing approved plans and eligible legacy work retain their authority form.
New Task-loop behavior is encoded in shared skills and profiles and applies
after the updated bundle is installed and a new Codex session loads it.

The installer inventory does not add or remove an asset. It distributes changed
skill and profile bytes through the existing mapping. The implementation must
keep all affected skills, fallback prompts, agent profiles, README guidance, and
asset-contract tests semantically aligned.

Local implementation and verification do not install into the live Codex home.
Installation, publication, or branch disposition remains a separate
owner-controlled action.

### Evaluation

Completion requires fresh repository validation of the changed assets and
focused contract tests. Operational evaluation should compare the old and new
workflow on representative Tasks and corrections using:

- required-evidence completeness;
- verdict and finding agreement;
- missed mutation or target-identity failures;
- total tokens and latency by phase;
- reviewer-wave wall-clock time;
- correction-loop convergence and fallbacks to full traversal.

Lower tokens, turns, or latency count as an improvement only while the existing
quality and evidence bar remains satisfied.

## Alternatives considered

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
