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

For standalone review without an approved policy, when the user prohibits
agents, the lead may execute each selected read-only perspective and any required
adversarial integration sequentially. Report the result as `standalone-only`; it
is never approved-policy completion or coordinator completion evidence.

Otherwise pass each already-selected perspective and complete reviewer message
to `agent-teams-driven-development`. For new-format planned work, only the bound
Task orchestrator dispatches these reviewer leaves; for lightweight work, the
root dispatches them directly. The adapter calls `list_agents` before each wave
and uses the lower of configured `agents.max_threads` and observed runtime
capacity.
The root is excluded from `max_threads`; every Task orchestrator and leaf is
counted. Never exceed the Task loop's root-granted lease of at most three leaves
or its smaller current grant. Queue remaining required reviewers in
deterministic policy order without reducing scope, independence, or applicable
breadth. A Task orchestrator may request capacity but may not infer or expand
its own lease. An unavailable required reviewer returns `BLOCKED` with the role,
configured/observed/effective capacity, grant, queue, gap, and re-entry
condition.

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
- fresh verification commands and observed results;
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

When required, give `adversarial-integrator` the same exact planned, lightweight,
or eligible legacy authority identity and direct source access, plus Review
context, target evidence, verification, policy, prior triage decisions, and
adversarial reports.
The integrator remains read-only, deduplicates, verifies evidence, resolves
contradictions, and drops unsupported, second-order, and artifact-inapplicable
findings. It does not invent findings or lower the Acceptance threshold.

## Report

Merge duplicates and report in Japanese:

- target form; workspace, branch, base, merge base, starting and ending head,
  exact range, composed tree, or bounded fileset;
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
