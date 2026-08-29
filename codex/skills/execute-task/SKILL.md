---
name: execute-task
description: Produce or accept one planned or eligible legacy Task PR with one writer, an exact base-to-head range, fresh verification, policy-selected review, and bounded correction.
---

# Execute one task

Own candidate implementation and authoritative acceptance of exactly one
planned and eligible legacy Task PR or bounded correction. Do not select a
workflow path, schedule dependencies or PR topology, publish, merge, or choose
branch disposition from this skill.

## Require one task handoff

Before implementation, require one concise plain-language handoff containing
the new planned contract form, an explicitly eligible legacy form, or an
approved promotion-reconciliation form.

For the new form, require the common core and planned variant. Reject a
lightweight handoff instead of interpreting its root-owned authority here.

The common core contains:

- exact authority identity and currentness evidence, assigned obligations,
  protected constraints, non-goals, and delegated local decisions;
- the separate Review context and complete active Review policy;
- the required discipline and applicable repository guidance;
- approved task workspace, branch, Task PR identity, base ref and exact base
  commit, current head, merge base, exact base-to-head range, inspected diff,
  and starting Git status including index, worktree, and relevant untracked
  state;
- responsibility and ownership boundaries, verification routes and observable
  obligations, and the responsibility-scoped commit intent with its fixed
  message or approved writer message-selection authority;
- attributable commits, prior verification and review, concerns, gaps, and
  re-entry evidence when applicable;
- configured, observed, and effective subagent capacity, currently live
  identities, and every selected or queued role;
- contractually significant files, interfaces, signatures, ordering, and exact
  commands only when the authority fixes them.

The planned variant adds:

- exact approved Feature Contract identity, path, approval and currentness
  evidence, and clauses assigned to this task;
- the exact Task Contract, applicable shared interfaces, and adjacent-task
  obligations;
- coordination directory, Task worktree, Herdr workspace and pane identities,
  branch, Task PR, planned base, starting head, and candidate or authoritative
  mode;
- the exact planned `search-cache.md` path, a current matching entry or miss,
  its source identity and invalidation conditions, and the Feature-lead-only
  writer boundary;
- the one bound `task-orchestrator` identity and its current root-granted Task
  leaf lease;
- for re-entry, prior candidate, acceptance, interruption, correction, or stale
  evidence, including authorized final-base materialization or restack evidence
  when applicable.

For a plan already executing before the contract-centered format, accept its
approved task specification and referenced design sources as the authority only
when the coordinator establishes unchanged approval, in-flight status, no
material ambiguity, and no owner migration choice. Require its available scope,
non-goals, discipline, responsibility or file boundaries, commit intent,
verification, Review context and policy, workspace, and exact task base. Do not
manufacture Feature or Task Contract artifacts or silently fill a material gap.

For promotion reconciliation, require the current approved Feature and Task
Contracts, the dedicated reconciliation Task Contract, original lightweight
base, promotion head, execution-starting head, exact unaccepted range and
commits, attributable approved artifact state, complete change-to-contract
attribution, prior writer and gate evidence, Review context and policy,
verification obligations, and gaps. This form accepts attributable preserved
work under current authority; it does not authorize new semantics or history
rewriting.

The Review context describes the artifact, purpose, consumers, interpretation or
execution model, material quality criteria and realistic failures, approved
non-problems, and inapplicable assumptions. The Review policy records mode,
rationale, risk surfaces, per-task gate, integration required and conditional
perspectives, skips with reasons, findings-only general integration and
authority-defect priority, residual risk, capacity and queue rules, and the
common Acceptance threshold.

Reject missing, stale, contradictory, or mode-inconsistent input. Return the
named gap to the invoking skill; do not infer a decision, expand scope, duplicate
the handoff in a new wrapper, or weaken evidence.

Stop with `Escalate` when implementation needs a new or changed goal, scope,
responsibility owner, public or shared interface semantic, invariant, material
failure behavior, compatibility promise, verification obligation, Review
policy, or authority. A newly discovered private file or local interface inside
the approved responsibility is not a deviation by itself; an unexpected owner
or shared seam is.

