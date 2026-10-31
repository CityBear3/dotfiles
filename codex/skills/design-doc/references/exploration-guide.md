# Explore architecture through the Design Doc

Use this guide inside `design-doc` after the shared-model checkpoint. The
working Draft is the canvas for architecture, explanation, and validation;
Design Readiness is required for final approval, not for opening the Draft.

## Develop a model the architect can use

Recover aligned requirements, observable specifications, constraints, and exact
existing decisions before proposing solutions. Start with a coarse complete
model that locates the problem, major concepts, responsibilities, and paths.
For a small change, a short explanation or sketch can suffice.

Human and AI both help form the framing and decomposition, not just choose
within an AI-generated menu. Either may offer an unfinished intuition, diagram,
conceptual interface, alternative, or concrete case. When the user has no
initial proposal, contribute a small tentative model and relevant facts rather
than demand a blank-page design. Fix neither the first speaker nor an idea or
option count. The user can delegate sketching, investigation, and PoC work
while remaining involved in the architectural reasoning.

Use the model to explain how requirements map to structure, why responsibilities
and interactions establish guarantees, and what changes when conditions or
requirements change. Trace a relevant path across boundaries; a collection of
individually justified choices may still miss their combined behavior. A
scenario, model edit, or experimental observation can expose that gap. Update
the affected model and explanation, then reconnect them to the whole.

A decision tree is a provisional aid, not proof that the problem was decomposed
correctly or that the design is understood. Revisit concepts, boundaries, and
dependencies when reasoning or evidence challenges them. Keep aligned inputs
and unchanged approved decisions intact; a proposed material reframe stays
provisional until the user chooses it. Changes to required behavior or scope
use the input re-entry rules below.

## Choose the next useful form of collaboration

Work on a coherent slice with enough context to relate it to the whole. Choose
among discussion, drawing, writing, examples, inspection, and experiments based
on what would advance it. Questions can clarify intent or provoke a useful
alternative; do not impose one-question-at-a-time or batched-round progression,
require every response to end with a question, or override user pacing.
Connected concerns may need to be examined together, and either participant can
change the form of the work.

Use grilling to expose an assumption or stress-test a concrete premise,
interaction, or guarantee. Apply the same evidence and counterexamples to human
and AI proposals. It can contribute while ideas are being generated; do not add
a mandatory grilling phase or turn the architect into an interview respondent.
Make candidate assumptions and accepted burdens visible, without manufacturing
alternatives or making agreement with a recommendation the main activity.

When the user cannot reason about a proposal yet, supply the missing context,
walk through a concrete path, or make a small model or experiment available to
react to. Repeated assent warrants checking what is actually settled, not
assuming understanding. Help resolve the affected gap without grading the user,
requiring a quiz or teach-back, or adding an understanding-approval gate. The
purpose is shared architectural work, not proof of the user's private thoughts.

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
When conversation cannot establish how a mechanism behaves, use something
concrete to inspect or try instead of rephrasing the question until the user
guesses. An observation may require revising the question or model itself.

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
