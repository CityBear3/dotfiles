---
name: create-pr
description: Create a GitHub pull request with a structured description grounded in the branch diff, approved design decisions, plan, and repository template. Use only when the user asks Codex to publish a pull request.
---

# Create a pull request

Treat PR creation as an external write.

## Gather evidence

Determine the base branch from the request or repository default. Inspect:

- `git status --short`;
- commits in `<base>..HEAD`;
- `git diff --stat <base>...HEAD` and the full diff;
- relevant Design Docs and plans;
- the repository PR template, preferring `.github/pull_request_template.md`, then other conventional template locations;
- fresh verification results for the branch.

Stop if there are unintended uncommitted changes or required verification is failing.

## Draft

Follow the repository template. Otherwise include:

- Summary
- Motivation
- Design decisions
- Changes
- Verification
- Known limitations or follow-ups

Describe observed results; do not claim checks that were not run. Keep implementation narration subordinate to user-visible behavior and design rationale.

## Publish

Show the proposed title and body before the external write unless the user's request already approved the exact publication. Then use `gh pr create` with the resolved base and head.

Do not push a missing remote branch, create follow-up issues, add reviewers, or comment elsewhere without authorization.

Return the PR URL and the verification evidence included in the description.
