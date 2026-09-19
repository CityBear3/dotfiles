---
name: go-implementation
description: Apply official Go guidance and tools when editing Go code, packages, or module settings.
---

# Go implementation guidance

Apply these defaults to Go implementation work. Keep reusable conventions here
rather than copying them into plans or contracts.

## Resolve project authority and Go version

- Follow the closest repository `AGENTS.md`, approved design and contracts,
  established public API, and repository verification commands before these
  defaults.
- Check `go.mod`, applicable `go.work`, toolchain settings, and CI to establish
  supported Go versions and build configurations. Use APIs and syntax compatible
  with those requirements; do not implicitly upgrade Go or dependencies.
- Keep formatting and modernization within the approved change. Preserve
  unrelated code and existing compatibility and failure contracts.

## Use official guidance

- Use [Effective Go](https://go.dev/doc/effective_go) for core idioms. It is not
  actively updated and does not cover later additions such as modules and
  generics.
- Supplement it with [Go Code Review Comments](https://go.dev/wiki/CodeReviewComments),
  including guidance on interfaces, contexts, errors, and goroutine lifetimes.
- For a particular API or language feature, consult `go doc` with the project's
  toolchain and the relevant [release notes](https://go.dev/doc/devel/release)
  or version-appropriate official documentation. Check the supported version
  before adopting examples from newer documentation.

## Select tools by purpose

Use the repository's verification commands when available. Otherwise select
the affected packages and relevant dependents, running commands from the
appropriate module or workspace directory with the applicable build settings.
Use `./...` when it represents the intended scope; account for separate modules.

- **Format:** use `gofmt -w` on edited files, or `go fmt` on scoped packages.
  Let the formatter decide layout and inspect the resulting diff.
- **Static checks:** run `go vet` on the relevant packages. Investigate its
  diagnostics; passing vet does not establish behavioral correctness.
- **Behavior:** run `go test` for the changed behavior and relevant regressions.
  Its default vet checks are only a subset of `go vet`.
- **Concurrency:** when a change affects concurrent behavior, add
  `go test -race` on the relevant packages where supported. It detects races
  only on executed paths, so exercise the affected behavior.

See the [Go command documentation](https://pkg.go.dev/cmd/go) and
[race detector guide](https://go.dev/doc/articles/race_detector) for details.

## Apply modernization deliberately

Go 1.26 revamped `go fix` to modernize language and standard-library usage.
Check the project's toolchain and `go help fix` before selecting flags.

- On toolchains supporting it, preview scoped packages with `go fix -diff`.
- Apply relevant fixes with `go fix` only within the approved scope; select
  specific fixers when needed. A routine implementation task does not authorize
  repository-wide modernization.
- Inspect the changes for API, compatibility, and behavior effects, then rerun
  affected verification. Keep broad modernization separately reviewable when
  it is part of the requested work.

See [Using go fix to modernize Go code](https://go.dev/blog/gofix) for preview,
fixer selection, and Go-version constraints.
