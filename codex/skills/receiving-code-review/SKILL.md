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

- the coordinator-frozen content-bound target identity and exact coordinator
  target request verbatim, including base, current HEAD, full range, diff
  contents, changed files, and strict repository-state evidence;
- fresh verification `PASS` and review `FINDINGS` evidence for that same
  identity, request, current HEAD, and range;
- every surviving finding in the complete final schema, including final severity
  exactly `Must Fix` or `Should Improve` and its stable finding key;
- the approved decision source, scope, non-goals, review policy, and
  implementation authorization;
- the approved Design Doc and plan when present;
- bounded retry history for a repeated finding;
- the coordinator-owned resolved-finding registry for the target, including an
  explicitly empty registry.

Validate the target-bound Git objects, range and diff contents, changed files,
and current state. Never rename, regenerate, or substitute the coordinator-frozen
identity.

## Standalone read-only entry

When the user invokes this skill outside the coordinator, resolve through local
read-only investigation:

- each complete finding;
- the current code, head, and relevant range;
- applicable repository guidance;
- available design and plan evidence.

Implementation authorization is not required to evaluate a standalone finding.
The evaluation does not authorize a change.

## Establish an all-or-nothing triage snapshot

Before classifying any item, validate every entry input and every finding schema.
Capture the current HEAD, target identity and range, index entries and staged
diff, worktree status and diff, immutable identities for in-scope tracked
contents, and a complete bounded untracked-path inventory with content identities
and in-scope attribution. Also record unrelated dirty state and the available
code, test, decision, review, and retry evidence.

If the target is stale or changed, an input or required evidence is missing, or
a required dependency, permission, or external/runtime condition prevents the
checks, return top-level `BLOCKED` before item classification. Preserve the
validated evidence, but do not return any provisional `Fix`, `Push back`, or
`Escalate`.

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
     correction. Return exactly two non-overlapping records:
     1. one immutable canonical correction context containing the correction
        task and expected behavior, decision source and non-goals, discipline,
        file responsibilities, workspace and working directory, exact correction
        base, every verification command and expected result, complete active
        Review policy and provenance, capacity and queue rules, and optional
        non-duplicative plan-task context; and
     2. one mutable correction/task record containing the canonical context
        identity/reference, finding key and retained attempt history, lifecycle
        phase, attributable partial execution evidence, gaps, and exact re-entry
        condition.
     Exclude the key, history, lifecycle, partial evidence, gaps, and re-entry
     state from the immutable context. Do not duplicate immutable context fields
     in the mutable record or supply either field from another authority. Do not
     implement it here. Assign the complete immutable context one content
     identity and make the mutable record reference that identity exactly once.
   - **Push back** — incorrect, unsupported, preference-only, stale, not
     reproducible on the current head, or already decided without new evidence.
     Cite the controlling decision or code and test evidence so the coordinator
     can store the target-identity-plus-stable-key registry entry.
   - **Escalate** — resolution requires a design or public-contract decision,
     material scope expansion, new authority, or, in a coordinator-managed entry,
     the bounded retry stop has been reached.
7. Record current-head evidence and one concrete next action.

## Revalidate and report

Use concise technical language. Cite code, tests, or decisions. Avoid performative agreement, defensive phrasing, and speculative concessions.

Immediately before reporting, capture the same HEAD, target, index, worktree,
tracked-content, and complete bounded untracked path/content evidence again.
Compare it with the entry snapshot. A concurrent commit, changed target content,
index or worktree mutation, or added, removed, or changed untracked file makes
the target stale: discard provisional classifications and return `BLOCKED`.

Return exactly one top-level phase result:

- `TRIAGED` only when target and evidence validation succeeded at entry and
  immediately before report and every item is classified exactly `Fix`,
  `Push back`, or `Escalate`;
- `BLOCKED` for a stale or changed target, missing evidence or input, or a
  dependency, permission, external, or runtime failure.

For either result, return the coordinator-frozen identity and exact target
request verbatim when they were supplied; never create a replacement identity.

For `TRIAGED`, report for each finding:

- current head, immutable target identity, and reviewed range;
- stable finding key and retained attempt history; for `Fix`, satisfy both
  through the sole mutable correction/task record rather than a parallel copy;
- classification: `Fix`, `Push back`, or `Escalate`;
- requirement and concrete evidence;
- impact and exact next action;
- confirmation that no required target, input, evidence, or runtime gap remains.

For `BLOCKED`, report a stable gap key, likely ownership, the
coordinator-frozen target identity when supplied (or the missing-identity
evidence), available target evidence, every preserved check result, every
unverified gap, and the exact condition required for safe re-entry. Do not report
an item classification, correction context, mutable correction/task record,
phase-completion status, or phase advancement.

For a coordinator-managed entry, do not ask for additional approval for a `Fix`
when implementation authorization is already recorded and the correction remains
within approved scope. Return its immutable canonical correction context and
separate mutable correction/task record to the coordinator for `execute-task`.
Return `Push back` with decision or code evidence so the coordinator can update
the resolved-finding registry. Return `Escalate` with the exact user-owned
decision, policy replacement, or new authority required.

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
