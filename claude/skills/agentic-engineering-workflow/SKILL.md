---
name: agentic-engineering-workflow
description: Route engineering work across read-only, lightweight, and planned paths while enforcing approval gates and coordinating cross-phase transitions. Use for any engineering request that may cross investigation, design, planning, implementation, verification, review, and branch completion.
argument-hint: "[request]"
---

# Agentic engineering workflow

Own path classification and cross-phase transitions only. Let each phase skill
own investigation, task execution, plan orchestration, scheduling, verification,
review, triage, and publication mechanics. Follow repository guidance and
explicit user instructions when they are stricter.

Invoke phase skills through the Skill tool (`/design-discussion`,
`/create-workspace`, `/design-doc`, `/create-plan`, `/execute-plan`,
`/execute-task`, `/verify`, `/review`, `/receiving-code-review`,
`/finish-branch`, `/session-teardown`); never perform another phase's work
inline and never collapse phases. The engineer may invoke `/design-discussion`
directly for consultation; when that discussion reveals a change request, it
returns here for path selection.

Treat `verify`, `review`, and `receiving-code-review` as check-only phases. They
return evidence or classifications and never edit tracked state, commit a fix, or
advance the workflow. This coordinator consumes their results and selects the
next phase.

This skill runs in the coordinator session: the session the engineer opened in
the feature's coordination worktree. Name that session `<feature>-coord`
(`--name` at launch or `/rename`) before any planned task is dispatched, because
Task sessions address their results to it by name.

## Classify the request

Inspect the relevant repository state before selecting a route.

- For an explanation, diagnosis, review, planning, or other read-only request,
  inspect and report without implementing.
- For an explicit change request, use the lightweight path only when its complete
  eligibility contract holds. Otherwise use the planned path.
- Honor a request to skip a phase or avoid agents only when every remaining
  approved contract can still be satisfied. Never invent a user-owned decision
  or silently weaken evidence. When the user overrides the execution mechanism
  ("direct edit", "no agents"), ask one separate question whether the
  verification and review loop still applies; do not bury that question in a
  list of confirmations.

For every transition retain:

- the active path and phase;
- approved scope, decision source, and non-goals;
- the applicable Design Doc or decision record and the Feature Contract's
  source, approval state, storage form, and currentness;
- the Review context and complete active Review policy;
- for planned work, the Task dependency DAG, PR topology, task workspaces and
  Task session names, accepted and candidate results, stale descendants, and
  integration-only composition;
- the next automatic action or user-controlled gate;
- the evidence required to leave the phase;
- every unresolved condition that prevents a safe transition.

## Use the lightweight path only when fully eligible

Require all of these conditions after investigation:

- the user explicitly requested a change;
- the objective, expected behavior, and scope are uniquely determined;
- no architecture, public API or other public contract, schema, or error-model
  decision changes;
- no material user-owned trade-off remains;
- the work is one coherent change;
- the work needs no external write, publication, destructive action, or material
  scope expansion.

Do not use file count or changed-line count as eligibility criteria. Treat
security or permission boundaries, persistent-data migration, concurrency or
recovery guarantees, and data-loss risk as disqualifying unless investigation
shows that the requested change does not alter that contract.

Treat the original change request as implementation approval when every criterion
holds. Confirm the workspace with `/create-workspace`. Derive one concise
in-memory Feature Contract from the request and repository evidence; because the
route is one coherent task, use the same contract as its Task Contract. Select
TDD for production behavior and a contract-appropriate discipline for content,
configuration, refactoring, or mechanical migrations.

The lightweight Feature/Task Contract must make the context and goal, scope and
non-goals, design sources and approved decisions with precedence, observable and
preserved behavior, compatibility and material failure behavior,
responsibilities and interfaces, protected constraints, verification
obligations, evidence-backed assumptions, and explicitly approved deferrals
unambiguous. Record that there are no approved deferrals when none exist. It adds
no contract file or separate approval gate. Keep it recoverable in the current
handoff and evidence for the duration of the task.

If implementation exposes a disqualifying risk or material decision, preserve
the evidence and stop the lightweight path. Return to `/design-discussion`, then
planning after the user settles the revised scope. Do not silently broaden the
policy and continue.

Also promote to the planned path when the work no longer fits one coherent task,
needs durable cross-session coordination, or a material part of the in-memory
contract cannot be recovered after interruption or context compaction. Preserve
observed work and state; do not improvise another lightweight task. Record the
original lightweight task base, current head, exact unaccepted range and commits,
changed files, writer and gate evidence, ownership attribution, concerns, and
gaps. Never let preserved unaccepted work become the new plan's invisible
baseline.

