---
name: review
description: Run read-only evidence-based review of an exact verified Task PR, an approved integration-only target, or a standalone scope using the applicable Review policy. Invoke with `/review` from `execute-task`, from the coordinator for a targeted integration review, or standalone.
argument-hint: "[file paths, range, or branch (optional)]"
---

# Review a verified Task PR or integration target

Review the requested scope, not the entire repository by default. Remain
check-only and read-only. Keep every reviewer and integrator read-only. Do not
mutate source or Git state, implement or stage a fix, classify findings for
triage, or advance another workflow phase.

Invoke phase skills through the Skill tool; never perform another phase's work
inline.

## Resolve the review target

Use one target form:

- a coordinator-managed Task PR with exact planned base, merge base, current
  head, and range;
- a coordinator-managed targeted integration review over an exact composed tree
  and named integration-only obligation;
- an eligible legacy coordinator-managed committed range;
- a standalone committed range;
- a standalone current index/worktree snapshot;
- a standalone bounded explicit fileset (`$ARGUMENTS` may name the files,
  range, or branch).

Record target form, base and head when applicable, current status, exact range or
bounded files, diff, changed files, relevant untracked paths, primary language,
repository guidance, and limitations before dispatch.

## Coordinator-managed entry

Require one exact coordinator target and its authority.

For a Task PR require:

- Task Contract and PR identities, task workspace and branch, planned base ref
  and commit, merge base, current head, exact range, diff, status, and changed
  files;
- fresh coordinator verification `PASS` for that same unchanged head and range;
- no unexplained in-scope source state outside the committed range;
- approved scope, non-goals, Review context, and complete Review policy;
- applicable Feature and Task Contract authority, current dependency and shared-
  interface evidence, observed commands, concerns, prior triage decisions, and
  known gaps.

For a planned targeted integration review require:

- approved Design Doc when applicable, Feature Contract, Task Contract set,
  Implementation Plan, Review context and policy, and their approval state;
- every current accepted Task PR result, both topologies, exact composition, and
  fresh integration verification `PASS`;
- the named integration-only obligation or concrete policy trigger that makes
  this review applicable.

Reject an ordinary full-feature review target for new-format work. Task PR gates
are authoritative; the integration target may examine only the approved
cross-task surface and evidence that triggered it.

For a lightweight targeted integration review require:

- the complete recoverable combined in-memory Feature/Task Contract, original
  request authority and design sources, Review context, and current policy;
- the current exact accepted lightweight Task PR and its base, head, tree,
  range, status, verification, review, and triage evidence;
- fresh integration verification `PASS` for that same head and tree;
- the named integration-only obligation or concrete current policy trigger.

Do not require a Design Doc, contract file, Implementation Plan, Task DAG,
multi-PR topology, or temporary multi-head composition for this authority form.
Review only the named integration surface and do not replay the Task PR gate.

For a lightweight Task PR target, accept the complete combined in-memory
Feature/Task Contract, original request authority and design sources, exact Task
PR target, and fresh verification `PASS`. Require the contract to remain
completely recoverable and no promotion condition or material change to be
unresolved. This Task PR review also satisfies feature review when no
integration-only trigger exists. Do not require an Implementation Plan or
contract file.

For a plan approved and already executing before the contract-centered format,
accept its exact approved plan and referenced design sources in place of Feature
and Task Contract artifacts only when the coordinator supplies unchanged
approval and in-flight evidence, no material ambiguity, and no owner migration
choice. Use its original scope, task specifications, verification and completion
criteria, Review context, and Review policy. Do not manufacture new artifacts.

Resolve workspace, branch, base, head, merge base, range or composed tree, diff,
and changed files directly from Git. Require target state to match supplied
evidence. Return `BLOCKED` without dispatch when it does not resolve, evidence is
stale, in-scope state falls outside the target, or a required input is missing.
Standalone evidence never satisfies a coordinator target.

## Standalone read-only entry

Resolve the requested committed range, index/worktree, or bounded fileset through
local read-only investigation. Record available verification, Design Doc,
Feature Contract, Task Contracts, plan, decision, and repository-guidance
evidence. Absent or stale verification is a limitation.

