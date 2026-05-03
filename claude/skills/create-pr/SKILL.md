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
4. Identify the relevant Design Doc, plan, or `/design-discussion` outcome if it exists. If a plan exists with an "Alternative Solutions Considered" section, lift the key decisions for the PR's Design Decisions section.

### Step 2: Draft PR Description

Based on the commits, diff, and design context, draft:

- **Title**: Short (under 70 characters), descriptive of the change
- **Body** using this structure:

```
## Summary

<1-3 sentences describing what this PR does and why>

## Design Decisions

<EITHER a link to the Design Doc (if one exists) — one line — OR an inline summary of the key design choices and the alternatives that were considered and rejected. Include this section even when no Design Doc exists; it is the durable record of why this approach was chosen.>

## Changes

<Bulleted list of key changes, grouped by concern>

## Test Plan

<How the changes were verified — tests added, manual testing, etc.>

🤖 Generated with [Claude Code](https://claude.com/claude-code)
```

The **Design Decisions** section is the most-often-skimped part of PR descriptions. For refactor / small-feature PRs that skipped Design Doc, this is where the "why this shape, not the alternatives" lives — otherwise that context dies in chat history. Lift content from the plan's "Alternative Solutions Considered" section verbatim if it exists.

### Step 3: Confirm with User

Present the draft title and body to the user for review. Do NOT create the PR until the user approves.

### Step 4: Create PR

After approval:

1. Push the branch if not already pushed: `git push -u origin HEAD`
2. Create the PR: `gh pr create --title "<title>" --body "<body>" --base <base>`
3. Return the PR URL to the user.

## Rules

- Never create a PR without the user's explicit approval of the title and body
- Always check for uncommitted changes first
- Always push before creating the PR
- If the branch has no commits ahead of the base, inform the user and stop