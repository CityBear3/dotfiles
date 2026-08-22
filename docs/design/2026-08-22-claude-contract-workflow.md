# [Design Doc] Claude Contract-Centered Workflow

- Owner: Repository owner
- Drafted by: Claude Code from owner-approved design decisions
- Date: 2026-08-22
- Status: Approved (2026-08-22; implemented by direct edit at the owner's instruction)
- Extends:
  - `docs/design/2026-07-29-codex-agentic-engineering-workflow.md`
  - `docs/design/2026-08-13-codex-design-centered-contract-workflow.md`
  - `docs/design/2026-08-18-codex-pr-scoped-task-execution.md`

## Context and scope

The Codex configuration in this repository has reached a contract-centered,
PR-scoped engineering workflow. A thin `AGENTS.global.md` points at one
coordinator skill that classifies requests into read-only, lightweight, and
planned paths; design is captured in three layers (Design Doc, Feature
Contract, Task Contract); an Implementation Plan records a Task dependency DAG
and a PR topology; and each Task Contract runs its own bounded loop of
implementation, verification, policy-selected review, triage, and correction
until it is internally accepted.

The Claude Code configuration has not followed. Its always-loaded
`CLAUDE.global.md` carries the Core Flow diagram, the automatic transitions, the
triage contract, the workspace policy, and the agent model policy. Its
`create-plan` produces procedural plans with exact files, complete code, and
exact commands. Its `execute-plan` hands the whole plan to
`agent-teams-driven-development`, a persistent four-member team pinned to Opus,
and its `review` always launches seven reviewers plus an integrator regardless
of risk. There is no Feature Contract, no Task Contract, no dependency graph,
and no way for independent tasks to proceed in parallel.

The owner wants the Claude workflow to match the Codex workflow as closely as
possible, because one structure across both tools is easier to maintain and
easier to analyze. The Claude runtime differs from Codex in three ways that
shape the execution substrate rather than the contracts: Claude Code sessions
can message each other by name, one-shot subagents are substantially cheaper
than persistent agent teams, and three model tiers (Fable, Opus, Sonnet) are
available with different cost profiles. This design keeps the Codex contracts
and phase boundaries unchanged and maps them onto that substrate.

The owner also wants a proof of concept, on the Claude side only, for
capturing the tacit norms of a project — what its reviewers reject, how its
code is shaped, which tests it writes and which it deliberately does not — so
that implementation and review agents stop producing code that a human
immediately recognizes as foreign to the project.

### Goals

- Adopt the Codex three-layer contract model, approval gates, Implementation
  Plan structure, Review context, Review policy, Task dependency DAG, and PR
  topology on the Claude side without changing their meaning.
- Run each Task Contract's loop in its own Claude Code session, in its own
  herdr-managed worktree, launched and driven by the coordinator session
  through cross-session messaging.
- Replace the persistent agent team with one-shot subagents inside each Task
  session: one implementer as the sole writer, plus read-only verifiers and
  reviewers selected by the approved Review policy.
- Fix one model tier per role: Fable for the coordinator session, Opus for Task
  sessions, Sonnet for every subagent.
- Carry over the `focused`, `adaptive`, and `deep` review modes and the common
  Acceptance threshold, and align the agent profile set with Codex one to one.
- Move the workflow flow out of `CLAUDE.global.md` into a coordinator skill,
  leaving the global file as a short set of principles with one pointer, as
  `AGENTS.global.md` is.
- Let the agent draft Design Docs, Feature Contracts, and plans from
  owner-approved decision records, as Codex does, replacing the rule that the
  engineer must write Design Doc prose first.
- Provide a user-invoked-only skill that captures per-feature project norms
  into a workspace-only artifact that the plan references, and define how its
  effect is observed.

### Non-goals

- Change any Codex asset. The proof-of-concept skill is ported to Codex only
  after it has demonstrated an effect.
- Redefine any contract, approval gate, Acceptance threshold, or review-mode
  semantics established by the three extended Design Docs.
- Introduce a machine-readable workflow-state schema, a message schema, or a
  persistent runtime-state file.
- Provide a headless (`claude -p`) fallback for Task sessions, or any
  non-herdr mechanism for creating Task worktrees.
- Add a manual `/review <mode>` argument, a per-plan model override, or any
  other knob that Codex does not have.
- Make the project-norms artifact persistent across features, or let the
  proof-of-concept skill edit plan prose or any workflow skill.
- Automatically publish, push, merge, retarget, or delete branches, PRs, or
  worktrees from any session.
- Migrate archived plans under `claude/plans/` to the new format.

## Overview

The workflow has three tiers of execution, each with a fixed model and a fixed
set of skills.

```text
Coordinator session (Fable)                     coordination worktree
  agentic-engineering-workflow                  docs/plans/<feature>/
    design-discussion / design-doc                feature-contract.md
    create-plan / create-workspace                implementation-plan.md
    execute-plan  ──── SendMessage ────┐          project-rules.md (PoC)
    verify / review (integration only) │
    receiving-code-review              │
    finish-branch / session-teardown   │
                                       │
           ┌───────────────────────────┼───────────────────────────┐
           v                           v                           v
Task session (Opus)          Task session (Opus)          Task session (Opus)
  execute-task                 execute-task                 execute-task
  Task worktree = herdr ws     Task worktree = herdr ws     Task worktree = herdr ws
    implementer (Sonnet)         implementer (Sonnet)         implementer (Sonnet)
    verifier / reviewers         verifier / reviewers         verifier / reviewers
    (Sonnet, one-shot)           (Sonnet, one-shot)           (Sonnet, one-shot)
```

The coordinator session is the one the engineer sits in. It runs the
coordinator skill, owns every approval gate, and holds the approved artifacts
in the coordination worktree exactly as the Codex design prescribes. When an
approved plan reaches execution, `execute-plan` — still running in the
coordinator session — creates one herdr worktree per ready task, starts an
Opus session in that worktree's pane, and sends it a task handoff message. The
Task session runs `execute-task`: it never edits files itself, but launches a
Sonnet implementer subagent as the single writer and Sonnet verifier and
reviewer subagents as the policy-selected gate, loops through triage and
bounded correction, and reports its result back to the coordinator by message.
The coordinator updates both graphs, releases dependents, and, once every
task is accepted, runs only the integration-only obligations before handing
the topology to `finish-branch`.

The lightweight path does not spawn a Task session. It runs `execute-task` in
the coordinator session, in the engineer's own worktree, with the same Sonnet
subagents.

The skill inventory mirrors Codex one to one. Two skills are new on the Claude
side (`agentic-engineering-workflow`, `execute-task`), one is retired
(`agent-teams-driven-development`), and one is the proof of concept
(`inject-project-rules`). The agent profile set grows from eleven to the
fourteen Codex profiles.

## Detailed design

### Responsibility ownership

The Codex ownership split is adopted without change: the coordinator owns path
classification, gate order, artifact approval state, and cross-phase
transitions; `design-discussion` and `design-doc` own Feature Contract
construction; `create-plan` owns decomposition, both graphs, Review context,
and Review policy; `execute-plan` owns readiness, scheduling, handoffs,
staleness propagation, and evidence aggregation; `execute-task` owns one Task
PR's loop; `verify`, `review`, and `receiving-code-review` are check-only;
`finish-branch` owns disposition choices without making them.

Three responsibilities are specific to this design. `execute-plan` owns the
lifecycle of Task sessions: creating their worktrees through
`create-workspace`, launching them, addressing them, and deciding when they
are no longer needed. `execute-task` always runs in the session that owns the
checkout it works on — a Task session on the planned path, the coordinator
session on the lightweight path — and that session is never a writer. The
global guidance file owns nothing but principles and the pointer to the
coordinator.

### Session topology and workspaces

The existing workspace policy, one worktree equals one herdr workspace equals
one session, is kept and applied at task granularity. The coordination
worktree is the feature workspace the engineer opens; it holds the Feature
Contract, the Implementation Plan, and, for the proof of concept, the project
norms file. Every Task Contract that the approved plan marks as running in its
own checkout gets its own worktree, created with
`herdr worktree create --cwd <repo-root> --branch <task-branch> --base <planned-base> --no-focus --json`.
Because herdr creates a workspace together with the worktree, the task
worktree is a workspace whose initial pane is a shell at the worktree path.
The coordinator never splits panes inside its own workspace.

The Task session is started in that initial pane with
`herdr agent start <name> --kind claude --pane <pane-id> -- --name <name> --model opus --permission-mode <mode>`.
This is the one place where the policy "the engineer opens sessions" is
narrowed: the engineer opens the coordination session, and the coordinator
opens Task sessions. `create-workspace` records this split. The permission
mode of Task sessions is a workflow setting recorded in the coordinator skill;
its initial value is `acceptEdits`, chosen so that edits inside the task
worktree proceed without prompts while anything outside the ordinary edit and
test surface still surfaces in the visible pane where the engineer can answer
it. `bypassPermissions` is never used.

Session names are part of the approved plan. The coordinator session names
itself `<feature>-coord` (by `--name` at launch or `/rename` before
execution), and each Task session is `<feature>-task-<n>` where `<n>` is the
task's position in the plan. Names are the addresses used by `SendMessage`,
so they must be unique on the machine for the duration of the feature.

Fan-in restacks, corrections, and re-verification all happen inside the
task's own checkout. Task worktrees and their herdr workspaces persist until
the engineer removes them; the workflow never removes a worktree, a branch, or
a PR as an implied cleanup action, exactly as in the Codex design.

If the herdr CLI or its socket is unreachable when a Task session must be
created, `execute-plan` returns `BLOCKED` with the task identity and the
condition for re-entry. There is no alternative launch path.

### Handoff and result messages

All authority flows through files; messages carry identities and evidence.
The task handoff is one `SendMessage` from the coordinator session to the Task
session whose body contains, in plain language, the same fields the Codex
`execute-plan` handoff contains: the exact Feature Contract identity and path,
its approval and currentness evidence, the clauses assigned to the task, the
exact Task Contract (by plan path and section), applicable shared interfaces,
the Review context and complete Review policy (by path and section), the
declared discipline and applicable repository guidance — including the project
norms file and rule identifiers when the plan references them — the
coordination directory, the task workspace, branch, and planned PR identity,
the starting commit, planned base ref and commit, current head, whether the
handoff is candidate or authoritative, responsibility boundaries, verification
routes, the commit intent, and any contractually significant files,
signatures, ordering, or commands.

The message references the contract and plan files by absolute path inside
the coordination worktree rather than copying their prose, because the task
worktree does not contain those ignored files and because the Codex design
already forbids copying unchanged prose into handoffs. The Task session reads
the referenced sections directly. Absolute paths appear only in messages,
never in tracked files.

The result is one `SendMessage` from the Task session to the coordinator
session. Its body is the `execute-task` return defined by Codex: exactly one
of `Candidate`, `Accepted`, `BLOCKED`, or `Escalate`, followed by the exact
authority and Task Contract content accepted, mode, writer status, task and
correction commits, workspace and branch, planned base ref and commit, merge
base, current head, exact range, changed files, commands and observed results,
pre-commit inspection, the gate result when authoritative, capacity evidence,
concerns, gaps, and the exact re-entry condition.

The message body is the only evidence the coordinator accepts. herdr's agent
state (`working`, `idle`, `blocked`, `done`) is used for liveness — to detect
that a Task session died or stalled without reporting — and never as a
completion signal. A result whose branch, base, or head does not match what
the coordinator observes in the task worktree is treated as a workspace
mismatch and returns `BLOCKED`, as the Codex `execute-plan` already requires.

A Task session persists across the task's whole loop. A planned correction is
another message to the same session, carrying the bounded correction handoff
that `receiving-code-review` produces; authoritative re-entry of a candidate
after its final base is materialized is likewise a message to the same
session. The coordinator tells the Task session when the task's role is
complete; the session and its pane then remain available for the engineer to
inspect until the worktree is removed.

### The Task session loop

`execute-task` is ported from Codex without semantic change and runs with the
Task session as its lead. The lead is the Task orchestrator: it validates the
handoff, resolves the workspace and Git identities, selects the writer, drives
the loop, invokes `verify`, `review`, and `receiving-code-review`, and
composes the result message. It does not edit files, stage, or commit. This
narrows the Codex rule "the lead when direct execution is authorized,
otherwise one implementer" to "always one implementer": the Task session's
context is reserved for evidence, triage, and the correction loop, and the
finding a reviewer raises is never triaged by the context that wrote the code.

The writer is one `implementer` subagent, launched by the Agent tool without a
`name` so it is a one-shot subagent rather than a teammate, in the foreground
so its report returns as the tool result, and with `model: "sonnet"` passed
explicitly. Bounded corrections are sent to that same subagent through
`SendMessage` so it keeps its context; if the subagent is no longer reachable,
a new implementer is launched with the correction handoff and the current
diff, which the Codex design already allows because the correction handoff is
self-contained. The writer's report states `DONE`, `DONE_WITH_CONCERNS`,
`BLOCKED`, or `NEEDS_CONTEXT` and the fields the Codex `execute-task` requires;
the lead interprets it exactly as Codex does.

Verification runs through the `implementation-verifier` profile, which on the
Claude side is defined as check-only: it may run build, test, lint,
non-mutating format checks, and diff inspection, and may create ignored build
artifacts, but must not write tracked or in-scope source files and must not
run a formatter in a mode that writes. This differs from the Codex profile,
whose workspace-write sandbox makes it incompatible with the Codex `verify`
phase; on the Claude side the profile is made compatible by instruction and
by denying the edit tools, so `verify` has a named route instead of falling
back to the lead.

Review subagents are launched as the approved Review policy selects, all
name-less, foreground, and pinned to Sonnet. When any adversarial perspective
runs, `adversarial-integrator` runs after them and returns its integrated
section inline. Triage follows `receiving-code-review`; `Fix` items go back to
the writer with a bounded correction handoff, `Push back` items close the gate
when nothing else remains, and `Escalate` items end the task with an
`Escalate` result message. The acceptance conditions, the staleness rules,
and the prohibition on rewriting history to manufacture a topology are the
Codex rules unchanged.

The per-task review report that the current Claude `review` skill appends as
HTML next to the plan is kept, but written per task as
`docs/plans/<feature>/review-<task>.html` so that concurrent Task sessions
never append to the same file. The path is registered in the clone-local
`info/exclude` as today.

### Review policy and the agent profile set

The three review modes keep their Codex contracts. The per-task gate runs in
the Task session; integration perspectives run in the coordinator session
against the exact composed tree and only when the approved policy requires or
conditionally triggers them. The Claude profiles map onto the Codex
perspectives as follows.

| Profile | Role in Claude | Writes files | Model |
| --- | --- | --- | --- |
| `implementer` | sole writer of one Task PR candidate; TDD for behavior changes | yes, within its task worktree | sonnet |
| `implementation-verifier` | check-only verification executor | no (ignored artifacts only) | sonnet |
| `code-reviewer` | combined specification-and-quality gate for `focused` | no | sonnet |
| `spec-reviewer` | independent specification gate for `adaptive` and `deep` | no | sonnet |
| `code-quality-reviewer` | independent quality gate for `adaptive` and `deep` | no | sonnet |
| `test-coverage-reviewer` | behavioral coverage and assertion strength | no | sonnet |
| `design-alignment-reviewer` | alignment with Design Doc, Feature Contract, Task Contract | no | sonnet |
| `scope-reviewer` | scope, non-goals, missing deliverables | no | sonnet |
| `code-architect` | architecture context for design discussion; structural-coherence perspective in review | no | sonnet |
| `adversarial-api-reviewer` | misuse-prone interfaces (extended thinking) | no | sonnet |
| `adversarial-robustness-reviewer` | reachable failure modes (extended thinking) | no | sonnet |
| `adversarial-performance-reviewer` | measurable hot-path cost (extended thinking) | no | sonnet |
| `adversarial-tests-reviewer` | tests that do not prove behavior (extended thinking) | no | sonnet |
| `adversarial-integrator` | deduplicates and normalizes adversarial findings | no | sonnet |

Three profiles are new on the Claude side: `spec-reviewer` and
`code-quality-reviewer` replace the role prompts that
`agent-teams-driven-development` layered on `code-reviewer`, and `implementer`
replaces that skill's implementer prompt. `code-architect` keeps its
design-discussion use and gains the Codex review role. Every profile that does
not write declares it in its definition and denies the edit tools in its
frontmatter, closing the drift observed on 2026-07-13 in which a reviewer
edited the working tree instead of reporting. The adversarial profiles keep
the `ultrathink` instruction in their prompts.

The current fixed dispatch of seven reviewers becomes the `deep` integration
case: it runs only when the approved policy names integration-only
obligations or cross-task risks that require every applicable perspective,
and the default `adaptive` policy sends most review effort to the per-task
gates.

### Model policy

Models are pinned per tier and never inherited. The coordinator session runs
on whatever model the engineer chose for it, which is Fable today; that cost
is acceptable because the coordinator's tokens are orchestration and
approval, not implementation or review. Task sessions are launched with
`--model opus`: they carry the loop, the triage, and the evidence, and they are
long-lived. Every subagent is Sonnet, declared in its profile's frontmatter
and repeated as an explicit `model` argument at the call site so the policy
is auditable where it is applied. The plan never records a model.

This reverses two earlier decisions, retired on 2026-08-22: that teammates
stay on Opus, and that the persistent team is kept. Both were conditioned on
cost triggers that have now fired. The known Opus weakness with multibyte
tool-call output remains relevant to Task sessions; the existing mitigation
of thinking in English is kept, and the Task handoff and result messages are
written in English for the same reason. The revisit trigger for the Sonnet
reviewers is an observed case of a subtle defect that the old Opus reviewers
would have caught; the revisit trigger for Sonnet implementers is a task whose
accepted result the engineer rejects on quality grounds. A per-task model
override is explicitly not added; if the trigger fires, the profile changes.

### Staleness across sessions

The Codex staleness rules apply unchanged; what this design adds is where
evidence lives. An `Accepted` result is bound to the exact base, head, merge
base, range, contract authority, and consumed interfaces recorded in the
result message. Before each scheduling wave, `execute-plan` re-resolves every
task branch, base, head, and status directly from Git across all task
worktrees and compares them with the last accepted messages; any difference
marks the task and its descendants through both graphs stale. A restack or
retarget requires its explicit authority and is performed in the task's own
checkout; the coordinator then sends an authoritative re-entry message to the
same Task session, which reruns fresh verification and review on the new
range before the task can release dependents again.

A Task session that has been lost — its pane gone, its herdr state
`unknown`, and no result message — is never replaced until the coordinator
has inspected the task worktree and confirmed that its in-flight edits and
commits are attributable to that task and descend from its base. If they are
not, the coordinator returns `BLOCKED` with the observed Git state and does
not clean, reset, or recommit.

### Global guidance and the coordinator

`CLAUDE.global.md` is restructured to the shape of `AGENTS.global.md`:
Collaboration, Implementation, Engineering workflow, and Verification, each a
short list of principles stated once, plus the language rule. The Engineering
workflow section names `agentic-engineering-workflow` as the source of truth
for path selection, approval gates, and cross-phase transitions, and states
that authorized local implementation, verification, review, and in-scope
correction continue without repeated approval prompts while user-owned
decisions, missing authority, plan deviations, material scope expansion, and
publication or disposition choices stop.

The Core Flow diagram, the list of automatic transitions, the triage contract,
the Bugfix Flow, the Workspaces section, and the Agents model policy leave the
global file and reappear, in their Codex form, inside the coordinator skill
and the phase skills that own them. The Claude-specific behavioral rules that
do not describe the flow — investigate before asking, lead with the outcome,
the division of responsibility and its red flags, the escalation rule, and the
evidence-before-claims rule — are kept, condensed into the matching sections,
because they are about how this model should behave rather than how the
workflow is wired. The exact text is part of this design's approval.

Two rules change meaning rather than location. The entry point moves from
"all work begins with `/design-discussion`" to the Codex rule that engineering
requests use the coordinator, which selects the path and invokes
`design-discussion` when material decisions remain; `/design-discussion`
stays user-invocable for consultation. And the Design Doc authorship rule
changes from engineer-first prose to the Codex rule: the engineer owns every
material decision and the decision record, and the agent drafts the Design
Doc, the Feature Contract, and the plan from that record, returning any
material ambiguity to discussion instead of filling it in. The engineer may
still author a draft, in which case the skill provides research, template,
critique, and requested edits.

Every phase boundary is still crossed by invoking the phase skill through the
Skill tool, never by performing a phase inline; the coordinator skill invokes
phase skills the same way the current skills invoke each other.

### Skill inventory

| Codex skill | Claude skill | Change |
| --- | --- | --- |
| `agentic-engineering-workflow` | new | port; owns the flow that leaves `CLAUDE.global.md` |
| `design-discussion` | rewrite | decision record, Feature Contract construction, workspace timing |
| `design-doc` | rewrite | agent drafts from the decision record; derives the Feature Contract |
| `create-workspace` | rewrite | coordination, Task PR, and integration workspaces; Task session launch policy |
| `create-plan` | rewrite | contract-centered plan with both graphs, Review context, Review policy |
| `execute-plan` | rewrite | scheduler over Task sessions; messaging; staleness; aggregation |
| `execute-task` | new | port; runs in the Task session with Sonnet subagents |
| `verify` | rewrite | target forms; check-only executor |
| `review` | rewrite | target forms; modes; policy validation; per-task report file |
| `receiving-code-review` | rewrite | coordinator-managed and standalone entry; bounded correction handoff |
| `finish-branch` | rewrite | task, lightweight, feature, and legacy modes |
| `commit`, `create-pr`, `session-teardown`, `dispatching-parallel-agents`, `test-driven-development`, `systematic-debugging`, `walkthrough-plan` | align | wording and inputs aligned with the new contracts |
| `agent-teams-driven-development` | retired | prompts absorbed into `implementer`, `spec-reviewer`, `code-quality-reviewer` |
| — | `inject-project-rules` (new, Claude only) | proof of concept |

The legacy-plan compatibility clauses that Codex carries in its coordinator,
`execute-plan`, `execute-task`, `verify`, `review`, and `finish-branch` are
ported unchanged even though no Claude plan will be executing under the old
format when this lands; keeping the text identical keeps the two skill sets
diffable. `claude/install.sh` keeps distributing `CLAUDE.global.md`,
`skills/`, and `agents/`, but manages only the names that come from this
repository: it records them in `~/.claude/.dotfiles-managed`, removes a name
only when a previous install managed it and the repository has since dropped
it, and never touches skills, agents, hooks, or plugins that other tools or the
engineer installed. A `--dry-run` prints the planned actions.

### Project norms injection (proof of concept)

The tacit norms of a project — the patterns its reviewers reject, the shape of
its code, and above all which test cases it writes and which it deliberately
omits — are what a human absorbs from reading the existing code and what an
agent does not. The proof of concept makes those norms explicit for one
feature and binds them into the plan's existing slots, without touching any
workflow skill.

The artifact is `docs/plans/<feature>/project-rules.md`, a workspace-only
file beside the Feature Contract and Implementation Plan that is retired with
the worktree. It has three fixed sections, each a list of short identified
rules grouped by layer where relevant:

```markdown
## Review NG
- [ng-01] handlers never touch the database directly; go through the repository

## Code conventions
### handler
- [code-h-01] follow `src/handlers/get_user.rs` for error mapping and response assembly

## Test conventions
### handler
- [test-h-01] add: one happy path, one representative validation failure, one propagated lower-layer error
- [test-h-02] do not add: argument-order permutations, exhaustive parameter combinations
- [test-h-03] write like `tests/handlers/get_user_test.rs` (fixture construction, assertion granularity)
```

`Review NG` carries rejected patterns and project invariants; `Code
conventions` carries layer responsibilities and exemplar files to imitate;
`Test conventions` carries the cases to add, the cases the project
deliberately does not write, and exemplar tests. A rule that would imply
feature behavior absent from the Feature Contract is not a project norm and is
returned as a gap to the Feature Contract gate rather than recorded. How to
start the application or run the tests is repository guidance and is out of
scope.

The skill `inject-project-rules` is user-invoked only
(`disable-model-invocation: true`) and follows the `design-doc` pattern: it
investigates the existing code and tests, using read-only subagents where
useful; it discusses one topic at a time — exemplar candidates per layer, the
case categories present in existing tests and their style, the generic
categories that are absent, and review rejections the engineer remembers —
and records an omitted case as a norm only when the engineer confirms it is
deliberate, because absence alone does not distinguish an intentional gap from
an accidental one; and once shared understanding is reached it writes the
file for the engineer's approval. There is no draft/apply split and no
separate hand-editing step.

The plan does not receive the rules' prose. `create-plan`, as part of its
ordinary drafting or on the engineer's request during plan review, references
the file from the slots that already exist: the Review context lists the
omitted test cases and accepted patterns as approved non-problems by rule
identifier; the Review policy cites `Review NG` items as grounds for `Must
Fix`; and each Task Contract's applicable repository guidance names the rule
identifiers for its layer. The default moment to run the skill is after
Feature Contract approval and before `create-plan`, so the plan is born with
the references; running it after the plan draft means asking `create-plan`
for a revision, and running it after plan approval is an ordinary plan change
that invalidates approval. Task handoffs already carry applicable repository
guidance, so the Task session, its implementer, and its reviewers receive the
path and identifiers without any change to the handoff.

The effect is judged by the engineer's own review rather than by agent
metrics. For each Task PR the engineer records three lines in a local file
outside the feature worktree (`claude/plans/poc-project-rules-observations.md`
in the dotfiles checkout, an ignored path): the number of rule violations
found in self-review before publication, whether the implementation reads as
native to the project next to its exemplars, and the number of team review
comments with the subset that the rules already covered. One feature without
the rules followed by one or two with them, on the same project, is the
comparison. Agent-side signals — findings that contradict the rules, and
correction rounds caused by style — are consulted only to choose the next
step when the primary signal is poor: reviewers ignoring the reference points
to inlining the rules into Review context and policy, implementers ignoring
it points to inlining into Task Contract obligations, and team comments the
rules never covered point to widening the skill's investigation.

### Failure behavior

The workflow returns `Escalate`, `BLOCKED`, a plan deviation, or stale
evidence under exactly the Codex conditions. Session mechanics add four
`BLOCKED` causes: herdr is unreachable when a Task worktree or session must be
created; a Task session's result message disagrees with the Git state of its
worktree; a Task session is lost with unattributable in-flight state; and a
message cannot be delivered to a named session. None of these causes the
coordinator to reset, clean, rewrite history, or start a replacement writer.

## Cross-cutting concerns

### Compaction, fresh sessions, and memory

Approved artifacts live in the coordination worktree and are the recovery
source for the coordinator, as in Codex. A Task session recovers from its
handoff message, the files it references, and the Git state of its worktree;
it never depends on the coordinator's conversation. Claude Code's auto-memory
is keyed by working directory, so a worktree session does not see the memory
of the main checkout and its own memory is orphaned when the worktree is
removed. This design therefore requires that every handoff and result be
self-sufficient and that nothing the workflow needs be recorded only in
memory. The owner's standing guidance for this repository is kept in the main
checkout's memory and re-applied by the engineer when opening a worktree
session; sharing memory across worktrees is outside this design.

### Cost

The design moves tokens down the tiers. Orchestration and approval stay on
Fable; each task's loop runs on Opus; all implementation, verification, and
review runs on Sonnet one-shot subagents that are summarized back into the
Task session rather than persisting as teammates. The default `adaptive`
policy replaces the unconditional seven-reviewer fan-out with per-task gates
and risk-selected perspectives, so review cost scales with recorded risk
rather than with the number of tasks. Parallel tasks multiply cost by the
number of concurrent writers the plan permits, which the plan's capacity
section bounds.

### Permissions and external writes

Task sessions run with `acceptEdits` inside their own worktree. They never
push, publish, merge, retarget, or delete anything: publication remains a
`finish-branch` choice made by the engineer in the coordinator session, and
`create-pr` writes only an authorized exact head and base. Subagents that do
not write deny the edit tools; the implementer writes only inside its task
worktree. Absolute paths and session names appear in messages and ignored
files only; tracked files, including Design Docs and commit messages, use
`~`-prefixed or repository-relative paths and contain no usernames or
addresses.

### Bootstrapping

The skills this design describes do not exist until the plan that implements
them is executed. That plan is executed by either the current Claude workflow
or the Codex workflow, at the owner's choice before `create-plan`; neither
choice changes the design. If the current Claude workflow is used, the
engineer will be asked separately whether its verification and review loop
applies, as the standing guidance for overriding the execution mechanism
requires.

### Compatibility

Archived plans under `claude/plans/` are not migrated. The
`agent-teams-driven-development` skill is removed from distribution; the
`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` setting becomes irrelevant to the
workflow and is not managed by it. The per-run review timing log under
`~/.claude/usage-data/review-timings/` is kept as it is. The `hints/` language
files under the review skill are kept and loaded as investigation prompts.

## Alternatives considered

### One feature branch and one feature PR

Rejected, as in the Codex design, and additionally because it would make the
Claude `execute-plan`, `execute-task`, and `finish-branch` diverge from Codex
and leave two workflows to maintain. A single-publication option that stacks
every task and publishes only the top is cheap to add later and is deferred.

### Split a pane inside the coordination workspace for each Task session

Rejected because herdr creates a workspace for every worktree; a pane split
would put a session in the coordinator's workspace while its checkout lives
elsewhere, and would hide the task from the workspace list and its agent
status.

### Headless `claude -p` Task sessions

Rejected as the primary mechanism because a `-p` session ends after one turn,
so every correction would be a `--resume` relaunch rather than a message to a
live session, and because it has no pane for permission prompts or for the
engineer to inspect. It is not added as a fallback either; one launch path is
enough and herdr is already required by the workspace policy.

### Task session as the writer

Rejected because the per-task loop includes verification, policy-selected
review, triage, and bounded correction; keeping the writer out of that
context preserves the triage independence and the one-writer rule by role, and
keeps `execute-task` identical to Codex. The loss of Opus as the writer is
accepted and covered by the revisit trigger.

### Keep the persistent agent team

Rejected because teammates are separate Claude instances with full contexts
and the feature is experimental; one-shot subagents return summaries and are
the cheaper and better documented path. The earlier decision to keep the team
was explicitly conditioned on cost signals.

### Keep the unconditional seven-reviewer review

Rejected because it reviews every feature at `deep` intensity regardless of
risk, and because the Codex policy already encodes the same perspectives
behind a mode that the engineer approves per plan.

### Keep the Core Flow in `CLAUDE.global.md`

Rejected because the coordinator rules are long, apply only to engineering
sessions, and are loaded into every session when they live in the global
file; a skill loads them on demand, and the Codex global file demonstrates the
resulting shape.

### Engineer-first Design Doc prose

Replaced by the Codex rule at the owner's instruction. The owner retains every
material decision through the decision record; the document is drafted from
that record and approved by the owner.

### A persistent project-rules file, or injecting rule prose into the plan

Deferred. A persistent file would need a team decision to track it, would go
stale, and would be hard to remove; a per-feature file disappears with its
worktree. Injecting prose into the plan would make a second skill edit
`create-plan`'s artifact; referencing by identifier from existing slots keeps
the proof of concept independent, and inlining remains the documented next
step if references prove too weak.
