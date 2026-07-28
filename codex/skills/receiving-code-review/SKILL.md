---
name: receiving-code-review
description: Verify review findings against current code and classify each as Fix, Push back, or Escalate. Use from the workflow coordinator for authorized review loops or standalone for read-only feedback evaluation.
---

# Triage current-head review findings

Treat review as technical evidence, not an instruction to agree.
Remain check-only and read-only. Do not mutate the index, tracked files, or
in-scope source; edit, stage, or commit a fix; dispatch a writer; or advance
another workflow phase. Return classification evidence to the coordinator or
requester only.

## Coordinator-managed entry

When the workflow coordinator invokes this skill, require:

- the exact current HEAD, immutable full-range review target identity, and
  `FINDINGS` verdict;
- every surviving finding in the complete final schema, including final severity
  exactly `Must Fix` or `Should Improve`;
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
5. Assign or preserve a stable finding key based on the violated requirement and
   concrete reachable behavior, not a transient line number.
6. Classify the item as exactly one of:
   - **Fix** — verified on the current head, within requested or approved scope,
     compatible with available approved decisions, and authorized for local
     correction. Return a complete canonical correction input containing the
     correction task and expected behavior, finding key and retained attempt
     history, decision source and non-goals, discipline, file responsibilities,
     workspace and working directory, exact correction base, every verification
     command and expected result, complete active Review policy and provenance,
     capacity and queue rules, and optional non-duplicative plan task context.
     Do not implement it here.
   - **Push back** — incorrect, unsupported, preference-only, stale, not
     reproducible on the current head, or already decided without new evidence.
     Cite the controlling decision or code and test evidence.
   - **Escalate** — resolution requires a design or public-contract decision,
     material scope expansion, new authority, or, in a coordinator-managed entry,
     the bounded retry stop has been reached.
7. Record current-head evidence and one concrete next action.

## Report

Use concise technical language. Cite code, tests, or decisions. Avoid performative agreement, defensive phrasing, and speculative concessions.

Return only `Fix`, `Push back`, or `Escalate`; never return an implementation or
phase-completion status from this skill.

For each finding report:

- current head, immutable target identity, and reviewed range;
- stable finding key and retained attempt history;
- classification: `Fix`, `Push back`, or `Escalate`;
- requirement and concrete evidence;
- impact and exact next action;
- any unverified gap.

For a coordinator-managed entry, do not ask for additional approval for a `Fix`
when implementation authorization is already recorded and the correction remains
within approved scope. Return its complete canonical correction input to the
coordinator for `execute-task`. Return `Push back` with decision or code evidence
so triage can continue. Return `Escalate` with the exact user-owned decision,
policy replacement, or new authority required.

For a standalone read-only entry, return a `Fix` as evaluation only with proposed
execution steps. Do not edit files, start a fix, or advance another workflow
phase.

After an authorized coordinator-managed `Fix`, the coordinator must require fresh
correction commit, new head, and exact correction range from `execute-task`, then
fresh global verification and the complete applicable final review against the
new full target. Earlier verification, review, and classification evidence is
stale after the fix commit. For planned work, the coordinator routes the
correction through `execute-plan` so its last-accepted aggregate and one
in-flight partial record remain separate.
