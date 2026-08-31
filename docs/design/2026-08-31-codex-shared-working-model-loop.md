# [Design Doc] Codex Shared Working Model and Design Decision Loops

- Owner: Repository owner
- Drafted by: Codex from settled owner decisions
- Date: 2026-08-31
- Status: Approved
- Approved by: Repository owner on 2026-08-31
- Authority transfer: Complete; the settled living record was checked and retired
- Refines: `docs/design/2026-08-27-codex-design-alignment-gate.md`
- Extends: `docs/design/2026-08-13-codex-design-centered-contract-workflow.md`

## Context and scope

The approved Design Alignment Gate makes shared understanding a prerequisite to
planned engineering work and requires `design-discussion` to investigate the
repository before exploring one material decision at a time. Its interaction
contract nevertheless moves directly from investigated facts to options,
trade-offs, a recommendation, and a user choice. It does not separately verify
that the user and Agent agree on the current processing and responsibility
placement, desired outcome, constraints, hidden assumptions, or applicable
success and failure paths.

In practice, the Agent can therefore form a plausible summary, propose a design,
and ask for approval before the user has had an opportunity to expose a mismatch
in the problem model. The one-question decision loop limits how choices are
settled, but it does not distinguish a question that corrects understanding from
a question that creates design authority.

This design refines planned discussion into two ordered loops. An understanding
loop first treats the Agent's working model as a falsifiable hypothesis and
repairs one material misunderstanding or hidden assumption at a time. A separate
design-decision loop begins only after the complete model has been re-synthesized
and the user has had a chance to correct it. Design Readiness requires both
model alignment and resolution of all applicable material design branches.

### Goals

- Establish an evidence-backed shared model of the current system and intended
  result before presenting design options or recommendations.
- Separate factual and interpretive clarification from user-owned design
  decisions and approval.
- Give the user a coarse complete context before examining one high-impact
  uncertainty at a time.
- Make the transition from understanding to design decisions explicit and
  recoverable across compaction or a fresh session.
- Return to model alignment when a decision or new evidence invalidates an
  assumption on which later decisions depend.
- Keep the process proportional to the actual design surface rather than the
  number of checklist dimensions.
- Preserve existing approval, Feature Contract, planning, execution, review, and
  publication boundaries after Design Readiness.

### Non-goals

- Add a separate alignment or `grill-me` skill.
- Turn the workflow into a machine-readable state machine, fixed questionnaire,
  or required number of questions.
- Require a full interview for an explicit lightweight change whose behavior,
  scope, constraints, and lack of material trade-offs are already unambiguous.
- Add holistic approval of the working model before the existing Design Doc or
  no-Design-Doc approval boundary.
- Let the Agent use model alignment to introduce a preferred architecture,
  expand scope, or manufacture speculative requirements.
- Change Design Doc authority transfer, Feature Contract approval,
  Implementation Plan approval, Task execution, review selection, publication,
  or branch-disposition policy except where those phases consume the revised
  Design Readiness result.
- Prescribe exact skill wording or private implementation structure.

### Explicit deferrals

None.

## Overview

The planned path uses a coarse-to-focused understanding loop followed by the
existing one-question design-decision loop.

```text
request and repository evidence
              |
              v
   provisional working model
   - current flow and owners
   - desired result
   - constraints and non-goals
   - settled decisions
   - assumptions and unknowns
   - applicable success, failure,
     conflict, recovery, and proof paths
              |
              v
  +------------------------------------------------+
  | understanding / grill-me loop                  |<-----------+
  |                                                |            |
  | choose highest-impact uncertainty              |            |
  | -> ask one non-decision question               |            |
  | -> update the affected model slice             |            |
  | -> expose new contradictions and dependencies  |            |
  +-----------------------+------------------------+            |
                          |                                     |
              material mismatch remains -----------------------+
                          |
                          | no material mismatch
                          v
              re-synthesize complete model
                          |
                          v
              shared-model checkpoint
              (confirmation, not approval)
                          |
              correction needed -------------------------------+
                          |
                          | model aligned
                          v
  +------------------------------------------------+
  | design-decision loop                           |<-----------+
  |                                                |            |
  | one reachable material decision                |            |
  | -> options, trade-offs, and recommendation     |            |
  | -> user choice, record, and dependencies       |            |
  +-----------------------+------------------------+            |
                          |                                     |
              unresolved decision -----------------------------+
                          |
              model invalidated -------------------> understanding loop
                          |
              discovery required -----------------> evidence handoff
                          |                               |
                          |<--- re-enter model alignment-+
                          |
                          | all material decisions resolved
                          v
                Design Readiness Gate
                          |
            Design Doc / decision-record approval
                          |
                          v
                  Feature Contract
```

