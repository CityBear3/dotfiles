# Implementation Plan: Codex Design-Centered Contract Workflow

> **Execution:** Run this plan only after user approval.

- Owner: Repository owner
- Drafted by: Codex from the approved Feature Contract and repository evidence
- Date: 2026-08-13
- Status: Approved

## Goal

Update the Codex engineering workflow so every implementation has a Feature
Contract, planned work derives a complete Task Contract set only after separate
Feature Contract approval, and implementation agents can autonomously choose and
verify local implementation details while stopping at any contract-meaning
change.

The completed workflow must preserve the existing one-writer, Git evidence,
independent review, safety, publication, and branch-disposition boundaries.

## Authorities and precedence

1. `docs/design/2026-08-13-codex-design-centered-contract-workflow.md`
2. `docs/plans/2026-08-13-codex-design-contract-workflow/feature-contract.md`
3. This approved Implementation Plan and its Task Contracts
4. Applicable repository and personal `AGENTS.md` guidance

The Design Doc owns durable architecture. The Feature Contract owns feature
success and protected behavior. This plan owns decomposition and orchestration.
If a task cannot satisfy this plan without changing a higher authority's
meaning, execution stops and returns the gap to its owning approval gate.

## Architecture and implementation summary

- `agentic-engineering-workflow` remains the coordinator and gains explicit
  workspace, Design Doc, Feature Contract, and Implementation Plan transitions.
- `design-discussion` and `design-doc` construct Feature Contracts from their
  respective approved design sources; the coordinator owns approval state.
- `create-plan` maps an approved Feature Contract into Task Contracts, shared
  interfaces, coverage, integration obligations, and review policy.
- `execute-plan` and `execute-task` propagate those contracts and allow local
  implementation and focused verification choices only inside their bounds.
- `verify` and `review` prove the integrated current head against the Feature
  Contract; task evidence remains necessary but does not replace feature-level
  proof.
- Natural-language skills and prompts remain the execution medium. No schema,
  digest, registry, or new phase skill is introduced.

## Working context and observed baseline

- Technology: Markdown Codex skills and prompts, TOML agent profiles, and the
  unchanged Rust 2024 installer.
- Working directory: repository root (`.`).
- Branch: `codex/design-contract-workflow`.
- Implementation base: `a75435fec8498475a674b1b9c5ce39e9c5a21b93`.
- Approved Design Doc: `docs/design/2026-08-13-codex-design-centered-contract-workflow.md`.
- Approved Feature Contract:
  `docs/plans/2026-08-13-codex-design-contract-workflow/feature-contract.md`.
- At plan creation, the Design Doc is untracked and `docs/plans/` is ignored by
  `.gitignore`; no runtime skill or installer source is modified.
- Observed baseline:
  `cargo test --manifest-path codex/installer/Cargo.toml --locked` passes 128
  unit tests, 1 compiled-binary process test, 1 end-to-end test, and doc tests.

## Execution approach for this implementation

The repository owner authorized direct lead implementation because this change
modifies the workflow skills that would otherwise orchestrate their own update.
The lead is the sole writer and applies Task Contracts 1–4 sequentially without
dispatching an implementation agent or running preplanned per-task reviewer
loops. At each task boundary the lead inspects the exact diff, contract coverage,
current Git evidence, and applicable focused checks before creating the planned
commit.

This is an execution-policy exception for implementing this approved feature; it
does not change the Feature Contract or the behavior the updated skills must
define. Independent review is concentrated on the final integrated head.
Correction loops start only for a concrete failed verification or accepted final
review finding. Existing one-writer, current-evidence, contract-change,
publication, and branch-disposition boundaries remain in force.

## Fixed decisions and non-goals

### Fixed decisions

- Design Doc, Feature Contract, and complete Implementation Plan retain separate
  approval gates.
- The planned Feature Contract and Implementation Plan use separate files in one
  feature directory. Task Contracts are normative plan sections, not files.
- New planned work cannot enter `create-plan` without an approved, current
  Feature Contract.
- Exact files, signatures, ordering, and commands are mandatory only when their
  identity is part of an ownership, interface, safety, reproducibility,
  migration, or coverage contract.
- A writer may choose private files, helpers, local types and interfaces,
  algorithms, edit order, standard commands, and additional focused checks
  within the Task Contract. It reports what it actually changed and observed.
- Shared interfaces have one owner. A responsibility or contract-meaning change
  is not an implementation detail and returns to the owning approval gate.
