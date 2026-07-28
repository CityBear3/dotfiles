---
name: review
description: Run a read-only, evidence-based review of a current head using an approved review policy when supplied or applicable perspective selection for standalone requests. Use from the workflow coordinator after verification or standalone when the user requests review.
---

# Review the verified current head

Review the requested scope, not the entire repository by default.
Remain check-only and read-only. Keep every dispatched reviewer and integrator
read-only. Do not mutate the index, tracked files, or in-scope source; create a
commit; implement or stage a fix; classify findings for triage; or advance
another workflow phase.

## Use one immutable review target

Use exactly one target form:

- an exact committed range identified by base commit, head commit, range, and
  diff;
- a captured current index/worktree snapshot identified by HEAD, index state,
  staged diff, worktree diff, and path/content identities for in-scope untracked
  files;
- a bounded explicit fileset identified by path inventory and immutable content
  identities for every reviewed file.

For coordinator-managed review, receive the exact content-bound identity that
`verify` resolved plus the coordinator target request verbatim. Validate the
identity-bound Git objects, range and diff content, changed files, current HEAD,
index, worktree, and in-scope untracked evidence before dispatch and immediately
before reporting. Never rename, regenerate, or substitute that identity. For
standalone review, create one separate standalone-only stable identity from every
immutable field required by the selected target schema.

Record the target form and identity, HEAD, index/worktree and in-scope untracked
evidence, unrelated dirty state, changed files, primary language, repository
guidance, and limitations before dispatch.

## Coordinator-managed entry

When the workflow coordinator invokes this skill, require:

- the coordinator-frozen content-bound immutable target identity and exact
  coordinator target request containing base, current HEAD, full range, diff
  contents, changed files, and strict repository-state evidence;
- fresh coordinator-managed verification with a `PASS` verdict for that same
  target identity, current HEAD, and full range;
- no in-scope index, worktree, or untracked source state outside the committed
  target;
- changed files and primary language;
- approved scope, decision source, and non-goals;
- the approved Design Doc and implementation plan when present;
- repository `AGENTS.md` guidance;
- the complete approved Review policy and provenance;
- the coordinator-owned resolved-finding registry for this target identity, or
  one immutable resolvable reference to it, including an explicitly empty
  registry.

Never accept a standalone-only verification or review target as coordinator
completion evidence.

## Standalone read-only entry

When the user invokes review outside the coordinator, resolve the explicitly
requested target as one of the three target forms above through local read-only
investigation:

- the requested scope;
- changed files and primary language;
- applicable repository guidance;
- available verification evidence;
- available design, decision, and plan evidence.

Record absent or stale verification as a limitation. Use an approved review
policy when one is available. Without an approved review policy, do not invent
one; select evidence-based applicable perspectives under the standalone contract
below and report the missing policy.

Label an index/worktree snapshot or explicit fileset result `standalone-only`.
It can answer the requested review question but cannot satisfy the coordinator's
immutable current-HEAD completion gate.

## Validate an available review policy

For coordinator-managed review, and for standalone review with an approved policy,
validate that the review policy records:

- mode: `focused`, `adaptive`, or `deep`, with rationale and risk surfaces;
- the per-task gate and its current-head completion evidence;
- final required reviewers with reasons;
- final conditional reviewers with exact triggers;
- explicitly skipped perspectives with reasons;
- `adversarial-integrator` as required, conditional with an exact trigger, or
  skipped with a reason;
- residual risk;
- configured capacity and queue rules;
- the Acceptance threshold.

Reject stale verification, a missing field, an unknown mode, or a
mode-inconsistent reviewer inventory. In coordinator-managed review, return the
gap as `BLOCKED` without dispatching reviewers. In standalone review, report the
policy limitation and do not claim a policy-complete verdict.

Record the current head before review and require it to remain unchanged. Treat
an uncommitted change in reviewed scope as stale current-head verification
evidence.

Load `hints/<primary-language>.md` when present. Treat hints as prompts for investigation, not mandatory findings.

## Reconcile actual risk before dispatch

When an approved policy exists, compare the immutable target's actual diff,
files, behavior, tests, public seams, state transitions, and failure paths with
the policy's recorded risk surfaces and complete required, conditional, and
skipped reviewer inventory.

When a material observed risk is absent from the approved policy, return a named
policy gap before dispatch. Do not add a reviewer, ignore the risk, reinterpret a
skip, or mutate the policy. A coordinator-managed policy gap requires the
coordinator to `Escalate` for a complete user-approved replacement policy. In
standalone review with a policy, report the gap and do not claim policy-complete
coverage. Record non-material evidence and limitations without changing policy.

For standalone review without a policy, build an observed risk inventory from
that same target evidence and reconcile it with the selected standalone
perspectives. Report the missing policy as a limitation, not as permission to
omit an applicable perspective or as an approved-policy gap.

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
  perspective.

For coordinator-managed `focused` or `adaptive`, run
`adversarial-integrator` only when the complete approved required inventory
contains it or its approved conditional trigger holds. For coordinator-managed
`deep`, require and run it whenever any adversarial reviewer runs; a skipped
integrator in that case is a mode-inconsistent policy gap. Do not run an
explicitly skipped integrator.

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

Whenever any adversarial perspective runs, require adversarial integration.
Report that no approved review policy exists, every perspective run or skipped
with its reason, and the resulting coverage limitations. Do not present this
selection as an approved policy.

## Capacity and dispatch

