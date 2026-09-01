---
name: create-pr
description: Create an authorized GitHub Task PR against its approved base with a structured description grounded in exact accepted publication evidence, without rerunning verification or review.
---

# Create a pull request

Treat PR creation as an external write.

## Gather evidence

Resolve one exact publication authority: a planned Task Contract and PR topology
entry; a recoverable lightweight combined Feature/Task Contract and its accepted
planned base; or an eligible legacy task authority and its approved base. Do not
substitute the repository default for a resolved base. Inspect:

- `git status --short`;
- the exact head branch and object, planned base branch and object, merge base,
  and current parent PR state when stacked;
- commits in `<base>..HEAD`;
- `git diff --stat <base>...HEAD` and the full diff;
- relevant planned Design Doc, Feature Contract, Implementation Plan, and Task
  Contract content and original paths, including an exact captured publication
  handoff after an authorized prior workspace-lifecycle action removed their
  source paths; the complete
  lightweight combined contract and original request authority; or the exact
  eligible legacy sources;
- for planned or lightweight work, current internal `Accepted` verification,
  review, and triage evidence for that exact PR range;
- for eligible legacy work, current verification, review, and triage evidence
  satisfying its unchanged approved completion criteria, without requiring a
  new internal `Accepted` state;
- the repository PR template, preferring `.github/pull_request_template.md`, then other conventional template locations.

Treat the accepted publication handoff as the source of verification, review,
and triage evidence. Re-resolve the base, head, merge base, range, and status to
confirm that the handoff still names the current target. Entering the
publication phase or the passage of time does not by itself make unchanged
evidence stale.

Do not invoke `verify`, `review`, or `receiving-code-review`, and do not rerun
their checks solely to create the PR. Git identity and status checks in this
skill are publication preflight checks, not a new Acceptance gate. If the
publication target or its controlling authority changed, or required evidence
is missing, stop with the exact mismatch and return it to the owning workflow;
do not recreate Acceptance evidence inside `create-pr`.

Stop if the branch, base, range, ancestry, applicable accepted or legacy
completion evidence, or status differs from the approved publication target. A
candidate or stale task is not publishable. Do not push a missing branch,
retarget a PR, or restack history from this skill.

Do not require planned artifact files from lightweight work or manufacture the
new Task PR topology for eligible legacy work.

For planned work, normally use the ignored contract and plan artifacts while
their coordination worktree exists. If an authorized prior workspace-lifecycle
action already removed those sources, require the exact captured contract
content and topology evidence from `finish-branch`; do not reconstruct their
content from memory.

## Draft

Follow the repository template. Otherwise include:

- Summary
- Motivation
- Design decisions
- Authority and planned or approved base
- Changes
- Verification
- Known limitations or follow-ups

Describe the accepted observed results; do not claim checks that were not run
or present publication preflight checks as verification. Keep implementation
narration subordinate to user-visible behavior and design rationale.

## Publish

Show the proposed title, exact base and head, and body before the external write
unless the user's request already approved those exact publication values. Then
use `gh pr create` with explicit resolved `--base` and `--head` arguments.

Do not push a missing remote branch, create follow-up issues, add reviewers, or comment elsewhere without authorization.

Return the PR URL, base and head, stack parent when applicable, and the
verification and review evidence included in the description. PR creation does
not remove Feature Contract or Implementation Plan artifacts, remove their
worktree, or grant merge authority.
