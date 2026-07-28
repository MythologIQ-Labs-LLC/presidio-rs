# Architecture

## Status

- **Document state:** Active target architecture
- **Applies to:** Public development of a reusable Rust privacy library
- **Last updated:** 2026-07-28
- **Decision model:** Material decisions are recorded through ADRs and may change when evidence changes
- **Current governing decision:** [ADR 0008](../adr/0008-stage-secure-functional-alpha-through-evidence-gated-pipeline.md)

## Purpose

`presidio-rs` is an offline-first Rust library for detecting and transforming supported sensitive-data spans in UTF-8 text.

The project is not attempting to reproduce every Microsoft Presidio feature or Python class boundary. It is building a small, explainable Rust privacy kernel whose behavior can be bounded, measured, and adopted by more than one consumer without product-specific coupling.

The useful inheritance from Python Presidio includes architecture, release history, design discussions, failures, fixes, false-positive tradeoffs, security changes, evaluation practices, configuration evolution, and governance lessons. Those lessons inform decisions but do not make Python behavior a compatibility oracle.

## Alpha gates

The project distinguishes two alpha states.

### Public foundation alpha

The repository is:

- publicly readable and anonymously cloneable;
- buildable and testable from public documentation;
- governed through contribution, security, provenance, and release controls;
- explicit about maturity and unsupported behavior; and
- usable by contributors without private project context.

This gate establishes a credible public project. It does not establish a safe transformation boundary.

### Secure functional alpha

The authoritative text pipeline is:

- bounded;
- source-exact;
- explicit about supported and unsupported scope;
- deterministic under named policies;
- failure-safe;
- auditable without plaintext by default;
- reproducibly evaluated; and
- exercised through regressions, fuzzing, and an external consumer path.

A secure functional alpha does not guarantee complete PII detection, production certification, regulatory compliance, stable `1.0` compatibility, or drop-in compatibility with Microsoft Presidio.

## Architectural goals

1. Preserve exact byte coordinates against original UTF-8 input.
2. Bind findings and transformations to the exact source document.
3. Preserve recognition evidence before threshold, context, or resolution policy changes it.
4. Separate recognition mechanics, context, thresholding, resolution, and transformation.
5. Make resolution explicit, deterministic, and versioned.
6. Make anonymization fallible, atomic, observable, and testable.
7. Keep the default core offline, synchronous, dependency-light, and embeddable.
8. Require conservative, locale-aware, measured defaults.
9. Keep semantic models and other heavy capabilities outside the default dependency graph.
10. Treat public APIs, entity IDs, recognizer IDs, policy IDs, serialized formats, errors, and output semantics as compatibility surfaces.
11. Require evidence before changing defaults or making quality, security, compatibility, or performance claims.
12. Preserve the option to adopt, collaborate with, or migrate to a better Rust project.

## Explicit non-goals for secure functional alpha

The alpha does not include:

- hosted HTTP services;
- authentication or authorization infrastructure;
- general policy-management software;
- structured or tabular de-identification orchestration;
- OCR, image, DICOM, audio, or video redaction;
- automatic runtime model downloads;
- streaming redaction;
- stable cross-language bindings;
- a stable serialized configuration format;
- broad semantic recognition in the default crate;
- `no_std` support without a demonstrated consumer requirement;
- a legal or regulatory compliance guarantee; or
- full behavioral compatibility with Microsoft Presidio.

## Decision and delivery order

Material alpha work follows this order:

```text
Presidio evidence and decision ledger
        |
        v
Secure alpha contract and threat model
        |
        v
Bounded document and request contracts
        |
        v
Recognition, evidence, and safe defaults
        |
        v
Context and threshold policy
        |
        v
Named resolution policy
        |
        v
Complete anonymization-plan validation
        |
        v
Atomic execution and auditable report
        |
        v
Evaluation, regressions, fuzzing, and consumer validation
```

Resolution precedes authoritative anonymization. Recognizer expansion follows the secure functional alpha gate rather than competing with it.

## System context

