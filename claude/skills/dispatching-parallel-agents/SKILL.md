---
name: dispatching-parallel-agents
description: |
  Use when facing 2+ independent tasks that can be worked on without shared state or sequential dependencies.
  Dispatches one focused agent per problem domain, running concurrently.
  Cross-cutting skill — invoke when multiple unrelated failures or tasks can be parallelized.
---

# Dispatching Parallel Agents

Delegate independent tasks to specialized agents with isolated context. Each agent gets exactly the context it needs to focus on its domain. Multiple agents work concurrently, preserving the lead's context for coordination.

**Core principle:** One agent per independent problem domain. Let them work concurrently.

**Announce at start:** "I'm using the dispatching-parallel-agents skill to investigate these in parallel."

## When to Use

**Use when:**
- 2+ test files failing with different root causes
- Multiple subsystems broken independently
- Each problem can be understood without context from others
- No shared state between investigations
- Independent research questions across different parts of the codebase

**Don't use when:**
- Failures are related (fix one might fix others) — investigate together first
- Need to understand full system state before acting
- Agents would interfere with each other (editing same files, using same resources)
- Exploratory debugging where the problem domain isn't yet clear

## The Pattern

### 1. Identify Independent Domains

Group failures or tasks by what's being addressed:
- Domain A: Tool approval flow (file X)
- Domain B: Batch completion behavior (file Y)
- Domain C: Abort functionality (file Z)

Each domain is independent — fixing one doesn't affect another.

### 2. Create Focused Agent Tasks

Each agent gets:
- **Specific scope**: One test file or subsystem
- **Clear goal**: e.g., "Make these tests pass"
- **Constraints**: e.g., "Don't change other code"
- **Expected output**: Summary of what was found and fixed

### 3. Dispatch in Parallel

In Claude Code, send multiple Agent() calls in **one message** to run concurrently:

```
Agent({ subagent_type: "general-purpose", description: "Fix abort tests", prompt: ... })
Agent({ subagent_type: "general-purpose", description: "Fix batch tests", prompt: ... })
Agent({ subagent_type: "general-purpose", description: "Fix race tests", prompt: ... })
```

All three run simultaneously.

### 4. Review and Integrate

When agents return:
- Read each summary
- Check for conflicts (did agents edit the same code?)
- Run full test suite to verify fixes work together
- Spot check for systematic errors

## Agent Prompt Structure

Good agent prompts are:

1. **Focused** — One clear problem domain
2. **Self-contained** — All context needed; agents don't see your conversation
3. **Specific about output** — What should the agent return?

Example:

```
Fix the 3 failing tests in src/agents/agent-tool-abort.test.ts:

1. "should abort tool with partial output capture" - expects 'interrupted at' in message
2. "should handle mixed completed and aborted tools" - fast tool aborted instead of completed
3. "should properly track pendingToolCount" - expects 3 results but gets 0

These appear to be timing/race condition issues. Your task:

1. Read the test file and understand what each test verifies
2. Identify root cause — timing issues or actual bugs?
3. Fix by:
   - Replacing arbitrary timeouts with event-based waiting
   - Fixing bugs in abort implementation if found
   - Adjusting test expectations if testing changed behavior

Do NOT just increase timeouts — find the real issue.

Return: Summary of what you found and what you fixed.
```

## Common Mistakes

| ❌ Avoid | ✅ Instead |
|---|---|
| "Fix all the tests" (too broad) | "Fix tests in file X" (specific scope) |
| "Fix the race condition" (no context) | Paste exact error messages and test names |
| No constraints (agent refactors everything) | "Do NOT change production code" |
| "Fix it" (vague output) | "Return summary of root cause and changes" |

## Verification

After agents return:

1. **Review each summary** — Understand what changed
2. **Check for conflicts** — Did agents edit the same code?
3. **Run full suite** — Verify all fixes work together
4. **Spot check** — Agents can make systematic errors; sample-check the changes

## Brief Example

**Scenario**: 6 test failures across 3 files after a major refactoring.

| Domain | File | Failures |
|---|---|---|
| Abort logic | agent-tool-abort.test.ts | 3 |
| Batch completion | batch-completion-behavior.test.ts | 2 |
| Race conditions | tool-approval-race-conditions.test.ts | 1 |

These domains are independent. Dispatch 3 agents in parallel, one per domain. Time-to-resolution: ~equivalent to longest single investigation, not the sum.

## Integration

**Cross-cutting skill** — invoked from any skill or directly when independent parallel work is identified.

Common contexts:
- During `/systematic-debugging` when multiple unrelated failures surface
- During `/agent-teams-driven-development` for parallel review of independent task groups
- During `/design-discussion` for parallel exploration of independent design questions
