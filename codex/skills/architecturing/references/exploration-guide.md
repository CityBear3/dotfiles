# Explore architecture through the Design Doc

Use this guide inside `architecturing` after the shared-model checkpoint. The
working Draft is the canvas for architecture, explanation, and validation;
Design Readiness is required for final approval, not for opening the Draft.
Before AI writes its foundation, organized findings or revisions, follow the
skill's
[preview and write-confirmation rule](../SKILL.md#explore-validate-and-write-together).

## Start from known inputs and a document outline

Recover aligned requirements, observable specifications, constraints and exact
existing decisions. Open the one working Draft with known Context, Goals,
Non-goals, an outline and clearly separated writing prompts. Do not pre-fill
it with provisional designs or unorganized opinions. Adapt the
[guided Japanese template](../assets/design-doc-template-ja.md) for a new Draft;
reuse a suitable existing document without imposing another scaffold.

AI may populate known Context, Goals, Non-goals and constraints from aligned
sources. Develop a whole-system view and compare candidate structures in the
design loop, then write organized findings into the affected sections. Keep
unexamined choices open instead of producing a complete narrative ahead of the
engineer's exploration. Do not require the engineer to supply a blank-page
design. Neither the first speaker nor the number of ideas is fixed.

The overall design scope stays visible throughout section work. Revisit the
framing and decomposition when evidence changes them; a decision tree or an
outline is a navigation aid, not proof of shared understanding.

## Explore and draft by heading or subheading

Use a heading, subheading or related group of sections as the normal work unit.
Pick a coherent design question with enough context to connect it to the whole.
Split, merge, rename or reorder headings as the design develops; do not make
template sections a fixed sequence or require each to be finished before the
next can be explored.

The normal flow is:

```text
Model Aligned
    |
    v
Known inputs, outline and writing prompts in the Draft
    |
    v
Choose a heading, subheading or connected group
    |
    v
Explore the whole view and section questions; compare and prototype
    |
    v
AI previews organized results; writes after confirmation and checks consequences
    |   Return to exploration for new questions
    v
Reconnect changes to the whole view, major paths and related sections
    |   Return to section work for remaining questions or inconsistencies
    v
Design Ready, final editing and exact whole-document approval
```

### Investigate, compare and prototype

Either participant may originate an approach. AI can inspect sources, gather
comparison material, prepare base code, or build and run an isolated probe.
Either participant may write or modify its code; switch authors as useful.
Preserve engineer-led experiments rather than taking them over. The engineer
can change conditions, try a competing approach or revise a model, then use
the observations to shape the design. No manual-coding quota establishes
understanding, and code generation does not by itself replace that reasoning.

Return the observations, relevant evidence and limits needed for the current
question, with sources available for inspection. Do not make a long AI research
report a prerequisite for further work. Use the proportional validation rules
below; not every design decision requires a code experiment.

### Consolidate exploration results

Use the design loop for provisional ideas and unorganized opinions. Rough notes,
diagrams and pseudocode can support discussion and experiments without becoming
initial Draft content. Organize the considered approaches, comparison criteria,
observations, reasons, uncertainties and resulting changes before adding them
to the affected Draft sections. Writing may expose the next question; it does
not wait for every architectural choice to settle.

AI can prepare the document foundation and help organize findings within
the agreed scope. Support the engineer's own drafting and trial-and-error as
normal design work without requiring polished human prose, a submission before
each edit, or repeated explanations of aligned inputs. If the engineer has no
starting point, offer a focused tentative sketch or relevant facts to work with.
The main activity should not become reading and assenting to AI explanations.

### Edit the affected sections together

AI organizes and refines the rough material, improving wording, structure,
diagrams and references while preserving intent, uncertainty and reasons.
Consolidate chosen mechanisms in Detailed design and real rejected alternatives
with their reasons in Alternatives considered; avoid duplicating their evolving
explanations. Do not invent a rationale or settle a material assumption to make
the prose look complete. Bring such questions back to exploration.

Present the proposed sections or a useful diff before writing under the skill's
write-confirmation rule. Do not regenerate or require rereading the entire
document after each local edit. Full-document presentation still applies at
final approval. Keep temporary writing aids visually distinct as described in
the [writing guide](writing-guide.md#develop-the-explanation-with-the-user).
Guide text is a writing aid, not a questionnaire or evidence that every prompted
concern applies to the project.

### Reconnect the result to the whole

Update the whole-system view, module/API ownership, source dependencies, major
end-to-end paths and affected sections. Trace how responsibilities and
interactions compose into guarantees, including changed or failing conditions.
Individually plausible sections may still conflict when combined; use those
conflicts to choose the next investigation or edit. Tentative sections can remain
open while related work proceeds. Only resolved material questions or explicitly
accepted deferrals support Design Readiness under the skill's existing rules.

Discussion and questions support this loop. Honor the user's pacing; impose no
one-question progression, mandatory writing round, per-chapter approval or
requirement to end each reply with a question. Grilling may scrutinize a premise
or guarantee, without making the engineer an interview respondent. Repeated
assent does not prove understanding. Offer relevant context, a concrete path or
an experiment when needed, without quizzes, scores or a teach-back gate.

Keep aligned inputs and unchanged approved decisions intact. Proposed material
reframing remains provisional until the engineer chooses it; changes to required
behavior or scope use the re-entry rules below.

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

AI may research, create, modify and run a scoped PoC, or support one written by
the engineer. Code authorship remains flexible while the engineer shapes the
question and interprets results. When the user is using code or a visual model
to think, support that work without taking over. Frequent validation does not
require a code PoC for each naming or private-helper choice, and it does not
create a formal `verify`/review gate for each experiment.

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