The working model is not a proposed solution. The checkpoint establishes that
the participants are discussing the same system and outcome; it does not approve
an architecture. Options and recommendations belong only to the second loop.

## Detailed design

### Terminology and interaction contracts

**Working model** is the smallest complete evidence-backed account that gives
the user enough context to challenge the Agent's understanding. It is
provisional and falsifiable. It distinguishes:

- repository-observed facts;
- user-stated intent and already approved authority; and
- Agent inference, assumptions, contradictions, and unresolved questions.

It covers current processing, behavior, and responsibility placement; desired
outcome and observable completion; constraints, invariants, and non-goals;
settled decisions; and applicable success, failure, recovery, concurrency or
conflict, and verification paths. Applicability determines depth; the list is
not a questionnaire.

**Understanding question** tests or repairs the working model. It does not offer
design options, recommend an architecture, or request approval. An answer may
confirm evidence, correct an inference, add a domain constraint, or expose a new
dependency. It is not recorded as a design decision unless the user separately
settles a material choice.

**Design-decision question** resolves one reachable material choice after the
shared model is aligned. It retains the existing interaction contract: state the
decision, present the smallest viable option set, explain codebase-specific
trade-offs, recommend one answer with rationale, ask only that question, then
record the user's choice and newly reachable dependencies.

**Shared-model checkpoint** re-synthesizes the complete current model and gives
the user an explicit opportunity to correct material errors or omissions. It is
an alignment result, not holistic design approval. The later approved Design Doc
or no-Design-Doc decision record remains the design authority.

### Responsibility boundaries

`agentic-engineering-workflow` owns:

- routing explicit changes to lightweight or planned alignment;
- carrying existing authority, repository evidence, workspace identity, and
  unresolved evidence into planned discussion;
- retaining the current conceptual phase, working-model alignment state,
  unresolved understanding questions, design branches, and discovery re-entry
  evidence;
- preventing options, Design Readiness, or downstream drafting from bypassing
  the applicable alignment state; and
- reopening the correct model or decision branch when downstream evidence
  invalidates it.

`design-discussion` owns:

- producing the provisional working model from repository evidence, user intent,
  and exact existing authority;
- selecting and asking one highest-impact material understanding question at a
  time;
- incorporating answers, exposing contradictions and dependencies, and
  re-synthesizing the complete model at branch boundaries;
- reporting the shared-model checkpoint result without treating it as approval;
- entering the existing one-question design-decision loop only after alignment;
- returning to model alignment when later evidence invalidates the model;
- maintaining provisional and settled information without conflating them; and
- reporting Design Readiness and Design Doc applicability.

`design-doc` keeps its existing responsibility. It consumes only a complete
settled source after Design Readiness, drafts durable prose without adding a
decision, returns a newly exposed ambiguity to the affected discussion branch,
and transfers authority only after exact document approval and a successful
transfer check.

### Planned discussion states and transitions

The states are a natural-language coordination contract. They need not be
serialized as an enum or enforced by a separate engine.

| State | Meaning | Allowed next states |
| --- | --- | --- |
| Investigating | Current repository facts, authority, and feature boundary are being established | Model Aligning; operational stop |
| Model Aligning | A provisional model exists and material misunderstandings or assumptions are being tested | Model Aligned; Discovery Pending; operational stop |
| Model Aligned | The complete model has been re-synthesized and the user has had a chance to correct it | Decision Exploration; Design Ready; Model Aligning |
| Decision Exploration | One reachable material design decision is being settled | Decision Exploration; Model Aligning; Discovery Pending; Design Ready |
| Discovery Pending | A prototype, benchmark, visual artifact, or other evidence is required | Model Aligning after the named evidence exists; operational stop |
| Design Ready | The model is aligned and every applicable readiness dimension and material design branch is settled | Design Doc or no-Design-Doc authority path |

Model alignment holds only when no known material error or omission remains,
current and desired responsibilities and paths are mutually understood, and
every remaining unknown is classified as a design decision or explicit discovery
need. If no material design decision remains, `Model Aligned` may proceed
directly to the complete Design Readiness check.