## Prepare the lightweight task

Before invoking `/execute-task`, derive a concise Review context from the approved
request and repository evidence. State the artifact and purpose, its consumers
and interpretation or execution model, material quality criteria and realistic
failures, approved non-problems, and inapplicable assumptions. Keep it separate
from the Review policy.

Materialize the complete lightweight policy before implementation. If completing
that policy requires a material user-owned choice, or observed risk makes
`focused` inappropriate, return to the planned path before invoking
`/execute-task`. Do not silently select or strengthen policy to keep the
lightweight path.

Use `focused` as the lightweight default:

- one combined `code-reviewer` specification-and-quality Task PR gate;
- a Task PR `test-coverage-reviewer` when behavior or tests changed;
- no second feature review when that one Task PR covers the complete contract;
- explicit reasons for skipped perspectives;
- at most four concurrent subagents in this session unless a stricter
  repository limit applies;
- deterministic queueing without reducing selected scope;
- the common Acceptance threshold.

Acceptance keeps only artifact-applicable findings with an approved requirement,
concrete reachable evidence, material consequence, and proportionate correction.
Preference, speculation, generic best practice, optional polish, and objections
to approved decisions without new evidence are not findings. A proposed new
state machine, schema, identity system, or comparable mechanism is `Escalate`
unless it is necessary and proportionate to a proven in-scope violation.

An explicitly approved `adaptive` or `deep` mode replaces the default.
Both require independent read-only specification and quality task reviewers.
`Adaptive` selects task and integration perspectives for recorded risks. `Deep`
runs every perspective applicable to an actual task or integration surface, not
every configured reviewer. If a required independent reviewer cannot be
established, report `BLOCKED`; do not substitute a lead pass. A no-agent
instruction that conflicts with an approved independent gate is `Escalate`
unless the user approves a policy change.

Invoke `/execute-task` in this coordinator session — the lightweight path does
not start a Task session — with one plain-language task handoff containing:

- the complete in-memory Feature/Task Contract, including its design sources and
  approved decisions, goal, observable and preserved behavior, compatibility,
  material failure behavior, responsibilities, interfaces, constraints,
  non-goals, verification obligations, assumptions, and approved deferrals;
- the Review context and complete Review policy;
- the discipline and applicable repository guidance;
- coordination directory, task workspace and branch, and planned PR identity;
- planned base ref and exact commit, current head, and authoritative mode;
- responsibility and ownership boundaries;
- the responsibility-scoped commit intent and writer authority to select its
  message unless the request contractually fixes that message;
- the applicable verification route and expected observations;
- exact files, signatures, ordering, or commands only when their identity is
  contractually significant.

Do not dispatch roles, load reviewer prompts, implement, commit, or manage
corrections in this coordinator.

## Use approval gates on the planned path

Resolve planned-path entry in this order:

1. When architecture, scope, algorithms, public contracts, or another material
   trade-off remains unresolved, invoke `/design-discussion` and let the user
   make each material choice.
2. As soon as investigation makes the purpose and initial feature boundary
   identifiable, invoke `/create-workspace` to establish or confirm the feature
   coordination workspace before writing the first recoverable planned-path
   artifact.
3. For settled work with cross-cutting architecture, durable contracts, or
   significant decisions worth preserving, invoke `/design-doc`. Require
   separate user approval of the drafted Design Doc.
4. Construct a complete Feature Contract. After an approved Design Doc, use
   `/design-doc` to derive it from that source. Without a Design Doc, use
   `/design-discussion` to derive it from the approved decision record and
   repository evidence.
5. Write the Feature Contract at
   `docs/plans/YYYY-MM-DD-<feature>/feature-contract.md` as an ignored,
   workspace-only execution artifact and require its separate user approval.
   Do not force-add, stage, or commit it unless the user explicitly chooses
   archival. Do not treat Design Doc approval, artifact existence, or a
   conversation summary as Feature Contract approval.
6. Only after the Feature Contract is approved and current, invoke
   `/create-plan` to create the ignored, workspace-only `implementation-plan.md`
   beside it. When the engineer has produced `project-rules.md` in the same
   directory through `/inject-project-rules`, `create-plan` references its rule
   identifiers; this coordinator never invokes that skill itself. Require
   separate approval of the complete Implementation Plan, its Task Contract
   set, Review context, and Review policy before invoking `/execute-plan`. Do
   not force-add, stage, or commit the plan unless the user explicitly chooses
   archival.

