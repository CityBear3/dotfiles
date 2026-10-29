---
name: design-discussion
description: Explore unsettled requirements when needed, confirm shared understanding, then scrutinize architectural decisions toward a Design Doc.
---

# Design discussion

Keep architecture, scope, algorithms, public contracts, and material trade-offs
under user control. Act as an investigator and sounding board.

## Establish inputs and select the phase

Accept from `agentic-engineering-workflow` the planned route, confirmed
workspace, repository identity and evidence, living-record location, exact
existing authority and its currentness, and unresolved evidence. Do not persist
a material decision until the coordinator has confirmed the checkout, branch,
and starting ref.

In `Investigating`, resolve repository facts and inspect existing PBI items,
requirements, specifications, and prior decisions before asking the user.
Identify each source's exact reference, revision or observation point, applicable
scope, authority, and currentness. Do not infer agreement from file existence.

Use `Requirements Exploration` when what to build still needs choices. Skip it
when existing sources already establish the needed outcomes and behavior;
partial sources require exploration only of the missing or conflicting slice.
Use `Model Aligning` to confirm shared understanding of those inputs, then
`Architecture Exploration` to develop and scrutinize how to satisfy them.
Keep these activities explicit in questions, summaries, and records.

Report the current conceptual phase as `Investigating`,
`Requirements Exploration`, `Model Aligning`, `Model Aligned`,
`Architecture Exploration`, `Discovery Pending`, or `Design Ready`. These names
coordinate handoffs; do not add a state engine or infer a later phase from an
artifact's existence. Stop with the exact unresolved claim and evidence already
tried when repeated questions do not reduce the same material uncertainty.

## Explore requirements when needed

Work from the user's intent and available evidence to identify the problem,
people or callers affected, desired outcomes, scope and non-goals, priorities,
and observable behavior and completion conditions. The user may not yet know
the answers. Ask one material product question at a time; propose plausible
alternatives when useful, explain their effects on value, behavior, and scope,
and let the user choose. Test tentative requirements against concrete usage
scenarios. Do not invent needs, audiences, constraints, or agreed decisions.

Distinguish a proposed behavior from a chosen one and an assumption from an
established constraint. Preserve source-backed requirements and explicit
user-chosen technical policies. Leave new architecture and implementation
mechanisms for `Architecture Exploration`; a technical proposal is not a new
requirement merely because the Agent recommends it.

Consolidate newly defined requirements and observable specifications in
`requirements.md` or a source-linked supplement at the project's established
requirements location. When creating one, read
[requirements-format.md](references/requirements-format.md). Use a durable,
feature-scoped source when no project convention exists; do not make the ignored
decision record the only surviving requirements source. Drafting does not
authorize staging, publication, or updates to an external PBI or specification.

Reuse readable existing PBI items and specifications as originals. Do not
duplicate or reformat them to match a template. Record their references,
applicable scope, and any agreed supplements; keep newly selected behavior
traceable to its source and rationale. A faithful capture of already settled
conversation inputs is consolidation, not a reason to force exploration.

When the candidate describes what to build well enough to check its meaning,
return it and its sources for `Model Aligning`. Keep unresolved material product
choices in exploration unless the user explicitly accepts a deferral and its
impact. This adds no separate holistic requirements-approval gate: individual
choices remain user-owned, and the shared-model checkpoint confirms the inputs.

## Build and align the working model

Read relevant code, tests, documentation, and history before asking questions.
Present the smallest complete provisional working model that gives the user
enough context to challenge it. Cover, when applicable:

- the problem, desired outcomes, and required observable behavior;
- scope, non-goals, and observable completion conditions;
- established constraints, invariants, and decisions with their exact authority;
- current-system behavior, processing, and responsibility placement as facts;
- required success, failure, recovery, concurrency, or conflict guarantees; and
- assumptions, contradictions, understanding gaps, and pending solution choices.

Distinguish repository-observed facts, user-stated intent, and Agent inference.
The working model is a falsifiable account of the problem, not approved
authority or a design proposal.

Within the model, distinguish requirements (the outcomes and needs to satisfy)
from specifications (observable behavior under stated conditions at the
relevant system or component boundary). Keep both separate from established
constraints or decisions, current-system facts, and unresolved design choices.
Existing implementation is context, not automatically a requirement for the
new design. Preserve an explicit user-chosen technology or approved mechanism
as an established constraint; do not reopen it merely to enforce this boundary.
Keep tentative solution suggestions and Agent proposals pending rather than
promoting them to requirements.

Classify by meaning, not technical vocabulary. For example, requiring retries
to avoid duplicate records specifies behavior; choosing idempotency keys or a
database constraint to provide that guarantee is a solution decision unless
already fixed by explicit authority. Investigate current mechanisms as needed,
but leave new algorithms, storage choices, responsibility placement, and
recovery mechanisms for Architecture Exploration.

