# Requirements source and format

Use this reference when creating or supplementing requirements during planned
discussion. The output describes what to build and the observable behavior it
must satisfy. Architectural decisions and their rationale belong in the Design
Doc, with explicit prior technical choices retained as established constraints.

## Reuse existing sources

- Use readable PBI items, specifications, and requirements documents as originals.
  Identify their exact reference, revision or observation point, applicable scope,
  and authority in the working record. File existence does not establish agreement.
- Resolve discoverable facts first. Explore only missing or conflicting product
  choices; do not silently choose precedence between contradictory authorities.
- Do not transcribe or reformat complete existing sources. Capture newly agreed
  requirements in an authorized update or a source-linked supplement, showing
  which part it adds or supersedes and why. External writes need their authority.

## Create a requirements document

Use `requirements.md` as the default name when a new source is needed, following
the project's documentation layout. Keep its identity feature-scoped and its
location available to downstream sessions. A surviving requirements source must
not depend solely on the temporary ignored decision record or its lifetime.

The following is a reference structure, not a mandatory schema. Concise documents
with purpose, requirements, initial non-goals, non-functional requirements,
environment requirements, and established technical policies are valid too.
Use only applicable sections; add observable specifications, completion criteria,
and unresolved questions where the existing prose does not convey them.
Requirement IDs are useful when cross-references need them, not for every line.

```markdown
# Requirements: <subject>

## Problem and desired outcomes

Who or what is affected, the problem to solve, and the expected value.

## Scope and non-goals

What this work includes and explicitly leaves out.

## Requirements

Required capabilities or outcomes, with their reason and source where useful.

## Observable specifications

Conditions or triggers, expected results, and relevant failure or boundary behavior.

## Non-functional and environment requirements

Applicable quality needs, supported environments, and compatibility obligations.

## Established constraints and technical policies

Prior explicit choices and their authority, distinguished from new proposals.

## Completion criteria

The observable conditions by which the required result can be checked.

## Open questions and assumptions

What remains undecided or unverified, its impact, and the evidence or decision needed.
```

## Alignment and lifetime

Keep draft proposals, hypotheses, and user-settled choices distinguishable.
Record `Draft` or `Aligned` in the document or working record as appropriate;
do not modify a read-only original merely to add workflow metadata. The existing
Model Aligning checkpoint confirms shared understanding, not holistic design
approval or implementation authority. Unresolved material product choices return
to Requirements Exploration unless explicitly deferred with their impact.

Keep the resulting source or agreed supplement available after Design Doc
approval. The Design Doc references these inputs and preserves the architectural
choices made to realize them. Deleting the temporary record must not lose a
requirement, a specification, or the source and meaning of an agreed supplement.
