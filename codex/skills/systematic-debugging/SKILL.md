---
name: systematic-debugging
description: Investigate bugs, unexpected behavior, or unexplained verification failures and return causal evidence to the owning workflow.
---

# Systematic debugging

Own reproduction, causal investigation, and diagnostic handoff. Work within
the request's authority and the owning workflow's writer boundaries.

## Establish the evidence

- Capture the exact command, input, environment, expected result, and actual result.
- Establish whether the failure is current and repeatable. Reduce the
  reproduction when that helps distinguish causes while preserving the symptom.
- Trace relevant data and control flow across component boundaries using logs,
  return values, persisted state, and side effects.
- Compare a working path, earlier revision, or minimal example when it helps
  distinguish competing explanations. Test one causal hypothesis at a time.

If reproduction is unavailable or intermittent, collect observations and
state the uncertainty. Do not invent a confirmed cause from a plausible explanation.

For a diagnosis-only request, keep tracked source read-only. Use temporary
diagnostics only when the request or owning loop already authorizes them and
they answer a specific question. Retain the observations and remove only your
own temporary diagnostic artifacts when finished; preserve unrelated state.

Continue while an authorized observation can materially reduce uncertainty.
When progress repeatedly stalls, or the next observation needs unavailable
inputs or new authority, return the evidence, attempted hypotheses, exact gap,
and re-entry condition to the owner. Do not use a fixed attempt count as a
stopping rule or repeat an unchanged failed experiment without new evidence.

## Return diagnostic evidence

Report the reproduction and observed results, then distinguish established facts,
supported hypotheses, and remaining uncertainty. A confirmed root cause identifies:

- the triggering condition;
- the incorrect assumption or implementation;
- the path from trigger to observed symptom;
- evidence that rules out leading alternatives.

For a diagnosis-only request, return the findings to the user. When a correction
is already authorized, return the diagnosis to the existing `execute-task` or
`execute-lightweight-task` loop and continue within that authority without a new
approval merely for completing diagnosis. That loop owns the permanent change,
applicable development discipline, verification, review, and acceptance.

This skill does not implement permanent corrections or choose their acceptance
checks. An unresolved user-owned design or scope decision returns through the
coordinator's existing boundary.
