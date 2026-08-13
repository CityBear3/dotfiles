---
name: execute-task
description: Execute and accept one path-neutral implementation task with one writer, exact evidence, a policy-selected gate, and bounded correction.
---

# Execute one task

Own implementation and acceptance of exactly one lightweight task, approved-plan
task, or bounded correction. Do not select a workflow path, schedule plan
dependencies, run global verification or final review, publish, merge, or choose
branch disposition from this skill.

## Require one task handoff

Before implementation, require one concise plain-language handoff containing
either the new contract form or an explicitly eligible legacy form.

For the new form require:

- the approved Feature Contract or lightweight in-memory Feature Contract and
  its clauses assigned to this task;
- the exact Task Contract, or the same combined contract for lightweight work;
- applicable shared interfaces, adjacent-task obligations, protected
  constraints, and delegated local decisions;
- the separate Review context and complete active Review policy;
- the required discipline and applicable repository guidance;
- approved workspace and working directory;
- the exact task base commit;
- responsibility and ownership boundaries;
- verification routes and observable obligations;
- the responsibility-scoped commit intent and its fixed message or approved
  writer message-selection authority;
- contractually significant files, signatures, ordering, and exact commands
  only when the contract fixes them.

For a plan already executing before the contract-centered format, accept its
approved task specification and referenced design sources as the authority only
when the coordinator establishes unchanged approval, in-flight status, no
material ambiguity, and no owner migration choice. Require its available scope,
non-goals, discipline, responsibility or file boundaries, commit intent,
verification, Review context and policy, workspace, and exact task base. Do not
manufacture Feature or Task Contract artifacts or silently fill a material gap.

The Review context describes the artifact, purpose, consumers, interpretation or
execution model, material quality criteria and realistic failures, approved
non-problems, and inapplicable assumptions. The Review policy records mode,
rationale, risk surfaces, per-task gate, final required and conditional
perspectives, skips with reasons, residual risk, capacity and queue rules, and
the common Acceptance threshold.

Reject missing, stale, contradictory, or mode-inconsistent input. Return the
named gap to the invoking skill; do not infer a decision, expand scope, duplicate
the handoff in a new wrapper, or weaken evidence.

Stop with `Escalate` when implementation needs a new or changed goal, scope,
responsibility owner, public or shared interface semantic, invariant, material
failure behavior, compatibility promise, verification obligation, Review
policy, or authority. A newly discovered private file or local interface inside
the approved responsibility is not a deviation by itself; an unexpected owner
or shared seam is.

Require the task base to resolve, equal the starting head for fresh work, and
remain an ancestor of every head used as evidence. Recheck it after commits and
before acceptance. On failure, preserve state and return `BLOCKED`; never rewrite
history to manufacture ancestry.

## Choose one writer

Keep exactly one writer: the lead when direct execution is authorized, otherwise
one `implementer`. Resolve the role before loading its prompt: use the named
profile when available, or
[implementer-prompt.md](../agent-teams-driven-development/implementer-prompt.md)
as its fallback. Pass only the selected role and task handoff to
`agent-teams-driven-development`.

Require production behavior changes to use red, green, refactor and report the
observed red failure. For content, configuration, refactoring, or mechanical
migrations, apply the declared discipline and preserve the relevant green
baseline. Preserve unrelated changes.

Inside the Task Contract, let the writer choose private files, helpers, local
types and interfaces, algorithms, edit order, applicable standard verification
commands, and additional focused non-destructive checks. Require every actual
choice and changed file to remain within the approved responsibility and be
reported with evidence.

Require the writer to report:

- `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED`, or `NEEDS_CONTEXT`;
- changed files and implemented behavior;
- every command, why it was required or selected, and its expected and observed
  result;
- pre-commit diff inspection and self-review;
- when complete, the commit and new head;
- concerns and every known gap.

Interpret the report as follows:

- `DONE` advances only after the requested commit, current-state checks,
  verification, report, and ownership evidence all agree. It is not task
  acceptance by itself.
- `DONE_WITH_CONCERNS` requires classification of every concern as an authorized
  correction, operational `BLOCKED` gap, or user-owned `Escalate` decision.
- `BLOCKED` preserves the operational gap and observed state.
- `NEEDS_CONTEXT` is `BLOCKED` when the missing input is safely discoverable
  within current authority, otherwise `Escalate`.

After any incomplete response, partial edit, partial commit, interruption, or
lost response, do not start or replace a writer until the prior writer is
confirmed inactive.

## Produce current task evidence

For a fresh task:

1. Record the exact task base and starting status.
2. Implement only the declared scope with the selected discipline.
3. Run every contractually required exact command, select applicable standard
   and focused checks, and record all observed results.
4. Inspect the working-tree diff and Task Contract coverage, including unrelated
   state and actual changed files.
5. Correct concrete in-scope failures while contract meaning remains unchanged.
6. Create only the declared responsibility-scoped task commit.
7. Record the new current head.
8. Inspect the exact task-base-to-current-head range and diff.
9. Run the policy-selected per-task gate against that current range.
10. Apply the common Acceptance threshold and record the contract observations,
    commit, range, verification, gate, concerns, and gaps.