```text
                           Consumer-owned boundary

  logs       prompts       documents       tool output       custom text
    |           |               |                |                 |
    +-----------+---------------+----------------+-----------------+
                                |
                                v
                     +-----------------------+
                     |   presidio-rs core    |
                     |                       |
                     | TextDocument          |
                     | AnalysisRequest       |
                     | Recognizer selection  |
                     | Candidate collection  |
                     | Context and threshold |
                     | Resolution            |
                     | Anonymization plan    |
                     | Atomic execution      |
                     +-----------------------+
                                |
                      Reports and operations
                                |
                                v
                      Consumer-owned decision
                     allow | block | redact | review

 Optional capability boundaries:

   evaluation tools       corpus, metrics, errors, differential learning
   semantic adapters      tokenizer, model, device, artifact lifecycle
   consumer adapters      services, storage, orchestration, interfaces
```

The core detects and transforms text. The consumer decides whether output may proceed, whether incomplete analysis fails open or closed, what telemetry is appropriate, and which legal, operational, and data-handling obligations apply.

## Authoritative pipeline

```text
Input validation and limits
        |
        v
TextDocument and exact source identity
        |
        v
Recognizer selection by metadata and request policy
        |
        v
Validated source-bound candidate emission
        |
        v
Boundary-aware context and threshold decisions
        |
        v
Named and versioned resolution policy
        |
        v
AnalysisReport with candidates, resolved findings, issues, and limits
        |
        v
AnonymizationPolicy and complete plan validation
        |
        v
Atomic transformation
        |
        v
AnonymizationReport with source-to-output operation records
```

Each stage has a distinct input, output, error contract, and diagnostic evidence surface.

## Core value types

The durable public model uses validated values rather than raw primitive fields.

```rust
pub struct Span {
    start: usize,
    end: usize,
}

pub struct EntityId(Arc<str>);
pub struct Confidence(f32);
pub struct RecognizerId(Arc<str>);
pub struct DocumentId(Arc<str>);

pub struct Finding {
    entity: EntityId,
    span: Span,
    confidence: Confidence,
    recognizer: RecognizerId,
    recognizer_version: Arc<str>,
    evidence: Vec<Evidence>,
    locale: Option<Locale>,
    document_binding: DocumentBinding,
}
```

Required invariants:

- spans are ordered byte ranges into original UTF-8 input;
- transformation spans begin and end on UTF-8 character boundaries;
- confidence is finite and constrained to `[0.0, 1.0]`;
- identifiers are bounded, validated, and serialization-safe;
- findings preserve recognizer identity, version, source binding, and evidence;
- scores from different recognizer families are not assumed to be calibrated equivalents; and
- no raw sensitive value is required in a finding or diagnostic record.

`EntityType` and `RecognizerResult` remain legacy-compatible facades until consumers migrate to the authoritative path.

## Text and source-coordinate model

`TextDocument` owns the relationship among:

- original UTF-8 text;
- opaque document identity;
- source byte length;
- source fingerprint;
- normalized or tokenized views when available;
- normalized-to-original offset mappings; and
- input-size limits.

All externally visible spans use byte offsets into the original input.

Recognizers may operate on normalized, tokenized, or model-specific views only when they can map findings exactly back to original coordinates. A recognizer that cannot produce exact source offsets returns a typed failure or unsupported result. Approximate spans cannot enter resolution or transformation.

Document fingerprints are integrity metadata, not encryption or anonymity. Consumers must treat document IDs, fingerprints, and reports according to their sensitivity and retention policies.

## Resource model

Resource behavior is deterministic and explicit.

The authoritative request supports limits for:

- input bytes;
- selected recognizers;
- candidates;
- retained issues;
- diagnostic evidence;
- backend work where the backend supports a limit contract; and
- transformed output size.

A limit may produce:

- request rejection;
- a typed incomplete-analysis status;
- a bounded report with explicit truncation; or
- a policy-defined refusal to transform.

Silent truncation is not acceptable at a privacy boundary.

## Recognizer contract

The analyzer depends on one backend-neutral contract.

```rust
pub trait Recognizer: Send + Sync {
    fn metadata(&self) -> &RecognizerMetadata;

    fn supports(&self, request: &AnalysisRequest) -> bool;

    fn recognize(
        &self,
        document: &TextDocument<'_>,
        request: &AnalysisRequest,
        emitter: &mut CandidateEmitter<'_, '_>,
    ) -> Result<(), RecognitionError>;
}
```

`PatternRecognizer` is one implementation. Optional implementations may include:

- structural and checksum recognizers;
- dictionaries or gazetteers;
- vendor-secret recognizers;
- mature phone-number or country-specific parsing libraries;
- semantic token-classification adapters; and
- organization-specific consumer recognizers.

The candidate emitter enforces entity declaration, exact spans, finite confidence, authoritative recognizer identity, source binding, and remaining resource limits.

