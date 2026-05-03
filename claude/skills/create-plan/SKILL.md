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

A complete real-world example lives next to this skill at `example-plan.md` — refer to it whenever the format is ambiguous.

**Plan Header:**

`````markdown
# [Feature Name] Implementation Plan

> **Execution:** Use `/execute-plan` to dispatch this plan to agent-teams. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** [One sentence]
**Architecture:** [2-3 sentences explaining the overall approach and how the tasks compose]
**Tech Stack:** [Key technologies / libraries / language version]

**Working directory:** `[absolute or repo-relative path]` (run all build/test commands from there).
**Branch:** `[branch-name]`.
**Baseline before Task 1:** [N tests passing, lint clean, fmt clean — engineer must verify before starting].

**Per-task verification command** (mandatory before each commit):
```sh
[exact command, e.g. `cd compiler && cargo test --quiet && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check`]
```

---
`````

**Task Format:**

Each task includes:
- **Why** — 1-3 sentence motivation (what problem this addresses, why now)
- **Files** — Create / Modify (with line ranges where applicable) / Test
- **Migration table** (optional) — used when 5+ similar sites are migrated; lists per-site parameters in tabular form
- **Helper / pattern code** (optional) — full code shown once at the top of the task, referenced by steps
- **Steps** — checkboxes; bite-sized (2-5 min each); concrete code or commands

`````markdown
### Task N: [Component Name]

**Why:** [Motivation — what problem this task addresses, why now.]

**Files:**
- Create: `exact/path/to/file.rs`
- Modify: `exact/path/to/existing.rs:123-145`
- Test: `tests/exact/path/to/test.rs`

[Optional — only if 5+ similar sites:]

| Site (file:line) | Param 1 | Param 2 | ... |
|---|---|---|---|
| ... | ... | ... | ... |

[Optional — helper or migration template, shown once:]

```rust
// Helper definition or before/after migration template
```

### Steps

- [ ] **Step 1: [Concrete action]**

[Code or instructions]

- [ ] **Step 2: Write the failing test**

```rust
#[test]
fn test_specific_behavior() {
    let result = function(input);
    assert_eq!(result, expected);
}
```

- [ ] **Step 3: Run test to verify it fails**

Run: `cargo test test_specific_behavior -- --nocapture`
Expected: FAIL with "function not defined"

- [ ] **Step 4: Write minimal implementation**

```rust
fn function(input: T) -> U { expected }
```

- [ ] **Step N-1: Verify**

```sh
[per-task verification command from header]
```

Expected: [N tests pass, lint clean, fmt clean].

- [ ] **Step N: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
[Subject line, ≤72 chars, imperative voice]

[Body explaining why, not what — 2-5 sentences]
EOF
)"
```
`````

**Final sections** (after all task definitions):

`````markdown
## Final verification (after all tasks)

```sh
[full test/lint/fmt suite + smoke test, e.g. CLI run on a sample input]
```

Expected: [total test count, all passing; lint clean; fmt clean; smoke test specific output].

## Push and PR

```sh
git push -u origin [branch-name]
gh pr create --base main --title "[PR title]" --body "..."
```

PR description should explain [what each commit does, any behavior changes, links to design doc / issue].

## Out of scope

- [Explicitly deferred items — list them so they aren't forgotten and so the PR's scope is clear]
- [Forward references to future PRs / stages]
`````

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
