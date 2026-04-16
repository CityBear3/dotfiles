---
name: verify
description: |
  Post-implementation verification skill. Delegates to the implementation-verifier agent
  for build, test, lint, diff review, and readability checks. Enforces the Iron Law:
  no completion claims without fresh verification evidence.
  Invoke with `/verify` after /execute-plan completes.
---

# Verify

Run formal post-implementation verification by delegating to the `implementation-verifier` agent. Enforce the Iron Law: evidence before claims, always.

**Announce at start:** "I'm using the verify skill to run post-implementation verification."

## Core Principle

**Evidence before claims, always.**

Claiming work is complete without fresh verification is dishonesty, not efficiency.

**Violating the letter of this rule is violating the spirit of this rule.**

## The Iron Law

```
NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE
```

If you haven't run the verification command in this session, you cannot claim it passes.

## Entry Conditions

- Implementation is complete (all tasks in the plan are completed by agent-teams, or the engineer has reported direct completion)
- The agent-teams' implementer ran tests as part of work, but those results do NOT substitute for /verify
- The engineer is ready for formal verification

## The Gate Function

Before claiming any status or expressing satisfaction:

1. **IDENTIFY**: What command proves this claim?
2. **RUN**: Execute the FULL command (fresh, complete)
3. **READ**: Full output — check exit code, count failures
4. **VERIFY**: Does output confirm the claim?
   - If NO: State actual status with evidence
   - If YES: State claim WITH evidence
5. **ONLY THEN**: Make the claim

**Skip any step = lying, not verifying.**

## Flow

### Step 1: Gather Context

Before launching the agent, collect from the current conversation:
- The approved plan (and Design Doc if any)
- The list of changed files
- The project-level CLAUDE.md path (if known)

### Step 2: Launch Agent

Use the Agent tool to launch the `implementation-verifier` agent with the gathered context. Include in the prompt:
- The plan or Design Doc content (or note that none exists)
- The scope of changes (changed files, relevant modules)
- The project root path

### Step 3: Present Results

When the agent completes, present its verification report to the engineer.

**Apply the Iron Law to the agent's report:** if the agent claims success, do not propagate the claim — verify by checking the actual evidence (test output, build exit code, diff). Trust the evidence, not the report.

If deviations from the plan or readability suggestions are found, ask the engineer how to proceed.

### Step 4: Transition

After verification passes and the engineer reviews the report:

→ Transition to `/review` for comprehensive code review.

If verification fails:

→ Return to `/execute-plan` (or restart agent-teams for the failing task) to fix issues, then re-run `/verify`.

## Common Failures

| Claim | Requires | Not Sufficient |
|-------|----------|----------------|
| Tests pass | Test command output: 0 failures | Previous run, "should pass" |
| Linter clean | Linter output: 0 errors | Partial check, extrapolation |
| Build succeeds | Build command: exit 0 | Linter passing, logs look good |
| Bug fixed | Test original symptom: passes | Code changed, assumed fixed |
| Regression test works | Red-green cycle verified | Test passes once |
| Agent completed | VCS diff shows changes | Agent reports "success" |
| Requirements met | Line-by-line checklist against plan | Tests passing |

## Key Patterns

**Tests:**
```
✅ [Run test command] [See: 34/34 pass] "All tests pass"
❌ "Should pass now" / "Looks correct"
```

**Regression tests (TDD Red-Green):**
```
✅ Write → Run (pass) → Revert fix → Run (MUST FAIL) → Restore → Run (pass)
❌ "I've written a regression test" (without red-green verification)
```

**Build:**
```
✅ [Run build] [See: exit 0] "Build passes"
❌ "Linter passed" (linter doesn't check compilation)
```

**Requirements:**
```
✅ Re-read plan → Create checklist → Verify each → Report gaps or completion
❌ "Tests pass, requirements met"
```

**Agent delegation:**
```
✅ Agent reports success → Check VCS diff → Verify changes → Report actual state
❌ Trust agent report
```

## Red Flags - STOP

- Using "should", "probably", "seems to" about verification results
- Expressing satisfaction before verification ("Great!", "Perfect!", "Done!")
- About to report completion without fresh verification
- Trusting a previous run's results
- Trusting the implementation-verifier agent's success report without checking evidence
- Relying on partial verification
- Thinking "just this once"
- Tired and wanting work over
- **ANY wording implying success without having run verification**

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "Should work now" | RUN the verification |
| "I'm confident" | Confidence ≠ evidence |
| "Just this once" | No exceptions |
| "Linter passed" | Linter ≠ compiler ≠ tests |
| "Agent said success" | Verify independently |
| "I'm tired" | Exhaustion ≠ excuse |
| "Partial check is enough" | Partial proves nothing |
| "Different words so rule doesn't apply" | Spirit over letter |
| "Implementer already ran tests" | /verify is a separate, formal gate |

## When To Apply

The Iron Law applies broadly — not just within /verify, but ALWAYS before:
- ANY variation of success/completion claims
- ANY expression of satisfaction
- ANY positive statement about work state
- Committing, PR creation, task completion
- Moving to next task
- Delegating to agents

The /verify skill is the formal application of this principle for post-implementation verification.

## Important Rules

- Always gather context before launching the agent — the agent runs in isolation and cannot access the current conversation history.
- If no plan or Design Doc exists in the current conversation, tell the agent to skip the diff review and readability check steps.
- Do not re-run the agent unless the engineer requests it after making fixes.
- The engineer's review is a mandatory gate. Verification passing does not mean the work is done — the engineer must review.

## The Bottom Line

**No shortcuts for verification.**

Run the command. Read the output. THEN claim the result.

This is non-negotiable.