- Review context remains a derived interpretation aid, and Review policy remains
  the independent breadth, capacity, and Acceptance control.
- Tasks execute sequentially with one writer. Parallel plan tasks, task
  worktrees, and multiple writers remain deferred.

### Non-goals

- No historical Design Doc or plan migration.
- No new Feature Contract skill or machine-readable contract schema.
- No installer production-code or test change unless implementation uncovers a
  concrete generic-inventory regression.
- No reviewer model or reasoning-effort change.
- No sandbox, external-write, publication, merge, destructive-action, or branch
  disposition expansion.
- No runtime dependency from `codex/` to `claude/`.

## Shared interface contracts

### Planned artifact lifecycle

The coordinator treats these as separate approved identities:

```text
approved Design Doc or approved decision record
                -> approved Feature Contract
                -> approved Implementation Plan
                -> task execution
```

The persisted planned-path interface is:

```text
docs/plans/YYYY-MM-DD-<feature>/
├── feature-contract.md
└── implementation-plan.md
```

A resumed session re-reads the artifacts and repository state. A material edit
invalidates the affected approval; conversation memory alone does not restore it.

### Task handoff

Every planned task handoff carries:

- the approved Feature Contract and applicable clauses;
- the exact Task Contract and relevant shared interfaces;
- Review context and the complete Review policy;
- discipline, workspace, working directory, exact task base, responsibility and
  ownership boundaries;
- verification route, observable obligations, and only those exact commands
  whose identity is contractually significant.

The lightweight path derives the same information as one in-memory combined
Feature/Task Contract. The writer reports actual files, commands, results,
commit, head, range, concerns, and gaps.

### Change classification

- A private implementation choice inside the approved responsibility and
  observable behavior is autonomous.
- A newly discovered private file inside the same ownership boundary is not a
  deviation by itself.
- A new or changed goal, scope, owner, public or shared interface semantic,
  invariant, failure behavior, compatibility promise, verification obligation,
  Review policy, or authority stops execution.
- A Feature Contract meaning change returns to its approval gate and may reopen
  design. A Task Contract meaning change invalidates the Implementation Plan.

### Current evidence

Existing exact task-base, head, range, status, changed-files, commit, verification,
review, and gap evidence remains mandatory. Contract evidence adds design intent;
it does not replace observed repository evidence.

## Feature Contract coverage

| Feature verification obligation | Owning proof |
| --- | --- |
| 1. Ordered planned route | Task 1 and integration scenario 1 |
| 2. Source-specific Feature Contract construction | Task 1 and integration scenario 2 |
| 3. Separate artifacts and embedded Task Contracts | Tasks 1–2 and tracked-file inspection |
| 4. Contract-derived decomposition and coverage | Task 2 and integration scenario 3 |
| 5. Contract-aware task handoff and acceptance | Task 3 and integration scenario 4 |
| 6. Conditional exact detail | Tasks 2–3 and integration scenario 5 |
| 7. Feature-level verification and review | Task 4 and integration scenario 6 |
| 8. Lightweight combined contract and promotion | Tasks 1 and 3, integration scenario 7 |
| 9. Fresh-session reconstruction | Tasks 1–4 and integration scenario 8 |
| 10. Installer, whitespace, and sensitive-data regression | Final verification commands |

No Feature Contract clause is intentionally left to an unowned implementation
task. Obligations that require the complete integrated flow are proved only by
the final integration scenarios.

## Review context

- **Artifact and purpose:** Natural-language Codex skills, fallback prompts, and
  agent profiles that route engineering work through design contracts and an
  autonomous but bounded implementation loop.
- **Consumers:** The coordinator, design and planning phases, one writer,
  independent task reviewers, final reviewers, and the installer that
  inventories and copies the assets.
- **Material criteria:** One unambiguous approval order; clear ownership between
  skills; complete Feature-to-Task coverage; exact propagation of approved
  contracts; autonomous local choices; current Git and verification evidence;
  and explicit return on semantic contract change.
- **Material failures:** Planning before Feature Contract approval; combining
  approval identities; decomposition inventing design; unowned shared
  interfaces; forcing private file/function procedures; continuing after a
  contract change; treating task checks as feature integration proof; or losing
  approved intent after compaction or a fresh session.
- **Approved non-problems:** Natural-language handoffs need no serialization,
  schema, digest, or registry. Historical plans need no migration. Markdown-only
  behavior does not by itself require new Rust tests. Parallel plan execution is
  not implemented.
