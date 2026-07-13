---
name: review
description: |
  Run a comprehensive code review using verification and adversarial reviewer agents in parallel.
  Use after verification passes, or when the user requests a review.
  Invoke with `/review`.
argument-hint: "[file paths or branch name (optional)]"
---

# Code Review

Run a comprehensive review by launching verification and adversarial reviewer agents in parallel, integrating adversarial findings, then presenting a unified report.

**Announce at start:** "I'm using the review skill to run a comprehensive code review."

## Entry Conditions

- `verify` has passed (build, test, lint all green)
- Or the engineer explicitly requests a review at any point

## Input

`$ARGUMENTS` optionally specifies file paths or a branch to review. If omitted, review all changes in the current branch compared to the base branch.

## Execution

### Step 1: Determine Scope and Build Context Bundle

#### Scope

Identify the files to review:

- If file paths are given, use those
- If a branch is given, diff against the base branch
- If nothing is given, use `git diff` to find changed files

#### Context Bundle

Build a context bundle to pass to every reviewer. Each source is read **fail-safe**: if absent, skip and continue with that field empty. Record what was loaded for the report's Context section.

1. **Design Doc** — Identify and read relevant sections.
2. **Plan** — Identify and read, especially "Alternative Solutions Considered" and "Out of scope" (used by adversarial personas for `already_decided_check`).
3. **Project rules**:
   - `CLAUDE.md` at the project root
   - All `*.md` files under `.claude/rules/` (glob)
4. **Language detection**: inspect manifest files in this priority order and stop at the first match:
   - `Cargo.toml` → `rust`
   - `go.mod` → `go`
   - `package.json` → `typescript` (treat both TS and JS projects as `typescript`)
   - `pyproject.toml` or `requirements.txt` → `python`
   - none → `unknown`
5. **Language hints**: read `~/.claude/skills/review/hints/<primary_language>.md` if it exists. Skip if `primary_language` is `unknown` or the file does not exist.

The bundle is structured as:

```
context_bundle:
  scope:
    diff: <branch vs base diff>
    changed_files: [<path>...]
  intent:
    design_doc: <relevant sections | empty>
    plan:
      alternative_solutions: <... | empty>
      out_of_scope: <... | empty>
      tasks: <... | empty>
  conventions:
    claude_md: <project CLAUDE.md content | empty>
    rules: [{name: <filename>, content: <...>}]  # all .claude/rules/*.md
  language_hints:
    primary_language: <rust|go|typescript|python|unknown>
    hints: <content of ~/.claude/skills/review/hints/<lang>.md | empty>
```

### Step 2: Dispatch All 7 Reviewer Agents Simultaneously

**Dispatch all 7 agents in a single batch via parallel Agent tool calls in ONE message.** **Do NOT pass a `name` parameter to these Agent calls.** A named spawn becomes a persistent *teammate* (its own tmux pane, mailbox, and agent-teams lifecycle); a name-less spawn is a one-shot *subagent* that returns its findings directly as the tool result. Review is fan-out → aggregate, so the reviewers must be name-less subagents — passing `name` triggers the agent-teams pane / zombie / shutdown problems. **Pass `model: "opus"` explicitly on every Agent call.** The agent definitions' frontmatter already pins opus, but the explicit parameter keeps the cost policy auditable at the call site and guards against frontmatter drift — reviewers must never inherit the lead's session model (e.g. Fable 5, whose per-token cost is too high for a 7-agent fan-out). **Run them in the foreground — do NOT set `run_in_background: true`** — so each reviewer's findings return inline as the tool result; backgrounding switches retrieval to async notifications and complicates the Step 2.5 / Step 3 aggregation. Do NOT launch verification agents first, wait for them, then launch adversarial agents. Both layers run **concurrently**. The "Verification" / "Adversarial" labels below are categorical (model tier, output schema, depth of reasoning), **not** execution phases.

