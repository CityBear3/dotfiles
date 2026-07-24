---
name: receiving-code-review
description: Evaluate code-review feedback against the approved design and codebase before accepting, rejecting, or escalating it. Use whenever review feedback arrives, especially when it is ambiguous or technically questionable.
---

# Receive code review

Treat review as technical evidence, not an instruction to agree.

## Process each item

1. Read the complete item and locate its cited code.
2. Restate the concrete requirement.
3. Reproduce or verify the claim against current behavior.
4. Check repository guidance, the approved design, plan, and scope.
5. Classify it:
   - **Fix** — valid, in scope, and does not change the approved design.
   - **Push back** — incorrect, already decided, unsupported, or unnecessary.
   - **Escalate** — requires a new architecture, public contract, or material scope expansion.
6. Record the evidence and next action.

Ask the reviewer or user only when missing information materially changes the classification.

## Applying fixes

- Convert valid items into concrete plan steps.
- Use TDD for behavior changes.
- Address one independent item at a time and run focused verification.
- Re-run the relevant review after fixes; a fix is not proven merely because it was edited.

## Responding

Use concise technical language. Cite code, tests, or decisions. Avoid performative agreement, defensive phrasing, and speculative concessions.

When pushing back, explain the mismatch and provide evidence. When escalating, name the decision the user must make and why existing authority is insufficient.