- **Inapplicable assumptions:** Database identity, distributed transaction,
  network protocol, memory allocation, and application runtime throughput do not
  apply unless the implementation adds such a consumer or path.
- **New-evidence rule:** Reconsider an approved non-problem only with materially
  new evidence of a concrete reachable failure or contract violation.

## Review policy

- **Mode:** `focused`, with broad independent final perspectives selected for
  the recorded cross-skill risks.
- **Rationale:** Direct lead editing avoids recursively running the workflow
  being modified and keeps one semantic writer across natural-language assets.
  Concentrating independent review on the integrated head checks the cross-skill
  contracts without repeated per-task Agent loops.
- **Risk surfaces:** Stale approval, missing contract propagation, duplicated
  responsibility, ambiguous interface ownership, over-prescribed task detail,
  weak autonomous-loop stop conditions, compaction recovery, and repeated prompt
  or verification cost.
- **Per-task gate:** The lead is the sole writer and performs one combined
  specification-and-quality contract check over the exact task diff. No
  implementer or per-task reviewer is dispatched. A concrete failure stops the
  sequence for bounded correction and a fresh combined check.
- **Final required standard reviewers:** `code-reviewer`,
  `design-alignment-reviewer`, `scope-reviewer`, and `code-architect`.
- **Final required adversarial reviewers:**
  `adversarial-api-reviewer` for cross-skill handoff misuse and naming traps;
  `adversarial-robustness-reviewer` for stale state, invalidation, interruption,
  and partial failure; and `adversarial-performance-reviewer` for measurable
  repeated context, command, or I/O cost.
- **Integration:** Run `adversarial-integrator` after the adversarial reviewers
  with the same Review context, contracts, target evidence, and Acceptance rule.
- **Conditional reviewers:** `test-coverage-reviewer` and
  `adversarial-tests-reviewer` become required only if executable production
  behavior, installer tests, test fixtures, or test infrastructure changes.
- **Explicitly skipped perspectives:** The two test perspectives are currently
  skipped because the approved scope changes interpreted Markdown/TOML contracts
  and preserves the generic installer implementation and executable tests.
- **Residual risk:** Natural-language routing cannot be exhaustively executed as
  a state machine. Representative scenarios and independent review reduce but do
  not eliminate interpretation variance.
- **Capacity and queue order:** Use at most six total threads including the lead,
  or lower observed capacity. No per-task reviewer is dispatched for this direct
  implementation. Final standard reviewers run before adversarial reviewers;
  the adversarial integrator runs last. Queue without reducing approved scope or
  independence.
- **Acceptance:** Keep only artifact-applicable `Must Fix` or `Should Improve`
  findings with an approved requirement, concrete reachable evidence, material
  consequence, and proportionate correction. `Should Improve` requires a
  concrete maintainability consequence or measurable repeated cost. Drop
  preference, speculation, generic best practice, optional polish, inapplicable
  assumptions, and objections to approved decisions without new evidence. An
  unproven architectural mechanism is `Escalate`, not an implementation fix.

## Task Contract 1: Establish contract-aware workflow entry

### Purpose and expected result

Make the coordinator, workspace phase, and design phases enforce the approved
artifact order and source-specific Feature Contract construction. Persist the
approved Design Doc, Feature Contract, and this approved plan in the branch.

### Feature Contract clauses satisfied

- Planned-path order and early workspace establishment.
- Separate Feature Contract construction and approval.
- Lightweight combined Feature/Task Contract derivation and promotion boundary.
- Durable artifact recovery after compaction or a fresh session.

### Responsibility and ownership boundaries

- Own coordinator transitions, design-phase handoff, workspace timing, approval
  invalidation, and the three approved planning artifacts.
- Do not define Task Contract decomposition, writer mechanics, final verification,
  or reviewer selection beyond the coordinator data they must receive.

### Applicable shared interfaces

- Implement the planned artifact lifecycle and change-classification contracts.
- `design-doc` constructs a Feature Contract only after Design Doc approval;
  `design-discussion` does so only when no Design Doc is warranted.
- The coordinator, not either design skill, owns approval state and next-phase
  transition.

### Protected constraints

- Preserve read-only request routing and every existing user-owned design,
  external-write, destructive-action, publication, and branch boundary.
- Do not infer approval from an artifact's existence or a conversation summary.
- Keep Design Doc and Feature Contract as separate user-visible approvals.

### Verification obligations

- A planned request cannot reach planning without a current approved Feature
  Contract.
