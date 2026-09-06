---
name: agentic-engineering-workflow
description: Route engineering work across read-only, lightweight, and planned paths while enforcing approval gates and coordinating cross-phase transitions.
---

# Agentic engineering workflow

Own path classification and cross-phase transitions only. Let each phase skill
own investigation, task execution, plan orchestration, scheduling, verification,
review, triage, and publication mechanics. Follow repository guidance and
explicit user instructions when they are stricter.

Treat `verify`, `review`, and `receiving-code-review` as check-only phases. They
return evidence or classifications and never edit tracked state, commit a fix, or
advance the workflow. This coordinator consumes their results and selects the
next phase.

## Classify the request

Inspect the relevant repository state before selecting a route.

- For an explanation, diagnosis, review, planning, or other read-only request,
  inspect and report without implementing.
- For an explicit change request, use the lightweight path only when its complete
  eligibility contract holds. Otherwise use the planned path.
- Honor a request to skip a phase or avoid agents only when every remaining
  approved contract can still be satisfied. Never invent a user-owned decision
  or silently weaken evidence.

For every transition retain:

- the active path and phase;
- approved scope, non-goals, and the exact alignment source with its approval
  state and currentness;
- the applicable Design Doc or decision record and the Feature Contract's
  source, approval state, storage form, and currentness;
- for planned work, the living decision record location, shared working-model
  alignment state, Design Readiness result, unresolved understanding questions,
  unresolved design branches, and any discovery re-entry evidence;
- the Review context and complete active Review policy;
- for planned work, the Task dependency DAG, PR topology, task workspaces,
  accepted and candidate results, stale descendants, and integration-only
  composition;
- the next automatic action or user-controlled gate;
- the evidence required to leave the phase;
- every unresolved condition that prevents a safe transition.

### Route standalone read-only checks

Treat a user-requested verification or review outside a planned, lightweight,
integration-only, or eligible legacy Task gate as standalone authority. Resolve
an exact committed range, current index/worktree snapshot, or explicit bounded
fileset and pass it to `verify` or `review`. Standalone is not a CLI, session,
branch, or worktree mode and does not require Herdr, a Task Contract, DAG, or
PR topology.

The root owns the standalone target and its native verification-runner,
reviewer and conditional finding-integrator leaves directly. Runtime admission determines which selected
roles start; retain a rejected spawn as pending and retry after a completion or
mailbox event without reducing selected scope. When the user explicitly
prohibits agents, the lead may run compatible checks and perspectives
sequentially. Label the result `standalone-only`; never use it as Task,
coordinator, or Acceptance evidence.

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

When the explicit request and repository evidence unambiguously establish the
goal, expected behavior, scope, and constraints, retain the request's exact
explicit content as the approved alignment source. Add no decision-record file
or separate alignment approval gate. If only non-material omissions remain,
derive one concise alignment record, present it, and ask once for confirmation;
an unpresented or unconfirmed Agent summary is not shared authority. A material
user-owned choice, durable coordination requirement, or unrecoverable in-memory
contract uses the planned path.

Treat the complete lightweight alignment source as implementation approval when
every eligibility criterion holds. Confirm the workspace with
`create-workspace`. Derive one concise in-memory Feature Contract from the
alignment source and repository evidence; because the route is one coherent
task, use the same contract as its Task Contract. Identify the material property
and reliable verification oracle, then apply the `test-driven-development`
applicability decision before selecting discipline. Record TDD as `applicable`,
`not applicable`, or `required but blocked`, with its reason; select a
contract-appropriate baseline and validation discipline when it is not
applicable.

The lightweight Feature/Task Contract must make the context and goal, scope and
non-goals, design sources and approved decisions with precedence, observable and
preserved behavior, compatibility and material failure behavior,
responsibilities and interfaces, protected constraints, verification
obligations, evidence-backed assumptions, and explicitly approved deferrals
unambiguous. Record that there are no approved deferrals when none exist. It adds
no contract file or separate approval gate. Keep it recoverable in the current
handoff and evidence for the duration of the task.

