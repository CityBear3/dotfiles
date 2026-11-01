# Prepare the Task Lead pane

Use this for a new planned Task workspace after its direct Git identity checks
pass. Follow the installed Herdr Skill's environment, explicit identity and
process-readiness rules.

## Verify the initial Task Lead shell

Retain the initial pane returned by worktree creation as the Task Lead pane.
Confirm its workspace, exact Task worktree cwd and interactive shell readiness
using live Herdr state. The shell must be in the foreground and ready for
`dispatch-task-leads`. Keep the engineer's focus unchanged.

Do not split an editor pane or start Vim/Neovim during this preparation. The
engineer opens and rearranges an editor manually when needed, coordinating
manual saves with the Task Lead so their writes do not overlap.

If the Task Lead pane identity, cwd or occupant is missing or uncertain,
preserve the returned state and report BLOCKED with the attempted operation
and observations. Do not kill an occupant, send a shell command over another
foreground process, or substitute the focused pane for a missing verified shell.

## Return and reuse the mapping

Return the validated worktree/workspace identity, Task Lead pane ID and shell
readiness to `execute-plan`. No editor pane or editor startup result is required
for dispatch, Task verification or Acceptance.

On re-entry, resolve the recorded Task Lead pane and its current occupant before
acting. Reuse the matching session instead of creating another pane or writer.
Identify it by the recorded mapping, not left/right position, current focus,
or an assumption that the initial pane still hosts it. Preserve active processes,
manually opened editors and the engineer's layout changes. Do not migrate
previously approved Tasks or rearrange an existing running workspace to impose
this new default.
