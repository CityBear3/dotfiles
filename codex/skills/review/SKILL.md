---
name: review
description: Run a read-only, evidence-based review of a current head using an approved review policy when supplied or applicable perspective selection for standalone requests. Use from the workflow coordinator after verification or standalone when the user requests review.
---

# Review the verified current head

Review the requested scope, not the entire repository by default.
Remain read-only and keep every dispatched reviewer read-only.

## Coordinator-managed entry

When the workflow coordinator invokes this skill, require:

- the current head commit and exact base-to-head range;
- fresh verification with a `PASS` verdict for that same current head and range;
- changed files and primary language;
- approved scope, decision source, and non-goals;
- the approved Design Doc and implementation plan when present;
- repository `AGENTS.md` guidance;
- the complete approved review policy.

## Standalone read-only entry

When the user invokes review outside the coordinator, resolve through local
read-only investigation:

- the requested scope;
- the current head and exact base-to-head range;
- changed files and primary language;
- applicable repository guidance;
- available verification evidence;
- available design, decision, and plan evidence.

Record absent or stale verification as a limitation. Use an approved review
policy when one is available. Without an approved review policy, do not invent
one; select evidence-based applicable perspectives under the standalone contract
below and report the missing policy.

## Validate an available review policy

For coordinator-managed review, and for standalone review with an approved policy,
validate that the review policy records:

- mode: `focused`, `adaptive`, or `deep`, with rationale and risk surfaces;
- the per-task gate and its current-head completion evidence;
- final required reviewers;
- final conditional reviewers with exact triggers;
- explicitly skipped perspectives with reasons;
- residual risk;
- configured capacity and queue rules;
- the Acceptance threshold.

Reject stale verification, a missing field, an unknown mode, or a
mode-inconsistent reviewer inventory. In coordinator-managed review, return the
gap to the coordinator without dispatching reviewers. In standalone review,
report the policy limitation and do not claim a policy-complete verdict.

For direct/no-agent per-task evidence, accept both approved sequential
specification and quality passes as mode-consistent current-head completion
evidence when the user explicitly prohibited agents and the approved policy does
not make agent-level independence non-waivable. Preserve the recorded lack of
agent-level independence as an accepted residual limitation and risk, not an
unresolved finding or policy gap.

If the approved policy explicitly makes agent-level independence non-waivable,
reject direct/no-agent evidence. Do not waive the requirement or dispatch agents
contrary to the user instruction. Return the conflict to the coordinator with the
exact policy or user decision required for `Escalate`; in standalone review,
report that conflict and do not claim a policy-complete verdict.

Record the current head before review and require it to remain unchanged. Treat
an uncommitted change in reviewed scope as stale current-head verification
evidence.

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

## Standalone selection without an approved policy

Select only perspectives applicable to the observed scope:

- run `code-reviewer` for correctness and maintainability;
- run `test-coverage-reviewer` when behavior or tests changed;
- run `design-alignment-reviewer` when an approved Design Doc is available;
- run `scope-reviewer` for an implementation plan or narrow migration;
- run `code-architect` for material responsibility or dependency changes;
- run adversarial API, robustness, performance, or tests perspectives only when
  their corresponding risk surface is present.

Integrate adversarial results whenever any adversarial perspective runs. Report
that no approved review policy exists, every perspective run or skipped with its
reason, and the resulting coverage limitations. Do not present this selection as
an approved policy.

## Capacity and dispatch

When the user prohibits agents, do not call `list_agents` and do not dispatch
subagents. Have the lead execute every approved or selected perspective as a
distinct sequential read-only pass using the named profile's complete role
contract or its complete fallback prompt, and evaluate each pass separately.
Preserve the complete required scope without reducing it. When any adversarial
pass runs, perform adversarial integration as a separate lead pass afterward.
When every required or triggered perspective and any required adversarial
integration run and return clean results, final perspective coverage is satisfied.
Report the lack of agent-level independence as an accepted residual limitation,
evidence, and risk. It does not prevent a clean verdict and is not an unresolved
finding or policy gap unless the approved policy explicitly makes independence
non-waivable.

Otherwise, use `list_agents` before each dispatch wave. With an approved policy,
the effective capacity is the lower of its configured capacity and the currently
observed runtime capacity. Without an approved policy, use observed runtime
capacity and report the absence of an approved configured limit. Count the lead.
Run independent read-only reviewers concurrently only while slots are free, and
queue every remaining required reviewer without reducing review scope. Do not
return a clean verdict while a required reviewer remains queued; if the queue
cannot complete, report the reviewer as an unverified gap.

When dispatching and a named profile is selectable, use it. Otherwise provide a
complete fallback prompt containing the profile's role, context, constraints,
evidence rules, and output schema. Reviewers and the integrator do not edit files
or spawn descendants.

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

## Report

Apply the approved Acceptance threshold when a policy exists. Otherwise apply the
evidence standard above and report the missing policy as a limitation. Merge
duplicates and report in Japanese:

- approved mode or `none`, recorded or observed risk surfaces, and current exact
  range;
- verification evidence inspected and whether it is fresh;
- reviewers run, queued, and skipped with reasons;
- Must Fix and Should Improve findings;
- positive observations only when useful;
- accepted direct/no-agent independence limitations and residual risk, separately
  from gaps;
- every unverified or policy gap;
- clean review or changes-required verdict.

Read the current head again before reporting. If it changed, mark verification
and review evidence stale and return that gap. Do not classify findings as
`Fix`, `Push back`, or `Escalate`, start triage, edit code, or advance phases from
this skill.

For coordinator-managed review, return all findings and evidence to the
coordinator. For standalone review, report them directly to the requester; do not
automatically fix findings or advance another phase.
