# Explore architecture through the Design Doc

Use this guide inside `design-doc` after the shared-model checkpoint. The
working Draft is the canvas for architecture, explanation, and validation;
Design Readiness is required for final approval, not for opening the Draft.

## Start with a map and one material question

Recover the aligned requirements, observable specifications, established
constraints, and existing decisions before proposing solutions. Build a coarse
complete map so the user can place each question in the whole design. For a
small change, this may be a short explanation rather than a diagram.

Work through one consequential question at a time. Human and AI can both offer
hypotheses, sketches, or alternatives; fix neither the first speaker nor the
number of options. Allow an unfinished intuition or a concrete example to
start a branch. When the user has no initial proposal, offer a small tentative
model and the relevant facts rather than demanding a blank-page design.

Make a proposal's assumptions and intended guarantee visible. Compare viable
alternatives against the same aligned inputs, and challenge both human and AI
proposals with concrete counterexamples. Do not turn the interaction into a
sequence of recommendations requiring only assent, flatter a human proposal,
or manufacture alternatives to fill the document. The user owns material
architecture, scope, algorithms, contracts, and trade-offs.

Use the Draft to expose unclear responsibility, model relationships,
dependencies, and behavior. When a choice settles, update its explanation and
rationale in place. Re-read the affected path: a diagram, API sketch, or
paragraph can reveal a missing premise and start the next question.

## Granularity

Architecture includes significant decisions at the relevant system or component
scope: communication APIs and module/library function or type APIs (interfaces,
traits, callbacks), persistent schemas, and domain/type models. Examine
responsibility and data ownership, dependency direction, invariants,
compatibility, change impact, and major success/failure/recovery flows.

Keep this at Design Doc granularity. Explain conceptual models and major API
abstractions, including their principal operations and guarantees. Public
visibility alone does not make every type, method, or signature a user-owned
decision. Use exact types, signatures, or representations when a concrete
compatibility obligation, consumer guarantee, or coordination between independent
implementation tasks depends on that exact choice. Leave helper decomposition
and representations that preserve settled guarantees to implementation; they
are not material open questions or deferrals.

Use the [writing guide](writing-guide.md) for C4 structural views drawn with
Mermaid flowcharts, focused model/API views, sequence diagrams, and connected
prose. Sketches explain candidates as well as selected decisions; do not make
a provisional diagram look like an approved contract.

## Validate frequently and proportionately

Treat an uncertain claim as something to check as it arises. Use concrete
scenarios and counterexamples, relevant source/API inspection, small isolated
PoCs, traces, or measurements according to what would actually resolve it.
Perform available lightweight checks within the authorized scope rather than
collecting all validation for a late phase. Reuse current applicable evidence;
do not repeat a successful check solely because another paragraph changed.

For consequential checks, make the uncertainty and expected outcome explicit,
share the actual observations, and interpret the difference together before
recommending a design update. Give the user opportunities to predict behavior
or change an input; do not require a prediction or approval before every routine
lookup or probe. A validated hypothesis can justify retaining the current
design as well as changing it.

AI may research, create, and run a scoped PoC while the human shapes the
question and interprets results. When the user is using code or a visual model
to think, support their investigation and critique without taking over their
prototype. Frequent validation does not require a code PoC for each naming or
private-helper choice, and it does not create a formal `verify`/review gate
for each experiment.

Record what was tested, relevant conditions, observed result, implications, and
limits in the affected Draft section or a precise supporting reference. A
successful PoC establishes only the tested claim: it is neither selection of
the proposed design nor production acceptance. Later implementation still
needs its own authorized contracts and verification.

If evidence requires an unavailable environment, user-led experiment, or other
discovery work, report `Discovery Pending` with the exact question, attempted
checks, evidence needed, and re-entry condition. Resume the affected branch
when it arrives and preserve unaffected decisions. Stop repeating probes that
do not reduce the same material uncertainty; do not silently call the question
resolved or deferred.

## Keep one architecture work product

Keep selected approaches and rationale, viable alternatives and their reasons,
evidence and limits, accepted deferrals, and open architectural questions in the
Draft. Distinguish proposals, assumptions, and individually settled decisions
without requiring a state engine or a label on every sentence. Only an explicit
user choice settles a material decision; a polished explanation or passing PoC
does not. A deferral requires the user's acceptance of its intent and impact.

The pre-design `decision-record.md` retains input/alignment history and exact
source identities. Do not maintain a second evolving architecture there. Link
its aligned inputs into the Draft; requirements originals and agreed
supplements remain independent input authority. Keep the record until the
final approval and preservation check, even though architecture updates now
live in the Draft.

For a revision, retain the exact prior approved document/revision as authority
for unchanged decisions while developing the changed slice in the Draft.
Reopen dependent decisions whose meaning changes, not every settled branch.

## Re-enter the right activity

An unclear meaning of established inputs returns through the coordinator to
`Model Aligning`. A new or changed outcome, scope, priority, or required behavior
returns to `Requirements Exploration`, then alignment. Keep unresolved
realization choices in this architecture/Draft loop. Preserve the Draft and
unaffected settled work across these transitions, updating only affected input
and design slices after re-alignment.

Do not redefine the required result or narrow scope to make a preferred
mechanism succeed. After interruption, recheck repository/authority identity,
the working Draft, input record, and pending evidence. Resume from the earliest
activity whose conditions still hold. Draft existence is not alignment,
readiness, approval, or authorization to implement.
