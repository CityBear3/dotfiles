# [Design Doc] Rust Codex Installer Addendum

- Author: Repository owner
- Date: 2026-07-19
- Last updated: 2026-07-23
- Status: Approved
- Related plans:
  - `docs/plans/2026-07-22-rust-codex-installer-rebuild.md`
  - `docs/plans/2026-07-22-codex-workflow-assets.md`

## Context and Scope

The Codex configuration migration originally selected a Python 3.11 installer so
that a fresh checkout could use only the standard library. During implementation,
the installer grew to cover stable filesystem snapshots, ownership manifests,
cross-device archives, rollback, restore bundles, and failure injection. The
install path and the generated restore program evolved as separate transaction
implementations. Review found that a restore failure concurrent with an external
edit could move the edited generation into staging and then delete it during
rollback cleanup. Additional patching can close that individual race, but it does
not remove the structural risk of two transaction engines drifting apart.

This addendum replaces the Python installer design with a macOS-only Rust
installer. Rust is not treated as an automatic filesystem-safety guarantee. The
reliability improvement comes from using one typed transaction engine for install,
restore, and automatic rollback; persisting write-ahead intent before filesystem
transitions; and using synchronized, no-replace publication for managed entries.
The installer is a personal workstation tool rather than a general concurrent
configuration service. Its design therefore favors explicit operating assumptions
and a small recovery protocol over continuous filesystem-generation proofs.

### Goals

The Rust installer must preserve the original migration's ownership and
configuration guarantees. It updates only the three managed root configuration
keys and two managed agent keys, while retaining existing Context Window, Auto
Compact, statusline, MCP, permission, hook, authentication, provider, and other
unmanaged settings byte-for-byte. It installs only repository-owned top-level
guidance, skills, and custom agents. Previously owned stale assets may be archived,
but `.system` content and every manifest-external asset remain outside installer
ownership.

Install, restore, and interrupted-transaction rollback must use the same state
machine and filesystem primitives. Mutating operations for one Codex home are
serialized by one exclusive file lock. The operator does not run another installer
or edit managed destinations from the start of a mutating operation until any
unfinished transaction has been rolled back. A process crash or power loss must
leave enough durable journal state for a later mutating invocation to classify each
recorded transition from its phase and transaction-owned paths, then resume rollback
without overwriting an unrecorded path.

The migration must retain a non-mutating dry-run for Codex targets and state. It
must also retain the adaptive thread selection, explicit target-directory options,
manifest-limited pruning, conflict adoption, deterministic operator messages, and
testable failure injection established by the Python implementation. Dry-run does
not acquire or create the operation lock and does not promise a coherent view while
a mutating invocation is running.

### Non-Goals

The installer does not support Linux, Windows, or WSL in this version. The owner's
WSL environments do not run Codex, so non-macOS platforms must reject mutating
operations before changing destinations. Cross-platform business logic may remain
testable, but only the macOS filesystem backend receives the safety guarantee.

The design does not provide offline bootstrap, vendor Cargo dependencies, or store
compiled installer binaries in backups. It does not modify anything under
`claude/`, manage the existing Codex statusline, set explicit Context Window or
Auto Compact values, package a plugin, or retain Python as a runtime fallback after
the Rust cutover.

The installer does not create or initialize a Codex home. Codex must already be
installed and its configured home must exist as a directory. The design does not
support external edits during a transaction, arbitrary historical-backup selection,
an explicit recover command, or mutation-time defense against an operator replacing
roots, ancestors, or managed entries. Static preflight still rejects unsafe paths,
symlinks, special files, and unmanaged conflicts before mutation.

## Overview

The repository gains an independent Rust crate under `codex/installer/` with a
committed `Cargo.lock`. `codex/install.sh` remains a thin launcher and invokes the
crate with `cargo run --locked --release`. Every operation therefore builds or
validates the current checkout before execution; Cargo's incremental cache avoids a
full rebuild when inputs are unchanged. Cargo registry and build-cache writes are
bootstrap effects and are explicitly outside the installer's dry-run guarantee.
After the Rust process starts, dry-run may not create or change Codex homes, skills
homes, installer state, manifests, backups, staging siblings, or recovery data.