| # | Agent | Layer | Model | Extended Thinking | Purpose |
|---|---|---|---|---|---|
| 1 | `design-alignment-reviewer` | Verification | opus | no | Does the implementation match the design doc? |
| 2 | `scope-reviewer` | Verification | opus | no | Does the implementation cover the plan's scope? |
| 3 | `test-coverage-reviewer` | Verification | opus | no | Are all use cases covered? Edge cases? |
| 4 | `adversarial-robustness-reviewer` | Adversarial | opus | **yes** (`ultrathink`) | Hunt for failure modes |
| 5 | `adversarial-api-reviewer` | Adversarial | opus | **yes** (`ultrathink`) | Hunt for misuse-prone APIs |
| 6 | `adversarial-performance-reviewer` | Adversarial | opus | **yes** (`ultrathink`) | Hunt for measurable cost on hot paths |
| 7 | `adversarial-tests-reviewer` | Adversarial | opus | **yes** (`ultrathink`) | Hunt for tests that don't prove behavior |

Each agent receives the file list and the context bundle in its prompt.

**Wall-clock measurement**: Record dispatch time and each agent's completion time. After all 7 agents complete (before Step 2.5), append the timings to:

```
~/.claude/usage-data/review-timings/<ISO-8601-timestamp>.json
```

with structure:

```json
{
  "started_at": "<ISO-8601>",
  "scope": {"changed_files": <N>, "primary_language": "<lang>"},
  "agents": [
    {"name": "design-alignment-reviewer", "started_at": "...", "completed_at": "...", "duration_ms": <int>},
    ...
  ]
}
```

If `~/.claude/usage-data/review-timings/` does not exist, create it.

#### Adversarial-layer requirements (agents 4–7)

Each adversarial persona must:

- Include `ultrathink` in the prompt so they use extended thinking; deeper-reasoning model surfaces subtle, hypothesis-driven issues
- Use the context bundle's Design Doc / Plan / rules / hints to inform the hunt
- Produce findings in the structured YAML schema (see "Adversarial Output Schema" below), including an `already_decided_check` field that records consultation of Design Doc and Plan
- Report every genuine concern found, including uncertain ones, each carrying a `confidence` field. The finder stage optimizes for **coverage**; importance/confidence filtering happens downstream in the integrator (Step 2.5). Fabricated evidence is forbidden; honest uncertainty (`confidence: low`) is not. Return `findings: []` (with a `considered:` list) only when a genuine hunt surfaces nothing

### Step 2.5: Integrate Adversarial Findings (depends on agents 4–7 only)

When the 4 adversarial agents (4–7) complete, launch the `adversarial-integrator` agent. **This step does NOT wait for verification agents (1–3)** — verification findings flow directly to Step 3 in parallel.

Dispatch the integrator under the same rules as the Step 2 reviewers, for the same reasons: **name-less** (one-shot subagent, not a teammate), **foreground** (do NOT set `run_in_background: true`), and with **`model: "opus"` passed explicitly**. The integrator's output channel is its final text: the integrated section must come back **inline as the Agent tool result** — never as an Artifact, never as a file (see Red Flags).

Inputs to the integrator:

- The 4 adversarial findings
- Design Doc / Plan / CLAUDE.md / rules from the context bundle (for already-decided filtering)

The integrator returns a single deduplicated, severity-normalized markdown section inline as its final text — this return value is what Step 3 embeds in the unified report. Verification findings are NOT passed to the integrator.

### Step 3: Unified Report

Present a single report:

```
## Context

- Scope: <N> files changed in <branch> vs <base>
- Language: <primary_language> (hints: <loaded | not found>)
- Project rules: CLAUDE.md <loaded | not found> / .claude/rules/ (<N> files: <names>)
- Design Doc: <name> §<sections> | not found
- Plan: <name> | not found

## Verification Layer

### Design Alignment
[findings from design-alignment-reviewer]

### Scope Completeness
[findings from scope-reviewer]

### Test Coverage
[findings from test-coverage-reviewer]

## Adversarial Layer

[integrated findings from adversarial-integrator]

## Summary

[Must Fix / Should Improve counts, prioritized action items]
```

### Step 4: Autonomous Triage + Transition

After producing the report, Claude Code applies `/receiving-code-review` discipline to **triage each Must Fix / Should Improve item** into one of three outcomes (per CLAUDE.md Core Flow):

**Push back** — already decided (Design Doc / Design Discussion / plan's "Alternative Solutions" / plan's "Out of scope"), violates YAGNI, technically wrong, or reviewer lacks context.
→ Annotate the item in the report as "pushed back" with the decision source cited. No fix task is created. No engineer prompt.