If implementation exposes a disqualifying risk or material decision, preserve
the evidence and stop the lightweight path. Return to `design-discussion`, then
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

Before invoking `execute-lightweight-task`, derive a concise Review context from
the approved request and repository evidence. State the artifact and purpose,
its consumers and interpretation or execution model, material quality criteria
and realistic failures, approved non-problems, and inapplicable assumptions.
Keep it separate from the Review policy.

Materialize the complete lightweight policy before implementation. If completing
that policy requires a material user-owned choice, or observed risk makes
`focused` inappropriate, return to the planned path before invoking
`execute-lightweight-task`. Do not silently select or strengthen policy to keep the
lightweight path.

Use `focused` as the lightweight default:

- one independent `focused-reviewer` covering specification, implementation
  and test quality, with the Feature Lead as direct sole writer;
- a mechanical `verification-runner` before review;
- fixed runner Luna/low, focused-reviewer Sol/high and conditional
  finding-integrator Sol/high bindings; Feature Lead retains session defaults;
- no second feature review when that one Task PR covers the complete contract;
- explicit reasons for skipped perspectives;
- direct root dispatch of phase-valid leaves with no descendants;
- runtime-managed thread admission, preserving selected roles as pending in
  policy order when admission is temporarily unavailable;
- conditional finding integration for overlap, conflict, authority defects,
  scope-sensitive remedies or non-trivial attribution; no integrator for
  all-clean reports or a simple clear finding eligible for direct triage;
- the common Acceptance threshold.

Acceptance keeps only artifact-applicable findings with an approved requirement,
concrete reachable evidence, material consequence, and proportionate correction.
Preference, speculation, generic best practice, optional polish, and objections
to approved decisions without new evidence are not findings. A proposed new
state machine, schema, identity system, or comparable mechanism is `Escalate`
unless it is necessary and proportionate to a proven in-scope violation.

When the required risk or independence makes focused lightweight review
inappropriate, return to planned discussion before editing. Planned adaptive
and deep policies retain separate specification and implementation-quality
review. If any required independent reviewer cannot be established, report
BLOCKED; do not substitute a lead pass. A no-agent instruction conflicting with
an approved gate requires engineer policy authority.

Give `execute-lightweight-task` one plain-language task handoff containing:

- the complete in-memory Feature/Task Contract, including its design sources and
  approved decisions, goal, observable and preserved behavior, compatibility,
  material failure behavior, responsibilities, interfaces, constraints,
  non-goals, verification obligations, assumptions, and approved deferrals;
- the Review context and complete Review policy;
- the discipline and applicable repository guidance;
- task workspace and branch, Task PR identity, planned base ref and exact
  commit, current head, merge base, exact range, inspected diff, and starting
  Git status including index, worktree, and relevant untracked state;
- responsibility and ownership boundaries;
- the responsibility-scoped commit intent and writer authority to select its
  message unless the request contractually fixes that message;
- the applicable verification route and expected observations, including the
  root-owned Task-loop owner's obligation to build one in-memory current-head
  Verification Matrix after the commit and invalidate it on a head, range,
  controlling-authority, or material-route change;
- attributable commits, prior verification and review, concerns, gaps, and
  re-entry evidence when applicable;
- the root-owned lightweight loop identity, selected or pending roles, and
  attributable runtime-rejection or interruption evidence when applicable;
- exact files, signatures, ordering, or commands only when their identity is
  contractually significant.

This is the common Task evidence plus exactly the lightweight variant. Do not
add a Herdr workspace, Task DAG, PR topology, or another planned-only field.

Do not dispatch roles, load reviewer prompts, implement, commit, or manage
corrections in this coordinator.

When an authorized lightweight correction re-enters `execute-lightweight-task`,
retain H1, reports/triage and the complete selected coverage. Require a bounded
H2 commit and rebuilt matrix with fresh H2 verification. The loop owner supplies
review's impact map; finding owners and affected reviewers rerun, while carried
coverage needs explicit non-invalidation evidence. Uncertainty means rerun.
`review` owns those rules. Ordinary planned corrections stay inside the
independent Task Lead's `execute-task`; Feature-level/integration feedback
routes to the owning Task through `execute-plan`.