- Workspace establishment occurs after the initial feature boundary is known and
  before the first durable planned artifact.
- Both Design-Doc and no-Design-Doc routes end in a separately presented Feature
  Contract.
- Lightweight work creates no artifact file and promotes to planned work on a
  material change or unrecoverable long-lived context.
- The three approved artifacts are tracked despite the repository-wide
  `docs/plans/` ignore rule.

### Dependencies

None. This task establishes the interfaces consumed by every later task.

### Discipline

Contract-focused content migration by the lead as sole writer. Preserve the
observed green installer baseline, inspect the exact task diff, and do not add
production behavior or executable tests.

### Explicit non-goals

- Do not change Git workspace mechanics beyond when the coordinator invokes
  `create-workspace`.
- Do not create another skill or approval identity.

### Delegated local decisions

The writer may choose concise section placement, wording, and cross-references
that preserve the approved lifecycle and existing skill responsibility style.

### Contractually significant files and commit

- Add the approved Design Doc and both files under
  `docs/plans/2026-08-13-codex-design-contract-workflow/`.
- Modify `codex/skills/agentic-engineering-workflow/SKILL.md`.
- Modify `codex/skills/create-workspace/SKILL.md`.
- Modify `codex/skills/design-discussion/SKILL.md`.
- Modify `codex/skills/design-doc/SKILL.md`.
- Use explicit force-add only for the two approved files under ignored
  `docs/plans/`; do not change `.gitignore`.
- Commit message: `codex: establish design contract workflow`.

## Task Contract 2: Derive responsibility-centered Task Contracts

### Purpose and expected result

Make planning consume an approved Feature Contract and produce the directory
format, shared interfaces, complete Task Contract set, coverage proof,
integration obligations, Review context, and Review policy without prescribing
private implementation procedure.

### Feature Contract clauses satisfied

- Separate Feature Contract and Implementation Plan artifacts.
- Contract-derived task decomposition, shared-interface ownership, and full
  clause coverage.
- Conditional use of exact files, signatures, ordering, and commands.
- Fresh-session comprehension of the complete plan.

### Responsibility and ownership boundaries

- Own planning input validation, decomposition, plan structure, coverage, and
  read-only plan walkthrough.
- Do not change feature semantics, approve artifacts, execute tasks, or choose
  private implementation details.

### Applicable shared interfaces

- Consume the planned artifact lifecycle established by Task 1.
- Emit the Task handoff source of truth consumed by Task 3.
- Record each shared interface once with one owner and named consumers; Task
  Contracts reference rather than redefine it.

### Protected constraints

- Return ambiguous responsibility, state ownership, interface, error semantics,
  scope, or other material design to the coordinator.
- Require every Task Contract core field and explain every Feature Contract
  clause through task or integration coverage.
- Keep Review context derived and Review policy separate.

### Verification obligations

- The planning entry rejects a missing, draft, stale, or contradicted Feature
  Contract.
- The plan template contains extractable Task Contract sections with purpose,
  clauses, ownership, interfaces, constraints, observations, dependencies,
  non-goals, and delegated decisions.
- Exact details are required only when their identity affects correctness.
- The example demonstrates responsibility-centered detail and at least one
  delegated local choice.
- Plan walkthrough reads Feature Contract, Implementation Plan, and referenced
  Design Doc, and explains obligations rather than assuming an exhaustive file
  recipe.

### Dependencies

Depends on Task 1's approved artifact lifecycle.

### Discipline

Contract-focused content migration by the lead as sole writer. Preserve the
green baseline and validate the plan template, coverage semantics, and delegated
implementation detail through focused content inspection.

### Explicit non-goals

- Do not create per-task files or a machine-readable coverage schema.
- Do not move Review policy into Feature or Task Contracts.

### Delegated local decisions

The writer may choose the clearest natural-language plan template and example,
provided every required Task Contract field and coverage relationship is
recoverable by a fresh implementer.

### Contractually significant files and commit

- Modify `codex/skills/create-plan/SKILL.md`.
- Modify `codex/skills/create-plan/example-plan.md`.
- Modify `codex/skills/walkthrough-plan/SKILL.md`.
- Commit message: `codex: add task contract planning`.

## Task Contract 3: Strengthen autonomous task execution

### Purpose and expected result

Change the shared task handoff, writer loop, task review, and correction route so
one writer can select local implementation and focused checks inside the Task
Contract, while exact current evidence and contract-change stop conditions remain
mandatory.

### Feature Contract clauses satisfied

