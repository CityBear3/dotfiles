---
name: review
description: Run read-only evidence-based review of an exact verified Task PR, an approved integration-only target, or a standalone scope using the applicable Review policy.
---

# Review a verified Task PR or integration target

Review the requested scope, not the entire repository by default. Remain
check-only and read-only. Keep every reviewer and integrator read-only. Do not
mutate source or Git state, implement or stage a fix, classify findings for
triage, or advance another workflow phase.

## Resolve the review target

Use one target form:

- a coordinator-managed Task PR with exact planned base, merge base, current
  head, and range;
- a coordinator-managed targeted integration review over an exact composed tree
  and named integration-only obligation;
- an eligible legacy coordinator-managed committed range;
- a standalone committed range;
- a standalone current index/worktree snapshot;
- a standalone bounded explicit fileset.

Record target form, base and head when applicable, current status, exact range or
bounded files, diff, changed files, relevant untracked paths, primary language,
repository guidance, and limitations before dispatch.

## Coordinator-managed entry

Require one exact coordinator target and its authority.

For a Task PR require:

- Task Contract and PR identities, task workspace and branch, planned base ref
  and commit, merge base, current head, exact range, diff, status, and changed
  files;
- fresh coordinator verification `PASS` for that same unchanged head and range;
- the completed current-head Verification Matrix returned by that verifier;
- no unexplained in-scope source state outside the committed range;
- approved scope, non-goals, Review context, and complete Review policy;
- applicable Feature and Task Contract authority, current dependency and shared-
  interface evidence, observed commands, concerns, prior triage decisions, and
  known gaps;
- execution context: the bound Task orchestrator for new-format planned work or
  the root-owned loop for lightweight work, plus configured, observed, and
  effective subagent capacity, current root-granted leaf count, live identities,
  and the ordered selected-reviewer queue.

For a planned targeted integration review require:

- approved Design Doc when applicable, Feature Contract, Task Contract set,
  Implementation Plan, Review context and policy, and their approval state;
- every current accepted Task PR result, both topologies, exact composition, and
  fresh integration verification `PASS`;
- the named integration-only obligation or concrete policy trigger that makes
  this review applicable.

Reject an ordinary full-feature review target for new-format work. Task PR gates
are authoritative; the integration target may examine only the approved
cross-task surface and evidence that triggered it.

For a lightweight targeted integration review require:

- the complete recoverable combined in-memory Feature/Task Contract, original
  request authority and design sources, Review context, and current policy;
- the current exact accepted lightweight Task PR and its base, head, tree,
  range, status, verification, review, and triage evidence;
- fresh integration verification `PASS` for that same head and tree;
- the named integration-only obligation or concrete current policy trigger.

Do not require a Design Doc, contract file, Implementation Plan, Task DAG,
multi-PR topology, or temporary multi-head composition for this authority form.
Review only the named integration surface and do not replay the Task PR gate.

For a lightweight Task PR target, accept the complete combined in-memory
Feature/Task Contract, original request authority and design sources, exact Task
PR target, and fresh verification `PASS`. Require the contract to remain
completely recoverable and no promotion condition or material change to be
unresolved. This Task PR review also satisfies feature review when no
integration-only trigger exists. Do not require an Implementation Plan or
contract file.

For a plan approved and already executing before the contract-centered format,
accept its exact approved plan and referenced design sources in place of Feature
and Task Contract artifacts only when the coordinator supplies unchanged
approval and in-flight evidence, no material ambiguity, and no owner migration
choice. Use its original scope, task specifications, verification and completion
criteria, Review context, and Review policy. Do not manufacture new artifacts.

Resolve workspace, branch, base, head, merge base, range or composed tree, diff,
and changed files directly from Git. Require target state to match supplied
evidence. Return `BLOCKED` without dispatch when it does not resolve, evidence is
stale, in-scope state falls outside the target, or a required input is missing.
Standalone evidence never satisfies a coordinator target.

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
Every standalone review result is labeled `standalone-only` whether the root
dispatches leaves or the lead applies an explicit no-agent fallback.

Record standalone as a root-owned execution context, configured, observed, and
effective global subagent capacity, live identities, the selected-role queue,
and a root-granted target-local count of normally one and at most three
concurrent leaves. It has no Task lease. Selected roles beyond the current grant
remain queued in their original order, and the target never infers authority to
consume every globally free slot.

Review context is an interpretation aid. It must not add to, weaken, or replace
an available Design Doc, Feature Contract, Task Contract, or Implementation
Plan.

## Validate policy and actual risk

When a Review policy exists, require:

- mode: `focused`, `adaptive`, or `deep`, with rationale and risk surfaces;
- the per-task gate; `adaptive` and `deep` require
  independent specification and quality review;
- integration required reviewers with reasons;
- integration conditional reviewers with exact triggers;
- explicitly skipped perspectives with reasons;
- adversarial integration rules;
- findings-only general integration and authority-defect priority rules; treat
  these as current workflow invariants rather than requiring an eligible legacy
  plan to manufacture a new stored policy field;
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

