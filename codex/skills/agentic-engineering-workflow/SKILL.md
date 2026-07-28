---
name: agentic-engineering-workflow
description: Route engineering work across read-only, lightweight, and planned paths while enforcing approval gates and coordinating cross-phase transitions.
---

# Agentic engineering workflow

Own path classification and cross-phase transitions only. Let each phase skill
own investigation, task execution, plan orchestration, verification, review, and
publication mechanics. Follow repository guidance and explicit user instructions
when they are stricter.

## Classify the request

Inspect the relevant repository state before selecting a route.

- For an explanation, diagnosis, review, planning, or other read-only request,
  inspect and report without implementing.
- For an explicit change request, use the lightweight path only when its complete
  eligibility contract holds. Otherwise use the planned path.
- Honor an explicit request to skip a phase or avoid agents only when the
  remaining approved contracts can still be satisfied. Never invent a missing
  user-owned decision or silently weaken evidence.

For every transition retain:

- the active path and phase;
- approved scope, decision source, and non-goals;
- the complete active Review policy and provenance;
- the next automatic action or user-controlled gate;
- the evidence required to leave the phase;
- every unresolved condition that blocks a safe transition.

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

If implementation exposes a disqualifying or material risk, stop the lightweight
path and preserve the evidence. Return to `design-discussion`, then planning
after the user settles the revised scope. Do not silently broaden the policy and
continue.

## Materialize the complete lightweight policy

Before invoking `execute-task`, deterministically create one complete lightweight
Review policy from the approved Design default, original request authorization,
and observed risk and capacity evidence.

Use `focused` as the Design default. Use `adaptive` or `deep` only when the
original request explicitly authorizes that mode. If observed risk would require
a material mode or reviewer choice, conflicts with the requested mode, or is not
covered by the default, return to design or planning for the user-owned policy
decision.

Record every required field:

- mode, rationale, and observed risk surfaces;
- the mode-consistent per-task gate;
- final required reviewers with reasons;
- final conditional reviewers with their triggers and reasons;
- skipped perspectives with reasons;
- residual risk;
- configured and observed capacity plus deterministic queue rules;
- the Acceptance threshold;
- field-level provenance citing the Design default, original request
  authorization, or observed risk and capacity evidence.

Reject an incomplete, contradictory, or mode-inconsistent policy. Do not resolve
reviewer prompts, dispatch roles, normalize severity, implement, commit, or
manage corrections in this coordinator.

Build the canonical lightweight task context required by `execute-task`,
including the complete task, decision source and non-goals, discipline,
workspace, base commit, exact verification, complete active policy and
provenance, capacity, and prior stable-key history. Invoke `execute-task` once for
that task.

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

Require approval of the implementation plan and its complete Review policy before
using `create-workspace` and `execute-plan`. Stop for an unresolved design
choice, approval gate, plan deviation, material scope expansion, external write,
publication, merge, discard, destructive action, or other missing authority.
Do not repeat an approval prompt while its recorded decision remains applicable.

Pass the approved plan, complete policy and provenance, workspace, and retained
decision context to `execute-plan`. That skill owns dependency order, per-task
handoff, ordered evidence aggregation, and plan-deviation detection.

## Advance only on current evidence

Make these cross-phase transitions automatically within approved scope:

1. When lightweight `execute-task` returns `Accepted` for the current head and
   exact task range, send that head, range, policy, and acceptance evidence to
   `verify`.
2. When `execute-plan` returns every ordered task record plus a separate
   aggregate final HEAD and full implementation range, confirm that the
   aggregate head is current, then send the aggregate evidence to `verify`.
3. On verification PASS for that same head, send the current head, target range,
   policy, and verification evidence to `review`.
4. Send review findings to `receiving-code-review`.
5. Classify each surviving result as:
   - `Fix` when valid, authorized, in scope, and compatible with approved
     decisions;
   - `Push back` when incorrect, unsupported, preference-only, or already
     decided without new evidence;
   - `Escalate` when it requires a decision, authority, scope or policy change,
     or `execute-task` reports exhausted retry.
6. For an authorized `Fix`, create a bounded canonical correction context and
   invoke `execute-task`; after it returns `Accepted`, restart the required
   cross-phase verification and review for its new head.
7. For `Push back`, retain the decision and evidence and continue triage.
8. For `Escalate`, stop and ask for the named decision or authority.

Treat an in-scope verification failure as an automatic transition through
`systematic-debugging` to an authorized bounded `execute-task` correction.
Preserve approved decisions, non-goals, policy, failed command, and observed
evidence. Stop when the correction would change design, scope, policy, or
authority.

Never advance a `BLOCKED` or incomplete handoff. Require `execute-task` to return
its exact current head and range, and require `execute-plan` to return all task
acceptance records plus the distinct aggregate head and range. On re-entry,
retain current evidence and the exact unresolved condition rather than reopening
settled design without new evidence.

## Terminate only at a real boundary

Enter `finish-branch` only when verification is current for the head commit and
the approved final review has no remaining `Must Fix` or `Should Improve`
finding. Then stop for the user's publication or branch-disposition choice.

Never treat an edit, successful command, implementation commit, agent
self-review, stale per-task approval, or incomplete aggregate as workflow
completion. Report concise current-head evidence, the complete policy and
provenance, transitions taken, remaining findings, and every unverified gap.
