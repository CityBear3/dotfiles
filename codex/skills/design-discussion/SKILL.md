---
name: design-discussion
description: Build shared understanding of requirements and specifications, then develop and challenge design proposals toward a Design Doc.
---

# Design discussion

Keep architecture, scope, algorithms, public contracts, and material trade-offs
under user control. Act as an investigator and sounding board.

## Build and align the working model

Accept from `agentic-engineering-workflow` the planned route, confirmed
workspace, repository identity and evidence, living-record location, exact
existing authority and its currentness, and unresolved evidence. Do not persist
a material decision until the coordinator has confirmed the checkout, branch,
and starting ref.

Use `Model Aligning` to build shared understanding of what to create: required
outcomes, observable specifications, scope, and established constraints. Use
`Design Exploration` to develop and scrutinize how to satisfy those aligned
inputs. Keep this boundary explicit in questions, summaries, and records. The
durable design output is a Design Doc; the living record is intermediate work.

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
recovery mechanisms for Design Exploration.

Resolve repository-discoverable facts before asking the user. Then test the
model one material misunderstanding or hidden assumption at a time, choosing
the question whose answer could most change required behavior, scope, completion
conditions, or constraints. An understanding question clarifies what is needed;
do not frame it as a choice of implementation mechanism or attach design
options, a recommendation, or an approval request. Incorporate the answer,
identify any newly exposed contradiction or dependency, and ask the next
material understanding question. Show only the affected model slice after
ordinary answers; re-synthesize the complete model at a branch boundary or
before entering design decisions.

Treat the shared model as aligned only when no known material error or omission
remains in the problem understanding and the purpose, required behavior, scope,
completion conditions, and established constraints are mutually understood.
State them without selecting an unresolved solution. List remaining how-to
choices and explicit discovery needs separately; those choices need not be
settled to report `Model Aligned`. An unanswered requirement remains an
understanding gap unless explicitly deferred with its intent and impact; do not
relabel it a solution choice to pass the checkpoint. Give the user a chance to
correct the complete model at that checkpoint. Confirmation of understanding
is not holistic design approval. Do not enter design choices before this
checkpoint.

Report the current conceptual phase as `Investigating`, `Model Aligning`,
`Model Aligned`, `Design Exploration`, `Discovery Pending`, or `Design Ready`.
These names coordinate handoffs; do not add a state engine or infer that a later
phase holds because a record exists. Stop with the exact unresolved claim and
evidence already tried when repeated questions do not reduce the same material
uncertainty.

An exact, current, explicitly approved Design Doc can satisfy the readiness
dimensions it covers. Do not repeat those decisions. Past conversation, an
unapproved artifact, or an Agent-authored summary is not approved authority.
Reopen only a missing or changed branch and the settled decisions whose meaning
depends on it. For already-approved work governed by an earlier workflow,
retain its exact design authority under the coordinator's continuation rules.

## Maintain the living decision record

For planned discussion with an unresolved model or design branch not fully
covered by exact current approved authority, create the ignored workspace
artifact
`docs/plans/YYYY-MM-DD-<feature>/decision-record.md` after workspace
confirmation and before presenting the provisional model. Do not omit it
because the discussion is expected to be short. In a provisional section,
record repository and authority identity, the current conceptual phase,
checkpoint and alignment result, user-stated intent, and material model claims
classified as confirmed, evidence-backed, inferred, or unresolved. Keep this
section separate from settled design decisions. Within the model, keep
requirements, observable specifications, established constraints, current-system
facts, and pending design choices distinct; retain the aligned inputs at the
checkpoint. Record requirement or specification changes in their model slice,
with their authority, rather than as design choices. Update the design decision
section as choices settle with:

- selected approaches and rationale;
- rejected alternatives and reasons;
- scope and non-goals;
- explicitly accepted deferrals with their intent and impact;
- unresolved material questions in a separate section.

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

In `Design Exploration`, develop a design proposal and challenge its premises,
responsibility placement, structure, dependencies, major processing flows, and
chosen mechanisms against the aligned requirements, specifications, and
constraints. Explore architecture, algorithms, interfaces, storage, and
recovery where they remain open. Use agreed outcomes and boundary behavior as
evaluation criteria; do not silently redefine them while selecting a design.

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

If an answer or new evidence invalidates the shared model, explicitly return to
`Model Aligning` before the next design question. A question about what outcome
is needed or what observable behavior to guarantee belongs to requirements or
specifications, even when discovered through design. Clarify and align that
affected slice, record it separately from design choices, then resume
`Design Exploration`. Do not silently change target behavior or narrow scope
to fit a preferred solution.

When a decision requires a prototype, visual artifact, benchmark, or another
discovery phase, record the explicit handoff and the evidence required to
re-enter that branch. The question remains unresolved until that evidence is
available. Report `Discovery Pending`, then re-enter `Model Aligning` with the
named evidence before presenting another design choice. Support user-authored
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
aligned requirements, observable specifications and established constraints,
unresolved understanding questions, pending design choices, Design Readiness
result, unresolved design branches and dependencies, relevant evidence and any
discovery re-entry condition, and the target Design Doc or need to create one,
with exact existing document coverage, to `agentic-engineering-workflow`.
Let the coordinator own
transitions, Design Doc approval and transfer, and Feature Contract approval.