Approval remains attached to the exact task base, current head, and range that
were reviewed. Never replace them with a later aggregate range.

## Resume only safe attributable state

Before resuming after an interruption:

1. confirm the prior writer is inactive and no writer overlaps;
2. inspect the current HEAD, status, commits, and task-base-to-current diff;
3. attribute all in-scope edits and commits to this task;
4. confirm the original handoff still applies.

When all checks pass, continue from the observed state. If implementation is
already committed and its verification remains fresh for that unchanged head,
resume only the pending read-only gate; do not create a duplicate commit. If any
check is uncertain, preserve all state and return `BLOCKED` with the observed
agent and Git evidence plus the exact re-entry condition. Never clean, reset,
rebase, amend, discard, or silently restart to force progress.

Use `Escalate` only when resumption requires a material architecture, goal,
scope, responsibility, public or shared interface, invariant, verification,
policy, or authority decision.

## Give reviewers direct current evidence

Every task reviewer receives, without another identity or duplicate record:

- the Feature Contract, assigned clauses, exact Task Contract, shared interfaces,
  constraints, non-goals, and delegated decisions, or the exact eligible legacy
  task authority and its referenced design sources;
- the Review context and active Review policy;
- working directory, task base, current head, exact range, and inspected diff;
- responsibility boundaries and actual changed files;
- the complete writer report;
- every verification obligation and fresh required or selected command with its
  expected and observed result;
- commits, pre-commit inspection, repository guidance, concerns, and gaps.

Before dispatch, apply the ancestry invariant above and confirm that HEAD, range,
changed files, inspected diff, and post-edit verification still agree. Missing,
contradictory, or stale evidence returns `BLOCKED`.

## Load only the selected review contract

For every selected reviewer, use its named profile when available; otherwise load
only its corresponding fallback:

- For `focused`, select `code-reviewer` for one combined
  specification-and-quality gate, with
  [focused-reviewer-prompt.md](../agent-teams-driven-development/focused-reviewer-prompt.md)
  as fallback. An approved no-agent policy may instead use the lead.
- For `adaptive` and `deep`, select independent `spec-reviewer` and
  `code-quality-reviewer` roles, with
  [spec-reviewer-prompt.md](../agent-teams-driven-development/spec-reviewer-prompt.md)
  and
  [code-quality-reviewer-prompt.md](../agent-teams-driven-development/code-quality-reviewer-prompt.md)
  as their fallbacks.

Never load unselected prompts or replace `adaptive` or `deep` independent
reviewers with lead passes. Queue a selected role when capacity is temporarily
unavailable. Return `BLOCKED` when a required role cannot be established, or
`Escalate` when a no-agent instruction conflicts with the approved gate.

`agent-teams-driven-development` schedules only the selected contracts. This
skill remains responsible for the task meaning, review mode, finding
normalization, Acceptance, corrections, and acceptance.

## Apply the common finding threshold

Specification findings use `Must Fix` or `Should Improve`. For `adaptive` and
`deep`, map an evidence-qualified quality `Critical` to `Must Fix` and
`Important` to `Should Improve`; do not promote lower labels or non-findings.

Keep only findings that apply to the Review context, identify a concrete
reachable behavior or approved-contract violation, cite evidence, state a
material consequence, and propose a proportionate correction. `Should Improve`
requires a concrete maintainability consequence or measurable repeated cost.
Drop preference-only, speculative, unsupported, inapplicable, or already-decided
objections without materially new evidence.

## Correct and re-review without an open-ended loop

For each authorized correction, retain the exact concrete finding or failed
observation and every observed correction attempt. Give the existing writer only
the bounded correction, unchanged Feature and Task Contracts with shared
interfaces or unchanged eligible legacy task authority, Review context, Review
policy, current task base, responsibility boundaries, and verification
obligations.

Then:

1. run fresh contractually required and selected task verification;
2. inspect the correction diff;
3. create only the declared correction commit;
4. record the new current head;
5. inspect the updated exact task-base-to-head range;
6. rerun the same complete policy-selected gate against that range.

Do not reuse stale verification, approval, head, or range. If the same concrete
problem repeats without progress or another action would repeat an observed
failed correction, stop with `Escalate` and report the attempts and remaining
gap. Do not create another identifier or tracking schema for the finding.

## Return task acceptance

Return:

- `Accepted` only when the ancestry invariant holds, every contractually fixed
  exact command and selected check passes, every observable Task Contract
  obligation or eligible legacy task criterion has current evidence, and the
  complete selected gate approves the current head;
- `BLOCKED` when a safe writer state, command, permission, range, reviewer, or
  other operational prerequisite cannot be established;
- `Escalate` for a material decision, scope or policy change, explicit
  independent-gate/no-agent conflict, plan deviation, or repeated correction
  without progress.

Include writer status, task and correction commits, exact task base, current
head, exact range, changed files, commands and observed results, pre-commit
inspection, gate result, capacity or queue evidence, concerns, gaps, and exact
re-entry condition. Return this evidence to the invoking coordinator or
`execute-plan`; do not advance another task or cross-phase gate.
