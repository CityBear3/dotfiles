# Detailed Design Section Guide

This guide helps determine what to write in the Detailed Design section of a Design Doc.
The Detailed Design section documents **architectural decisions and their rationale** — not implementation instructions or low-level specifications. The code itself serves as the detailed specification.

The content and structure of this section depend on three axes.

## Three Axes

| Axis | Determines |
|---|---|
| **Design Target** | What decisions to document |
| **Design Purpose** | What trade-offs to emphasize |
| **Scope** | What abstraction level to reason at |

Identify where your design falls on each axis before writing. The combination shapes the section.

---

## Axis 1: Design Target — What Decisions to Document

### API / Interface

- Contract semantics: what guarantees does this interface make to its consumers?
- Versioning and compatibility strategy: how will the contract evolve without breaking consumers?
- Error model: what failure modes are exposed and what is the consumer's expected response?

### Data Model

- Entity relationships: why are entities structured this way and what alternatives were considered?
- Lifecycle decisions: what governs how entities are created, change state, and become obsolete?
- Consistency trade-offs: what consistency level is required and what is sacrificed to achieve it?

### Architecture / Component Structure

- Responsibility boundaries: why are components divided this way?
- Communication patterns: why synchronous vs. asynchronous? Push vs. pull?
- Dependency direction: what depends on what, and why is this direction chosen?

### Workflow / Business Logic

- State transitions: what states exist, what triggers transitions, and why these states?
- Decision criteria: what determines branching, and where is the boundary of each path?
- Edge cases: what boundary conditions are architecturally significant and how does the design handle them?

### Data Flow

- Flow topology: what is the path from source to sink and why?
- Ordering and integrity: what guarantees does the design make about data ordering and correctness?
- Observability: where are the critical points to monitor and what do they reveal?

These categories often overlap. A single design may cover multiple targets — document each one only where an architecturally significant decision exists.

---

## Axis 2: Design Purpose — What Trade-offs to Emphasize

The purpose of the design determines which trade-offs deserve deeper discussion.

### Performance Improvement

- What is the current bottleneck and how was it identified?
- What quantitative target defines success?
- What is being traded for performance (complexity, consistency, resource cost)?

### Extensibility / Flexibility

- What kinds of future changes does this design accommodate, and what does it explicitly not accommodate?
- What is the cost of the added flexibility (indirection, complexity)?

### Reliability / Fault Tolerance

- What are the failure modes and what is the blast radius of each?
- What recovery strategy is chosen and what are its limitations?
- What consistency guarantees hold under failure?

### Migration / Refactoring

- What is the migration path and why incremental vs. big-bang (or vice versa)?
- How do old and new coexist during transition?
- What is the rollback strategy and at what point does rollback become infeasible?

### New Feature / Capability

- How does the feature integrate with existing architecture without distorting it?
- What new concepts are introduced to the domain and why are they necessary?
- What is the impact on existing interfaces and contracts?

---

## Axis 3: Scope — What Abstraction Level to Reason At

### Single Component

Designing within one module, service, or bounded context.

Document:
- Responsibility boundaries within the component and the rationale for the separation
- Key invariants the design must preserve
- The contract this component exposes to the rest of the system

### Single System

Designing across multiple components within one system.

Document:
- High-level architecture: how components relate and why they are structured this way
- Contracts between components: APIs, events, shared data structures
- Data flow through the system and the rationale for the chosen topology
- Cross-cutting concerns: how authentication, observability, and error handling are approached at the system level

### Multiple Systems

Designing across system boundaries.

Document:
- Overall topology: how systems relate and why this structure was chosen
- Inter-system contracts: API specifications, event schemas, shared data formats — and who owns them
- Consistency model across systems: what guarantees are made and what trade-offs are accepted
- Failure isolation: how failures in one system are prevented from cascading
- Ownership and coordination: which teams own which boundaries, and how deployment is coordinated

---

## How to Use This Guide

1. Before writing the Detailed Design section, identify your position on each axis.
2. Use the relevant items as a checklist of **decisions to document** — not a template to fill in.
3. Only describe what is architecturally significant. Skip anything that is self-evident from the code or not relevant to your design.
4. If an important concern is not listed here, add it.
5. Every item in the Detailed Design section should answer "**what was decided and why**" — not "how to implement it."