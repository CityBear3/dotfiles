---
name: agentic-engineering-workflow
description: Select the workflow and next phase for engineering investigation, design, implementation, or completion.
---

# Agentic engineering workflow

Own path classification and cross-phase transitions only. Let each phase skill
own investigation, task execution, plan orchestration, scheduling, verification,
review, triage, and publication mechanics. Follow repository guidance and
explicit user instructions when they are stricter.

Treat `verify`, `review`, and `receiving-code-review` as check-only phases. They
return evidence or classifications and never edit tracked state, commit a fix, or
advance the workflow. This coordinator consumes their results and selects the
next phase.

## Classify the request

Inspect the relevant repository state before selecting a route.

- For an explanation, diagnosis, review, planning, or other read-only request,
  inspect and report without implementing.
- For an explicit change request, use the lightweight path only when its complete
  eligibility contract holds. Otherwise use the planned path.
- Honor a request to skip a phase or avoid agents only when every remaining
  approved contract can still be satisfied. Never invent a user-owned decision
  or silently weaken evidence.

For every transition retain:

- the active path and phase;
- approved scope, non-goals, and the exact alignment source with its approval
  state and currentness;
- the applicable Design Doc or decision record and the Feature Contract's
  source, approval state, storage form, and currentness;
- for planned work, the living decision record location, shared working-model
  alignment state, Design Readiness result, unresolved understanding questions,
  unresolved design branches, and any discovery re-entry evidence;
- the Review context and complete active Review policy;
- for planned work, the Task dependency DAG, PR topology, task workspaces,
  accepted and candidate results, stale descendants, and integration-only
  composition;
- the next automatic action or user-controlled gate;
- the evidence required to leave the phase;
- every unresolved condition that prevents a safe transition.

## Read the relevant route

Read only the references needed for the current decision. The rules in these
references belong to this coordinator; phase skills retain their own mechanics.
Do not load all routes or references at entry.

- **Read-only explanation, diagnosis, or planning:** investigate and report
  within the requested scope. No execution or acceptance reference is needed
  solely for an explanation. For an explicit verification or independent review,
  read [standalone.md](references/standalone.md), then use `verify` or `review`.
- **Explicit change:** read [lightweight.md](references/lightweight.md) to check
  the complete eligibility contract. It also owns lightweight preparation and
  the combined handoff. Use that route only when every condition holds.
- **Planned work:** when lightweight eligibility fails, or the request enters
  or resumes planned work, read [planned.md](references/planned.md). It owns
  Design Readiness, approval transitions, promotion and legacy continuation.
  Design discussion alone does not authorize implementation.
- **Execution or gate recovery:** before entering an authorized implementation,
  correction, dependency-release or acceptance loop, read
  [execution.md](references/execution.md). It owns shared execution constraints,
  recoverable gaps, Task Accepted consumption and Feature/integration acceptance.
- **Evidence decisions:** before preparing verification/review or deciding
  whether an existing result applies, read [evidence.md](references/evidence.md).
  It owns coverage reuse, invalidation and advancing-base comparisons.
- **Completion or publication:** before finishing a branch, preparing publication
  or responding to a concern after loop completion, read
  [completion.md](references/completion.md). A completed loop's concern boundary
  also applies when following an execution or evidence reference.
- **Reusable discovery:** read [discovery-sharing.md](references/discovery-sharing.md)
  only when costly findings may be reused across independent planned sessions.

## Continue within authority

Continue authorized local implementation, verification, review and in-scope
correction without repeated approval prompts. A phase's `BLOCKED` result or a
historical procedural gap returns to the owning loop for classification and
available recovery under [execution.md](references/execution.md#recover-within-the-authorized-loop);
it does not automatically end the task or request engineer intervention.

Hold the affected transition when a user-owned decision, missing authority,
material deviation or scope expansion remains after authorized recovery.
Publication, external writes, destructive actions and branch disposition retain
their approval boundaries. Read the completion reference before reopening a
completed loop. Do not change approved topology, model allocations or quality
policy while reorganizing a handoff or resuming work.