## Recognizer metadata and default promotion

Each recognizer exposes:

- stable recognizer ID;
- version;
- supported entities;
- supported locales and countries;
- detection mechanism;
- required capabilities;
- source and prior-art attribution;
- default-enabled status; and
- evaluation receipt identity when available.

A recognizer may be default-enabled only when all of these are true:

1. its intended locale and country scope are explicit;
2. its source and provenance are acceptable;
3. false-positive and false-negative regressions exist;
4. its evaluation receipt identifies corpus, version, configuration, and limitations;
5. its context and threshold behavior are documented;
6. its performance and resource behavior fit the default core; and
7. maintainers accept its ongoing maintenance cost.

Country-specific, weak-pattern, highly ambiguous, or unevaluated recognizers remain opt-in.

Recognizer count is not a maturity metric.

## Registry and construction

Programmatic typed construction is the primary alpha API.

The target builder is immutable after construction and shareable through `Arc`.

```rust
let analyzer = AnalyzerBuilder::new()
    .with_default_recognizers()
    .with_recognizer(custom)
    .with_context_policy(context)
    .with_resolution_policy(ResolutionPolicy::ConservativeRedaction)
    .build()?;
```

Selection supports:

- entity IDs;
- recognizer IDs;
- locale and country;
- capabilities;
- default or explicitly enabled recognizers;
- allowlists and denylists; and
- consumer-provided policy.

The mutable legacy registry remains supported for compatibility but is not the target alpha construction model.

## Evidence and explainability

Recognition and policy decisions produce bounded evidence.

Evidence may identify:

- pattern ID and match method;
- structural or checksum validation;
- supporting or negative context token identity without raw matched text;
- prefilter or keyword decision;
- model identity, label, and raw probability;
- normalization or mapping applied;
- allowlist or denylist decision;
- threshold decision;
- limit decision;
- resolution decision; and
- evaluation receipt identity.

The library supports two report levels:

- **compact:** candidates or resolved findings plus essential status and issues;
- **diagnostic:** bounded decision evidence for tests, debugging, evaluation, and audits.

Diagnostic reports do not copy source plaintext by default. Plaintext diagnostic output, if ever supported, requires explicit caller opt-in and prominent warnings.

## Context and threshold policy

Context enhancement is separate from recognition mechanics.

The alpha context contract supports:

- positive context;
- negative context;
- token- or boundary-aware matching;
- configurable proximity;
- case behavior;
- locale behavior;
- optional cross-entity evidence; and
- evidence explaining every score change.

Context does not imply score calibration. Scores remain heuristic unless a pinned evaluation demonstrates calibration for a specific recognizer, corpus, and configuration.

Thresholding is an explicit policy stage and does not destroy original candidate evidence.

## Resolution

Resolution is a named policy stage, not an incidental sort.

The core provides at least:

- `ReportAll`: preserve all qualifying candidates;
- `BestCandidate`: select candidates through explicit deterministic priority rules;
- `ConservativeRedaction`: produce transformation-safe unions for overlapping qualifying spans; and
- a future custom resolver interface after the built-in contracts stabilize.

Every policy defines:

- full overlap;
- containment and nesting;
- partial intersection;
- adjacency;
- equal confidence;
- entity priority;
- recognizer priority;
- same-recognizer duplicates; and
- stable tie-breaking.

The report preserves original candidates separately from resolved findings and records the policy identity, version, and decision evidence.

Resolution must be stable before authoritative anonymization is implemented.

## Anonymization architecture

Anonymization is split into planning and execution.

```text
AnalysisReport or resolved findings
        |
        v
AnonymizationPolicy
        |
        v
AnonymizationPlan validation
        |
        v
Atomic transformation
        |
        v
AnonymizationReport
```

The authoritative API is fallible and document-bound.

```rust
pub fn anonymize(
    document: &TextDocument<'_>,
    findings: &[ResolvedFinding],
    policy: &AnonymizationPolicy,
) -> Result<AnonymizationReport, AnonymizationError>;
```

The complete plan validates before any output is returned.

Validation includes:

- exact document identity and fingerprint;
- UTF-8-safe source spans;
- resolved overlap requirements;
- supported entity and operator combinations;
- replacement and mask constraints;
- output-size bounds; and
- policy version and required capabilities.

A failure produces no successful transformed result. Partial transformation cannot be represented as complete success.

The report contains:

- transformed text;
- applied operations;
- source spans;
- output spans;
- policy and engine identity;
- warnings and typed issues;
- rejected operations when planning fails before execution; and
- document identity.

The legacy anonymizer remains available but is not the authoritative secure-alpha boundary.

## Operators and cryptography

The authoritative alpha supports:

- replacement;
- redaction; and
- masking with explicit validated semantics.

Deterministic salted SHA-256 remains legacy-compatible and security-sensitive. It is not an ordinary authoritative-alpha operator.

Before hashing or pseudonymization is enabled on the authoritative path, the project must choose reviewed semantics under issue #37. A keyed design requires:

- caller-provided secret material;
- secret wrappers without accidental `Debug` or serialization;
- domain separation;
- tenant or context isolation;
- output encoding and truncation policy;
- collision analysis;
- test vectors;
- rotation and migration behavior;
- explicit correlation and linkability semantics; and
- independent review where practical.

Reversible encryption is outside the core until key management, authenticated metadata, nonce handling, rotation, and consumer demand are designed.

## Evaluation and Presidio differential learning

Evaluation tooling is separable from the runtime dependency graph.

The evidence program includes:

- a versioned synthetic and redistributable corpus schema;
- exact-span and overlap-tolerant metrics;
- per-entity, recognizer, locale, country, and corpus-family analysis;
- false-positive and false-negative regressions;
- corpus provenance and licensing;
- machine-readable receipts;
- historical Presidio failure fixtures; and
- classification of Rust and Python differences.

Python Presidio is a comparison source, not the expected output oracle.

Differences are classified as:

- intentional Rust safety improvement;
- intentional scope difference;
- Python behavior worth matching;
- Rust defect;
- upstream defect or disputed behavior;
- taxonomy mismatch; or
- unresolved evidence gap.

Synthetic template families are split across training and evaluation boundaries as families rather than only as generated rows to reduce leakage.

No quality, compatibility, or superiority claim is made without pinned engine, recognizer, policy, corpus, configuration, metric, and reproduction identity.

## Fuzzing and property testing

Initial secure-alpha targets cover:

- span construction and UTF-8 boundaries;
- request construction and limits;
- candidate emission;
- context and threshold policy;
- resolution;
- anonymization planning and execution;
- source-to-output mapping; and
- report serialization when enabled.

Minimized failures become retained regression fixtures.

Fuzzing supplements, but does not replace, explicit historical and semantic test cases.

## Configuration

Programmatic typed construction remains primary through the secure functional alpha.

Serialized configuration is deferred until recognition, context, threshold, resolution, operator, and report contracts stabilize.

A future serialized format requires:

- explicit schema version;
- stable identifiers;
- validation without panics;
- unknown-field behavior;
- migration guidance;
- secure defaults;
- configuration fingerprints; and
- compatibility fixtures.

Configuration must not become an undocumented second public API.

## Crate boundaries

The architecture keeps one primary crate while the core contracts are moving.

A separate crate is justified when it:

- introduces a materially heavier dependency graph;
- has a distinct release, licensing, or platform lifecycle;
- is independently consumable;
- requires a different MSRV or support matrix; or
- prevents default consumers from paying build, binary, memory, startup, or audit cost.

Likely future boundaries include:

- evaluation and error-analysis tooling;
- backend-named semantic adapters;
- optional CLI tooling after the library contract stabilizes; and
- consumer-specific services or application adapters outside the core.

The project does not create microcrates for architectural appearance.

## Consumer integration surfaces

### Primary Rust API

The supported alpha integration is an in-process synchronous Rust library.

The synchronous core remains runtime-neutral and can be called from asynchronous applications through consumer-owned bounded worker strategies.

### Batch

Batch processing is deferred until the single-document contract is stable. A batch design must preserve per-document identity, limits, issues, and receipts.

### Streaming

Streaming redaction is deferred because sensitive values may cross arbitrary chunk boundaries.

Any future design requires bounded lookbehind, commit watermarks, maximum-span assumptions, final-buffer semantics, chunk-boundary permutation tests, and documented degraded guarantees.

### WASM

WASM remains a feasibility target requiring a dedicated support matrix, performance evidence, and explicit cryptographic behavior.

### FFI and language bindings

C, Python, Node, and other bindings follow Rust API stabilization. Bindings multiply compatibility and security surfaces and do not drive the core prematurely.

### `no_std`

`no_std` remains deferred without a demonstrated consumer requirement.

