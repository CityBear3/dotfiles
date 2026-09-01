# [Design Doc] Codex Installer Result Presentation

- Author: Repository owner
- Date: 2026-09-01
- Last updated: 2026-09-01
- Status: Approved

## Context and scope

`codex/installer` currently combines application execution with human-readable
rendering. Its public `run_from` function and application functions return a
pre-rendered `String`; successful install and restore operations emit only
`install complete` or `restore complete`; and the plan module renders dry-run
lines directly. This makes the result difficult to scan and makes a future
presentation such as a TUI depend on output strings rather than operation
semantics.

This design introduces a presentation-neutral result between application
execution and external presentation. The CLI renders that result as a compact,
static table with status icons, restrained color, and an operation summary. A
future TUI may consume the same semantic result, but no TUI is implemented by
this feature.

The approved [Rust Codex Installer Addendum](./2026-07-19-rust-codex-installer-addendum.md)
remains authoritative for planning, transaction, failure, rollback, recovery,
commit, finalization, and cleanup semantics. This document refines only result
representation and presentation. If a presentation choice could imply a clean
result before that addendum permits one, the addendum takes precedence.

### Goals

- Make install, restore, and dry-run results easy for a person to scan.
- Use one compact `STATUS / ACTION / ASSET / PATH` table structure for planned
  and completed actions.
- Separate application results from terminal rendering so the CLI and a future
  TUI are replaceable presentation adapters.
- Use icons and color without making color the only carrier of meaning.
- Keep redirected, piped, captured, and CI output human-readable and free of
  ANSI escapes.
- Preserve truthful failure and recovery reporting from the approved installer
  design.
- Keep output deterministic under explicit rendering capabilities so it can be
  verified with focused tests.

### Non-goals

- A live progress display, spinner, animation, artificial delay, or interactive
  TUI.
- A serialized, cross-process, cross-crate, or third-party-stable result
  protocol.
- Stable machine parsing of human-readable stdout, including a JSON mode.
- Persistent diagnostic log creation or rotation.
- Changes to plan classification, transaction ordering, backup selection,
  rollback, cleanup, ownership, configuration merging, or filesystem safety.
- Integration or removal of the shell-owned `codex-upgrade` helper action. Its
  future removal is separate work.
- New color or glyph-selection command-line options.
- Deliberate path truncation, renderer-controlled wrapping, or a general
  terminal-layout framework.

## Overview

Application execution returns a typed semantic report rather than terminal
text. An outer CLI presentation adapter chooses the destination stream, detects
that stream's capabilities, invokes a deterministic renderer, and writes the
rendered result. Terminal concepts point inward no further than this adapter.

```text
command parsing -> application execution -> Result<semantic report, InstallerError>
                                               |                    |
                                               +---- CLI adapter ----+
                                               |     stdout/stderr
                                               |
                                               +---- future TUI
```

The semantic report is an in-process contract within the installer package. It
describes the operation mode, actions, resolved paths, selected dry-run thread
count when applicable, and completion counts without containing table headers,
column padding, ANSI styling, home-relative display strings, or terminal types.
Private type names and field layout remain implementation decisions.

The CLI is the only presentation implemented here. It emits complete, static
output after execution resolves; it never redraws prior lines or delays work so
progress can be observed.

## Detailed design

### Responsibility and dependency boundaries

Command parsing and application execution remain responsible for validating the
request, producing a plan, running the transaction when required, and returning
typed success or failure semantics. They do not inspect whether stdout or stderr
is a terminal, read presentation-related environment variables, select colors,
format tables, or call a presenter.

Successful execution returns a presentation-neutral operation report:

- a dry-run report represents the complete plan, including `NO-OP` actions and
  the selected `max_threads` value;
- a successful mutating report represents the completed install or restore plan,
  including enough information to render changed rows and count unchanged rows;
- content snapshots, desired file contents, authentication data, and other
  configuration values are not presentation data and are not exposed for
  rendering.

A mutating success report may be returned only after the transaction engine and
finalization path return success under the approved installer addendum. A no-op
report may be returned after planning proves that no mutation is required. Any
failure returns the typed `InstallerError`; the application does not construct a
partially successful table.

The CLI adapter consumes the typed result. It preserves the existing destination
and exit-status policy: successful and dry-run results go to stdout; installer
failures go to stderr; and Clap display output such as help retains its existing
stdout and zero-exit behavior. The renderer receives explicit capabilities and
path-display context. It does not inspect process globals itself.

The current `run_from -> Result<String, InstallerError>` shape is replaced by the
typed result path rather than retained beside it. The crate is not published,
and repository inspection found no consumer beyond its binary and crate-internal
tests, so there is no compatibility reason to maintain two rendering paths.

### Common table model

Dry-run and changed success output use these columns:

| Column | Meaning |
|---|---|
| `STATUS` | Planned, successful, or unchanged state represented by an icon |
| `ACTION` | Existing plan operation: `CREATE`, `REPLACE`, `REMOVE`, or `NO-OP` |
| `ASSET` | Display-only composition of category and optional name |
| `PATH` | Resolved destination path in ordinary display form |

`ASSET` uses the existing category label alone when there is no name, such as
`config`, and `category/name` when a name exists, such as `skill/review`. It is
not a new domain identifier. Separate `TYPE` and `NAME` columns are not added.

The table has no outer border and no vertical rules. A single horizontal
separator follows the header. Column widths are derived deterministically from
the rendered header and rows. Paths are not shortened with an ellipsis; an
ordinary terminal may wrap a long physical line.

For successful and dry-run output, the user's home directory is rendered as `~`
and its descendants as `~/...`. A path outside the home directory remains
absolute. Error and recovery diagnostics retain absolute paths so recovery
instructions remain unambiguous.

### Status and color semantics

| Status | Meaning | Color when enabled |
|---|---|---|
| `•` | A dry-run mutation is planned | Blue or cyan |
| `✓` | A mutating action completed successfully | Green |
| `–` | The action requires no change | Light gray |
| `✗` | An operation failed or rolled back | Red |
| `!` | Live state committed, but finalization or cleanup remains incomplete | Yellow |

Action text does not receive action-specific colors. In particular, a successful
`REMOVE` is not red. Counts such as `5 unchanged` use the standard text color.
The completion icon `🍺` is reserved for completely successful changed install
or restore execution and appears in the green completion summary.

Every meaning remains available through the status symbol, action, and message;
color is supplementary.

### Dry-run presentation

Dry-run emits a standard-color context line immediately before the table:

```text
Dry run · max threads 6

STATUS  ACTION   ASSET         PATH
------  -------  ------------  -------------------------
•       REPLACE  config        ~/.codex/config.toml
–       NO-OP    skill/review  ~/.agents/skills/review
```

All plan actions appear, including `NO-OP`. Planned mutations use `•`; no-op
actions use `–`. Dry-run never uses `✓` or `🍺` because no action was applied.
The `max threads` context preserves the current ability to inspect the value
selected for an `auto` request without adding a non-action row to the table.

Dry-run remains non-mutating and does not acquire or create the operation lock,
as required by the approved installer addendum. Presentation occurs only after
planning and does not alter that boundary.

### Successful install and restore presentation

A successful mutating operation displays only changed actions in the table. Each
displayed row uses `✓`; `NO-OP` rows are omitted but contribute to the unchanged
count in the completion summary. The summary identifies install or restore,
reports changed and unchanged counts, includes `🍺`, and is green. For example:

```text
STATUS  ACTION   ASSET               PATH
------  -------  ------------------  ------------------------------
✓       REPLACE  config              ~/.codex/config.toml
✓       REMOVE   agent/legacy-agent  ~/.codex/agents/legacy-agent.toml

🍺 Install complete · 2 changed · 5 unchanged
```

The example fixes the information and visual hierarchy, not private column-width
helpers or a table-library choice.

When every install action is `NO-OP`, the empty table is omitted and the CLI
emits:

```text
✓  Already up to date · N unchanged
```

When every restore action is `NO-OP`, it emits:

```text
✓  Already matches latest backup · N unchanged
```

Only `✓` is green in these no-op summaries; the message and count retain the
standard text color. Neither no-op form uses `🍺` because no change was applied.

### Failure and recovery presentation

Failures do not emit per-entry result tables. Before commit, actions may have
been rolled back; after commit, cleanup may remain incomplete. Marking individual
rows failed or successful would therefore misrepresent final filesystem state.

An ordinary failure, a cleanly rolled-back transaction, a rollback failure, an
unclassifiable transaction, or unresolved WAL authority uses a red `✗` heading.
A `CommittedCleanupIncomplete` outcome uses a yellow `!` heading because live
state is committed but the operation is not cleanly complete. Existing detailed
error information follows the heading, including transaction identifiers,
recovery instructions, WAL locations, and absolute paths where applicable.

Every failure preserves its existing nonzero exit code and stderr destination.
No failure or incomplete-cleanup path emits `🍺`, a success table, or a clean
completion message. Clap help and version display are not installer failures and
remain outside these failure headings.

### Terminal capability policy

The CLI adapter evaluates capabilities for the stream it will actually write:
stdout for success and dry-run output, and stderr for failures. ANSI color is
enabled only when all of the following hold:

- the destination stream is a TTY;
- `NO_COLOR` is unset or empty; and
- `TERM` is not `dumb`.

Otherwise, the same table, symbols, text, and completion icon are emitted without
ANSI escape sequences. Unicode symbols `•`, `✓`, `–`, `✗`, `!`, and `🍺` remain
enabled for redirected, piped, CI, and `NO_COLOR` output because this command's
stdout is human-readable UTF-8 rather than a machine protocol.

