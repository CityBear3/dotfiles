---
name: architecturing
description: Explore architecture and coauthor an evolving Design Doc after model alignment, finalize its approval, then derive the Feature Contract.
---

# Architecturing

Keep material architecture, scope, algorithms, public contracts, and trade-offs
under user control. The purpose is for the human to construct the design as an
architect and retain an understanding they can use to explain, predict, and
revise it. Human and AI jointly shape the framing, models, structure, and
experiments. Agreement with recommendations or decision-tree coverage alone
does not establish this understanding. Grilling supports the work by challenging
premises and guarantees; it is not the default interaction format. Authorship,
an experiment's success, understanding, and document approval are distinct.

## Enter with aligned inputs

Accept from `agentic-engineering-workflow` the confirmed workspace/repository
identity, aligned working model and checkpoint result, requirements and
specification sources with authority/currentness/scope, established constraints,
input/alignment resume checkpoint, pending architectural questions, discovery
evidence, and the Design Doc to create, revise, or reuse. Do not infer alignment from an
artifact's existence or begin solution choices while a material input meaning
or product choice is unresolved.

An exact, current, approved Design Doc can cover unchanged decisions and
readiness dimensions. Reuse that authority without rewriting it or repeating
approval. For a revision, retain its exact approved revision as unchanged
authority while developing the changed slice. A user-authored Draft can be the
editing starting point, but does not establish alignment or approve its proposals.

After `Model Aligned`, create or revise a working Draft even while architecture
is unsettled. Start with known Context, Goals, Non-goals and a useful outline or
clearly separated writing prompts. Do not put provisional designs or unorganized
opinions into a new Draft. Initially keep Detailed design to its chapter heading
and a short writing guide; add subsections from the actual design questions
that emerge in exploration. Explore proposals in the design loop, then consolidate
organized findings in the document. Mark it as Draft and distinguish unresolved
questions from individually settled choices. The coordinator identifies the
target; add no whether-to-document approval. Drafting and scoped discovery do not authorize production
implementation, staging, publication, or a Feature Contract.

## Explore, validate, and write together

Read [references/exploration-guide.md](references/exploration-guide.md) for the
collaborative architecture loop, granularity, frequent validation, artifact
ownership, and re-entry. Read
[references/writing-guide.md](references/writing-guide.md) for chapter purposes
and how to explain the structure and behavior.

