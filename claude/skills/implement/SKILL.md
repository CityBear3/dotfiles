---
name: implement
description: |
  Execute an approved implementation plan. Follows the plan faithfully, maintains discipline,
  and runs verification before reporting completion.
  Invoke with `/implement` after an implement-plan is approved.
---

# Implement — Execution

Execute the approved implementation plan faithfully.

**Announce at start:** "I'm using the implement skill to execute this task."

## Entry Conditions

- An implement-plan exists and has been approved by the engineer (or the engineer has approved skipping implement-plan for a trivial task)
- The task's executor is confirmed (Engineer or Claude Code)
- A feature branch exists (never implement directly on main/master)

## The Process

### For Claude Code Tasks

#### Step 1: Confirm Readiness

- Verify you are on a feature branch
- Verify the implement-plan is approved
- Verify any prerequisite tasks are complete

#### Step 2: Execute Steps

Follow each step in the implement-plan exactly:
- Make changes as specified
- Use `test-driven-development` skill when writing new functionality or fixing bugs
- Use `commit` skill at natural commit points (after each meaningful, verifiable unit of work)
- If a step is unclear, stop and ask the engineer — do not guess

#### Step 3: Run Verification

After all steps are complete, run verification before reporting:

1. **Build**: Ensure the code compiles without errors
2. **Test**: Run relevant tests (unit tests at minimum; integration tests if affected)
3. **Lint / Format**: Run linters or formatters if configured in the project
4. **Diff Review**: Review the final diff to confirm changes match the implement-plan

If any step fails, fix the issue and re-run from that step. Do not report completion with known failures unless the failure is pre-existing and unrelated to the current change.

#### Step 4: Report and Transition

Report completion with verification evidence. Then:

→ Transition to `verify` for formal verification (delegates to `implementation-verifier` agent).

### For Engineer Tasks

Claude Code shifts to a **support role**:
- Research: Look up relevant code, dependencies, or patterns when asked
- Answer questions: Explain existing code behavior, constraints, or conventions
- Review: Review the engineer's code when asked
- Context: Provide information from the codebase or design doc

Do not:
- Write implementation code for the engineer
- Take over implementation when the engineer is working
- Suggest "let me do this part" for subtasks within an Engineer task

When the engineer completes the task, confirm readiness to transition to `verify`.

## Implementation Discipline

- Follow the approved implement-plan faithfully. Do not make ad-hoc design changes during implementation.
- Keep changes minimal and focused. Do not refactor, add comments, or "improve" code outside the scope of the task.
- When uncertain about an implementation detail not covered by the plan, ask the engineer rather than guessing.

## Failure Tolerance

- Always work on a feature branch, never directly on main or the primary development branch.
- Rely on CI as a safety net. If CI is configured, ensure changes pass CI checks.
- If a change turns out to be wrong, prefer reverting to patching. Keep the commit history clean enough to revert safely.

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Implementing without an approved plan | Stop. Get plan approval first. |
| Making design changes during implementation | Flag the issue. Return to `implement-plan` or `design-doc` if design needs to change. |
| Writing code for an Engineer task | Shift to support role. Research, answer, review — don't write. |
| Reporting completion without running verification | Run build, test, lint, diff review. Then report with evidence. |
| "Improving" code outside the task scope | Stay within scope. No drive-by refactoring. |
| Implementing on main/master branch | Stop. Create a feature branch first. |
| Committing with known test failures | Fix failures first. Pre-existing failures must be noted explicitly. |

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "Just a small design tweak during implementation" | Design changes go through the design process. No exceptions. |
| "This code nearby could use improvement" | Out of scope. File a separate task if needed. |
| "The engineer is busy, I'll handle this Engineer task" | The task was assigned to the engineer for a reason. Wait or ask. |
| "Tests pass, no need for formal verification" | Passing tests ≠ complete verification. Run the full check. |
| "I'll add the commit later" | Commit at natural points. Atomic commits enable clean reverts. |

## Rules

- Never start implementation on main/master without explicit engineer consent
- The engineer's review is a mandatory gate before any change is merged or shared
- Claude Code must not treat its own verification as a substitute for human review
- If verification reveals issues not covered by the implement-plan, stop and consult the engineer