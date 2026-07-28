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

## Own the bounded target manifest and resolved-finding history

For every coordinator-managed global gate, create one bounded
`coordinator-target-manifest/v1` request with exactly this field set:

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

Set `schema_version` to `coordinator-target-manifest/v1` and `scope_kind` to
`committed-range`. Set `range_ref` to the exact
`base_commit_oid..head_commit_oid` reference. Build `changed_path_manifest` as a
bounded, canonically ordered list of changed-path records resolved locally from
those Git objects; each record contains status, old and new path when applicable,
modes, and base/head blob or tree object IDs, never file contents or a literal
diff/patch payload. Under this schema, use SHA-256 over canonical serialization
for the changed-path and repository-state digests. Set `strict_clean_assertion`
only when the index, tracked worktree, and in-scope untracked source state prove
that no in-scope content exists outside the committed range.

Supply every field, but do not create the target identity. Require `verify` to
re-resolve the commits, trees, range, bounded changed paths, and current
repository state locally; validate every supplied object ID, record, digest, and
clean assertion; then derive exactly once the content-bound target identity as
the schema-tagged SHA-256 digest of the complete canonical manifest. Accept only
the same manifest plus that returned identity. Freeze both and pass them verbatim
to `review`, `receiving-code-review`, and `finish-branch`. Those phases locally
re-resolve and validate the referenced Git content and state; they never require
or re-inline a literal patch or file-content payload, rename or regenerate the
manifest or identity, or substitute another field set. A stale or missing commit,
tree, changed-path record/digest, index/worktree/in-scope state digest, or clean
assertion is `BLOCKED` with a stable gap key, ownership, and exact re-entry
condition.

A later accepted correction creates a new manifest request and lets the next
coordinator-managed verification derive a new identity. Standalone `verify` and
`review` may create their own identities only under their explicit standalone
target schemas; those identities remain separate and standalone-only.

Maintain a resolved-finding registry keyed by the frozen target identity plus
stable finding key. Require reviewer output, integration, review synthesis, and
triage to preserve each coordinator-managed stable key verbatim. For every
`Push back`, store that exact key, its concrete requirement/behavior identity,
the classification, and the controlling code, test, Design, plan, or other
approved-decision evidence. An empty registry is still explicit input. Pass the
registry or one immutable resolvable reference to it with every fresh
final-review request and every reviewer or integrator dispatch.

A finding with the same target and stable key must not survive dispatch or
synthesis without materially new evidence. New evidence is a delta under the
same requirement/behavior key and permits re-evaluation only when the reviewer
cites that exact delta. Create a new key only when the concrete requirement or
reachable behavior is materially different under the bounded coordinator retry
rules. Retain bounded attempt history for `Fix` separately; the
resolved-finding registry never resets or waives that retry contract.

## Advance only on current evidence

Make these cross-phase transitions automatically within approved scope:

1. When lightweight `execute-task` returns `Accepted` for the current head and
   exact task range, create the exact coordinator target manifest request from
   the original lightweight base through that current HEAD. Require no
   in-scope index, worktree, or untracked source state outside the committed
   range, then send the request, complete policy, and acceptance evidence to
   `verify`.
2. When `execute-plan` returns every ordered task record plus a separate
   aggregate final HEAD and full implementation range, confirm that the
   aggregate head is current and the committed range contains no extra in-scope
   index/worktree source state, then send the exact full-range target manifest
   request and aggregate evidence to `verify`.
3. Accept only a coordinator-managed verification `PASS` that resolves one
   content-bound identity once and whose returned manifest fields, exact current
   HEAD, and full range match the coordinator request. Freeze that identity and
   pass it verbatim with the same manifest, complete approved policy, fresh
   verification evidence, and current resolved-finding registry to `review`.
   Standalone-only snapshot or fileset evidence never advances.
4. Accept from `review` exactly `CLEAN`, `FINDINGS`, or `BLOCKED`. Send only
   schema-complete `FINDINGS`, the frozen target identity and manifest verbatim,
   evidence for that same target, and the current resolved-finding registry to
   `receiving-code-review`, preserving every stable finding key verbatim. A
   missing or unknown severity first requires reviewer re-output and can never be
   inferred, dropped, or treated as clean.
