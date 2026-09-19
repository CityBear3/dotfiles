# Prepare Task workspace panes

Use this for a new planned Task workspace after its direct Git identity checks
pass. Keep the engineer's normal Neovim on the left and the independent Task
Lead on the right, both using the exact Task worktree. Follow the installed
Herdr Skill's environment, explicit identity and process-readiness rules.

## Prepare the two panes

Retain the initial pane returned by worktree creation as the editor pane.
Confirm its workspace, worktree cwd and interactive shell readiness, then split
that exact pane to the right without changing the engineer's focus:

```sh
herdr pane split --pane <editor-pane-id> --direction right \
  --cwd <absolute-task-worktree> --no-focus
```

Shell-quote the actual dynamic arguments. Read the new Task Lead pane ID from
`.result.pane.pane_id`; retain it separately from the editor pane ID. Check the
returned workspace, both panes' cwd and left/right layout using live Herdr
state. The right pane must be at an interactive shell prompt with the shell in
the foreground, ready for `dispatching-parallel-agents`.

If splitting fails or the Task Lead pane identity, cwd or occupant is uncertain,
preserve the returned state and report BLOCKED with the attempted operation and
observations. Do not substitute the editor pane for a missing Task Lead shell.

## Open the editor

Use `herdr pane run` with the explicit left pane ID only while that pane is a
verified available shell in the Task worktree. Start normal `nvim` with the
engineer's usual configuration and startup experience. Set `NVIM_LOG_FILE` to
a shell-quoted temporary log path so logging does not dirty the worktree.
Do not force a file, read-only mode or a special editor configuration.

Confirm the foreground Neovim process and visible editor output before reporting
that it opened. Command submission alone is not startup evidence. The engineer
navigates the Task worktree and coordinates manual saves with the Task Lead so
their writes do not overlap.

If Neovim cannot start or exits, leave the left pane at its shell and report the
Task, worktree, both pane IDs, attempted launch and observed error. Continue
with the available right Task Lead shell; editor failure does not block Task
execution or require an editor repair before dispatch. If an uncertain
foreground process remains, report its state without killing it or sending a
shell command over it. The independent right shell can still be used.

## Return and reuse the mapping

Return the validated worktree/workspace identity, editor pane ID and startup
outcome, and Task Lead pane ID and shell readiness to `execute-plan`. Only the
Task Lead pane is a dispatch target. Editor state is for the engineer's use,
not Task verification or Acceptance evidence.

On re-entry, resolve the recorded panes and their current occupants before
acting. Reuse a matching editor and the existing Task Lead session instead of
splitting or launching duplicates. An editor retry uses its confirmed shell;
an already-running editor is not a shell-command target. Preserve active
processes and the engineer's layout changes. Do not migrate previously approved
Tasks or rearrange an existing running workspace to impose this new layout.
