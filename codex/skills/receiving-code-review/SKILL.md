---
name: receiving-code-review
description: Verify review findings against current code and classify each as Fix, Push back, or Escalate. Use from the workflow coordinator for authorized review loops or standalone for read-only feedback evaluation.
---

# Triage current-head review findings

Treat review as technical evidence, not an instruction to agree. Remain
check-only and read-only. Do not mutate source or Git state, implement or stage a
fix, dispatch a writer, or advance another workflow phase.

## Coordinator-managed entry

Require:

- exact implementation base, current head, full range, diff, status, and changed
  files;
- fresh verification `PASS` and review `FINDINGS` for that same unchanged head
  and range;
- each finding's severity, file and line, concrete behavior, requirement,
  evidence, impact, proposed correction, and confidence;
- approved scope, decisions, non-goals, Review context, Review policy, and
  implementation authorization;
- one exact authority form: approved Design Doc when present, Feature Contract,
  Implementation Plan, and applicable Task Contracts; the complete lightweight
  combined Feature/Task Contract with original request authority and exact
  accepted-task evidence; or the exact eligible legacy plan authority;
- observed correction attempts and prior triage decisions.

Resolve base, head, range, diff, status, and changed files directly from Git.
Return top-level `BLOCKED` without classifying findings when the target is stale,
evidence is missing, or in-scope source state falls outside the reviewed range.

## Standalone read-only entry

Resolve each complete finding, current code, head, relevant range or files,
repository guidance, and available design and plan evidence through read-only
investigation. Implementation authorization is not required to evaluate
standalone feedback, and the evaluation does not authorize a change.

Derive or use the available Review context and disclose material assumptions.
Return missing or stale verification as a limitation.

## Establish one current snapshot

Before classifying any item, capture:

- current HEAD, base and range when applicable;
- `git status --short`, staged and unstaged diffs, changed files, and relevant
  untracked paths;
- available code, tests, approved decisions, Review context, Review policy,
  review evidence, and observed correction attempts.

If a ref, finding, required input, dependency, permission, or runtime condition
is missing, return top-level `BLOCKED` before classification. Preserve observed
evidence and state the exact re-entry condition.

## Process each finding

For each item:

1. Read the complete finding and cited code.
2. Restate the concrete requirement.
3. Reproduce or verify the claim against the current head.
4. Check repository guidance, current code and tests, approved scope,
   non-goals, Design Doc, Feature Contract, Task Contracts, eligible legacy plan
   authority, plan, Review context, and Review policy as applicable.
5. Classify it as exactly one:
   - **Fix** — verified on the current head, in approved scope, compatible with
     approved decisions, proportionate, and authorized for local correction.
   - **Push back** — incorrect, unsupported, preference-only, speculative,
     second-order, artifact-inapplicable, stale, not reproducible, or already
     decided without materially new evidence.
   - **Escalate** — requires a design or public-contract decision, architecture
     mechanism without proven proportionate need, material scope or policy
     change, new authority, or a stop after repeated correction without progress.
6. Record current-head evidence and one concrete next action.

For `Fix`, return one bounded plain-language correction handoff:

- exact finding and concrete evidence;
- smallest authorized correction;
- unchanged planned Feature and applicable Task Contract, unchanged lightweight
  combined contract, or unchanged eligible legacy task authority, plus shared
  interfaces when applicable, constraints, and non-goals;
- Review context and unchanged Review policy;
- discipline and responsibility boundaries;
- a correction commit intent bounded to the finding and either its fixed message
  or explicit writer authority to select the correction message;
- current task base, verification obligations, and contractually required exact
  commands;
- observed prior attempts, concerns, and gaps.

Do not choose the lightweight or planned builder here. The coordinator routes
the handoff directly to `execute-task` for lightweight work or through
`execute-plan` for planned work.

For `Push back`, cite controlling code, test, Design, plan, or approved decision
evidence. The same finding may be reconsidered only with materially new evidence
of a reachable failure or approved-contract violation.

## Revalidate and report

Immediately before reporting, capture the same HEAD, status, diffs, changed
files, relevant untracked paths, and range again. If any in-scope state changed,
discard provisional classifications and return `BLOCKED`. Do not restore, reset,
clean, or discard state.

Return exactly one top-level result:

- `TRIAGED` when every finding is classified as `Fix`, `Push back`, or
  `Escalate`;
- `BLOCKED` for stale state, missing evidence or input, or an external/runtime
  failure.

For `TRIAGED`, report:

- base, current head, reviewed range or bounded files, status, and changed files;
- Review context and verification/review evidence inspected;
- classification, requirement, evidence, impact, and next action for each item;
- bounded correction handoff for every `Fix`;
- controlling evidence for every `Push back`;
- exact user-owned decision or authority for every `Escalate`;
- concerns, observed attempts, and gaps.

For `BLOCKED`, report available target evidence, preserved checks, every gap, and
the exact safe re-entry condition. Do not report provisional classifications.

An authorized coordinator-managed `Fix` does not need another approval when it
remains within scope. It still requires bounded implementation, a correction
commit and new head, fresh verification, and complete fresh review over the full
updated range. Earlier verification, review, and triage evidence becomes stale.

For standalone review feedback, return evaluation and proposed steps only. Do
not implement, start another phase, or imply authorization.
