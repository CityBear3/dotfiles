---
name: implement-plan
description: |
  Create a detailed implementation plan for a single task from the plan.
  Defines the specific steps, affected files, approach, and verification method.
  Invoke with `/implement-plan` for each task before starting implementation.
---

# Implement-Plan — Implementation Planning

Create a detailed implementation plan for a single task before starting implementation.

**Announce at start:** "I'm using the implement-plan skill to create an implementation plan for this task."

## When Required

Create an implement-plan when any of the following applies:
- The task involves changes across 3 or more files
- The task requires modifying existing architecture or interfaces
- The implementation approach is not obvious from the task description
- The engineer explicitly asks for a plan

Do not create an implement-plan for straightforward, single-file changes or well-defined small tasks. In that case, transition directly to `implement` with the engineer's approval.

## Entry Conditions

- A plan exists with approved tasks and executor assignments
- The current task has been identified and its executor confirmed
- Previous tasks in the plan are complete (or the engineer approves out-of-order execution)

## The Process

### Step 1: Analyze the Task

Read the task description from the plan. Understand:
- What the task achieves
- Which files and modules are affected
- How this task relates to other tasks in the plan
- What the completion criteria are

### Step 2: Draft the Implement-Plan

Address each of these sections:

1. **Goal**: What the task achieves (one sentence)
2. **Scope**: Which files/modules are affected, with specific paths
3. **Steps**: Ordered list of verifiable sub-steps
4. **Approach**: How to implement, including key technical decisions
5. **Verification**: How to confirm the implementation is correct (specific test commands, build commands, manual checks)
6. **Risks**: Anything that could go wrong or needs the engineer's input

### Step 3: Engineer Review

Present the implement-plan to the engineer. For **Claude Code tasks**, the engineer reviews and approves. For **Engineer tasks**, Claude Code presents the plan as a suggested approach — the engineer may follow it, modify it, or discard it entirely.

Do not proceed to implementation until the engineer approves.

### Step 4: Transition

After the implement-plan is approved:

→ Transition to `implement` to execute the plan.

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Starting implementation without an approved implement-plan (when one is required) | Draft the plan first. Get approval. Then implement. |
| Implement-plan deviates from the Design Doc or approved plan | The implement-plan must be consistent. Flag deviations to the engineer. |
| Vague steps like "implement the feature" or "add error handling" | Every step must be specific: which file, what change, what the result should be. |
| Skipping the Risks section | Even if no risks are apparent, state "No identified risks" explicitly. |
| Creating an implement-plan for a trivial change | Ask the engineer if a plan is needed. Don't over-process simple tasks. |

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "The task is clear from the plan, no implement-plan needed" | If it crosses 3+ files or touches interfaces, it needs a plan. |
| "I'll figure out the details during implementation" | That's what implement-plan prevents. Details should be explicit before starting. |
| "The engineer will catch issues in review" | Review is not a substitute for planning. Catch issues before writing code. |

## Rules

- An implement-plan is a tactical artifact for a single task — not a design document
- If the implement-plan reveals that the task should be split, return to the `plan` skill
- If the implement-plan reveals a design issue, escalate to the engineer — do not make design decisions
- The implement-plan must specify concrete verification steps, not just "run tests"