Require the task branch, planned base, current head, and workspace to resolve.
For fresh implementation, require the supplied starting commit to equal the
workspace HEAD. For authoritative acceptance, require the exact planned PR base
commit to be an ancestor of the reviewed head and resolve the merge-base-derived
range and diff directly from Git. For an approved promotion reconciliation only,
the original lightweight base may precede the starting head exactly by the
supplied attributable envelope. Recheck ancestry, base, branch, range, and
status after commits and before acceptance. On failure, preserve state and
return `BLOCKED`; never rewrite history to manufacture the planned topology.

## Run in the owning Task-loop context

For new-format planned work, this skill runs inside the non-writing Task
orchestrator bound to the supplied Task Contract. Only that orchestrator
dispatches this Task's leaves. Eligible legacy work retains its exact approved
invoking context. Reject a new-format planned handoff delivered to an unbound
identity and reject root-owned lightweight authority.

Treat `agents.max_threads` as subagent capacity across the complete descendant
tree, excluding the root and counting every Task orchestrator and leaf. Use the
lower configured or currently observed capacity. The root alone grants leaf
capacity. Start this loop with one baseline leaf and use it serially for the
writer, verifier, findings integration, triage, and correction. Only after a
fresh verifier `PASS` and selection of at least two independent source reviewers
may the owner request a temporary reviewer-wave expansion. The root may grant at
most three total Task leaves or the smaller current capacity; only the selected
source reviewers use that expansion. Revoke it before findings integration,
triage, or correction. Free capacity is availability, not authority. Queue
already-selected roles in deterministic policy order without dropping,
substituting, reordering, or weakening them. Do not begin a planned orchestrator
turn unless its grant includes the baseline leaf.

## Choose one writer

Keep exactly one writer: the lead when direct execution is authorized, otherwise
one `implementer`. Promotion reconciliation and authoritative re-entry of an
attributable candidate begin with no active writer when the preserved or
restacked range needs only fresh verification and review; select one writer only
for an authorized bounded correction. Resolve the role before loading its
prompt: use the named profile when available, or
[implementer-prompt.md](../agent-teams-driven-development/implementer-prompt.md)
as its fallback. Keep the complete Task-loop handoff and evidence at the owning
coordinator. Construct one compact writer role message from the complete
Task-loop handoff containing only the owned responsibility, applicable
authority clauses and preserved boundaries, discipline, candidate target,
commit intent, focused writer-side obligations, exact workspace/base/head,
one-writer boundary, applicable planned cache path and current hit, and
escalation conditions. Keep the exact authority sources directly available.
Omit Review context and Review policy, review
scheduling, completed gate evidence, capacity, and queue state from the writer
message. Pass only the selected role and writer role message to
`agent-teams-driven-development`.

Every implementer, verifier, reviewer, adversarial integrator, and review
integrator dispatched by this loop is a leaf and must not spawn descendants. A
capacity lease changes
only scheduling concurrency; it grants no source, Git, policy, publication, or
cleanup authority. The Task orchestrator remains non-writing, and the
implementer remains the sole Task source writer.

For new-format planned work, look up a current matching cache entry before new discovery.
The cache never replaces fresh Git, authority, verification, or review evidence.
Every consumer must return attributable cache candidates to the Feature lead;
only that lead edits `search-cache.md`.

Independent initial authority reads, repository searches, relevant file reads,
and Git inspection may run in one bounded programmatic batch only when their
results remain separately attributable. End the batch and stop before a
result-dependent judgment, approval, escalation, semantic diagnosis, edit, or
dependent validation. Pass the declared `test-driven-development` applicability
decision and reason unchanged. Require the writer to apply that Skill when TDD
is applicable, use the supplied baseline and validation when it is not, and
return `BLOCKED` for `required but blocked`. Preserve the returned discipline
evidence for later gates. For other declared disciplines, preserve their
baseline and validation. Preserve unrelated changes.

Inside the applicable new-format or legacy authority, let the writer choose
private files, helpers, local types and interfaces, algorithms, edit order,
focused non-destructive checks, and only a local type or build check needed for
a coherent candidate when those choices are delegated or unspecified. Require
every actual choice and changed file to remain within the approved
responsibility and be reported with evidence. Do not make the writer duplicate
the authoritative full format, build, lint, package/workspace/full-test, smoke,
or integration suite unless exact authority fixes one as a pre-commit command.

Require the writer to report:

- `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED`, or `NEEDS_CONTEXT`;
- changed files and implemented behavior;
- discipline decision and evidence plus attributable cache candidates;
- every command, why it was required or selected, and its expected and observed
  result;
