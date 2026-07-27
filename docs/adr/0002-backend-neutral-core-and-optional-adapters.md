# ADR 0002: Backend-Neutral Core with Optional Capability Adapters

- **Status:** Accepted for planning; implementation remains phased
- **Date:** 2026-07-27
- **Decision owners:** MythologIQ Labs LLC maintainers

## Context

The current implementation directly orchestrates `PatternRecognizer` values and reserves several entity types for a possible future NER backend.

The project may serve multiple Rust consumers with different runtime, dependency, platform, and policy constraints. Some consumers may need only deterministic pattern and structural detection. Others may eventually require semantic recognition, batch processing, WASM, or specialized recognizers.

Several parallel Rust projects already expose recognizer traits, NLP seams, and broader feature sets. Recreating a monolithic full-suite port would increase duplication, dependency weight, and maintenance burden without establishing a clear differentiator.

## Decision

`presidio-rs` will evolve toward a small backend-neutral core that owns:

- source text and byte-offset semantics;
- validated entity, span, confidence, finding, and error types;
- recognizer registration and orchestration;
- candidate evidence and provenance;
- context enhancement;
- explicit threshold and conflict resolution;
- anonymization planning and execution; and
- compatibility and report contracts.

Recognition mechanisms are supplied through a narrow `Recognizer: Send + Sync` contract.

Pattern recognition remains a first-party implementation. Heavy or specialized capabilities are isolated as optional adapters when justified by dependency, platform, release, or licensing boundaries.

Expected optional capabilities include:

- semantic or NER backends;
- evaluation and corpus tooling;
- CLI surfaces;
- bindings or FFI; and
- consumer-specific adapters.

The default core remains synchronous, offline, and free of runtime model downloads.

## Policy boundary

The reusable core returns findings and transformations. Consumers own:

- allow, block, review, or release decisions;
- fail-open or fail-closed behavior;
- telemetry and logging;
- storage and retention;
- compliance interpretation;
- orchestration; and
- product-specific policy.

Consumer requirements enter the core only when independently justified for multiple plausible users.

## Crate structure

The project will remain one primary crate during architectural stabilization.

A new crate is created only when the component:

- introduces a materially heavier dependency graph;
- has a distinct release or license lifecycle;
- is independently useful;
- requires a different MSRV or platform matrix; or
- would otherwise impose cost on default consumers.

## Alternatives considered

### Permanent pattern-only crate

Rejected as the sole architectural direction because semantic detection may be necessary for names, prose locations, and other unstructured entities. The pattern core remains independently useful, but the architecture must permit optional semantic recognition.

### Monolithic crate containing pattern, NLP, model, CLI, service, and bindings

Rejected because every consumer would inherit unnecessary build, audit, platform, and maintenance cost. It would also make optional capability choices part of the core compatibility contract.

### Mirror Microsoft Presidio's Python package structure directly

Rejected. Presidio remains a valuable architectural reference, but Rust ownership, traits, and package boundaries should be designed idiomatically and against actual consumer requirements.

### Select a semantic runtime immediately

Rejected. Candle, ONNX Runtime, Tract, and existing Rust implementations have different licensing, native-dependency, artifact, platform, and maintenance implications. The core must not depend on that choice.

### Split into multiple crates immediately

Rejected. Premature package boundaries would create release and dependency overhead while core concepts are still changing.

## Consequences

### Positive

- Pattern-only consumers retain a small dependency graph.
- Multiple recognizer mechanisms can use one analysis and anonymization contract.
- Semantic runtime decisions remain reversible.
- Consumer policy remains outside the reusable core.
- The architecture can support collaboration with existing Rust projects.
- Heavy capability costs are visible through explicit feature or crate choices.

### Costs

- Trait and evidence design requires more work before feature expansion.
- Optional adapters require integration and compatibility testing.
- Findings from different recognizer families may not share calibrated score semantics.
- More explicit error and report contracts increase initial API design effort.

### Risks

- The recognizer trait may become too broad or too restrictive.
- Adapter boundaries may hide inefficient data copying.
- Multiple backends may fragment behavior and quality expectations.
- A general architecture may be overbuilt before consumer needs are proven.

## Controls

- Validate the trait through at least three recognizer mechanisms before treating it as stable.
- Use real consumer pilots and downstream compile fixtures.
- Preserve raw evidence and recognizer identity.
- Report backend-specific evaluation and resource characteristics.
- Require ADRs for heavy dependencies and semantic runtime selection.
- Keep serialized configuration deferred until concepts stabilize.

## Validation

This decision is validated when:

- pattern recognizers implement the common contract;
- an external custom recognizer can be added without modifying the core;
- at least two consumers integrate without private forks;
- optional capability experiments do not enter the default dependency graph; and
- performance and ergonomics remain acceptable under measured workloads.

## Revisit conditions

Revisit when:

- a credible existing Rust project should become the adopted core;
- trait dispatch creates measured unacceptable overhead;
- most consumers require one semantic backend by default;
- platform constraints make the current crate shape impractical; or
- consumer pilots demonstrate that the core and policy boundary is incorrectly placed.
