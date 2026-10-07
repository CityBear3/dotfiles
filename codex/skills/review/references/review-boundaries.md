# Improvement and incomplete-coverage boundaries

Review for better implementation within the contract, project rules and approved
scope. Coverage mode (`adaptive`, `deep`, or eligible lightweight `focused`)
selects perspectives; there is no separate improvement or reporting mode.
Preserve already-approved and in-flight policies rather than silently migrating
their coverage, allocations or Acceptance. Honor an explicitly narrowed review
request without inventing another configurable axis or weakening required gates.

## Evaluate worthwhile improvements

Must Fix retains the existing material-defect and required-evidence threshold.
Should Improve may also identify a concrete quality improvement in the inspected
implementation without requiring a current bug or contract violation. Show the
actual code or relevant scenario, expected benefit, assumptions, change and
verification cost, trade-offs, and a bounded proportionate option. Examples
include simplifying duplicated decisions, making the current control flow easier
to understand, or strengthening proof of an applicable behavior. These are
investigation directions, not quotas or universal requirements.

Keep only improvements whose concrete benefit justifies their change, complexity
and verification cost. Generic preferences, trivial polish, speculative future
consumers and mechanisms without demonstrated need do not qualify. Respect
approved non-problems and settled decisions; reassess them only with materially
new evidence. Do not invent a new requirement to justify an improvement.

A qualifying improvement is a Should Improve finding and enters the existing
integration/triage route. Triage verifies the benefit and remedy as well as
scope and authority. An owned, bounded implementation improvement within
existing authorization can be Fix without separate approval for every item;
raw review advice alone never authorizes edits. A design/public-contract change,
material scope expansion or unresolved user-owned trade-off still needs the
engineer. Standalone review remains read-only and grants no implementation
authority. Independent out-of-scope concerns remain non-blocking, not a backlog.

## Keep autonomous correction proportionate

Correct only triaged authorized findings with the smallest sufficient change
and necessary proof, including qualified improvements. Do not bundle unrelated
cleanup, extra mechanisms or tests without a concrete justified purpose.
Re-review corrected findings and affected coverage under the existing impact
rules. Investigate newly exposed regressions or evidence gaps, but do not use
correction review to restart improvement discovery over unaffected code, revisit
settled Push back without new evidence, or seek an ideal implementation. Once
required coverage is complete and qualified findings are resolved, finish the
loop rather than adding another improvement pass.

## Missing inputs and BLOCKED

Distinguish a source review from the owning workflow's gate decision. Before
declaring an input missing, inspect directly available relevant sources.

- A source reviewer returns CLEAN or FINDINGS for a meaningfully inspected,
  stable bounded scope, stating complete or partial coverage, unavailable
  surfaces, assumptions and any unfulfilled requested obligation. Missing
  unrelated tests, a base diff, or Task artifacts does not block a standalone
  fileset review merely because those inputs appear in a general role template.
- Return BLOCKED when the target cannot be identified or remains stale, the
  assigned review cannot be meaningfully performed, or an essential prerequisite
  such as required authority, access or independent allocation is unavailable.
  Identify the affected obligation and exact recovery condition. Preserve
  independently usable observations; do not invent certainty about the gap.
- A partial leaf CLEAN is never complete Task/integration Acceptance evidence.
  The owner holds the affected gate until every required obligation has current
  verification and review coverage. Recover missing inputs or dispatch only
  uncovered/invalidated checks under the existing policy. An authority gap
  follows the existing authority-defect route; it is not permission to invent
  a guarantee or choose responsibility placement.

For example, an examples-only standalone review can be CLEAN with a disclosed
absence of test bodies. A required test-quality pass with those same bodies
unavailable leaves that Task gate held. An unknown or concurrently changed
target is BLOCKED.