The binary exposes install and dry-run behavior compatible with the existing CLI,
plus `restore [--state-dir <PATH>]` for the latest backup. It does not expose an
explicit recover operation: every mutating invocation resolves an unfinished
transaction before planning new work. Backups do not contain the executable, and
the current checkout is rebuilt for restoration. The journal schemas remain
versioned and an unknown version fails before filesystem mutation. The V1 schemas
defined by this revision are pre-release contracts; earlier development artifacts
receive no compatibility reader or migration.

The default installer state root is `$XDG_STATE_HOME/dotfiles-codex-installer` when
`XDG_STATE_HOME` is set, and `~/.local/state/dotfiles-codex-installer` otherwise.
The tool name is the top-level state namespace so that a future Codex release can
use its own `codex` state directory without colliding with installer manifests,
journals, backups, or recovery data. `--state-dir` continues to override the
complete root explicitly.

The following state machine is shared by install and restore:

```text
planned -> prepared -> applying -> committed -> cleaning_up -> complete
                         |
                         v
                  rolling_back -> rolled_back
```

`complete` and `rolled_back` are clean terminal states. Failure before `committed`
enters rollback; failure after `committed` resumes publication and cleanup rather
than undoing the installed state. If recorded phases and path existence cannot be
classified, the invocation fails closed and leaves the journal plus all referenced
paths available for diagnosis.

## Detailed design

### Operation and locking model

Install and restore require an existing Codex home and open
`<codex-home>/codex-manifest-installer.lock` for reading and writing, creating only
the lock file when it is absent. The process calls the standard library's blocking
exclusive `File::lock` and retains the open file through startup rollback, planning,
commit or rollback, backup publication, and cleanup. A second mutating invocation
for the same Codex home waits for the first and then reads state afresh. The lock file
is not removed on exit and is never part of manifest ownership, backup payload, or
restore mutation.

The lock contains no PID or operation metadata and has no state-path hash. The
installer does not impose owner or permission requirements on the Codex home or
revalidate the lock inode and ancestor chain. Different `--codex-home` values that
share another destination are outside the concurrency contract. Restore may read
the latest backup journal once to locate the Codex home, but it acquires the lock
and then resolves the latest marker and journal again before trusting any
transactional state. Dry-run does not open the lock file.

### Content snapshots and mutation boundary

Preflight captures content needed for configuration merging, no-op classification,
unmanaged-conflict detection, staging, and backup-copy verification. Content
fingerprints serve those purposes only; inode, device, modification-time, and
ancestor-generation metadata do not grant mutation authority and are not persisted
as transaction progress. The engine does not repeatedly prove that a path still
names the same filesystem generation during a transaction.

Requested roots are normalized before planning and source, Codex-home, skills-home,
and state roots may not overlap. Every persisted managed locator is root-relative
and rejects absolute paths, empty components, `.` and `..`. Static preflight rejects
symlinks and special files in managed source or destination trees, and manifest
ownership or explicit adoption is required before an existing destination can be
changed. `.system` and every manifest-external asset remain excluded from mutation.

No-replace rename semantics are used when publishing a destination. V1 requires
every source and destination used by the transaction move protocol to be on the
same filesystem. If the platform reports `EXDEV`, the source and destination remain
unchanged, the already-durable pending intent remains in the canonical journal, and
the operation returns an unsupported-cross-device error. Synchronous rollback or
the next mutating invocation classifies that source-only state as not applied,
clears the pending intent, and removes recorded transaction-owned staging. Backup
payload capture and materialization are separate backup-store responsibilities and
are not a transaction move fallback. A directory is removed only when the
transaction records responsibility for it and it is empty. The engine never
recursively deletes or synchronizes the skills or agents parent directory.

### Journal, commit, and crash recovery

An executing transaction owns one versioned write-ahead journal under the configured
state directory. The journal records the transaction ID, normalized root paths,
root-relative live, staging, and tombstone locators, entry and transaction
phases, and any pending move intent. It does not contain filesystem
generation authority, retained-generation records, or quarantine protocols.

The engine or application creates and makes the selected state directory durable
after full mutating preflight and before constructing a WAL store. The WAL store
requires that directory to exist as an ordinary no-symlink directory and never
creates it. It validates or creates the direct `transaction` child without following
a symlink or accepting a special file. Before creating a WAL, it synchronizes the
state directory after validating or creating that child, so recovery after a crash
between directory creation and parent synchronization closes that durability gap.