- Feature- and Task-Contract-aware handoff and acceptance.
- Responsibility-centered ownership with conditional exact detail.
- Autonomous implementation, feedback, bounded correction, and escalation.
- Lightweight single-contract execution through the same task seam.

### Responsibility and ownership boundaries

- Own sequential plan handoff, one-task execution, scheduling input, writer and
  per-task reviewer prompts, named writer/specification-reviewer alignment, and
  bounded correction handoff.
- Do not own Feature Contract approval, task decomposition, global verification,
  final review, publication, or branch disposition.

### Applicable shared interfaces

- Consume the Task handoff emitted by Task 2.
- Pass the Feature Contract, applicable Task Contract, shared interfaces,
  Review context, Review policy, task base, responsibility boundary, and
  verification obligations without inventing a wrapper contract.
- Return actual files, commands, results, commit, head, range, review outcome,
  concerns, and gaps.

### Protected constraints

- Keep exactly one writer and sequential plan-task execution.
- Preserve task-base ancestry, fresh current-head evidence, independent
  policy-selected task review, and unrelated user changes.
- Never turn a responsibility, interface, invariant, behavior, scope, policy, or
  authority change into a private implementation choice.
- Do not require a predeclared private file merely because the writer discovers
  it during implementation.

### Verification obligations

- Writer and reviewer handoffs use Feature and Task Contracts as normative
  requirements, not a flattened list of ad hoc decisions.
- Standard verification routes may be selected from current repository evidence;
  exact commands remain mandatory only when the plan marks them contractually
  significant. Every command actually run is reported.
- The loop inspects implementation, obtains fresh feedback, reviews the diff and
  coverage, corrects concrete in-scope failures, and stops on repeated
  non-progress or a required contract change.
- A correction preserves the applicable contracts and returns to plan/design
  when its proposed meaning exceeds them.
- Named profiles and fallback prompts impose the same contract.

### Dependencies

Depends on Tasks 1–2. Later tasks may not weaken this shared handoff.

### Discipline

Contract-focused content migration by the lead as sole writer. Preserve the
green baseline and check named profiles, fallback prompts, and skill consumers
as one shared natural-language handoff interface.

### Explicit non-goals

- No parallel tasks, multiple writers, unbounded self-correction, or autonomous
  approval changes.
- No change to reviewer models, reasoning effort, or Acceptance semantics.

### Delegated local decisions

The writer may reorganize sections and reduce duplicated prompt text where the
selected role still receives the complete normative contract and evidence.

### Contractually significant files and commit

- Modify `codex/skills/execute-plan/SKILL.md`.
- Modify `codex/skills/execute-task/SKILL.md`.
- Modify `codex/skills/agent-teams-driven-development/SKILL.md`.
- Modify all four Markdown prompts under
  `codex/skills/agent-teams-driven-development/`.
- Modify `codex/skills/receiving-code-review/SKILL.md`.
- Modify `codex/agents/implementer.toml` and `codex/agents/spec-reviewer.toml`.
- Commit message: `codex: make task execution contract aware`.

## Task Contract 4: Prove integrated Feature Contract alignment

### Purpose and expected result

Make global verification, final review, and branch completion use the approved
Design Doc, Feature Contract, Task Contract set, and integrated obligations for
the exact current head.

### Feature Contract clauses satisfied

- Feature-level verification beyond individual Task Contract acceptance.
- Final contract-alignment and scope review.
- Current, recoverable completion evidence for a fresh coordinator session.

### Responsibility and ownership boundaries

- Own coordinator verification input, final reviewer evidence, relevant named
  design/scope reviewer interpretation, and branch completion evidence.
- Keep verification and review read-only. Keep triage classification and branch
  disposition under their existing owners.

### Applicable shared interfaces

- Consume approved contracts, aggregate task results, integration obligations,
  Review context, Review policy, and exact Git evidence.
- Distinguish Task Contract proof from Feature Contract observations that exist
  only at the integrated boundary.
- Return `PASS`/`FAIL`/`BLOCKED` and `CLEAN`/`FINDINGS`/`BLOCKED` under the
  existing verdict contracts.

### Protected constraints

- Do not let Review context add requirements or replace an approved contract.
- Do not accept stale verification, incomplete contract coverage, or a successful
  task-local command as final feature proof.
- Preserve check-only behavior, reviewer independence, common Acceptance,
  correction routing, and user-owned branch disposition.

### Verification obligations

- `verify` reports every Feature Contract observation and identifies unproved
  integration obligations.
