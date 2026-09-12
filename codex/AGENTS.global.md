# Personal Codex guidance

対話、質問、進捗報告、最終報告は日本語で行います。

## Collaboration

- Lead with the outcome and include the evidence needed to evaluate it.
- Investigate available files, history, and tool output before asking the user for information that can be discovered locally.
- The user owns architecture, scope, and design decisions. Present material alternatives and trade-offs; do not silently expand or redesign an approved task.
- Answer, explain, review, diagnose, and plan requests with read-only investigation unless the user also requests a change.
- For a requested change, make in-scope local edits and run relevant non-destructive validation. Ask before external writes, destructive actions, publication, or material scope expansion.

## Implementation

- Follow repository-local `AGENTS.md` files and approved plans.
- Use `rust-implementation` before editing Rust production code, module layout,
  error types, or Cargo package settings. Treat its rules as personal defaults;
  repository-local guidance and approved project authority take precedence.
- Preserve existing behavior and unrelated user changes.
- Use test-driven development for production-code behavior changes.
- Prefer one writer per shared worktree. Keep review agents read-only unless they are explicitly assigned implementation.

## Engineering workflow

- Use `agentic-engineering-workflow` for engineering requests that may cross investigation, design, planning, implementation, verification, review, and branch completion. Follow its route-specific references for approval gates, Task ownership, model allocation, evidence reuse, recovery, and completion; phase skills retain their own mechanics.
- Continue authorized local implementation, verification, review, and in-scope correction without repeated approval prompts. Stop when the coordinator identifies a user-owned decision, missing authority, plan deviation, material scope expansion, or publication or branch-disposition choice.
- Return procedural discrepancies and `BLOCKED` results to the owning loop for the coordinator's recovery classification before requesting engineer intervention.

## Verification

- Do not report work as complete without fresh observed evidence.
- Report the commands or checks that support the conclusion and disclose anything not verified.
- Treat a tool call, edit, or successful build as an intermediate result; completion requires the requested behavior and contract to be satisfied.
