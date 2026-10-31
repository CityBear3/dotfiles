---
name: design-doc
description: Explore architecture and coauthor an evolving Design Doc after model alignment, finalize its approval, then derive the Feature Contract.
---

# Design Doc support

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
pre-design input record, pending architectural questions, discovery evidence,
and the Design Doc to create, revise, or reuse. Do not infer alignment from an
artifact's existence or begin solution choices while a material input meaning
or product choice is unresolved.

An exact, current, approved Design Doc can cover unchanged decisions and
readiness dimensions. Reuse that authority without rewriting it or repeating
approval. For a revision, retain its exact approved revision as unchanged
authority while developing the changed slice. A user-authored Draft can be the
editing starting point, but does not establish alignment or approve its proposals.

After `Model Aligned`, create or revise a working Draft even while architecture
is unsettled. Start with the known Context, Goals, Non-goals and a coarse
Overview or question map. Mark the document as Draft and keep tentative
structures and unresolved questions distinguishable from individually settled
choices. The coordinator identifies the target; add no whether-to-document
approval. Drafting and scoped discovery do not authorize production
implementation, staging, publication, or a Feature Contract.

## Explore, validate, and write together

Read [references/exploration-guide.md](references/exploration-guide.md) for the
collaborative architecture loop, granularity, frequent validation, artifact
ownership, and re-entry. Read
[references/writing-guide.md](references/writing-guide.md) for chapter purposes
and how to explain the structure and behavior.

Build a coarse complete model together, then work on a coherent slice using
discussion, diagrams, conceptual sketches, examples, writing, or probes as
useful. Let either participant originate and revise the decomposition and
hypotheses. Keep framing and dependencies provisional while alternatives are
explored; the user selects material approaches and accepted burdens. Choose
questions when they advance the work, without a fixed one-question or round
rule, a requirement to end each reply with a question, or a human idea quota.
Honor the user's pacing preferences. Use the model to reason about whole paths,
composed guarantees, and changed conditions; do not substitute repeated assent
or a list of settled decisions for that work. Grilling can help expose a premise
or test a claim during exploration without becoming a separate mandatory phase.

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
writing, but cannot silently become settled contracts. Do not maintain a second
evolving architecture in `decision-record.md`; that record retains the aligned
inputs and their history until final approval and preservation checking.

Keep the current conceptual phase as `Architecture Exploration`,
`Discovery Pending`, or `Design Ready` in coordinator handoffs. These are
handoff meanings, not a new state engine. Input misunderstandings return through
the coordinator to `Model Aligning`; new or changed required outcomes return to
`Requirements Exploration` followed by alignment. Unresolved realization
choices remain in this skill's architecture/Draft loop. Preserve unaffected
decisions and the Draft on re-entry; never narrow requirements to fit a mechanism.

## Develop a reader-oriented explanation

Recover the user's purpose, background, and priorities from the existing
requirements, model alignment, and discussion. Require no human-written prose
or repeated explanation. Ask about a specific unrecoverable gap rather than
inventing a generic pain story.

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
proposals and choices. Preserve the user's prose unless an edit is requested.

Explain both communication APIs and module/library function or type APIs, and
persistent and domain/type models when significant. Show conceptual models and
major abstractions with focused diagrams, tables, or short signatures, including
ownership, relationships, invariants, and consumer obligations. Distinguish
explanatory names from exact declarations. Use exact types, signatures, or
representations only when a material guarantee or compatibility obligation
depends on them; leave representation choices that preserve the design to
implementation. Reference maintained definitions rather than exhaustive copies.

## Template

```markdown
# [Design Doc] Title

## Context and scope

### Goals

### Non-goals

## Overview

## Detailed design

## Cross-cutting concerns

## Alternatives considered
```

The detailed design should explain architecture, state transitions, invariants, public interfaces, schemas, error models, and trade-offs when those are part of the contract. Leave private helper structure and step-by-step implementation to code and the implementation plan.

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
an exact pre-finalization Draft revision/checkpoint, including working decisions,
evidence, accepted deferrals, and relevant notes, plus the aligned input record
and references. This is a preservation source, not another edited architecture
record or approval artifact.

For both human- and AI-authored material, check:

- readers without the conversation can follow the background, problem, scope,
  overall approach, and individual decisions without supplying missing links;
- condensation and reorganization preserve material decisions, guarantees,
  constraints, and rationale in the appropriate sections or surviving sources;
- goals and non-goals are distinguishable;
- terms and assumptions are defined;
- requirements are measurable where necessary;
- public contracts, failure behavior, and invariants are explicit;
- alternatives include the reason they were rejected;
- the overview establishes the system's major responsibilities, boundaries,
  and relationships so readers can place the detailed decisions;
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
before final approval adds no separate gate. A new material design question
resumes this skill's loop and invalidates affected readiness; an input change
returns to the appropriate activity through the coordinator.

## Transfer design authority

For a new or revised document, compare the exact approved Design Doc with the
retained pre-finalization Draft checkpoint and its individually settled choices,
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
this check. Retire the owned temporary input record/checkpoint only after exact
approval and successful preservation checking. If coverage is missing, retain
the sources and report the gap; do not silently fill a new material decision.
If repairing a preservation gap changes the final text, present the repaired
exact version for the existing whole-document approval before completing
transfer or deriving the Feature Contract. Do not repeat settled design choices
or introduce another kind of approval.
Requirements originals, agreed supplements, and relied-on experiment evidence
must remain available. An unchanged exact approved document needs no repeated
approval or manufactured checkpoint.

Return to `agentic-engineering-workflow` the current phase, working/final Draft
identity and status, aligned sources/currentness, pending questions/evidence,
readiness result, exact approval state, preservation-check result, temporary
record/checkpoint lifecycle, and any affected re-entry. This skill does not enter
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
