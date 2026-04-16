---
name: create-plan
description: |
  Decompose a Design Doc or scope-clear task into bite-sized, independently verifiable tasks.
  Each task contains complete steps (write test, implement, verify, commit) with exact file paths and code.
  The plan is self-contained for autonomous execution by agent-teams.
  Invoke with `/create-plan` when entering the planning phase.
---

# Create Plan

Decompose a Design Doc or scope-clear task into bite-sized, independently verifiable tasks. The plan must be self-contained: an agent (or fresh subagent) with zero context should be able to execute each task following only the plan.

**Announce at start:** "I'm using the create-plan skill to create the implementation plan."

## Entry Conditions

One of the following must be true:
- A Design Doc exists and has been approved by the engineer
- The scope is clear from `/design-discussion` and the engineer has approved transitioning to planning
- The engineer has explicitly requested task decomposition for a known scope

## Plan Storage

Save plans to `docs/plans/YYYY-MM-DD-<feature-name>.md`.

(Engineer's project-level conventions override this default.)

## Scope Check

If the scope covers multiple independent subsystems, suggest decomposing into separate plans — one per subsystem. Each plan should produce working, testable software on its own.

If the scope is too vague to bite-size, return to `/design-discussion`. A plan cannot resolve a vague design.

## Process

### Step 1: Understand the Scope

Read the Design Doc (if present) or gather context from the discussion. Identify what needs to be built, what exists that will be affected, and the completion criteria.

### Step 2: Map the File Structure

Before defining tasks, map out which files will be created or modified and what each is responsible for. This is where decomposition decisions get locked in.

- Design units with clear boundaries and one clear responsibility
- Files that change together should live together — split by responsibility, not by technical layer
- In existing codebases, follow established patterns

### Step 3: Decompose into Bite-Sized Tasks

Each **Task** represents a meaningful, independently verifiable unit. Each task contains **steps** — bite-sized actions (2–5 minutes each), typically: write failing test → run to verify it fails → implement minimal code → run to verify pass → commit.

Steps must include:
- **Exact file paths** (not "the test file")
- **Complete code** (not "implement the feature")
- **Exact commands** (not "run the tests")
- **Expected output** for verification steps

### Step 4: Write the Plan

Write the plan to `docs/plans/YYYY-MM-DD-<feature-name>.md` using the format below.

**Plan Header:**

```markdown
# [Feature Name] Implementation Plan

> **Execution:** Use `/execute-plan` to dispatch this plan to agent-teams. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** [One sentence]
**Architecture:** [2-3 sentences]
**Tech Stack:** [Key technologies/libraries]

---
```

**Task Format:**

````markdown
### Task N: [Component Name]

**Files:**
- Create: `exact/path/to/file.py`
- Modify: `exact/path/to/existing.py:123-145`
- Test: `tests/exact/path/to/test.py`

- [ ] **Step 1: Write the failing test**

```python
def test_specific_behavior():
    result = function(input)
    assert result == expected
```

- [ ] **Step 2: Run test to verify it fails**

Run: `pytest tests/path/test.py::test_name -v`
Expected: FAIL with "function not defined"

- [ ] **Step 3: Write minimal implementation**

```python
def function(input):
    return expected
```

- [ ] **Step 4: Run test to verify it passes**

Run: `pytest tests/path/test.py::test_name -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add tests/path/test.py src/path/file.py
git commit -m "feat: add specific feature"
```
````

### Step 5: Self-Review

After writing the complete plan, review with fresh eyes:

1. **Spec coverage:** Skim the Design Doc / discussion. Can you point to a task that implements each requirement? List any gaps.
2. **Placeholder scan:** Search for red flags (see "No Placeholders" below). Fix them.
3. **Type consistency:** Do types, method signatures, and property names match across tasks? A function called `clearLayers()` in Task 3 but `clearFullLayers()` in Task 7 is a bug.

Fix issues inline. No need to re-review — just fix and move on.

### Step 6: Engineer Review

Present the plan to the engineer. The engineer reviews task scope, order, dependencies, and whether any task needs a Design Doc first.

Do not proceed until the engineer approves the plan.

### Step 7: Transition

After the plan is approved:

→ Transition to `/execute-plan` to dispatch the plan to agent-teams.

## No Placeholders

Every step must contain the actual content an executor needs. These are **plan failures** — never write them:

- "TBD", "TODO", "implement later", "fill in details"
- "Add appropriate error handling" / "add validation" / "handle edge cases" without specifics
- "Write tests for the above" without actual test code
- "Similar to Task N" — repeat the code (the executor may read tasks out of order)
- Steps that describe what to do without showing how (code blocks required for code steps)
- References to types, functions, or methods not defined in any task

## Key Principles

- **Self-contained**: An agent with zero context should be able to execute each task following only the plan
- **DRY, YAGNI, TDD, frequent commits** — apply to both plan and resulting code
- **Engineer reviews and approves** before any task is executed
- **Bite-sized steps**: 2–5 minutes per step

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Tasks that can only be verified after multiple tasks complete | Break down further. Each task must be independently verifiable. |
| Vague steps like "implement the feature" or "add error handling" | Every step must be specific: which file, what change, what result. |
| Proceeding to /execute-plan without engineer approval | Stop. The engineer must approve the plan first. |
| Tasks too large ("Implement the entire auth system") | Decompose until each task fits in one execution session. |
| Tasks too granular ("Add import statement on line 5") | Tasks should be meaningful, verifiable units. |

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "These tasks are too small to need detail" | Detail enables autonomous execution. Without it, agent-teams blocks on questions. |
| "The executor will figure out the details" | That's what create-plan prevents. Details belong in the plan. |
| "The engineer will catch issues in review" | Review is not a substitute for self-review. Catch issues before review. |
| "The Design Doc covers this, tasks are obvious" | Obvious to you ≠ obvious to the executor. Make tasks explicit. |

## Rules

- A plan requires the engineer's approval before any task is executed
- Every task must specify completion criteria (implicit in steps) and affected files
- If a task cannot be decomposed into bite-sized verifiable steps, the design is too vague — return to `/design-discussion` or `/design-doc`
- If the Design Doc exists, the plan must be consistent with it — do not deviate without the engineer's explicit approval