Use an approved Review policy when one is available. Without one, select only
perspectives applicable to observed risk and report the missing policy; do not
present the result as policy-complete coordinator evidence.

Derive the smallest reasonable Review context from the requested artifact,
repository evidence, and available decisions. State:

- artifact and purpose;
- consumers and execution or interpretation model;
- material quality criteria and realistic failures;
- approved or inferred non-problems;
- inapplicable assumptions;
- every material assumption made because approved context was unavailable.

A standalone worktree or fileset review may answer the direct request, but never
substitutes for current-head coordinator review.

Review context is an interpretation aid. It must not add to, weaken, or replace
an available Design Doc, Feature Contract, Task Contract, or Implementation
Plan.

## Validate policy and actual risk

When a Review policy exists, require:

- mode: `focused`, `adaptive`, or `deep`, with rationale and risk surfaces;
- the per-task gate; `adaptive` and `deep` require
  independent specification and quality review;
- integration required reviewers with reasons;
- integration conditional reviewers with exact triggers;
- explicitly skipped perspectives with reasons;
- adversarial integration rules;
- residual risk, capacity, deterministic queue order, and Acceptance.

Compare the actual artifact, diff, behavior, tests, public seams, responsibilities,
and failure paths with the recorded risks and skips. A material risk absent from
an approved policy is a policy gap, not permission to add or omit a reviewer.
For coordinator review, return `BLOCKED` so the coordinator can `Escalate` for a
replacement policy. For standalone review, report the limitation.

Record the current head before review and require it to remain unchanged. Treat
an uncommitted in-scope change as stale coordinator verification.

Load `hints/<primary-language>.md` when present. Detect the primary language
from manifest files in this order and stop at the first match: `Cargo.toml`
(rust), `go.mod` (go), `package.json` (typescript), `pyproject.toml` or
`requirements.txt` (python); otherwise `unknown` and no hints. Treat hints as
investigation prompts, not mandatory findings. Also read the project's
`CLAUDE.md` and every `.claude/rules/*.md` fail-safe as repository guidance.

## Select applicable perspectives

Standard perspectives:

- `code-reviewer`;
- `spec-reviewer` and `code-quality-reviewer` (the independent specification
  and quality gates);
- `test-coverage-reviewer`;
- `design-alignment-reviewer`;
- `scope-reviewer`;
- `code-architect`.

Adversarial perspectives:

- `adversarial-api-reviewer`;
- `adversarial-robustness-reviewer`;
- `adversarial-performance-reviewer`;
- `adversarial-tests-reviewer`.

For a Task PR, apply the approved per-task mode:

- `focused`: require the one combined `code-reviewer` gate, require
  `test-coverage-reviewer` when that Task PR changes behavior or tests, and run
  only additional task perspectives explicitly recorded by policy.
- `adaptive`: require independent `spec-reviewer` and `code-quality-reviewer`
  task gates and any triggered task perspective selected for recorded risk.
- `deep`: require independent `spec-reviewer` and `code-quality-reviewer` task
  gates and every perspective applicable to that Task PR's artifact and
  observed risks.

For targeted integration review, run only the required or triggered integration
perspectives named by the approved policy. Do not replay each Task PR reviewer
over the combined tree. Whenever an adversarial perspective runs, require
`adversarial-integrator` for that target.

For every mode, preserve skipped perspectives and their reasons. Reject a `deep`
policy that skips an applicable perspective. Whenever any adversarial
perspective runs, require `adversarial-integrator`.

Without an approved policy, select the same perspectives by observed
applicability: general review always; test coverage for behavior or test changes;
design alignment when an approved Design Doc or Feature Contract exists; scope
review for a plan or narrow migration; architecture for material responsibility
changes; adversarial profiles only for their corresponding concrete risk. Record
every run and skip with reasons.

## Preserve independence and capacity

An approved `focused` policy may use a complete lead Task PR review when the
user prohibits agents. `Adaptive` and `deep` independent perspectives cannot be
replaced by sequential lead passes. A no-agent conflict is `Escalate` for
coordinator review or a standalone limitation.