Transaction-owned entry locators use only
`StateDir/transaction/work/<transaction-id>/<role>/...`, where role is exactly
`stage` or `tombstone` and must match the corresponding record field.
These locators, all live locators, and entries are pairwise non-overlapping. Live
locators are limited to `CodexHome/config.toml`, `CodexHome/AGENTS.md`, a direct
safe `CodexHome/agents/<name>.toml`, a direct safe `SkillsHome/<name>` other than
`.system`, or `StateDir/manifest-v1.json`, according to category. Categories are
`config`, `global_agents`, `skill`, `agent`, and `manifest`; operations are
`create`, `replace`, and `remove`. No-op actions are omitted from the WAL. The
operation, required stage/tombstone/fingerprint fields, entry phase, transaction
phase, and pending edge must be semantically consistent before load, create,
replace, rollback, or cleanup can mutate a referenced path.
Every existing transaction-locator ancestor from `transaction/work` through the
transaction ID, role, and any nested parent is checked with no-follow metadata and
must be an ordinary directory. This check runs while loading or replacing the WAL
and again immediately before move, rollback, or cleanup activity.

Each journal revision is written to a temporary file, synchronized, atomically
renamed over the single canonical journal, and followed by parent-directory
synchronization before the corresponding filesystem transition begins. A leftover
temporary journal is not an alternate authority. Startup uses the canonical journal
and discards an ordinary incomplete temporary replacement with directory
synchronization. An absent canonical journal means no active transaction; a corrupt
or unknown canonical journal fails before any referenced path is mutated.

If a WAL replacement reports an error after the canonical rename may have happened,
the caller reloads the canonical WAL and replaces its in-memory value with that
authority before returning. This applies to pending-intent publication, target-phase
completion, and rollback phase changes. If canonical reload itself fails, the error
explicitly prohibits further mutation with the unresolved in-memory value.

Every move intent records one entry index and its post-move target phase. The same
WAL revision keeps the entry at its old phase and makes that intent durable before
an exclusive rename. After the rename, both parent directories are synchronized;
one WAL replacement then clears the intent and advances the entry to its target
phase atomically. A caller cannot advance the phase separately. Pending intents are
accepted only for one exact live/stage/tombstone edge of the indexed entry:

| Transaction | Operation | Edge | Current entry state | Target entry state |
|---|---|---|---|---|
| applying | create | stage -> live | staged | desired installed |
| applying | replace | live -> tombstone | staged | prior isolated |
| applying | replace | stage -> live | prior isolated | desired installed |
| applying | remove | live -> tombstone | planned | desired installed |
| rolling back | create | live -> stage | desired installed | staged |
| rolling back | replace | live -> stage | desired installed | prior isolated |
| rolling back | replace | tombstone -> live | prior isolated | staged |
| rolling back | remove | tombstone -> live | desired installed | planned |

With the exclusive-lock and no-external-edit assumptions, recovery
classifies an interrupted rename from path existence: source-only means the rename
has not run, and destination-only means it has run. Both paths present or both paths
absent violate the finite-state protocol, so recovery leaves them untouched and
fails closed. An `EXDEV` result leaves the source-only state and durable intent
unchanged. Rollback classifies it as not applied, clears the intent, cleans recorded
staging, and never creates a copy-publication subprotocol in V1.

Before its first mutation, rollback observes any pending move and computes each
entry's effective phase: an applied move uses its target phase, while a not-applied
move retains its old phase. It then globally preflights every live, stage, and
tombstone tree with no-follow traversal and requires this exact existence topology:

| Operation | Effective entry phase | Live | Stage | Tombstone |
|---|---|---|---|---|
| create | planned | absent | optional | not recorded |
| create | staged | absent | optional | not recorded |
| create | desired installed | present | absent | not recorded |
| replace | planned | present | optional | absent |
| replace | staged | present | optional | absent |
| replace | prior isolated | absent | present | present |
| replace | desired installed | present | absent | present |
| remove | planned | present | not recorded | absent |
| remove | desired installed | absent | not recorded | present |

Here, optional permits either an absent stage or transaction-owned partial staging
that rollback will remove. Any other ordinary-file presence is inconsistent, not
cleanup material. A failure in any entry leaves every entry and the canonical WAL
unchanged.

