---
name: design-doc
description: |
  Support writing formal Design Docs. Provides template expansion, sounding board,
  ambiguity detection, redundancy detection, and guideline compliance review.
  The user is the author; Claude Code acts as editor and sounding board.
  Invoked from /design-discussion when the work warrants formal documentation,
  or directly with `/design-doc <topic>` when an existing discussion has settled the design.
---

# Design Doc Support

Support the user in writing Design Docs. Claude Code is the **editor and sounding board**, not the **author**.

**Announce at start:** "I'm using the design-doc skill to support writing this Design Doc."

## Why Design Docs Matter

Design Docs ensure that the intent, rationale, and architectural context behind the code are always recoverable. Code implemented from a Design Doc serves as its own detailed specification (see "Code as Specification" in CLAUDE.md).

## When to Use

This skill is invoked when the work warrants formal documentation: multiple components, cross-cutting concerns, significant architectural decisions, or future-reference value.

Typical entry path is `/design-discussion → /design-doc → /create-plan`. The discussion phase establishes the design direction; this skill formalizes it into a Design Doc.

Skip this skill for smaller work where a Design Doc would be ceremony — go directly from `/design-discussion` to `/create-plan`.

## Claude Code's Role

The engineer is the author and writer of Design Docs. The governing principle is
**ideation primacy**: the engineer's own thinking must reach the page before any
AI-generated prose touches it. Writing is thinking — the act of writing forces the
engineer to clarify reasoning, confront trade-offs, and internalize decisions. If AI
prose arrives first it *anchors* the engineer's thought: people converge on a fluent
draft they are shown, adopt framings they do not fully hold, and lose the ideation
initiative — all while still feeling in control. The cost is not a worse document; it
is an engineer who cannot confidently explain a design they never actually thought
through.

**Ideation primacy — the one rule:** AI prose must never precede the engineer's own
first draft of a section. Claude Code does not draft, fill in, or ghostwrite Design
Doc prose, and does not offer "a draft to react to" before the engineer has written.

"Prose" here means the reasoning content — the paragraphs where the design is
argued. Structural scaffolding is treated separately: Claude Code may lay out the
**fixed template skeleton** (the canonical headings in the Design Doc Template), since
that is convention, not design thinking. But it must not propose the **design-specific
subsection decomposition** — how this particular design is carved into components and
what they are named. How a design is decomposed is itself an architectural decision;
proposing it anchors the engineer's structure. The subsection breakdown is the
engineer's.

Within that rule, Claude Code collaborates in two modes:

- **Thinking collaboration (unbounded, encouraged).** Before and during writing:
  challenge assumptions, surface trade-offs, ask clarifying questions, gather codebase
  context, and point out missing sections or logical gaps. This shapes the engineer's
  thinking but never produces document prose.
- **Post-draft critique (permitted).** *After* the engineer has written a passage,
  Claude Code may critique it, name specific weaknesses, and — only when explicitly
  asked — make targeted edits to text the engineer already wrote. It must not expand
  or fill in sections on the engineer's behalf.

## Flow

### Step 1: Template Expansion

Set `$ARGUMENTS` as the title and present a skeleton based on the Design Doc Template below.

If `$ARGUMENTS` is empty, ask the engineer for the topic before expanding the template.

### Step 2: Writing

The engineer writes the Design Doc. Claude Code does not draft, rewrite, or ghostwrite prose.

**Handling sub-decisions during writing:**
If a sub-decision arises during writing that wasn't resolved in `/design-discussion`, pause writing and return to discussion mode for that sub-decision. Either invoke `/design-discussion` explicitly, or run a brief exploration in-place: research → propose 2–3 options → trade-offs → engineer selects. Do not draft text for an unresolved sub-decision.

Claude Code supports the writing process by:

**Sounding board:**
- Challenge design decisions by asking about alternatives and trade-offs
- Ask questions that deepen thinking: "What is the rationale behind this constraint?", "Why was this option ruled out?"

