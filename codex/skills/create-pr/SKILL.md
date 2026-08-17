---
name: create-pr
description: Create an authorized GitHub Task PR against its approved planned base with a structured description grounded in exact Task Contract, range, and verification evidence.
---

# Create a pull request

Treat PR creation as an external write.

## Gather evidence

Resolve the exact Task Contract and planned PR topology entry. Do not substitute
the repository default for a planned base. Inspect:

- `git status --short`;
- the exact head branch and object, planned base branch and object, merge base,
  and current parent PR state when stacked;
- commits in `<base>..HEAD`;
- `git diff --stat <base>...HEAD` and the full diff;
- relevant Design Doc, Feature Contract, Implementation Plan, and Task Contract;
- current internal `Accepted` verification and review evidence for that exact
  PR range;
- the repository PR template, preferring `.github/pull_request_template.md`, then other conventional template locations;
- fresh verification results for the branch.

Stop if the branch, base, range, ancestry, accepted evidence, or status differs
from the approved publication target. A candidate or stale task is not
publishable. Do not push a missing branch, retarget a PR, or restack history from
this skill.

## Draft

Follow the repository template. Otherwise include:

- Summary
- Motivation
- Design decisions
- Task Contract and planned base
- Changes
- Verification
- Known limitations or follow-ups

Describe observed results; do not claim checks that were not run. Keep implementation narration subordinate to user-visible behavior and design rationale.

## Publish

Show the proposed title, exact base and head, and body before the external write
unless the user's request already approved those exact publication values. Then
use `gh pr create` with explicit resolved `--base` and `--head` arguments.

Do not push a missing remote branch, create follow-up issues, add reviewers, or comment elsewhere without authorization.

Return the PR URL, base and head, stack parent when applicable, and the
verification and review evidence included in the description. PR creation does
not retire Feature Contract or Implementation Plan artifacts or grant merge
authority.