5. Consume from `receiving-code-review` only a top-level `TRIAGED` whose target
   and entry/exit repository-state checks succeeded and whose every item is
   exactly one of:
   - `Fix` when valid, authorized, in scope, and compatible with approved
     decisions;
   - `Push back` when incorrect, unsupported, preference-only, or already
     decided without new evidence;
   - `Escalate` when it requires a decision, authority, scope or policy change,
     or `execute-task` reports exhausted retry.
   Require every coordinator-managed classification and correction record to
   preserve the review-supplied key verbatim. A provided key that conflicts with
   its concrete requirement/behavior identity makes triage `BLOCKED` as a schema
   gap; never re-key it. A triage `BLOCKED` never supplies item classifications
   and never advances.
6. Require every `Fix` to return exactly two non-overlapping path-neutral
   records:
   - one immutable correction specification containing target identity, exact
     requirement, concrete evidence, exact correction, decision source and
     non-goals, discipline, file responsibilities, and verification requirements
     with expected results;
   - one mutable correction record containing only the correction-specification
     identity/reference, preserved stable key, retained attempt history,
     lifecycle and attributable partial evidence, gaps, and re-entry condition.
   The correction-specification identity is not an `execute-task` canonical
   context identity. Route the specification and mutable record according to the
   active path:
   - for lightweight work, this coordinator is the sole canonical builder. Build
     exactly one `execute-task` correction context from the specification plus
     the retained lightweight task context, policy/provenance, workspace,
     capacity, and queue rules; bind that one context identity into the mutable
     correction record without removing its specification reference, then pass
     the context and mutable record separately to `execute-task`; retain the
     original lightweight base and restart global verification and final review
     against the full updated lightweight range;
   - for planned work, convert the correction specification into one concrete
     authorized plan correction step. Send that step, the unchanged specification
     and mutable correction record, original implementation base, ordered
     accepted task records, current aggregate head, and full range to
     `execute-plan`. `execute-plan` remains the sole planned-path canonical
     context builder, binds that context identity into the mutable record, and
     invokes `execute-task`; require it to append the accepted correction record
     and return the updated aggregate final HEAD and full implementation range.
7. Do not accept a final `Fix` until `execute-task` has produced a correction
   commit, new HEAD, exact correction range, current evidence, and task
   acceptance. After path-specific acceptance, build a new exact full target
   manifest request and restart global verification plus the complete final
   review for that target, never only the correction task range. Earlier
   identity and global evidence are stale.
8. For `Push back`, write the target identity, stable key, classification, and
   controlling evidence to the resolved-finding registry using the exact
   review-supplied key and continue triage.
   When no accepted finding remains, run a fresh complete final review of the
   unchanged full target with that registry supplied to every reviewer and
   integrator. Require its report to show registry application and return
   `CLEAN`; never reinterpret the earlier `FINDINGS` verdict as clean. Suppress
   the same key without materially new evidence; when new evidence exists,
   require the exact delta before re-evaluation.
9. For `Escalate`, stop and ask for the named decision or authority.

Classify every verification `FAIL` or `BLOCKED`, review `BLOCKED`, triage
`BLOCKED`, or other non-success before taking another action:

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
history, resolved-finding registry, successful evidence, ownership, gaps, and
re-entry condition across every stop or rerun. Never advance `BLOCKED`
verification to review, `BLOCKED` review to triage, `BLOCKED` triage to a
correction, or any incomplete result to `finish-branch`.

Require `execute-task` to return its exact current head and range, and
`execute-plan` to return all task acceptance records plus the distinct aggregate
head and range. On re-entry, retain current evidence and the exact unresolved
condition rather than reopening settled design without new evidence.

## Terminate only at a real boundary

Enter `finish-branch` only when the exact current HEAD and full implementation
range have a strict coordinator-managed fresh verification `PASS` and final
review `CLEAN` for the same coordinator-frozen immutable target identity, the
complete approved policy and actual-risk inventory are satisfied, and no finding
or gap remains. Pass that identity and its target manifest verbatim to
`finish-branch`. Never use standalone-only evidence for completion. Then stop
for the user's publication or branch-disposition choice.

Never treat an edit, successful command, implementation commit, agent
self-review, stale per-task approval, or incomplete aggregate as workflow
completion. Report concise current-head evidence, the complete policy and
provenance, transitions taken, remaining findings, and every unverified gap.
