---
name: execute-lightweight-task
description: Produce or accept one root-owned lightweight Task PR from a recoverable combined contract with an exact range, fresh verification, policy-selected review, and bounded correction.
---

# Execute one lightweight task

Own implementation and authoritative acceptance of exactly one root-owned
lightweight Task PR or bounded correction. Do not select a workflow path,
publish, merge, or choose branch disposition from this skill.

## Require one lightweight task handoff

The lightweight handoff contains:

- the complete recoverable combined in-memory Feature/Task Contract, original
  request and design authority, exact authority identity and currentness,
  assigned obligations, protected constraints, non-goals, and delegated local
  decisions;
- the root-owned lightweight loop identity and current root-granted leaf count;
- the exact Task PR target: workspace, branch, base ref and exact base commit,
  current head, merge base, exact base-to-head range, inspected diff, and
  starting Git status including index, worktree, and relevant untracked state;
- the separate Review context and complete active Review policy;
- the discipline and applicable repository guidance;
- responsibility and ownership boundaries, verification obligations and
  observable routes, and the responsibility-scoped commit intent with its fixed
  message or approved writer message-selection authority;
- attributable commits, prior verification and review, concerns, gaps, and
  prior attributable lightweight evidence for correction or re-entry;
- configured, observed, and effective subagent capacity, live identities, and
  every selected or queued role;
- contractually significant files, interfaces, signatures, ordering, and exact
  commands only when the authority fixes them.

The lightweight variant never requires or fabricates another planned-only
artifact.

Reject a planned-only handoff instead of interpreting it as lightweight. A
Task orchestrator, Herdr workspace requirement, Task DAG, PR topology, planned
`search-cache.md`, or `execute-plan` correction route belongs to the planned
executor. Do not infer or discard those fields to force this route.

The Review context describes the artifact, purpose, consumers, interpretation
or execution model, material quality criteria and realistic failures, approved
non-problems, and inapplicable assumptions. The Review policy records mode,
rationale, risk surfaces, the Task PR gate, any integration perspective, skips
with reasons, findings-only general integration, authority-defect priority,
residual risk, capacity and queue rules, and the common Acceptance threshold.

Reject missing, stale, contradictory, or unrecoverable input. Return the named
gap to `agentic-engineering-workflow`; do not infer a decision, expand scope,
add a wrapper, or weaken evidence. Stop with `Escalate` when implementation
needs a new or changed goal, scope, responsibility owner, public or shared
interface semantic, invariant, material failure behavior, compatibility
promise, verification obligation, Review policy, or authority.

Resolve the supplied workspace, branch, base, current head, merge base, range,
diff, and status directly from Git. For fresh implementation, require the
supplied starting commit to equal workspace HEAD. For acceptance, require the
exact base commit to be an ancestor of the reviewed head. Recheck all of them
after commits and before acceptance. On failure, preserve state and return
`BLOCKED`; never rewrite history to manufacture the target.

## Run in the root-owned Task-loop context

The root owns this lightweight loop and dispatches its bounded leaves directly
through `agent-teams-driven-development`. Every implementer, verifier, reviewer,
adversarial integrator, and findings integrator is a leaf and must not spawn
descendants. Every new leaf uses explicit `fork_turns="none"` and one complete
role-specific handoff with directly readable authority and Git sources. If
no-history creation is unavailable, return `BLOCKED` instead of inheriting
parent turns.

Treat `agents.max_threads` as subagent capacity across the complete descendant
tree, excluding the root and counting every live leaf. Use the lower configured
or currently observed capacity. The root alone grants leaf capacity. Start with
one baseline leaf and use it serially for the writer, verifier, findings
integration, triage, and correction. Only after fresh verifier `PASS` and
selection of at least two independent source reviewers may the root temporarily
expand the reviewer wave, up to three total Task leaves or smaller current
capacity. Revoke expansion before integration, triage, or correction. Queue
selected roles in policy order; free capacity is availability, not authority.

