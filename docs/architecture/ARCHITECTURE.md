# Target Architecture

## Status

- **Document state:** Proposed target architecture
- **Applies to:** private development toward a reusable Rust library
- **Last updated:** 2026-07-27
- **Decision model:** architecture decisions are recorded through ADRs and may change when evidence changes

## Purpose

`presidio-rs` is intended to become a reusable, offline-first Rust library for detecting and transforming supported sensitive-data spans in text.

The architecture must support more than one Rust consumer without allowing any one consumer to define the core library around its private policy, runtime, or product assumptions.

The project is not trying to reproduce every Microsoft Presidio feature. It is building a small, explainable Rust privacy kernel with optional capabilities and measurable behavior.

## Architectural goals

1. Preserve byte-accurate findings against original UTF-8 input.
2. Separate detection evidence from consumer policy decisions.
3. Support pattern, structural, contextual, and future semantic recognizers through one extensible contract.
4. Preserve candidate provenance before overlap or policy resolution.
5. Make anonymization fallible, observable, and testable.
6. Keep the default model-free core offline, synchronous, dependency-light, and embeddable.
7. Allow multiple Rust consumers to adopt the crate without product-specific coupling.
8. Keep optional heavy capabilities, especially semantic models, outside the default dependency graph.
9. Treat public APIs, serialized formats, entity identifiers, and output semantics as compatibility surfaces.
10. Require evaluation evidence before changing default recognizers, thresholds, or superiority claims.

## Non-goals

The core project does not initially aim to provide:

- a hosted service;
- a general policy-management platform;
- OCR, image, DICOM, audio, or video redaction;
- a legal or regulatory compliance guarantee;
- arbitrary structured-data de-identification;
- automatic runtime model downloads;
- a permanent commitment to any specific NER runtime or model;
- a stable C ABI before the Rust API stabilizes;
- `no_std` support without a demonstrated consumer requirement; or
- full behavioral compatibility with Microsoft Presidio.

## System context

```text
                         Consumer-owned boundary

  logs      prompts      documents      tool output      custom text
    │          │              │               │                │
    └──────────┴──────────────┴───────────────┴────────────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │   presidio-rs core  │
                    │                     │
                    │ TextDocument        │
                    │ RecognizerRegistry  │
                    │ Analyzer            │
                    │ Resolver            │
                    │ Anonymizer          │
                    └─────────────────────┘
                               │
                     Findings and reports
                               │
                               ▼
                    Consumer-owned policy
                  allow | block | redact | review

 Optional capability crates:

  presidio-rs-eval          evaluation and error analysis
  presidio-rs-ner-*         semantic recognizer adapters
  consumer adapters         maintained outside the core unless broadly reusable
```

The library detects and transforms. The consumer decides whether output may proceed, whether errors fail open or fail closed, what telemetry is appropriate, and what data-handling obligations apply.

## Core value types

The current public model of `EntityType`, raw `usize` offsets, and `f32` scores is useful for a prototype but too weak for a durable multi-consumer contract.

The target value model is:

```rust
pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub struct EntityId(Arc<str>);

pub struct Confidence(f32);

pub struct RecognizerId(Arc<str>);

pub struct Finding {
    pub entity: EntityId,
    pub span: Span,
    pub confidence: Confidence,
    pub recognizer: RecognizerId,
    pub recognizer_version: Arc<str>,
    pub evidence: Vec<Evidence>,
    pub locale: Option<Locale>,
}
```

Required invariants:

- spans are ordered, non-negative byte ranges into the original UTF-8 input;
- span construction validates character boundaries where transformation requires them;
- confidence is finite and constrained to `[0.0, 1.0]`;
- entity and recognizer identifiers are stable, validated, and serialization-safe;
- a finding records which recognizer produced it and why;
- scores from different recognizer families are not assumed to be calibrated equivalents.

The existing `EntityType` enum should remain as a temporary compatibility facade while the open identifier model is introduced.

## Text and offset model

`TextDocument` owns the relationship among:

