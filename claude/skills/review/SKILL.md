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

The four agents are defined in `skills/review/agents/`:

1. **Design Alignment** (`design-alignment-reviewer`) — Does the implementation match the design doc?
2. **Code Quality** (`code-quality-reviewer`) — Naming, patterns, error handling, complexity
3. **Test Coverage** (`test-coverage-reviewer`) — Are all use cases covered? Edge cases?
4. **Scope Completeness** (`scope-reviewer`) — Does the implementation cover the plan's scope?

For each agent, provide:
- The list of files to review
- The content of the relevant design doc (for design-alignment-reviewer)
- The content of the plan (for scope-reviewer)

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

### Step 4: Transition

After the engineer reviews the report:

- If issues need fixing → Return to `implement` to address them, then re-run `verify` → `review`
- If review passes → Transition to `finish-branch`

The engineer decides whether issues are blocking or acceptable.

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

## Important Rules

- The engineer's judgment overrides review findings. They decide what to fix and what to accept.
- Human review gate: Claude Code's review does not replace the engineer's review. Both are required before merging.