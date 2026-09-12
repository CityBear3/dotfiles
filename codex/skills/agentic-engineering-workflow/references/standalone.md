# Standalone checks

## Route standalone read-only checks

Treat a user-requested verification or review outside a planned, lightweight,
integration-only, or eligible legacy Task gate as standalone authority. Resolve
an exact committed range, current index/worktree snapshot, or explicit bounded
fileset and pass it to `verify` or `review`. Standalone is not a CLI, session,
branch, or worktree mode and does not require Herdr, a Task Contract, DAG, or
PR topology.

The root owns the standalone target and its native verification-runner,
reviewer and conditional finding-integrator leaves directly. Runtime admission determines which selected
roles start; retain a rejected spawn as pending and retry after a completion or
mailbox event without reducing selected scope. When the user explicitly
prohibits agents, the lead may run compatible checks and perspectives
sequentially. Label the original result `standalone-only`. It is not Task or
Feature Acceptance; any later use must pass the [evidence-applicability and coverage mapping](evidence.md#reuse-evidence-by-applicability). Lead-only checks cannot supply an independent gate.