Resolve repository-discoverable facts before asking the user. Confirm the
meaning of established requirements and specifications against their sources,
one material misunderstanding or hidden assumption at a time. Choose the
clarification most likely to change the shared understanding of required
behavior, scope, completion conditions, or constraints. An understanding question
clarifies what is needed; do not frame it as a choice of implementation mechanism or attach design
options, a recommendation, or an approval request. Incorporate the answer,
identify any newly exposed contradiction or dependency, and ask the next
material understanding question. Show only the affected model slice after
ordinary answers; re-synthesize the complete model at a branch boundary or
before entering design decisions.

If answering requires a new outcome, priority, scope, or behavioral choice,
explicitly return that slice to `Requirements Exploration`. Do not disguise a
new product decision as clarification or pass it forward as a design choice.
Preserve unaffected settled inputs when resuming alignment.

Treat the shared model as aligned only when no known material error or omission
remains in the problem understanding and the purpose, required behavior, scope,
completion conditions, and established constraints are mutually understood.
State them without selecting an unresolved solution. List remaining how-to
choices and explicit discovery needs separately; those choices need not be
settled to report `Model Aligned`. Unresolved meaning remains an understanding
gap; an unmade product choice returns to `Requirements Exploration`. Neither
may be relabeled as a realization choice to pass the checkpoint. An explicitly
accepted deferral retains its classification, intent, and impact. Give the user a chance to
correct the complete model at that checkpoint. Confirmation of understanding
is not holistic design approval. Do not enter design choices before this
checkpoint.

An exact, current, explicitly approved Design Doc can satisfy the readiness
dimensions it covers. Do not repeat those decisions. Past conversation, an
unapproved artifact, or an Agent-authored summary is not approved authority.
Reopen only a missing or changed branch and the settled decisions whose meaning
depends on it. For already-approved work governed by an earlier workflow,
retain its exact design authority under the coordinator's continuation rules.

## Maintain the living decision record

For planned discussion with an unresolved requirement, model, or design branch
not fully covered by exact current approved authority, create the ignored
workspace artifact
`docs/plans/YYYY-MM-DD-<feature>/decision-record.md` after workspace
confirmation and before presenting a requirements proposal or provisional model.
Do not omit it because the discussion is expected to be short. In a provisional
section, record repository and authority identity, the current conceptual phase,
checkpoint and alignment result, user-stated intent, and material model claims
classified as confirmed, evidence-backed, inferred, or unresolved. Keep this
section separate from settled design decisions. Within the model, keep
requirements, observable specifications, established constraints, current-system
facts, pending product choices, and pending design choices distinct; retain the
aligned inputs at the checkpoint. Record requirement or specification changes
in their model slice, with their authority, rather than as design choices. Update the design decision
section as choices settle with:

- selected approaches and rationale;
- rejected alternatives and reasons;
- scope and non-goals;
- explicitly accepted deferrals with their intent and impact;
- unresolved material questions in a separate section.

Retain requirements-source references, their applicable scope and currentness,
and the draft/aligned status of the working inputs. For existing read-only
sources, keep this metadata here without editing the originals. Newly defined
requirements and specifications must have a surviving source or supplement;
the working record is not their long-term original.

Only explicitly settled choices enter the record as decisions. An Agent
recommendation is not a decision, and file existence is not approval. Do not
treat a question as deferred unless the user explicitly accepts both the
deferral and its impact.

After interruption or compaction, recheck repository identity, authority
currentness, and this record. Resume at the earliest conceptual phase whose exit
conditions remain satisfied; do not infer model alignment or approval from a
summary or file presence.

## Explore

Enter this phase only after the shared-model checkpoint. Resolve understanding
gaps first; presenting an option is not a substitute for learning what the
system currently does or what outcome the user means.

In `Architecture Exploration`, develop a design proposal and challenge its
premises, responsibility placement, structure, dependencies, major processing
flows, and chosen mechanisms against the aligned requirements, specifications, and
constraints. Explore architecture, algorithms, interfaces, storage, and
recovery where they remain open. Use agreed outcomes and boundary behavior as
evaluation criteria; do not silently redefine them while selecting a design.

Architecture includes significant design decisions at the relevant system or
component scope: communication APIs and module/library function or type APIs
(interfaces, traits, callbacks), database models, and domain/type models.
Examine responsibility and data ownership, dependency direction, invariants,
compatibility, and change impact. Assigning a component boundary or mechanism
to realize an agreed guarantee is architectural work; changing the required
outcome itself is a requirements or specification decision.