Contract tests cover the prohibited ordinary-file presence for every topology row
independently. Separate role-oriented tests cover live, stage, and tombstone trees
with representative missing, symlink, and FIFO/special failures; they do not
multiply every topology row by every file kind. Every failure preserves referenced
paths and the canonical WAL bytes.

Before planning new mutable work, the engine rolls back any pre-commit canonical
journal. An ordinary runtime error attempts the same rollback before returning. A
committed journal instead finishes backup publication and cleanup. If rollback or
cleanup fails, the journal remains and the next mutating invocation retries it
before starting a new transaction. There is no separate recover command.

Fault tests at both canonical-publication boundaries continue from the exact
generated directory and WAL state. They discard the faulting store, construct a
fresh store, call startup recovery, and assert terminal rollback plus exact live,
stage, and WAL state.

The manifest is the last live destination switched before commit. A successful
install publishes an immutable restore journal and payload under a transaction-ID
backup directory. `backups/latest` is a small marker naming the single retained
backup. Before changing live destinations, the engine ensures that the latest backup
represents the current state: it may reuse an exact existing match, otherwise it
makes a new backup durable. After committing the live transaction, it atomically
selects a new backup marker when needed and only then removes the previous backup. A
crash during cleanup may temporarily leave more than one backup directory; the
marker defines the only restorable backup and the next mutating invocation removes
stale directories. The system never deletes the previous backup before the
replacement is durable and selected.

### Shared rollback and restore semantics

Rollback is a reverse transaction executed by the same engine and move protocol as
the forward operation. Create moves desired live content back to staging before
idempotent cleanup. Replace moves desired live content back to staging, restores the
tombstone to live, and then cleans staging. Remove restores the tombstone directly
to live. Every inverse rename uses a durable pending intent; rollback never directly
deletes desired live content. Paths that do not match the recorded finite state
are reported and left in place; rollback does not invent ownership, quarantine
unknown content, or delete an ambiguous path.

Before rollback changes its first path or WAL phase, it validates required live or
tombstone content according to operation and phase and preflights every existing
transaction-owned stage/tombstone tree for every entry. Committed cleanup likewise
preflights every entry before deleting its first tree. Tree cleanup first builds
complete bottom-up plans and only then removes ordinary files or empty directories,
so a later invalid entry cannot leave an earlier entry partially cleaned.

Restore has no arbitrary backup-path option. It resolves `backups/latest` in the
default or explicitly selected state directory, performs complete preflight and
staging, restores assets before the manifest, and can itself be rolled back if it
does not commit. If backup A replaces problematic live state B successfully, A
remains the latest backup and B is not promoted into backup history. B exists only
in transaction-owned rollback locations while restore is incomplete and is removed
after commit. A later install from restored state A may reuse A as the backup of its
pre-install state.

### Configuration merge contract

The text-preserving configuration merger moves into the Rust crate. It continues to
validate the existing document, managed fragment, and final candidate as TOML while
editing only ordinary single-line assignments for the managed keys. It does not
serialize the complete document. Comments, whitespace, tables, and unmanaged
assignments retain their original bytes apart from normalization to one trailing
newline.

The managed root keys are `model`, `model_reasoning_effort`, and
`plan_mode_reasoning_effort`; the managed `[agents]` keys are `max_threads` and
`max_depth`. `model_context_window`, `model_auto_compact_token_limit`, and
`model_auto_compact_token_limit_scope` are explicitly unmanaged. Existing values for
those keys are preserved, and missing values remain absent so Codex uses its native
defaults.

### Build and migration

The launcher builds the current checked-out Rust source with the committed lockfile
for install and restore. Backups retain only data and versioned journals, not
binaries or source snapshots. This revision replaces the unreleased development V1
journal formats in place and does not migrate artifacts produced by earlier commits.
Unknown schema versions fail before mutation. Compatibility policy for any future
schema change is a separate design decision rather than functionality implemented
speculatively in this version.

The earlier Python prototype already remains only in Git history and is never a
runtime fallback. The current unreleased Rust generation-tracking core may serve as
a short-lived behavioral oracle for retained configuration, inventory, and CLI
defaults while responsibility-focused replacement modules are built beside it.
Cutover occurs only after the replacement suite covers every retained configuration
and installer contract, the simple operation lock, durable journal boundaries,
automatic rollback, latest-backup restore, the normal-binary end-to-end installation,
and the current-machine dry-run. The cutover commit removes the rejected Rust
filesystem, planner, transaction, and obsolete integration-test contracts after
equivalent public-boundary tests exist. Existing Git commits remain intact as
implementation history.