## Share costly discovery across independent sessions when useful

For new-format planned work, the Feature Lead may create
`docs/plans/YYYY-MM-DD-<feature>/search-cache.md` when another independent Task
Lead or session is likely to reuse a finding that would be costly to rediscover.
It is an optional shared discovery memo, not a same-session memory log or a
record of every search. Use the Task handoff directly for one-off sharing;
do not duplicate it in a cache without a further reuse need.

Keep entries concise: the finding, its exact source, and applicability or
recheck conditions. Include useful negative results only when the same reuse
benefit exists. Use version, ref, path, scope, or observation date when needed
to identify the source and decide whether the finding still applies. Changed
sources or assumptions and contradictory observations require rechecking;
reuse is navigation, not a prohibition on searching again.

The Feature Lead is the only writer. Task Leads and leaves may return useful
cross-session findings for inclusion instead of editing the file. Pass only
relevant entries or directly readable references to consumers. Do not require
cache creation, an empty file, a lookup before every search, or a cache-miss
report. An absent file, absent entry, or stale entry never blocks progress;
perform the needed discovery instead.

If created, the file is ignored, workspace-only, and non-authoritative. It never
substitutes for direct current Git and authority resolution, mechanical
verification, or policy-selected review. Keep it with the Implementation Plan
through publication, feedback re-entry, and disposition evidence. Retire it
only when removal of that exact coordination worktree is separately authorized;
warn that ignored artifacts are not recoverable from Git unless the owner
chooses archival. Lightweight and eligible legacy work do not acquire this
artifact solely to fit the new format.

## Use approval gates on the planned path

Track planned discussion as `Investigating`, `Model Aligning`, `Model Aligned`,
`Decision Exploration`, `Discovery Pending`, or `Design Ready`. These are
handoff semantics, not a separately implemented state engine. Never infer a
later phase from conversation history or artifact existence.

Resolve planned-path entry in this order:

1. Investigate repository facts until the purpose and initial feature boundary
   are identifiable. Resolve any existing design source's exact content,
   approval state, currentness, covered dimensions, and missing or changed
   branches.
2. Use `create-workspace` to establish or confirm the feature checkout, branch,
   and starting ref before writing the first recoverable planned-path artifact.
   An already suitable current checkout is sufficient.
3. When a model or design branch is unresolved and exact current approved
   authority does not fully cover it, require `design-discussion` to create the
   ignored
   `docs/plans/YYYY-MM-DD-<feature>/decision-record.md` after workspace
   confirmation and before it presents the provisional model. The record must
   preserve repository and authority identity, current conceptual phase,
   checkpoint and alignment result, classified model claims, and separate
   provisional from settled state. Do not omit it because the discussion is
   expected to be short or create a duplicate when exact current approved
   authority already covers the work. File existence is not alignment or
   approval. Let `design-discussion` own model construction, understanding
   questions, synthesis, and the shared-model checkpoint. Do not permit options,
   recommendations, or downstream drafting until it reports `Model Aligned`.
4. Only after the shared model is aligned, let the user settle one reachable
   material design decision at a time while `design-discussion` follows branches
   and dependencies.
5. Require `design-discussion` to report Design Readiness before advancing. Do
   not replace the gate with an assertion that the design is probably settled.
6. When a Design Doc is warranted, reuse an exact, current, approved Design Doc
   for unchanged coverage without repeating its completed approval. Otherwise,
   pass the settled source and readiness result to `design-doc`. The temporary
   decision record needs no separate holistic approval. Require user approval of
   the exact new or revised Design Doc as the one holistic design approval, then
   require a successful authority-transfer check before deleting the living
   record.
7. When no Design Doc is warranted, present the complete decision record for the
   one holistic design approval before Feature Contract drafting. Retain that
   approved record as design authority throughout the active workspace
   lifecycle.