For standalone review without an approved policy, when the user prohibits
agents, the lead may execute each selected read-only perspective and any required
adversarial integration sequentially. Report the result as `standalone-only`; it
is never approved-policy completion or coordinator completion evidence.

Otherwise dispatch each already-selected perspective with the Agent tool:
`Agent({ subagent_type: "<profile>", model: "sonnet", prompt: <complete reviewer message> })`.
Pass no `name` (a named spawn becomes a persistent teammate; reviewers must be
one-shot subagents that return findings as the tool result), run every
reviewer in the foreground (never `run_in_background: true`), and pass
`model: "sonnet"` explicitly at every call even though the definitions pin it.
Launch all selected reviewers in one message so they run concurrently, up to
the capacity recorded in the approved Review policy (default: at most four
concurrent subagents per session); queue the rest in deterministic policy
order. Do not reduce scope, independence, or applicable breadth. An unavailable
required reviewer returns `BLOCKED` with the role, observed capacity, gap, and
re-entry condition.

Use the named profiles in `~/.claude/agents/`; when a profile cannot be
selected, provide a complete fallback role prompt carrying the same read-only
contract. Reviewers and integrators do not edit files or spawn descendants.
Include `ultrathink` in every adversarial reviewer prompt so it uses extended
thinking.

Record dispatch and completion times for every reviewer and append them to
`~/.claude/usage-data/review-timings/<ISO-8601-timestamp>.json`
(`started_at`, `scope` with `changed_files` and `primary_language`, and one
`agents` entry per reviewer with `name`, `started_at`, `completed_at`,
`duration_ms`), creating the directory when absent.

## Give every reviewer artifact-aware evidence

Pass directly to every selected reviewer:

- target kind; exact workspace, branch, planned base, merge base, current head,
  range, composed tree, or bounded standalone files; diff, status, and changed
  files;
- exact authority identity, source path or in-memory identity, and currentness
  evidence for the approved planned contracts, complete lightweight combined
  contract, or exact eligible legacy plan; plus the clauses and integration
  obligations applicable to that perspective;
- approved scope and non-goals;
- the same Review context and Review policy when available, including any
  `project-rules.md` identifiers the plan references as non-problems or Must
  Fix grounds;
- fresh verification commands and observed results;
- relevant dependency and Task PR outcomes, prior triage decisions, concerns,
  and gaps;
- repository guidance and language hints;
- that reviewer's selected perspective and output expectations (adversarial
  reviewers return the YAML findings schema their definitions describe,
  including `already_decided_check` and `confidence`; they report every genuine
  concern and let the integrator filter).

Do not create another wrapper identity or repeat the evidence in competing
formats. Before dispatch, confirm the current head, range, diff, status, and
changed files are unchanged.

Keep every exact authority source directly available to every reviewer. Eagerly
load complete sources for design-alignment, scope, or another perspective that
owns whole-contract coverage. Other perspectives start with applicable clauses
and inspect additional source sections when their evidence requires it; do not
copy or require an unconditional reread of unrelated unchanged prose.

## Apply the common Acceptance threshold

Every finding must include severity `Must Fix` or `Should Improve`, file and
line, concrete observed or reachable behavior, violated requirement or quality
consequence, evidence, impact, proportionate correction, and confidence.

Keep a finding only when it:

- applies to the artifact and consumer model in the Review context;
- identifies a concrete reachable behavior or approved-contract violation;
- cites evidence and a material consequence;
- proposes a proportionate correction.

`Should Improve` requires a concrete maintainability consequence or measurable
repeated cost. Drop preference-only, speculative, second-order,
artifact-inapplicable, optional-polish, generic-best-practice, and unsupported
findings.

An approved non-problem may be revisited only with materially new evidence of a
concrete reachable failure or contract violation. A rephrasing, changed line
number, or imagined future consumer is not new evidence. Apply the same rule to
prior `Push back` decisions.

A suggestion to add a state machine, schema, identity mechanism, or another
architectural system is not a `Fix` without a proven in-scope violation and
proportionate need. Drop it when it is unsupported optional design. When it
exposes a material user-owned architecture choice, return `BLOCKED` with a
design gap so the coordinator can `Escalate`; do not label it `Must Fix` or
`Should Improve`.

## Integrate adversarial review