**Fix** — minor improvements, bugs, or quality items within the existing design.
→ Append a fix task to the plan's "Post-/review iteration" section with concrete steps. Then complete Step 5 and re-invoke `/execute-plan` autonomously. The loop continues: /execute-plan → /verify → /review → (triage again). No engineer prompt.

**Escalate** — architecture / Design Doc contract change / scope expansion beyond the plan / substantive new evidence overturning a prior decision.
→ Complete Step 5, then stop the loop. Present to the engineer: the item, the triage reasoning, what design change appears necessary, the recommended next step.

Whatever the triage outcomes, complete **Step 5 (append the local review report)** before executing any transition defined in this step — the fix-loop re-entry, the escalation stop, and the clean `/finish-branch` transition all happen only after the report file is updated.

**Engineer involvement at this step**:

The engineer is **NOT prompted** for triage decisions, for choosing what to fix, or for confirmation to re-enter `/execute-plan`. The loop runs autonomously per CLAUDE.md's Autonomous loop phase.

The engineer is surfaced only when:

- An item is **escalated** (above), OR
- All items are resolved and the report has no remaining Must Fix / Should Improve. An item counts as **resolved** only when it was pushed back, or when a subsequent fresh `/review` run no longer reports it — executing a fix does NOT resolve an item. A report from which even one Fix task was executed is therefore never clean: the Fix path always re-enters the loop (`/execute-plan` → `/verify` → `/review`), and only that next fresh `/review` run — never the agent-teams internal reviewers' (spec-reviewer / code-quality-reviewer) approval — renders the clean verdict. When the clean verdict holds, first complete **Step 5** for this iteration, then present the final clean report with the triage summary and **transition to `/finish-branch` automatically** — on a clean review this transition is **NOT** gated (per CLAUDE.md Role and Autonomy). Do NOT pause to ask the engineer for approval to proceed; the engineer's control point is `/finish-branch`'s own options menu (PR / merge / keep / discard), which always stops for the engineer's choice. (`/finish-branch` is in turn followed by the terminal `/session-teardown` wrap-up.)

**Triage summary format** (appended to the report when surfacing to the engineer):

```
## Triage Summary

- Pushed back: <N> items (sources: <decision sources>)
- Fixed: <N> fix tasks appended and executed across <K> loop iterations
- Escalated: <N> items (see escalation section above)
```

**Loop termination guard**: if the same item recurs across 2 consecutive review iterations after a Fix attempt, escalate it instead — the fix is not working and may require a design change.

If unsure between Fix and Escalate for an item, lean toward **Fix**. The engineer can override during the next plan review.

### Step 5: Append the Local Review Report (every iteration, before transitioning)

After triage completes — whatever the outcome (fix-loop re-entry, escalation, or clean) — append this iteration's report to a local HTML file next to the plan, then transition. The file is the engineer-facing record of what each iteration found and how it was triaged: browser-readable, workspace-local, and gone when the worktree is removed. Do NOT publish the review report as an Artifact — external hosting is not part of this flow.

If Step 1's context bundle identified no plan file, skip this step and note the skip in the chat report (the report lives next to the plan and has no home without one).

1. **Path**: same directory and basename as the plan file, suffixed `-review.html`. Example: plan `docs/plans/2026-07-13-foo.md` → report `docs/plans/2026-07-13-foo-review.html`.
2. **Keep it uncommitted** — idempotently register the report's repo-relative path in the clone-local exclude file (never the repo's tracked `.gitignore`). Register the exact path, not a `*-review.html` glob: a glob would silently hide unrelated untracked files matching the suffix anywhere in the clone, defeating the reason info/exclude was chosen (leave the target repo untouched):

   ```sh
   REPORT="<repo-relative report path from item 1, e.g. docs/plans/2026-07-13-foo-review.html>"
   EXCLUDE="$(git rev-parse --path-format=absolute --git-common-dir)/info/exclude"
   grep -qxF "$REPORT" "$EXCLUDE" 2>/dev/null || echo "$REPORT" >> "$EXCLUDE"
   ```

   `info/exclude` lives in the common git dir, so it covers every worktree of the clone without touching shared repo state.
