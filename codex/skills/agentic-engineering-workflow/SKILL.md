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
- approved scope, decision source, and non-goals;
- the applicable Design Doc or decision record and the Feature Contract's
  source, approval state, storage form, and currentness;
- the Review context and complete active Review policy;
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
holds. Confirm the workspace with `create-workspace`. Derive one concise
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

Before invoking `execute-task`, derive a concise Review context from the approved
request and repository evidence. State the artifact and purpose, its consumers
and interpretation or execution model, material quality criteria and realistic
failures, approved non-problems, and inapplicable assumptions. Keep it separate
from the Review policy.

Materialize the complete lightweight policy before implementation. If completing
that policy requires a material user-owned choice, or observed risk makes
`focused` inappropriate, return to the planned path before invoking
`execute-task`. Do not silently select or strengthen policy to keep the
lightweight path.

Use `focused` as the lightweight default:

- one combined specification-and-quality per-task gate;
- final `code-reviewer`;
- final `test-coverage-reviewer` only when behavior or tests changed;
- explicit reasons for skipped perspectives;
- a configured maximum of six total threads including the lead unless a stricter
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
`Adaptive` selects final perspectives for recorded risks. `Deep` runs every
perspective applicable to the artifact and observed risks, not every configured
reviewer. If a required independent reviewer cannot be established, report
`BLOCKED`; do not substitute a lead pass. A no-agent instruction that conflicts
with an approved independent gate is `Escalate` unless the user approves a
policy change.

Give `execute-task` one plain-language task handoff containing:

- the complete in-memory Feature/Task Contract, including its design sources and
  approved decisions, goal, observable and preserved behavior, compatibility,
  material failure behavior, responsibilities, interfaces, constraints,
  non-goals, verification obligations, assumptions, and approved deferrals;
- the Review context and complete Review policy;
- the discipline and applicable repository guidance;
- working directory and approved workspace;
- exact task base, which is the current head before implementation;
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
   trade-off remains unresolved, use `design-discussion` and let the user make
   each material choice.
2. As soon as investigation makes the purpose and initial feature boundary
   identifiable, use `create-workspace` to establish or confirm the feature
   workspace before writing the first recoverable planned-path artifact.
3. For settled work with cross-cutting architecture, durable contracts, or
   significant decisions worth preserving, use `design-doc`. Require separate
   user approval of the drafted Design Doc.
4. Construct a complete Feature Contract. After an approved Design Doc, use
   `design-doc` to derive it from that source. Without a Design Doc, use
   `design-discussion` to derive it from the approved decision record and
   repository evidence.
5. Write the Feature Contract at
   `docs/plans/YYYY-MM-DD-<feature>/feature-contract.md` as an ignored,
   workspace-only execution artifact and require its separate user approval.
   Do not force-add, stage, or commit it unless the user explicitly chooses
   archival. Do not treat Design Doc approval, artifact existence, or a
   conversation summary as Feature Contract approval.
6. Only after the Feature Contract is approved and current, use `create-plan` to
   create the ignored, workspace-only `implementation-plan.md` beside it.
   Require separate approval of the complete Implementation Plan, its Task
   Contract set, Review context, and Review policy before using `execute-plan`.
   Do not force-add, stage, or commit the plan unless the user explicitly
   chooses archival.

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

Pass exact authority paths and approval/currentness evidence, applicable Feature
Contract clauses and Task Contracts, Review context, complete policy, working
directory, workspace, task base, retained decisions, and any promoted
unaccepted range to `execute-plan`. Reference unchanged source prose instead of
copying unrelated sections into every handoff. That skill owns dependency order,
per-task handoff, promotion reconciliation, ordered evidence aggregation, and
plan-deviation detection.

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
`execute-plan` and every final gate.

New work, a legacy plan whose approval or in-flight status cannot be established,
or any material ambiguity uses the new planned path. If continuation needs a new
goal, scope, responsibility, public or shared interface, invariant, failure
behavior, compatibility promise, or verification obligation, preserve state and
return to design; let the owner choose migration rather than performing it
silently.