Keep exploration at the architectural and contract level appropriate for a
Design Doc: responsibility boundaries, dependency direction, key domain
concepts, mechanisms that preserve agreed invariants and consumer guarantees,
and material trade-offs. Public visibility alone, including an API exposed by
a core or domain module, does not make every type, method, or signature a
user-owned design decision.

Use exact types, signatures, or representations only when a concrete
compatibility obligation, consumer guarantee, or coordination between
independent implementation tasks depends on that exact choice. Leave method
decomposition, helper types, and representations that preserve settled
guarantees to implementation. These choices are not unresolved material
questions or approved deferrals and require no separate user approval. Return
to design discussion if they would change a settled responsibility, contract,
invariant, or material trade-off.

Work through one material design question at a time. State the current proposal
or assumption and the aligned input it serves, then ask the question most
likely to expose a weak premise, unclear responsibility, or unproved guarantee.
Use repository evidence and concrete counterexamples to test the answer. Follow
newly reachable design questions and dependencies rather than a fixed checklist.

When an unresolved user-owned choice needs a decision, compare the smallest set
of viable alternatives, explain concrete trade-offs, recommend one with its
reason, and let the user decide. Record only settled choices and their rationale.
Do not manufacture alternatives or repeat settled choices solely to make the
discussion look rigorous. Grilling should deepen the design proposal and its
justification at Design Doc granularity.

Classify a newly exposed gap before the next question. Return misunderstanding
of established inputs to `Model Aligning`; return new or changed outcomes,
scope, priorities, or required behavior to `Requirements Exploration`, then
confirm the affected slice in `Model Aligning`. Keep unresolved realization
choices in `Architecture Exploration`. Record product and architectural choices
separately and preserve unaffected alignment. Do not silently change target
behavior or narrow scope to fit a preferred solution.

When a decision requires a prototype, visual artifact, benchmark, or another
discovery phase, record the explicit handoff and the evidence required to
re-enter that branch. The question remains unresolved until that evidence is
available. Report `Discovery Pending`; when evidence arrives, classify whether
it changes requirements, understanding, or architecture and resume that affected
branch with the evidence. Reuse unaffected alignment. Support user-authored
prototypes with research, diagnostics, or review; do not take over
implementation while the user is using code to explore the design.

## Assess Design Readiness

Report planned design as ready only after the shared model is aligned and every
applicable condition holds:

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

Apply the [exploration granularity](#explore) to readiness: implementation
details that preserve settled design decisions do not block readiness or need
an explicitly approved deferral.

`Design Ready` means the settled design is ready for Design Doc drafting and
approval. It does not substitute the working record for the final design output.

Treat these as applicability dimensions, not ten mandatory user questions. Do
not expand scope or ask about irrelevant dimensions to make the process appear
more rigorous. Report the exact missing branch or dimension when readiness does
not hold; do not fill it by inference. Exact current approved authority may
satisfy the dimensions it covers, including consolidation of unchanged
decisions, without copying them into a duplicate living record.

## Scale the process

- Route bugs through systematic debugging before planning a fix.
- For a small change, the initial model and alignment may take one exchange. For
  a broad system, start with a coarse complete map, examine one subsystem or
  path at a time, and re-synthesize the whole before design decisions.
- On the planned path, deliver the settled design through a Design Doc under
  the coordinator's
  [output policy](../agentic-engineering-workflow/references/planned.md#deliver-the-design-doc).
  Identify the existing document to revise or reuse, or the need for a new one.
  A short Design Doc is sufficient when it preserves the important decisions,
  rationale, and system explanation; do not expand scope to fill it.
- Do not implement from this skill.

## Deliver the design through a Design Doc

With Design Readiness, return the complete settled source to the coordinator
for `design-doc` to draft or revise the durable output. Reuse an exact, current,
approved Design Doc when it already covers the unchanged design. The living
record is working evidence, not an alternative final design authority for new
planned work.

The coordinator obtains approval of the exact Design Doc and, for a new or
revised document, a successful authority-transfer check before retiring the
living record. `design-doc` then derives the separate Feature Contract. This
skill does not approve either artifact, derive a contract directly from the
working record, or enter planning.

## Handoff

Return the current conceptual phase, current decision record, repository and
existing-authority identity and currentness, working-model alignment state,
requirements sources and their currentness, aligned requirements, observable
specifications and established constraints, unresolved understanding questions,
pending product choices kept separate from pending design choices, Design
Readiness result, unresolved design branches and dependencies, relevant evidence
and any discovery re-entry condition, and the target Design Doc or need to create one,
with exact existing document coverage, to `agentic-engineering-workflow`.
Let the coordinator own transitions, Design Doc approval and transfer, and
Feature Contract approval.
