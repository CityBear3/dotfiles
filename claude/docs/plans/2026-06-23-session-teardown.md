# session-teardown Skill Implementation Plan

> **Execution:** Use `/execute-plan` to dispatch this plan to agent-teams. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a `session-teardown` skill that best-effort shuts down the agent-teams team and prompts the engineer to end the session, invoked automatically after `/finish-branch`, and wire it into the Agentic Engineering Workflow.

**Architecture:** A new standalone skill `skills/session-teardown/SKILL.md` performs (1) best-effort fire-and-forget teammate shutdown and (2) a session-end prompt — never running `/exit` itself. `/finish-branch` gains a Transition section that hands off to it after the chosen option completes. `CLAUDE.md` documents it as the terminal Core-Flow phase and states the confirmation policy. `agent-teams-driven-development` is reconciled so the existing "never shut down mid-loop" rule coexists with this sanctioned end-of-session shutdown. Reflected to `~/.claude/` via `install.sh`.

**Tech Stack:** Markdown skill definitions (Claude Code skills). No executable code; verification is structural (grep / read-back) plus `install.sh` propagation.

**Working directory:** `/Users/sakumatomoya/workspace/dotfiles/claude` (run all commands from there).
**Branch:** `feat/session-teardown-skill` (NOT main — create via `/using-git-worktrees` or `git switch -c` before Task 1).
**Baseline before Task 1:** clean working tree; current branch is `main` at the latest commit; `skills/session-teardown/` does not yet exist.

**Per-task verification command** (run before each commit; adapt the grep targets per task):
```sh
cd /Users/sakumatomoya/workspace/dotfiles/claude && git diff --stat && echo "--- grep checks below ---"
```

> **Note on edit location** (user MEMORY `feedback_dotfiles_install`): edit **only** the dotfiles source under `/Users/sakumatomoya/workspace/dotfiles/claude/`. Never edit `~/.claude/` directly. `install.sh` propagates source → `~/.claude/` (Task 5).

---

### Task 1: Create the `session-teardown` skill

**Why:** This is the core artifact — the skill that performs best-effort team shutdown and the session-end prompt. Everything else wires into it.

**Behavior change:** no (new skill definition / documentation)
**Discipline:** doc-edit — verification is structural (file exists, frontmatter + key sections present via grep). No test suite.

**Files:**
- Create: `skills/session-teardown/SKILL.md`

### Steps

- [ ] **Step 1: Create `skills/session-teardown/SKILL.md`**

Create the file with exactly this content:

````markdown
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
````

- [ ] **Step 2: Verify**

```sh
cd /Users/sakumatomoya/workspace/dotfiles/claude
test -f skills/session-teardown/SKILL.md && echo "FILE OK"
grep -q "^name: session-teardown" skills/session-teardown/SKILL.md && echo "FRONTMATTER OK"
grep -qE "fire-and-forget|best-effort" skills/session-teardown/SKILL.md && echo "SHUTDOWN-STANCE OK"
grep -q "Never run .*/exit. yourself" skills/session-teardown/SKILL.md && echo "NO-AUTO-EXIT OK"
```

Expected: `FILE OK`, `FRONTMATTER OK`, `SHUTDOWN-STANCE OK`, `NO-AUTO-EXIT OK`.

- [ ] **Step 3: Commit**

```sh
git add skills/session-teardown/SKILL.md
git commit -m "$(cat <<'EOF'
Add: session-teardown skill (best-effort team shutdown + session-end prompt)

New end-of-session skill. Best-effort fire-and-forget teammate shutdown
(no shutdown_response wait, no TeamDelete) and a prompt for the engineer
to end the session; session exit (auto-cleanup) is the reliable cleanup.
Never runs /exit itself.
EOF
)"
```

---

### Task 2: Wire `finish-branch` → `session-teardown`

**Why:** Without a Transition in finish-branch, nothing invokes the new skill. This is the load-bearing wiring edit.

**Behavior change:** no (skill prose / wiring)
**Discipline:** doc-edit — verify the 4-option menu is untouched and the Transition references session-teardown.

**Files:**
- Modify: `skills/finish-branch/SKILL.md` (append a Transition section after `## Rules`, ending at current line 120)

### Steps

- [ ] **Step 1: Append a Transition section at end of `skills/finish-branch/SKILL.md`**

After the final `## Rules` list (current last line is the Option 2 rule ending `...before deleting the feature branch.`), append:

````markdown

## Transition

After the chosen option (1–4) completes — including after `create-pr` returns for Option 1 — hand off to wrap up the session:

→ Transition to `/session-teardown` to best-effort shut down the agent-teams team and prompt the engineer to end the session.

This runs after the option's git cleanup; it does not add a 5th menu option.
````

- [ ] **Step 2: Verify (Transition added, 4-option menu intact)**

```sh
cd /Users/sakumatomoya/workspace/dotfiles/claude
grep -q "Transition to .*/session-teardown" skills/finish-branch/SKILL.md && echo "HANDOFF OK"
grep -q "Present exactly 4 options" skills/finish-branch/SKILL.md && echo "4-OPTIONS RULE INTACT"
grep -c "^#### Option " skills/finish-branch/SKILL.md   # expect 4
```

Expected: `HANDOFF OK`, `4-OPTIONS RULE INTACT`, and the count `4`.

- [ ] **Step 3: Commit**

```sh
git add skills/finish-branch/SKILL.md
git commit -m "$(cat <<'EOF'
Update: finish-branch hands off to session-teardown after option completes

Adds a Transition section that invokes /session-teardown once the chosen
option (PR/merge/keep/discard) finishes. The 4-option menu is unchanged.
EOF
)"
```

---

### Task 3: Integrate into `CLAUDE.md` (Core Flow + confirmation policy)

**Why:** Document `session-teardown` as the terminal Core-Flow phase and state that the `finish-branch → session-teardown` transition runs automatically while `/exit` is always the engineer's action. Classification decision: register in the **Core Flow diagram + prose only** (NOT the Cross-cutting Skills list — avoid double-listing).

**Behavior change:** no (workflow documentation)
**Discipline:** doc-edit — verify diagram node, prose, and confirmation clause are present.

**Files:**
- Modify: `skills/../CLAUDE.md` → `/Users/sakumatomoya/workspace/dotfiles/claude/CLAUDE.md` (Core Flow mermaid ~line 81; loop-exit prose line 102; What Requires Confirmation line 48)

### Steps

- [ ] **Step 1: Add the terminal node to the Core Flow mermaid**

In `CLAUDE.md`, find:
```
    G -->|なし| F[finish-branch]
```
Replace with:
```
    G -->|なし| F[finish-branch]
    F --> K[session-teardown]
```

- [ ] **Step 2: Extend the loop-exit prose (line 102)**

Find:
```
Already-decided items are never escalated; minor fixes never trigger escalation. The loop continues until `review` reports no remaining items, at which point the flow proceeds to `finish-branch`.
```
Replace with:
```
Already-decided items are never escalated; minor fixes never trigger escalation. The loop continues until `review` reports no remaining items, at which point the flow proceeds to `finish-branch`, then to `session-teardown` — the terminal wrap-up that best-effort shuts down the team and prompts the engineer to end the session (session exit is the reliable cleanup).
```

- [ ] **Step 3: State the confirmation policy (What Requires Confirmation, line 48)**

Find:
```
- Transitioning between workflow phases (e.g., create-plan → execute-plan, review → finish-branch). The autonomous review feedback loop (review → triage → execute-plan re-entry for fix tasks) is **not** a phase transition and does NOT require confirmation — it is part of the autonomous loop phase per "Agentic Orchestration > Core Flow".
```
Replace with:
```
- Transitioning between workflow phases (e.g., create-plan → execute-plan, review → finish-branch). The autonomous review feedback loop (review → triage → execute-plan re-entry for fix tasks) is **not** a phase transition and does NOT require confirmation — it is part of the autonomous loop phase per "Agentic Orchestration > Core Flow". The `finish-branch → session-teardown` transition is the terminal wrap-up and also runs automatically (not gated); ending the session (`/exit`) is always the engineer's action — Claude Code never runs it.
```

- [ ] **Step 4: Verify**

```sh
cd /Users/sakumatomoya/workspace/dotfiles/claude
grep -q "F --> K\[session-teardown\]" CLAUDE.md && echo "DIAGRAM OK"
grep -q "then to .session-teardown." CLAUDE.md && echo "PROSE OK"
grep -q "finish-branch → session-teardown. transition is the terminal wrap-up" CLAUDE.md && echo "CONFIRM-POLICY OK"
grep -c "session-teardown" CLAUDE.md   # expect 3 (diagram + prose + confirmation)
test "$(grep -c 'session-teardown' CLAUDE.md)" -eq 3 && echo "NO DOUBLE-LISTING (3 refs, not in cross-cutting list)"
```

Expected: `DIAGRAM OK`, `PROSE OK`, `CONFIRM-POLICY OK`, count `3`, `NO DOUBLE-LISTING ...`.

