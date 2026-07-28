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

When the request does not explicitly approve another mode, materialize this
complete `focused` base policy:

- **Mode and rationale:** `focused`, because the Design makes it the lightweight
  default and the recorded eligibility evidence proves a uniquely specified
  coherent change with no material user-owned choice or qualifying material risk.
- **Risk surfaces:** record every observed non-material surface and the evidence
  that keeps it lightweight. An absent surface is not silently treated as
  reviewed.
- **Per-task gate:** one combined specification-and-quality pass using the
  complete focused contract.
- **Final required reviewer:** `code-reviewer`, for general correctness and
  maintainability review of the verified current change.
- **Final conditional reviewer:** `test-coverage-reviewer` exactly when behavior
  or tests changed. Record that trigger and determine it from the task contract,
  changed files, and current diff.
- **Skipped perspectives:** classify `design-alignment-reviewer`,
  `scope-reviewer`, `code-architect`, `adversarial-api-reviewer`,
  `adversarial-robustness-reviewer`, `adversarial-performance-reviewer`, and
  `adversarial-tests-reviewer` as skipped because no corresponding qualifying
  lightweight risk is present. Classify `adversarial-integrator` as skipped
  because no adversarial perspective is selected. Record that reason for each
  role.
- **Residual risk:** a combined task gate and general final review do not provide
  independent specialist or adversarial coverage; lightweight eligibility,
  exact task evidence, fresh global verification, final `code-reviewer`, and the
  conditional test-coverage pass bound that risk.
- **Capacity and queue:** use a stricter applicable repository limit when one is
  declared, otherwise configure the workflow maximum of six total threads
  including the lead. Record observed runtime capacity and effective capacity as
  their minimum. Queue without reducing scope in this order: selected task
  writer, focused per-task reviewer, final `code-reviewer`, then triggered
  `test-coverage-reviewer`.
- **Acceptance:** only a concrete `Must Fix` or `Should Improve` finding with a
  cited contract, reachable consequence, and specific correction survives.
  Drop preference-only comments, speculative future concerns, and unsupported
  objections to approved decisions.

Record field-level provenance: Design default for mode, gate, final applicability,
workflow maximum, and Acceptance; original request for implementation and any
mode or no-agent authorization; eligibility and repository evidence for risk,
skips, residual risk, and a stricter configured limit; current runtime evidence
for observed/effective capacity and queue state.

If any skipped perspective becomes applicable, the observed risk conflicts with
the lightweight base policy. Stop for a planned policy instead of adding a
reviewer or silently strengthening `focused`.

When the request explicitly approves `adaptive` or `deep`, construct the policy
mode-consistently from observed applicable risks and the current final-review
applicability contract. Both modes require independent specification and quality
task reviewers. For `adaptive`, classify every defined final perspective as
required, conditional with an exact trigger, or skipped with a reason based only
on recorded applicable risk. For `deep`, classify every applicable standard and
adversarial perspective as required and require `adversarial-integrator` whenever
an adversarial perspective runs; classify only demonstrably inapplicable
perspectives as skipped with reasons. Apply the same capacity, queue, Acceptance,
residual-risk, and per-field provenance requirements.

Reject an incomplete, contradictory, or mode-inconsistent policy. If
classification requires a material choice, a risk is incompletely classified, or
the requested mode conflicts with observed risk, return to planning. Do not
resolve reviewer prompts, dispatch roles, normalize severity, implement, commit,
or manage corrections in this coordinator.

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
6. Route an authorized `Fix` according to its active path:
   - for lightweight work, create a bounded correction task and invoke
     `execute-task`; retain the original lightweight base and restart global
     verification and final review against the full updated lightweight range;
   - for planned work, add a concrete authorized correction step to the current
     plan and invoke `execute-plan` with the original implementation base,
     ordered accepted task records, current aggregate head and full range, and
     retained stable-key history. Require `execute-plan` to invoke
     `execute-task`, append the accepted correction record, and return the
     updated aggregate final HEAD and full implementation range.
7. After the path-specific correction returns current accepted evidence, restart
   global verification and complete final review against the full updated change
   range, never only the correction task range.
8. For `Push back`, retain the decision and evidence and continue triage.
9. For `Escalate`, stop and ask for the named decision or authority.

Treat an in-scope verification failure as an automatic transition through
`systematic-debugging` to the same path-specific correction route: lightweight
through a bounded `execute-task`, planned through a concrete `execute-plan`
correction step and then `execute-task`. Preserve the original change base,
ordered planned task map when present, approved decisions, non-goals, policy,
stable-key history, failed command, and observed evidence. Stop when the
correction would change design, scope, policy, or authority.

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
