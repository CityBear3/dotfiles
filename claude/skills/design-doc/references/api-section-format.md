# API Section Format

This reference defines the format for the **API specification portion** of the Detailed Design section. The structure takes from Google's AIPs (https://google.aip.dev/) the prominence of a central Guidance section, but adapts the rest to Design Doc conventions: a dedicated **Design decisions** section makes the choices and reasoning prominent, and **Examples** is its own block for content not derivable from the proto (error responses, sequences). Apply this format when the design exposes one or more RPCs and consumers need a stable reference for the contract, design decisions, and error model.

This format applies **only inside the API portion** of the Detailed Design section. Other parts (architecture, data flow, workflow) should follow the narrative prose style described in SKILL.md.

## When to Apply

- The design exposes a network API (gRPC service or equivalent RPC system)
- The API is consumed by code outside the team that owns it
- The error model is part of the contract that consumers must integrate with

Skip this format for:
- Internal-only interfaces with a single in-team consumer
- Throwaway or experimental APIs not yet committed
- Library/SDK surfaces (function-level APIs) — describe in prose

This format is currently scoped to **gRPC-based design**. REST mapping is not yet covered.

## Heading Hierarchy

```
## API                              (H2 — within the Detailed Design section)
### Overview                        (H3 — prose only, no subsections)
### Resource model                  (H3 — has the three subsections)
   #### Design decisions
   #### Guidance
   #### Examples
### <MethodName>                    (H3 — one block per method)
   #### Design decisions
   #### Guidance                    (with "Possible errors" list at end)
   #### Examples
### Errors                          (H3 — central error model)
   #### Design decisions
   #### Guidance                    (reason vocabulary table)
   #### Examples
```

H2 marks chapters of the Design Doc, H3 marks blocks within the API section, H4 marks the three-subsection triplet (Design decisions / Guidance / Examples). Overview is the only H3 block without H4 children.

A note on terminology: "Guidance" matches the AIP convention. "Design decisions" and "Examples" are not standard AIP section names — AIPs typically weave examples into Guidance and use a "Rationale" subsection only when justification needs special depth (e.g., AIP-193). Design Docs benefit from making these explicit, which is why they are separate H4 sections here.

## Section Purposes

### Resource model
Define the proto/schema and the cross-cutting design decisions that span all methods on the resource: which fields are mutable vs `OUTPUT_ONLY`, etag use, naming conventions, authentication scopes.

- **Design decisions**: why this resource shape, why these mutability rules, why etag, why this resource decomposition
- **Guidance**: rules that apply regardless of which method is called (etag handling, output-only treatment, immutable fields, auth scopes, naming)
- **Examples**: representative resource literals showing key states (e.g., normal, draining, error)

### Per-method block
Cover only the decisions specific to that method.

- **Design decisions**: why this method shape (Unary vs streaming, why field_mask, why this return type, why this validation rule)
- **Guidance**: behavioral rules unique to this method (validation, side effects, retry semantics) followed by a **Possible errors** list of reason values this method can return
- **Examples**: sequences spanning multiple calls, or behaviors not visible from the proto alone (skip single success request/response — they duplicate the proto)

### Errors
The central design and reference for the error model.

- **Design decisions**: why gRPC canonical codes, why `ErrorInfo.reason` strings (with optional proto enum for type safety), why specific code-to-reason mappings, why this `domain` value
- **Guidance**: the reference proto enum (if Option C is used), the full reason vocabulary table with `method` column showing where each reason can occur, supplementary rules (domain value, RetryInfo expectations, metadata conventions)
- **Examples**: concrete `google.rpc.Status` responses showing the `details` structure

## Section Content Rules