- pre-commit diff inspection and self-review;
- when complete, the commit and new head;
- concerns and every known gap.

Interpret the report as follows:

- `DONE` advances only after the requested commit, current-state checks,
  verification, report, and ownership evidence all agree. It is not task
  acceptance by itself.
- `DONE_WITH_CONCERNS` requires classification of every concern as an authorized
  correction, operational `BLOCKED` gap, or user-owned `Escalate` decision.
- `BLOCKED` preserves the operational gap and observed state.
- `NEEDS_CONTEXT` is `BLOCKED` when the missing input is safely discoverable
  within current authority, otherwise `Escalate`.

After any incomplete response, partial edit, partial commit, interruption, or
lost response, do not start or replace a writer until the prior writer is
confirmed inactive.

## Produce current Task PR evidence

For candidate or fresh authoritative implementation:

1. Record the task and PR identities, workspace, branch, starting commit,
   planned base ref and commit, current head, status, owning Task-loop context,
   current capacity evidence, grant, and selected-role queue.
2. Implement only the declared scope with the selected discipline.
3. Run every contractually required exact command, select applicable standard
   and focused checks, and record all observed results.
4. Inspect the working-tree diff and applicable authority coverage, including
   unrelated state and actual changed files.
5. Correct concrete in-scope failures while contract meaning remains unchanged.
6. Create only the declared responsibility-scoped task commit.
7. Record the new current head and inspect the attributable commit range.

Writer completion and its commit are Candidate evidence even in authoritative
mode. They never establish Task acceptance. After the exact committed head,
merge base, range, changed files, and status resolve, the Task-loop owner builds
one in-memory current-head Verification Matrix. For every applicable observable
obligation record:

- the obligation and authority;
- one bounded command or check;
- its expected observation; and
- whether a non-match is `FAIL` or `BLOCKED`.

One observation may support multiple explicitly mapped obligations, but no
obligation may disappear because commands are grouped. The matrix is the
verifier input and completed report spine, not a persistent schema. Invalidate
and rebuild it when the head, range, controlling authority, or material
verification route changes. A missing, incomplete, contradictory, or stale
matrix is `BLOCKED`; do not let another role fill a row by inference.

When the handoff is candidate mode because the final PR base is unavailable,
stop after recording preliminary checks, the candidate commit and head, changed
files, and gaps. Return `Candidate`; do not run the authoritative policy gate,
release a dependent, or describe the preliminary range as accepted.

For authoritative re-entry of an attributable candidate, replace implementation
steps 1–7 with validation of the supplied candidate result, the authorized
final-base materialization or restack, the current commits and changed files,
and the resulting exact PR range. Existing attributable restacked commits
satisfy the task commit requirement when no correction is needed; do not select
a writer or create another commit. Then run authoritative steps 8–11 with fresh
verification and review. Return `BLOCKED` for an unattributable commit, conflict,
or unexplained range rather than reimplementing the task.

For authoritative mode, continue:

8. Resolve the current planned base commit, merge base, exact base-to-head range,
   diff, and status.
9. Run fresh Task Contract verification against that exact PR range.
10. Run the complete policy-selected Task PR review against the unchanged range.
11. Apply the common Acceptance threshold and record contract observations,
    commits, range, verification, review, findings integration, triage,
    non-blocking concerns, and gaps.

Acceptance remains attached to that exact Task Contract authority, PR base,
head, merge base, range, and status. Never replace it with a later descendant or
synthetic feature range.

For promotion reconciliation, replace implementation steps 1–8 with inspection
of the supplied original base-to-current range, validation of every ownership
mapping, fresh current-contract verification, and any authorized bounded
correction. Existing preserved commits satisfy the commit requirement when the
range needs no edit. Create only a declared bounded artifact commit when approved
design or plan files remain uncommitted. Then run authoritative steps 8–11
against the full reconciled range.

## Resume only safe attributable state

Before resuming after an interruption:

1. confirm the prior writer is inactive and no writer overlaps;
2. inspect the task workspace's branch, planned base, HEAD, status, commits, and
   exact PR diff;
3. attribute all in-scope edits and commits to this task;
4. confirm the original handoff still applies.