## Choose one writer

Keep exactly one writer: the root when direct execution is authorized,
otherwise one `implementer`. Resolve the role before loading its prompt: use
the named profile when available, or
[implementer-prompt.md](../agent-teams-driven-development/implementer-prompt.md)
as its fallback.

Construct one compact writer role message containing only the purpose and
expected result, owned responsibility, applicable combined-contract identity
and currentness, assigned clauses and preserved boundaries, constraints and
non-goals, delegated local decisions, discipline, workspace and branch, exact
base and starting head, responsibility-scoped commit intent and message
authority, focused writer-side checks, and every contractually fixed file,
signature, ordering rule, or command. Keep exact authority sources directly
available. Omit Review context and policy, completed gate evidence, review
scheduling, capacity, and queue state from the writer message.

Independent initial authority reads, repository searches, relevant file reads,
and Git inspection may run in one bounded programmatic batch only when each
result remains attributable. Stop before a result-dependent judgment. Preserve
behavior-change order exactly: focused RED -> production edit -> focused GREEN
-> refactor while green. Run independent mechanical post-edit checks only after
focused GREEN and never batch across those stages.

Require the writer to report the actual pre-production RED and reason; never
recreate historical RED evidence after the production edit. For content,
configuration, refactoring, or mechanical migrations, preserve the declared
green baseline. Historical discipline gaps are separate from current defects,
material evidence gaps, contract deviations, and controlling authority.

The writer reports `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED`, or `NEEDS_CONTEXT`,
changed files and behavior, actual discipline history, all commands and observed
results, pre-commit inspection, the commit and new head when complete, concerns,
and gaps. Classify that result before producing Task evidence. Only verified
`DONE` may advance, and it is Candidate evidence only, never task acceptance.
For `DONE_WITH_CONCERNS`, classify each concern as an authorized correction,
operational `BLOCKED`, or user-owned `Escalate`.
`BLOCKED` preserves the gap and observed state. `NEEDS_CONTEXT` is `BLOCKED`
when the missing input is safely discoverable within current authority;
otherwise it is `Escalate`. Do not let another writer result enter Task evidence
or the authoritative gate unclassified.

## Produce current lightweight Task PR evidence

For fresh implementation:

1. Record the combined-contract identity and currentness, workspace, branch,
   starting commit, exact base, head, merge base, range, diff, status, root-owned
   loop identity, capacity, grant, and queue.
2. Implement only the declared scope with the selected discipline.
3. Run every contractually required exact command plus focused checks needed for
   the owned responsibility.
4. Inspect authority coverage, the working-tree diff, unrelated state, and all
   changed files.
5. Create only the responsibility-scoped Task commit.
6. Record the new head and inspect the attributable base-to-head range.

After the exact committed head resolves, build one in-memory current-head
Verification Matrix. Map every observable obligation to its authority, one
bounded command or check, expected observation, and `FAIL` or `BLOCKED`
non-match. Invalidate and rebuild the matrix when head, range, controlling
authority, or a material verification route changes. Never persist it as a
schema or let another role infer a missing row.

## Resume only safe attributable state

Before resuming after interruption, replacement, partial edit, partial commit,
or a lost report:

1. confirm the prior writer is inactive and no writer overlaps;
2. inspect branch, base, HEAD, status, commits, and exact diff directly;
3. attribute every in-scope edit and commit to this task;
4. revalidate the combined contract and original handoff.

An idle identity receives a fresh complete handoff and repeats direct Git and
authority validation. If committed implementation and its checks remain current
for the unchanged head, continue only the pending read-only gate. Otherwise
preserve state and return `BLOCKED`; never clean, reset, rebase, amend, discard,
or silently restart.

## Give each check phase direct role-specific evidence

Keep the complete combined contract and Task identity in this root-owned loop.
Give `verify` the exact target, status precondition, changed-file inventory,
completed Verification Matrix, command environment, allowed ignored artifacts,
source-mutation invariant, and `PASS`/`FAIL`/`BLOCKED` result contract. Keep
authority sources directly available without sending unrelated Review policy.

