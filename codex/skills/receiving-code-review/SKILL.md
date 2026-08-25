---
name: receiving-code-review
description: Verify review findings against current code and classify each as Fix, Push back, or Escalate. Use from the workflow coordinator for authorized review loops or standalone for read-only feedback evaluation.
---

# Triage current-target review findings

Treat review as technical evidence, not an instruction to agree. Remain
check-only and read-only. Do not mutate source or Git state, implement or stage a
fix, dispatch a writer, or advance another workflow phase.

## Coordinator-managed entry

Require:

- target kind; exact task workspace and branch, planned PR base, merge base,
  current head and range, or exact integration composition; diff, status, and
  changed files;
- fresh verification `PASS` and either workflow review `FINDINGS` or complete
  human review feedback anchored to that same unchanged head and range;
- for workflow `FINDINGS`, every source report plus the general
  `review-integrator` report for the exact unchanged target, or an explicitly
  authorized focused no-agent lead integration statement that says no
  independent integrator ran; direct human feedback does not fabricate this
  workflow evidence;
- each ordinary finding's severity, file and line, concrete behavior,
  requirement, evidence, impact, proposed correction, and confidence; or each
  separately reported authority-gap claim with its exact authority and defect
  evidence, which does not need a fabricated finding severity;
- approved scope, decisions, non-goals, Review context, Review policy, and
  implementation authorization;
- one exact authority form: approved Design Doc when present, Feature Contract,
  Implementation Plan, and applicable Task Contracts; the complete lightweight
  combined Feature/Task Contract with original request authority and exact task
  evidence; or the exact eligible legacy plan authority;
- observed correction attempts and prior triage decisions.

Resolve the workspace, branch, base, merge base, head, range or composition,
diff, status, and changed files directly from Git.
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

- current HEAD, workspace, branch, planned base, merge base, range or
  integration composition when applicable;
- `git status --short`, staged and unstaged diffs, changed files, and relevant
  untracked paths;
- available code, tests, approved decisions, Review context, Review policy,
  review evidence, and observed correction attempts.

If a ref, finding, required input, dependency, permission, or runtime condition
is missing, return top-level `BLOCKED` before classification. Preserve observed
evidence and state the exact re-entry condition.

## Process each finding

For each item:

1. Read the complete source finding, any supplied integrated assessment, and
   cited code.
2. Restate the concrete requirement and its exact authority.
3. Reproduce or verify the claimed problem against the current unchanged target.
4. Check repository guidance, current code and tests, approved scope,
   non-goals, Design Doc, Feature Contract, Task Contracts, eligible legacy plan
   authority, plan, Review context, and Review policy as applicable.
5. Record whether the current range introduced, worsened, merely exposed, or did
   not cause the problem, and whether ownership belongs to the current Task,
   another approved responsibility, an independent pre-existing concern, or an
   authority gap.
6. Evaluate the proposed remedy separately: whether it is necessary,
   proportionate, in current scope, and already determined by approved authority.
7. Classify each integrated item as exactly one:
   - **Fix** — the current-target problem is verified, the current authority
     owns it, and one bounded proportionate correction is already authorized
     without an unresolved design choice.
   - **Push back** — the problem or proposed remedy is incorrect, unsupported,
     preference-only, speculative, second-order, artifact-inapplicable, stale,
     not reproducible, already decided without materially new evidence, or
     disproportionate. When one report combined an excessive remedy with a
     separately evidence-grounded smaller problem, push back the excessive
     remedy item and classify the integrator's distinct smaller problem item on
     its own evidence.
   - **Escalate** — requires a design or public-contract decision, a material
     scope or policy change, new authority, an architecture mechanism without
     proven proportionate need, or a stop after repeated correction without
     progress. When the applicable Design Doc is missing, contradictory, or
     materially ambiguous, use reason exactly `Design Escalation`.
8. Record current-head evidence and one concrete next action.

A valid problem outside the current authority does not become a `Fix`. When it
is independent of the current change and reveals no authority defect, classify
the current correction request `Push back` and retain the verified problem
separately as a non-blocking concern. Do not create a persistent backlog. When
it reveals a Design authority defect, classify `Escalate` with reason `Design
Escalation`; do not propose silent authority repair.

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
- current planned Task PR base or integration target, verification obligations,
  and contractually required exact commands;
- observed prior attempts, concerns, and gaps.

Do not choose the lightweight or planned builder here. The coordinator routes
the handoff directly to `execute-task` for lightweight work or through
`execute-plan` for planned work.

For `Push back`, cite controlling code, test, Design, plan, or approved decision
evidence. The same finding may be reconsidered only with materially new evidence
of a reachable failure or approved-contract violation.

For `Design Escalation`, identify the exact missing, contradictory, or
materially ambiguous Design Doc authority and the engineer-owned decision
needed. No correction handoff is valid until that authority is approved and its
semantic impact is propagated.

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

- target kind, feedback origin, workspace, branch, base, merge base, current
  head, reviewed range or composition, bounded files, status, and changed files;
- Review context and verification plus workflow or human review evidence
  inspected;
- classification, requirement, evidence, impact, and next action for each item;
- separate problem-validity and proposed-remedy assessments, origin, scope
  owner, and design-sufficiency evidence for each item;
- bounded correction handoff for every `Fix`;
- controlling evidence for every `Push back`;
- exact user-owned decision or authority for every `Escalate`;
- non-blocking concerns retained without a backlog, observed attempts, and gaps.

For `BLOCKED`, report available target evidence, preserved checks, every gap, and
the exact safe re-entry condition. Do not report provisional classifications.

An authorized coordinator-managed `Fix` does not need another approval when it
remains within scope. A Task PR fix still requires bounded implementation, a
correction commit and new head, fresh verification, and complete fresh task
review over the updated PR range. An integration finding routes to its owning
Task Contract, invalidates affected descendants through both topologies, and
then requires fresh affected task and integration evidence. Earlier evidence
for changed targets becomes stale.

A `Design Escalation` stops unstarted review and correction work for the
affected target and returns to the engineer immediately. After an approved
authority change, mark only semantically affected Tasks and their transitive
dependents stale; directly revalidate and retain unchanged Accepted Tasks.

For standalone review feedback, return evaluation and proposed steps only. Do
not implement, start another phase, or imply authorization.
