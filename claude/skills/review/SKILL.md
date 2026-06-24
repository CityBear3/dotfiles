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

**Dispatch all 7 agents in a single batch via parallel Agent tool calls in ONE message.** **Do NOT pass a `name` parameter to these Agent calls.** A named spawn becomes a persistent *teammate* (its own tmux pane, mailbox, and agent-teams lifecycle); a name-less spawn is a one-shot *subagent* that returns its findings directly as the tool result. Review is fan-out → aggregate, so the reviewers must be name-less subagents — passing `name` triggers the agent-teams pane / zombie / shutdown problems. **Run them in the foreground — do NOT set `run_in_background: true`** — so each reviewer's findings return inline as the tool result; backgrounding switches retrieval to async notifications and complicates the Step 2.5 / Step 3 aggregation. Do NOT launch verification agents first, wait for them, then launch adversarial agents. Both layers run **concurrently**. The "Verification" / "Adversarial" labels below are categorical (model tier, output schema, depth of reasoning), **not** execution phases.

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
- Return `findings: []` (with a `considered:` list of what was examined) when no genuine concerns were found. **Null-finding is acceptable** — speculative or "just in case" findings are forbidden

### Step 2.5: Integrate Adversarial Findings (depends on agents 4–7 only)

When the 4 adversarial agents (4–7) complete, launch the `adversarial-integrator` agent. **This step does NOT wait for verification agents (1–3)** — verification findings flow directly to Step 3 in parallel.

Inputs to the integrator:

- The 4 adversarial findings
- Design Doc / Plan / CLAUDE.md / rules from the context bundle (for already-decided filtering)

The integrator returns a single deduplicated, severity-normalized markdown section. Verification findings are NOT passed to the integrator.

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
→ Append a fix task to the plan's "Post-/review iteration" section with concrete steps. Then re-invoke `/execute-plan` autonomously. The loop continues: /execute-plan → /verify → /review → (triage again). No engineer prompt.

**Escalate** — architecture / Design Doc contract change / scope expansion beyond the plan / substantive new evidence overturning a prior decision.
→ Stop the loop. Present to the engineer: the item, the triage reasoning, what design change appears necessary, the recommended next step.

**Engineer involvement at this step**:

The engineer is **NOT prompted** for triage decisions, for choosing what to fix, or for confirmation to re-enter `/execute-plan`. The loop runs autonomously per CLAUDE.md's Autonomous loop phase.

The engineer is surfaced only when:

- An item is **escalated** (above), OR
- All items are resolved (any combination of push back / fix / no items at all) and the report has no remaining Must Fix / Should Improve. In this case, present the final clean report with the triage summary and **transition to `/finish-branch`** — this is a phase transition and DOES require engineer confirmation per CLAUDE.md Role and Autonomy. (`/finish-branch` is in turn followed by the terminal `/session-teardown` wrap-up.)

**Triage summary format** (appended to the report when surfacing to the engineer):

```
## Triage Summary

- Pushed back: <N> items (sources: <decision sources>)
- Fixed: <N> fix tasks appended and executed across <K> loop iterations
- Escalated: <N> items (see escalation section above)
```

**Loop termination guard**: if the same item recurs across 2 consecutive review iterations after a Fix attempt, escalate it instead — the fix is not working and may require a design change.

If unsure between Fix and Escalate for an item, lean toward **Fix**. The engineer can override during the next plan review.

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
| Treating "review → execute-plan" as a phase transition requiring confirmation | The review feedback loop is part of the autonomous loop phase per CLAUDE.md. Phase transition only applies to "review → finish-branch" (loop exit on clean review). |
| Re-prompting "shall I proceed with fixes?" after producing the report | The plan already authorized autonomous execution. Append fix tasks and re-invoke `/execute-plan` directly. |
| Adversarial persona inventing speculative findings to "find something" | Null-finding is acceptable. Return `findings: []` with `considered:` when no genuine concern with concrete reproduction can be constructed. |
| Skipping language hint loading because the file is missing | Each context source is read fail-safe. Empty fields are normal; record them in the Context section. |
| Dispatching verification agents (1–3) first, waiting for completion, then dispatching adversarial agents (4–7) | All 7 agents launch in a single batch via parallel Agent tool calls in ONE message. The Verification / Adversarial labels are categorical, not phasal. Two-wave dispatch defeats the wall-clock benefit and is forbidden. |
| Passing a `name` to a reviewer Agent call | Dispatch name-less. A named spawn becomes a teammate (tmux pane + agent-teams lifecycle bugs); reviewers must be one-shot subagents that return findings directly. |
| Setting `run_in_background: true` on a reviewer Agent call | Run foreground. Findings must return inline as the tool result for Step 2.5 / Step 3 aggregation. |

## Important Rules

- The engineer's judgment overrides review findings during escalation or at the final transition to `/finish-branch`. Per-item triage (push back / fix / escalate) is Claude Code's responsibility, executed autonomously per `/receiving-code-review` without prompting the engineer.
- Human review gate: Claude Code's review does not replace the engineer's review. Both are required before merging.
- Adding support for a new language is done by dropping a new `hints/<lang>.md` file in this skill's `hints/` directory. The skill auto-loads any file matching `<detected_language>.md` — no code change required.

## Integration

When the engineer (or Claude Code) reads this review report, apply `/receiving-code-review` discipline: verify before implementing, no performative agreement.

This skill uses 8 agents defined at the top-level `agents/` directory:

- **Verification reviewers (3, no extended thinking)**: `design-alignment-reviewer`, `scope-reviewer`, `test-coverage-reviewer`
- **Adversarial personas (4, with extended thinking)**: `adversarial-robustness-reviewer`, `adversarial-api-reviewer`, `adversarial-performance-reviewer`, `adversarial-tests-reviewer`
- **Integrator (1, lightweight)**: `adversarial-integrator`

These are distinct from the `code-reviewer` agent used by `/agent-teams-driven-development` for lightweight per-task gates.
