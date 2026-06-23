---
name: session-teardown
description: |
  Wrap up a finished session: best-effort shut down the agent-teams team, then prompt
  the engineer to end the session (the reliable cleanup). Runs automatically after
  finish-branch. Never runs /exit itself.
---

# Session Teardown

Wrap up after a branch is finished: release the agent-teams team (best-effort) and prompt the engineer to end the session. The **reliable** cleanup is session exit — Claude Code never exits on its own.

**Announce at start:** "I'm using the session-teardown skill to wrap up the session."

**Core principle:** Best-effort team shutdown → prompt session end. Session exit (auto-cleanup) is the source of truth; explicit shutdown is a courtesy.

## When to Use

Invoked automatically at the end of `/finish-branch`, after the chosen option (PR / merge / keep / discard) has completed. Not invoked directly by the engineer in normal flow.

## Background

Per the agent-teams model (v2.1.178+): there is one implicit team per session, and **the team and its directories are cleaned up automatically when the session exits** — there is no `TeamDelete`. So the guaranteed way to release the team is to end the session.

An explicit teammate shutdown is **best-effort only**: in current Claude Code it can be a no-op or leave a stale roster (known experimental-agent-teams shutdown bugs). This skill therefore shuts down **fire-and-forget** and never depends on the result.

This is the **one sanctioned teardown point** — session end. It does **not** contradict `/agent-teams-driven-development`, which forbids shutting down teammates *mid-loop* (that blocks the lead and stalls the loop). Here the loop is over.

## Process

### Step 1: Identify the live team

Determine whether this session has live teammates spawned by `/agent-teams-driven-development` (e.g. `implementer`, `spec-reviewer`, `code-quality-reviewer`).

- If there is **no live team** (none spawned, or a trivial change that skipped agent-teams), skip Step 2 and go to Step 3.
- `/verify` and `/review` use one-shot subagents that self-complete — they are **not** teammates and need no teardown.

### Step 2: Best-effort shutdown (fire-and-forget)

For each live teammate, send a single shutdown request and **immediately continue** — do not wait for a reply:

```
SendMessage({ to: <name>, message: { type: "shutdown_request", reason: "session wrap-up" } })
```

(A plain-text "shut down" request also works.)

- **Do NOT wait for `shutdown_response`.** Waiting blocks the lead (observed failure).
- **Do NOT call `TeamDelete`** — it no longer exists (removed in v2.1.178).
- If a teammate ignores the request or a stale roster remains, that is acceptable — the session exit in Step 3 reclaims everything.

### Step 3: Prompt the engineer to end the session

Tell the engineer the work is complete and that ending the session is the reliable cleanup. **Never run `/exit` yourself** — the engineer decides.

```
Branch finished and the team has been asked to shut down (best-effort).
The reliable cleanup is ending this session — running /exit releases the
team and its directories automatically.

- If you're done: run /exit.
- If you have more work: continue here (the idle team is harmless and is
  reclaimed when the session eventually exits).
```

## Red Flags

| Violation | Correct Behavior |
|---|---|
| Wait for a teammate's `shutdown_response` | Fire-and-forget. Never block on shutdown. |
| Run `/exit` automatically | Never. Prompt the engineer; session end is their decision. |
| Call `TeamDelete` | It was removed in v2.1.178. Rely on session-exit auto-cleanup. |
| Shut down teammates mid-loop to "save resources" | Forbidden by `/agent-teams-driven-development` (blocks the loop). This skill runs only at session end. |
| Block / retry until the team is confirmed gone | Best-effort only. Session exit is the source of truth. |

## Integration

- Invoked by `/finish-branch` (its Transition step) after the chosen option completes.
- Tears down the team created by `/agent-teams-driven-development`; see that skill's "Teammate Lifecycle" for why mid-loop shutdown is forbidden and end-of-session shutdown is the sanctioned point.
