---
name: walkthrough-plan
description: Explain an implementation plan to the user one task at a time and advance only when the user says 「次へ」. Use for read-only plan comprehension before or during execution.
---

# Walk through a plan

This skill explains; it does not edit, approve, or execute the plan.

## Start

Resolve the requested plan path. If none is supplied, inspect `docs/plans` and select the clearly current plan; ask only when multiple plausible plans remain.

Require coordinator evidence that identifies the exact approved content and
confirms repository currentness before calling any artifact or task current and
approved. A `Status: Approved` field, file existence, Git history, or conversation
summary alone is not approval evidence. Compare the current files with the
supplied evidence and stop the approved walkthrough when a material edit or
uncertain version is found.

Without exact-content approval evidence, this skill may still explain the file
only when the user asked for an unverified draft walkthrough. Label every task
`承認未確認` and do not describe it as the current approved contract or imply that
it authorizes execution.

For new-format work, read the complete Implementation Plan, its sibling approved
Feature Contract, and any directly referenced Design Doc before explaining Task
Contract 1. Verify their approval and precedence from the artifacts rather than
conversation memory, while using the coordinator evidence above to bind approval
to their exact content.

Before Task Contract 1, explain the plan's two distinct topologies: the logical
Task dependency DAG and the PR base topology. Identify sibling PRs, stacked
chains, any fan-in linearization, tasks allowed to produce a candidate before
their final base exists, concurrency exclusions, and integration-only
compositions. Do not describe a Git stack edge as a logical dependency.

When the plan was approved and already executing before the contract-centered
format, its exact-content approval, in-flight and unchanged state is established,
no material
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
6. planned PR parent or sibling relationship, workspace, final-base readiness,
   concurrency eligibility, and staleness triggers;
7. local decisions delegated to the implementation agent;
8. contractually significant files, signatures, ordering, or commands when
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

After the final task, summarize dependency release, PR publication order,
feature acceptance and integration-only evidence, workspace-artifact lifetime,
and the remaining user-controlled publication, merge, and disposition gates.
Do not treat completing the walkthrough as plan approval.
