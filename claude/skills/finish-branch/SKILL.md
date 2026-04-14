---
name: finish-branch
description: |
  Complete development work on a branch. Verifies tests pass, presents structured options
  (create PR, merge locally, keep as-is, discard), and handles cleanup.
  Invoke with `/finish-branch` after review is complete.
---

# Finish Branch

Guide completion of development work by presenting clear options and handling the chosen workflow.

**Announce at start:** "I'm using the finish-branch skill to complete this work."

**Core principle:** Verify tests → Present options → Execute choice → Clean up.

## Entry Conditions

- `review` has been completed and the engineer is satisfied with the results
- Or the engineer explicitly decides to finish the branch at any point

## The Process

### Step 1: Verify Tests

Before presenting options, verify tests pass:

```bash
# Run project's test suite (check project CLAUDE.md for specific commands)
```

If tests fail:
```
Tests failing (<N> failures). Must fix before completing:
[Show failures]
Cannot proceed with merge/PR until tests pass.
```
Stop. Do not proceed to Step 2.

If tests pass: Continue to Step 2.

### Step 2: Determine Base Branch

```bash
git merge-base HEAD main 2>/dev/null || git merge-base HEAD master 2>/dev/null
```

Or ask: "This branch split from main — is that correct?"

### Step 3: Present Options

Present exactly these 4 options:

```
Implementation complete. What would you like to do?

1. Create a Pull Request
2. Merge back to <base-branch> locally
3. Keep the branch as-is (I'll handle it later)
4. Discard this work

Which option?
```

Do not add explanation — keep options concise.

### Step 4: Execute Choice

#### Option 1: Create a Pull Request

→ Invoke `create-pr` skill.

#### Option 2: Merge Locally

```bash
git checkout <base-branch>
git pull
git merge <feature-branch>
# Verify tests on merged result
git branch -d <feature-branch>
```

#### Option 3: Keep As-Is

Report: "Keeping branch `<name>`. You can return to it later."

#### Option 4: Discard

**Confirm first:**
```
This will permanently delete:
- Branch <name>
- All commits: <commit-list>

Type 'discard' to confirm.
```

Wait for exact confirmation. If confirmed:
```bash
git checkout <base-branch>
git branch -D <feature-branch>
```

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Proceeding with failing tests | Stop. Fix tests first. |
| Merging without verifying tests on the merged result | Run tests after merge before reporting success. |
| Deleting work without confirmation | Require typed "discard" confirmation for Option 4. |
| Auto-selecting an option | Always present the 4 options. The engineer chooses. |
| Force-pushing without explicit request | Never force-push unless the engineer explicitly asks. |

## Rules

- Always verify tests before offering options
- Present exactly 4 options — no more, no less
- Get typed confirmation for discard (Option 4)
- For Option 1, delegate to `create-pr` skill — do not duplicate its logic
- For Option 2, verify tests pass on the merged result before deleting the feature branch