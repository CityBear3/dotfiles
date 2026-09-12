# Completion and publication

## Handle publication and completion boundaries

Once the relevant execution loop is complete, the Feature Lead directly makes
a bounded read-only check of changes since the accepted target, using retained
context and evidence. Inspect only the delta and the assumptions or behavior it
may affect. Do not dispatch a reviewer or rebuild a verification/review handoff
for this check. It is a currentness check, not a new Acceptance gate or a
substitute for the completed independent review.

Apply the [Accepted consumption boundary](execution.md#consume-accepted-as-a-completed-task-decision). Routine receipt, PR drafting or
merge preparation does not authorize a new search for Task-local quality
problems. Recover a missing result reference from retained sources without
replaying its gate; the concern path below applies when an actual unmet
obligation, relevant mismatch or concrete behavior concern is established.

If no concern remains, continue the already-authorized completion/publication
operation. A Git identity change with established evidence applicability,
including a compatible base-tip advance, does not require discussion by itself.
If a material mismatch, missing required evidence, or possible behavior problem
appears, preserve the original evidence and hold the affected operation. Share
the observed facts, expected behavior, possible impact and remaining unknowns
with the engineer, distinguishing confirmed facts from hypotheses. Establish a
shared understanding of the problem before proposing remedies or deciding how
to proceed. Do not automatically invoke verification, review, triage, correction
or a model escalation merely because a concern was found.

After the problem is understood together, discuss the response and resume work
only within the engineer-agreed response and execution authority. Shared problem
understanding alone is not implementation approval; prior implementation or
publication approval alone does not authorize reopening completed work. Reuse
exact understanding and applicable authorization already supplied by the
engineer without asking again. Active approved Task corrections, dependency
scheduling and integration work retain their existing autonomous loop rules.

An internally accepted Task PR is eligible for publication before Feature
Accepted. If the user requests publication, pass only that task's exact current
evidence to `finish-branch` task mode. Publication is optional for dependency
release, remains an external-write gate, and never retires Feature Contract or
Implementation Plan artifacts.

When human feedback arrives for a published Task PR, re-resolve that exact
branch, planned base, head, range, and contract authority. Apply the completion
discussion boundary above before routing the anchored feedback to triage or a
correction loop. Once the response and execution are authorized, use
`receiving-code-review` where applicable, preserve an accepted result for a
verified `Push back`, and route an authorized `Fix` through the owning Task
Contract and correction loop. Changed consumed inputs or requirements make
affected descendants stale through both topologies; a new head alone does not.
Return `Escalate` to the owning approval gate. Any
resulting push, restack, retarget, or PR update remains separately authorized.

After Feature Accepted, pass the complete topology and feature evidence to
`finish-branch` feature mode. Keep ignored plan artifacts and any existing
`search-cache.md` in the coordination worktree; let an explicitly authorized
later removal of that worktree clean them up with the workspace. Preserve
durable Design Docs and present remaining publication or branch-disposition
choices. Archive plan artifacts only when the
user explicitly requests preservation beyond the worktree lifecycle.

Never treat an edit, candidate, successful command, commit, agent self-review,
stale task result, task count, or incomplete integration evidence as workflow
completion. Report each exact Task PR, feature evidence, Review context, policy,
transitions, remaining findings, and every gap.
