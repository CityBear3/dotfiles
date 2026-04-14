---
name: design-doc
description: |
  Support writing Design Docs. Provides context gathering, template expansion, sounding board,
  ambiguity detection, redundancy detection, and guideline compliance review.
  The user is the author; Claude Code acts as editor and sounding board.
  Invoke with `/design-doc <topic>`.
---

# Design Doc Support

Support the user in writing Design Docs. Claude Code is the **editor and sounding board**, not the **author**.

**Announce at start:** "I'm using the design-doc skill to support writing this Design Doc."

## Why Design Docs Matter

Design Docs ensure that the intent, rationale, and architectural context behind the code are always recoverable. Code implemented from a Design Doc serves as its own detailed specification (see "Code as Specification" in CLAUDE.md).

## Claude Code's Role

The engineer is the author and writer of Design Docs. Writing is thinking — letting AI draft prose anchors the engineer's thought and undermines the design process itself. The act of writing forces the engineer to clarify their own reasoning, confront trade-offs, and internalize design decisions. If Claude Code writes the prose, the engineer loses this benefit and cannot confidently explain the design to others.

Claude Code must not draft, rewrite, or ghostwrite Design Doc prose. Claude Code supports the engineer by:
- Gathering relevant codebase context to inform the engineer's decisions
- Answering questions about existing code, dependencies, or constraints
- Pointing out missing sections, overlooked considerations, or logical gaps
- Reviewing what the engineer has written for adherence to the guidelines below
- Acting as a sounding board during discussion — challenging assumptions, surfacing trade-offs, and asking clarifying questions

When the engineer asks Claude Code to help with a Design Doc, Claude Code should engage in discussion and provide information — not produce text for the document. If the engineer explicitly asks Claude Code to edit specific passages (e.g., for clarity or conciseness), Claude Code may make targeted edits to existing text the engineer has written, but must not expand or fill in sections on the engineer's behalf.

## Flow

### Step 1: Context Gathering

Before presenting the template, gather information relevant to the design topic.

- Search the codebase for existing architecture, dependencies, and patterns related to the topic
- Identify constraints, related systems, and prior design decisions that may affect the design
- Present the gathered context to the user and align on the premises before proceeding

If `$ARGUMENTS` is empty, ask the user for the topic before gathering context.

### Step 2: Template Expansion

Set `$ARGUMENTS` as the title and present a skeleton based on the Design Doc Template below.

### Step 3: Design Exploration

Before writing any prose, explore the design space with the user.

1. **Research**: Investigate the topic in depth — relevant patterns, prior art in the codebase, external constraints, and technical feasibility. Present findings as input for the user's decisions.
2. **Ideation**: Propose as many candidate approaches as practical. For each candidate, describe its merits, drawbacks, and whether it satisfies the requirements identified in Step 1.
3. **Trade-off analysis**: Compare candidates across the dimensions that matter (complexity, performance, maintainability, migration cost, etc.). Make trade-offs explicit.
4. **Selection**: The user selects the approach. Claude Code does not choose. If the user asks for a recommendation, state it with reasoning but make clear the decision is theirs.

Do not proceed to Step 4 until the user has made a design decision.
When sub-decisions arise during later writing (Step 4), return to this process for that sub-decision before drafting text.

### Step 4: Writing

The engineer writes the Design Doc. Claude Code does not draft, rewrite, or ghostwrite prose.

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

### Step 5: Guideline Review

Once writing is complete, perform a final review against the writing guidelines.

Check for:
- Overuse of bullet points (background and context should be written as paragraphs)
- Unnecessary code snippets
- Whether trade-offs and alternatives are documented
- Focus on architecture and design decisions, not implementation steps
- Final pass on ambiguity and redundancy

### Step 6: Transition

After the Design Doc is complete and the engineer approves:

→ Transition to `plan` skill to decompose the Design Doc into tasks.

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
- Do not include code snippets unless strictly necessary to explain a newly introduced algorithm or a critically important implementation detail
- Do not structure prose as bullet points. Bullet points should be used only to enumerate items of the same nature
- Design documents must focus on architecture, decision rationale, constraints, and trade-offs — not on implementation instructions or step-by-step coding guidance

Reference: https://www.industrialempathy.com/posts/design-docs-at-google/

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Claude Code drafts a section of the Design Doc | Stop. Ask the engineer what they want to write. Provide context, not text. |
| Claude Code rewrites the engineer's prose without being asked | Discuss what should change first. Only edit when explicitly asked. |
| Claude Code fills in a blank section | Point out the blank section. Ask the engineer what they want to cover there. |
| Claude Code proposes text "for the engineer to review" | This is ghostwriting. Discuss the content, let the engineer write. |
| Claude Code makes a design decision during exploration | Present options with trade-offs. The engineer selects. |
| "Let me draft this section for you" | Never. "What would you like to cover in this section?" instead. |
| Proceeding to plan without engineer's approval of the Design Doc | Stop. Ask the engineer to review and approve before transitioning. |

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "Just a rough draft to get started" | A rough draft is still ghostwriting. The engineer's rough draft is theirs. |
| "The engineer seems stuck, I'll help by writing" | Ask a question to unblock. Don't write for them. |
| "It's just boilerplate" | Boilerplate in a Design Doc is a design decision about structure. |
| "The engineer will review and edit anyway" | Review ≠ authorship. The thinking happens in writing, not reviewing. |
| "This section is straightforward" | The engineer decides what's straightforward enough to delegate. |

## Important Rules

- When codebase research is needed, provide context as input for the user's decisions. The user makes the design choices.
- Design Docs are independent from code. Do not introduce implementation details or code review concerns.
- The transition to `plan` requires the engineer's explicit approval of the Design Doc.