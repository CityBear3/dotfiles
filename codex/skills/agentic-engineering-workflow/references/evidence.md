# Evidence applicability

## Reuse evidence by applicability

For an existing Accepted result, the Feature Lead uses the [Accepted consumption boundary](execution.md#consume-accepted-as-a-completed-task-decision) to compare its target and relied-on context. Row-level and reviewer-level
coverage mapping belongs to the Task-loop owner; an unchanged Accepted result
does not require the Feature Lead to reconstruct that mapping.

HEAD, branch, commit message or history changes alone do not invalidate
verification or review. The owning lead compares the actual target content and
relied-on context: relevant files (including modes, symlinks and untracked or
generated inputs), base/dependencies/shared interfaces, approved requirements,
and commands, environment and verification conditions. Git metadata matters
when it is itself an input to the behavior or check. A matching diff alone
does not prove unchanged base context; unchanged HEAD does not prove unchanged
worktree content.

Retain each original report, observed target/snapshot, commands, results and
independence. Record the current target and a bounded, concrete comparison
showing which obligations and reviewer perspectives that evidence still covers.
Do not relabel carried results as newly executed or newly reviewed. Current
evidence means fresh observations or explicitly supported carry-forward, not
necessarily a new invocation. No new evidence store or schema is required.

Independent pre-commit evidence may cover the resulting committed Task when
its snapshot and relevant inputs match. For earlier standalone reports, also
map the actual inspected authority, obligations, reviewer coverage and fixed
allocations to the approved Task policy. Keep the standalone report's original
label; the owner records its applicability separately. Neither a label change,
writer self-check nor missing independence satisfies a Task gate. Verify or
review only uncovered or invalidated obligations; unresolved findings stay open.

During active approved work, actual input or requirement changes require an
impact map and fresh affected checks/review under the phase skills. Uncertain
applicability or missing evidence is a gap to resolve, not proof of equivalence.
Preserve unaffected evidence with explicit reasons and the full required
coverage. Evidence applicability does not authorize execution: at completion
or publication, follow the [discussion boundary](completion.md#handle-publication-and-completion-boundaries) before reopening work.

## Distinguish a reviewed base from an advancing branch tip

Keep the exact base commit used for accepted evidence separate from the latest
observed tip of the planned PR base branch. The reviewed base remains an
ancestor of the reviewed head; the branch tip may advance independently and
need not be an ancestor of that head. Do not replace the reviewed base or widen
the accepted range merely to match that tip.

When the same base branch advances by fast-forward, use a bounded Git and
dependency check to confirm that the head/tree, merge base, PR commit set and
diff, source status, controlling authority, and relied-on dependencies and
shared-interface assumptions remain unchanged. If they do, retain Accepted,
verification, review and triage evidence with its original target and record
the new observed tip and comparison result. A tip-only update requires neither
fresh verification/review nor a return to the Task loop, including at PR
creation. Do not dispatch reviewers merely to establish that equivalence.

During active execution, changed Git identities prompt the [applicability comparison](evidence.md#reuse-evidence-by-applicability); actual changed inputs or requirements follow the owning
loop's impact and invalidation rules.
A retarget, rewritten base history, or missing evidence does not qualify for the
tip-only exception. At completion/publication, use the [engineer discussion boundary](completion.md#handle-publication-and-completion-boundaries) instead of automatically reopening that loop.
Keep latest-base compatibility separate from Task evidence. During active work,
refresh only an applicable named integration obligation whose inputs changed.
At completion, missing or stale required integration evidence is a concern for
that same discussion boundary, not permission to rerun checks. Publication and
history-change permissions are unchanged.
