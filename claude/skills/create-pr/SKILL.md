---
name: create-pr
description: Create a GitHub pull request with a structured description derived from the design decisions, plan, and changes. Invoke with `/create-pr`.
argument-hint: "[base branch (optional, defaults to main)]"
---

# Create Pull Request

Create a GitHub pull request with a well-structured description.

## Input

`$ARGUMENTS` optionally specifies the base branch. Defaults to `main`.

## Execution

### Step 1: Gather Context

1. Run `git status` to check for uncommitted changes. If there are uncommitted changes, warn the user and stop.
2. Run `git log <base>..HEAD --oneline` to list all commits on this branch.
3. Run `git diff <base>...HEAD` to see the full diff.
4. **Check for a PR template** in the repository. Look in this order and use the first match:
   - `.github/pull_request_template.md`
   - `.github/PULL_REQUEST_TEMPLATE.md`
   - `.github/PULL_REQUEST_TEMPLATE/*.md` (multiple templates — ask the engineer which to use)
   - Root-level `pull_request_template.md` / `PULL_REQUEST_TEMPLATE.md`
5. Identify the relevant Design Doc, plan, or `/design-discussion` outcome if it exists. If a plan exists with an "Alternative Solutions Considered" section, lift the key decisions for the PR's Design Decisions section.

### Step 2: Draft PR Description

#### 2a. Decide the body structure

- **If a PR template was found in Step 1** → use the template's sections **verbatim** (do not add, remove, or reorder sections). The project's convention takes precedence over this skill's default structure.
- **If no PR template was found** → use the fallback structure below.

#### 2b. Distinguish AI-draftable vs engineer-required sections

| Section type | Examples | Who writes |
|---|---|---|
| **Fact-based** | Summary, Changes, Test Plan, file lists, command outputs | AI may draft (verifiable from diff / commits / test results) |
| **Intent-based** | Design Decisions, Mental models, Trade-offs, Risks, Migration notes, "Why this shape" | **Engineer required** |

**For intent-based sections, AI MUST NOT invent content.** Hallucinated rationale is worse than no rationale — it misleads reviewers and rots into false documentation.

Instead:
- **If a Design Doc exists** → write a one-line link reference and stop. The Design Doc is the durable record; do not duplicate its content inline.
- **If a plan with "Alternative Solutions Considered" exists** → AI may lift content **verbatim** (this is a record of the engineer's prior decision, not AI generation).
- **Otherwise** → insert a placeholder for the engineer to fill:

  ```
  <!-- ENGINEER: write the design intent here. Examples of what to capture:
       - The mental model / framing behind this change (not visible in the diff)
       - Trade-offs considered and rejected, with reasons
       - Hidden assumptions, invariants, or constraints
       - Why this shape and not an alternative
       Reviewers cannot read your mind — write what the code cannot say. -->
  ```

#### 2c. Title

Short (under 70 characters), descriptive of the change. AI-draftable.

#### 2d. Fallback body structure (when no PR template exists)

```
## Summary

<AI-draftable: 1-3 sentences from the diff describing what this PR does>

## Design Decisions

<Design Doc link OR engineer-written rationale OR <!-- ENGINEER --> placeholder>

## Changes

<AI-draftable: bulleted list of key changes, grouped by concern>

## Test Plan

<AI-draftable: tests added, manual verification performed>

🤖 Generated with [Claude Code](https://claude.com/claude-code)
```

**Rationale for this split**: AI can see code and conversation records, but not the engineer's mental model — the framing, the rejected paths, the unstated assumptions. Asking AI to write design rationale produces plausible-sounding but unverifiable prose that misleads reviewers. The engineer is the only source of truth for intent.

### Step 3: Engineer Completion

This is **not just a review step** — it is where the engineer fills in mental models, design rationale, and any context that does not appear in the code. The draft from Step 2 is a starting point, not a finished product.

1. Show the draft title and body.
2. **Explicitly highlight any `<!-- ENGINEER: ... -->` placeholders** that remain. Quote each one and ask the engineer to fill it.
3. Even if no placeholder exists (e.g., a Design Doc link was used), ask one focused question:

   > "Is there context, a mental model, or a constraint behind this change that doesn't appear in the code or the linked Design Doc? Reviewers will read only the PR — anything not written here is invisible to them."

4. Wait for the engineer to either:
   - Provide additions (you incorporate them into the body)
   - Edit the draft directly
   - Explicitly confirm "the draft is complete as-is"

**Do not create the PR while any `<!-- ENGINEER -->` placeholder remains.** Removing a placeholder without filling it is a violation — the engineer either fills it or explicitly states that the section is intentionally empty.

### Step 4: Create PR

After engineer completion:

1. Verify no `<!-- ENGINEER: ... -->` placeholders remain in the body.
2. Push the branch if not already pushed: `git push -u origin HEAD`
3. Create the PR: `gh pr create --title "<title>" --body "<body>" --base <base>`
4. Return the PR URL to the user.

## Rules

- Never create a PR without the engineer's explicit completion of intent-based sections
- **Respect existing PR templates** — if a template was found in Step 1, use its sections verbatim (do not impose this skill's fallback structure on top)
- **Never invent design rationale, mental models, or unstated assumptions** — AI may draft fact-based sections only; intent-based sections require the engineer
- **Prefer linking the Design Doc over inlining** rationale, when one exists — avoid duplication that will rot
- Never proceed to PR creation while any `<!-- ENGINEER: ... -->` placeholder remains
- Always check for uncommitted changes first
- Always push before creating the PR
- If the branch has no commits ahead of the base, inform the user and stop