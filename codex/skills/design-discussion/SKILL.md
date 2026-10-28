---
name: design-discussion
description: Align required outcomes and constraints, then explore user-owned solution choices on planned-path entry or design re-entry.
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

Use `Model Aligning` to establish what must be achieved and the conditions it
must satisfy. Use `Decision Exploration` to choose how to achieve those aligned
requirements. Keep this boundary explicit in questions, summaries, and records.

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

Also distinguish requirements, established constraints or decisions,
current-system facts, and unresolved solution choices. Existing implementation
is context, not automatically a requirement for the new design. Preserve an
explicit user-chosen technology or approved mechanism as an established
constraint; do not reopen it merely to enforce this boundary. Keep tentative
solution suggestions and Agent proposals pending rather than promoting them to
requirements.

Classify by meaning, not technical vocabulary. For example, requiring retries
to avoid duplicate records specifies behavior; choosing idempotency keys or a
database constraint to provide that guarantee is a solution decision unless
already fixed by explicit authority. Investigate current mechanisms as needed,
but leave new algorithms, storage choices, responsibility placement, and
recovery mechanisms for Decision Exploration.

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
`Model Aligned`, `Decision Exploration`, `Discovery Pending`, or `Design Ready`.
These names coordinate handoffs; do not add a state engine or infer that a later
phase holds because a record exists. Stop with the exact unresolved claim and
evidence already tried when repeated questions do not reduce the same material
uncertainty.

An exact, current, explicitly approved Design Doc or no-Design-Doc decision
record can satisfy the readiness dimensions it covers. Do not repeat those
decisions. Past conversation, an unapproved artifact, or an Agent-authored
summary is not approved authority. Reopen only a missing or changed branch and
the settled decisions whose meaning depends on it.

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
section separate from settled decisions. Within the model, keep requirements
and established constraints separate from current-system facts and pending
solution choices; retain the aligned requirement summary at the checkpoint.
Update the decision section as choices settle with:

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

In `Decision Exploration`, compare ways to satisfy the aligned requirements and
constraints. Select architecture, responsibility placement, algorithms,
interfaces, storage, and recovery mechanisms only where they remain open. Use
the required outcomes as the criteria for evaluating these choices.

Keep exploration at the architectural and contract level appropriate for a
Design Doc: responsibility boundaries, dependency direction, key domain
concepts, invariants, consumer-visible guarantees, and material trade-offs.
Public visibility alone, including an API exposed by a core or domain module,
does not make every type, method, or signature a user-owned design decision.

Use exact types, signatures, or representations only when a concrete
compatibility obligation, consumer guarantee, or coordination between
independent implementation tasks depends on that exact choice. Leave method
decomposition, helper types, and representations that preserve settled
guarantees to implementation. These choices are not unresolved material
questions or approved deferrals and require no separate user approval. Return
to design discussion if they would change a settled responsibility, contract,
invariant, or material trade-off.

Resolve only one material decision at a time:

1. state the decision, the aligned requirement or constraint it serves, and why
   it is now reachable;
2. present the smallest set of viable options;
3. explain concrete trade-offs in this codebase;
4. recommend one option and why;
5. ask only that question and let the user decide;
6. record the answer and identify newly reachable branches and dependencies
   between decisions.

If an answer or new evidence invalidates the shared model, return to model
alignment before presenting the next design decision. When exploration or a
need to decompose scope changes or exposes a gap in required behavior, scope,
or constraints, clarify and confirm the affected requirement there. Do not
silently change the target behavior or narrow scope to fit a preferred solution.

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
- On the planned path, default to preserving settled design in a Design Doc.
  With Design Readiness, return a reasoned recommendation to create or revise
  one, reuse an existing approved document, or use only the decision record
  under the coordinator's
  [documentation policy](../agentic-engineering-workflow/references/planned.md#recommend-the-documentation-approach).
  Consider decisions and trade-offs resolved during discussion, not just any
  remaining uncertainty. Let the coordinator obtain the engineer's confirmation
  before Design Doc drafting or decision-record-only Feature Contract drafting.
- Do not implement from this skill.

## Construct a Feature Contract without a Design Doc

When Design Readiness holds and the coordinator returns the engineer-confirmed
decision-record-only approach, present the complete decision record for explicit
user approval as the one holistic design approval. Retain that approved record
as the design authority throughout the active workspace lifecycle. Only then
derive a separate Feature Contract from that record and repository evidence.
Include:

- context and goal;
- scope and non-goals;
- design sources, decisions, and precedence;
- observable behavior, preserved behavior, and material failure behavior;
- responsibilities, interfaces, and important unchanged boundaries;
- protected constraints and invariants;
- verification obligations;
- evidence-backed assumptions and explicitly approved deferrals.

Add conditional concerns such as state transitions, schema lifecycle, error and
recovery semantics, concurrency, authorization, performance, migration, or
rollback only when they apply. If completing the contract would decide
architecture, responsibility, a public or shared interface, schema, error
behavior, scope, or another material trade-off, return that exact ambiguity to
design discussion. Do not fill it in while drafting.

Return the complete Feature Contract to the coordinator for separate user
approval and workspace-only storage at
`docs/plans/YYYY-MM-DD-<feature>/feature-contract.md`. Keep it ignored and do not
force-add, stage, or commit it unless the user explicitly chooses archival. This
skill does not approve the contract, enter planning, or combine it with an
Implementation Plan.

## Handoff

Return the current conceptual phase, current decision record, repository and
existing-authority identity and currentness, working-model alignment state,
aligned required outcomes and established constraints, unresolved understanding
questions, pending solution choices, Design Readiness result, unresolved design
branches and dependencies, relevant evidence and any discovery re-entry
condition, and the documentation recommendation, concrete reasons, exact existing
document coverage, and any applicable prior engineer confirmation to
`agentic-engineering-workflow`.
After the engineer confirms a decision-record-only approach and approves the
record, also return that confirmation, the decision-record approval state, and
draft Feature Contract. Let the coordinator own transitions and Feature Contract
approval.
