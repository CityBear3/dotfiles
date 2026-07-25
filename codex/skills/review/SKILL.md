---
name: review
description: Run an evidence-based, capacity-aware review of a branch or file set using applicable Codex reviewer profiles and language hints, then synthesize and triage findings. Use after verification passes or when the user requests comprehensive review.
---

# Review

Review the requested scope, not the entire repository by default.

## Build the context

Resolve:

- base and head commits or explicit file set;
- changed files and primary language;
- approved Design Doc, plan, alternatives, and non-goals;
- repository `AGENTS.md` guidance;
- fresh verification evidence.

Load `hints/<primary-language>.md` when present. Treat hints as prompts for investigation, not mandatory findings.

## Select reviewers

Always consider:

- `code-reviewer` for correctness and maintainability;
- `test-coverage-reviewer` when behavior or tests changed.

Add only when applicable:

- `design-alignment-reviewer` for an approved Design Doc;
- `scope-reviewer` for an implementation plan or narrow migration;
- `code-architect` for material responsibility or dependency changes;
- adversarial API, robustness, performance, and tests reviewers when those surfaces changed or the plan explicitly requests them.

Use `list_agents` to build a queue. Count the lead, respect configured maximum six and lower observed runtime capacity, and run independent read-only reviewers concurrently only while slots are free. Never reduce review scope silently because capacity is lower; queue remaining reviewers.

When a named profile is selectable, use it. Otherwise provide a complete fallback prompt containing the profile's role, context, constraints, evidence rules, and output schema.

## Evidence standard

Every finding must include:

- severity;
- file and line;
- concrete observed behavior or reachable scenario;
- violated requirement or quality consequence;
- specific correction;
- confidence when reachability is uncertain.

Do not manufacture findings. Drop preference-only comments and findings that merely contest an approved decision without new evidence.

## Adversarial integration

After applicable adversarial reviewers finish, use `adversarial-integrator` or its complete fallback prompt to deduplicate, verify evidence, normalize severity, and resolve contradictions. The integrator remains read-only and does not invent new findings.

## Synthesize and triage

Merge duplicates and classify each surviving item with `receiving-code-review`:

- push back with decision/evidence;
- fix within approved scope;
- escalate when a design or scope decision is required.

Report in Japanese:

- scope and context loaded;
- reviewers run and queued/skipped with reasons;
- Must Fix and Should Improve findings;
- positive observations only when useful;
- triage outcome;
- clean or changes-required verdict.

Do not edit code from this skill unless the user separately authorizes fixes.