## Select applicable perspectives

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

For a Task PR, apply the approved per-task mode:

- `focused`: require the one combined `code-reviewer` gate, require
  `test-coverage-reviewer` when that Task PR changes behavior or tests, and run
  only additional task perspectives explicitly recorded by policy.
- `adaptive`: require independent specification and quality task gates and any
  triggered task perspective selected for recorded risk.
- `deep`: require independent specification and quality task gates and every
  perspective applicable to that Task PR's artifact and observed risks.

For targeted integration review, run only the required or triggered integration
perspectives named by the approved policy. Do not replay each Task PR reviewer
over the combined tree. Whenever an adversarial perspective runs, require
`adversarial-integrator` for that target.

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

An approved `focused` policy may use a complete lead Task PR review when the
user prohibits agents. `Adaptive` and `deep` independent perspectives cannot be
replaced by sequential lead passes. A no-agent conflict is `Escalate` for
coordinator review or a standalone limitation.

For standalone review, when the user prohibits agents, the lead may execute each
selected read-only perspective and any required adversarial integration
sequentially. Report the result as `standalone-only`. When an available approved
policy requires adaptive or deep independence, disclose that it is not
policy-complete; standalone evidence is never coordinator completion evidence.

Otherwise pass each already-selected perspective and complete reviewer message
to `agent-teams-driven-development`. For new-format planned work, only the bound
Task orchestrator dispatches these reviewer leaves; for lightweight work, the
root dispatches them in its Task loop; for standalone work, the root dispatches
them as direct leaves of the standalone target. Another coordinator-owned target
uses its explicit root-owned context. The adapter calls `list_agents` before
each wave and uses the lower of configured `agents.max_threads` and observed
runtime capacity. The root is excluded from `max_threads`; every Task
orchestrator and leaf is counted.

Outside the source-reviewer wave, a Task loop uses its one root-granted baseline
leaf. When fresh verification passes and the policy selected at least two
independent source reviewers, the Task-loop owner may request a temporary
reviewer-wave expansion. Only the root may grant it, up to three total Task
leaves or the smaller current capacity, and only policy-selected source
reviewers consume it. Queue remaining reviewers in deterministic policy order
without reducing scope, independence, or breadth. Free capacity is not lease
authority, and unavailable expansion only increases latency while the baseline
queue can progress. The expansion must be revoked before findings integration,
triage, or correction and also when review exits early for a priority authority
assessment. An unavailable required reviewer or queue that cannot progress
returns `BLOCKED` with the role, configured/observed/effective capacity,
execution context, grant, queue, gap, and re-entry condition.

Use named profiles when selectable; otherwise provide a complete fallback role
prompt. Reviewers and integrators do not edit files or spawn descendants.

## Give every reviewer artifact-aware evidence

Pass directly to every selected reviewer:

- target kind; exact workspace, branch, planned base, merge base, current head,
  range, composed tree, or bounded standalone files; diff, status, and changed
  files;
- exact authority identity, source path or in-memory identity, and currentness
  evidence for the approved planned contracts, complete lightweight combined
  contract, or exact eligible legacy plan; plus the clauses and integration
  obligations applicable to that perspective;
- approved scope and non-goals;
- the same Review context and Review policy when available;
- the completed current-head Verification Matrix with fresh commands, expected
  observations, and observed results;
- relevant dependency and Task PR outcomes, prior triage decisions, concerns,
  and gaps;
- that reviewer's selected perspective and output expectations.

Do not create another wrapper identity or repeat the evidence in competing
formats. Before dispatch, confirm the current head, range, diff, status, and
changed files are unchanged.

Keep every exact authority source directly available to every reviewer. Eagerly
load complete sources for design-alignment, scope, or another perspective that
owns whole-contract coverage. Other perspectives start with applicable clauses
and inspect additional source sections when their evidence requires it; do not
copy or require an unconditional reread of unrelated unchanged prose.

## Review a corrected head delta-first

For a bounded correction from prior reviewed head `H1` to current head `H2`,
rerun the same complete policy-selected reviewer set. Do not recalculate
perspectives from the delta. Give every reviewer:

- prior head `H1`, current head `H2`, and the full `base..H2` target;
- the `H1..H2` correction delta and exact corrected finding;
- its prior report, the integrated assessment, and triage decision;
- the fresh completed Verification Matrix for `H2`; and
- the same authority, Review context, Review policy, and selected perspective.

Traversal is delta-first: start with the corrected finding and `H1..H2`, then
follow affected callers, tests, interfaces, responsibilities, and obligations.
Prior review evidence is navigation only. Return a new perspective result bound
to `H2` and a fresh verdict for the full `base..H2` target.

