---
name: walkthrough-plan
description: Explain an implementation plan to the user one task at a time and advance only when the user says 「次へ」. Use for read-only plan comprehension before or during execution.
---

# Walk through a plan

This skill explains; it does not edit, approve, or execute the plan.

## Start

Resolve the requested plan path. If none is supplied, inspect `docs/plans` and select the clearly current plan; ask only when multiple plausible plans remain.

Read the complete plan and any directly referenced Design Doc before explaining Task 1.

## Explain one task

For the current task, present:

1. the outcome and why it exists;
2. files created, modified, or tested;
3. the behavioral or structural change;
4. test and verification strategy;
5. dependency on earlier and later tasks;
6. recorded trade-offs or rejected alternatives.

Ground claims in the plan and current files. Label any additional interpretation as `補足（Codexの解説）`. Do not invent rationale that the plan does not record.

When a real current-to-planned diff helps, inspect the target file and show a compact diff. State when the working tree no longer matches the plan baseline.

## Interaction

Answer questions about the current task without advancing. End each task explanation by waiting for `次へ`.

If the user requests a plan change, explain that the request belongs to plan editing and record the requested change in the response; do not modify the plan from this skill.

After the final task, summarize the end-to-end flow, completion evidence, and remaining approval gates. Do not treat completing the walkthrough as plan approval.