For every new document or substantive revision, use
[Design docs — A design doc](https://www.industrialempathy.com/posts/design-doc-a-design-doc/)
as the presentation reference and apply the writing guide's local rules and
readability checks. Read the article when its contents are not already available
in this conversation; reuse an earlier reading rather than fetch it for every
edit. If access is unavailable, continue with the local guide and identify that
reference limitation in the handoff. Do not claim a fresh reading. The article
guides presentation; this skill retains authority, approval, and lifecycle rules.
For a new Japanese Draft, adapt the
[guided Japanese template](assets/design-doc-template-ja.md). Its chapter and
subheading prompts explain what to consider and how to start writing. Reuse a
suitable existing Draft instead of replacing it. Consult the
[complete Japanese example](references/example-design-doc-ja.md) when a finished
explanation would help; its assumed design choices are illustrative, not project
authority or a starting architecture to copy.

Help the engineer think through and contribute their own hypotheses, reasons
and uncertainties when seeking ideas or evaluation. Invite their current thinking
when useful, accept unorganized opinions or concerns as exploration material,
and provide relevant facts or decision points when they need a starting point.
Do not require questions on every turn or advance submission of their views.

Evaluate both participants' premises and proposals on their merits. State
problems and counterexamples with reasons rather than agreeing to accommodate
the engineer's view. Revise the assessment when facts, assumptions or priorities
change; material design choices remain with the engineer.

Develop a whole-system view together in the design loop, then use a heading,
subheading, or related group of sections as the normal work unit. Investigate,
compare approaches and prototype; rough notes, diagrams and pseudocode can
support that exploration. AI may prepare known background, a document outline,
discussion sketches and prototype code; either participant may write or modify
experiments. Consolidate the results into the affected Draft sections rather
than pre-filling them with provisional designs or unorganized opinions.
Support the engineer's trial and revision rather than making reading and
approving AI-generated prose the main exploration activity. Do not require a
blank-page draft, polished human prose, a coding quota, or a fixed first speaker.

Edit the affected sections from that work, preserving intent, rationale and
uncertainty. Return new assumptions or material design choices to exploration;
do not fill them in as settled decisions while polishing. Before AI creates or
edits a Design Doc, show the proposed sections or a useful diff in the
conversation and wait for the user's confirmation to write. Reuse confirmation
already given for that exact text or edit without asking again. This authorizes
the presented edit; holistic design approval remains the exact whole-document
approval below. Keep previews focused on the affected sections rather than
regenerating the whole document each turn. Reconnect changes to the whole-system
view, major paths and related sections. Headings may split, merge or move; a
section can remain tentative while related work proceeds. This adds no
chapter-level design approval or fixed writing round.

Let either participant originate and revise the framing and hypotheses; the user
selects material approaches and accepted burdens. Use discussion and questions
to support this work, honoring the user's pacing without a one-question rule,
an idea quota, or a requirement to end every reply with a question. Reason about
whole paths, composed guarantees and changed conditions, not just assent or
settled branch lists. Grilling is a supporting tool, not the central format.

Validate uncertain claims frequently: concrete scenarios, source/API checks,
isolated PoCs, traces, or measurements as appropriate. Run available lightweight
checks within authorized scope without a new approval for every routine probe.
For consequential checks share the prediction, actual observations,
interpretation, and limits. Support the user's own prototypes without taking
them over. Reuse current evidence and stop repeating checks that do not reduce
the uncertainty. A PoC is design evidence, not production acceptance or adoption
of the proposed architecture.

Update the affected Draft sections as the design changes: selected approaches
and reasons, alternatives, model/API relationships, invariants, evidence and
limits, accepted deferrals, and open questions. Proposals may develop while
writing, but cannot silently become settled contracts. Keep the resume checkpoint
focused on current input/alignment state and source references until final
approval and preservation checking; the Draft holds the evolving architecture.

Keep the current conceptual phase as `Architecture Exploration`,
`Discovery Pending`, or `Design Ready` in coordinator handoffs. These are
handoff meanings, not a new state engine. Input misunderstandings return through
the coordinator to `Model Aligning`; new or changed required outcomes return to
`Requirements Exploration` followed by alignment. Unresolved realization
choices remain in this skill's architecture/Draft loop. Preserve unaffected
decisions and the Draft on re-entry; never narrow requirements to fit a mechanism.

## Develop a reader-oriented explanation

Recover the user's purpose, background, and priorities from the existing
requirements, model alignment, and discussion. AI can write that known foundation;
do not ask the user to repeat it. Accept rough human drafting as part of design
work without requiring polished prose or a separate submission. Ask about a
specific unrecoverable gap rather than inventing a generic pain story.

For a new document, normally begin with a short Context draft and a compact
outline of what readers will learn and in what order. Use feedback to refine
the explanation while exploring the design; this is editing, not chapter
approval. Honor a suitable existing draft, agreed framing, or an explicit request
for a complete Draft, and keep revisions focused. A complete-looking Draft
with open choices is still unapproved and not `Design Ready`.

Scale detail to significant decisions and future readers' needs. Connect the
mechanism, rationale, and material consequences. Identify requirements,
observable specifications, and established constraints with their exact
surviving authorities in Context and scope. Keep them distinct from design
proposals and choices. Within the agreed editing scope, polish the user's notes
and prose while preserving their meaning and reasons; honor any request to keep
wording verbatim. Editorial fluency does not authorize new design decisions.

Build each substantial explanation from what the reader already understands
to the understanding the next passage needs. Apply the writing guide's
continuous-reading check to actual transitions and revise missing links;
formatting compliance alone does not establish this continuity. Use that
diagnostic without per-paragraph role labels or another human approval step.

Show the major module/package hierarchy at the design scope with a tree or
equivalent structural view, mapped to the relevant source files/directories.
Identify each boundary's responsibilities, API owner, consumers, visibility,
supported access/re-export paths, and source dependency direction. Distinguish
a parent module file from its child files, and API ownership from the location
of underlying definitions. Explain the chosen boundaries and which structural
relationships or paths the design fixes; leave local helper structure flexible.

Explain both communication APIs and module/library function or type APIs, and
persistent and domain/type models when significant. Show conceptual models and
major abstractions with focused diagrams, tables, or short signatures, including
ownership, relationships, invariants, and consumer obligations. Describe each
major API in the form selected by the design. For Rust, show the significant
part of a trait declaration when a trait defines the boundary; otherwise show
function signatures or the type's public operations. Do not introduce a trait
solely to document a contract. Distinguish conceptual sketches and explanatory
names from declarations the implementation must follow. Use exact types,
signatures, or representations only when a material guarantee or compatibility
obligation depends on them; leave representation choices that preserve the design to
implementation. Reference maintained definitions rather than exhaustive copies.

Explain module APIs as a coherent caller-facing boundary: role and consumers,
representative types and operations, meaningful usage, guarantees and caller
obligations, and the reasons for that shape. Keep the declarations and their
semantics close enough to read together. Use this explanatory progression
without imposing a fixed subsection set; leave unsettled lifecycle or failure
choices visible instead of filling them in with plausible type names.

## Template

Use [assets/design-doc-template-ja.md](assets/design-doc-template-ja.md) as an
output scaffold, adapting its language and headings to the project. Keep useful
guidance visible while drafting a section, using labeled blockquotes or another
clearly distinct annotation for temporary writing aids. Follow the writing
guide's distinction between these aids and design content. For Detailed design,
include only its chapter heading and a short guide initially. Keep module/API,
state, flow and other coverage dimensions in the skill's references as applicable
checks; let the actual questions determine output subsections. The guidance is
not a questionnaire to complete before work can start. Explore rough ideas in the
design loop and put organized results and open questions in the affected
sections. Consolidate them as decisions
develop, and remove superseded drafting prompts and empty placeholders before
final presentation without losing reasons, evidence or accepted deferrals. Do
not copy the finished example's decisions.

Open Detailed design with the important properties or decision criteria, the
conditions that make them difficult to achieve, and the central approach. Build
the following sections around how the mechanisms address those difficulties,
including reasons and accepted constraints. Use natural prose, lists or diagrams;
do not require fixed headings or invent a difficulty. Keep unknowns explicit in
a Draft. Explain architecture, state transitions, invariants, public interfaces,
schemas, error models and trade-offs when they are part of the contract. Leave
private helper structure and step-by-step implementation to code and the plan.

For gRPC API sections meeting its applicability conditions, read
[references/api-section-format.md](references/api-section-format.md). Describe
module/library APIs and other surfaces with the narrative and conceptual views
appropriate to their contracts; do not impose the gRPC reference format on
them. For detailed-design structure and review criteria, read
[references/detailed-design-guide.md](references/detailed-design-guide.md).

## Assess readiness for final approval

`Design Ready` means the working design and its explanation are ready to
finalize and submit for exact whole-document approval. It is not a prerequisite
for creating or updating a Draft, and is not approval itself. Report it only
when the shared model is aligned and every applicable condition holds:

1. repository-discoverable facts have been investigated;
2. purpose and observable completion conditions are settled;
3. scope, non-goals, constraints, and invariants are settled;
4. significant responsibility boundaries, dependency direction, model
   relationships/ownership, and interface semantics are settled;
5. expected behavior and its verification obligations are settled;
6. applicable failure/recovery, migration, concurrency, authorization,
   performance, and comparable design concerns are settled;
7. material design branches and dependencies are resolved, with evidence
   appropriate to consequential assumptions and its limits recorded;
8. unavailable discovery has an explicit question, handoff, and re-entry
   condition; unfinished material discovery still blocks readiness unless the
   user explicitly accepts its deferral and impact;
9. no material question remains except such explicitly accepted deferrals; and
10. settled choices, reasons, alternatives, evidence, input sources, and accepted
    deferrals are consolidated in the working Draft.

The model and explanation must connect applicable end-to-end paths,
responsibilities, and interactions to the guarantees they establish. Address
material inconsistencies exposed by scenarios, failure cases, or changes in
conditions under the readiness rules above. Settled branch lists alone are
insufficient. Use the collaborative work and evidence to assess this; add no
user exam, score, mandatory teach-back,
or separate approval of understanding. Do not claim to certify a private mental
state from assent or polished prose.

These are applicability dimensions, not mandatory questions or a demand for a
code PoC for every decision. Apply the exploration granularity; private
implementation choices do not block readiness or require deferrals. Reuse exact
current approved coverage and current applicable evidence. Do not claim a
production performance or correctness result from a limited probe.

## Review and approval

Review the developing Draft as part of the loop. When ready to finalize, retain
an exact pre-finalization Draft snapshot, including working decisions, evidence,
accepted deferrals and relevant notes, plus aligned source references and the
resume checkpoint. This is a preservation source, not another edited architecture
record or approval artifact.

For both human- and AI-authored material, check:

- readers without the conversation can follow the background, problem, scope,
  overall approach, and individual decisions without supplying missing links;
- skimming headings, opening sentences, and list items reveals the central
  approach and major decisions before the reader has to reconstruct
  implementation details;
- paragraphs are compact enough to scan; separate operations and conditions
  are easy to locate, with reasons and causal links preserved nearby;
- the explanation builds on established understanding at significant
  transitions, with actual premises, referents, and causal or other links;
- Detailed design connects important properties, actual difficulties and the
  central approach to the mechanisms, reasons and constraints in its sections;
  lists serve their explanatory or navigation role, with needed interpretation;
- condensation and reorganization preserve material decisions, guarantees,
  constraints, and rationale in the appropriate sections or surviving sources;
- goals and non-goals are distinguishable;
- terms and assumptions are defined;
- requirements are measurable where necessary;
- public contracts, failure behavior, and invariants are explicit;
- alternatives include the reason they were rejected;
- the overview establishes the system's major responsibilities, boundaries,
  and relationships so readers can place the detailed decisions;
- the major module hierarchy, relevant source mapping, API owners, supported
  access paths, and source dependencies are explicit and consistent with
  the interface descriptions;
- important models and module/API contracts can be reconstructed from the
  document, with ownership, invariants, and caller obligations explicit;
- ordering, asynchronous handoffs, and concurrency important to the design are
  explained with a sequence diagram or another suitable behavioral view;
- each detailed section has a coherent design question, and mechanisms,
  rationale, and consequences are readable together;
- diagrams or tables clarify real relationships instead of decorating prose;
- implementation details do not obscure design decisions;
- claims have source/evidence or explicit proposal/unknown status in a Draft;
  the final design contains settled choices except explicitly accepted deferrals.

Present the exact complete document for user approval as the one holistic design
approval. A snapshot, editorial exchange, individual choice, or successful PoC
does not replace it. Mechanical consolidation or meaning-preserving clarification
follows the same preview and write-confirmation rule and adds no further design
approval. A new material design question resumes this skill's loop and
invalidates affected readiness; an input change returns to the appropriate
activity through the coordinator.

## Transfer design authority

For a new or revised document, compare the exact approved Design Doc with the
retained pre-finalization Draft snapshot and its individually settled choices,
aligned input authorities, evidence, and accepted changes. For a revision also
check preservation of unchanged prior approved decisions. Verify:

- selected approaches and rationale;
- aligned requirements/specifications or exact surviving source references,
  distinguishable from design decisions;
- rejected alternatives and reasons;
- scope, non-goals, important contracts, constraints, and invariants;
- consequential evidence and its limits;
- explicitly accepted deferrals with their intent and impact.

Do not compare the final document only to itself or discard working notes before
this check. Retire the owned resume checkpoint and Draft snapshot only after exact
approval and successful preservation checking. If coverage is missing, retain
the sources and report the gap; do not silently fill a new material decision.
If repairing a preservation gap changes the final text, present the repaired
exact version for the existing whole-document approval before completing
transfer or deriving the Feature Contract. Do not repeat settled design choices
or introduce another kind of approval.
Requirements originals, agreed supplements, and relied-on experiment evidence
must remain available. An unchanged exact approved document needs no repeated
approval or manufactured snapshot.

Return to `agentic-engineering-workflow` the current phase, working/final Draft
identity and status, aligned sources/currentness, pending questions/evidence,
readiness result, exact approval state, preservation-check result, temporary
checkpoint/snapshot lifecycle, and any affected re-entry. This skill does not enter
implementation planning.

## Derive the Feature Contract after approval

Only after the coordinator confirms approval of the exact Design Doc and, for a
new or revised document, a successful authority-transfer check, derive a
separate Feature Contract for the current feature. Use the approved document as
the durable design source and repository evidence for its current application.
The Feature Contract must state:

- context and goal;
- scope and non-goals;
- design sources, applied decisions, and precedence;
- observable behavior, compatibility, and material failure behavior;
- responsibilities, interfaces, and important unchanged boundaries;
- protected constraints and invariants;
- verification obligations;
- evidence-backed assumptions and explicitly approved deferrals.

Include applicable conditional concerns such as state transitions, API or event
semantics, schema lifecycle, errors and recovery, concurrency, authorization,
performance, migration, or rollback. Place interface detail at the earliest
layer where an independent consumer or implementation task depends on it; do not
promote private helper structure into the contract.

If a complete Feature Contract cannot be derived without changing or adding a
material architecture, responsibility, public contract, schema, error model,
scope, or trade-off, return the gap to the coordinator so the Design Doc or
design discussion can be reopened. Do not silently supplement an approved Design
Doc during contract drafting.

Return the complete Feature Contract for separate user approval and
workspace-only storage at
`docs/plans/YYYY-MM-DD-<feature>/feature-contract.md`. Keep it ignored and do not
force-add, stage, or commit it unless the user explicitly chooses archival.
Design Doc approval does not approve this artifact, and this skill does not
enter planning.