- [ ] **Step 5: Commit**

```sh
git add CLAUDE.md
git commit -m "$(cat <<'EOF'
Update: wire session-teardown into CLAUDE.md Core Flow + confirmation policy

Adds session-teardown as the terminal Core-Flow node and loop-exit step,
and documents that finish-branch → session-teardown runs automatically
while /exit is always the engineer's action. Registered in the diagram +
prose only (not double-listed in Cross-cutting Skills).
EOF
)"
```

---

### Task 4: Reconcile `agent-teams-driven-development` + consistency edits

**Why:** The agent-teams skill currently says "No explicit shutdown" / "Cleanup is automatic at session exit." Without reconciliation, the new explicit (best-effort) end-of-session shutdown reads as a contradiction. Also update the two consistency touchpoints flagged in recon.

**Behavior change:** no (doc reconciliation)
**Discipline:** doc-edit — verify the new cross-references exist and the mid-loop rule is preserved.

**Files:**
- Modify: `skills/agent-teams-driven-development/SKILL.md` (Teammate Lifecycle section)
- Modify: `skills/using-git-worktrees/SKILL.md` (Integration "Pairs with")
- Modify: `skills/review/SKILL.md` (clean-review exit line)

### Steps

- [ ] **Step 1: Add the end-of-session carve-out to agent-teams Teammate Lifecycle**

In `skills/agent-teams-driven-development/SKILL.md`, find this bullet (last in "## Teammate Lifecycle"):
```
- **If you must free a teammate mid-session** (e.g. the model-floor correction in Step 4), send a plain-text shutdown request fire-and-forget and continue immediately — never block on the reply.
```
Replace with:
```
- **If you must free a teammate mid-session** (e.g. the model-floor correction in Step 4), send a plain-text shutdown request fire-and-forget and continue immediately — never block on the reply.
- **End-of-session teardown is `/session-teardown`'s job** (invoked from `/finish-branch`): the one sanctioned point for a best-effort, fire-and-forget team shutdown — at session end, when the loop is over. The mid-loop rules above (never originate a blocking shutdown; never tear down per pass) still hold.
```

- [ ] **Step 2: Add session-teardown to using-git-worktrees "Pairs with"**

In `skills/using-git-worktrees/SKILL.md`, find:
```
**Pairs with:**
- `/finish-branch` — REQUIRED for cleanup after work complete
```
Replace with:
```
**Pairs with:**
- `/finish-branch` — REQUIRED for cleanup after work complete
- `/session-teardown` — runs after `/finish-branch` to release the agent-teams team at session end
```

- [ ] **Step 3: Soften the "final finish-branch transition" in review**

In `skills/review/SKILL.md`, find (in Step 4, the clean-review exit bullet):
```
present the final clean report with the triage summary and **transition to `/finish-branch`** — this is a phase transition and DOES require engineer confirmation per CLAUDE.md Role and Autonomy.
```
Replace with:
```
present the final clean report with the triage summary and **transition to `/finish-branch`** — this is a phase transition and DOES require engineer confirmation per CLAUDE.md Role and Autonomy. (`/finish-branch` is in turn followed by the terminal `/session-teardown` wrap-up.)
```

- [ ] **Step 4: Verify**

```sh
cd /Users/sakumatomoya/workspace/dotfiles/claude
grep -q "End-of-session teardown is ./session-teardown" skills/agent-teams-driven-development/SKILL.md && echo "AGENT-TEAMS RECONCILED"
grep -q "never block on the reply" skills/agent-teams-driven-development/SKILL.md && echo "MID-LOOP RULE PRESERVED"
grep -q "/session-teardown. — runs after" skills/using-git-worktrees/SKILL.md && echo "WORKTREES OK"
grep -q "followed by the terminal ./session-teardown" skills/review/SKILL.md && echo "REVIEW OK"
```

Expected: `AGENT-TEAMS RECONCILED`, `MID-LOOP RULE PRESERVED`, `WORKTREES OK`, `REVIEW OK`.

- [ ] **Step 5: Commit**

```sh
git add skills/agent-teams-driven-development/SKILL.md skills/using-git-worktrees/SKILL.md skills/review/SKILL.md
git commit -m "$(cat <<'EOF'
Update: reconcile agent-teams / worktrees / review with session-teardown

agent-teams Teammate Lifecycle now names session-teardown as the one
sanctioned end-of-session shutdown point (mid-loop no-shutdown rule
preserved). using-git-worktrees and review reference the new terminal
wrap-up for an accurate integration map.
EOF
)"
```

---