3. **First iteration**: if the report file does not exist, create it from the template below, filling `[feature]` from the plan basename and `[branch]` from the current branch.
4. **Append the iteration section**: insert a new `<section class="iteration">` immediately before the closing `</main>` tag. Determine the iteration number by counting the existing `<section class="iteration">` blocks already in the file and adding 1 — the first append into a freshly created file is iteration 1. The section contains, in order: an `<h2>` with that iteration number and today's date, the Step 3 unified report, and the Step 4 triage summary including pushed-back items with their cited decision sources. Convert the markdown mechanically — `##`/`###` headings to `<h3>`/`<h4>`, code fences to `<pre><code>`, finding icons and structure preserved. **Escape every transcribed body before it goes into the HTML** — replace `&` → `&amp;`, then `<` → `&lt;`, then `>` → `&gt;` (in that order) across all finding text and especially code snippets, so that HTML/XML literals in the reviewed diff (e.g. a `</main>` or `<section>` string) render as visible text and cannot corrupt the document or the `</main>` append anchor the next iteration relies on. This is a transcription of content the lead already produced, not a rewrite: do not re-analyze, re-word, or drop findings.

Template (first creation only):

```html
<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Review Report: [feature] ([branch])</title>
<style>
  :root { color-scheme: light dark; }
  body { font-family: -apple-system, "Hiragino Sans", sans-serif; max-width: 60rem; margin: 2rem auto; padding: 0 1rem; line-height: 1.7; }
  section.iteration { border-top: 2px solid rgba(128,128,128,.4); margin-top: 2.5rem; padding-top: 1rem; }
  pre { background: rgba(128,128,128,.12); padding: .75rem; border-radius: 6px; overflow-x: auto; }
  code { font-family: ui-monospace, "SF Mono", Menlo, monospace; font-size: .9em; }
</style>
</head>
<body>
<h1>Review Report: [feature] ([branch])</h1>
<main>
</main>
</body>
</html>
```

## Finding Format

### Icons

- 🔴 **Must Fix** — Bugs, incorrect behavior, security issues, design violations
- 🟡 **Should Improve** — Code smell, suboptimal patterns, missing edge cases, maintainability concerns
- 🟢 **Good** — Well-implemented aspects worth noting (use sparingly, only for genuinely notable decisions)

### Structure

For each finding:

```
<icon> **<short title>**

📄 `<file_path>:<line_number>`
```<language>
<relevant code snippet (3-10 lines, focused on the issue)>
```

**Issue**: <what is wrong or could be improved>

**Suggestion**: <concrete improvement with code if applicable>

**Trade-off**: <what the suggestion costs — complexity, performance, scope creep, etc. If no trade-off, state "None">
```

### Rules

- Always include the file path and line number
- Always include a code snippet showing the relevant code
- Always include a trade-off analysis, even if it's "None"
- Group findings by aspect (sub-section), not by severity
- At the end of the report, provide a prioritized summary: Must Fix items first, then Should Improve, with count per category
- Do NOT mark things as "Good" that are merely adequate — reserve it for genuinely good design decisions

## Adversarial Output Schema

Each adversarial persona must produce findings in this YAML structure (the integrator parses these and emits the markdown Finding Format above):

```yaml
findings:
  - title: <短い見出し>
    hypothesis: "X が起きうる。なぜなら Y"
    evidence:
      - file: <path>
        lines: <range>
        observation: <観察された事実>
    reproduction: "入力 / 操作 Z で再現可能"
    already_decided_check: "Design Doc §X / Plan Alternative Solutions / Out of scope を確認: <該当なし | 該当あり: 出典>"
    severity_suggestion: Critical | Important | Minor
    confidence: high | medium | low  # finding が実在し到達可能だという確信度(再現の具体性とは独立の軸)
    rationale: <severity の根拠 1 行>
considered:
  - <レビューした観点 1>
  - <レビューした観点 2>
```