## Cross-cutting Concerns

Safety tests use real temporary directories together with a narrow filesystem
interface and named failure checkpoints. The suite must cover blocking serialization
of mutating operations, lock-free dry-run, static symlink and special-file rejection,
unmanaged preservation, `EXDEV` non-mutation followed by rollback or restart, every
durable single-journal boundary, each legal and illegal move-existence state,
process termination followed by automatic rollback, latest-backup publication,
restore, and cleanup failure.
Race injection for external edits, root replacement, or ancestor replacement is not
part of the supported contract.

Operator output is part of the recovery model. Errors distinguish preflight failure,
clean rollback, rollback failure, and committed operation with cleanup warning. If
the finite-state protocol cannot classify a path, output includes the transaction
ID, canonical journal path, and affected live, staging, tombstone, backup, and
other recorded entry paths. The journal replacement `.tmp` file is an implementation
detail rather than a transaction locator or alternate authority. A clean result is
never reported while an incomplete required transaction remains.

The Rust crate remains independent from `claude/statusline`; no root Cargo workspace
is introduced. macOS-specific filesystem behavior is isolated behind a backend so
that the transaction state machine can be unit-tested without claiming Linux or WSL
support. Non-macOS mutation attempts fail before destination changes.

Dependencies are fetched online by Cargo and pinned by `Cargo.lock`. The design
prefers a small set of maintained crates for CLI parsing, serialization, temporary
storage, and macOS system calls, but dependency selection is an implementation-plan
decision. Authentication data, configuration values, and file contents must not be
written into logs or journals beyond content fingerprints and paths required for
planning, backup verification, rollback, and restore.

## Alternatives

Continuing to harden the Python implementation would require the least new code and
could fix the currently known race. It was rejected as the final architecture because
install and generated restore remain separate transaction implementations, which
allows their failure semantics to drift. The earlier Python code was useful as an
oracle during the initial migration and remains available through Git history.

Moving only filesystem operations into Rust while retaining the Python configuration
merger would reduce initial porting work. It was rejected as the steady state because
the subprocess boundary would split snapshot ownership and error handling across two
runtimes. The final system would still need Python and Rust packaging and tests.

Storing the compiled binary in every backup would make each restore independent of
the current checkout. It was rejected because the owner does not want binary copies
in backups and every target machine already provides Cargo with online dependency
access. The chosen alternative rebuilds current source and keeps persisted schemas
versioned so unsupported data fails before mutation.

Rebuilding the exact Git revision that performed an install would avoid journal
reader evolution but would make restoration depend on repository history, worktree
construction, and the continued availability of that revision. It was rejected in
favor of a versioned journal protocol and the current checkout.

Supporting Linux or WSL in the first Rust version would require another filesystem
backend and platform-specific integration tests. It was deferred because Codex is
not used in the owner's WSL environments.

Omitting a lock entirely would match the normal single-operator workflow, but one
persistent file in the existing Codex home cheaply prevents accidental concurrent
installer mutations. A state-path hash, external temporary lock hierarchy, owner and
mode enforcement, holder metadata, and repeated inode validation were rejected
because their complexity does not match the personal-workstation threat model.
Dry-run remains lock-free so observation does not create files.

Persisting device, inode, timestamp, ancestor-chain, and content-digest generations
through every transition would detect external changes during execution. It was
rejected because the operator does not edit managed paths until an unfinished
transaction is resolved. Content fingerprints remain only where content comparison
or copy verification requires them; static no-follow and ownership preflight remain
because they protect against pre-existing unsafe or unmanaged entries.

A two-slot WAL with adjacent-revision lineage and salvage selection would retain a
second journal authority during replacement. It was rejected in favor of one
canonical journal updated by synchronized atomic replacement. An absent canonical
journal means no transaction after an ordinary leftover `.tmp` is discarded; a
corrupt or semantically inconsistent canonical journal fails closed rather than
invoking a second recovery protocol.

Retaining every successful backup or allowing an arbitrary backup path would provide
history browsing. It was rejected because the required operation is one-step return
to the latest known-good state. The latest marker is published before an older
backup is pruned, so a crash may temporarily retain extra backup directories but
never selects them for restore.