When the user prohibits agents, do not call `list_agents` or dispatch subagents.
For an approved `focused` policy, the lead may execute its complete approved
combined final-review pass with the same target, evidence, and output schema.
For `adaptive` or `deep`, required independent perspectives cannot be replaced by
sequential lead passes. Return the exact no-agent/policy conflict for coordinator
`Escalate`, or as a standalone policy limitation, and do not claim a clean
policy-complete verdict. Never count waived independence toward completion.
For standalone review without a policy, the lead may execute each selected
read-only perspective and any required adversarial integration, but the result
remains standalone-only and is never approved-policy completion evidence.

Otherwise, use `list_agents` before each dispatch wave. With an approved policy,
the effective capacity is the lower of its configured capacity and the currently
observed runtime capacity. Without an approved policy, use observed runtime
capacity and report the absence of an approved configured limit. Count the lead.
Run independent read-only reviewers concurrently only while slots are free, and
queue every remaining required reviewer without reducing review scope. Do not
return a clean verdict while a required reviewer remains queued; if the queue
cannot complete, return `BLOCKED` with a stable availability-gap key and exact
re-entry condition.

When dispatching and a named profile is selectable, use it. Otherwise provide a
complete fallback prompt containing the profile's role, context, constraints,
evidence rules, and output schema. Reviewers and the integrator do not edit files
or spawn descendants. For coordinator-managed review, pass the exact target
identity and the complete resolved-finding registry or its one immutable
reference to every reviewer and integrator dispatch.

## Require one final finding schema

Every named-profile or fallback dispatch message, including standard,
adversarial, and integrator messages, must require every returned finding to
include:

- one stable finding key based on the violated requirement and reachable
  behavior, not a transient line number;
- final severity exactly `Must Fix` or `Should Improve`;
- file and line as `file:line`;
- concrete observed behavior or reachable scenario;
- violated requirement or quality consequence;
- concrete evidence;
- impact;
- specific correction;
- confidence.

A native or suggested role label may be retained as evidence only when that
finding also supplies an accepted final severity explicitly. Missing or unknown
final severity is a schema gap: request schema-compliant re-output from the same
reviewer or integrator before Acceptance, deduplication, triage, or a clean
verdict. Never infer, drop, promote, or normalize the missing label. If
schema-compliant re-output cannot be obtained, return `BLOCKED` with the role,
target identity, stable schema-gap key, and exact re-entry condition.

Do not manufacture findings. Drop preference-only comments and findings that merely contest an approved decision without new evidence.

## Apply the resolved-finding registry

For coordinator-managed review, apply the registry independently at reviewer
output and again during final synthesis. When the same target identity and stable
finding key already has a `Push back` entry, the reviewer or integrator must not
re-emit it and synthesis must drop it unless the output cites materially new
code, test, Design, plan, or approved-decision evidence. A changed line number,
rephrasing, confidence, or repeated assertion is not new evidence.

When materially new evidence exists, permit re-evaluation only when the finding
identifies the registry entry and cites the exact evidence delta. Preserve that
delta through integration and the final report. Do not use the registry to drop
a different key, suppress new evidence, reset bounded `Fix` attempts, or convert
an earlier `FINDINGS` result to `CLEAN`. A clean result after pushback requires
this complete fresh review and its normal dispatch, synthesis, and current-state
checks.

## Adversarial integration

When the applicable mode rule above requires integration, use
`adversarial-integrator` or its complete fallback prompt to deduplicate, verify
evidence, and resolve contradictions. Require the same final finding schema.
The integrator remains read-only, does not invent findings, and does not infer or
normalize a missing final severity. For coordinator-managed review, it also
receives and applies the exact same resolved-finding registry or reference.

## Report

Apply the approved Acceptance threshold when a policy exists. Otherwise apply the
final finding schema above and report the missing policy as a limitation. Merge
duplicates and report in Japanese:

- target form and identity; for coordinator-managed review, the exact target
  request received verbatim from the coordinator; starting and ending HEAD;
  exact range, snapshot, or fileset; index/worktree and in-scope untracked
  path/content evidence; and unrelated dirty state;
- approved mode or `none`, recorded or observed risk surfaces, and actual-risk
  reconciliation;
- verification evidence inspected and whether it is fresh;
- inspection or dispatch commands and evidence, checks not run, standalone-only
  status, and limitations;
- reviewers run, queued, and skipped with reasons;
- resolved-finding registry identity/reference, every registry key supplied to
  dispatches, suppressed keys, and any materially new evidence delta that
  permitted re-evaluation;
- Must Fix and Should Improve findings;
- positive observations only when useful;
- residual risk separately from gaps;
- every unverified or policy gap;
- verdict exactly `CLEAN`, `FINDINGS`, or `BLOCKED`.

For `BLOCKED`, include a stable gap key, likely ownership, immutable target
identity, preserved reviewer/queue evidence, and the exact condition required for
safe re-entry. For `FINDINGS`, preserve a stable key for each accepted finding.
Do not advance a `BLOCKED` result or treat missing schema output as clean.

Read the current head again before reporting. If it changed, mark verification
and review evidence stale and return `BLOCKED`. Recheck index/worktree and
in-scope source evidence and return `BLOCKED` for an unattributed mutation. Do
not regenerate or rename a coordinator-frozen target identity, classify findings
as `Fix`, `Push back`, or `Escalate`, start triage, edit code, commit, or advance
phases from this skill.

For coordinator-managed review, return all findings and evidence to the
coordinator. For standalone review, report them directly to the requester; do not
automatically fix findings or advance another phase.
