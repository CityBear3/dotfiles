# Planned work

## Use approval gates on the planned path

Track planned discussion as `Investigating`, `Requirements Exploration`,
`Model Aligning`, `Model Aligned`, `Architecture Exploration`,
`Discovery Pending`, or `Design Ready`. These are handoff semantics, not a
separately implemented state engine. Never infer a later phase from conversation
history or artifact existence.

Resolve planned-path entry in this order:

1. Investigate repository facts and existing PBI items, requirements,
   specifications, and design sources until the purpose and initial feature
   boundary are identifiable. Resolve each source's exact reference, revision or
   observation point, authority, applicable scope, currentness, and missing or
   conflicting parts. Existing implementation is context, not automatically a
   requirement. Distinguish missing facts from unresolved product choices.
2. Use `create-workspace` to establish or confirm the feature checkout, branch,
   and starting ref before writing the first recoverable planned-path artifact.
   An already suitable current checkout is sufficient.
3. When requirements or model understanding are unresolved and exact current
   approved authority does not fully cover them, require `design-discussion` to
   create the ignored
   `docs/plans/YYYY-MM-DD-<feature>/decision-record.md` after workspace
   confirmation and before the first requirements proposal or provisional model.
   This input/alignment record retains repository/source identity, phase,
   checkpoint and alignment result, and classified input claims. Do not omit it
   because discussion is short or duplicate exact approved coverage. File
   existence is not alignment or approval. If inputs are already demonstrably
   aligned and only realization choices remain, proceed to the Draft without
   creating a parallel architecture record. Let `design-discussion` own product
   exploration, model construction, understanding questions, and alignment.
4. Use `Requirements Exploration` only when what to build needs choices. Let
   the user select outcomes, scope, priorities, and observable behavior through
   questions, scenarios, and alternatives. Preserve established inputs and
   explore only missing or conflicting slices. Consolidate new requirements in
   a durable `requirements.md` or source-linked supplement using the project's
   format; reuse sufficient PBI items and specifications without copying or
   reformatting them. Track their sources and draft/aligned status. This adds
   no separate holistic requirements-approval gate.
5. Use `Model Aligning` directly for established inputs, or after requirements
   exploration, to confirm shared meaning. Distinguish requirements, observable
   specifications, scope, completion conditions, and established constraints
   from current-system facts and pending product or architectural choices.
   Return new product choices to `Requirements Exploration`; resolve factual or
   interpretation gaps without inventing decisions. Do not require new
   responsibility placement or implementation mechanisms for `Model Aligned`,
   or present architectural options before its shared-model checkpoint.
