---
name: review
description: Run the approved policy-aware, capacity-aware final review against a freshly verified current head and return evidence-backed findings to the workflow coordinator. Use after verification passes or when the user requests comprehensive review.
---

# Review the verified current head

Review the requested scope, not the entire repository by default.
Remain read-only and keep every dispatched reviewer read-only.

## Validate entry and review policy

Require:

- the current head commit and exact base-to-head range;
- fresh verification with a `PASS` verdict for that same current head and range;
- changed files and primary language;
- approved scope, decision source, and non-goals;
- the approved Design Doc and implementation plan when present;
- repository `AGENTS.md` guidance;
- the complete approved review policy.

Validate that the review policy records:

- mode: `focused`, `adaptive`, or `deep`, with rationale and risk surfaces;
- the per-task gate and its current-head completion evidence;
- final required reviewers;
- final conditional reviewers with exact triggers;
- explicitly skipped perspectives with reasons;
- residual risk;
- configured capacity and queue rules;
- the Acceptance threshold.

Reject stale verification, a missing field, an unknown mode, or a
mode-inconsistent reviewer inventory. Return the gap to the coordinator without
dispatching reviewers. Record the current head before dispatch and require it to
remain unchanged throughout review. Treat an uncommitted change in reviewed scope
as stale current-head verification evidence.

Load `hints/<primary-language>.md` when present. Treat hints as prompts for investigation, not mandatory findings.

## Select final reviewers

Use these standard perspectives when the approved mode and recorded risk make
them applicable:

- `code-reviewer`;
- `test-coverage-reviewer`;
- `design-alignment-reviewer`;
- `scope-reviewer`;
- `code-architect`.

Use these adversarial perspectives when applicable:

- `adversarial-api-reviewer`;
- `adversarial-robustness-reviewer`;
- `adversarial-performance-reviewer`;
- `adversarial-tests-reviewer`.

Apply the approved mode exactly:

- `focused`: require `code-reviewer`; require `test-coverage-reviewer` when
  behavior or tests changed; and run each additional risk reviewer recorded in
  the active review policy as required or when its conditional trigger holds.
- `adaptive`: run only required standard or adversarial reviewers supported by
  recorded risk, plus conditional reviewers whose recorded trigger holds. Do not
  add a perspective for an unrecorded risk.
- `deep`: run all applicable standard and adversarial perspectives. Require the
  policy inventory to classify each perspective as required, conditional with a
  trigger, or skipped with a reason; reject the policy if it skips an applicable
  perspective. Whenever any adversarial reviewer runs, require and run
  `adversarial-integrator`.

Do not run an explicitly skipped perspective. Preserve its reason and residual
risk in the report. Do not silently omit a required reviewer or a conditional
reviewer whose trigger holds.

## Capacity and dispatch

Use `list_agents` before each dispatch wave. The effective capacity is the lower
of the approved configured capacity and the currently observed runtime capacity.
Count the lead. Run independent read-only reviewers concurrently only while slots
are free, and queue every remaining required reviewer without reducing review
scope. Do not return a clean verdict while a required reviewer remains queued; if
the queue cannot complete, report the reviewer as an unverified gap.

When a named profile is selectable, use it. Otherwise provide a complete fallback
prompt containing the profile's role, context, constraints, evidence rules, and
output schema. Reviewers and the integrator do not edit files or spawn
descendants.

## Evidence standard

Every finding must include:

- severity;
- file and line;
- concrete observed behavior or reachable scenario;
- violated requirement or quality consequence;
- specific correction;
- confidence when reachability is uncertain.

Do not manufacture findings. Drop preference-only comments and findings that merely contest an approved decision without new evidence.

## Adversarial integration

After any adversarial reviewers finish, use `adversarial-integrator` or its
complete fallback prompt to deduplicate, verify evidence, normalize severity, and
resolve contradictions. The integrator remains read-only and does not invent new
findings.

## Return to the coordinator

Apply the approved Acceptance threshold, merge duplicates, and report in Japanese:

- mode, recorded risk surfaces, and current exact range;
- fresh verification `PASS` head inspected;
- reviewers run, queued, and skipped with reasons;
- Must Fix and Should Improve findings;
- positive observations only when useful;
- every unverified or policy gap;
- clean review or changes-required verdict.

Read the current head again before reporting. If it changed, mark verification
and review evidence stale and return that gap. Do not classify findings as
`Fix`, `Push back`, or `Escalate`, start triage, edit code, or advance phases from
this skill; return all findings and evidence to the coordinator.