## Semantic adapters

Semantic recognition is optional and outside the secure functional alpha critical path.

A semantic adapter owns:

- tokenizer and model loading;
- model artifact provenance and licensing;
- label mapping;
- exact source-offset alignment;
- windowing and truncation;
- device selection;
- inference errors;
- resource limits; and
- model-specific evaluation.

The core does not download models, own global model caches, select devices, or require an async runtime.

A semantic backend is adopted only when evaluation demonstrates that its quality gain justifies dependency, artifact-size, startup, memory, platform, and maintenance costs.

## Compatibility policy

Compatibility surfaces include:

- public Rust API;
- feature names and defaults;
- entity and recognizer IDs;
- policy IDs and versions;
- serialized findings and reports;
- byte-offset semantics;
- score, context, and threshold semantics;
- resolution behavior;
- anonymization output; and
- error classifications.

Required controls include:

- downstream compile fixtures;
- `cargo-semver-checks` after a versioned baseline exists;
- deprecation before removal where practical;
- migration notes;
- feature-matrix CI; and
- downstream pilot tests before semantic changes are released.

Behavioral compatibility with Microsoft Presidio is claimed only for pinned fixtures where it has been demonstrated.

## Security architecture

The default core:

- initiates no network or filesystem I/O;
- forbids crate-local `unsafe` code;
- bounds input, candidates, issues, evidence, backend work, and output;
- validates source identity, offsets, resolution, and transformations;
- exposes explicit incomplete and failure states;
- minimizes plaintext duplication;
- keeps diagnostics plaintext-free by default;
- maintains dependency, license, provenance, history, DCO, MSRV, and package gates;
- uses fuzzing, adversarial tests, and retained regressions;
- reviews corpus provenance and privacy; and
- documents caller responsibilities.

Rust memory safety does not prove detection correctness, cryptographic correctness, availability, or safe consumer behavior.

The secure-alpha threat model must cover:

- wrong-document application;
- Unicode and offset confusion;
- overlap and partial-intersection ambiguity;
- partial transformation;
- false confidence from weak defaults;
- diagnostic data leakage;
- algorithmic denial of service;
- unbounded custom backends;
- linkable or brute-forceable pseudonyms;
- configuration drift;
- dependency or release compromise; and
- consumer fail-open behavior.

## Observability

The library returns structured reports and does not emit sensitive plaintext logs by default.

Optional tracing may emit:

- recognizer ID and version;
- policy ID and version;
- duration;
- input length;
- candidate and resolved counts;
- operation count;
- limit status; and
- error class.

Tracing must not emit matched values or full input unless a caller explicitly enables a diagnostic mode with appropriate warnings and controls.

## Architecture governance

Architecture is developed continuously and evidence-first.

Required cadence:

- primary-source Presidio archaeology before material alpha decisions;
- a decision ledger entry for adopt, adapt, reject, defer, or investigate outcomes;
- ADRs for public API, dependency, normalization, context, resolution, cryptography, configuration, serialization, and compatibility decisions;
- weekly architecture review during active alpha work;
- a phase-exit review before each roadmap gate;
- consumer compatibility review at least every four weeks;
- active Rust landscape monitoring; and
- explicit validation or reversal of assumptions after evaluation and pilots.

## Roadmap and issue mapping

- #33: Presidio architecture archaeology and decision ledger
- #34: secure functional alpha contract
- #35: explainability, context evidence, and safe defaults
- #36: differential learning and historical regression harness
- #37: authoritative hash and pseudonymization decision
- #14: secure functional alpha delivery
- #15: two-consumer and compatibility validation

## Open architectural questions

1. Final public project and package name before crates.io publication.
2. Whether another Rust project should be adopted, extended, or collaborated with.
3. Final immutable builder and registry shape.
4. Exact context policy representation and calibration evidence.
5. Whether phone recognition should use a mature parsing crate.
6. Whether keyed pseudonymization belongs in the core or a separate cryptographic adapter.
7. Which locales and countries qualify for the supported alpha quality envelope.
8. What evidence threshold qualifies a recognizer for default enablement beyond the minimum policy in this document.
9. Whether report serialization belongs in the core and when it becomes a durable wire contract.
10. Which semantic backend, if any, is justified after the model-free alpha.
11. Whether WASM becomes a real consumer requirement.
12. What consumer evidence is sufficient to freeze a beta API baseline.

These remain scheduled decisions rather than hidden implementation assumptions.
