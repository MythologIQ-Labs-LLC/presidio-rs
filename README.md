<div align="center">

# presidio-rs

**Offline, Rust-native PII detection and anonymization**

A small, embeddable library for identifying and transforming structured sensitive data without requiring Python, a network service, or runtime model downloads.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.74%2B-orange.svg)](Cargo.toml)
[![CI](https://github.com/MythologIQ-Labs-LLC/presidio-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/MythologIQ-Labs-LLC/presidio-rs/actions/workflows/ci.yml)
[![unsafe forbidden](https://img.shields.io/badge/crate--local%20unsafe-forbidden-success.svg)](src/lib.rs)

**Project status: early-stage, privately developed, and not production certified**

</div>

> [!IMPORTANT]
> `presidio-rs` is an independently governed Rust project being developed privately with open-source-grade practices. It is not affiliated with, sponsored by, or endorsed by Microsoft. Microsoft Presidio is acknowledged as a design reference under its MIT license.

## Current capabilities

`presidio-rs` analyzes UTF-8 text for supported categories of personally identifiable information and secrets. The current implementation provides:

- compiled regular-expression recognizers
- optional checksum validators
- nearby context-word scoring
- bounded, evidence-bearing findings
- candidate-preserving analysis reports
- authoritative recognizer IDs, versions, locales, mechanisms, capabilities, and attribution
- exact source binding through document IDs, byte lengths, and SHA-256 fingerprints
- request-time entity, recognizer, locale, capability, input, candidate, and issue controls
- a backend-neutral `Recognizer` trait for consumer-supplied structural, dictionary, or semantic backends
- validated candidate emission that enforces entity declarations, UTF-8 spans, confidence bounds, provenance, document binding, and resource limits
- legacy replacement, redaction, masking, and deterministic hashing

The crate performs no network or filesystem I/O itself. Its current direct dependencies are `regex`, `sha2`, and optional `serde`.

## Two API paths

The library currently preserves two additive API paths while consumers migrate.

### Legacy-compatible path

Use this path for existing `PatternRecognizer`, `RecognizerResult`, and `AnonymizerEngine` integrations.

```rust
use presidio::{AnalyzerEngine, AnonymizerEngine, Operator};

let analyzer = AnalyzerEngine::new();
let text = "Email jane@acme.com about card 4111 1111 1111 1111.";

let findings = analyzer.analyze(text, None);
let clean = AnonymizerEngine::new(Operator::Replace(None))
    .anonymize(text, &findings);

assert_eq!(
    clean,
    "Email <EMAIL_ADDRESS> about card <CREDIT_CARD>."
);
```

This path remains source compatible. It uses the closed `EntityType` taxonomy and current overlap and threshold behavior.

### Authoritative request path

Use this path for exact document binding, open entity identifiers, recognizer provenance, typed backend failures, deterministic request selection, and bounded reports.

```rust
use presidio::{AnalysisRequest, AnalyzerEngine, DocumentId, TextDocument};

let document = TextDocument::new(
    DocumentId::new("request-42").expect("valid document ID"),
    "Email jane@acme.com",
);
let request = AnalysisRequest::new();

let report = AnalyzerEngine::new()
    .analyze_request(&document, &request)
    .expect("bounded analysis");

report
    .validate_for_document(&document)
    .expect("matching source document");

let email = report
    .candidates()
    .iter()
    .find(|finding| finding.entity().as_str() == "EMAIL_ADDRESS")
    .expect("email candidate");

assert_eq!(
    email.slice_document(&document).expect("bound source slice"),
    "jane@acme.com"
);
```

`AnalysisReport::candidates()` is authoritative for the request path. `legacy_compatible_results()` is only a compatibility projection. When an open entity cannot be represented by `EntityType`, `report.status().legacy_projection_incomplete()` is true.

## Expected use cases

Good fits include:

- sanitizing application logs and telemetry before persistence
- inspecting prompts, model responses, or tool output before release
- redacting structured identifiers in local or offline processing pipelines
- embedding PII controls in desktop, edge, sandboxed, or air-gapped software
- protecting command-line output and generated diagnostic bundles
- scanning text fields before they cross a service or plugin boundary
- adding organization-specific identifiers through strict pattern recognizers
- integrating an optional local semantic backend without making it a default crate dependency
- preserving recognizer provenance and backend failures for consumer policy decisions

The library is especially useful when introducing a Python runtime, HTTP sidecar, or model-serving process would be disproportionate to the required deployment scope.

## What it is not

`presidio-rs` is not currently:

- a complete reimplementation or drop-in replacement for Microsoft Presidio
- a guarantee that all sensitive information will be detected
- a built-in NLP or named-entity recognition system
- able by default to reliably detect arbitrary person names or prose locations
- an OCR, image, DICOM, audio, or video redactor
- a structured or tabular de-identification framework
- a hosted API or network service
- a substitute for data classification, threat modeling, access control, or human review
- evidence by itself that an application satisfies a legal or regulatory obligation
- a stable serialized request or report protocol

Detection systems produce false positives and false negatives. Applications must choose coverage, thresholds, failure behavior, retention, and review requirements appropriate to their risks.

## Installation

While the repository remains private, authorized consumers should use a Git dependency or workspace path.

```toml
[dependencies]
presidio-rs = { git = "https://github.com/MythologIQ-Labs-LLC/presidio-rs" }
```

```toml
[dependencies]
presidio-rs = { path = "../presidio-rs" }
```

The package name is `presidio-rs`; the Rust library is imported as `presidio`. Public publication and final naming remain separate future decisions.

## Request controls

`AnalysisRequest` supports:

- open entity allowlists
- explicit recognizer allowlists
- locale selection
- available capability declarations
- minimum confidence for the legacy-compatible projection
- maximum UTF-8 input bytes
- maximum candidates across selected recognizers
- maximum retained analysis issues

The default request selects recognizers whose metadata is marked default-enabled, accepts at most 1 MiB of source text, retains at most 10,000 candidates, and retains at most 100 issue details.

Legacy pattern registrations created through `RecognizerRegistry::add` have unknown provenance. They continue to run through legacy APIs but are deliberately skipped by `analyze_request`. The report records one typed `LegacyRecognizersSkipped` issue rather than fabricating identities or versions.

## Custom recognizer backends

Consumer backends implement the object-safe `Recognizer` trait and emit findings through `CandidateEmitter`.

```rust
use presidio::{
    AnalysisRequest, CandidateEmitter, RecognitionError, Recognizer,
    RecognizerMetadata, TextDocument,
};

struct LocalBackend {
    metadata: RecognizerMetadata,
}

impl Recognizer for LocalBackend {
    fn metadata(&self) -> &RecognizerMetadata {
        &self.metadata
    }

    fn recognize(
        &self,
        document: &TextDocument<'_>,
        _request: &AnalysisRequest,
        emitter: &mut CandidateEmitter<'_, '_>,
    ) -> Result<(), RecognitionError> {
        // Run the local backend, map every result to original UTF-8 byte
        // coordinates, and call emitter.emit(...).
        let _ = (document, emitter);
        Ok(())
    }
}
```

The emitter rejects undeclared entities, invalid UTF-8 spans, non-finite or out-of-range confidence values, and emissions beyond the remaining global candidate capacity. It attaches authoritative recognizer identity and exact document binding to accepted findings.

Backends return bounded, non-plaintext `RecognitionError` values with stable categories, stable codes, and retryability. Consumer applications decide whether those issues should fail open, fail closed, block, retry, or require human review.

## Supported built-in entities

| Entity | Output tag | Detection method | Validator |
|---|---|---|---|
| Credit card | `CREDIT_CARD` | Pattern | Luhn |
| US Social Security number | `US_SSN` | Pattern | None |
| Email address | `EMAIL_ADDRESS` | Pattern | None |
| Phone number | `PHONE_NUMBER` | Pattern | None |
| IP address | `IP_ADDRESS` | IPv4 and IPv6 patterns | None |
| MAC address | `MAC_ADDRESS` | Pattern | None |
| IBAN | `IBAN_CODE` | Pattern | Mod-97 |
| Cryptocurrency wallet | `CRYPTO` | Selected Bitcoin and Ethereum patterns | None |
| URL | `URL` | Pattern | None |
| US ITIN | `US_ITIN` | Pattern | None |
| API key | `API_KEY` | Selected vendor formats | None |
| Person | `PERSON` | Reserved for an optional semantic backend | Not emitted by built-ins |
| Location | `LOCATION` | Reserved for an optional semantic backend | Not emitted by built-ins |
| NRP | `NRP` | Reserved for an optional semantic backend | Not emitted by built-ins |

A listed entity does not imply coverage of every country, vendor, formatting variation, or malformed representation.

## Anonymization operators

| Operator | Behavior |
|---|---|
| `Replace(None)` | Replaces a span with its entity tag, such as `<EMAIL_ADDRESS>` |
| `Replace(Some(value))` | Replaces a span with a caller-provided value |
| `Redact` | Removes the span |
| `Mask` | Masks characters while optionally preserving the final characters |
| `Hash` | Produces a deterministic salted SHA-256 token |

The current anonymization API consumes legacy `RecognizerResult` values. Fallible anonymization over document-bound findings and an explicit permanent resolution policy are the next architectural slice.

The `Hash` operator enables deterministic correlation. It does not guarantee irreversible anonymity. Low-entropy values can remain guessable, particularly if a salt is disclosed or candidate values can be tested.

## Architecture

```text
Legacy path
-----------
&str
  -> RecognizerRegistry
  -> pattern matching, validators, and context scoring
  -> overlap and threshold policy
  -> Vec<RecognizerResult>
  -> AnonymizerEngine

Authoritative request path
--------------------------
TextDocument + AnalysisRequest
  -> strict metadata-backed pattern adapters
  -> optional Arc<dyn Recognizer> backends
  -> CandidateEmitter invariant enforcement
  -> AnalysisReport
       -> source-bound candidate findings
       -> recognizer metadata catalog
       -> typed issues and truncation status
       -> legacy-compatible projection
```

Current architecture decisions:

- [Target architecture](docs/architecture/ARCHITECTURE.md)
- [ADR 0002: Backend-neutral core and optional adapters](docs/adr/0002-backend-neutral-core-and-optional-adapters.md)
- [ADR 0003: Validated core types](docs/adr/0003-stage-core-types-before-engine-migration.md)
- [ADR 0004: Candidate-preserving reports](docs/adr/0004-add-candidate-preserving-analysis-report.md)
- [ADR 0005: Recognizer metadata](docs/adr/0005-add-backend-neutral-recognizer-metadata.md)
- [ADR 0006: Exact document binding](docs/adr/0006-bind-findings-to-text-documents.md)
- [ADR 0007: Requests and recognizer execution](docs/adr/0007-add-analysis-request-and-recognizer-trait.md)
- [Project documentation index](docs/README.md)

## Security and privacy boundaries

The crate itself:

- performs no network or filesystem I/O
- does not download patterns or models
- forbids crate-local `unsafe` code with `#![forbid(unsafe_code)]`
- borrows raw source text rather than copying it into reports
- omits source plaintext from `TextDocument` debug output
- uses bounded identifier-shaped evidence and failure codes
- binds request-oriented findings to exact source identity and content

A document fingerprint is an integrity mechanism, not encryption or anonymization. Bindings may be sensitive metadata, especially for low-entropy or guessable content. Use opaque document IDs and appropriate retention and access controls.

Before using the crate at a security boundary:

- define required entity and locale coverage
- test representative, malformed, and adversarial inputs
- determine acceptable false-negative and false-positive rates
- choose explicit confidence and failure policies
- decide whether backend issues fail open or fail closed
- validate Unicode and original-coordinate behavior
- define handling for unsupported or unrecognized semantic PII
- monitor regressions when recognizers or dependencies change

Report suspected vulnerabilities through GitHub private vulnerability reporting. Do not disclose security-sensitive findings in a public issue. See [SECURITY.md](SECURITY.md).

## Development plan

The active planning baseline is a 30-week private program from August 3, 2026 through February 26, 2027. Architecture review, consumer feedback, risk review, and parallel-project research continue throughout every phase.

- [Multi-phase development plan](docs/planning/DEVELOPMENT_PLAN.md)
- [Development risk and assumption register](docs/planning/RISK_REGISTER.md)
- [Active Rust privacy landscape watch](docs/research/ACTIVE_LANDSCAPE_WATCH.md)
- [Parallel efforts and architectural lessons](docs/research/PARALLEL_EFFORTS_AND_LESSONS.md)

The build-versus-adopt-versus-collaborate decision remains active. Sunk cost is not a reason to duplicate a better maintained Rust implementation.

## Development

```bash
cargo build --all-features
cargo test --all-features
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --no-deps --all-features
cargo audit
```

The minimum supported Rust version is declared in `Cargo.toml` and verified in CI.

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting changes. Contributions require:

- acceptance of the [Contributor License Agreement](CONTRIBUTOR_LICENSE_AGREEMENT.md)
- DCO sign-off on every commit
- appropriate tests and documentation
- attribution for adapted algorithms, datasets, APIs, or prior art
- a responsible human contributor who can explain and defend the change

## Claims policy

Project documentation distinguishes among:

- **implemented:** visible in source and tests
- **measured:** supported by a reproducible benchmark or evaluation artifact
- **planned:** proposed but not implemented

Performance, accuracy, security, compliance, and cost claims must not be presented as measured facts without reproducible evidence.

## License and acknowledgements

Copyright (c) 2026 MythologIQ Labs LLC and contributors.

Licensed under the [MIT License](LICENSE).

The analyzer and anonymizer concepts are informed by [Microsoft Presidio](https://github.com/microsoft/presidio), which is also distributed under the MIT License. No Microsoft Presidio source code is linked, vendored, or redistributed by this crate.

Microsoft and Microsoft Presidio remain the property of their respective owners. Their names are used only to identify the referenced project.