When required, give `adversarial-integrator` (dispatched under the same rules:
name-less, foreground, `model: "sonnet"`) the same exact planned, lightweight,
or eligible legacy authority identity and direct source access, plus Review
context, target evidence, verification, policy, prior triage decisions, and
adversarial reports. The integrator remains read-only, deduplicates, verifies
evidence, resolves contradictions, and drops unsupported, second-order, and
artifact-inapplicable findings. It does not invent findings or lower the
Acceptance threshold. Its integrated section returns inline as its final text,
never as an Artifact or a file.

## Report

Merge duplicates and report in Japanese:

- target form; workspace, branch, base, merge base, starting and ending head,
  exact range, composed tree, or bounded fileset;
- starting and ending status, diff scope, and changed files;
- Review context and disclosed standalone assumptions;
- approved mode or `none`, observed risks, and policy reconciliation;
- fresh verification commands and results inspected;
- assigned Feature and Task Contract observations, targeted integration
  alignment, or eligible legacy criteria and original-authority alignment
  inspected;
- reviewers run, queued, and skipped with reasons;
- reviewer and integrator outcomes;
- accepted Must Fix and Should Improve findings, each with file and line, a
  focused code snippet, the issue, a concrete suggestion, and its trade-off;
- separate policy or design gaps requiring coordinator `Escalate`;
- residual risk, limitations, every gap, and exact re-entry condition;
- verdict exactly `CLEAN`, `FINDINGS`, or `BLOCKED`.

Return `CLEAN` only when all required applicable perspectives completed, the
common Acceptance threshold leaves no finding, verification is fresh, and the
target is unchanged, with no policy or design gap. A clean review is a valid
result.

Read current head and status again before reporting. If either changed, return
`BLOCKED` with preserved reviewer evidence and the stale-state gap. Do not start
triage, edit code, commit, or advance phases. Coordinator review returns evidence
to the coordinator; standalone review reports directly to the requester.

## Append the per-task review report

For a coordinator-managed target whose plan directory is known, append the
report to a local HTML file beside the plan before returning: for a Task PR,
`docs/plans/<feature>/review-<task>.html` where `<task>` is the task's plan
identifier; for a targeted integration review, `review-integration.html`. One
file per task keeps concurrent Task sessions from writing the same file. Skip
this step, and say so, for standalone review or when no plan directory exists.

Keep the file uncommitted: register its exact repository-relative path in the
clone-local exclude file, never in the tracked `.gitignore`:

```sh
REPORT="docs/plans/<feature>/review-<task>.html"
EXCLUDE="$(git rev-parse --path-format=absolute --git-common-dir)/info/exclude"
grep -qxF "$REPORT" "$EXCLUDE" 2>/dev/null || echo "$REPORT" >> "$EXCLUDE"
```

On first creation write this template, filling the feature and branch:

```html
<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Review Report: [feature] / [task] ([branch])</title>
<style>
  :root { color-scheme: light dark; }
  body { font-family: -apple-system, "Hiragino Sans", sans-serif; max-width: 60rem; margin: 2rem auto; padding: 0 1rem; line-height: 1.7; }
  section.iteration { border-top: 2px solid rgba(128,128,128,.4); margin-top: 2.5rem; padding-top: 1rem; }
  pre { background: rgba(128,128,128,.12); padding: .75rem; border-radius: 6px; overflow-x: auto; }
  code { font-family: ui-monospace, "SF Mono", Menlo, monospace; font-size: .9em; }
</style>
</head>
<body>
<h1>Review Report: [feature] / [task] ([branch])</h1>
<main>
</main>
</body>
</html>
```

Then insert a new `<section class="iteration">` immediately before the closing
`</main>`, numbered by counting existing iteration sections plus one, containing
an `<h2>` with the iteration number and date, the report above, and the head
and range reviewed. Convert the markdown mechanically (`##`/`###` to
`<h3>`/`<h4>`, code fences to `<pre><code>`), escaping `&`, then `<`, then `>`
in every transcribed body so that HTML literals in the reviewed diff cannot
corrupt the document or the `</main>` anchor. This is a transcription of the
report already produced, not a rewrite. Never publish the report as an
Artifact.
