# Source-review depth

Coverage mode chooses perspectives. Depth governs investigation within each
source reviewer's assigned perspective and exact target. Use `deep` by default
for new policies, including eligible lightweight and new standalone reviews.
Honor explicitly narrowed requests within applicable gates. Existing approved
policies retain their original meaning; a missing depth field is not permission
to upgrade them. Depth is separate from model reasoning effort and never changes
allocations, reviewer count or implementation authority.

## Investigate deeply within the assigned perspective

Derive relevant obligations, invariants and responsibility boundaries from
applicable authority. Distinguish approved guarantees from assumptions made by
the implementation, callers or tests. Preserve settled trade-offs and non-goals.

Trace material execution or interpretation paths through actual callers,
collaborators and tests. Follow relevant boundary inputs, error outcomes,
partial state, ordering or concurrency only when the assigned perspective and
realistic risk justify them. Reading surrounding dependencies to understand a
path does not claim coverage of another perspective or an unrelated Task.

Form concrete risk hypotheses and try counterexamples against those paths.
Check disconfirming evidence before retaining a claim: validation in another
layer, actual caller constraints, permitted call ordering, existing recovery,
or an approved trade-off may make the suspected problem inapplicable. Do not
invent consumers, guarantees or architecture to make a hypothesis succeed.

For assigned implementation-quality coverage, use the decision criteria in the
[shared implementation quality guidance](../../execute-task/references/implementation-quality.md)
to assess actual responsibility placement, dependencies, state/error handling,
readability and testability. Use hints and version-appropriate official
documentation where they resolve a material question; evaluate the resulting
code and concrete trade-offs, not whether the writer recorded enough research.

For assigned test-quality coverage, apply the responsibility, defect-detection
and replaceable-internals questions in the
[test contract](../../test-driven-development/SKILL.md#test-contract). Inspect
what assertions, fixtures and mocks actually prove. Consider a realistic faulty
implementation that could escape them, then establish a material defect,
required-evidence gap or worthwhile
improvement to proof of an applicable behavior. A test-passing hypothetical
defect alone is not a finding.

Use directly readable sources, raw observations and permitted non-mutating
checks as evidence. A concrete source path can establish a defect without a new
executable reproduction; do not demand one for every hypothesis. Do not modify
the reviewed source, create reproduction artifacts there, dispatch another
reviewer or redefine the Verification Matrix. Report a material evidence gap
against its actual obligation for the owner to recover. A newly discovered
material risk outside policy follows the existing owner policy-resolution rule.

## Findings and completion

Apply the common [review boundaries](review-boundaries.md). Deep investigation
does not lower the evidence threshold, raise severity, make every possible
improvement mandatory, or authorize a more elaborate remedy. Qualifying Should
Improve findings still require concrete benefit that justifies change,
complexity and verification cost.

Finish when the assigned material paths, assumptions and risks have been
meaningfully inspected and the report supports findings or a bounded clean
result, with essential gaps identified. Do not require exhaustive proof, a
finding quota, repeated passes until a defect appears, or a transcript of every
discarded hypothesis. Report effective depth, material paths or hypotheses
examined, decisive evidence and limitations concisely.

Correction review keeps this depth within corrected findings and affected
coverage. Existing impact and carry-forward rules decide targeted versus full
traversal; deep alone never restarts discovery over unaffected code or reopens
resolved Push back without materially new evidence. Finish the loop when
required coverage is complete and qualified findings are resolved.

This source-investigation instruction does not turn finding integration or
triage into another source review. Integration reconciles supplied findings
only when its existing triggers apply; triage independently decides problem and
remedy validity, proportionality and authority. Their criteria and final action
ownership remain unchanged.
