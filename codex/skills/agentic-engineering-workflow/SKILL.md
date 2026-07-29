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

## Preserve the bounded global review target

For every coordinator-managed global gate, create one
`coordinator-target-manifest/v1` request with:

```text
schema_version
scope_kind
base_commit_oid
head_commit_oid
range_ref
base_tree_oid
head_tree_oid
changed_path_manifest
changed_path_manifest_digest
index_state_digest
worktree_state_digest
in_scope_untracked_state_digest
strict_clean_assertion
```

Set `scope_kind` to `committed-range` and `range_ref` to the exact
`base_commit_oid..head_commit_oid` reference. Resolve changed paths locally from
those Git objects and record status, old and new path when applicable, modes, and
base/head object IDs without embedding file contents. Use SHA-256 over canonical
serialization for the changed-path and repository-state digests. Set
`strict_clean_assertion` only when no in-scope content exists outside the
committed range.

Require `verify` to re-resolve the commits, trees, range, changed paths, and
current repository state before deriving the content-bound target identity.
Freeze the validated manifest and returned identity for `review`,
`receiving-code-review`, and `finish-branch`. A missing or stale object, digest,
or clean assertion is `BLOCKED`.

A later accepted correction creates a new manifest request and target identity.
Standalone verification and review targets never advance this coordinated flow.

For a `Push back`, retain the concrete finding and controlling approved evidence
with the current target. Give that decision evidence to a fresh review of the
unchanged target. The same finding does not survive again without materially new
evidence. Do not assign finding keys or create another finding protocol.

## Advance only on current evidence

Advance automatically within approved scope:

1. Accept from lightweight `execute-task` only an `Accepted` result for the
   current head and exact task base-to-head range. Build the target request from
   the original lightweight base through that head.
2. Accept from `execute-plan` only all ordered task results plus the distinct
   aggregate current head and full implementation range. Build the target request
   from the original plan base through that head.
3. Require no unexplained in-scope work outside the requested range. Pass the
   target request, Review context, Review policy, task evidence, changed files,
   and observed commands to `verify`.
4. Advance to `review` only on a fresh verification `PASS` whose returned
   manifest, target identity, current head, and range match the request. Pass the
   same frozen target, Review context, policy, and verification evidence to every
   selected final reviewer and adversarial integrator.
5. Send concrete current findings with the same frozen target and current
   evidence to `receiving-code-review` for `Fix`, `Push back`, or `Escalate`
   classification. Do not reinterpret a blocked or incomplete result as clean.

For `Fix`, route one bounded correction through the active path. Retain the
approved decisions, non-goals, Review context, Review policy, exact finding,
current task base and head, and the observed correction attempts. Lightweight
work returns directly to `execute-task`; planned work returns through
`execute-plan` as a concrete plan correction step. After acceptance, rerun fresh
global verification and the complete final review over the full updated range.

If the same concrete problem repeats without progress or another action would
repeat an observed failed correction, stop and report the attempts and remaining
gap. Use `Escalate` when resolution needs a user-owned decision, new authority,
scope, or policy. Use `BLOCKED` when current operational state cannot be
established. Never discard uncertain state to force progress.

Never advance failed or `BLOCKED` verification to review, blocked review to
triage, unresolved triage to correction, or incomplete evidence to
`finish-branch`.

## Terminate only at a real boundary

Enter `finish-branch` only when fresh verification passes and final review is
clean for the exact current head and full implementation range, the approved
Review policy is satisfied, the frozen target still matches, and no finding or
gap remains. Pass that target to `finish-branch`, then stop for the user's
publication or branch-disposition choice.

Never treat an edit, successful command, implementation commit, agent
self-review, stale per-task approval, or incomplete aggregate as workflow
completion. Report concise current-head evidence, Review context, policy,
transitions taken, remaining findings, and every unverified gap.