For a promotion with preserved unaccepted work, also give `create-plan` the
recorded lightweight base-to-current range and evidence. Require the new plan to
map every preserved behavior and changed file to its owning Task Contract and a
first promotion-reconciliation step. Keep the original lightweight base as the
aggregate implementation base. If attribution is incomplete, conflicting, or
unsafe, preserve state and stop; do not bless the current head as a clean base.
Immediately before execution, refresh the current head and status. Extend the
reconciliation envelope beyond the recorded promotion head only for attributable
approved design, contract, or plan artifact state; any intervening feature-source
change is a new gap and stops execution.

When an applicable Design Doc or Feature Contract already exists, verify its
exact content, source, approval state, and currentness rather than repeating the
completed gate. A material change to goal, scope, responsibility, public or
shared interface semantics, invariant, failure behavior, compatibility, or
verification obligation invalidates the dependent approval. Return first to the
design source when it is insufficient, then reapprove the Feature Contract and
revalidate the complete plan. A meaning change confined to a Task Contract
invalidates Implementation Plan approval.

Before presenting a revised plan, identify every previously accepted task whose
exact Feature Contract authority, assigned Feature clause meaning, Task Contract
content, dependency, or consumed shared-interface meaning changed. Mark those
results and any transitively dependent results stale. A reapproved contract or
plan does not revive them: require fresh acceptance against both current
authorities before releasing dependents, aggregating completion, or entering
final gates. Retain an accepted result only when its exact Feature authority,
assigned clauses, owning Task Contract, and every relied-on interface and
dependency remain semantically unchanged.

Stop for an unresolved design choice, approval gate, plan deviation, material
scope expansion, external write, publication, merge, discard, destructive
action, or other missing authority. Do not repeat an approval prompt while its
exact decision and artifact remain applicable.

After Implementation Plan approval and before `/execute-plan`, invoke
`/create-workspace` to validate the approved topology: every task identity, its
workspace path, branch name, starting ref, planned PR base, and Task session
name must match the approved plan, and no task branch may already be checked
out elsewhere. Plan approval fixes the requested topology but does not replace
any separate approval that `create-workspace` requires for a branch or worktree
state change. Task worktrees and Task sessions for tasks that are not yet
dependency-ready are created later by `execute-plan`, through the same skill,
when those tasks become ready. Stop at that gate when needed, then resume only
after every task identity, workspace, branch, starting ref, and planned PR base
matches the approved plan.

Pass exact authority paths and approval/currentness evidence, applicable Feature
Contract clauses and Task Contracts, Review context, complete policy,
coordination workspace, Task DAG, PR topology, task workspace rules and session
names, retained decisions, and any promoted unaccepted range to `/execute-plan`.
Reference unchanged source prose instead of copying unrelated sections into
every handoff. That skill owns readiness, candidate and authoritative task
handoffs, Task session launch and messaging, workspace mapping, staleness
propagation, promotion reconciliation, and exact evidence aggregation.

### Continue an eligible legacy plan

Do not force the new artifact sequence onto a plan that was approved and already
executing before this contract-centered workflow. Treat it as eligible only when
its exact approval and in-flight state are established, its referenced Design
Doc or decision sources remain applicable, no material ambiguity prevents safe
continuation, and the owner did not choose migration.

For that narrow case, keep the approved legacy plan and its referenced design
sources as the execution authority. Resume its existing task specifications,
Review context and policy, verification, review, correction, and completion
criteria without manufacturing Feature or Task Contract files or requiring
reapproval solely for format. Pass the explicit legacy status and authority to
`/execute-plan` and every final gate.

New work, a legacy plan whose approval or in-flight status cannot be established,
or any material ambiguity uses the new planned path. If continuation needs a new
goal, scope, responsibility, public or shared interface, invariant, failure
behavior, compatibility promise, or verification obligation, preserve state and
return to design; let the owner choose migration rather than performing it
silently.

## Prepare current task and feature evidence

For planned work retain:

- the original implementation base and coordination workspace;
- both approved topologies and every task workspace, branch, Task session name,
  planned base, merge base, head, exact range, status, changed files, commits,
  dependency evidence, verification, review, triage, and publication state;
- accepted, candidate, stale, blocked, and in-flight results without conflating
  them;
- approved Design Doc when applicable, Feature Contract, complete Task Contract
  set, coverage, shared interfaces, integration-only obligations, Review
  context, and policy;
- exact temporary integration compositions and their evidence;
- concerns, unresolved findings, and every gap.

