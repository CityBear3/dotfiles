# Engineering artifact storage and ownership

Apply this policy when selecting, creating, revising or retiring workflow
artifacts. Explicit repository placement rules and approved existing artifact
identities take precedence over these defaults. Keep existing authoritative
documents and in-flight paths in place; a new convention does not authorize
moving, copying or deleting them.

## Keep one living Design Doc

For a new Design Doc, use `design/YYYY-MM-DD-<feature>.md` from its first Draft
through approval and later revisions. Edit that same canonical document in the
design loop. Do not maintain a living Design Doc under `.work/` and copy it into
`design/` at approval or implementation completion.

Keep the document's Draft/approval state and exact approved revision explicit
in the document or existing handoff evidence. When revising an approved design,
distinguish the proposed changes from the exact prior approved authority.
Neither its path, existence nor a status label proves approval. Placement in
`design/` makes it eligible for version control; it does not authorize staging,
committing, publication or implementation.

Existing checkpoints and handoffs reference the canonical Design Doc's path,
revision and state rather than duplicate its contents. When the preservation
check needs a pre-finalization Draft snapshot, keep an immutable copy under the
feature's `.work/` directory, labelled with its source path and exact revision
or content fingerprint. It is comparison evidence, never a second editable
design or approval source. Retire it only under the existing preservation rules.

## Keep workspace-only execution artifacts under .work

Use `.work/YYYY-MM-DD-<feature>/` for the resume checkpoint, Feature Contract,
Implementation Plan, optional search cache and required temporary snapshots.
Ensure `/.work/` is ignored before writing workspace-only artifacts, using the
repository's exclusion policy. Do not force-add, stage or commit these files
unless the user explicitly chooses archival. Do not create empty placeholders,
extra indexes or optional artifacts merely to populate the directory.

Keep the existing approval, authority, sole-writer and retention rules for each
artifact. Ignoring a file does not make it disposable: contracts and plans stay
available through their existing worktree lifecycle. Requirements originals,
agreed supplements and relied-on evidence that must survive cleanup remain at
durable, referenced sources; `.work/` must not be their only surviving location.
Retain the project's established requirements location instead of relocating
originals to fit this layout.

## Bound document generation by the approved work

Keep architectural explanation and decision rationale in the canonical Design
Doc, with references to its inputs. Creating or completing a design, Task,
review or branch does not authorize an additional architecture overview,
design-and-usage guide, README, runbook or other public documentation.

Create or expand user-facing documentation only when the request, approved
scope or explicit repository rules require it. Update the appropriate existing
document where possible. This does not prohibit required code documentation,
authorized requirements sources or the workflow artifacts defined above.