When all checks pass, continue from the observed state. If implementation is
already committed and its verification remains fresh for that unchanged head,
resume only the pending read-only gate; do not create a duplicate commit. If any
check is uncertain, preserve all state and return `BLOCKED` with the observed
agent and Git evidence plus the exact re-entry condition. Never clean, reset,
rebase, amend, discard, or silently restart to force progress.

Use `Escalate` only when resumption requires a material architecture, goal,
scope, responsibility, public or shared interface, invariant, verification,
policy, or authority decision.

## Give each check phase direct role-specific evidence

Keep the complete Task identity and authority evidence in the owning Task loop;
do not copy it into one broad common leaf wrapper. Give `verify` only the exact
workspace, branch, planned base, merge base, current head, range, clean-state
precondition, changed-file inventory, completed-input Verification Matrix,
command-environment facts, allowed ignored-artifact boundary, source-mutation
invariant, and `PASS`/`FAIL`/`BLOCKED` report contract. Keep exact authority
sources directly available for a matrix row, but do not send the complete Review
policy unless one of its exact constraints changes the verification route.

When a verifier, reviewer, or integrator needs discovery, pass any current
matching cache hit as navigation together with its source identity and
invalidation conditions. The recipient still resolves the exact current target
and authority directly and returns any cache candidate separately from its
verdict.

After a fresh verifier `PASS`, give each selected reviewer the exact unchanged
verified target, diff, changed files, its applicable authority clauses and
perspective, Review context and complete Review policy, the completed
Verification Matrix, and relevant prior triage, concerns, or gaps. Keep exact
authority sources directly available without inlining unrelated unchanged
prose. Give a findings integrator only the unchanged target, complete reports
for its integration, applicable authority, Review context and policy, prior
triage, and origin/remedy evidence it must assess.

Writer reports remain candidate evidence, verifier reports return the completed
matrix and one mechanical verdict, reviewers return perspective-specific
semantic results, and integrators return reconciled findings. Preserve each
report directly; do not translate it into a competing evidence format.

Before dispatch, apply the ancestry invariant above and confirm that branch,
planned base, merge base, HEAD, range, changed files, inspected diff, and
post-edit verification still agree. Missing, contradictory, preliminary, or
stale evidence returns `BLOCKED`.

## Invoke the authoritative Task PR checks

Invoke `verify` first with the current-head Verification Matrix for the exact
authoritative Task PR. Proceed only on its fresh completed-matrix `PASS` for the
unchanged planned base, merge base, head, range, diff, and status. Then invoke
`review` with that completed matrix and the complete approved policy. Let
`review` select and schedule only the policy-required task perspectives and
return `CLEAN`, `FINDINGS`, or `BLOCKED`.

All new-format planned verifier and reviewer leaves remain descendants of the
bound Task orchestrator. Schedule them through
`agent-teams-driven-development` under the unchanged current lease. Queue a
selected check when capacity is insufficient; do not move it to the root,
substitute another role, or treat a self-observed free slot as a lease
expansion.

Do not substitute writer self-checks, preliminary candidate checks, standalone
results, or a lead summary for either coordinator-managed phase. An approved
no-agent `focused` policy may use the lead only when `review` permits it;
`adaptive` and `deep` independence remains mandatory.

Require `review` to integrate every `FINDINGS` result before sending it to
`receiving-code-review`. Accept either an independent `review-integrator` report
for the exact unchanged target or, under an explicitly authorized focused
no-agent fallback, the lead's sequential application of the same integration
contract with an explicit statement that no independent integrator ran. Raw
reviewer findings never authorize a correction. This skill consumes the review
and triage results, owns the bounded correction loop, and returns task
acceptance; it does not reinterpret a blocked check as clean.

After triage, route any `Fix` through the correction loop and return any
`Escalate` to the coordinator. A reason of `Design Escalation` returns
immediately without starting queued review or correction work. Preserve an
independent, out-of-scope valid problem as a non-blocking concern in the Task
result without creating a backlog or silently expanding the Task. When every
finding is `Push back` on the same unchanged target and no `Fix` or `Escalate`
remains, close the task gate with the review, integration, and triage evidence
and do not rerun review merely to obtain the literal word `CLEAN`. A complete
task gate is closed either by `CLEAN` or by exact `FINDINGS` evidence whose every
item has a current `Push back` classification.

## Apply the common finding threshold