- original UTF-8 text;
- normalized views;
- tokenized views;
- normalized-to-original offset mappings;
- input size limits; and
- document identity used to prevent applying findings to the wrong text.

```rust
pub struct TextDocument<'a> {
    original: &'a str,
    document_id: DocumentId,
    // lazily computed normalized and tokenized views
}
```

### Offset rule

All externally visible spans use byte offsets into the original input.

Recognizers may operate on normalized or tokenized representations only when they can map findings back to original input coordinates. A recognizer that cannot provide valid source offsets must return an error or explicitly unsupported result rather than an approximate span.

### Input limits

Consumers must be able to configure maximum input size and maximum finding count. The library should expose deterministic behavior when limits are reached:

- reject the request;
- return a truncated analysis with an explicit status; or
- process bounded windows under a documented policy.

Silent truncation is not acceptable at a privacy boundary.

## Recognizer contract

The analyzer should depend on a backend-neutral recognizer interface rather than on `PatternRecognizer` directly.

```rust
pub trait Recognizer: Send + Sync {
    fn metadata(&self) -> &RecognizerMetadata;

    fn supports(&self, request: &AnalysisRequest) -> bool;

    fn analyze(
        &self,
        document: &TextDocument<'_>,
        request: &AnalysisRequest,
        findings: &mut Vec<Finding>,
    ) -> Result<(), RecognitionError>;
}
```

`PatternRecognizer` becomes one implementation. Future implementations may include:

- checksum or structural recognizers;
- dictionary or gazetteer recognizers;
- vendor-secret recognizers;
- phone-number or country-specific parsing libraries;
- semantic token-classification recognizers; and
- organization-specific recognizers supplied by consumers.

### Recognizer metadata

Each recognizer exposes:

- stable recognizer ID;
- version;
- supported entities;
- supported locales or countries;
- detection mechanism;
- required capabilities;
- default-enabled status;
- source or prior-art attribution; and
- evaluation receipt identifier when available.

This metadata supports diagnostics, regression analysis, configuration, and consumer compatibility.

## Registry and construction

The analyzer is immutable after construction and shareable through `Arc`.

```rust
let analyzer = AnalyzerBuilder::new()
    .with_predefined_recognizers()
    .with_recognizer(custom)
    .with_resolution_policy(ResolutionPolicy::ConservativeRedaction)
    .build()?;
```

Runtime mutation of the registry should be deprecated after an equivalent builder API exists. Immutable construction makes concurrent use and consumer reasoning simpler.

Registry filtering should support:

- entity IDs;
- recognizer IDs;
- locale and country;
- capability requirements;
- default or explicitly enabled recognizers; and
- consumer-provided allow and deny lists.

## Analysis pipeline

```text
Input validation
      │
      ▼
TextDocument and source-coordinate model
      │
      ▼
Recognizer selection
      │
      ▼
Candidate collection with evidence
      │
      ▼
Candidate validation and context enhancement
      │
      ▼
Thresholding under explicit policy
      │
      ▼
Conflict and overlap resolution
      │
      ▼
AnalysisReport
```

The analyzer must preserve all qualifying candidates until an explicit resolution stage.

## Evidence and explainability

`Evidence` should be an extensible enum or tagged structure containing information such as:

- pattern ID and match method;
- checksum or structural validation result;
- supporting or negative context token;
- prefilter or keyword hit;
- model identity and label;
- raw model probability;
- normalization applied;
- allowlist or deny-list decision; and
- resolution decision.

The library should support two report levels:

- **compact:** findings and essential status for normal runtime use;
- **diagnostic:** detailed evidence for tests, debugging, evaluation, and audits.

Diagnostic reports must avoid copying plaintext sensitive values unless the caller explicitly opts in.

## Context handling

Substring context matching should be replaced with token- or boundary-aware matching.

The context subsystem should support:

- positive context;
- negative context;
- configurable proximity;
- case handling;
- locale-aware tokenization where available; and
- evidence explaining the score change.

Context rules must not imply that all scores are statistically calibrated. They are heuristic evidence unless an evaluation demonstrates otherwise.

