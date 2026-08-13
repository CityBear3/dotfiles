---
name: review
description: Run a read-only, evidence-based review of a current head using an approved review policy when supplied or applicable perspective selection for standalone requests. Use from the workflow coordinator after verification or standalone when the user requests review.
---

# Review the verified current head

Review the requested scope, not the entire repository by default. Remain
check-only and read-only. Keep every reviewer and integrator read-only. Do not
mutate source or Git state, implement or stage a fix, classify findings for
triage, or advance another workflow phase.

## Resolve the review target

Use one target form:

- a coordinator-managed committed range with exact base, current head, and
  `base..head` range;
- a standalone committed range;
- a standalone current index/worktree snapshot;
- a standalone bounded explicit fileset.

Record target form, base and head when applicable, current status, exact range or
bounded files, diff, changed files, relevant untracked paths, primary language,
repository guidance, and limitations before dispatch.

## Coordinator-managed entry

Require the exact target and one approved authority form.

For new-format work require:

- exact implementation base, current head, full range, diff, current status, and
  changed files;
- fresh coordinator verification `PASS` for that same unchanged head and range;
- no unexplained in-scope source state outside the committed range;
- approved Design Doc when applicable, Feature Contract, complete Task Contract
  set, Implementation Plan, and their approval state;
- complete Feature Contract coverage and integration-only obligations;
- approved scope, non-goals, Review context, and complete Review policy;
- task-review outcomes, observed commands, concerns, prior triage decisions, and
  known gaps.

For a plan approved and already executing before the contract-centered format,
accept its exact approved plan and referenced design sources in place of Feature
and Task Contract artifacts only when the coordinator supplies unchanged
approval and in-flight evidence, no material ambiguity, and no owner migration
choice. Use its original scope, task specifications, verification and completion
criteria, Review context, and Review policy. Do not manufacture new artifacts.

Resolve base, head, range, diff, and changed files directly from Git. Require
repository HEAD and status to match the supplied evidence. Return `BLOCKED`
without dispatch when the range does not resolve, evidence is stale, in-scope
state falls outside the range, or a required input is missing.

Standalone verification or review evidence never satisfies this coordinator
completion gate.

## Standalone read-only entry

Resolve the requested committed range, index/worktree, or bounded fileset through
local read-only investigation. Record available verification, Design Doc,
Feature Contract, Task Contracts, plan, decision, and repository-guidance
evidence. Absent or stale verification is a limitation.

Use an approved Review policy when one is available. Without one, select only
perspectives applicable to observed risk and report the missing policy; do not
present the result as policy-complete coordinator evidence.

Derive the smallest reasonable Review context from the requested artifact,
repository evidence, and available decisions. State:

- artifact and purpose;
- consumers and execution or interpretation model;
- material quality criteria and realistic failures;
- approved or inferred non-problems;
- inapplicable assumptions;
- every material assumption made because approved context was unavailable.

A standalone worktree or fileset review may answer the direct request, but never
substitutes for current-head coordinator review.

Review context is an interpretation aid. It must not add to, weaken, or replace
an available Design Doc, Feature Contract, Task Contract, or Implementation
Plan.

## Validate policy and actual risk

When a Review policy exists, require:

- mode: `focused`, `adaptive`, or `deep`, with rationale and risk surfaces;
- the per-task gate and current outcomes; `adaptive` and `deep` require
  independent specification and quality review;
- final required reviewers with reasons;
- conditional reviewers with exact triggers;
- explicitly skipped perspectives with reasons;
- adversarial integration rules;
- residual risk, capacity, deterministic queue order, and Acceptance.

Compare the actual artifact, diff, behavior, tests, public seams, responsibilities,
and failure paths with the recorded risks and skips. A material risk absent from
an approved policy is a policy gap, not permission to add or omit a reviewer.
For coordinator review, return `BLOCKED` so the coordinator can `Escalate` for a
replacement policy. For standalone review, report the limitation.

Record the current head before review and require it to remain unchanged. Treat
an uncommitted in-scope change as stale coordinator verification.

Load `hints/<primary-language>.md` when present. Treat hints as investigation
prompts, not mandatory findings.

## Select applicable final perspectives

Standard perspectives:

- `code-reviewer`;
- `test-coverage-reviewer`;
- `design-alignment-reviewer`;
- `scope-reviewer`;
- `code-architect`.

Adversarial perspectives:

- `adversarial-api-reviewer`;
- `adversarial-robustness-reviewer`;
- `adversarial-performance-reviewer`;
- `adversarial-tests-reviewer`.

Apply the approved mode:

- `focused`: require `code-reviewer`, require `test-coverage-reviewer` when
  behavior or tests changed, and run only additional recorded risk perspectives.
- `adaptive`: run required and triggered perspectives selected for recorded
  risk.
- `deep`: run every perspective applicable to the artifact and observed risks.
  Do not run an inapplicable profile merely to maximize reviewer count.