After fresh verifier `PASS`, give each selected reviewer the exact unchanged
verified target, diff, changed files, applicable authority clauses and
perspective, Review context and complete policy, completed matrix, and relevant
prior triage, concerns, or gaps. Give a findings integrator only the unchanged
target, complete reports, applicable authority, Review context and policy, and
the origin/remedy evidence it must reconcile. Preserve every role report
directly instead of translating it into a competing format.

## Invoke the authoritative lightweight Task PR checks

Invoke `verify` first against the exact current lightweight Task PR and its
completed matrix. Continue only on fresh `PASS` for the unchanged base, merge
base, head, range, diff, and status. Then invoke `review` with that matrix and
complete policy. Let it select and schedule only required perspectives and
return `CLEAN`, `FINDINGS`, or `BLOCKED`.

The root dispatches these leaves through `agent-teams-driven-development` under
the current grant. Do not substitute writer self-checks, preliminary checks,
standalone results, or a root summary for the gate. Require `review` to
integrate `FINDINGS` before `receiving-code-review` triage. Raw findings never
authorize correction.

Apply the common finding threshold: retain only findings applicable to Review
context with an approved requirement, concrete reachable evidence, material
consequence, and proportionate remedy. After triage, correct every `Fix`, retain
current `Push back`, return `Escalate` immediately, and preserve an independent
out-of-scope valid problem only as a non-blocking concern. A gate closes through
`CLEAN` or exact findings whose every item has a current `Push back`.

## Correct and re-review without an open-ended loop

For each authorized lightweight correction, retain the exact finding or failed
observation, observed attempts, prior reviewed head `H1`, reports and triage,
and the unchanged complete selected reviewer set. Give the writer a fresh
bounded correction message with the unchanged combined contract, exact current
target, responsibility boundaries, focused obligations, and correction commit
intent. Do not add a planned cache or route correction through `execute-plan`.

Then:

1. implement only the correction, run focused writer checks, inspect its diff,
   and create only the correction commit;
2. record `H2`, status, merge base, full `base..H2` target, and `H1..H2` delta;
3. rebuild the matrix and invoke fresh `verify` against `H2`;
4. after `PASS`, rerun the same complete reviewer set with prior evidence, the
   correction delta, full target, and fresh matrix;
5. require delta-first traversal and a fresh full-target verdict, then integrate
   and triage any findings against unchanged `H2`.

Prior verdicts are navigation only and never authorize `H2`. Switch to ordinary
full traversal when the correction escapes authorization; changes a material
interface, responsibility, authority, policy, or test strategy; lacks complete
prior evidence; exposes another finding; or cannot prove prior areas unaffected.
Do not recalculate reviewer selection from the delta. If the same concrete
problem repeats without progress, stop with `Escalate` and report the attempts.

## Return lightweight task acceptance

Return the complete result to `agentic-engineering-workflow`. `Accepted` ends
the loop without another wait or polling. Re-entry always requires a fresh
complete handoff and Git/authority validation.

Return:

- `Accepted` only when every fixed command and selected check passes, every
  combined-contract obligation has current evidence, and the complete selected
  gate is closed by `CLEAN` or resolved current `Push back` triage;
- `BLOCKED` when a safe writer state, command, permission, range, reviewer, or
  operational prerequisite cannot be established;
- `Escalate` for a material decision, scope or policy change, explicit
  independent-gate conflict, promotion to planned work, or repeated correction
  without progress.

Include exact authority identity/currentness, writer state, starting and final
Git status, commits, workspace and branch, base, merge base, head, exact range,
changed files, commands and results, pre-commit inspection, matrix and gate
results, review/integration/triage, TDD history, non-blocking concerns, root loop
identity, capacity, grant, queue, gaps, and exact re-entry condition. Do not
advance an integration-only gate, publication, or branch disposition here.
