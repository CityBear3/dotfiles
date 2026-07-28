---
name: agentic-engineering-workflow
description: Route engineering work across read-only, lightweight, and planned paths while enforcing approval gates and coordinating cross-phase transitions.
---

# Agentic engineering workflow

Own path classification and cross-phase transitions only. Let each phase skill
own investigation, task execution, plan orchestration, verification, review, and
publication mechanics. Follow repository guidance and explicit user instructions
when they are stricter.

Treat `verify`, `review`, and `receiving-code-review` as check-only phases. They
return evidence or classifications and never edit tracked/source state, commit a
fix, or advance the workflow. This coordinator alone consumes their results and
selects the next phase.

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

An explicit no-agent instruction is compatible only with an approved `focused`
lead-pass contract. It conflicts with `adaptive` or `deep` independent per-task
and final perspectives. Never replace those perspectives with sequential lead
passes or record waived independence as accepted completion evidence; return the
exact policy/user conflict for `Escalate`.

Reject an incomplete, contradictory, or mode-inconsistent policy. If
classification requires a material choice, a risk is incompletely classified, or
the requested mode conflicts with observed risk, return to planning. Do not
resolve reviewer prompts, dispatch roles, normalize severity, implement, commit,
or manage corrections in this coordinator.

Build the canonical lightweight task context required by `execute-task`,
including the complete task, decision source and non-goals, discipline,
workspace, base commit, exact verification, complete active policy and
provenance, and capacity. Carry prior stable-key history in the mutable task
record outside that immutable context. Invoke `execute-task` once for that task.

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
   exact task range, create one immutable coordinator verification target from
   the original lightweight base through that exact current HEAD. Require no
   in-scope index, worktree, or untracked source state outside the committed
   target, then send the target, complete policy, and acceptance evidence to
   `verify`.
2. When `execute-plan` returns every ordered task record plus a separate
   aggregate final HEAD and full implementation range, confirm that the
   aggregate head is current and the committed range contains no extra in-scope
   index/worktree source state, then send that immutable full-range target and
   aggregate evidence to `verify`.
3. Accept only a coordinator-managed verification `PASS` whose target identity,
   exact current HEAD, and full range match the immutable target. Send that same
   target, complete approved policy, and fresh verification evidence to
   `review`. Standalone-only snapshot or fileset evidence never advances.
4. Accept from `review` exactly `CLEAN`, `FINDINGS`, or `BLOCKED`. Send only
   schema-complete `FINDINGS` for the same immutable target to
   `receiving-code-review`. A missing or unknown severity first requires
   reviewer re-output and can never be inferred, dropped, or treated as clean.
5. Consume from `receiving-code-review` only:
   - `Fix` when valid, authorized, in scope, and compatible with approved
     decisions;
   - `Push back` when incorrect, unsupported, preference-only, or already
     decided without new evidence;
   - `Escalate` when it requires a decision, authority, scope or policy change,
     or `execute-task` reports exhausted retry.
6. Require every `Fix` to include its stable key, retained attempt history, and
   complete canonical correction input. Route it according to the active path:
   - for lightweight work, create a bounded correction task and invoke
     `execute-task`; retain the original lightweight base and restart global
     verification and final review against the full updated lightweight range;
   - for planned work, add a concrete authorized correction step to the current
     plan and invoke `execute-plan` with the original implementation base,
     ordered accepted task records, current aggregate head and full range, and
     retained stable-key history. Require `execute-plan` to invoke
     `execute-task`, append the accepted correction record, and return the
     updated aggregate final HEAD and full implementation range.
7. Do not accept a final `Fix` until `execute-task` has produced a correction
   commit, new HEAD, exact correction range, current evidence, and task
   acceptance. After path-specific acceptance, build a new immutable full target
   and restart global verification plus the complete final review for that
   target, never only the correction task range. Earlier global evidence is
   stale.
8. For `Push back`, retain the decision and evidence and continue triage. When
   no accepted finding remains, require a fresh complete final-review `CLEAN`
   result for the unchanged full target; never reinterpret the earlier
   `FINDINGS` verdict as clean.
9. For `Escalate`, stop and ask for the named decision or authority.

Classify every verification `FAIL` or `BLOCKED`, review `BLOCKED`, or other
non-success before taking another action:

- when a safely discoverable local input can be resolved read-only within
  existing authority, resolve it and rerun the same phase against the unchanged
  target;
- when the cause is local, recoverable, within approved scope, and correction is
  already authorized, use `systematic-debugging`, then the same path-specific
  `execute-task` correction route;
- when resolution needs a user-owned decision, new authority, material scope, or
  a complete replacement Review policy, return `Escalate` with the exact choice;
- when the cause is external or runtime state, stop with `BLOCKED`, ownership
  evidence, and the exact condition required for re-entry.

A material actual-diff risk missing from the approved policy is the third case:
never add or skip a reviewer silently; require a complete user-approved
replacement policy. Preserve the immutable current target, stable key and attempt
history, successful evidence, ownership, gaps, and re-entry condition across
every stop or rerun. Never advance `BLOCKED` verification to review, `BLOCKED`
review to triage, or any incomplete result to `finish-branch`.

Require `execute-task` to return its exact current head and range, and
`execute-plan` to return all task acceptance records plus the distinct aggregate
head and range. On re-entry, retain current evidence and the exact unresolved
condition rather than reopening settled design without new evidence.

## Terminate only at a real boundary

Enter `finish-branch` only when the exact current HEAD and full implementation
range have a strict coordinator-managed fresh verification `PASS` and final
review `CLEAN` for the same immutable target, the complete approved policy and
actual-risk inventory are satisfied, and no finding or gap remains. Never use
standalone-only evidence for completion. Then stop for the user's publication or
branch-disposition choice.

Never treat an edit, successful command, implementation commit, agent
self-review, stale per-task approval, or incomplete aggregate as workflow
completion. Report concise current-head evidence, the complete policy and
provenance, transitions taken, remaining findings, and every unverified gap.
