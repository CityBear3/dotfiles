---
name: plan
description: |
  Decompose a Design Doc or large task into small, independently verifiable tasks with executor assignments.
  Each task is assigned to either the Engineer or Claude Code based on Division of Responsibility.
  Invoke with `/plan` when a Design Doc is ready or a task needs decomposition.
---

# Plan — Task Decomposition

Break down the Design Doc or large task into small, independently verifiable units with executor assignments.

**Announce at start:** "I'm using the plan skill to decompose this into tasks."

## Entry Conditions

One of the following must be true:
- A Design Doc exists and has been approved by the engineer
- The engineer has explicitly requested task decomposition for a known scope
- Transitioned from `design-doc` or `investigate` with the engineer's approval

## The Process

### Step 1: Understand Scope

Read the Design Doc or gather context about the task. Identify:
- What needs to be built or changed
- What already exists that will be affected
- What the completion criteria are

### Step 2: Decompose into Tasks

Each task must:
- Be completable in a single conversation or implement-plan session
- Have clear completion criteria that can be verified (test passes, build succeeds, behavior confirmed)
- Not depend on unfinished tasks to be verified

### Step 3: Assign Executors

Each task must specify an **executor** — either **Engineer** or **Claude Code**.

**Engineer tasks**: Tasks where the engineer's judgment, understanding, or knowledge-building is essential.
- Core algorithms, security-critical logic
- Core logic where lack of understanding would impair future maintenance
- Areas unfamiliar to the engineer where writing builds essential knowledge
- Design decisions that emerge during implementation

**Claude Code tasks**: Tasks where the design is settled, the pattern is clear, and the result is verifiable.
- Implementing well-specified interfaces
- Writing tests for defined behavior
- Boilerplate, configuration, wiring
- Refactoring with clear before/after states

### Step 4: Present the Plan

Present the task list in this format:

```
## Plan: <title>

### Task 1: <title>
**Executor:** Engineer / Claude Code
**Files:** <affected files>
**Completion criteria:** <how to verify this task is done>
**Description:** <what needs to be done>

### Task 2: <title>
...
```

### Step 5: Engineer Review

The engineer reviews and adjusts:
- Task scope and boundaries
- Executor assignments
- Order and dependencies
- Whether any task needs a Design Doc first

Do not proceed until the engineer approves the plan.

### Step 6: Transition

After the plan is approved, for each task in order:

→ Transition to `implement-plan` to create a detailed implementation plan for the task.

For Engineer tasks, Claude Code shifts to support role during `implement-plan` and `implement`.

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Assigning an Engineer task to Claude Code because "it's faster" | Follow Division of Responsibility. Speed is not the criterion — judgment is. |
| Creating tasks that can only be verified after multiple tasks complete | Break down further. Each task must be independently verifiable. |
| Skipping executor assignment | Every task must have an explicit executor. |
| Proceeding to implement-plan without plan approval | Stop. The engineer must approve the plan first. |
| Making a task too large ("Implement the entire auth system") | Decompose until each task is completable in one session. |
| Making a task too granular ("Add import statement on line 5") | Tasks should represent meaningful, verifiable units of work. |

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "This is clearly a Claude Code task, no need to mark it" | Mark every task. The engineer may disagree with the assignment. |
| "The engineer can always change assignments later" | Get it right in the plan. Late reassignment wastes implementation work. |
| "These tasks are too small to need a plan" | If the work was directed here, it needs a plan. The engineer decides what's too small. |
| "The Design Doc covers this, tasks are obvious" | Obvious to you ≠ obvious to the engineer. Make tasks explicit. |

## Rules

- A plan requires the engineer's approval before any task is implemented
- Every task must have an executor, completion criteria, and affected files
- If a task cannot be decomposed into independently verifiable units, suggest writing a Design Doc first
- If the Design Doc already exists, the plan must be consistent with it — do not deviate without the engineer's explicit approval