For lightweight work retain its one exact Task PR and complete recoverable
combined contract. For eligible legacy work retain the original single-range
evidence required by its unchanged plan.

Require no unexplained in-scope state in any task checkout. Re-read affected
branches, bases, heads, ranges, worktrees, and status before every transition.
Standalone verification or review never substitutes for coordinator evidence. A
Task session's result message is evidence only when its branch, base, and head
match what this coordinator observes in that task's worktree.

## Advance only on current evidence

Advance automatically within approved local scope:

1. Accept from lightweight `/execute-task` only a current `Accepted` result for
   its exact Task PR range. When its combined contract has no integration-only
   obligation, that result is also Feature Accepted; do not repeat verification
   or review.
   When a named integration-only obligation remains, use that accepted head and
   tree as the exact single-task integration target and continue at steps 4–6
   with the recoverable combined contract and lightweight policy. Do not invoke
   `/execute-plan` or require planned artifacts, a Task DAG, or a multi-PR
   topology for that target.
2. Accept from `/execute-plan` only `TasksAccepted` with every Task Contract
   represented by a current authoritative result, both topologies resolved,
   complete coverage, and exact integration composition inputs.
3. Revalidate task currentness and Feature Contract coverage. If no
   integration-only obligation remains, mark the feature accepted without a
   synthetic aggregate range or repeated review.
4. For each integration-only obligation, pass only its exact composed tree,
   accepted Task PR inputs, expected observations, and contract evidence to
   `/verify`. Accept only a fresh `PASS` for the unchanged composition.
   Diagnose a `FAIL` before correction. Route an authorized planned correction
   through its owning Task Contract in `/execute-plan`; route an authorized
   lightweight correction directly to its combined-contract Task in
   `/execute-task`. After the new Task head passes its complete fresh task gate,
   rerun the same named integration evidence.
5. Invoke `/review` only when the approved policy requires or conditionally
   triggers a targeted integration perspective. Pass the same exact integration
   authority and evidence to every selected reviewer. Do not invoke an ordinary
   full-feature review. Integration reviewers run as subagents of this
   coordinator session under the same dispatch rules `review` defines.
6. Send concrete integration `FINDINGS` to `/receiving-code-review`. For planned
   work, route an authorized `Fix` to its owning Task Contract through
   `/execute-plan`, mark affected descendants stale, and rerun fresh affected
   task and integration evidence. For lightweight work, route the `Fix` directly
   to its combined-contract Task through `/execute-task`, then rerun its
   complete fresh task gate and the same named integration evidence on the new
   head. Preserve `Push back` while its target and controlling evidence remain
   unchanged. Return a user-owned decision as `Escalate`.
7. Mark Feature Accepted only when every Task PR result and integration
   obligation is current and no finding, policy gap, design gap, or operational
   gap survives.

Diagnose failed verification before correction. Never advance failed or blocked
verification to review, blocked review to triage, unresolved triage to
correction, or incomplete evidence to completion. Stop repeated non-progress
with its observed attempts. Never discard uncertain state to force progress.

## Handle publication and completion boundaries

An internally accepted Task PR is eligible for publication before Feature
Accepted. If the user requests publication, pass only that task's exact current
evidence to `/finish-branch` task mode. Publication is optional for dependency
release, remains an external-write gate, and never retires Feature Contract or
Implementation Plan artifacts.

When human feedback arrives for a published Task PR, re-resolve that exact
branch, planned base, head, range, and contract authority and pass the anchored
feedback to `/receiving-code-review`. Preserve an accepted result for a verified
`Push back`. Route an authorized `Fix` through the same owning Task Contract and
task correction loop; a new head makes affected descendants stale through both
topologies. Return `Escalate` to the owning approval gate. Any resulting push,
restack, retarget, or PR update remains separately authorized.

After Feature Accepted, pass the complete topology and feature evidence to
`/finish-branch` feature mode. Keep ignored plan artifacts in the coordination
worktree; let an explicitly authorized later removal of that worktree clean them
up with the workspace. Preserve durable Design Docs and present remaining
publication or branch-disposition choices. Archive plan artifacts only when the
user explicitly requests preservation beyond the worktree lifecycle. After
`finish-branch` completes, invoke `/session-teardown`; ending the session itself
is always the engineer's action.

Never treat an edit, candidate, successful command, commit, agent self-review,
stale task result, task count, or incomplete integration evidence as workflow
completion. Report each exact Task PR, feature evidence, Review context, policy,
transitions, remaining findings, and every gap.
