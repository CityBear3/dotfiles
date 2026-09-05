# Codex configuration

This directory is the source of truth for the personal Codex bundle.

## Standalone Codex update helper

`bin/codex-upgrade` updates a standalone Codex installation and refreshes an app-server daemon that was already running. It targets the current standalone installation, where `codex update` performs the self-update; it does not add Homebrew-specific behavior.

The helper quietly probes `codex app-server daemon version` before updating. After a successful update, it restarts the daemon and prints its version only when that probe succeeded. Any probe failure is treated as the daemon being stopped or unreachable, so the helper leaves it stopped and prints a message instead of starting it. An update, restart, or post-restart version failure is returned to the caller.

Run it directly from the repository root when needed:

```sh
./codex/bin/codex-upgrade
```

The normal `install.sh` flow publishes the helper automatically when `$HOME/.local/bin` is already on `PATH`:

```sh
./codex/install.sh
codex-upgrade
```

The shell launcher creates the absolute link `$HOME/.local/bin/codex-upgrade` only after a successful Rust install. A dry-run reports `CREATE` or `NO-OP` without creating `$HOME/.local/bin` or the link. An existing file, directory, or symlink with another recorded target is reported as `CONFLICT`, is never replaced, and stops before the Rust install starts.

The link is a workstation bootstrap asset owned by `install.sh`, not by the Rust installer's manifest, backup, or restore process. Rust `restore` therefore leaves it unchanged. Because it points into this checkout, tracked helper edits take effect immediately; moving the checkout leaves a stale link that a later install reports as a conflict instead of retargeting automatically.

The app-server daemon command is experimental, so this script remains the isolated place to update its invocation if the CLI changes.

## Rust installer

Use `install.sh` as the normal entry point. It resolves the installer manifest relative to its own location, so it can be invoked from any working directory.

Mutating `install` and `restore` commands are supported only on macOS; on non-macOS platforms, dry-run remains available, while mutating commands are explicitly rejected before any destination changes.

Prerequisites:

- Codex is already installed and the resolved Codex home exists.
- `rustc` and `cargo` are available. Cargo may access the network when dependencies are not already cached.

```sh
# Preview the default install.
./codex/install.sh --dry-run

# Install. The explicit `install` subcommand is optional.
./codex/install.sh install
./codex/install.sh

# Restore the selected pre-install backup.
./codex/install.sh restore

# Show the available install or restore options.
./codex/install.sh --help
./codex/install.sh restore --help
```

The install options are:

| Option | Meaning |
|---|---|
| `--dry-run` | Print `CREATE`, `REPLACE`, `REMOVE`, and `NO-OP` actions without changing the destination or state |
| `--adopt-existing` | Allow the first install to take ownership of existing same-name assets |
| `--agent-threads auto\|2..=32` | Select the value merged into `agents.max_threads`; the default is `auto` |
| `--codex-home PATH` | Override the Codex home |
| `--skills-home PATH` | Override the personal skills destination |
| `--state-dir PATH` | Override the installer state directory |

`restore` accepts only `--state-dir`. The selected backup records the Codex and skills roots that it belongs to, so restore obtains those roots from the backup rather than accepting new destination overrides.

Dry-run computes and renders an install plan without taking the operation lock or creating destination or state files. It also previews the shell-owned helper link without creating its parent directory or the link. The launcher itself still invokes Cargo, so it may create or update the repository-local Cargo build directory described below. The launcher uses `cargo run --quiet --locked --release` on every invocation instead of installing or copying a standalone Rust installer binary.

### Adopting existing assets

An existing same-name guidance file, skill, or agent is a conflict until the installer owns it. Review the dry-run and use `--adopt-existing` for the first intentional adoption:

```sh
./codex/install.sh --dry-run --adopt-existing
./codex/install.sh --adopt-existing
```

A successful install writes the ownership manifest. Later installs use that manifest to replace or remove only installer-owned names, so `--adopt-existing` is no longer needed for those assets.

### Agent concurrency

`--agent-threads` accepts `auto` or an integer from `2` through `32`; `auto` is the default. The selected value is written to `agents.max_threads`.

Automatic selection uses logical CPU count and physical memory:

