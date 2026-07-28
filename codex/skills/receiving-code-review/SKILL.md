---
name: receiving-code-review
description: Verify review findings against current code and classify each as Fix, Push back, or Escalate. Use from the workflow coordinator for authorized review loops or standalone for read-only feedback evaluation.
---

# Triage current-head review findings

Treat review as technical evidence, not an instruction to agree.
Remain read-only. Do not edit files, create commits, or start a correction.

## Coordinator-managed entry

When the workflow coordinator invokes this skill, require:

- the current head and exact reviewed range;
- every surviving finding and its review evidence;
- the approved decision source, scope, non-goals, review policy, and
  implementation authorization;
- the approved Design Doc and plan when present;
- bounded retry history for a repeated finding.

## Standalone read-only entry

When the user invokes this skill outside the coordinator, resolve through local
read-only investigation:

- each complete finding;
- the current code, head, and relevant range;
- applicable repository guidance;
- available design and plan evidence.

Implementation authorization is not required to evaluate a standalone finding.
The evaluation does not authorize a change.

## Process each item

1. Read the complete item and locate its cited code.
2. Restate the concrete requirement.
3. Reproduce or verify the claim against the current head.
4. Check repository guidance, current code, the requested or approved scope, and
   every available approved decision source, Design Doc, and plan.
5. Classify the item as exactly one of:
   - **Fix** — verified on the current head, within requested or approved scope,
     and compatible with available approved decisions. Convert it into concrete
     execution steps naming the behavior, files, discipline, and exact
     verification.
   - **Push back** — incorrect, unsupported, preference-only, stale, not
     reproducible on the current head, or already decided without new evidence.
     Cite the controlling decision or code and test evidence.
   - **Escalate** — resolution requires a design or public-contract decision,
     material scope expansion, new authority, or, in a coordinator-managed entry,
     the bounded retry stop has been reached.
6. Record current-head evidence and one concrete next action.

## Report

Use concise technical language. Cite code, tests, or decisions. Avoid performative agreement, defensive phrasing, and speculative concessions.

For each finding report:

- current head and reviewed range;
- classification: `Fix`, `Push back`, or `Escalate`;
- requirement and concrete evidence;
- impact and exact next action;
- any unverified gap.

For a coordinator-managed entry, do not ask for additional approval for a `Fix`
when implementation authorization is already recorded and the correction remains
within approved scope. Return it to the coordinator for the bounded correction
loop. Return `Push back` with decision or code evidence so triage can continue.
Return `Escalate` with the exact user-owned decision or new authority required.

For a standalone read-only entry, return a `Fix` as evaluation only with proposed
execution steps. Do not edit files, start a fix, or advance another workflow
phase.

After an authorized coordinator-managed `Fix`, the coordinator must require fresh
verification and the complete applicable final review against the new current
head. Earlier verification, review, and classification evidence is stale after
the fix commit.
