---
name: execute-plan
description: Execute an approved implementation plan on a feature branch, normally through the bounded agent-teams workflow and then fresh verification. Use after plan approval; honor an explicit user request to execute directly without subagents.
---

# Execute an approved plan

## Entry

Confirm:

- the plan is approved and current;
- the working tree is on the intended non-default feature branch or workspace;
- unresolved dirty changes are understood;
- baseline and required tools are available.

If the current checkout is unsuitable, use `create-workspace`. Stop for missing design decisions rather than choosing them during execution.

## Choose execution mode

- Default: use `agent-teams-driven-development`.
- If the user explicitly says not to use agents, execute each task directly in this session with the same task, commit, and verification boundaries.

Do not spawn agents contrary to the user's current instruction.

## Execute

Follow tasks in dependency order. Apply the discipline declared by each task. Preserve unrelated changes and do not add speculative work.

For every task:

1. establish the task's starting state;
2. implement only its declared scope;
3. run its exact verification;
4. inspect the diff;
5. create the declared focused commit;
6. record evidence and any gap.

If the plan must change, stop and return to the user. If the same verification failure survives two causal fix attempts, stop with the evidence.

## Transition

After all tasks complete, use `verify`. Do not claim the plan is complete from edits or commits alone. Publication and branch disposition remain separate user-controlled actions.
