---
name: walkthrough-plan
description: Explain an implementation plan to the user one task at a time and advance only when the user says 「次へ」. Use for read-only plan comprehension before or during execution.
---

# Walk through a plan

This skill explains; it does not edit, approve, or execute the plan.

## Start

Resolve the requested plan path. If none is supplied, inspect `docs/plans` and select the clearly current plan; ask only when multiple plausible plans remain.

For new-format work, read the complete Implementation Plan, its sibling approved
Feature Contract, and any directly referenced Design Doc before explaining Task
Contract 1. Verify their approval and precedence from the artifacts rather than
conversation memory.

When the plan was approved and already executing before the contract-centered
format, its in-flight and unchanged approval state is established, no material
ambiguity exists, and the owner did not choose migration, read the legacy plan
and its referenced design sources instead. Explain its existing task sections
under that authority without requiring or manufacturing sibling contract files.

## Explain one task

For a current new-format Task Contract, present:

1. the outcome and why it exists;
2. the Feature Contract clauses it satisfies;
3. responsibility, ownership, and shared-interface boundaries;
4. protected constraints and observable verification obligations;
5. dependency on earlier and later tasks, including integration-only proof;
6. local decisions delegated to the implementation agent;
7. contractually significant files, signatures, ordering, or commands when
   present.

For a current eligible legacy task specification, instead present only the
fields its approved authority actually contains: its outcome and requirements,
scope or responsibility boundaries, constraints and verification criteria,
dependencies, and fixed implementation or commit details when present. Do not
invent Feature Contract clauses, shared interfaces, integration-only proofs, or
delegated decisions that the legacy plan did not record.

Ground claims in the plan and current files. Label any additional interpretation as `補足（Codexの解説）`. Do not invent rationale that the plan does not record.

Do not present a Task Contract as an exhaustive file recipe. Explain the
distinction between required responsibility and delegated private implementation
when it matters.

When a real current-to-planned diff helps, inspect the target file and show a compact diff. State when the working tree no longer matches the plan baseline.

## Interaction

Answer questions about the current task without advancing. End each task explanation by waiting for `次へ`.

If the user requests a plan change, explain that the request belongs to plan editing and record the requested change in the response; do not modify the plan from this skill.

After the final task, summarize the end-to-end flow, completion evidence, and remaining approval gates. Do not treat completing the walkthrough as plan approval.
