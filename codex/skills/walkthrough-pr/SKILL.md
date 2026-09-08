---
name: walkthrough-pr
description: Help an engineer understand a PR through dialogue about purpose, responsibilities, contracts, invariants, failure behavior, and test evidence, with selective code inspection. Use for human review comprehension, not an autonomous AI review or branch-management workflow.
---

# Walk through a PR

Help the engineer judge whether a change solves the intended problem and delivers
value with acceptable behavior and maintainability. The outcome is an engineer
who can explain the change, its important guarantees, their evidence, and the
remaining uncertainty. Reading every line is not a completion requirement.

Keep the reviewed repository read-only. Preparing and navigating the viewing
surface selected by the engineer is part of this walkthrough. Do not edit source,
switch branches, submit reviews, approve a PR, or start implementation or
automated review gates. An explicit request for a separate action belongs to its
applicable workflow.

## Establish the target and change map

Resolve the requested PR, diff, or local change from available repository and
read-only PR information. Identify the repository and exact compared revisions,
or the staged/unstaged/untracked scope when reviewing local changes. Ask only
when the target remains ambiguous; do not assume the current checkout is the PR.
Use read-only access to the target without changing the engineer's checkout.

Inspect the complete diff and relevant surrounding code before presenting a
change map. Group all changed artifacts by purpose and responsibility, including
deletions, tests, configuration, and generated changes when present. Account for
each group, but do not dump a file inventory or require the engineer to read
every line. Surface changes whose purpose or relationship is unexplained.

Consult the requirements, PR description, and design sources that are available.
Their absence does not block a walkthrough or require creating plan artifacts.
Label inferred intent and unresolved requirements. The implementation shows what
the code does; it cannot by itself establish what the product should do.

Keep the map and evidence tied to the inspected revisions. If the target changes
during the conversation, inspect the delta and revisit affected explanations and
evidence before relying on them again.

## Lead a conversation, one topic at a time

After resolving the target, confirm the viewing preference before the first
walkthrough topic: conversation excerpts, Hunk, nvim in a Herdr pane, or editor
links. Ask once when the preference is not already established; retain an
earlier choice and let the engineer change it later. Inspect the diff while
waiting, but do not create a viewer pane before the engineer selects that mode.

For Hunk or nvim, follow [viewing-surface preparation](references/review-surfaces.md)
to resolve permissions, prepare or reuse the review pane, and verify the displayed
target before proceeding. The selection authorizes the normal local viewer setup
described there; do not ask again for each pane or navigation operation. Runtime
permission requirements still apply. For editor links, provide verified file and
line references; presenting a link does not itself open or control nvim. For
conversation excerpts, no viewer setup is needed.

Start with a compact purpose and boundary overview. Present only enough detail
for the current topic, attach concrete evidence to important claims, and leave
room for the engineer's questions. Answer those questions without automatically
advancing. Move on when the engineer signals readiness or requests another
topic; allow skipping already-understood material and returning to earlier parts.
Do not deliver all five stages as a single unsolicited report.

Use the following sequence as a default, adapting its depth to the change and
the engineer's familiarity. Inspect and show code whenever a question needs it;
code inspection is available throughout, not reserved for the final stage.

1. **Purpose and boundary.** Explain the problem, affected users or callers,
   observable before/after behavior, preserved behavior, and scope. Connect the
   change groups to that purpose and expose unrelated or unexplained changes.
2. **Structure and responsibilities.** Trace a representative path through the
   system. Identify who owns decisions, state, side effects, and interfaces.
   Explain dependency direction and why responsibility sits at each boundary
   when evidence exists; label inferred rationale. Use a small diagram or table
   when it makes the relationships easier to understand.
3. **Contracts, invariants, and failure.** State applicable preconditions,
   postconditions, and guarantees, who enforces them, and what they depend on.
   Follow realistic failures at affected boundaries: what has already happened,
   what remains, what the caller observes, and how recovery or retry works.
   Consider concurrency, authorization, compatibility, and persistence when the
   change touches them, without manufacturing a universal checklist.
4. **Test evidence.** Connect important claims to test inputs, assertions, and
   available execution results. Explain what the tests establish and what they
   leave open. Check whether breaking the claimed guarantee would make the test
   fail and whether mocks bypass the behavior at issue. Distinguish test source,
   recorded results for this target, and behavior actually observed in this
   session; do not describe unread or unrun tests as verified. Tests support
   claims under particular conditions, not a blanket proof of correctness.
5. **Focused code inspection.** Resolve outstanding questions using the actual
   implementation and tests. Include important guarantee-enforcing code even
   when the engineer has not expressed a concern. For each location, state what
   to check and why; expand to callers, callees, or unchanged context when the
   guarantee crosses the diff boundary.

Invite the engineer to predict an outcome or restate a responsibility when that
would expose an uncertain assumption. Keep this collaborative rather than an
exam or a mandatory recitation after every step. A fluent AI explanation or
user acknowledgment alone does not establish that a guarantee holds.

## Ground understanding in implementation

Link important explanations to the actual file and symbol, with line references
or a short exact excerpt when useful. Clearly mark pseudocode; never present a
reconstructed snippet as the reviewed code. Separate required behavior, behavior
read from implementation, observed execution, and inference or unknowns.

Select inspection depth from the consequence of an incorrect assumption and the
strength of the evidence. Routine repeated changes can be understood as a group;
decision points, state transitions, commit boundaries, and material failure
handling often need closer inspection. Do not assume AI-authored code is safer
or that only locations the engineer already suspects need attention.

If an explanation conflicts with the code, a guarantee has no clear owner, or
test evidence misses a material scenario, follow the relevant path and correct
the working understanding. Expose the concrete gap and its consequence. Do not
silently invent a new requirement, broaden into an unrelated audit, or fix code
as part of explaining it. Missing evidence remains an explicit uncertainty.

For example, after a claim that retries cannot create duplicate records, locate
the mechanism that enforces uniqueness. Examine whether concurrent retries and
partial completion preserve the claim, and which of those cases the tests cover.
An existence check or a sequential test alone may not support that guarantee.

## Show code in the engineer's preferred surface

Use conversation excerpts, editor links, or an available Hunk session according
to the engineer's preference. Keep the same question, source location, and review
target across surfaces. Hunk is optional; its absence does not block the dialogue.

Use the prepared Hunk session or nvim pane to show each relevant location with
the question it helps answer. Follow the same [surface procedures](references/review-surfaces.md)
when navigating, reconnecting, or changing viewers. Do not mistake a sandbox
access failure for an absent Hunk session. If the chosen surface is unavailable,
explain the specific limitation and agree on another surface; do not silently
switch the engineer's preference or claim the requested viewer was opened.

## Close with understanding and remaining uncertainty

When the engineer finishes, summarize the purpose and responsibility model,
important guarantees and their evidence, locations actually inspected together,
and unresolved questions or unexamined areas. Keep agent inspection distinct
from what the engineer has examined; do not claim that they read every line.

Completion means the engineer can explain the overall change and judge its
important guarantees using concrete evidence while knowing the limits of that
evidence. Do not infer comprehension merely from finishing the explanation.
The engineer decides whether that understanding is sufficient to accept the PR;
completing the walkthrough is not approval, verification PASS, or Task Acceptance.