- Every final reviewer receives the same approved contract sources and current
  range evidence; design and scope reviewers map findings to the correct layer.
- `finish-branch` requires current Feature Contract completion evidence and no
  surviving contract or policy gap.
- A fresh session can identify the approved artifacts, exact target, commands
  run, reviewer outcomes, residual risk, and next user-controlled gate.

### Dependencies

Depends on Tasks 1–3 and their unchanged shared interfaces.

### Discipline

Contract-focused content migration by the lead as sole writer. Preserve the
check-only phase boundaries and validate the integrated contract evidence before
the independent final review.

### Explicit non-goals

- Do not merge verification and review or allow either to fix findings.
- Do not change publication, merge, discard, or teardown authority.

### Delegated local decisions

The writer may reduce redundant lists by directly referencing the approved
artifacts, provided every consumer receives enough exact evidence to operate
without conversation reconstruction.

### Contractually significant files and commit

- Modify `codex/skills/verify/SKILL.md`.
- Modify `codex/skills/review/SKILL.md`.
- Modify `codex/skills/finish-branch/SKILL.md`.
- Modify `codex/agents/design-alignment-reviewer.toml` and
  `codex/agents/scope-reviewer.toml`.
- Commit message: `codex: verify feature contract alignment`.

## Integration verification

### Required scenarios

1. A planned change establishes its workspace, optionally approves a Design Doc,
   separately approves a Feature Contract, approves a complete Implementation
   Plan, then executes; no later gate substitutes for an earlier one.
2. Both design-source routes construct the same Feature Contract core without
   giving a design skill approval authority.
3. Planning detects a missing error semantic or conflicting shared-interface
   owner and returns to design; it does not invent the answer in a Task Contract.
4. A writer discovers a private file inside its owned responsibility, chooses a
   focused check, reports both, and continues without a plan rewrite.
5. A writer discovers that another task must consume a changed shared interface
   and stops for plan or design approval rather than changing it locally.
6. All tasks pass, but an integration-only Feature Contract observation fails;
   global verification returns `FAIL` and review does not start.
7. A lightweight task exposes a second responsibility or loses recoverable
   material context and promotes to the planned path.
8. A fresh session reconstructs the design source, approved Feature Contract,
   Task Contracts, dependencies, Review policy, current Git evidence, and next
   gate from tracked workspace artifacts.

### Required final commands and observations

The exact forms below are required because they provide reproducible repository
and installer evidence. Run them against the final unchanged implementation
head.

```sh
cargo test --manifest-path codex/installer/Cargo.toml --locked
git diff --check a75435fec8498475a674b1b9c5ce39e9c5a21b93..HEAD
git diff --name-only a75435fec8498475a674b1b9c5ce39e9c5a21b93..HEAD -- codex/installer
```

Expected results: all installer tests pass; the diff has no whitespace errors;
and no installer source or test file changed.

Also inspect the final range and demonstrate:

- every changed skill retains valid frontmatter name and a complete instruction
  body;
- the planned route, both Feature Contract construction routes, Task Contract
  coverage, contract-aware handoff, autonomous-loop stop conditions, and
  integrated verification are each stated by their owning skill;
- universal requirements for exact private files or every exact verification
  command are absent, while conditional contractually significant detail remains;
- the approved Design Doc, Feature Contract, and Implementation Plan are tracked
  at their exact paths;
- runtime Codex assets have no `claude/` dependency;
- the changed assets and new documents contain no credentials, personal data,
  private tokens, or unnecessary sensitive runtime values;
- a non-mutating installer dry run inventories the modified skills and leaves its
  selected temporary destinations absent.

The implementation agent may select additional focused, non-destructive commands
to prove these observations and must report their exact forms and results.

## Post-review iteration

Only after a concrete failed verification or accepted final-review finding,
route each verified in-scope `Fix` through the Task Contract that owns the
violated responsibility. Preserve the Feature Contract and unchanged Task
Contracts, use the current head as correction base, run fresh task verification
and the lead's complete combined task check, then rerun full integration
verification and final review for the updated range.

Return to Implementation Plan approval if a correction changes a Task Contract.
Return to Feature Contract or Design Doc approval if it changes feature or
architectural meaning. Stop after repeated non-progress with the observed
attempts and remaining gap.

## Publication and branch disposition

Do not install to personal destinations, push, create a pull request, merge,
discard, delete a branch or worktree, or otherwise publish or dispose of the
workspace without the user's explicit choice after fresh verification and clean
review.