The adapter passes an explicit capability value to a deterministic renderer.
Tests can therefore select styled or plain rendering without changing process
environment or requiring a pseudo-terminal. This feature adds neither
`--color=always|never` nor an ASCII fallback option.

### Future presentation adapters

A future TUI may consume the same typed operation report and typed errors, then
apply its own capability and layout policy. It does not consume the CLI table or
ANSI output, and the application does not call into it. This design does not
promise that the report is serialized, versioned independently, or stable for
third-party consumers; it is an internal seam that preserves dependency
direction within this package.

The shell launcher remains outside this seam. Its temporary `codex-upgrade`
helper output is neither inserted into the Rust operation report nor reproduced
in the CLI table.

## Cross-cutting concerns

### Recovery correctness

Presentation cannot weaken transaction truth. A green table or `🍺` summary is
reachable only after complete success. Red and yellow headings preserve the
distinction between failed or rolled-back work and committed live state with
incomplete cleanup. The approved installer addendum remains the source of truth
for those classifications and their recovery details.

### Determinism and verification

Production behavior changes use causal test-driven development. Verification
must cover:

- exact plain rendering for dry-run, changed install, changed restore, install
  no-op, and restore no-op reports;
- deterministic row order, column structure, asset labels, home-relative paths,
  and untruncated paths;
- ANSI placement for every status class and proof that action text and unchanged
  counts are not accidentally colored;
- the capability matrix for stdout and stderr TTY state, nonempty `NO_COLOR`,
  `TERM=dumb`, redirected output, and color-enabled output;
- process-level stdout, stderr, and exit-code behavior;
- absence of success tables and `🍺` for rolled-back, rollback-incomplete,
  unclassifiable, unresolved-authority, and committed-cleanup-incomplete errors;
- existing dry-run non-mutation, install and restore state, rollback, cleanup,
  and recovery tests.

Renderer tests receive explicit capabilities and home-path context. They do not
depend on the developer's terminal or environment.

### Security and privacy

Rows contain only operation, asset label, and destination path. Desired content,
captured configuration, authentication values, and provider data are never
rendered. Ordinary result paths may abbreviate the home prefix, while errors keep
absolute locations needed for diagnosis.

### Performance and compatibility

Rendering is a single bounded pass over the completed report. It introduces no
background task, refresh loop, terminal polling, or delay. Table-library and
styling-library selection are implementation-plan concerns and must not pull
terminal dependencies into planning or transaction modules.

The human-readable table is not a stable parsing grammar. Changing `run_from`
from a string result to a typed report intentionally changes an internal package
boundary; no repository consumer requires compatibility with the current string
return. Filesystem state, journal, backup, manifest, locking, and command-line
input contracts are unchanged.

## Alternatives considered

- **Live progress or a transient TUI:** rejected because normal work is short;
  the display would flicker or encourage artificial delay.
- **Inject a presenter into application execution:** rejected because it reverses
  the dependency direction and makes execution aware of external presentation.
- **Keep both string and structured result APIs:** rejected because there is no
  compatibility consumer and two paths add unnecessary behavior and test surface.
- **Keep dry-run rendering in the plan module:** rejected because it couples
  domain planning to one human presentation.
- **Full table borders:** rejected because they add width and noise around long
  paths. Omitting the header separator was also rejected because headings and
  rows become harder to distinguish.
- **Separate `TYPE` and `NAME` columns:** rejected because unnamed assets make
  them ambiguous and sparse; `ASSET` expresses the display concept directly.
- **Use `RESULT` or omit the first dry-run column:** rejected because planned work
  has no applied result and one shared `STATUS` structure is clearer.
- **Action-specific colors:** rejected because a red successful `REMOVE` resembles
  failure. Color communicates status only.
- **Absolute home paths or deliberate ellipsis in ordinary tables:** rejected as
  repetitive noise or information loss. Errors retain absolute paths.
- **Show an empty table for all-no-op runs:** rejected in favor of concise,
  mode-specific summaries.
- **Show per-entry failure rows:** rejected because rollback and post-commit
  cleanup make independent row outcomes misleading.
- **Remove Unicode when color is disabled:** rejected because symbols remain
  meaningful in human-readable UTF-8 output. Explicit color or glyph flags were
  rejected as unjustified option-surface expansion.
- **Move `max threads` after the dry-run table or remove it:** rejected because it
  is useful plan context, especially for `auto`, and belongs before the actions.
- **Integrate the shell-owned helper action:** rejected because that helper is
  planned for separate removal and is not part of the Rust result boundary.
- **Revise the Rust installer addendum instead of writing this document:**
  rejected because presentation has a distinct purpose and approval scope; the
  addendum should remain focused on transaction and recovery guarantees.