For every mode, preserve skipped perspectives and their reasons. Reject a `deep`
policy that skips an applicable perspective. Whenever any adversarial
perspective runs, require `adversarial-integrator`.

Without an approved policy, select the same perspectives by observed
applicability: general review always; test coverage for behavior or test changes;
design alignment when an approved Design Doc or Feature Contract exists; scope
review for a plan or narrow migration; architecture for material responsibility
changes; adversarial profiles only for their corresponding concrete risk. Record
every run and skip with reasons.

## Preserve independence and capacity

An approved `focused` policy may use a complete lead final-review pass when the
user prohibits agents. `Adaptive` and `deep` independent perspectives cannot be
replaced by sequential lead passes. A no-agent conflict is `Escalate` for
coordinator review or a standalone limitation.

For standalone review without an approved policy, when the user prohibits
agents, the lead may execute each selected read-only perspective and any required
adversarial integration sequentially. Report the result as `standalone-only`; it
is never approved-policy completion or coordinator completion evidence.

Otherwise call `list_agents` before each dispatch wave. Use the lower of approved
configured capacity and observed capacity, count the lead, and queue remaining
required reviewers in deterministic policy order. Do not reduce scope,
independence, or applicable breadth. An unavailable required reviewer returns
`BLOCKED` with the role, observed capacity, gap, and re-entry condition.

Use named profiles when selectable; otherwise provide a complete fallback role
prompt. Reviewers and integrators do not edit files or spawn descendants.

## Give every reviewer artifact-aware evidence

Pass directly to every final reviewer:

- exact base, current head, range or bounded standalone files, diff, status, and
  changed files;
- approved Design Doc when applicable, Feature Contract, complete Task Contract
  set, Implementation Plan, Feature Contract coverage, and integration-only
  obligations, or the exact eligible legacy plan and referenced design sources;
- approved scope and non-goals;
- the same Review context and Review policy when available;
- fresh verification commands and observed results;
- relevant task-review outcomes, prior triage decisions, concerns, and gaps;
- that reviewer's selected perspective and output expectations.

Do not create another wrapper identity or repeat the evidence in competing
formats. Before dispatch, confirm the current head, range, diff, status, and
changed files are unchanged.

## Apply the common Acceptance threshold

Every finding must include severity `Must Fix` or `Should Improve`, file and
line, concrete observed or reachable behavior, violated requirement or quality
consequence, evidence, impact, proportionate correction, and confidence.

Keep a finding only when it:

- applies to the artifact and consumer model in the Review context;
- identifies a concrete reachable behavior or approved-contract violation;
- cites evidence and a material consequence;
- proposes a proportionate correction.

`Should Improve` requires a concrete maintainability consequence or measurable
repeated cost. Drop preference-only, speculative, second-order,
artifact-inapplicable, optional-polish, generic-best-practice, and unsupported
findings.

An approved non-problem may be revisited only with materially new evidence of a
concrete reachable failure or contract violation. A rephrasing, changed line
number, or imagined future consumer is not new evidence. Apply the same rule to
prior `Push back` decisions.

A suggestion to add a state machine, schema, identity mechanism, or another
architectural system is not a `Fix` without a proven in-scope violation and
proportionate need. Drop it when it is unsupported optional design. When it
exposes a material user-owned architecture choice, return `BLOCKED` with a
design gap so the coordinator can `Escalate`; do not label it `Must Fix` or
`Should Improve`.

## Integrate adversarial review

When required, give `adversarial-integrator` the same applicable new-format
contract artifacts or exact eligible legacy authority, plus Review context,
target evidence, verification, policy, prior triage decisions, and adversarial
reports.
The integrator remains read-only, deduplicates, verifies evidence, resolves
contradictions, and drops unsupported, second-order, and artifact-inapplicable
findings. It does not invent findings or lower the Acceptance threshold.

## Report

Merge duplicates and report in Japanese:

- target form; base, starting and ending head, exact range or bounded fileset;
- starting and ending status, diff scope, and changed files;
- Review context and disclosed standalone assumptions;
- approved mode or `none`, observed risks, and policy reconciliation;
- fresh verification commands and results inspected;
- Feature Contract observations and design, task, and integration alignment, or
  eligible legacy completion criteria and original-authority alignment,
  inspected;
- reviewers run, queued, and skipped with reasons;
- reviewer and integrator outcomes;
- accepted Must Fix and Should Improve findings;
- separate policy or design gaps requiring coordinator `Escalate`;
- residual risk, limitations, every gap, and exact re-entry condition;
- verdict exactly `CLEAN`, `FINDINGS`, or `BLOCKED`.

Return `CLEAN` only when all required applicable perspectives completed, the
common Acceptance threshold leaves no finding, verification is fresh, and the
target is unchanged, with no policy or design gap. A clean review is a valid
result.

Read current head and status again before reporting. If either changed, return
`BLOCKED` with preserved reviewer evidence and the stale-state gap. Do not start
triage, edit code, commit, or advance phases. Coordinator review returns evidence
to the coordinator; standalone review reports directly to the requester.
