---
name: execute-task
description: Produce or accept one Task PR with one writer, an exact planned base-to-head range, fresh verification, policy-selected review, and bounded correction.
---

# Execute one task

Own candidate implementation and authoritative acceptance of exactly one
lightweight Task PR, approved-plan Task PR, or bounded correction. Do not select
a workflow path, schedule dependencies or PR topology, publish, merge, or choose
branch disposition from this skill.

## Require one task handoff

Before implementation, require one concise plain-language handoff containing
the new contract form, an explicitly eligible legacy form, or an approved
promotion-reconciliation form.

For the new form require:

- exact approved Feature Contract identity, path, and currentness evidence, or
  lightweight in-memory contract identity, plus the clauses assigned to this
  task;
- the exact Task Contract, or the same combined contract for lightweight work;
- applicable shared interfaces, adjacent-task obligations, protected
  constraints, and delegated local decisions;
- the separate Review context and complete active Review policy;
- the required discipline and applicable repository guidance;
- approved task workspace, branch, and coordination directory;
- planned PR identity, base ref and exact base commit, current head, and whether
  the handoff is candidate or authoritative;
- current merge base, exact base-to-head range, diff, status, attributable
  commits, prior verification and review, concerns, gaps, and re-entry evidence
  when applicable;
- execution context: the one bound `task-orchestrator` identity for new-format
  planned work, or the root-owned loop for lightweight work;
- configured, observed, and effective subagent capacity; the root-granted leaf
  count for this loop; currently live identities; and any roles already
  selected for the current wave;
- for authoritative re-entry of an attributable candidate, the candidate
  commit, head, preliminary evidence, and authorized final-base materialization
  or restack evidence;
- responsibility and ownership boundaries;
- verification routes and observable obligations;
- the responsibility-scoped commit intent and its fixed message or approved
  writer message-selection authority;
- contractually significant files, signatures, ordering, and exact commands
  only when the contract fixes them.

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
perspectives, skips with reasons, residual risk, capacity and queue rules, and
the common Acceptance threshold.

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
dispatches this Task's leaves. For lightweight work, the root owns this loop and
dispatches the same bounded leaves directly; do not introduce a Task
orchestrator, planned Task artifacts, or Herdr requirement solely for
lightweight execution. Eligible legacy work retains its exact approved invoking
context. Reject a new-format planned handoff delivered to an unbound identity
or a lightweight handoff that pretends to have a planned Task orchestrator.

Treat `agents.max_threads` as subagent capacity across the complete descendant
tree, excluding the root and counting every Task orchestrator and leaf. Use the
lower configured or currently observed capacity. The root alone grants leaf
capacity. This loop normally receives one leaf and may use at most three
concurrent leaves or its smaller current grant. Request missing capacity without
self-expanding; queue already-selected roles in order without dropping,
substituting, reordering, or weakening them. Do not begin a planned orchestrator
turn unless its grant includes a baseline leaf.

## Choose one writer

Keep exactly one writer: the lead when direct execution is authorized, otherwise
one `implementer`. Promotion reconciliation and authoritative re-entry of an
attributable candidate begin with no active writer when the preserved or
restacked range needs only fresh verification and review; select one writer only
for an authorized bounded correction. Resolve the role before loading its
prompt: use the named profile when available, or
[implementer-prompt.md](../agent-teams-driven-development/implementer-prompt.md)
as its fallback. Pass only the selected role and task handoff to
`agent-teams-driven-development`.

Every implementer, verifier, reviewer, and adversarial integrator dispatched by
this loop is a leaf and must not spawn descendants. A capacity lease changes
only scheduling concurrency; it grants no source, Git, policy, publication, or
cleanup authority. The Task orchestrator remains non-writing, and the
implementer remains the sole Task source writer.

Require production behavior changes to use red, green, refactor and report the
observed red failure. For content, configuration, refactoring, or mechanical
migrations, apply the declared discipline and preserve the relevant green
baseline. Preserve unrelated changes.

Inside the applicable new-format or legacy authority, let the writer choose
private files, helpers, local types and interfaces, algorithms, edit order,
applicable standard verification commands, and additional focused
non-destructive checks when those choices are delegated or unspecified. Require
every actual choice and changed file to remain within the approved responsibility
and be reported with evidence.

Require the writer to report:

- `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED`, or `NEEDS_CONTEXT`;
- changed files and implemented behavior;
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
    commits, range, verification, review, concerns, and gaps.

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

## Give check phases direct current evidence

Pass the same Task PR evidence directly to `verify` and then `review`, without
another identity or duplicate record:

- exact authority identity, path or in-memory identity, approval/currentness
  evidence, assigned Feature Contract clauses, exact Task Contract, shared
  interfaces, constraints, non-goals, and delegated decisions; the exact
  eligible legacy task authority and its referenced design sources; or the
  promotion-reconciliation authority and attribution;
- the Review context and active Review policy;
- task workspace, branch, planned PR base ref and commit, merge base, current
  head, exact range, status, and inspected diff;
- responsibility boundaries and actual changed files;
- the complete writer report;
- every verification obligation and fresh required or selected command with its
  expected and observed result;
- commits, pre-commit inspection, repository guidance, concerns, and gaps;
- the planned Task orchestrator or lightweight root context, configured,
  observed, and effective capacity, current root grant, live identities, and
  queued roles.

Eagerly provide assigned clauses and evidence needed by the check. Keep the
exact authority source directly available, but do not inline or require an
unconditional reread of unrelated unchanged prose.

Before dispatch, apply the ancestry invariant above and confirm that branch,
planned base, merge base, HEAD, range, changed files, inspected diff, and
post-edit verification still agree. Missing, contradictory, preliminary, or
stale evidence returns `BLOCKED`.

## Invoke the authoritative Task PR checks

Invoke `verify` first for the exact authoritative Task PR. Proceed only on its
fresh `PASS` for the unchanged planned base, merge base, head, range, diff, and
status. Then invoke `review` with that verification and the complete approved
policy. Let `review` select and schedule only the policy-required task
perspectives and return `CLEAN`, `FINDINGS`, or `BLOCKED`.

All new-format planned verifier and reviewer leaves remain descendants of the
bound Task orchestrator; all lightweight leaves remain direct descendants of
the root. Schedule them through `agent-teams-driven-development` under the
unchanged current lease. Queue a selected check when capacity is insufficient;
do not move it to the root, substitute another role, or treat a self-observed
free slot as a lease expansion.

Do not substitute writer self-checks, preliminary candidate checks, standalone
results, or a lead summary for either coordinator-managed phase. An approved
no-agent `focused` policy may use the lead only when `review` permits it;
`adaptive` and `deep` independence remains mandatory.

Send `FINDINGS` to `receiving-code-review`. This skill consumes the check and
triage results, owns the bounded correction loop, and returns task acceptance;
it does not reinterpret a blocked check as clean.

After triage, route any `Fix` through the correction loop and return any
`Escalate` to the coordinator. When every finding is `Push back` on the same
unchanged target and no `Fix` or `Escalate` remains, close the task gate with the
review plus triage evidence and do not rerun review merely to obtain the literal
word `CLEAN`. A complete task gate is closed either by `CLEAN` or by exact
`FINDINGS` evidence whose every item has a current `Push back` classification.

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
observation and every observed correction attempt. Give the existing writer only
the bounded correction, unchanged Feature and Task Contracts with shared
interfaces, unchanged lightweight combined contract, or unchanged eligible
legacy task authority, Review context, Review policy, current planned PR base,
responsibility boundaries, and verification obligations. Also pass a correction
commit intent bounded to the finding and its fixed message or explicit writer
authority to select the correction message.

Then:

1. implement only the bounded correction and inspect its diff;
2. run any applicable writer or pre-commit checks without treating them as the
   authoritative gate;
3. create only the declared correction commit;
4. record the new current head, status, merge base, and exact planned-base-to-
   head PR range;
5. invoke fresh authoritative `verify` against that committed range;
6. only after `PASS`, rerun the same complete policy-selected task review and
   triage against the unchanged range.

Do not reuse stale verification, approval, head, or range. If the same concrete
problem repeats without progress or another action would repeat an observed
failed correction, stop with `Escalate` and report the attempts and remaining
gap. Do not create another identifier or tracking schema for the finding.

## Return task acceptance

Return the complete result to the owning caller without translating or dropping
evidence. For new-format planned work, the Task orchestrator returns it to
`execute-plan`; for lightweight work, the root consumes it directly.
`Candidate`, `Accepted`, `BLOCKED`, and `Escalate` end the current planned
Task-orchestrator turn. An Accepted result does not start a wait or polling
loop. Re-entry always requires a fresh complete handoff and Git revalidation,
whether the same idle identity is reused or an attributable replacement is
selected.

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
authoritative, owning Task-loop identity, configured/observed/effective
capacity, root grant, selected and queued roles, concerns, gaps, and exact
re-entry condition.
Return this evidence to the invoking coordinator or
`execute-plan`; do not advance another task or cross-phase gate.
