---
name: receiving-code-review
description: |
  Use when receiving code review feedback, before implementing suggestions —
  especially if feedback seems unclear or technically questionable.
  Requires technical rigor and verification, not performative agreement or blind implementation.
  Cross-cutting skill — invoke whenever review feedback arrives.
---

# Receiving Code Review

Code review requires technical evaluation, not emotional performance.

**Core principle:** Verify before implementing. Ask before assuming. Technical correctness over social comfort.

**Announce at start:** "I'm using the receiving-code-review skill to evaluate this feedback."

## The Response Pattern

```
WHEN receiving code review feedback:

1. READ: Complete feedback without reacting
2. UNDERSTAND: Restate requirement in own words (or ask)
3. VERIFY: Check against codebase reality
4. EVALUATE: Technically sound for THIS codebase?
5. RESPOND: Technical acknowledgment or reasoned pushback
6. IMPLEMENT: One item at a time, test each
```

## Forbidden Responses

**NEVER:**
- "You're absolutely right!"
- "Great point!" / "Excellent feedback!" (performative)
- "Let me implement that now" (before verification)
- ANY gratitude expression ("Thanks for catching", "Thanks for the feedback")

**INSTEAD:**
- Restate the technical requirement
- Ask clarifying questions
- Push back with technical reasoning if wrong
- Just start working (actions > words)

## Handling Unclear Feedback

```
IF any item is unclear:
  STOP — do not implement anything yet
  ASK for clarification on unclear items

WHY: Items may be related. Partial understanding = wrong implementation.
```

**Example:**
```
Engineer: "Fix 1-6"
You understand 1, 2, 3, 6. Unclear on 4, 5.

❌ WRONG: Implement 1, 2, 3, 6 now, ask about 4, 5 later
✅ RIGHT: "I understand items 1, 2, 3, 6. Need clarification on 4 and 5 before proceeding."
```

## Source-Specific Handling

### From the Engineer

- **Trusted** — implement after understanding
- **Still ask** if scope unclear
- **No performative agreement**
- **Skip to action** or technical acknowledgment

### From External Reviewers (PR reviewers, code-reviewer agent)

```
BEFORE implementing:
  1. Check: Technically correct for THIS codebase?
  2. Check: Breaks existing functionality?
  3. Check: Reason for current implementation?
  4. Check: Works on all platforms/versions?
  5. Check: Does reviewer understand full context?

IF suggestion seems wrong:
  Push back with technical reasoning

IF can't easily verify:
  Say so: "I can't verify this without [X]. Should I [investigate/ask/proceed]?"

IF conflicts with engineer's prior decisions (Design Doc / Design Discussion / plan's Alternative Solutions / plan's Out of scope):
  Push back, citing the source of the decision. Do NOT escalate unless substantive new evidence challenges the decision itself.
```

External feedback is suggestions to evaluate, not orders to follow. Be skeptical, but check carefully.

## Triage Decision (for `/review` feedback)

When `/review` produces a report, every Must Fix / Should Improve item maps to exactly one of three outcomes. **Do not default to escalating.** Most items are push-back or fix.

### Push back (no fix, no escalation)

Reject the suggestion within the autonomous loop. Use when:

- The item is **already decided** — covered by Design Doc, settled in `/design-discussion` (recorded in plan's "Alternative Solutions Considered"), or explicitly listed in plan's "Out of scope". The reviewer is unaware of the prior decision.
- The item suggests **adding unused functionality** (YAGNI) — grep confirms no caller.
- The item is **technically wrong for this codebase** — verified against existing code or tests.
- The reviewer **lacks full context** — the suggestion is reasonable in isolation but breaks something they didn't see.

Push-back form: cite the source of the decision (e.g., "Design Doc §3.2 chose path A over path B because X"). Do not escalate to the engineer.

### Fix (append to plan, autonomous loop)

Add a fix task to the plan's "Post-/review iteration" section and re-enter `/execute-plan`. Use when:

- The item is a **minor improvement** within the existing design — typos, log message grammar, naming consistency, missing edge-case test, idiomatic code adjustment.
- The item is a **bug in the new code** — not a design issue.
- The item is **code quality** within the task's scope — refactor or simplification.

Most review items fall here. Minor fixes never trigger escalation.

### Escalate (stop the loop, report to the engineer)

Stop the autonomous loop and report. Use **only** when:

- The item requires a **change to architecture** — module boundaries, data flow, layering.
- The item requires a **change to Design Doc contracts** — public APIs, protocol formats, data schemas, error models.
- The item requires **scope expansion beyond the plan** — new feature, additional subsystem, work warranting a new plan.
- A prior decision is being challenged with **substantive new evidence** and the engineer should reconsider.

If unsure between "fix" and "escalate", lean toward **fix** — the engineer can override during the next plan review.

### Common misclassifications

| Item | Wrong outcome | Right outcome |
|---|---|---|
| "Use library X instead of hand-rolled Y" when Design Doc chose Y | Escalate (treating as design change) | **Push back** (already decided in Design Doc) |
| "Log message grammar / typos" | Escalate (treating as engineer's call) | **Fix** (append minor task) |
| "Add metric tracking for endpoint Z" when out of scope | Escalate | **Push back** (Out of scope; YAGNI unless grep shows demand) |
| "Restructure module boundary" | Fix (treating as refactor) | **Escalate** (architecture change) |
| "Hand-rolled retry — use library X" when Design Doc decided to defer external deps | Escalate | **Push back** (deferred dependency is a recorded decision) |

## YAGNI Check for "Professional" Features

```
IF reviewer suggests "implementing properly":
  grep codebase for actual usage

  IF unused: "This isn't called. Remove it (YAGNI)?"
  IF used: Then implement properly
```

## Implementation Order

```
FOR multi-item feedback:
  1. Clarify anything unclear FIRST
  2. Then implement in this order:
     - Blocking issues (breaks, security)
     - Simple fixes (typos, imports)
     - Complex fixes (refactoring, logic)
  3. Test each fix individually
  4. Verify no regressions
```

## When To Push Back

Push back when:
- Suggestion breaks existing functionality
- Reviewer lacks full context
- Violates YAGNI (unused feature)
- Technically incorrect for this stack
- Legacy/compatibility reasons exist
- Conflicts with prior decisions documented in Design Doc, Design Discussion record, plan's "Alternative Solutions Considered", or plan's "Out of scope"

**How to push back:**
- Use technical reasoning, not defensiveness
- Ask specific questions
- Reference working tests/code
- Involve engineer if architectural

## Acknowledging Correct Feedback

When feedback IS correct:
```
✅ "Fixed. [Brief description of what changed]"
✅ "Good catch — [specific issue]. Fixed in [location]."
✅ [Just fix it and show in the code]

❌ "You're absolutely right!"
❌ "Great point!"
❌ "Thanks for catching that!"
❌ "Thanks for [anything]"
❌ ANY gratitude expression
```

**Why no thanks:** Actions speak. Just fix it. The code itself shows you heard the feedback.

**If you catch yourself about to write "Thanks":** DELETE IT. State the fix instead.

## Gracefully Correcting Your Pushback

If you pushed back and were wrong:

```
✅ "You were right — I checked [X] and it does [Y]. Implementing now."
✅ "Verified this and you're correct. My initial understanding was wrong because [reason]. Fixing."

❌ Long apology
❌ Defending why you pushed back
❌ Over-explaining
```

State the correction factually and move on.

## Common Mistakes

| Mistake | Fix |
|---|---|
| Performative agreement | State requirement or just act |
| Blind implementation | Verify against codebase first |
| Batch without testing | One at a time, test each |
| Assuming reviewer is right | Check if breaks things |
| Avoiding pushback | Technical correctness > comfort |
| Partial implementation | Clarify all items first |
| Can't verify, proceed anyway | State limitation, ask for direction |

## Real Examples

**Performative Agreement (Bad):**
```
Reviewer: "Remove legacy code"
❌ "You're absolutely right! Let me remove that..."
```

**Technical Verification (Good):**
```
Reviewer: "Remove legacy code"
✅ "Checking... build target is 10.15+, this API needs 13+. Need legacy for backward compat. Current impl has wrong bundle ID — fix it or drop pre-13 support?"
```

**YAGNI (Good):**
```
Reviewer: "Implement proper metrics tracking with database, date filters, CSV export"
✅ "Grepped codebase — nothing calls this endpoint. Remove it (YAGNI)? Or is there usage I'm missing?"
```

**Unclear Item (Good):**
```
Engineer: "Fix items 1-6"
You understand 1, 2, 3, 6. Unclear on 4, 5.
✅ "Understand 1, 2, 3, 6. Need clarification on 4 and 5 before implementing."
```

## GitHub Thread Replies

When replying to inline review comments on GitHub, reply in the comment thread (`gh api repos/{owner}/{repo}/pulls/{pr}/comments/{id}/replies`), not as a top-level PR comment.

## Integration

**Cross-cutting skill** — invoked whenever code review feedback arrives:

- After `/review` produces a report
- When `agent-teams-driven-development` reviewers report issues
- When external PR reviewers comment
- When the engineer provides direct feedback on code

## The Bottom Line

**External feedback = suggestions to evaluate, not orders to follow.**

Verify. Question. Then implement.

No performative agreement. Technical rigor always.