Use ordinary full traversal when the correction escapes its bounded
authorization; changes a public or shared interface, responsibility boundary,
schema, error model, concurrency, security, dependency, or test strategy;
changes the base, controlling authority, or Review policy; lacks complete prior
reviewer or triage evidence; exposes another finding; or cannot establish that
previously inspected areas remain unaffected. A missing or stale prior report
disables the delta-first optimization but never removes fresh review. No prior
verdict authorizes `H2`, and no policy-selected reviewer may be skipped.

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
exposes a material user-owned architecture choice or possible authority defect,
preserve it separately as an authority-gap claim for priority general
integration; do not label the proposed mechanism `Must Fix` or `Should Improve`
and do not authorize it as a correction.

## Integrate adversarial review

When required, give `adversarial-integrator` the same exact planned, lightweight,
or eligible legacy authority identity and direct source access, plus Review
context, target evidence, verification, policy, prior triage decisions, and
adversarial reports.
The integrator remains read-only, deduplicates, verifies evidence, resolves
contradictions, and drops unsupported, second-order, and artifact-inapplicable
findings. It does not invent findings or lower the Acceptance threshold.

## Integrate every non-clean review before triage

When every selected reviewer and any required adversarial integration return
clean, do not run a general review integrator. When any source report returns a
finding, the owning Task loop or standalone root must run
`review-integrator` against the exact unchanged target before this skill reports
`FINDINGS`. For planned work the bound Task orchestrator dispatches it; for
lightweight, standalone, and another root-owned coordinator target the root
dispatches it in that explicit context. First revoke any temporary
reviewer-wave expansion, then use `agent-teams-driven-development` under the
baseline one-leaf grant and the same global capacity accounting.

Give `review-integrator`:

- the exact target identity, range or snapshot, starting status, diff, and
  changed files;
- every available source reviewer report and any adversarial integrated report;
- directly accessible approved authority and repository guidance, Review
  context and policy, prior triage, and relevant history needed for origin
  attribution;
- instructions to assess concrete problem validity separately from proposed
  remedy validity, and to record reproduction, exact authority, origin, scope
  owner, proportionality, design sufficiency, and confidence without inventing
  findings or issuing final workflow classifications.

For ordinary implementation findings, wait for every selected reviewer to
complete and run one general integration over the complete reports. When a
source report specifically claims that the Design Doc, Feature Contract, or
Task Contract is defective, pause only reviewers that have not started and
prioritize one integration turn for that authority claim. Do not interrupt
already-running read-only reviewers; preserve their completed reports.

If the integrated authority evidence establishes a missing, contradictory, or
materially ambiguous Design Doc, do not start the paused reviewers or any
correction. Return `FINDINGS` immediately with the integrated authority-defect
evidence and paused queue so `receiving-code-review` can classify `Design
Escalation`. If the claim is rejected or reduced to an implementation issue,
resume the selected reviewer queue and run one final integration over the
complete reports before returning.

Under an explicitly authorized focused no-agent gate, the lead may apply this
same integration contract sequentially to its own findings and must report that
no independent integrator ran. The same rule applies to an explicit no-agent
standalone review. It does not satisfy adaptive or deep independence and may not
claim an independent integration result.

The general review integrator is distinct from `adversarial-integrator`. It
deduplicates and reconciles evidence but does not authorize correction or
classify `Fix`, `Push back`, or `Escalate`. A raw reviewer finding still produces
`FINDINGS` with its integrated assessment even when that assessment recommends
that no current correction is justified; `receiving-code-review` owns the final
classification. If the required integration cannot run, return `BLOCKED`, not
raw findings.

## Report

Merge duplicates and report in Japanese:

- target form and `standalone-only` label when applicable; workspace, branch,
  base, merge base, starting and ending head, exact range, composed tree, or
  bounded fileset;
- starting and ending status, diff scope, and changed files;
- Review context and disclosed standalone assumptions;
- approved mode or `none`, observed risks, and policy reconciliation;
- fresh verification commands and results inspected;
- assigned Feature and Task Contract observations, targeted integration
  alignment, or eligible legacy criteria and original-authority alignment
  inspected;
- reviewers run, queued, and skipped with reasons;
- owning Task-loop context, configured/observed/effective capacity, root grant,
  live identities, and reviewer dispatch order;
- source reviewer, adversarial-integrator, and general review-integrator
  outcomes, including any priority authority assessment and paused queue;
- accepted Must Fix and Should Improve findings;
- separate policy or design gaps requiring coordinator `Escalate`;
- residual risk, limitations, every gap, and exact re-entry condition;
- verdict exactly `CLEAN`, `FINDINGS`, or `BLOCKED`.

Return `CLEAN` only when all required applicable perspectives completed, every
source report is clean, the common Acceptance threshold leaves no finding,
verification is fresh, and the target is unchanged, with no policy or design
gap. A clean review is a valid result and runs no general review integrator.

Return `FINDINGS` only with the required general integrated report or the
explicit no-agent lead integration statement. Keep independent non-blocking
concerns and candidate authority defects separate from current-Task correction
claims so triage can preserve their distinct dispositions.

Read current head and status again before reporting. If either changed, return
`BLOCKED` with preserved reviewer evidence and the stale-state gap. Do not start
triage, edit code, commit, or advance phases. Coordinator review returns evidence
to the coordinator; standalone review reports directly to the requester.
