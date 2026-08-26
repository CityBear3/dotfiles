# Codex upgrade helper link bootstrap

## Context and scope

`codex/bin/codex-upgrade` is the repository-owned entry point for updating a
standalone Codex installation and restarting an app-server daemon only when it
was already running. The helper can currently be invoked from the repository,
but exposing it as `codex-upgrade` requires a separately documented manual
symlink under `~/.local/bin`.

The Rust installer deliberately owns a different boundary: it synchronizes
Codex configuration, personal skills, and custom agents, and it transactionally
backs up and restores those managed destinations. Its content and destination
models reject symlinks. Extending those models for this one workstation
bootstrap link would mix environment setup with Codex asset synchronization and
would unnecessarily expand the ownership manifest, backup, WAL, and restore
contracts.

This document amends the statement in the Rust installer V1 design that
`codex/install.sh` remains only a thin Cargo launcher. The launcher remains the
entry point for the Rust installer, but also owns the one repository-to-`PATH`
link described here. No other Rust installer V1 responsibility or safety
contract changes.

### Goals

- A successful Codex install exposes the tracked helper as
  `~/.local/bin/codex-upgrade` without a separate manual setup step.
- Keep workstation bootstrap behavior in `codex/install.sh` and keep the Rust
  installer limited to Codex configuration, skill, and agent synchronization.
- Make repeated installation idempotent and surface destination conflicts
  without overwriting user-owned entries.
- Preserve dry-run as a non-mutating preview of both the Rust install and the
  helper-link action.

### Non-goals

- General-purpose management of executables or symlinks under `~/.local/bin`.
- Adding a binary destination root, symlink payloads, or link ownership to the
  Rust installer's manifest, backup, WAL, or restore schemas.
- Removing or restoring the helper link during `restore`.
- Modifying `PATH`, shell startup files, or Homebrew-managed Codex behavior.
- Automatically replacing a destination that is not the exact expected link.
- Supporting a configurable helper-link destination in this change.

## Overview

`codex/install.sh` continues to forward the existing CLI unchanged to the Rust
installer. For a mutating install—the default invocation or the explicit
`install` command—the launcher first checks whether the helper link can be
created safely, runs the Rust installer, and creates the link only after the
Rust process succeeds.

The source is resolved from the launcher's own directory rather than the
caller's working directory:

```text
codex/bin/codex-upgrade
        │
        └── absolute symlink ──> ~/.local/bin/codex-upgrade
```

Dry-run reports the prospective link action without creating the destination
directory or link. Restore and help invocations remain Rust-only operations and
never inspect or mutate the helper link.

## Detailed design

### Responsibility boundary

The Rust process remains the sole owner of transactional Codex asset
synchronization. The shell launcher owns the helper link because it is a
workstation bootstrap concern: it publishes a repository executable into a
directory expected to be on the user's `PATH` but does not copy or synchronize
Codex-managed content.

The link is intentionally absent from Rust ownership and recovery metadata.
Rust installation, backup, restore, and interrupted-transaction recovery must
behave exactly as before whether the helper link exists or not.

### Invocation behavior

The launcher applies link behavior only to installation invocations:

| Invocation kind | Link behavior |
|---|---|
| Default or explicit mutating `install` | Preflight, then create after Rust succeeds |
| Install with `--dry-run` | Report the action without mutation |
| `restore` | No link inspection or mutation |
| Help | No link inspection or mutation |

All arguments are forwarded to the Rust CLI without translation. Invalid Rust
arguments continue to be diagnosed by the Rust CLI and must not create the
link.

### Link states

The source is the absolute path to `bin/codex-upgrade` resolved relative to
`codex/install.sh`. The destination is fixed at
`$HOME/.local/bin/codex-upgrade`.

The launcher recognizes three observable states:

| Destination state | Action | Result |
|---|---|---|
| Absent | `CREATE` | A mutating install may create the parent directory and link |
| Symlink with the exact expected target | `NO-OP` | Success without changing the link |
| Any other entry or symlink target | `CONFLICT` | Fail without replacing or removing it |

The expected source must exist as an executable regular file. A missing,
non-regular, or non-executable source is an error and prevents link mutation.
The launcher never uses force replacement semantics.

### Ordering and failure behavior

For a mutating install, the launcher performs a read-only source and destination
preflight before starting Rust. A known helper-link conflict therefore prevents
the Codex synchronization from starting. After a successful Rust install, the
launcher creates a missing parent directory and then creates the symlink without
overwriting an entry that may have appeared since preflight.

The shell bootstrap and Rust transaction are deliberately not one transaction.
If Rust fails, no helper link is created. If link creation fails after Rust
succeeds—for example because permissions changed or another process won a
race—the overall command fails while the completed Rust synchronization remains
valid. The user can resolve the link error and rerun the idempotent install; the
launcher does not roll back Rust-managed state.

Dry-run performs no helper-link filesystem mutation, including creation of
`~/.local/bin`. It reports the same `CREATE`, `NO-OP`, or `CONFLICT` decision a
mutating invocation would make. A conflict produces a failing result so it
cannot be mistaken for an installable plan.

### Restore and lifecycle

The link points into the working tree, so later edits to the tracked helper take
effect without reinstalling the link. The link remains present across Rust
`restore` operations because it is outside the Rust installer's ownership and
backup boundary.

Moving or deleting the repository may leave a stale link. A later install from
a different checkout treats that link as a conflict rather than silently
retargeting it. Resolving such a conflict is an explicit user action.

## Cross-cutting concerns

### Safety

The launcher has one fixed destination and never recursively changes
`~/.local/bin`. It does not overwrite files, directories, or unexpected
symlinks. Tests exercise link behavior under an isolated temporary `HOME` and
must not access the real user destination.

### Compatibility

The Rust CLI, schemas, destination roots, and restore behavior remain unchanged.
Existing callers that invoke `restore` or help observe no new filesystem side
effects. Existing installations with the exact repository link become a
`NO-OP`.

The launcher remains relocatable: the source path is derived from the launcher's
location, so it can be invoked from any working directory. The generated link is
absolute, matching the existing documented manual setup.

### Verification

The automated shell regression suite is intentionally limited to the behaviors
that protect the user's home from an incorrect launcher branch. It must
demonstrate:

- a successful install creates the expected absolute link only after Rust
  succeeds;
- Rust failure does not create the link;
- an existing exact link is a no-op;
- dry-run reports but does not create the directory or link;
- restore and help do not inspect or change the link;
- an unexpected destination is preserved and reported as a conflict.

The suite does not need a combinatorial matrix for every destination type and
invocation, a deterministic preflight-race reproduction, or duplicate default
and explicit-install journeys. Fresh verification still includes shell syntax,
the existing Rust installer and standalone-helper suites, inspection of the
final source and schema diff, and review of the safety paths that are not given
dedicated regression cases.

## Alternatives considered

### Manage a regular executable copy in Rust

Rejected because the helper is a workstation command published into `PATH`, not
a Codex configuration, skill, or agent asset. Adding it would broaden Rust
destination ownership and recovery schemas even if the payload remained a
regular file.

### Manage the symlink in Rust

Rejected for the same responsibility mismatch and because the current Rust
content model intentionally rejects symlinks. Correct transactional handling
would require new link identity, backup, restore, and no-follow contracts.

### Keep the manual README command

Rejected because a new checkout or machine would still require an easy-to-miss
bootstrap step even though `codex/install.sh` is already the normal installation
entry point.

### Let `codex-upgrade` install its own link

Rejected because it makes command deployment a side effect of the runtime
update helper and creates a circular first-use workflow: the command cannot
bootstrap the convenient command name before it is already located and run.