A design answer can change the model. For example, moving retry ownership may
change the failure path and verification surface. The workflow therefore returns
to `Model Aligning` before offering the next decision whenever a newly settled
choice invalidates a current responsibility, path, constraint, or dependency.

### Understanding loop

The Agent first presents a coarse complete model rather than discovering the
system as an unframed sequence of questions. For a broad system, this model is a
map of the relevant components and paths, not a fully expanded specification.

For each iteration, the Agent:

1. resolves facts available from the repository or current authority;
2. selects the unresolved assumption whose correction could most change scope,
   responsibility placement, behavior, failure paths, or later decisions;
3. asks one understanding question without options, recommendation, or approval;
4. updates the affected model slice from the answer;
5. identifies newly exposed contradictions, dependencies, or evidence needs;
6. continues until no material model mismatch remains; and
7. re-synthesizes the complete model for the shared-model checkpoint.

Ordinary answers require only the changed slice to be shown. Full restatement on
every turn would increase interaction cost and obscure the actual correction.
The complete model is shown at a branch boundary, after a cross-cutting change,
on recovery when currentness cannot otherwise be established, and immediately
before design decisions begin.

### Design-decision loop

After the checkpoint, `design-discussion` applies the approved one-question
design-tree exploration. Understanding answers do not silently become settled
choices. Conversely, a material choice is not disguised as a factual
clarification to avoid explaining alternatives and trade-offs.

Only one material design question is active in the user interaction. Repository
research may examine independent evidence concurrently, but parallel questions
must not create competing decision contexts or ambiguous ordering. Each settled
choice identifies the branches it opens and the prior choices or model claims it
invalidates.

### Living decision record and recovery

The existing ignored workspace record remains the recoverable source before
Design Doc approval. It separates:

- a provisional working-model section containing evidence-backed current state,
  user intent, clearly labeled Agent inference, and unresolved understanding
  questions; and
- a decision section containing only selected approaches and rationale, rejected
  alternatives and reasons, scope and non-goals, explicit deferrals with intent
  and impact, and unresolved material design questions.

File presence is not alignment or approval. At the shared-model checkpoint, the
record identifies which model claims are confirmed, evidence-backed, inferred,
or still unresolved. Before Design Readiness, no material unresolved
understanding question may remain. For the no-Design-Doc path, the complete
record is consolidated before its existing holistic approval; unaccepted Agent
inference cannot become authority through consolidation.

After interruption or compaction, the coordinator rechecks repository identity,
authority currentness, and the record. It resumes at the earliest state whose
exit conditions remain satisfied. It does not repeat an unchanged complete
discussion, nor does it infer `Model Aligned` from a stale summary.

### Existing authority and partial re-entry

An exact, current, approved Design Doc or decision record can satisfy the model
and decision dimensions it already covers. The Agent presents only the smallest
complete context needed to understand the missing or changed branch, while
retaining the unchanged authority by reference.

A changed fact reopens affected model claims. A changed design choice reopens
that decision, dependent decisions, and any model paths whose meaning changes.
Independent approved choices remain current. Downstream discovery returns with
the exact evidence and re-enters model alignment before another design choice is
offered.

### Proportionality

Small planned changes may form and align the complete model in one exchange.
Large or cross-cutting systems begin with a coarse whole, then examine one
subsystem, boundary, or path at a time. The process is complete when material
understanding and decisions are resolved, not when a target question count is
reached.

Low-impact local implementation choices, repository-discoverable facts, and
speculative future requirements are not reasons to prolong either loop. When
the scope is too broad to model reliably, scope decomposition becomes the next
user-owned design decision after the current coarse model is understood.

### Compatibility and approval lifecycle

The lightweight path remains unchanged. A complete explicit request continues
to serve as its alignment source; only a material choice or durable coordination
requirement promotes it to this planned flow.

The shared-model checkpoint adds no holistic approval. After Design Readiness,
the approved lifecycle remains:

- Design Doc warranted: draft from settled decisions, approve the exact document,
  verify authority transfer, then retire the living record.
- No Design Doc: approve the consolidated decision record as design authority and
  retain it through the active workspace lifecycle.
- Derive and separately approve the Feature Contract, then derive and separately
  approve the Implementation Plan.

## Cross-cutting concerns

### Failure and non-progress behavior

The discussion does not advance when:

- current behavior or authority cannot be established well enough to form the
  applicable model;