- 4 threads when there are fewer than 8 logical CPUs or less than 16 GiB of memory.
- 8 threads when there are at least 12 logical CPUs and at least 32 GiB of memory.
- 6 threads otherwise.

### Agent hierarchy and inventory

The Feature Lead owns Feature authority, Task readiness, dependency release,
cross-Task staleness, integration and Feature Acceptance. Each new planned Task
uses an independent Codex root session in its approved Herdr worktree. That
Task Lead is both sole writer and local implementation/check/correction loop
owner; there is no separate implementer.

~~~text
Feature Lead (session defaults)
  |
  +-- Herdr --> Task session A / worktree A
  |               +-- native verification-runner
  |               +-- native spec + implementation-quality review
  |               `-- conditional native risk/integration
  |
  `-- Herdr --> Task session B / worktree B
                  `-- native checks owned by Task Lead B
~~~

Lightweight work is written directly by the Feature Lead and uses independent
native verification-runner and focused-reviewer gates. Feature integration and
standalone checks are also native leaves of the Feature session. Every new
native leaf has empty inherited history and no descendants.

There are eight logical roles, but only seven native profiles:

| Role | Model / effort | Boundary |
| --- | --- | --- |
| Task Lead | Sol/high or engineer-approved Astra/high | Independent Herdr session; sole Task writer |
| verification-runner | Luna/low | Declared commands/observations only |
| focused-reviewer | Sol/high | Lightweight spec, implementation and tests |
| spec-reviewer | Sol/high | Planned contract compliance |
| implementation-quality-reviewer | Sol/high | Implementation and test quality |
| risk-reviewer | Sol/xhigh | One selected risk perspective per invocation |
| finding-integrator | Sol/high | Conditional complex/conflicting findings |
| design-alignment-reviewer | Sol/xhigh | Composed/shared-boundary authority |

The shared Task root instructions live at
`skills/execute-task/references/task-lead.md` and install with that Skill.
They are supplied to both Sol and Astra roots through the launch handoff, not
through a fictitious native Task Lead profile. Herdr forwards explicit Codex
model, normal/Plan-mode effort and worktree arguments. Startup cannot silently
inherit the global Feature allocation. The plan records defaults and Task
overrides with required-quality/risk/cost rationale; the engineer confirms them
at plan approval. No extra startup approval, runtime promotion or fallback is
introduced. Feature Lead itself uses session defaults, not a plan binding.

Every safe `agents/*.toml` source is installer-managed. The seven source
profiles and their model/sandbox contracts install together with the Skills.
Retired writer, duplicate quality/test, specialist-risk and duplicate-integrator
profiles are removed only when previously installer-owned. Unrelated destination
agents and system Skills remain untouched.

The tracked configuration retains `agents.max_threads = 6` and
`agents.max_depth = 2`. Installation still changes only max_threads according
to the existing tier/override. These limits govern native session trees, not a
shared Feature-to-independent-Task tree or unlimited aggregate service capacity.
Runtime-rejected leaves stay pending in order and retry after progress.
Use bounded event-responsive waits within current tool/responsiveness limits;
do not busy-poll unchanged state or introduce leases or another scheduler.

The local loop owner constructs an exact current-head Verification Matrix:
stable row IDs, obligations, commands/checks, directory/environment, expected
mechanical observations, mismatch statuses, and source-state pre/final checks.
The runner executes supplied rows in order, fail-fast, captures attributable raw
output and always attempts the final mutation check. It cannot choose commands,
assess adequacy, diagnose, suggest fixes or decide Acceptance. Ignored build/test
artifacts are the only allowed writes. Every new head requires fresh verification.

Normal planned review uses independent spec and implementation-quality reviewers
in parallel after PASS. Risk review is policy-selected and parameterized.
Finding integration runs for overlap/conflict, authority defects, scope-sensitive
remedies or non-trivial attribution, not merely because a risk reviewer ran.
All-clean reports and a single clear bounded finding need no extra integrator;
the latter still requires evidence-based triage.

A correction retains the Task Lead session and creates H2 with fresh verification.
Finding-owning and affected reviewers rerun; prior clean coverage carries only
with exact prior reports/heads and concrete non-invalidation reasons. Uncertain
impact reruns. Carried evidence is never represented as a new-head inspection.

Task results carry exact Git/authority and independent gate evidence or direct
references. Herdr readiness/lifecycle, terminal output, agent identity and
conversation are not Acceptance. A wait timeout does not authorize another
writer. The Feature Lead validates results before releasing dependencies and
alone issues Feature Accepted.

The Feature Lead remains the only writer of the ignored planned search cache.
Task sessions and reviewers use current entries only as navigation and return
attributable candidates. Cache, authority, Git and review evidence retain their
existing separate lifecycles. TDD and focused writer checks likewise remain
distinct from fresh independent current-head acceptance.

Install the coherent asset revision only with separate owner authorization.
Previously approved/in-flight plans are not silently migrated: retain their exact
prior assets/topology/models, or stop for recovery or explicit migration.
Do not use a half-updated bundle or change an active Task's coordinator/model.
Installing does not update an already-running Codex session; new-session smoke
testing and live Herdr startup remain separate operator actions.

### Roots and managed destinations

Command-line options override these environment-derived defaults. The paths shown in the rest of this README use these default expressions; when an override option is supplied, replace the corresponding base path with that option's value.

| Purpose | Default path | Override |
|---|---|---|
| Codex files | `${CODEX_HOME:-$HOME/.codex}` | `--codex-home PATH` |
| Personal skills | `$HOME/.agents/skills` | `--skills-home PATH` |
| Installer state | `${XDG_STATE_HOME:-$HOME/.local/state}/dotfiles-codex-installer` | `--state-dir PATH` |

| Repository source | Personal destination |
|---|---|
| `AGENTS.global.md` | `${CODEX_HOME:-$HOME/.codex}/AGENTS.md` |
| `agents/<name>.toml` | `${CODEX_HOME:-$HOME/.codex}/agents/<name>.toml` |
| `skills/<name>/` | `$HOME/.agents/skills/<name>/` |
| `config.toml` | Six managed values merged into `${CODEX_HOME:-$HOME/.codex}/config.toml` |

The six managed configuration values are `model`, `model_reasoning_effort`, `plan_mode_reasoning_effort`, `tools.update_plan.enabled`, `agents.max_threads`, and `agents.max_depth`. The tracked value keeps the `update_plan` tool enabled. Other configuration bytes—including comments, statusline, context-window and auto-compact settings, MCP configuration, permissions, authentication, and providers—are preserved; the one exception is that the document ending is normalized to a single LF (`\n`).

The installer manages only declared or manifest-owned names. Unrelated sibling skills and agents are preserved. `.system` cannot be installer-owned or pruned; in particular, `${CODEX_HOME:-$HOME/.codex}/skills/.system` is outside the destination mapping.

### Rough behavior

For a default or explicit `install`, the shell launcher validates the helper source and destination before starting Rust. It reports the prospective helper action, runs the Rust sequence below, and creates a missing link only after Rust succeeds. A Rust failure leaves the link absent or unchanged. If link creation fails after Rust succeeds, the launcher fails without rolling back the valid Rust synchronization. Dry-run previews the same helper decision without mutation; restore and help do not inspect the helper destination.

Within that launcher boundary, a Rust install follows this sequence:

1. Resolve the source and destination roots, validate the source inventory, and merge the six managed configuration values with the live `config.toml`.
2. Compare the desired content with the live destinations and ownership manifest to build a plan.
3. For dry-run, print the plan and stop without changing installer-managed destinations or state.
4. For a mutating install, acquire the operation lock and recover or finalize any transaction left by an interrupted earlier run.
5. If the plan has live changes, capture the pre-install managed state as the restore backup, stage the desired content, and write the transaction WAL.
6. Apply creates, replacements, and removals. The ownership manifest is applied last.
7. Commit the transaction, select its backup as `latest`, remove older unselected backups, and clean the WAL and per-operation work tree.

A mutating no-op install still creates and acquires the persistent lock file, but it does not create a new backup or transaction. Restore loads the selected backup, acquires the lock recorded for its Codex home, performs the same startup recovery, checks the complete restore plan for conflicts, and then restores the captured content transactionally. A successful restore keeps that same backup selected.

### Files created in destination roots

The installer may create missing parent directories below the configured roots. The Codex home itself must already exist.

| Path | Contents and lifetime |
|---|---|
| `${CODEX_HOME:-$HOME/.codex}/codex-manifest-installer.lock` | Persistent empty file used only to serialize mutating commands; dry-run does not create it |
| `${CODEX_HOME:-$HOME/.codex}/config.toml` | Live Codex configuration with only the six declared values managed by this installer |
| `${CODEX_HOME:-$HOME/.codex}/AGENTS.md` | Global guidance copied from `AGENTS.global.md` |
| `${CODEX_HOME:-$HOME/.codex}/agents/<name>.toml` | Managed custom-agent definitions |
| `$HOME/.agents/skills/<name>/` | Managed personal skill directories |
| `${XDG_STATE_HOME:-$HOME/.local/state}/dotfiles-codex-installer/manifest-v1.json` | Persistent ownership manifest used to decide what later installs may replace or remove |

`AGENTS.md`, agent files, and skill directories are present only when declared by the repository inventory. A managed destination that is removed from the inventory may be removed by a later install; unrelated destinations are left alone.

The separate `$HOME/.local/bin/codex-upgrade` symlink is created by the shell launcher and deliberately does not appear in this Rust-managed destination table.

### State, backups, and temporary files

Mutating `install` and `restore` commands serialize through `${CODEX_HOME:-$HOME/.codex}/codex-manifest-installer.lock`. This is a persistent empty lock file. Dry-run does not create or acquire it.

Backups and transaction data are both stored below `${XDG_STATE_HOME:-$HOME/.local/state}/dotfiles-codex-installer`. The following tree shows the complete layout; temporary entries exist only while they are being written or when recovery from an interrupted operation is pending.

```text
${XDG_STATE_HOME:-$HOME/.local/state}/dotfiles-codex-installer/
├── manifest-v1.json
├── backups/
│   ├── latest
│   ├── latest.tmp
│   ├── .publication.tmp/
│   └── <backup-id>/
│       ├── journal-v1.json
│       └── payload/
│           ├── codex-home/
│           ├── skills-home/
│           └── state-dir/
└── transaction/
    ├── wal-v1.json
    ├── wal-v1.json.tmp
    └── work/
        └── <operation-id>/
            ├── stage/
            │   └── <index>
            └── tombstone/
                └── <index>
```

The `payload/codex-home/`, `payload/skills-home/`, and `payload/state-dir/` names are literal internal backup directories, not placeholders for environment variables. They preserve which configured root each captured relative path came from.

The state directory contains:

| Path | Meaning and lifetime |
|---|---|
| `manifest-v1.json` | Persistent list of names currently owned by the installer |
| `transaction/wal-v1.json` | Canonical write-ahead log while a mutation is in progress; retained after interruption and removed after successful commit or rollback |
| `transaction/wal-v1.json.tmp` | Temporary file used to atomically replace the canonical WAL; normally renamed immediately, and a stale ordinary file is discarded during the next transaction open |
| `transaction/work/<operation-id>/stage/<index>` | Desired file or directory content prepared before it is moved to a live destination |
| `transaction/work/<operation-id>/tombstone/<index>` | Previous live content isolated so an unfinished transaction can be rolled back |
| `backups/latest` | Persistent text file containing the selected restore backup ID |
| `backups/latest.tmp` | Temporary file used to atomically replace `latest`; normally renamed immediately |
| `backups/.publication.tmp/` | Temporary backup directory populated before it is atomically published under its final backup ID |
| `backups/<backup-id>/journal-v1.json` | Immutable backup metadata, recorded roots, ownership, and fingerprints |
| `backups/<backup-id>/payload/` | Captured pre-install content, grouped below `codex-home/`, `skills-home/`, and `state-dir/` |

At the start of a mutating command, the installer automatically recovers or finalizes an unfinished transaction before planning new work. There is no manual `recover` command.

An install with live mutations captures its pre-install managed state, commits the live changes, selects that backup through `backups/latest`, and removes older unselected backups during successful cleanup. Restore accepts only the selected latest backup; it does not accept an arbitrary backup path. After a successful restore, that same backup remains selected, no replacement backup is promoted, and successful cleanup retains only the selected backup directory. A completed transaction leaves no canonical WAL and no operation work tree.

The `transaction/`, `transaction/work/`, and `backups/` parent directories may remain after their temporary children have been cleaned. If the process stops partway through a mutation, the canonical WAL and its referenced work tree are recovery data rather than disposable scratch files. The next mutating command validates and uses them automatically; do not manually edit or delete them during normal operation.

### Repository-local build and test files

The wrapper does not install the Rust installer executable elsewhere, but Cargo keeps normal build artifacts:

| Path | Created by | Lifetime |
|---|---|---|
| `codex/installer/target/` | `install.sh` and crate-local Cargo commands, unless `CARGO_TARGET_DIR` overrides it | Reusable Cargo build cache; ignored by Git and retained until Cargo or the user cleans it |
| `codex/installer/target/release/dotfiles-codex-installer` | The release build run by `install.sh` | Cargo-managed executable used by `cargo run`; not copied into the Codex or skills roots |
| The operating system's temporary directory | Unit tests | Each unique test directory is removed when its `TempDir` is dropped |
| `codex/installer/target/tmp/process/<test-name>.../` | Process and end-to-end tests with the default Cargo target directory | Each unique test directory is removed when its `TempDir` is dropped; parent directories may remain |

Both target directories are repository-local and ignored by Git. They are build and development artifacts, not installer state and not part of backup or restore.

Rust installer tests create their own source fixture below the test temporary directory. They do not read or assert the contents of the tracked `config.toml`, `skills/`, or `agents/` assets, so instruction-only asset changes are validated independently from installer behavior.

## Manual installer fallback

Use this procedure only when the Rust installer cannot be built or run. It does not merge `config.toml`. Run the complete block from the dotfiles repository root.

Prerequisites:

- `~/.codex` already exists because Codex is installed.
- `install` and `rsync` are available.
- Any local edits inside same-name managed skills or agents have been reviewed or backed up.

```sh
set -euo pipefail

codex_source="$PWD/codex"
codex_home="${CODEX_HOME:-$HOME/.codex}"

command -v install >/dev/null
command -v rsync >/dev/null
test -d "$codex_home"
test -f "$codex_source/AGENTS.global.md"
test -d "$codex_source/agents"
test -d "$codex_source/skills"

mkdir -p "$codex_home/agents" "$HOME/.agents/skills"

install -m 0644 \
  "$codex_source/AGENTS.global.md" \
  "$codex_home/AGENTS.md"

for source in "$codex_source"/agents/*.toml; do
  install -m 0644 \
    "$source" \
    "$codex_home/agents/$(basename "$source")"
done

for source in "$codex_source"/skills/*; do
  destination="$HOME/.agents/skills/$(basename "$source")"
  mkdir -p "$destination"
  rsync -a --delete "$source/" "$destination/"
done
```

The `rsync --delete` boundary is each individual same-name skill directory. It removes files that disappeared from a managed skill, but it does not delete unrelated sibling skills. Agent files are installed one by one, so unrelated agent profiles are also preserved.

This procedure never targets `~/.codex/skills/.system`, the customized statusline, or `~/.codex/config.toml`.

### Verify the applied assets

Run from the repository root:

```sh
set -euo pipefail

codex_source="$PWD/codex"
codex_home="${CODEX_HOME:-$HOME/.codex}"

cmp "$codex_source/AGENTS.global.md" "$codex_home/AGENTS.md"

for source in "$codex_source"/agents/*.toml; do
  cmp "$source" "$codex_home/agents/$(basename "$source")"
done

for source in "$codex_source"/skills/*; do
  diff -qr \
    "$source" \
    "$HOME/.agents/skills/$(basename "$source")"
done

test -d "$codex_home/skills/.system"
```

No output from `cmp` and `diff` means the managed installed assets match the repository sources.

### Configuration limitation

The fallback intentionally does not copy `config.toml`, because replacing the live file would destroy unmanaged device-specific settings. When using the fallback, review the live `~/.codex/config.toml` and update only the six managed keys declared in this repository fragment. Never copy the fragment over the entire live file.
