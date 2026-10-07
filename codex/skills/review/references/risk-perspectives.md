# API contracts and behavioral responsibilities

Use this guidance for the selected `risk-reviewer` perspective. Run one
perspective per invocation, with its exact trigger, authority, surface,
threat/failure model, expected evidence and stop condition. Select only applicable
required/triggered coverage under the owning Review policy; do not add reviews
or invent guarantees independently.

## Communication API contracts — 通信 API 契約

Cover HTTP, gRPC, GraphQL, events, TCP and IPC when their exposed contracts
change. Inspect realistic consumers against the applicable protocol and schema:
message framing/shapes, version compatibility, error meanings, delivery and
retry semantics, duplication, ordering, timeouts, disconnects and stream
termination. Apply only obligations relevant to the actual communication model;
do not impose exactly-once delivery, ordering or other unsupported guarantees.

Trace a concrete interaction and show how a reachable consumer interprets it.
Expected defect evidence is a contract mismatch or realistic misuse with material
consequences, such as a changed wire field causing an existing consumer to
misinterpret a message. Internal versus external visibility is not the split:
IPC within one application still crosses a communication boundary.

## Function/type API contracts — 関数・型の API 契約

Cover module and library functions, types, traits, interfaces and callbacks when
their caller contracts change, including non-public module boundaries. Inspect
arguments/results, ownership and lifetime, side effects, errors and panics,
required call ordering and compatibility with actual consumers. A library
exported to other teams remains a function/type API.

Construct a realistic call sequence and check whether consumers can correctly
use the interface and handle its outcomes. Expected defect evidence is a concrete
caller-contract mismatch or reachable misuse with material consequences.
Language hints guide this investigation; they do not impose naming preferences
or hypothetical consumers as requirements.

## Behavioral responsibilities/guarantees — 振る舞いの責任と保証

Select this perspective when operation meaning, invariants, state transitions
or responsibility boundaries change. It applies to business applications and
infrastructure such as runtimes, parsers and storage components.

From the supplied specification or approved design, identify what an operation
promises and which boundary is assigned to guarantee it. Trace a concrete
scenario across components and alternate entry points. Inspect whether:

- required decisions or protocols have leaked to callers;
- multiple callers duplicate or disagree on the required decisions;
- an entry point can bypass an invariant or required transition;
- the composed path actually establishes the promised result.

For example, if the approved order component owns cancellation eligibility,
check whether callers must each enforce that rule before a generic status
update. If an async runtime owns making an eligible awakened task runnable,
trace wake handling, task state and queue membership to locate that guarantee;
check whether callers must assemble undocumented internal scheduling steps.
These examples are conditional, not universal design requirements.

Thin delegation and ports are valid when the assigned collaborator reliably
supplies the guarantee. Names or implementation thickness alone are not evidence.
Expected defect evidence identifies the approved guarantee/responsibility, actual
execution path, reachable leak/bypass/inconsistency and material consequence.
Do not invent a domain layer, new state machine or ideal responsibility placement.

## Boundaries with other perspectives

| Perspective | Primary question |
| --- | --- |
| Specification compliance | Does the implementation meet the required behavior and results? |
| API contracts | Can the consumer correctly use and interpret the interface? |
| Architecture/dependencies | Does responsibility placement and dependency structure follow the approved design? |
| Behavioral responsibilities/guarantees | Along a concrete execution path, who actually guarantees the promised meaning, invariants and transitions? |
| Robustness/recovery | Do guarantees survive reachable failure, concurrency, partial state, recovery and termination? |

These perspectives can observe the same defect. Keep selected coverage separate
and consolidate overlapping findings under the existing review integration
rules. All findings retain the common authority, evidence and proportionate
remedy criteria. The selected perspective may also report a grounded quality
improvement with justified benefit and cost under the review boundaries; do not
invent a guarantee or responsibility placement to support it. A missing supplied
guarantee or responsibility assignment is a disclosed gap, not permission to
treat the reviewer's preferred design as authority. Inspect independently
reviewable obligations and apply the [review boundaries](review-boundaries.md)
for partial coverage and BLOCKED. The owner recovers required authority before
accepting affected guarantee/responsibility coverage.