- the user identifies a material mismatch that has not been incorporated;
- an understanding question is being presented as a disguised design choice;
- a design option is presented before model alignment;
- a required discovery artifact lacks its named evidence or re-entry condition;
- interruption leaves model or decision currentness unverifiable; or
- repeated questions do not reduce the same material uncertainty.

An operational inability to read required state or preserve the record is
`BLOCKED`. A missing user-owned decision remains a design gate. A pending
prototype or benchmark is `Discovery Pending`, not an inferred answer. Repeated
non-progress stops with the exact unresolved claim and evidence already tried;
it does not continue by rephrasing the same approval request.

### Compaction and context cost

The initial model is complete enough to expose cross-cutting relationships but
coarse enough to challenge. Subsequent turns show deltas by default, reducing
repeated context. Periodic whole-model synthesis prevents those deltas from
becoming disconnected local agreements.

The living record carries recovery state without making conversation history an
authority source. Exact repository and approved-authority identities remain the
currentness anchors.

### Verification obligations

Validation must observe behavior, not merely required headings or phrases. At a
minimum, forward scenarios establish that:

1. an ambiguous planned change receives a provisional working model and one
   understanding question without options or a recommendation;
2. the Agent does not ask the first design-decision question until the complete
   model has been re-synthesized and corrected;
3. after alignment, one material decision presents viable options, concrete
   trade-offs, and a recommendation;
4. an answer or new fact that invalidates responsibility or path assumptions
   returns to model alignment before the next decision;
5. a discovery-dependent branch stops with exact evidence and re-entry
   conditions, then resumes through model alignment;
6. exact current approved authority avoids a redundant full interview while a
   changed branch reopens only affected model and decision dependencies;
7. a small uniquely determined change remains proportional and does not receive
   a manufactured questionnaire;
8. after simulated compaction, the record plus repository and authority identity
   recover the correct state without treating file presence as approval; and
9. changed skills remain structurally valid and agree on responsibility,
   transitions, approval boundaries, and handoff fields.

These scenarios may be evaluated with isolated prompts and temporary workspaces.
They do not require production-code TDD because the changed artifacts are
natural-language workflow contracts. Any automated checks must use test-owned
fixtures rather than production skill files as installer test data.

### Migration and rollout

Existing approved authority remains valid for unchanged behavior. This Design
Doc, after approval and successful transfer, supersedes only the interaction and
state semantics it explicitly refines in the 2026-08-27 Design Alignment Gate.

The current uncommitted skill edits are treated as implementation candidates.
They may be retained while the Feature Contract and Implementation Plan are
derived, but they do not establish compliance. Implementation aligns
`design-discussion` and the planned-path coordinator handoff with this document,
then performs structural validation and the behavioral forward scenarios before
installation or publication.

Rollback restores the prior skill and coordinator semantics together. A partial
rollout in which one skill expects the shared-model state and the other does not
is invalid because the handoff contract would be inconsistent.

## Alternatives considered

### Present one complete detailed model and request approval

Rejected because a large polished model encourages passive confirmation and
recreates the premature-approval failure. The selected design presents a coarse
whole, then actively tests its highest-impact uncertainties.

### Ask about every model dimension before presenting a whole

Rejected because the user cannot evaluate why each question matters or detect
cross-component contradictions without an initial system map.

### Continue directly from investigation to design decisions

Rejected because repository facts do not prove that Agent inference matches the
user's operational and product understanding. This is the current gap in the
approved Design Alignment Gate.

### Combine understanding and design choices in one loop

Rejected because factual correction, domain clarification, recommendation, and
authority creation would remain ambiguous. The two loops deliberately use
different question contracts.

### Require approval of the shared working model

Rejected because the checkpoint confirms mutual understanding but does not
select or approve an architecture. Holistic approval remains at the existing
Design Doc or no-Design-Doc authority boundary.

### Add a separate `grill-me` skill

Rejected because repository investigation, understanding, alternatives, and
user-owned design decisions are one responsibility. Splitting them would create
routing and recovery ambiguity.

### Persist the full conversation as the model

Rejected because conversation history is not durable authority, is expensive to
replay, and does not distinguish current evidence, inference, and settled
decisions. The living record stores only the recoverable model and decision
state.

### Define a machine-readable state machine or fixed questionnaire

Rejected because task applicability determines which paths and questions matter.
The conceptual states protect ordering and recovery without prescribing exact
phrasing, question count, or implementation machinery.
