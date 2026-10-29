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
3. When a requirement, model, or design branch is unresolved and exact current
   approved authority does not fully cover it, require `design-discussion` to
   create the ignored
   `docs/plans/YYYY-MM-DD-<feature>/decision-record.md` after workspace
   confirmation and before the first requirements proposal or provisional
   model. The record must preserve repository and authority identity, current conceptual phase,
   checkpoint and alignment result, classified model claims, and separate
   provisional from settled state. Do not omit it because the discussion is
   expected to be short or create a duplicate when exact current approved
   authority already covers the work. File existence is not alignment or
   approval. Let `design-discussion` own product exploration, model construction,
   understanding questions, synthesis, and the shared-model checkpoint.
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
6. Only after the inputs are aligned, use `Architecture Exploration` to develop
   and challenge significant design decisions against them. Let
   `design-discussion` follow material questions and dependencies, compare
   alternatives when a choice remains, and keep decisions under user control.
   Route misunderstanding of established inputs to `Model Aligning`, new
   required outcomes or behavior to `Requirements Exploration` followed by
   alignment, and unresolved realization choices to `Architecture Exploration`.
   Preserve unaffected settled branches.
7. Require `design-discussion` to report Design Readiness before advancing. Do
   not replace the gate with an assertion that the design is probably settled.
8. With Design Readiness, identify the Design Doc to create or revise, or the
   exact, current, approved document to reuse for unchanged coverage, under
   [the output policy](#deliver-the-design-doc). Pass the settled source,
   readiness result, target document, and existing coverage to `design-doc`.
   Do not add a separate whether-to-document confirmation.
9. Reuse the exact approved Design Doc without repeating its completed
   approval. For a new or revised document, the temporary record needs no
   separate holistic approval. Require user approval of the exact new or revised
   Design Doc as the one holistic design approval, then require a successful
   authority-transfer check before deleting the living record.
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

For new planned work, the durable output of Architecture Exploration is an
approved Design Doc. Create or revise one when the settled design adds or changes
its coverage. Reuse an exact, current, approved document when it already covers the
unchanged design; identify that coverage without manufacturing a duplicate.

Keep aligned requirements, observable specifications, and their authority
distinct from design choices and rationale. Reference the requirements originals
or agreed supplements so their content and authority survive the working record.
Scale the document to the decisions and future reader's needs; a short Design
Doc may be enough. Do not invent alternatives or expand scope to justify it.

The living decision record is intermediate evidence, not an alternative final
design authority or a direct source for a new Feature Contract. `Design Ready`
permits drafting; a new or revised Design Doc still needs exact user approval
and a successful transfer of the settled source before the record is deleted.
Preserve aligned inputs or their exact surviving authority references during
that transfer. Reusing an unchanged approved document needs neither another
approval nor a new temporary record solely for transfer.

The Design Doc's role as the output removes the separate whether-to-document
choice. It does not replace approval of the completed document, Feature
Contract, or Implementation Plan. Do not require holistic approval of both the
temporary record and its replacement Design Doc.

## Assess Design Readiness

Design Readiness holds only after `design-discussion` reports an aligned shared
working model and every applicable condition is satisfied:

1. repository-discoverable facts have been investigated;
2. purpose and observable completion conditions are settled;
3. scope, non-goals, constraints, and invariants are settled;
4. applicable responsibility boundaries, dependency direction, and interface
   semantics are settled;
5. expected behavior and its verification method are settled;
6. failure and recovery, migration, concurrency, authorization, performance,
   and comparable concerns are settled when applicable;
7. material design branches and dependencies between decisions are resolved;
8. questions requiring another discovery phase have explicit handoffs and
   evidence-based re-entry conditions;
9. no material question remains unresolved except an explicitly accepted
   deferral with recorded intent and impact; and
10. settled decisions are consolidated into the complete living record.

Apply the [exploration granularity](../../design-discussion/SKILL.md#explore)
when assessing readiness. Implementation details that preserve settled design
decisions do not block readiness or need an explicitly approved deferral.

These are applicability dimensions, not ten mandatory user questions. Do not
promote trivial work or manufacture speculative requirements to fill irrelevant
dimensions. Exact current approved authority may satisfy the dimensions it
covers, including consolidation of unchanged decisions, without creating a
duplicate record solely to repeat them.

The coordinator handoff to `design-discussion` carries the route, confirmed
workspace, repository identity and evidence, record location, exact existing
authority, and unresolved evidence. Receive its current conceptual phase,
working-model alignment state, requirements-source identities and currentness,
aligned requirements, observable specifications and established constraints,
unresolved understanding questions, pending product and architectural choices
kept separate, readiness result, unresolved design branches,
discovery re-entry condition, target Design Doc or need to create one, exact
existing coverage, and current record. The handoff to `design-doc` carries the
settled source, readiness result, target document, and existing coverage. Receive the exact
approval state, transfer-check result, record lifecycle, and any re-entry gap.

When discovery evidence arrives, classify whether it changes requirements,
understanding, or architecture and resume the affected branch with the evidence.
Reuse unaffected alignment. After interruption or compaction, recheck repository
identity, authority currentness, and the recoverable record, then resume at the
earliest phase whose exit conditions remain satisfied. Stop with the exact unresolved
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