When no genuine concerns are found, return `findings: []` with `considered:` populated.

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Skipping review because "verify passed" | Verify checks mechanics. Review checks design alignment and quality. Both are needed. |
| Running review before verify passes | Fix build/test/lint issues first. Don't waste review effort on broken code. |
| Proceeding to finish-branch with unaddressed Must Fix items | Must Fix items are blocking. Address them first. |
| Review without design doc context | If a design doc exists, include it. Otherwise note the gap in the Context section. |
| Asking the engineer how to handle each review item | Triage autonomously (push back / fix / escalate) per `/receiving-code-review`. Engineer involvement is restricted to escalations and the final `/finish-branch` transition. |
| Treating "review → execute-plan" or "review → finish-branch" as a transition requiring confirmation | Both run autonomously per CLAUDE.md. The review feedback loop and the clean-review `review → finish-branch` transition are NOT gated — neither requires engineer confirmation. The engineer's control point is `/finish-branch`'s own options menu. |
| Pausing to ask the engineer before transitioning `review → finish-branch` on a clean review | The transition is automatic (not gated). Invoke `/finish-branch` directly; it stops at its own options menu for the engineer's choice. |
| Re-prompting "shall I proceed with fixes?" after producing the report | The plan already authorized autonomous execution. Append fix tasks and re-invoke `/execute-plan` directly. |
| Adversarial persona self-filtering findings it judges uncertain or low-severity | Report them with `confidence` and severity; the integrator filters downstream. Finder-stage self-filtering silently drops real bugs (recall loss). |
| Adversarial persona fabricating evidence or reproductions to "find something" | Evidence must come from code actually read. Honest uncertainty is reported as `confidence: low`, never dressed up as certainty. |
| Skipping language hint loading because the file is missing | Each context source is read fail-safe. Empty fields are normal; record them in the Context section. |
| Dispatching verification agents (1–3) first, waiting for completion, then dispatching adversarial agents (4–7) | All 7 agents launch in a single batch via parallel Agent tool calls in ONE message. The Verification / Adversarial labels are categorical, not phasal. Two-wave dispatch defeats the wall-clock benefit and is forbidden. |
| Passing a `name` to a reviewer Agent call | Dispatch name-less. A named spawn becomes a teammate (tmux pane + agent-teams lifecycle bugs); reviewers must be one-shot subagents that return findings directly. |
| Setting `run_in_background: true` on a reviewer Agent call | Run foreground. Findings must return inline as the tool result for Step 2.5 / Step 3 aggregation. |
| Dispatching the integrator (Step 2.5) with a `name`, in the background, or without an explicit `model: "opus"` | The integrator follows the same dispatch rules as the Step 2 reviewers, for the same reasons: name-less one-shot subagent, foreground, model pinned explicitly at the call site. |
| The integrator returning its result via the Artifact tool or a file instead of its final text | The integrated section must come back inline as the Agent tool result — Step 3 embeds it in the unified report. Any other channel breaks the aggregation (this is the drift Step 2.5's dispatch rules exist to prevent). |
| Publishing the review report as an Artifact, or skipping the Step 5 local report | The engineer-facing record is the Step 5 local HTML next to the plan — appended every iteration (clean runs included), kept uncommitted via `info/exclude`. External hosting is not part of this flow. |

## Important Rules

- The engineer's judgment overrides review findings during escalation or at `/finish-branch`'s options menu. The `review → finish-branch` transition on a clean review is automatic (not gated); per-item triage (push back / fix / escalate) is Claude Code's responsibility, executed autonomously per `/receiving-code-review` without prompting the engineer.
- Human review gate: Claude Code's review does not replace the engineer's review. Both are required before merging.
- Adding support for a new language is done by dropping a new `hints/<lang>.md` file in this skill's `hints/` directory. The skill auto-loads any file matching `<detected_language>.md` — no code change required.

## Integration

When the engineer (or Claude Code) reads this review report, apply `/receiving-code-review` discipline: verify before implementing, no performative agreement.

This skill uses 8 agents defined at the top-level `agents/` directory:

- **Verification reviewers (3, no extended thinking)**: `design-alignment-reviewer`, `scope-reviewer`, `test-coverage-reviewer`
- **Adversarial personas (4, with extended thinking)**: `adversarial-robustness-reviewer`, `adversarial-api-reviewer`, `adversarial-performance-reviewer`, `adversarial-tests-reviewer`
- **Integrator (1, lightweight)**: `adversarial-integrator`

These are distinct from the `code-reviewer` agent used by `/agent-teams-driven-development` for lightweight per-task gates.
