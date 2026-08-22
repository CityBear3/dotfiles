---
name: inject-project-rules
description: Capture one feature's tacit project norms (review rejections, code conventions with exemplars, test cases to add and to deliberately omit) into a workspace-only project-rules.md that the Implementation Plan references by identifier. User-invoked only; proof of concept.
argument-hint: "[feature plan directory (optional)]"
disable-model-invocation: true
---

# Inject project rules

Make explicit, for one feature, the norms a human absorbs from reading the
project's existing code and an agent does not: what the project's reviewers
reject, how its code is shaped, which test cases it writes and which it
deliberately does not write. Record them once as `project-rules.md` so that
the plan, the implementer, and the reviewers are bound to the same norms.

This skill is invoked only by the engineer. It never runs automatically, is
never invoked by the coordinator, and does not edit any plan, contract, or
workflow skill. It produces exactly one artifact.

## Preconditions

Require:

- a planned-path feature whose Feature Contract is approved, with its plan
  directory `docs/plans/YYYY-MM-DD-<feature>/` present in the coordination
  worktree (`$ARGUMENTS` may name the directory; otherwise resolve the current
  feature's directory);
- read access to the project's existing code and tests.

The default moment is after Feature Contract approval and before
`/create-plan`, so the plan is born referencing the rules. Running it after a
plan draft is fine: the engineer then asks `create-plan` to revise the plan's
references. Running it after plan approval makes the added references an
ordinary plan change that invalidates approval. The lightweight path has no
plan file and is out of scope.

## Investigate before asking

Read the Feature Contract to learn which layers and responsibilities the
feature touches. Then read the project's existing code and tests for those
layers, using read-only subagents (Agent tool, no `name`, foreground,
`model: "sonnet"`, report-only) where the surface is large. Collect:

- candidate exemplar files per layer: the existing implementation closest to
  what each responsibility will add, and the existing tests closest to the
  tests it will need;
- the case categories present in existing tests for each layer and how those
  tests are written (fixture construction, naming, assertion granularity,
  file layout, use of test doubles or containers);
- generic test categories that are absent from existing tests for the layer —
  argument-order permutations, exhaustive parameter combinations, unicode and
  locale inputs, concurrency, large inputs, and similar — as omission
  candidates;
- patterns the repository guidance, review history, or code comments show to
  be rejected, and invariants the code enforces everywhere (authorization
  checks, logging restrictions, error mapping).

Do not ask the engineer anything this investigation answers.

## Discuss one topic at a time

Walk the engineer through the candidates the way `design-discussion` walks a
decision tree: one topic per turn, with the evidence and a recommendation, and
room to discuss before recording. Cover, in order:

1. exemplar files per layer — confirm or replace each candidate;
2. review rejections and invariants — confirm each candidate and add what the
   engineer remembers from team review that the code does not show;
3. code conventions per layer beyond the exemplars — only what a reader of the
   exemplar would still get wrong;
4. test cases to add per layer — the viewpoints the project actually covers;
5. test cases to omit per layer — present each omission candidate as a
   question. An absent category is recorded as a deliberate omission only when
   the engineer confirms it is deliberate; an unconfirmed absence is dropped,
   never recorded;
6. how tests are written — what the exemplar test does not make obvious.

A candidate that would imply feature behavior absent from the Feature Contract
(for example, "every new RPC accepts an idempotency key") is not a project norm;
report it as a gap to return to the Feature Contract gate and do not record it.
How to start the application or run the test suite is repository guidance, not
a rule, and is not recorded.

## Write the artifact

Once shared understanding is reached, write
`docs/plans/YYYY-MM-DD-<feature>/project-rules.md` in this shape and present
it for the engineer's approval. Keep rules short, identified, and grouped by
layer where the layer matters. Reference exemplars by repository-relative
path. Never write an absolute path or a username.

```markdown
# Project rules: <feature>

Source: confirmed with the engineer on YYYY-MM-DD from existing code and tests.

## Review NG
- [ng-01] <rejected pattern or invariant>

## Code conventions
### <layer>
- [code-<layer>-01] follow `<exemplar path>` for <what to imitate>
- [code-<layer>-02] <convention the exemplar does not make obvious>

## Test conventions
### <layer>
- [test-<layer>-01] add: <viewpoints and the cases that prove them>
- [test-<layer>-02] do not add: <deliberately omitted categories>
- [test-<layer>-03] write like `<exemplar test path>` (<what to imitate>)
```

The file is a workspace-only, ignored artifact beside `feature-contract.md`
and `implementation-plan.md`. Do not force-add, stage, or commit it; it is
retired with the worktree. It is not persisted across features in this proof
of concept.

When the engineer prefers to write the file, provide the investigation, the
template, and critique, and make only the edits the engineer requests.

## How the rules take effect

This skill does not inject anything into the plan. `create-plan` references
the file from its existing slots by identifier — omitted cases and accepted
patterns as approved non-problems in the Review context, `Review NG` items as
grounds for `Must Fix` in the Review policy, and each Task Contract's
applicable repository guidance naming the identifiers for its layer — and the
task handoff carries the path and identifiers to the Task session, its
implementer, and its reviewers. If `create-plan` already ran, tell the engineer
to ask it for that revision. Do not copy rule prose into the plan and do not
edit the plan here.

## Record the observation

Remind the engineer that the proof of concept is judged by their own review:
for each Task PR, the number of rule violations found in self-review before
publication, whether the implementation reads as native to the project next to
its exemplars, and the number of team review comments with the subset the rules
already covered, recorded as three lines per PR in
`claude/plans/poc-project-rules-observations.md` in the dotfiles main checkout.
Agent-side signals — reviewer findings that contradict the rules, correction
rounds caused by style — are consulted only when that primary signal is poor.
