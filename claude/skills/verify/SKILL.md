---
name: verify
description: |
  Post-implementation verification skill. Delegates to the implementation-verifier agent
  for build, test, lint, diff review, and readability checks.
  Invoke with `/verify` after completing an implementation.
---

# Verify

Run post-implementation verification by delegating to the `implementation-verifier` agent.

**Announce at start:** "I'm using the verify skill to run post-implementation verification."

**Core principle:** Evidence before claims, always.

## Entry Conditions

- Implementation is complete (all steps in the implement-plan are done)
- The `implement` skill's inline verification has passed (build, test, lint, diff)
- The engineer is ready for formal verification

## Flow

### Step 1: Gather Context

Before launching the agent, collect the following from the current conversation:
- The approved plan or Design Doc (if any)
- The list of changed files
- The project-level CLAUDE.md path (if known)

### Step 2: Launch Agent

Use the Agent tool to launch the `implementation-verifier` agent with the context gathered in Step 1.
Include in the prompt:
- The plan or Design Doc content (or a note that none exists)
- The scope of changes (changed files, relevant modules)
- The project root path

### Step 3: Present Results

When the agent completes, present its verification report to the user.
If deviations from the plan or readability suggestions are found, ask the user how to proceed.

### Step 4: Transition

After verification passes and the engineer reviews the report:

→ Transition to `review` for comprehensive code review.

If verification fails:
→ Return to `implement` to fix issues, then re-run `verify`.

## The Iron Law

```
NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE
```

If you haven't run the verification command in this session, you cannot claim it passes.

## Verification Checklist

| Claim | Requires | Not Sufficient |
|-------|----------|----------------|
| Tests pass | Test command output: 0 failures | Previous run, "should pass" |
| Linter clean | Linter output: 0 errors | Partial check, extrapolation |
| Build succeeds | Build command: exit 0 | Linter passing, logs look good |
| Bug fixed | Test original symptom: passes | Code changed, assumed fixed |
| Requirements met | Line-by-line checklist | Tests passing |

## Red Flags

| Violation | Correct Behavior |
|-----------|-----------------|
| Using "should", "probably", "seems to" about verification results | Run the command. Report actual output. |
| Expressing satisfaction before verification ("Great!", "Done!") | Verify first, then report with evidence. |
| About to report completion without fresh verification | Run the verification commands now. |
| Trusting a previous run's results | Re-run. Fresh evidence only. |
| Relying on partial verification | Run the full verification suite. |

## Rationalization Prevention

| Excuse | Reality |
|--------|---------|
| "Should work now" | RUN the verification. |
| "I'm confident" | Confidence ≠ evidence. |
| "Just this once" | No exceptions. |
| "Linter passed" | Linter ≠ compiler ≠ tests. |
| "Partial check is enough" | Partial proves nothing about the whole. |

## Important Rules

- Always gather context before launching the agent — the agent runs in isolation and cannot access the current conversation history.
- If no plan or Design Doc exists in the current conversation, tell the agent to skip the diff review and readability check steps.
- Do not re-run the agent unless the user requests it after making fixes.
- The engineer's review is a mandatory gate. Verification passing does not mean the work is done — the engineer must review.