- **Design decisions** captures the choices made and the reasoning that isn't visible in the proto. This is the highest-value section — it preserves intent that would otherwise be lost when only the schema remains. Each bullet states *what was decided* and *why*.
- **Guidance** captures behavioral rules not expressible in schema: staleness thresholds, clamping rules, retry policies, auth boundaries, output-only treatment. For methods, end with a "Possible errors" list pointing to the central Errors section.
- **Examples** is for content not derivable from the proto: error response shapes (the `details` envelope structure isn't visible from message definitions), multi-call sequences, good/bad contrasts. Single success request/response examples duplicate the proto and should be skipped.

## Error Model Convention

Use gRPC canonical status codes. Carry structured information in `google.rpc.Status.details` using standard types (`ErrorInfo`, `RetryInfo`, `BadRequest`).

For service-specific reason identifiers, use `google.rpc.ErrorInfo.reason` (UPPER_SNAKE_CASE strings). Two valid approaches:

- **Option B**: pure strings, defined only in the reason vocabulary table
- **Option C (recommended)**: define a proto enum (`<Service>ErrorReason`) for type safety in implementation and tests, but transmit on the wire as `ErrorInfo.reason` strings. New reasons can be added without breaking compatibility.

The `domain` field is fixed per service (e.g., `connectionpool.cacheme.example.com`). Clients should fall back to the gRPC code when they encounter an unknown reason.

The reason vocabulary table includes a `method` column to show where each reason can occur, mirrored by the per-method **Possible errors** list. This bidirectional cross-reference balances locality (readers of a method see what it can return) with non-duplication (the design and full vocabulary live in one place).

## Hybrid Errors Pattern

Errors are documented centrally, but each method's Guidance ends with a short list of the reasons that method can produce:

```
#### Guidance
(behavioral rules)

**Possible errors** — see Errors for details
- INSTANCE_NOT_REGISTERED
- INSTANCE_INITIALIZING
- ETAG_MISMATCH                  ← method-specific
```

When to switch to fully per-method errors instead:
- Many methods (5+) with mostly disjoint error sets
- Methods owned by separate teams with non-aligned change cadence
- Auto-generated proto docs are the primary deliverable and the Design Doc plays a secondary role

For the typical case (a handful of methods on a single resource with significant overlap), the hybrid pattern is appropriate.

## Worked Example

The following example illustrates the format applied to a control-plane API for a cache server's connection pool.

### Overview

Control-plane API for a cache server's bidirectional-stream connection capacity, exposed for autoscalers and instance pool management. Provides `GetConnectionPool` for retrieving current capacity per instance and `UpdateConnectionPool` for changing the maximum capacity.

### Resource model

```proto
syntax = "proto3";
package cacheme.connectionpool.v1;

import "google/api/field_behavior.proto";
import "google/protobuf/timestamp.proto";

// Singleton resource per instance.
// Name format: instances/{instance_id}/connectionPool
message ConnectionPool {
  string name = 1;

  int32 max_connections = 2;

  int32 active_connections = 3 [(google.api.field_behavior) = OUTPUT_ONLY];
  int32 available_connections = 4 [(google.api.field_behavior) = OUTPUT_ONLY];
  google.protobuf.Timestamp observed_at = 5 [(google.api.field_behavior) = OUTPUT_ONLY];

  string etag = 6;
}
```

#### Design decisions
- **Singleton resource**: standard Get/Update mount on it, giving uniform treatment across generated SDKs and observability tools.
- **Per-instance**: autoscalers operate at instance granularity, so an aggregate API would be insufficient.
- **Only `max_connections` is mutable**: other fields are runtime-derived; writing to them has no meaning.
- **`available_connections` is precomputed**: avoids client-side subtraction races for the hottest consumer (autoscaler).
- **`observed_at`**: snapshots can be stale via intermediate caching; without freshness, scaling decisions can misfire.
- **`etag`**: capacity changes are operationally significant; etag prevents stale-read overwrites.
- **One resource, not split**: capacity and connection state are tightly coupled observables; splitting them adds coordination cost without value.
- **Separated from data plane (bidirectional streams)**: control plane lives as a distinct service so authentication boundaries can differ.

#### Guidance
- `OUTPUT_ONLY` fields in an Update request are silently ignored, not rejected (AIP-203).
- Servers guarantee `available_connections >= 0`. Connections in graceful drain count as active.
- Capacity counts only schedulable connections (drain counts as active).
- The server returns `etag` from Get and validates it on Update when supplied.
- Resource names are immutable.
- Auth scopes: `connectionpool.read` (Get) and `connectionpool.admin` (Update) are separate.

#### Examples
```
// ConnectionPool — normal state
{
  name: "instances/cache-001/connectionPool"
  max_connections: 1000
  active_connections: 743
  available_connections: 257
  observed_at: "2026-05-05T10:30:00Z"
  etag: "v1:8f3a"
}

// ConnectionPool — draining
{
  name: "instances/cache-002/connectionPool"
  max_connections: 1000
  active_connections: 412      // includes draining connections
  available_connections: 0     // clamped to zero during drain
  observed_at: "2026-05-05T10:30:01Z"
  etag: "v1:1c44"
}
```

### GetConnectionPool

```proto
service ConnectionPoolService {
  rpc GetConnectionPool(GetConnectionPoolRequest) returns (ConnectionPool);
}

message GetConnectionPoolRequest {
  string name = 1 [(google.api.field_behavior) = REQUIRED];
}
```

#### Design decisions
- **Unary**: capacity polling tolerates seconds-scale latency; this is snapshot retrieval, not state notification.
- **Returns the full ConnectionPool**: clients pick the fields they need.

#### Guidance
- Clients discard responses where `observed_at` is older than poll interval × 2.
- Autoscalers decide on the median of N samples to suppress churn.
- Recommended polling interval: 2–5 seconds. Shorter intervals add observation noise without proportional benefit.

**Possible errors** — see Errors for details
- `INSTANCE_NOT_REGISTERED`
- `INSTANCE_INITIALIZING`
- `INSTANCE_DRAINING`
- `COUNTER_INCONSISTENCY`
- `CONTROL_PLANE_AUTH_REQUIRED`

#### Examples

Autoscaler decision loop:

```
loop:
  s1 = GetConnectionPool(instances/cache-001/connectionPool)  // available: 257, observed_at: T0
  s2 = GetConnectionPool(...)                                  // available: 240, observed_at: T0+2s
  s3 = GetConnectionPool(...)                                  // available: 245, observed_at: T0+4s
  if median(s1, s2, s3).available < threshold:
    request scale up
```

### UpdateConnectionPool

```proto
service ConnectionPoolService {
  rpc UpdateConnectionPool(UpdateConnectionPoolRequest) returns (ConnectionPool);
}

message UpdateConnectionPoolRequest {
  ConnectionPool connection_pool = 1 [(google.api.field_behavior) = REQUIRED];
  google.protobuf.FieldMask update_mask = 2 [(google.api.field_behavior) = REQUIRED];
}
```

#### Design decisions
- **Standard Update + field_mask**: extensible if more mutable fields appear later, without breaking compatibility.
- **`max_connections` is the only writable path**: everything else is `OUTPUT_ONLY`.
- **Reject new max < current active**: forced disconnects are operationally undesirable; reductions go through graceful drain.
- **Returns the post-update resource**: clients can immediately confirm applied state, especially the updated `observed_at`.

#### Guidance
- `update_mask` is required; an empty mask is `INVALID_ARGUMENT`.
- Currently the only mutable path is `max_connections`. Other paths return `INVALID_ARGUMENT` + `IMMUTABLE_FIELD_IN_UPDATE_MASK`.
- A new `max_connections < active_connections` returns `FAILED_PRECONDITION` + `MAX_CONNECTIONS_BELOW_ACTIVE`.
- Allowed range is [1, deployment-dependent upper bound]. Out-of-range returns `INVALID_ARGUMENT` + `MAX_CONNECTIONS_OUT_OF_RANGE`.
- Etag mismatch returns `ABORTED` + `ETAG_MISMATCH`. Clients should retry with Get → Update.
- Capacity reduction takes effect immediately for the new ceiling, but excess existing connections are not force-closed; they decay naturally. New connections are admitted up to the new max.

**Possible errors** — see Errors for details
- `INSTANCE_NOT_REGISTERED`
- `INSTANCE_INITIALIZING`
- `INSTANCE_DRAINING`
- `COUNTER_INCONSISTENCY`
- `CONTROL_PLANE_AUTH_REQUIRED`
- `ETAG_MISMATCH` ← Update-specific
- `MAX_CONNECTIONS_BELOW_ACTIVE` ← Update-specific
- `MAX_CONNECTIONS_OUT_OF_RANGE` ← Update-specific
- `IMMUTABLE_FIELD_IN_UPDATE_MASK` ← Update-specific

#### Examples

Increasing capacity from 1000 to 1500:

```
1. Get → ConnectionPool{ max: 1000, etag: "v1:8f3a", ... }

2. Update(
     connection_pool: {
       name: "instances/cache-001/connectionPool",
       max_connections: 1500,
       etag: "v1:8f3a"
     },
     update_mask: { paths: ["max_connections"] }
   )
   → ConnectionPool{ max: 1500, etag: "v1:c2e7", observed_at: ... }
```

Concurrent update with etag conflict:

```
A. Get → etag: "v1:8f3a"
B. Get → etag: "v1:8f3a"
A. Update(etag: "v1:8f3a", max: 1500) → etag: "v1:c2e7"  // success
B. Update(etag: "v1:8f3a", max: 1200) → ABORTED (ETAG_MISMATCH)
B. Get → etag: "v1:c2e7" → retry or abort decision
```

### Errors

#### Design decisions
- **gRPC canonical codes**: aligned with existing retry middleware and observability dashboards.
- **`ErrorInfo.reason` strings on the wire**: new reasons can be added without breaking proto compatibility.
- **Reference proto enum (Option C)**: implementations switch on the enum, tests reference it, documentation cites it. The enum is for type safety; the wire stays string.
- **`INSTANCE_INITIALIZING` and `INSTANCE_DRAINING` split**: both are `UNAVAILABLE` but autoscaler responses differ — for initializing, wait; for draining, exclude immediately. Without separate reasons, the distinction collapses.
- **Etag mismatch as `ABORTED`**: AIP-193 / google.rpc.Code convention — retryable after state reconciliation.
- **`MAX_CONNECTIONS_BELOW_ACTIVE` as `FAILED_PRECONDITION`**: the request is well-formed (so not `INVALID_ARGUMENT`), but inconsistent with current state.
- **Fixed `domain`**: when the same error mechanism appears in other services, `domain` identifies the source.

#### Guidance

```proto
// Reference enum. Wire transmits ErrorInfo.reason as a string.
enum ConnectionPoolErrorReason {
  CONNECTION_POOL_ERROR_REASON_UNSPECIFIED = 0;
  INSTANCE_NOT_REGISTERED = 1;
  INSTANCE_INITIALIZING = 2;
  INSTANCE_DRAINING = 3;
  COUNTER_INCONSISTENCY = 4;
  CONTROL_PLANE_AUTH_REQUIRED = 5;
  ETAG_MISMATCH = 6;
  MAX_CONNECTIONS_BELOW_ACTIVE = 7;
  MAX_CONNECTIONS_OUT_OF_RANGE = 8;
  IMMUTABLE_FIELD_IN_UPDATE_MASK = 9;
}
```

Reason vocabulary:

| reason | gRPC code | method | condition | client behavior |
|---|---|---|---|---|
| `INSTANCE_NOT_REGISTERED` | NOT_FOUND | Get/Update | instance ID is unknown | review configuration; not retryable |
| `INSTANCE_INITIALIZING` | UNAVAILABLE | Get/Update | instance is starting up | honor `RetryInfo`; autoscaler treats as "warming, exclude" |
| `INSTANCE_DRAINING` | UNAVAILABLE | Get/Update | graceful shutdown in progress | honor `RetryInfo`; autoscaler excludes immediately |
| `COUNTER_INCONSISTENCY` | INTERNAL | Get/Update | server-side state inconsistency | not retryable; mark for health-check review |
| `CONTROL_PLANE_AUTH_REQUIRED` | PERMISSION_DENIED | Get/Update | data-plane credentials used | not retryable; configuration bug |
| `ETAG_MISMATCH` | ABORTED | Update | etag does not match current | retry Get → Update |
| `MAX_CONNECTIONS_BELOW_ACTIVE` | FAILED_PRECONDITION | Update | new max < current active | drain first, or wait for active to fall |
| `MAX_CONNECTIONS_OUT_OF_RANGE` | INVALID_ARGUMENT | Update | value outside allowed range | adjust to allowed range |
| `IMMUTABLE_FIELD_IN_UPDATE_MASK` | INVALID_ARGUMENT | Update | mask references an immutable path | remove `OUTPUT_ONLY` paths from mask |

Supplementary rules:
- `domain` is fixed at `connectionpool.cacheme.example.com`.
- New reasons are backward-compatible additions; clients fall back to the gRPC code when seeing an unknown reason.
- `UNAVAILABLE` responses always carry `RetryInfo` with a recommended delay.
- `metadata` always includes `instance_id` for correlation across concurrent requests.

#### Examples

```
// UNAVAILABLE — instance starting up
google.rpc.Status {
  code: 14  // UNAVAILABLE
  message: "instance cache-001 is initializing"
  details: [
    ErrorInfo {
      reason: "INSTANCE_INITIALIZING"
      domain: "connectionpool.cacheme.example.com"
      metadata: { instance_id: "cache-001" }
    }
    RetryInfo { retry_delay: 5s }
  ]
}

// ABORTED — etag conflict on Update
google.rpc.Status {
  code: 10  // ABORTED
  message: "etag mismatch on instance cache-001"
  details: [
    ErrorInfo {
      reason: "ETAG_MISMATCH"
      domain: "connectionpool.cacheme.example.com"
      metadata: {
        instance_id: "cache-001"
        expected_etag: "v1:8f3a"
        current_etag: "v1:c2e7"
      }
    }
  ]
}

// FAILED_PRECONDITION — new max would force-disconnect
google.rpc.Status {
  code: 9  // FAILED_PRECONDITION
  message: "new max_connections (500) is below current active (743)"
  details: [
    ErrorInfo {
      reason: "MAX_CONNECTIONS_BELOW_ACTIVE"
      domain: "connectionpool.cacheme.example.com"
      metadata: {
        instance_id: "cache-001"
        requested_max: "500"
        current_active: "743"
      }
    }
  ]
}

// INVALID_ARGUMENT — update_mask contains immutable field
google.rpc.Status {
  code: 3  // INVALID_ARGUMENT
  message: "update_mask contains immutable path 'name'"
  details: [
    ErrorInfo {
      reason: "IMMUTABLE_FIELD_IN_UPDATE_MASK"
      domain: "connectionpool.cacheme.example.com"
      metadata: {
        instance_id: "cache-001"
        invalid_path: "name"
      }
    }
    BadRequest {
      field_violations: [{
        field: "update_mask.paths[0]"
        description: "field 'name' is immutable"
      }]
    }
  ]
}
```

## A Note on Authorship

The worked example above is a format illustration, not content to copy. Each Design Doc's API section must be written by the engineer based on the actual design under consideration. This reference shows *how the format is shaped*; the substance — which fields exist, which errors are exposed, what each rule actually says — is the engineer's design work.

## Related References

- [AIP-121] Resource-oriented design — https://google.aip.dev/121
- [AIP-131] Standard methods: Get — https://google.aip.dev/131
- [AIP-134] Standard methods: Update — https://google.aip.dev/134
- [AIP-148] Standard fields (etag, name) — https://google.aip.dev/148
- [AIP-161] Field masks — https://google.aip.dev/161
- [AIP-193] Errors — https://google.aip.dev/193
- [AIP-203] Field behavior documentation — https://google.aip.dev/203