### Task 5: Reflect to `~/.claude/` and final verification

**Why:** Skills/CLAUDE.md only take effect in `~/.claude/` after `install.sh` runs. This step propagates and verifies the whole change end-to-end.

**Behavior change:** no (deployment)
**Discipline:** doc-edit — run install.sh, verify propagation. No commit (install.sh does not modify the repo).

**Files:** none modified in the repo; runs `install.sh` which writes to `~/.claude/`.

### Steps

- [ ] **Step 1: Run install.sh**

```sh
bash /Users/sakumatomoya/workspace/dotfiles/claude/install.sh
```

Expected: prints `Copied CLAUDE.md`, `Synced skills/`, `Synced agents/`, `Done.`

- [ ] **Step 2: Verify propagation to ~/.claude/**

```sh
test -f ~/.claude/skills/session-teardown/SKILL.md && echo "SKILL PROPAGATED"
grep -q "F --> K\[session-teardown\]" ~/.claude/CLAUDE.md && echo "CLAUDE.md PROPAGATED"
grep -q "Transition to .*/session-teardown" ~/.claude/skills/finish-branch/SKILL.md && echo "FINISH-BRANCH PROPAGATED"
```

Expected: `SKILL PROPAGATED`, `CLAUDE.md PROPAGATED`, `FINISH-BRANCH PROPAGATED`.

---

## Final verification (after all tasks)

```sh
cd /Users/sakumatomoya/workspace/dotfiles/claude
echo "== source refs ==" && grep -rl "session-teardown" skills/ CLAUDE.md
echo "== new skill present ==" && test -f skills/session-teardown/SKILL.md && echo OK
echo "== finish-branch still 4 options ==" && test "$(grep -c '^#### Option ' skills/finish-branch/SKILL.md)" -eq 4 && echo OK
echo "== no TeamDelete reintroduced as instruction ==" && ! grep -RInE "call .*TeamDelete|use .*TeamDelete" skills/ && echo OK
echo "== propagated ==" && test -f ~/.claude/skills/session-teardown/SKILL.md && echo OK
git log --oneline -5
```

Expected: source refs in `session-teardown`, `finish-branch`, `agent-teams-driven-development`, `using-git-worktrees`, `review`, and `CLAUDE.md`; new skill present; finish-branch has exactly 4 options; no instruction to call TeamDelete; propagated; 4 new commits (Tasks 1–4) in the log.

## Post-/review iteration

Reserved for fix tasks appended by Claude Code after `/review` produces actionable items. Empty until `/review` runs.

(See CLAUDE.md "Core Flow" for the autonomous review feedback loop.)

## Push and PR

This change lives in the personal dotfiles repo, where the established convention is direct commits to `main` (see recent history). After the engineer reviews the branch:

```sh
git switch main
git merge --ff-only feat/session-teardown-skill   # or fast-forward per /finish-branch Option 2
git push origin main
```

(If the engineer prefers a PR, use `/finish-branch` Option 1 → `create-pr`.)

## Out of scope

- Converting `/verify` or `/review` to agent-teams — decided to keep them as subagents (fan-out → results; the official subagent-vs-team guidance and the agent-teams bug surface both favor subagents).
- Auto-running `/exit` — explicitly rejected; session end is always the engineer's action.
- Fixing the upstream experimental-agent-teams shutdown/zombie bugs (#68946 / #69022 / #38932 / #55586) — upstream, not fixable in skills; this skill works around them by being best-effort + relying on session-exit cleanup.
- Reintroducing any `TeamDelete` / blocking shutdown handshake — removed/forbidden since v2.1.178.

## Alternative Solutions Considered

- **Fold teardown into `finish-branch`** (no new skill): **Rejected** — keeps finish-branch's 4-option contract clean and separates "finish the branch" from "wrap up the session"; the engineer chose a standalone skill.
- **Prompt `/exit` only, no shutdown at all**: **Rejected** — best-effort fire-and-forget shutdown tidies panes/roster when it works and is harmless when it doesn't; the engineer chose best-effort + prompt.
- **Blocking shutdown handshake (wait for `shutdown_response`)**: **Rejected** — this is exactly the hang fixed in commit daed818; it stalls the lead.
- **`TeamDelete`-style hard teardown**: **Rejected** — `TeamDelete` was removed in v2.1.178; session-exit auto-cleanup is the supported mechanism.
- **Register in CLAUDE.md Cross-cutting Skills list**: **Rejected** — session-teardown is a terminal Core-Flow phase, not an invoked-as-needed cross-cutting helper; registered in the diagram + prose to avoid double-listing.
