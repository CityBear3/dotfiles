---
name: agentic-engineering-workflow
description: Route engineering work across read-only, lightweight, and planned paths while enforcing approval gates and coordinating cross-phase transitions.
---

# Agentic engineering workflow

Own path classification and cross-phase transitions only. Let each phase skill
own investigation, task execution, plan orchestration, scheduling, verification,
review, triage, and publication mechanics. Follow repository guidance and
explicit user instructions when they are stricter.

Treat `verify`, `review`, and `receiving-code-review` as check-only phases. They
return evidence or classifications and never edit tracked state, commit a fix, or
advance the workflow. This coordinator consumes their results and selects the
next phase.

## Classify the request

Inspect the relevant repository state before selecting a route.

- For an explanation, diagnosis, review, planning, or other read-only request,
  inspect and report without implementing.
- For an explicit change request, use the lightweight path only when its complete
  eligibility contract holds. Otherwise use the planned path.
- Honor a request to skip a phase or avoid agents only when every remaining
  approved contract can still be satisfied. Never invent a user-owned decision
  or silently weaken evidence.

For every transition retain:

- the active path and phase;
- approved scope, decision source, and non-goals;
- the Review context and complete active Review policy;
- the next automatic action or user-controlled gate;
- the evidence required to leave the phase;
- every unresolved condition that prevents a safe transition.

## Use the lightweight path only when fully eligible

Require all of these conditions after investigation:

- the user explicitly requested a change;
- the objective, expected behavior, and scope are uniquely determined;
- no architecture, public API or other public contract, schema, or error-model
  decision changes;
- no material user-owned trade-off remains;
- the work is one coherent change;
- the work needs no external write, publication, destructive action, or material
  scope expansion.

Do not use file count or changed-line count as eligibility criteria. Treat
security or permission boundaries, persistent-data migration, concurrency or
recovery guarantees, and data-loss risk as disqualifying unless investigation
shows that the requested change does not alter that contract.

Treat the original change request as implementation approval when every criterion
holds. Confirm the workspace with `create-workspace`. Select TDD for production
behavior and a contract-appropriate discipline for content, configuration,
refactoring, or mechanical migrations.

If implementation exposes a disqualifying risk or material decision, preserve
the evidence and stop the lightweight path. Return to `design-discussion`, then
planning after the user settles the revised scope. Do not silently broaden the
policy and continue.

## Prepare the lightweight task

Before invoking `execute-task`, derive a concise Review context from the approved
request and repository evidence. State the artifact and purpose, its consumers
and interpretation or execution model, material quality criteria and realistic
failures, approved non-problems, and inapplicable assumptions. Keep it separate
from the Review policy.

Materialize the complete lightweight policy before implementation. If completing
that policy requires a material user-owned choice, or observed risk makes
`focused` inappropriate, return to the planned path before invoking
`execute-task`. Do not silently select or strengthen policy to keep the
lightweight path.

Use `focused` as the lightweight default:

- one combined specification-and-quality per-task gate;
- final `code-reviewer`;
- final `test-coverage-reviewer` only when behavior or tests changed;
- explicit reasons for skipped perspectives;
- a configured maximum of six total threads including the lead unless a stricter
  repository limit applies;
- deterministic queueing without reducing selected scope;
- the common Acceptance threshold.

Acceptance keeps only artifact-applicable findings with an approved requirement,
concrete reachable evidence, material consequence, and proportionate correction.
Preference, speculation, generic best practice, optional polish, and objections
to approved decisions without new evidence are not findings. A proposed new
state machine, schema, identity system, or comparable mechanism is `Escalate`
unless it is necessary and proportionate to a proven in-scope violation.

An explicitly approved `adaptive` or `deep` mode replaces the default.
Both require independent read-only specification and quality task reviewers.
`Adaptive` selects final perspectives for recorded risks. `Deep` runs every
perspective applicable to the artifact and observed risks, not every configured
reviewer. If a required independent reviewer cannot be established, report
`BLOCKED`; do not substitute a lead pass. A no-agent instruction that conflicts
with an approved independent gate is `Escalate` unless the user approves a
policy change.

Give `execute-task` one plain-language task handoff containing:

- the task and expected behavior;
- approved decisions and non-goals;
- the Review context and complete Review policy;
- the discipline and applicable repository guidance;
- working directory and approved workspace;
- exact task base, which is the current head before implementation;
- file responsibilities and boundaries;
- every exact verification command and expected result.