8. Construct a complete Feature Contract. After an approved Design Doc and, for
   a new or revised document, its transfer check, use `design-doc` to derive it
   from that source. Without a Design Doc, use `design-discussion` to derive it
   from the approved decision record and repository evidence.
9. Write the Feature Contract at
   `docs/plans/YYYY-MM-DD-<feature>/feature-contract.md` as an ignored,
   workspace-only execution artifact and require its separate user approval.
   Do not force-add, stage, or commit it unless the user explicitly chooses
   archival. Do not treat Design Doc approval, artifact existence, or a
   conversation summary as Feature Contract approval.
10. Only after the Feature Contract is approved and current, use `create-plan` to
   create the ignored, workspace-only `implementation-plan.md` beside it.
   Require separate approval of the complete Implementation Plan, its Task
   Contract set, Review context, and Review policy before using `execute-plan`.
   Do not force-add, stage, or commit the plan unless the user explicitly
   chooses archival.

Design Readiness holds only after `design-discussion` reports an aligned shared
working model and every applicable condition is satisfied:

1. repository-discoverable facts have been investigated;
2. purpose and observable completion conditions are settled;
3. scope, non-goals, constraints, and invariants are settled;
4. applicable responsibility boundaries, dependency direction, and interfaces
   are settled;
5. expected behavior and its verification method are settled;
6. failure and recovery, migration, concurrency, authorization, performance,
   and comparable concerns are settled when applicable;
7. material design branches and dependencies between decisions are resolved;
8. questions requiring another discovery phase have explicit handoffs and
   evidence-based re-entry conditions;
9. no material question remains unresolved except an explicitly accepted
   deferral with recorded intent and impact; and
10. settled decisions are consolidated into the complete living record.

These are applicability dimensions, not ten mandatory user questions. Do not
promote trivial work or manufacture speculative requirements to fill irrelevant
dimensions. Exact current approved authority may satisfy the dimensions it
covers, including consolidation of unchanged decisions, without creating a
duplicate record solely to repeat them.

The coordinator handoff to `design-discussion` carries the route, confirmed
workspace, repository identity and evidence, record location, exact existing
authority, and unresolved evidence. Receive its current conceptual phase,
working-model alignment state, unresolved understanding questions, readiness
result, unresolved design branches, discovery re-entry condition, Design Doc
applicability, and current record. The handoff to `design-doc` carries the
settled source and readiness result. Receive the exact approval state,
transfer-check result, record lifecycle, and any re-entry gap.

When discovery evidence arrives, return through `Model Aligning` before another
design choice. After interruption or compaction, recheck repository identity,
authority currentness, and the recoverable record, then resume at the earliest
phase whose exit conditions remain satisfied. Stop with the exact unresolved
claim and evidence already tried when repeated questions do not reduce the same
material uncertainty.

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

When an applicable Design Doc or approved no-Design-Doc decision record already
exists, verify its exact content, approval state, currentness, and readiness
coverage rather than repeating covered discussion. Past conversation, an
unapproved artifact, or an Agent-authored summary is not approved authority. A
partial gap reopens only the missing branch; a changed choice also reopens every
dependent decision whose meaning may change. A material change to goal, scope,
responsibility, public or shared interface semantics, invariant, failure
behavior, compatibility, or verification obligation invalidates the dependent
approval. Return first to the affected design branch, then reapprove the Feature
Contract and revalidate the complete plan. A meaning change confined to a Task
Contract invalidates Implementation Plan approval.

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

After Implementation Plan approval, require its Task Contracts to fix each Task
workspace mode, branch identity, exact or deterministic starting-ref resolution
rule, and planned PR base. Treat explicit user authorization to start
`execute-plan` as authority to create or reuse those exact non-destructive local
Task workspaces and independent Herdr Task sessions at their approved model and
effort when their Tasks become dependency-ready. Do not establish every
Task workspace before execution or repeat approval for each planned branch or
worktree creation.

That execution authority does not cover an absent or ambiguous identity, a
mismatched existing workspace, an implicit fetch, a change to the user's
coordination checkout, an operation outside the approved plan, or a destructive,
history-rewriting, external, publication, merge, or cleanup action. Preserve the
observed state and stop at the applicable authority or correction boundary.

