---
name: review
description: |
  Run a comprehensive code review using aspect-specific review agents in parallel.
  Use after verification passes, or when the user requests a review.
  Invoke with `/review`.
argument-hint: "[file paths or branch name (optional)]"
---

# Code Review

Run a comprehensive review by launching aspect-specific review agents in parallel, then present a unified report.

**Announce at start:** "I'm using the review skill to run a comprehensive code review."

## Entry Conditions

- `verify` has passed (build, test, lint all green)
- Or the engineer explicitly requests a review at any point

## Input

`$ARGUMENTS` optionally specifies file paths or a branch to review. If omitted, review all changes in the current branch compared to the base branch.

## Execution

### Step 1: Determine Scope

Identify the files to review:
- If file paths are given, use those
- If a branch is given, diff against the base branch
- If nothing is given, use `git diff` to find changed files

Also identify the relevant design doc and plan for context.

### Step 2: Launch Review Agents

Launch 4 agents in parallel using the Agent tool, one per review aspect. Each agent receives the list of files to review and relevant context.

The four agents are defined at the top-level `agents/` directory (discovered by Claude Code's standard agent discovery):

1. **Design Alignment** (`design-alignment-reviewer`) — Does the implementation match the design doc?
2. **Code Quality** (`code-quality-reviewer`) — Naming, patterns, error handling, complexity, performance
3. **Test Coverage** (`test-coverage-reviewer`) — Are all use cases covered? Edge cases?
4. **Scope Completeness** (`scope-reviewer`) — Does the implementation cover the plan's scope?

For each agent, provide:
- The list of files to review
- The content of the relevant design doc (for design-alignment-reviewer)
- The content of the plan (for scope-reviewer)

When invoking `code-quality-reviewer`, include `ultrathink` in the prompt so the
agent uses extended thinking to dig into non-obvious problems and performance
implications. This reviewer runs on a deeper-reasoning model and is the place
where subtle issues should be surfaced.

### Step 3: Unified Report

Collect results from all agents and present a single report with this format:

```
## Review Report

### Design Alignment
[findings]

### Code Quality
[findings]

### Test Coverage
[findings]

### Scope Completeness
[findings]

### Summary
[overall assessment and prioritized action items]
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
- All items are resolved (any combination of push back / fix / no items at all) and the report has no remaining Must Fix / Should Improve. In this case, present the final clean report with the triage summary and **transition to `/finish-branch`** — this is a phase transition and DOES require engineer confirmation per CLAUDE.md Role and Autonomy.

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

- **Must Fix** — Bugs, incorrect behavior, security issues, design violations
- **Should Improve** — Code smell, suboptimal patterns, missing edge cases, maintainability concerns
- **Good** — Well-implemented aspects worth noting (use sparingly, only for genuinely notable decisions)

### Structure

For each finding:

```
<severity> **<short title>**

file_path:line_number
<relevant code snippet (3-10 lines, focused on the issue)>

**Issue**: <what is wrong or could be improved>

**Suggestion**: <concrete improvement with code if applicable>

**Trade-off**: <what the suggestion costs — complexity, performance, scope creep, etc. If no trade-off, state "None">
```

### Rules

- Always include the file path and line number
- Always include a code snippet showing the relevant code
- Always include a trade-off analysis, even if it's "None"
- Group findings by aspect, not by severity
- At the end, provide a prioritized summary: Must Fix items first, then Should Improve, with count per category
- Do NOT mark things as "Good" that are merely adequate — reserve it for genuinely good design decisions

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Skipping review because "verify passed" | Verify checks mechanics. Review checks design alignment and quality. Both are needed. |
| Running review before verify passes | Fix build/test/lint issues first. Don't waste review effort on broken code. |
| Proceeding to finish-branch with unaddressed Must Fix items | Must Fix items are blocking. Address them first. |
| Review without design doc context | If a design doc exists, include it. Otherwise note the gap. |
| Asking the engineer how to handle each review item | Triage autonomously (push back / fix / escalate) per `/receiving-code-review`. Engineer involvement is restricted to escalations and the final `/finish-branch` transition. |
| Treating "review → execute-plan" as a phase transition requiring confirmation | The review feedback loop is part of the autonomous loop phase per CLAUDE.md. Phase transition only applies to "review → finish-branch" (loop exit on clean review). |
| Re-prompting "shall I proceed with fixes?" after producing the report | The plan already authorized autonomous execution. Append fix tasks and re-invoke `/execute-plan` directly. |

## Important Rules

- The engineer's judgment overrides review findings during escalation or at the final transition to `/finish-branch`. Per-item triage (push back / fix / escalate) is Claude Code's responsibility, executed autonomously per `/receiving-code-review` without prompting the engineer.
- Human review gate: Claude Code's review does not replace the engineer's review. Both are required before merging.

## Integration

When the engineer (or Claude Code) reads this review report, apply `/receiving-code-review` discipline: verify before implementing, no performative agreement.

Note: This skill uses 4 specialized parallel reviewers (`design-alignment-reviewer`, `code-quality-reviewer`, `test-coverage-reviewer`, `scope-reviewer`) defined at the top-level `agents/` directory. These are distinct from the `code-reviewer` agent used by `/agent-teams-driven-development` for lightweight per-task gates.