**Ambiguity detection:**
Flag ambiguous statements and prompt the user to make them concrete.
- Undefined terms: domain or technical terms used without definition on first occurrence
- Unquantified requirements: vague expressions like "fast", "large volume", "sufficient" — ask for specific numbers or criteria
- Unclear scope boundaries: where the line between Goals and Non-Goals is blurry
- Implicit assumptions: unstated assumptions (e.g., dependency availability, expected data volume)
- Undefined edge cases: missing descriptions of error paths or boundary conditions

**Redundancy detection:**
- When the same content appears across multiple sections, suggest consolidating into one place
- When the same concept is expressed in different wording, flag the risk that readers may interpret them as distinct concepts
- Suggest simplifying verbose or roundabout phrasing

**Targeted editing:**
If the engineer explicitly asks Claude Code to edit specific passages (e.g., for clarity or conciseness), Claude Code may make targeted edits to existing text the engineer has written. Claude Code must not expand or fill in sections on the engineer's behalf.

#### Writing Style: Narrative-based Prose

The Detailed Design section should be written in narrative prose, similar to how RFC specifications describe protocols. The primary medium is coherent paragraphs that explain the design as a flowing narrative. Diagrams and bullet points are supplementary — used only when they clarify structure that prose alone cannot convey efficiently.

**Annotated example:**

~~~
## Detailed design                          <- Section heading

### Event routing                           <- Subsection: one per design component

When a domain event is published, the       <- Narrative: explain behavior in prose.
router inspects the event's aggregate type    Write with subject-verb structure so
and resolves a list of subscriber handlers    readers can follow the flow of processing.
registered for that type. Each handler is
invoked asynchronously; the router does not
wait for completion before dispatching to
the next handler. This allows a single
event to fan out without introducing
latency coupling between subscribers.       <- Trade-off: add 1-2 sentences of rationale.
                                              Detailed comparison goes in Alternatives.

The routing table is built at startup by    <- Continue explaining mechanisms.
scanning modules annotated with
@EventHandler. The scan is limited to the
bounded context's own module path to
prevent cross-context coupling.

  +----------+    resolve     +-----------+
  |  Event   |-------------->|  Router   | <- Diagram: use only when prose alone cannot
  +----------+               +-----+-----+   convey structure efficiently (e.g. component
                                   |          relationships, data flow).
                          +--------+--------+
                          v        v        v
                       Handler  Handler  Handler

Supported event types:                      <- Bullet points: use only to enumerate items
                                              of the same kind. Do not use for background
- OrderPlaced                                 or context.
- OrderCancelled
- InventoryAdjusted
~~~

### Step 3: Guideline Review

Once writing is complete, perform a final review against the writing guidelines.

Check for:
- Overuse of bullet points (background and context should be written as paragraphs)
- Unnecessary code snippets
- Whether trade-offs and alternatives are documented
- Focus on architecture and design decisions, not implementation steps
- Final pass on ambiguity and redundancy

### Step 4: Transition

After the Design Doc is complete and the engineer approves:

→ Transition to `/create-plan` to decompose the Design Doc into tasks.

## Design Doc Template

```
# [Design Doc] <Title of the Design Doc>
<Meta Information: Author, Date, etc.>

## Context and Scope
### Goals
### Non-Goals

## Overview

## Detailed design

## Cross-cutting Concerns

## Alternatives
```

Reference for template details: https://www.industrialempathy.com/posts/design-doc-a-design-doc/

## Writing Guidelines

- Design Docs are not code specifications, but high-level descriptions of design and architecture
- Must describe trade-offs, alternatives considered, and rationale behind design decisions
- Should include diagrams or visual aids to enhance understanding, but the primary medium is text-based explanations
- Do not structure prose as bullet points. Bullet points should be used only to enumerate items of the same nature
- Design documents must focus on architecture, decision rationale, constraints, and trade-offs — not on implementation instructions or step-by-step coding guidance

### Technical Detail in Design Docs

Design Docs may include technical detail when it materially clarifies the
design or pins down a contract that downstream readers must agree on. The
test is whether the detail is **part of the design** (something the engineer
decided and others must conform to) or **part of the implementation** (how
the code happens to do it). Include the former; omit the latter.