## Conflict and overlap resolution

Resolution is a policy stage, not an incidental sort operation.

The core should provide at least:

- `ReportAll`: retain all qualifying candidates;
- `BestCandidate`: choose one candidate according to explicit priority rules;
- `ConservativeRedaction`: merge the union of qualifying overlapping spans for transformation safety; and
- a custom resolver interface.

The analysis report records both source candidates and resolved findings when diagnostics are enabled.

Resolution rules must define tie-breaking, nested spans, adjacent spans, equal scores, entity priority, and same-recognizer duplicates.

## Anonymization architecture

Anonymization should be split into planning and execution.

```text
AnalysisReport
      │
      ▼
AnonymizationPolicy
      │
      ▼
AnonymizationPlan
      │
      ▼
Validated transformation
      │
      ▼
AnonymizationReport
```

The public API should become fallible:

```rust
pub fn anonymize(
    document: &TextDocument<'_>,
    findings: &[ResolvedFinding],
    policy: &AnonymizationPolicy,
) -> Result<AnonymizationReport, AnonymizationError>;
```

The report contains:

- transformed text;
- applied operations;
- source and output spans;
- skipped or rejected operations;
- warnings;
- document identity;
- policy version; and
- engine version.

Invalid offsets, overlapping plans, unsupported operators, and mismatched document identity must not be silently ignored.

## Pseudonymization and cryptography

The current salted SHA-256 operator should be deprecated before a production-oriented release.

The preferred direction is keyed pseudonymization, such as HMAC-SHA-256, with:

- caller-provided secret material;
- secret wrappers that avoid accidental `Debug` output;
- zeroization where practical;
- explicit domain separation;
- tenant or context isolation;
- output format and truncation policy;
- collision analysis; and
- documented rotation and correlation behavior.

Cryptographic changes require an ADR, independent review where practical, and test vectors.

Encryption-based reversible anonymization is not part of the core until key management, authenticated metadata, nonce handling, and consumer demand are explicitly designed.

## Configuration

Programmatic construction is the primary API during early development.

A serialized configuration format should be introduced only after the underlying concepts stabilize. When added, it requires:

- an explicit schema version;
- stable recognizer and entity IDs;
- validation without panics;
- unknown-field behavior;
- migration guidance;
- secure defaults; and
- configuration fingerprints for reproducibility.

Configuration evolution must not be allowed to become an undocumented second public API.

## Crate boundaries

### Initial shape

Keep one primary crate while the architecture is still moving. Internal modules may be reorganized without imposing premature package boundaries.

### Split criteria

Create a separate crate only when at least one of these is true:

- it introduces a materially heavier optional dependency graph;
- it has a different release or licensing lifecycle;
- it can be consumed independently;
- it requires a different MSRV or platform support matrix; or
- isolating it prevents default consumers from paying build, binary, or audit cost.

Expected future crates:

- `presidio-rs-eval`: corpus formats, evaluation, error analysis, and reports;
- `presidio-rs-ner-candle` or another backend-named semantic adapter;
- optional CLI tooling after the library contract stabilizes.

The project should not create microcrates merely to make the repository look architectural.

## Consumer integration surfaces

### Primary: Rust library

The primary supported integration is an in-process Rust library with synchronous APIs.

Reasons:

- pattern analysis is CPU-bound and does not require async I/O;
- consumers may use Tokio, async-std, no runtime, or embedded executors;
- synchronous core APIs remain easy to call from async applications through bounded worker strategies; and
- avoiding an async trait contract keeps the core runtime-neutral.

### Batch analysis

Batch APIs may be added for amortized setup and semantic-model efficiency. They should be synchronous iterators or explicit batch requests unless a concrete consumer requires async orchestration.

### Streaming

Streaming redaction is a later capability. A PII value can cross arbitrary chunk boundaries, so naive per-chunk redaction is unsafe.

Any streaming design requires:

- bounded lookbehind;
- a commit watermark;
- explicit maximum recognizer span assumptions;
- final-buffer flush semantics;
- tests for chunk boundary permutations; and
- a documented degraded-guarantee mode when full detection cannot be preserved.

### WASM

WASM is a feasibility target, not an initial guarantee. The core should avoid unnecessary blockers, but WASM support requires a dedicated platform matrix, performance evaluation, and clear entropy or cryptographic behavior.

### FFI

C ABI, Python, Node, or other bindings should follow Rust API stabilization. Bindings multiply compatibility and security surfaces and must not drive the core prematurely.

### `no_std`

`no_std` is deferred. The current regex and allocation model make it non-trivial, and no demonstrated consumer requirement currently justifies the complexity.

## Semantic recognition adapters

Semantic recognition is optional and outside the critical path for the model-free core.

The core exposes the recognizer contract; a semantic adapter owns:

- tokenizer and model loading;
- model artifact provenance;
- model licensing;
- label mapping;
- source-offset alignment;
- windowing and truncation;
- device selection;
- inference errors;
- resource limits; and
- model-specific evaluation.

No runtime downloads are permitted in the default project posture. Model bundles must be vendored or supplied by the consumer with immutable identity and integrity checks.

A semantic backend is adopted only when evaluation demonstrates that the quality gain justifies dependency, artifact-size, startup, memory, and maintenance costs.

## Compatibility policy

Before the first published release, the project still treats downstream Rust consumers as real compatibility partners.

Compatibility surfaces include:

- public Rust API;
- feature names and defaults;
- entity and recognizer IDs;
- serialized findings and reports;
- byte-offset semantics;
- score and threshold semantics;
- anonymization output; and
- error classifications.

Required controls:

- consumer compile fixtures;
- `cargo-semver-checks` after a versioned baseline exists;
- deprecation before removal where practical;
- migration notes for breaking changes;
- feature-matrix CI; and
- downstream pilot tests before releases that change semantics.

## Security architecture

The core security posture includes:

- no library-initiated network or filesystem I/O in the default core;
- crate-local `unsafe` forbidden;
- bounded input, candidate, and output behavior;
- validated offsets and transformations;
- explicit failure states;
- minimal plaintext duplication in diagnostics;
- dependency and license controls;
- fuzzing and adversarial tests;
- corpus provenance and privacy review; and
- documented caller responsibilities.

Rust memory safety does not prove detection correctness, cryptographic correctness, availability, or safe consumer behavior.

## Observability

The library returns structured reports and does not emit sensitive plaintext logs by default.

Optional tracing integration may emit:

- recognizer IDs;
- duration;
- input length;
- finding count;
- resolution count;
- limit status; and
- error class.

It must not emit matched values or full input unless a caller explicitly enables a diagnostic mode with appropriate warnings.

## Architecture governance

Architecture is developed continuously rather than frozen in an initial design phase.

Required cadence:

- weekly architecture review during active development;
- ADRs for material public API, dependency, semantic, normalization, resolution, cryptographic, and compatibility decisions;
- a phase-exit architecture review before each roadmap gate;
- a consumer compatibility review at least every four weeks;
- a landscape review at least every eight weeks while parallel Rust efforts are evolving; and
- explicit validation or reversal of assumptions after spikes and pilots.

## Open architectural questions

The following remain intentionally unresolved:

1. Final public project and crate name.
2. Whether an existing Rust project should be adopted, extended, or collaborated with instead of duplicating its scope.
3. Exact open entity-identifier representation and compatibility bridge from `EntityType`.
4. Whether configuration belongs in the core crate or a separate adapter.
5. Whether phone-number support should depend on a mature parsing crate.
6. Whether semantic recognition should use Candle, ONNX Runtime, Tract, or another backend.
7. Whether keyed pseudonymization belongs in the core or a separate cryptographic adapter.
8. Which locales and countries are in the supported quality envelope.
9. Whether WASM is a real consumer requirement.
10. What evidence threshold qualifies a recognizer for default enablement.

These questions are scheduled as decisions in the development plan rather than disguised as implementation details.