6. After `Model Aligned`, identify the Design Doc to create or revise, or exact
   current approved coverage to reuse, under [the output policy](#deliver-the-design-doc).
   Pass aligned sources, the checkpoint, input record, pending realization
   questions/evidence, target document, and existing coverage to `design-doc`.
   Do not wait for Design Readiness or add a whether-to-document confirmation.
7. Let `design-doc` own Architecture Exploration and the evolving Draft as one
   loop. Start a coarse complete map and one material question. Human and AI
   both propose and challenge hypotheses without a fixed first speaker or option
   count. Validate uncertainties frequently through proportionate scenarios,
   counterexamples, source/API checks, isolated PoCs, or measurements within
   authorized scope. Share consequential predictions, observations,
   interpretations and limits. Reuse current evidence, preserve user-led
   prototypes, and keep design choices under user control. A successful probe
   is neither adoption nor production acceptance.
8. Receive the current Draft and `design-doc`'s Design Readiness assessment.
   `Design Ready` means ready to finalize and seek whole-document approval;
   it does not authorize the first Draft. Keep unresolved material choices in
   the same loop. Route misunderstanding to Model Aligning and new outcomes,
   scope, or required behavior to Requirements Exploration then alignment,
   preserving unaffected Draft content and settled decisions.
9. Reuse an exact approved unchanged Design Doc without repeat approval. For a
   new or revised document, retain a pre-finalization Draft revision/checkpoint
   and input sources for a non-circular preservation comparison. Require user
   approval of the exact final Design Doc as the one holistic design approval,
   then a successful authority-transfer check before retiring owned temporary
   input records/checkpoints. Individual choices, experiments, chapter editing,
   and snapshots add no separate holistic approval and do not replace this one.
10. Construct a complete Feature Contract. After an approved Design Doc and, for
    a new or revised document, its transfer check, use `design-doc` to derive it
    from that source and repository evidence.
11. Write the Feature Contract at
    `docs/plans/YYYY-MM-DD-<feature>/feature-contract.md` as an ignored,
    workspace-only execution artifact and require its separate user approval.
    Do not force-add, stage, or commit it unless the user explicitly chooses
    archival. Do not treat Design Doc approval, artifact existence, or a
    conversation summary as Feature Contract approval.
12. Only after the Feature Contract is approved and current, use `create-plan` to
    create the ignored, workspace-only `implementation-plan.md` beside it.
    Require separate approval of the complete Implementation Plan, its Task
    Contract set, Review context, and Review policy before using `execute-plan`.
    Do not force-add, stage, or commit the plan unless the user explicitly
    chooses archival.

## Deliver the Design Doc

For new planned work, Architecture Exploration and Design Doc development share
one evolving Draft after Model Aligned. The durable output is the exact approved
Design Doc. Reuse exact, current, approved unchanged coverage without
manufacturing another document or repeating its approval.

Keep aligned requirements/specifications and their authority distinct from
architectural proposals, individually settled choices and reasons, alternatives,
evidence/limits, accepted deferrals, and open questions. The Draft is the single
architecture work product; the decision record retains pre-design inputs and
alignment, not a second evolving architecture. Requirements originals, agreed
supplements, and relied-on evidence remain independently available.

Scale the document to the decisions and future reader's needs. A short document
can suffice. Do not invent alternatives or expand scope to justify it. Drafting
starts before readiness; final approval requires Design Readiness, with no
material unresolved questions except explicitly accepted deferrals and impacts.

Before finalization, preserve an exact working-Draft revision/checkpoint that
retains individually settled choices, reasoning, evidence and working notes.
This is a snapshot, not another evolving architecture document or approval
artifact. After exact final approval, `design-doc` compares it with that
checkpoint, aligned inputs and surviving sources, and prior exact approved
coverage for unchanged decisions. Do not compare the final document only with
itself or discard the source of comparison first. Require preservation of
material choices/reasons, alternatives, guarantees, constraints, evidence/limits,
source references and accepted deferrals before retiring owned temporary records.
A gap retains the sources and returns the affected slice to the loop. Unchanged
approved reuse needs no new checkpoint solely for transfer.

Design Doc approval and this preservation check precede a new Feature Contract;
its separate approval and Implementation Plan/Task/Review approvals remain
required. Drafts, temporary records, and experiments do not authorize production
implementation or supply an alternative final design authority.

## Assess Design Readiness

`design-doc` owns the readiness assessment inside the integrated Draft loop.
The coordinator requires an aligned shared model and every applicable condition:

1. repository-discoverable facts have been investigated;
2. purpose and observable completion conditions are settled;
3. scope, non-goals, constraints, and invariants are settled;
4. significant responsibilities, dependency direction, model relationships and
   ownership, and interface semantics are settled;
5. expected behavior and its verification obligations are settled;
6. applicable failure/recovery, migration, concurrency, authorization,
   performance, and comparable concerns are settled;
7. material design branches and dependencies are resolved with appropriate
   evidence and its limits for consequential assumptions;
8. unavailable discovery has explicit questions, handoffs, and re-entry
   conditions; unfinished material discovery blocks readiness unless the user
   explicitly accepts its deferral and impact;
9. no other material question remains unresolved; and
10. choices, rationale, alternatives, evidence, input sources, and accepted
    deferrals are consolidated in the working Draft.

Apply the [exploration granularity](../../design-doc/references/exploration-guide.md#granularity).
Private implementation choices preserving the design do not block readiness or
need approved deferrals. These are applicability dimensions, not mandatory user
questions or code PoCs for every choice. Reuse exact current approved coverage
and applicable evidence rather than duplicating discovery or documentation.

## Coordinate handoffs and re-entry

To `design-discussion`, pass the route, confirmed workspace/repository, exact
existing authority/currentness, input-record location, and unresolved input
evidence. Receive phase, alignment/checkpoint, classified requirements,
specifications and constraints, source identities/currentness, unresolved
understanding and product choices separate from realization choices, accepted
input deferrals, discovery re-entry, and Design Doc create/revise/reuse coverage.

After Model Aligned, pass those inputs and remaining architectural questions to
`design-doc`. Receive working/final Draft identity, revision and status, pending
questions and evidence, readiness result, exact approval state,
preservation-check result, finalization checkpoint and temporary-record lifecycle,
and any affected re-entry. The coordinator owns cross-phase routing; the phase
skill owns architecture exploration, validation, writing and finalization.

When evidence arrives, classify whether it changes requirements, understanding,
or architecture and resume the affected slice with it. Reuse unaffected
alignment, Draft content, decisions and evidence. After interruption, recheck
repository identity, source currentness, input record, Draft revision and pending
evidence; resume at the phase whose exit conditions remain satisfied. Neither
a summary nor file existence establishes alignment, readiness or approval. Stop
with the exact claim, evidence tried, and re-entry need when repeated questions
or probes do not reduce the same material uncertainty.

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

When an applicable Design Doc already exists, verify its exact content,
approval state, currentness, and readiness coverage rather than repeating
covered discussion. Past conversation, an unapproved artifact, or an
Agent-authored summary is not approved authority. A
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

Apply the [continuation classification](execution.md#recover-within-the-authorized-loop) before treating a procedural gap
as a plan deviation or missing authority. Stop for an unresolved design choice,
approval gate, plan deviation, material scope expansion, external write,
publication, merge, discard, destructive
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
unaccepted range to `execute-plan`. Apply [discovery-sharing.md](discovery-sharing.md) only when another independent
session is likely to reuse costly discovery. Include an existing optional search-cache
path and relevant reusable entries only when useful to the receiving session.
Reference unchanged source prose instead of copying unrelated sections into
every handoff.
That skill owns readiness, Herdr dispatch of independent Task sessions,
candidate/authoritative handoffs, workspace/session mappings, cross-Task
staleness, promotion reconciliation and exact evidence aggregation. Each Task
Lead is sole writer and local `execute-task` loop owner, dispatching its native
check leaves itself. Feature Lead consumes Accepted under the [Accepted consumption boundary](execution.md#consume-accepted-as-a-completed-task-decision),
releases dependencies and alone accepts the Feature. Lightweight work remains
directly Feature-Lead-written without planned artifacts. Task and leaf allocations are
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