## Prepare concise final-gate evidence

For a coordinator-managed final gate, retain one current evidence summary:

- lightweight or planned path;
- original implementation base, current head, and exact full range;
- current `git status --short` and changed files;
- approved scope, decisions, and non-goals;
- one exact authority form: the planned Design Doc when applicable, Feature
  Contract, complete Task Contract set, and integration-only obligations; the
  complete lightweight combined Feature/Task Contract and its accepted task
  evidence; or the exact eligible legacy plan authority and completion criteria;
- Review context and complete Review policy;
- task commits and reviewer outcomes;
- observed verification commands and results;
- concerns, unresolved findings, and every gap.

Require no unexplained in-scope index, worktree, or untracked source change
outside the committed range. Re-read the current head and status before every
cross-phase transition. Standalone verification or review can answer its direct
request, but never substitutes for coordinator evidence against the current
implementation head.

## Advance only on current evidence

Advance automatically within approved scope:

1. Accept from lightweight `execute-task` only an `Accepted` result for the
   current head and exact task base-to-head range.
2. Accept from `execute-plan` only all ordered task results plus the distinct
   aggregate current head and full implementation range.
3. Build the concise evidence summary and pass it to `verify`. Accept only a
   fresh `PASS` for the same base, current head, full range, changed files,
   unchanged status, and every applicable Feature Contract observation or
   eligible legacy completion criterion, including obligations provable only
   after aggregation.
4. Pass that verification result, the applicable planned contract artifacts,
   complete lightweight combined contract, or exact eligible legacy authority,
   plus Review context, Review policy, exact range, diff, changed files,
   commands, task outcomes, and gaps to `review`.
   Require every selected final reviewer and adversarial integrator to receive
   the same authority and Review context.
5. Accept from `review` only `CLEAN`, `FINDINGS`, or `BLOCKED` for that unchanged
   current head and range. Send concrete current `FINDINGS` and supporting
   evidence to `receiving-code-review` for `Fix`, `Push back`, or `Escalate`
   classification. Do not reinterpret blocked or incomplete evidence as clean.

When global verification fails, diagnose it before acting. Route an authorized
in-scope correction through the active path, then rerun global verification.
Use `Escalate` for a user-owned decision or missing authority; return `BLOCKED`
when the required operational state cannot be established.

For `Fix`, route one bounded correction through the active path using the finding
and current evidence. After acceptance, rerun fresh global verification and the
final review for the updated range.

A `Push back` remains resolved while the reviewed target and controlling evidence
are unchanged; do not repeat the same review solely to reproduce that decision.
After all findings are triaged, continue only when no `Fix`, `Escalate`, or
surviving finding remains. Reconsider a pushed-back finding only with materially
new evidence.

If the same concrete problem repeats without progress or another action would
repeat an observed failed correction, stop and report the attempts and remaining
gap. Use `Escalate` when resolution needs a user-owned decision, new authority,
scope, or policy. Use `BLOCKED` when current operational state cannot be
established. Never discard uncertain state to force progress.

Never advance failed or `BLOCKED` verification to review, blocked review to
triage, unresolved triage to correction, or incomplete evidence to
`finish-branch`.

## Terminate only at a real boundary

Enter `finish-branch` only when fresh verification passes for the exact current
head and full implementation range, the approved Review policy is satisfied,
final review and triage leave no surviving finding or gap, and current status
still matches the reviewed evidence. Pass the concise final-gate evidence to
`finish-branch`. Let it inspect the active workspace contracts, remove the
ignored Feature Contract and Implementation Plan after their evidence is no
longer needed, preserve any durable Design Doc, then stop for the user's
publication or branch-disposition choice. Retain or archive plan artifacts only
when the user explicitly requests it.

Never treat an edit, successful command, implementation commit, agent
self-review, stale per-task approval, or incomplete aggregate as workflow
completion. Report concise current-head evidence, Review context, policy,
transitions taken, remaining findings, and every unverified gap.