Do not dispatch roles, load reviewer prompts, implement, commit, or manage
corrections in this coordinator.

## Use approval gates on the planned path

Resolve planned-path entry in this order:

1. When architecture, scope, algorithms, public contracts, or another material
   trade-off remains unresolved, use `design-discussion` and let the user make
   each material choice.
2. For an already-approved Design Doc, use `create-plan`.
3. For settled work with cross-cutting architecture, durable contracts, or
   significant decisions worth preserving, use `design-doc`. Require approval
   of the drafted Design Doc, then use `create-plan`.
4. For settled scope that is not lightweight-eligible and does not need a Design
   Doc, record approval to enter planning, then use `create-plan`.

Require approval of the implementation plan, Review context, and complete Review
policy before using `create-workspace` and `execute-plan`. Stop for an unresolved
design choice, approval gate, plan deviation, material scope expansion, external
write, publication, merge, discard, destructive action, or other missing
authority. Do not repeat an approval prompt while its decision remains
applicable.

Pass the approved plan, Review context, complete policy, working directory,
workspace, task base, and retained decisions to `execute-plan`. That skill owns
dependency order, per-task handoff, ordered evidence aggregation, and
plan-deviation detection.

## Prepare concise final-gate evidence

For a coordinator-managed final gate, retain one current evidence summary:

- lightweight or planned path;
- original implementation base, current head, and exact full range;
- current `git status --short` and changed files;
- approved scope, decisions, and non-goals;
- Review context and complete Review policy;
- task commits and reviewer outcomes;
- observed verification commands and results;
- concerns, unresolved findings, and every gap.

Require no unexplained in-scope index, worktree, or untracked source change
outside the committed range. Re-read the current head and status before every
cross-phase transition. Standalone verification or review can answer its direct
request, but never substitutes for coordinator evidence against the current
implementation head.

## Advance only on current evidence

Advance automatically within approved scope:

1. Accept from lightweight `execute-task` only an `Accepted` result for the
   current head and exact task base-to-head range.
2. Accept from `execute-plan` only all ordered task results plus the distinct
   aggregate current head and full implementation range.
3. Build the concise evidence summary and pass it to `verify`. Accept only a
   fresh `PASS` for the same base, current head, full range, changed files, and
   unchanged status.
4. Pass that verification result, Review context, Review policy, exact range,
   diff, changed files, commands, task outcomes, and gaps to `review`. Require
   every selected final reviewer and adversarial integrator to receive the same
   Review context.
5. Accept from `review` only `CLEAN`, `FINDINGS`, or `BLOCKED` for that unchanged
   current head and range. Send concrete current `FINDINGS` and supporting
   evidence to `receiving-code-review` for `Fix`, `Push back`, or `Escalate`
   classification. Do not reinterpret blocked or incomplete evidence as clean.

When global verification fails, diagnose it before acting. Route an authorized
in-scope correction through the active path, then rerun global verification.
Use `Escalate` for a user-owned decision or missing authority; return `BLOCKED`
when the required operational state cannot be established.

For `Fix`, route one bounded correction through the active path using the finding
and current evidence. After acceptance, rerun fresh global verification and the
final review for the updated range.

A `Push back` remains resolved while the reviewed target and controlling evidence
are unchanged; do not repeat the same review solely to reproduce that decision.
After all findings are triaged, continue only when no `Fix`, `Escalate`, or
surviving finding remains. Reconsider a pushed-back finding only with materially
new evidence.

If the same concrete problem repeats without progress or another action would
repeat an observed failed correction, stop and report the attempts and remaining
gap. Use `Escalate` when resolution needs a user-owned decision, new authority,
scope, or policy. Use `BLOCKED` when current operational state cannot be
established. Never discard uncertain state to force progress.

Never advance failed or `BLOCKED` verification to review, blocked review to
triage, unresolved triage to correction, or incomplete evidence to
`finish-branch`.

## Terminate only at a real boundary

Enter `finish-branch` only when fresh verification passes for the exact current
head and full implementation range, the approved Review policy is satisfied,
final review and triage leave no surviving finding or gap, and current status
still matches the reviewed evidence. Pass the concise final-gate evidence to
`finish-branch`, then stop for the user's publication or branch-disposition
choice.

Never treat an edit, successful command, implementation commit, agent
self-review, stale per-task approval, or incomplete aggregate as workflow
completion. Report concise current-head evidence, Review context, policy,
transitions taken, remaining findings, and every unverified gap.