**Include when relevant:**

- **Public interfaces and API signatures** — function/method signatures,
  endpoint shapes, CLI surfaces. These are contracts; the design fixes
  them, so they belong in the doc.
- **Protocol message formats** — request/response schemas, event payload
  structures, wire formats. The shape of what flows between components is
  a design decision.
- **Data structures and schemas** — table schemas, persisted formats,
  in-memory shapes that other components rely on. Anything readers need
  to reason about coupling.
- **Algorithm sketches** — when a non-obvious algorithm is core to the
  design, a short pseudocode or formula clarifies it better than prose.
- **State machines and lifecycles** — explicit states, transitions, and
  invariants for stateful components.
- **Error model** — what failure modes are exposed, expected handling,
  retry/idempotency semantics.

**Omit (belongs in code, not the doc):**

- Step-by-step implementation procedures or coding instructions
- Internal helper functions, private types, or implementation-only structures
- Boilerplate (error wrapping, logging, simple getters/setters)
- Full file contents or large code blocks that duplicate what the code itself will say

**Format guidance:**

Prefer the smallest form that conveys the contract. Type signatures or a
schema sketch are usually enough — full implementations are not. When
including a code-like artifact, treat it as a specification fragment, not
sample code: it should describe *what* must be true, not *how* to build it.

For gRPC-based APIs, the API portion of the Detailed Design section can
adopt a structured format with Resource model + per-method
"Design decisions / Guidance / Examples" subsections + a central Errors
block (with hybrid per-method "Possible errors" listing). The format
borrows the prominence of "Guidance" from Google's AIPs and adapts the
rest to Design Doc conventions. See `references/api-section-format.md`
for the format specification and a worked example.

References:
- General Design Doc style — https://www.industrialempathy.com/posts/design-docs-at-google/
- Detailed Design section — `references/detailed-design-guide.md`
- API section format (gRPC) — `references/api-section-format.md`

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Claude Code drafts a section of the Design Doc | Stop. Ask the engineer what they want to write. Provide context, not text. |
| Claude Code rewrites the engineer's prose without being asked | Discuss what should change first. Only edit when explicitly asked. |
| Claude Code fills in a blank section | Point out the blank section. Ask the engineer what they want to cover there. |
| Claude Code proposes text "for the engineer to review" | This is ghostwriting. Discuss the content, let the engineer write. |
| Claude Code offers a draft "for the engineer to react to" | Ideation-primacy violation. AI prose must not precede the engineer's draft. Ask what they want to write; critique only after they've written. |
| Claude Code makes a design decision during exploration | Present options with trade-offs. The engineer selects. |
| "Let me draft this section for you" | Never. "What would you like to cover in this section?" instead. |
| Proceeding to /create-plan without engineer's approval of the Design Doc | Stop. Ask the engineer to review and approve before transitioning. |

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "Just a rough draft to get started" | A rough draft is still ghostwriting. The engineer's rough draft is theirs. |
| "The engineer seems stuck, I'll help by writing" | Ask a question to unblock. Don't write for them. |
| "It's just boilerplate" | Boilerplate in a Design Doc is a design decision about structure. |
| "The engineer will review and edit anyway" | Review ≠ authorship. The thinking happens in writing, not reviewing. |
| "This section is straightforward" | The engineer decides what's straightforward enough to delegate. |

## Important Rules

- **Ideation primacy.** AI prose must never precede the engineer's first draft of a section. Thinking collaboration (dialogue, challenge, context-gathering) is unbounded; prose collaboration is limited to critiquing — and, only when asked, editing — text the engineer has already written.
- When codebase research is needed, provide context as input for the user's decisions. The user makes the design choices.
- Design Docs describe design, not implementation procedure. Technical contracts (interfaces, protocols, data structures, error models) belong in the doc when they are part of the design — see "Technical Detail in Design Docs". Internal implementation steps and code review concerns do not.
- The transition to `/create-plan` requires the engineer's explicit approval of the Design Doc.