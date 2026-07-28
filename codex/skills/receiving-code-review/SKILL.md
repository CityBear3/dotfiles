---
name: receiving-code-review
description: Verify final-review findings against the current head, classify each as Fix, Push back, or Escalate, and return evidence and next actions to the workflow coordinator. Use whenever review feedback arrives.
---

# Triage current-head review findings

Treat review as technical evidence, not an instruction to agree.
Remain read-only. Do not edit files, create commits, or start a correction.

## Inputs

Require the coordinator to supply:

- the current head and exact reviewed range;
- every surviving finding and its review evidence;
- the approved decision source, scope, non-goals, review policy, and
  implementation authorization;
- the approved Design Doc and plan when present;
- bounded retry history for a repeated finding.

## Process each item

1. Read the complete item and locate its cited code.
2. Restate the concrete requirement.
3. Reproduce or verify the claim against the current head.
4. Check repository guidance, the approved decision source and scope, and the
   Design Doc and plan when present.
5. Classify the item as exactly one of:
   - **Fix** — verified on the current head, within approved scope, and
     compatible with approved decisions. Convert it into concrete execution steps
     naming the behavior, files, discipline, and exact verification.
   - **Push back** — incorrect, unsupported, preference-only, stale, not
     reproducible on the current head, or already decided without new evidence.
     Cite the controlling decision or code and test evidence.
   - **Escalate** — resolution requires a design or public-contract decision,
     material scope expansion, new authority, or the coordinator's bounded retry
     stop has been reached.
6. Record current-head evidence and one concrete next action.

Do not ask for additional approval for a `Fix` when implementation authorization
is already recorded and the correction remains within approved scope. Return it
to the coordinator for the bounded correction loop. Return `Push back` with
decision or code evidence so triage can continue. Return `Escalate` with the exact
user-owned decision or new authority required.

## Return to the coordinator

Use concise technical language. Cite code, tests, or decisions. Avoid performative agreement, defensive phrasing, and speculative concessions.

For each finding report:

- current head and reviewed range;
- classification: `Fix`, `Push back`, or `Escalate`;
- requirement and concrete evidence;
- impact and exact next action;
- any unverified gap.

After an authorized `Fix`, the coordinator must require fresh verification and
the complete applicable final review against the new current head. Earlier
verification, review, and classification evidence is stale after the fix commit.