Pass exact authority paths and approval/currentness evidence, applicable Feature
Contract clauses and Task Contracts, Review context, complete policy,
coordination workspace, Task DAG, PR topology, task workspace rules, retained
decisions, the explicit execution-start authorization, and any promoted
unaccepted range to `execute-plan`. Include an existing optional search-cache
path and relevant reusable entries only when useful to the receiving session.
Reference unchanged source prose instead of copying unrelated sections into
every handoff.
That skill owns readiness, Herdr dispatch of independent Task sessions,
candidate/authoritative handoffs, workspace/session mappings, cross-Task
staleness, promotion reconciliation and exact evidence aggregation. Each Task
Lead is sole writer and local `execute-task` loop owner, dispatching its native
check leaves itself. Feature Lead validates returned evidence, releases
dependencies and alone accepts the Feature. Lightweight work remains directly
Feature-Lead-written without planned artifacts. Task and leaf allocations are
engineer-confirmed with plan approval; Feature defaults are not copied into the
plan, and no runtime promotion/fallback is allowed.

### Continue an eligible legacy plan

Before any re-entry, resolve the workflow revision approved for the plan.
Already-approved or in-flight plans retain their exact topology, allocations
and gates even when their artifact format is current. Use the prior coherent
assets, or stop for recovery/explicit migration if they are unavailable; do not
let a bundle update silently change an active execution. The older-format
exception below concerns artifacts, not permission to migrate execution.

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
`execute-plan` and every final gate.

New work, a legacy plan whose approval or in-flight status cannot be established,
or any material ambiguity uses the new planned path. If continuation needs a new
goal, scope, responsibility, public or shared interface, invariant, failure
behavior, compatibility promise, or verification obligation, preserve state and
return to design; let the owner choose migration rather than performing it
silently.

## Prepare current task and feature evidence

For planned work retain:

- the original implementation base and coordination workspace;
- both approved topologies and every task workspace, branch, planned base,
  merge base, head, exact range, status, changed files, commits, dependency
  evidence, verification, review, triage, and publication state;
- accepted, candidate, stale, blocked, and in-flight results without conflating
  them;
- approved Design Doc when applicable, Feature Contract, complete Task Contract
  set, coverage, shared interfaces, integration-only obligations, Review
  context, and policy;
- an optional search-cache path and reusable cross-session findings when
  present, with the Feature Lead as sole writer;
- exact temporary integration compositions and their evidence;
- concerns, unresolved findings, and every gap.

For lightweight work retain its one exact Task PR and complete recoverable
combined contract. For eligible legacy work retain the original single-range
evidence required by its unchanged plan.

Require no unexplained in-scope state in any task checkout. Re-read affected
branches, bases, heads, ranges, worktrees, and status before every transition.
Standalone verification or review never substitutes for coordinator evidence.

## Advance only on current evidence

Advance automatically within approved local scope:

1. Accept from lightweight `execute-lightweight-task` only a current `Accepted`
   result for its exact Task PR range. When its combined contract has no integration-only
   obligation, that result is also Feature Accepted; do not repeat verification
   or review.
   When a named integration-only obligation remains, use that accepted head and
   tree as the exact single-task integration target and continue at steps 4–6
   with the recoverable combined contract and lightweight policy. Do not invoke
   `execute-plan` or require planned artifacts, a Task DAG, or a multi-PR
   topology for that target.
2. Accept from `execute-plan` only `TasksAccepted` with every Task Contract
   represented by a current authoritative result, both topologies resolved,
   complete coverage, and exact integration composition inputs.
3. Revalidate task currentness and Feature Contract coverage. If no
   integration-only obligation remains, mark the feature accepted without a
   synthetic aggregate range or repeated review.
