---
name: agentic-engineering-workflow
description: Route engineering work across read-only, lightweight, and planned paths while enforcing approval gates and coordinating verification, review, re-entry, and completion. Use for engineering requests that may move between investigation, design, planning, implementation, review feedback, or branch completion.
---

# Agentic engineering workflow

Own path selection and cross-phase transitions. Let each phase skill own its
investigation, implementation, verification, review, and publication mechanics.
Follow repository-local guidance and explicit user instructions when they are
stricter.

## Classify the request

Inspect the relevant repository state before selecting a route.

- For an explanation, diagnosis, review, planning, or other read-only request,
  inspect and report without implementing. Do not treat a request for analysis or
  a plan as authorization to implement a change.
- For an explicit change request, choose the lightweight path only when its
  complete eligibility contract holds. Otherwise use the planned path.
- Honor an explicit request to skip a phase or execute directly without agents
  only when the same scope, evidence, verification, review, and approval
  boundaries remain intact. Never invent a missing user-owned decision.

For every transition, retain and report:

- the active path and phase;
- the approved scope and its decision source;
- the active review policy;
- the next automatic action or user-controlled gate;
- the evidence required to leave the phase;
- any unresolved condition that blocks a safe transition.

## Use the lightweight path only when fully eligible

Require all of these conditions after investigation:

- the user explicitly requested a change;
- the objective, expected behavior, and scope are uniquely determined;
- no architecture, public API or other public-contract, schema, or error-model
  decision changes;
- no material user-owned trade-off remains;
- the work is one coherent change;
- the work needs no external write, publication, destructive action, or material
  scope expansion.

Do not use file count or changed-line count as eligibility criteria. Treat
security or permission boundaries, persistent-data migration, concurrency or
recovery guarantees, and data-loss risk as disqualifying unless investigation
shows that the requested change does not alter that contract.

Treat the original change request as implementation approval once every criterion
holds. Confirm the workspace with `create-workspace`, select TDD for production
behavior or an explicit contract-appropriate discipline for content,
configuration, and mechanical migrations, and use `focused` review by default.
Apply `adaptive` or `deep` instead when the user explicitly requests it.

If implementation reveals a disqualifying risk or material decision, stop the
lightweight path. Return to `design-discussion`, preserve the evidence that caused
the reclassification, and create a plan after the revised scope is settled. Do
not merely strengthen review and continue.

## Use approval gates on the planned path

Use `design-discussion` whenever architecture, scope, algorithms, public
contracts, or another material trade-off remains unresolved. Let the user make
each material choice.

Use `design-doc` when the settled work has cross-cutting architecture, durable
contracts, or significant decisions worth preserving. Require user approval of
the drafted Design Doc. Otherwise proceed to `create-plan` only after the user
approves the transition from design discussion.

Require approval of the implementation plan, including its review policy, before
using `create-workspace` and `execute-plan`. Stop for:

- an unresolved design, architecture, scope, algorithm, or public-contract
  choice;
- Design Doc or implementation-plan approval;
- a plan deviation or material scope expansion;
- publication, push, pull-request creation, merge, discard, or destructive
  action;
- an external write or other authority not granted by the request.

Do not repeat an approval prompt after its decision source is recorded and the
work remains within that authority.

## Apply the review policy

Use `focused` as the lightweight default and `adaptive` as the planned default.
For planned work, apply the policy approved in the plan; recommend a different
mode only from concrete risk evidence. Keep the policy unchanged across
implementation and re-entry unless the user approves a replacement.

- `focused`: use one combined specification-and-quality per-task gate and the
  focused final-review contract.
- `adaptive`: use independent specification and quality per-task gates and only
  the final review perspectives justified by recorded risk surfaces.
- `deep`: use independent specification and quality per-task gates, every
  applicable standard and adversarial final perspective, and adversarial
  integration.

Apply the policy's recorded risk surfaces, required and skipped perspectives,
residual risk, capacity rules, and finding threshold. Queue approved reviewers
when capacity is constrained; never silently reduce the approved scope. Keep
model and reasoning-effort selection in reviewer profiles rather than the policy.

Accept only findings with a concrete reachable behavior or contract violation,
cited evidence, impact, and a specific correction. Remove preference-only,
speculative, or already-decided objections that lack new evidence.

## Advance automatically within approved scope

Once implementation is authorized, make these transitions without another user
prompt:

1. Send completed implementation to `verify`.
2. On verification PASS, send the current head and evidence to `review`.
3. Send review findings to `receiving-code-review`.
4. Classify every surviving finding:
   - `Fix`: valid, in scope, and compatible with approved decisions;
   - `Push back`: incorrect, unsupported, preference-only, or already decided;
   - `Escalate`: requires a new design decision, authority, or material scope
     expansion.
5. For `Fix`, execute a bounded correction, then run fresh verification and the
   complete applicable review again.
6. For `Push back`, retain the decision or code evidence and continue triage.
7. For `Escalate`, stop and ask the user for the named decision or authority.

Treat an in-scope verification failure as an automatic diagnosis-and-fix loop,
not a user gate. Use `systematic-debugging`, correct the cause within approved
scope, and run fresh verification. Stop when correction would change approved
design or scope, authority is missing, or the bounded retry condition shows that
the current approach is not working.

On re-entry, retain approved decisions, non-goals, review policy, verification
state, and the exact unresolved failure or finding. Do not reopen settled design
without new evidence.

## Terminate only at a real boundary

Enter `finish-branch` only when verification is current for the head commit and
the approved final review has no remaining Must Fix or Should Improve finding.
Then stop for the user's publication or branch-disposition choice.

Never treat an edit, successful command, implementation commit, agent self-review,
or earlier per-task approval as workflow completion. Report concise current-head
evidence: commands observed, review policy, reviewers run or skipped with reasons,
remaining findings, and every unverified gap.
