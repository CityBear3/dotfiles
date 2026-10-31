---
name: design-discussion
description: Explore unsettled requirements when needed and align shared understanding before architectural exploration through a Draft Design Doc.
---

# Design discussion

Keep architecture, scope, algorithms, public contracts, and material trade-offs
under user control. Act as an investigator and sounding board.

## Establish inputs and select the phase

Accept from `agentic-engineering-workflow` the planned route, confirmed
workspace, repository identity and evidence, input/alignment-record location, exact
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
Use `Model Aligning` to confirm shared understanding of those inputs. After
`Model Aligned`, hand off through the coordinator to `design-doc` for architecture
exploration and Draft coauthoring, even while realization choices remain open.
Keep the input activities distinct from architectural choices in questions,
summaries, and records.

Report the current conceptual phase as `Investigating`,
`Requirements Exploration`, `Model Aligning`, `Model Aligned`, or
`Discovery Pending` for unavailable input evidence. These names
coordinate handoffs; do not add a state engine or infer a later phase from an
artifact's existence. Stop with the exact unresolved claim and evidence already
tried when repeated questions do not reduce the same material uncertainty.

## Explore requirements when needed

Work from the user's intent and available evidence to identify the problem,
people or callers affected, desired outcomes, scope and non-goals, priorities,
and observable behavior and completion conditions. The user may not yet know
the answers. Develop concrete usage scenarios and a provisional account of the
needed behavior together; ask focused questions where input is missing. Propose
plausible alternatives when useful, explain their effects on value, behavior,
and scope, and let the user choose. Select a useful pace rather than require
one question per turn or a round of questions, and honor the user's preference.
Do not invent needs, audiences, constraints, or agreed decisions.

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

Resolve repository-discoverable facts before asking the user. Use the working
model and concrete examples to confirm the meaning of established requirements
and specifications against their sources. Clarify material misunderstandings
and hidden assumptions with focused questions when needed. An understanding
question clarifies required behavior, scope, completion conditions, or
constraints; do not turn it into a choice of a new implementation mechanism,
attach architectural recommendations, or request design approval.

Incorporate corrections into the affected coherent model slice. Explain or
illustrate a missing connection when that helps; an answer need not trigger
another question. Do not impose a question count, fixed rounds, or an exam.
Re-synthesize the complete model at a branch boundary or before design choices,
giving the user enough context to challenge the account. This checkpoint aligns
inputs; architectural model building continues in `design-doc` afterward.

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

An exact, current, explicitly approved Design Doc can supply the input
authority and settled design coverage it establishes. Do not repeat those decisions. Past conversation, an
unapproved artifact, or an Agent-authored summary is not approved authority.
Reopen only a missing or changed branch and the settled decisions whose meaning
depends on it. For already-approved work governed by an earlier workflow,
retain its exact design authority under the coordinator's continuation rules.

## Preserve pre-design inputs and alignment

For planned discussion with unresolved requirements or model understanding not
fully covered by exact current approved authority, create the ignored workspace
artifact `docs/plans/YYYY-MM-DD-<feature>/decision-record.md` after workspace
confirmation and before presenting a requirements proposal or provisional model.
Do not omit it because the discussion is expected to be short or duplicate exact
approved coverage solely for ceremony.

Use this record for repository and authority identity, current conceptual phase,
checkpoint and alignment result, user-stated intent, and material input claims
classified as confirmed, evidence-backed, inferred, or unresolved. Keep
requirements, observable specifications, established constraints, current-system
facts, pending product choices, and pending realization choices distinct. Retain
source references, revisions/currentness, applicable scope, rationale for
user-selected input changes, and accepted input deferrals with intent and impact.
An Agent proposal is not a selected input, and file existence is not approval.

Newly defined requirements/specifications need a durable original or supplement;
the ignored record is not their only surviving source. Keep metadata for
read-only existing sources without editing the originals.

After Model Aligned, keep evolving architecture, alternatives, design rationale,
experiment evidence, and design questions in the Draft owned by `design-doc`, not
in a second architecture section here. The input record remains available through
final Design Doc approval and the preservation check. On input re-entry, update
only affected input claims and alignment, then return them to the same Draft
loop. Preserve unaffected alignment and settled design coverage.

After interruption, recheck repository identity, source currentness, record, and
any existing Draft handoff. Resume at the phase whose exit conditions remain
satisfied; a summary or file presence cannot establish alignment or approval.

## Hand off to collaborative design

After the shared-model checkpoint, return the aligned inputs and remaining
realization questions through `agentic-engineering-workflow` to `design-doc`.
Architecture Exploration and Draft writing are one activity owned there; do not
require settled architecture or Design Readiness before drafting. The Draft
will carry proposals, selected choices and reasons, alternatives, evidence and
limits, accepted deferrals, and open questions.

Identify an existing document to revise or reuse, or the need to create one.
Reuse exact, current, approved coverage without repeating its decisions or
approval. A short Design Doc can suffice; do not expand scope to fill it. The
coordinator's [output policy](../agentic-engineering-workflow/references/planned.md#deliver-the-design-doc)
owns final approval, preservation, and subsequent Feature Contract approval.
This skill does not derive a contract from the input record or enter planning.

Scale alignment to the problem: a small change may take one exchange; a broad
system needs a coarse complete input map, focused questions, and a checkpoint
synthesis. Route bugs through systematic debugging before planning a fix. Do not
implement from this skill.

## Handoff

Return the current conceptual phase, confirmed repository/workspace and exact
source identities/currentness, input-record location, working-model alignment
state, aligned requirements/specifications/constraints and their sources,
unresolved understanding questions, pending product choices separate from
realization choices, accepted input deferrals and impacts, relevant discovery
evidence/re-entry condition, and target Design Doc create/revise/reuse coverage.
Do not label remaining realization choices as input gaps or readiness failures.
Let the coordinator own transitions to the integrated `design-doc` loop and its
final approval, preservation, and separate Feature Contract approval.