4. For each integration-only obligation, the Feature Lead builds the exact
   current-composition matrix, including commands, mechanical expectations and
   mutation checks. Pass only that matrix, target, environment and evidence to
   `verify`; dispatch its native runner in the Feature session. Accept only
   fresh `PASS` for the unchanged composition.
   Diagnose a `FAIL` before correction. Route an authorized planned correction
   through its owning Task Contract in `execute-plan`; route an authorized
   lightweight correction directly to its combined-contract Task in
   `execute-lightweight-task`. After the new Task head passes its complete fresh
   task gate, rerun the same named integration evidence.
5. Invoke `review` only when the approved policy requires or conditionally
   triggers a targeted integration perspective. Pass the same exact integration
   authority and evidence to every selected reviewer. Do not invoke ordinary
   full-feature `$review`.
6. Require `review` to supply conditional finding-integrator evidence or
   explicit simple-finding direct-triage eligibility for integration FINDINGS.
   Send the unchanged target and complete evidence to `receiving-code-review`.
   Raw reviewer output never authorizes correction.
   For planned work, route an authorized `Fix` to its owning Task Contract
   through `execute-plan`, mark only semantically affected results and their
   transitive dependents stale, and rerun fresh affected task and integration
   evidence. For lightweight work, route the `Fix` directly to its
   combined-contract Task through `execute-lightweight-task`, then rerun its
   complete fresh task gate and the same named integration evidence on the new
   head. Preserve
   `Push back` while its target and controlling evidence remain unchanged.
   Retain an independent out-of-scope valid problem as a non-blocking concern
   without expanding the current Task or creating a backlog. Return a
   user-owned decision as `Escalate`.
7. Mark Feature Accepted only when every Task PR result and integration
   obligation is current and no finding, policy gap, design gap, or operational
   gap survives.

Diagnose failed verification before correction. Never advance failed or blocked
verification to review, blocked review to triage, unresolved triage to
correction, or incomplete evidence to completion. Stop repeated non-progress
with its observed attempts. Never discard uncertain state to force progress.

### Return authority defects early

When integrated evidence shows that the applicable Design Doc is missing,
contradictory, or materially ambiguous, accept only `Escalate` with reason
`Design Escalation`. Stop every unstarted reviewer and correction queue for the
affected target, preserve already-running read-only reports when they complete,
and return the exact defect and authority evidence to the engineer. Do not
silently repair or reinterpret the Design Doc.

After the engineer approves a Design Doc change, rederive and reapprove only
the Feature Contract, Task Contracts, Review policy, or plan content whose
meaning changed. Mark only Tasks assigned those changed meanings and their
transitive dependents stale. Retain an unchanged Accepted Task after directly
revalidating its exact authority, dependencies, relied-on interfaces, base,
head, range, and status; a Design amendment does not make every Task stale by
default.

## Handle publication and completion boundaries

An internally accepted Task PR is eligible for publication before Feature
Accepted. If the user requests publication, pass only that task's exact current
evidence to `finish-branch` task mode. Publication is optional for dependency
release, remains an external-write gate, and never retires Feature Contract or
Implementation Plan artifacts.

When human feedback arrives for a published Task PR, re-resolve that exact
branch, planned base, head, range, and contract authority and pass the anchored
feedback to `receiving-code-review`. Preserve an accepted result for a verified
`Push back`. Route an authorized `Fix` through the same owning Task Contract and
task correction loop; a new head makes affected descendants stale through both
topologies. Return `Escalate` to the owning approval gate. Any resulting push,
restack, retarget, or PR update remains separately authorized.

After Feature Accepted, pass the complete topology and feature evidence to
`finish-branch` feature mode. Keep ignored plan artifacts and any existing
`search-cache.md` in the coordination worktree; let an explicitly authorized
later removal of that worktree clean them up with the workspace. Preserve
durable Design Docs and present remaining publication or branch-disposition
choices. Archive plan artifacts only when the
user explicitly requests preservation beyond the worktree lifecycle.

Never treat an edit, candidate, successful command, commit, agent self-review,
stale task result, task count, or incomplete integration evidence as workflow
completion. Report each exact Task PR, feature evidence, Review context, policy,
transitions, remaining findings, and every gap.
