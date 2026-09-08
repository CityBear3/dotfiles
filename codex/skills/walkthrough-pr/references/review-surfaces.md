# Prepare and navigate a review surface

Read this after the engineer selects Hunk or nvim, or when reconnecting to a
selected viewer. This procedure supports the walkthrough's comprehension task;
it does not start an AI reviewer, create a worktree, or change branches.

## Resolve the selection and access

Retain the selected surface, exact review target, repository path, and any known
viewer/pane identity. Confirm the selected executable is available. Learn syntax
from installed help; do not install or update tools as part of this procedure.
For nvim help and launch commands, set `NVIM_LOG_FILE` to a log in a temporary
inspection directory: its default log fallback can otherwise create `nvim.log`
in the working directory, including during `nvim --help`.

Distinguish a missing executable, an unstarted viewer, a wrong session or target,
and a sandbox permission failure. An empty Hunk session list is not sufficient
evidence that no viewer is running. Hunk uses a local daemon, and a sandbox can
block its loopback connection. Herdr has its own local API access boundary.

When evidence points to sandbox access, retry the specific inspection or control
operation through the environment's permission mechanism. Use existing applicable
authorization; otherwise explain the operation and request the required runtime
permission. Do not disable the sandbox globally, expose a daemon beyond local
access, change socket permissions, or treat a rejected operation as success.
Avoid repeated identical retries without new evidence. If access remains blocked,
report what could not be confirmed and let the engineer choose how to continue.
An inaccessible session alone never authorizes creating a duplicate viewer.

## Prepare a Herdr pane

For Hunk, resolve existing sessions using the Hunk section first. Enter pane
creation only when a matching viewer is absent and access is confirmed. A matching
existing Hunk session can be reused without opening another pane.

First read the installed Herdr skill via `herdr --skill` unless it is already in
context. Before inspecting or controlling live Herdr state, verify:

```sh
test "${HERDR_ENV:-}" = 1
```

If this fails, stop Herdr operations. Do not inspect the UI-focused session from
outside Herdr or manufacture its environment variables. Explain that pane setup
requires continuing inside Herdr; let the engineer choose that route or a surface
that does not need a new pane. Independent repository investigation may continue.

Inside Herdr, discover the caller and its workspace instead of relying on UI
focus. These are installed CLI examples; recheck help when syntax differs:

```sh
herdr pane current --current
herdr workspace get "$HERDR_WORKSPACE_ID"
herdr pane list --workspace "$HERDR_WORKSPACE_ID"
herdr pane layout --current
```

Confirm the caller workspace is appropriate for the review repository. Do not
create another workspace, tab, or worktree to resolve a mismatch implicitly.
Retain the explicit workspace and pane IDs returned by Herdr.

Reuse a known review viewer only when its owner, process, and displayed target
are confirmed. Before launching into an existing shell pane, check its process
and visible output: it must be a designated review pane at an interactive shell
prompt with no foreground command. An unrelated editor, agent, approval dialog,
or unknown occupant is not an available shell. Never stop or overwrite it.

If no suitable review pane exists, split the caller's pane. Honor an explicitly
requested direction; otherwise choose right for a wide pane and down for a tall
or narrow pane. Preserve focus and use the verified review repository as cwd:

```sh
herdr pane split --current --direction right --cwd "$review_repo" --no-focus
```

Here `review_repo` is a task-local variable containing the verified absolute
repository path, not an assumed shell variable. Replace `right` as appropriate.
Read the new ID from `.result.pane.pane_id`; never derive it from sidebar order.
Inspect readiness before running the viewer:

```sh
herdr pane process-info --pane "$review_pane_id"
herdr pane read "$review_pane_id" --source visible --lines 40
herdr pane run "$review_pane_id" "$viewer_command"
```

Set `review_pane_id` from the returned ID and construct `viewer_command` for the
selected viewer below. `pane run` submits shell command text: quote every dynamic
path/ref for that shell and validate numeric line arguments. JSON escaping alone
is not shell quoting. Use explicit pane IDs on every subsequent operation.

After launch, check the foreground process, visible output, and viewer-specific
target before calling setup successful. A created pane or a successful command
submission alone does not establish that the viewer opened the intended code.
Retain pane/session identities for navigation and report the pane to the engineer.

## Hunk

First inspect `hunk session --help` and `hunk session list --json`. Match a session
to the review repository and exact comparison, then inspect its context. If
multiple sessions share a repository, use a confirmed explicit session ID rather
than an ambiguous `--repo` lookup. Check the corresponding pane when inside Herdr.
Resolve access failures before deciding that a new Hunk process is needed.

Reuse a matching session. Otherwise, once absence is established, launch Hunk in
the prepared review pane with the exact comparison. Installed command shapes are:

```text
hunk diff <reviewed-base-commit> <reviewed-head-commit>
hunk diff --staged
hunk diff
```

Choose the shape matching the previously resolved target. Do not substitute
`hunk show HEAD`, a default branch, or the current working tree for a PR range.
For local changes, confirm that staged/unstaged/untracked coverage matches the
agreed scope. Do not reload an unrelated existing session to change its target.

After startup, resolve the new session and verify its context/comparison. Navigate
with the confirmed session ID, file, and old/new side of the diff. For example:

```text
hunk session navigate <session-id> --file <relative-path> --new-line <line>
```

Recheck context after navigation before describing the requested location as
displayed. If the session disappears, use the access diagnosis above before
starting another process. Consult the official
[live session documentation](https://www.hunk.dev/docs/agents/live-session-control/)
when needed.

## nvim and editor links

Editor links identify a file and location; they do not prove that nvim opened it.
Use supported local file links without inventing an editor URI scheme. Verify
that a linked local file contains the reviewed version before treating it as
evidence for a PR revision.

For nvim in Herdr, select the actual file and line to inspect, verify the file's
content against the review target, then launch it in the prepared pane. A
read-only viewing command shape supported by the installed nvim is:

```text
NVIM_LOG_FILE=<temporary-log-path> nvim -R -n -i NONE +<line> -- <absolute-file>
```

If the checkout differs from the reviewed revision, extract the exact file into
a temporary inspection directory without switching branches. Label it with its
original repository path and revision, and navigate that snapshot. Do not present
current checkout content as if it came from another revision. Apply the same
rule to editor links and base-side/deleted-file inspection.

Before navigating an existing nvim pane, confirm its process and viewer ownership.
Use its supported editor navigation or an explicitly verified RPC endpoint; never
submit a shell command through `pane run` while nvim is in the foreground. Inspect
the current buffer and cursor after navigation. Do not use forced buffer changes,
save files, discard edits, or send commands to an unidentified pane occupant.

## Keep setup subordinate to the dialogue

Resume the purpose/boundary topic after the chosen surface is ready. If setup is
blocked, keep the limitation explicit and agree on another surface or a deferred
setup; do not silently substitute one. Preserve the same question and review
target when changing surfaces. Keep the viewer available throughout the dialogue;
finishing the walkthrough does not close existing panes or discard buffers.
