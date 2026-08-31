---
name: design-discussion
description: Build an evidence-backed shared working model before resolving material design choices and assessing Design Readiness or Design Doc applicability. Use on planned-path entry or when downstream work re-enters an affected design branch.
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

Read relevant code, tests, documentation, and history before asking questions.
Present the smallest complete provisional working model that gives the user
enough context to challenge it. Cover, when applicable:

- current processing, behavior, and responsibility placement;
- desired outcome and observable completion;
- constraints, invariants, and non-goals;
- decisions already made and their exact authority;
- assumptions, contradictions, and remaining ambiguity; and
- success, failure, recovery, concurrency or conflict, and verification paths.

Distinguish repository-observed facts, user-stated intent, and Agent inference.
The working model is a falsifiable account of the problem, not approved
authority or a design proposal.

Resolve repository-discoverable facts before asking the user. Then test the
model one material misunderstanding or hidden assumption at a time, choosing
the question whose answer could most change scope, responsibilities, paths, or
later decisions. Do not frame an understanding question as a design choice, and
do not attach options, a recommendation, or an approval request. Incorporate
the answer, identify any newly exposed contradiction or dependency, and ask the
next material understanding question. Show only the affected model slice after
ordinary answers; re-synthesize the complete model at a branch boundary or
before entering design decisions.

Treat the shared model as aligned only when no known material error or omission
remains, current and desired responsibilities and paths are mutually understood,
and every remaining unknown is classified as a design decision or an explicit
discovery need. Give the user a chance to correct the complete model at that
checkpoint. Confirmation of understanding is not holistic design approval.
Do not enter design choices before this checkpoint.

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

For planned discussion that needs a recoverable living record, create the
ignored workspace artifact
`docs/plans/YYYY-MM-DD-<feature>/decision-record.md` after workspace
confirmation and before presenting the provisional model. Keep its
evidence-backed current state, user-stated intent, Agent inferences, and
unresolved model questions in a provisional section separate from settled
decisions. Update the decision section as choices settle with:

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

Resolve only one material decision at a time:

1. state the decision and why it is now reachable;
2. present the smallest set of viable options;
3. explain concrete trade-offs in this codebase;
4. recommend one option and why;
5. ask only that question and let the user decide;
6. record the answer and identify newly reachable branches and dependencies
   between decisions.

If an answer or new evidence invalidates the shared model, return to model
alignment before presenting the next design decision.

When a decision requires a prototype, visual artifact, benchmark, or another
discovery phase, record the explicit handoff and the evidence required to
re-enter that branch. The question remains unresolved until that evidence is
available. Report `Discovery Pending`, then re-enter `Model Aligning` with the
named evidence before presenting another design choice. Support user-authored
prototypes with research, diagnostics, or review; do not take over
implementation while the user is using code to explore the design.

If the design scope is too broad to explore reliably, make scope decomposition
the next user-owned design decision.

## Assess Design Readiness

Report planned design as ready only after the shared model is aligned and every
applicable condition holds:

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
- Use a Design Doc for cross-cutting architecture, durable public contracts, or decisions worth preserving.
- Skip the Design Doc when the settled scope does not need a durable architecture
  artifact, but still construct a Feature Contract before planning.
- Do not implement from this skill.

## Construct a Feature Contract without a Design Doc

When Design Readiness holds and the coordinator confirms that no Design Doc is
warranted, present the complete decision record for explicit user approval as
the one holistic design approval. Retain that approved record as the design
authority throughout the active workspace lifecycle. Only then derive a
separate Feature Contract from that record and repository evidence. Include:

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
unresolved understanding questions, Design Readiness result, unresolved design
branches and dependencies, relevant evidence and any discovery re-entry
condition, and whether the settled decisions warrant a Design Doc to
`agentic-engineering-workflow`.
When no Design Doc is warranted, also return the decision-record approval state
and draft Feature Contract. Let the coordinator own transitions and Feature
Contract approval.
