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

Use `coordinator-target-manifest/v1` with exactly this field set:

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

When the workflow coordinator invokes this skill, require:

- the coordinator-frozen content-bound target identity and exact coordinator
  target manifest verbatim containing every field above, with `schema_version`
  equal to `coordinator-target-manifest/v1`, `scope_kind` equal to
  `committed-range`, and no literal diff, patch, or file content payload;
- fresh verification `PASS` and review `FINDINGS` evidence for that same
  identity, manifest, current HEAD, and range;
- every surviving finding in the complete final schema, including final severity
  exactly `Must Fix` or `Should Improve` and its stable finding key;
- the approved decision source, scope, non-goals, review policy, and
  implementation authorization;
- the approved Design Doc and plan when present;
- bounded retry history for a repeated finding;
- the coordinator-owned resolved-finding registry for the target, including an
  explicitly empty registry.

Locally re-resolve the target-bound commit and tree object IDs, exact range,
canonically ordered bounded changed-path object records, every schema-defined
digest, current repository state, and clean assertion. Never require or re-inline
a literal patch or file-content payload, or rename, regenerate, or substitute the
coordinator-frozen identity or manifest. A missing or stale object, tree, range,
path record/digest, index/worktree/in-scope state digest, clean assertion,
identity, version, or field set is a top-level `BLOCKED` schema/evidence gap with
a stable gap key, likely ownership, and exact re-entry condition.

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
For coordinator-managed input, require each review-supplied stable finding key to
match the finding's recorded concrete requirement/behavior identity and preserve
it verbatim. If a provided key conflicts with that identity or its provenance is
missing, return top-level `BLOCKED` with a stable schema-gap key, likely
ownership, preserved evidence, and exact re-entry condition; never re-key or
classify the item.

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
5. Resolve the stable finding key by entry route:
   - for coordinator-managed review, preserve the provided key verbatim through
     classification, correction, registry evidence, and return; never assign,
     normalize, or replace it;
   - for standalone feedback, preserve a provided key, or, only when none was
     supplied, assign one based on the violated requirement and concrete
     reachable behavior rather than a transient line number.
6. Classify the item as exactly one of:
   - **Fix** — verified on the current head, within requested or approved scope,
     compatible with available approved decisions, and authorized for local
     correction. Return exactly two non-overlapping records:
     1. one immutable, path-neutral correction specification containing target
        identity, exact requirement, concrete evidence, exact correction,
        decision source and non-goals, discipline, file responsibilities, and
        verification requirements with expected results; and
     2. one mutable correction record containing only the correction-
        specification identity/reference, preserved finding key and retained
        attempt history, lifecycle phase, attributable partial execution
        evidence, gaps, and exact re-entry condition.
     Assign the complete immutable specification one content identity and make
     the mutable record reference it exactly once. That identity is a correction-
     specification identity, not an `execute-task` canonical context identity.
     Do not add workspace, policy/provenance, capacity/queue, optional plan-task
     context, lifecycle, key/history, or partial evidence to the specification;
     do not construct an `execute-task` canonical context or choose its path-
     specific builder here.
   - **Push back** — incorrect, unsupported, preference-only, stale, not
     reproducible on the current head, or already decided without new evidence.
     Cite the controlling decision or code and test evidence so the coordinator
     can store the target-identity-plus-exact-preserved-key registry entry.
   - **Escalate** — resolution requires a design or public-contract decision,
     material scope expansion, new authority, or, in a coordinator-managed entry,
     the bounded retry stop has been reached.
7. Record current-head evidence and one concrete next action.

## Revalidate and report

Use concise technical language. Cite code, tests, or decisions. Avoid performative agreement, defensive phrasing, and speculative concessions.

Immediately before reporting, capture the same HEAD, target, index, worktree,
tracked-content, and complete bounded untracked path/content evidence again.
For coordinator-managed triage, also re-resolve the manifest's commits, trees,
range, changed-path records and digest, all repository-state digests, clean
assertion, and identity from local Git state. Compare everything with the entry
snapshot. A concurrent commit, missing or changed object/tree/path/state,
identity mismatch, index or worktree mutation, or added, removed, or changed
untracked file makes the target stale: discard provisional classifications and
return `BLOCKED`.

Return exactly one top-level phase result:

- `TRIAGED` only when target and evidence validation succeeded at entry and
  immediately before report and every item is classified exactly `Fix`,
  `Push back`, or `Escalate`;
- `BLOCKED` for a stale or changed target, missing evidence or input, or a
  dependency, permission, external, or runtime failure.

For either result, return the coordinator-frozen identity and exact target
manifest verbatim when they were supplied; never create a replacement identity
or manifest.

For `TRIAGED`, report for each finding:

- current head, immutable target identity, and reviewed range;
- stable finding key and retained attempt history; for `Fix`, satisfy both
  through the sole mutable correction record rather than a parallel copy;
- classification: `Fix`, `Push back`, or `Escalate`;
- requirement and concrete evidence;
- impact and exact next action;
- confirmation that no required target, input, evidence, or runtime gap remains.

For `BLOCKED`, report a stable gap key, likely ownership, the
coordinator-frozen target identity when supplied (or the missing-identity
evidence), available target evidence, every preserved check result, every
unverified gap, and the exact condition required for safe re-entry. Do not report
an item classification, correction specification, mutable correction record,
phase-completion status, or phase advancement.

For a coordinator-managed entry, do not ask for additional approval for a `Fix`
when implementation authorization is already recorded and the correction remains
within approved scope. Return the immutable correction specification and separate
mutable correction record to the coordinator for path-aware routing; never send
them directly to `execute-task` or build a canonical task context here.
Return `Push back` with decision or code evidence so the coordinator can update
the resolved-finding registry using the exact preserved key. Return `Escalate`
with the exact user-owned decision, policy replacement, or new authority
required.

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