Specification findings use `Must Fix` or `Should Improve`. For `adaptive` and
`deep`, map an evidence-qualified quality `Critical` to `Must Fix` and
`Important` to `Should Improve`; do not promote lower labels or non-findings.

Keep only findings that apply to the Review context, identify a concrete
reachable behavior or approved-contract violation, cite evidence, state a
material consequence, and propose a proportionate correction. `Should Improve`
requires a concrete maintainability consequence or measurable repeated cost.
Drop preference-only, speculative, unsupported, inapplicable, or already-decided
objections without materially new evidence.

## Correct and re-review without an open-ended loop

For each authorized correction, retain the exact concrete finding or failed
observation, every observed correction attempt, prior reviewed head `H1`, prior
reviewer reports and triage, and the unchanged complete selected reviewer set.
Give the existing writer only the bounded correction, unchanged Feature and Task
Contracts with shared interfaces or unchanged eligible legacy task authority,
current planned PR base, responsibility boundaries, focused writer obligations,
and a correction commit intent bounded to the finding with its fixed message or
explicit writer message-selection authority.

For a new-format planned correction, also give the writer the exact planned
`search-cache.md` path, current matching entry or miss, its source identity,
currentness, and invalidation conditions, and the Feature-lead-only writer and
non-authority boundary. Omit this planned-only cache input for eligible legacy
corrections.

Then:

1. implement only the bounded correction and inspect its diff;
2. run any applicable writer or pre-commit checks without treating them as the
   authoritative gate;
3. create only the declared correction commit;
4. record new head `H2`, status, merge base, full `base..H2` target, and the
   `H1..H2` correction delta;
5. rebuild the Verification Matrix for `H2` and invoke fresh authoritative
   `verify` against that committed range;
6. only after `PASS`, rerun the same complete policy-selected reviewer set with
   prior reports and triage, the `H1..H2` correction delta, direct access to the
   full `base..H2` target, and the fresh completed matrix;
7. `review` owns correction-review scope and escalation; apply its
   targeted-default policy to the unchanged reviewer set;
8. run required findings integration and triage against the unchanged `H2`.

Reviewer selection is never recalculated from the delta. Do not duplicate or
override `review`'s correction traversal rules here.

Do not reuse stale verification, approval, head, or range. If the same concrete
problem repeats without progress or another action would repeat an observed
failed correction, stop with `Escalate` and report the attempts and remaining
gap. Do not create another identifier or tracking schema for the finding.

## Return task acceptance

Return the complete result to the owning caller without translating or dropping
evidence. For new-format planned work, the Task orchestrator returns it to
`execute-plan`. `Candidate`, `Accepted`, `BLOCKED`, and `Escalate` end the
current planned Task-orchestrator turn. An Accepted result does not start a wait
or polling loop. Re-entry always requires a fresh complete handoff and Git
revalidation, whether the same idle identity is reused or an attributable
replacement is selected.

Return:

- `Candidate` only for plan-authorized early implementation with an
  attributable commit and preliminary checks when the final PR base is not yet
  materialized; it is never task acceptance;
- `Accepted` only when the ancestry invariant holds, every contractually fixed
  exact command and selected check passes, every observable Task Contract
  obligation, eligible legacy task criterion, or promotion-reconciliation
  mapping has current evidence, and the complete selected gate is closed for the
  current head by `CLEAN` or by fully resolved current `Push back` triage;
- `BLOCKED` when a safe writer state, command, permission, range, reviewer, or
  other operational prerequisite cannot be established;
- `Escalate` for a material decision, scope or policy change, explicit
  independent-gate/no-agent conflict, plan deviation, or repeated correction
  without progress.

Include the exact authority and Task Contract content/currentness, mode, writer
status, starting Git status and final Git status, each including index, worktree,
and relevant untracked state, task and correction commits, workspace and branch,
planned PR base ref and commit, merge base, current head, exact range, changed
files, commands and observed results, pre-commit inspection, gate result when
authoritative, reviewer and findings-integration outcomes, triage, discipline
evidence, cache candidates, non-blocking concerns, owning Task-loop identity, configured,
observed, and
effective capacity, root grant, selected and queued roles, concerns, gaps, and
exact re-entry condition.
Return this evidence to the invoking coordinator or
`execute-plan`; do not advance another task or cross-phase gate.
