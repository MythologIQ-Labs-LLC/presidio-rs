<div align="center">

# presidio-rs

**Offline, Rust-native PII detection and anonymization**

An embeddable library for identifying and transforming structured sensitive data without requiring Python, a network service, or runtime model downloads.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.74%2B-orange.svg)](Cargo.toml)
[![CI](https://github.com/MythologIQ-Labs-LLC/presidio-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/MythologIQ-Labs-LLC/presidio-rs/actions/workflows/ci.yml)
[![unsafe forbidden](https://img.shields.io/badge/crate--local%20unsafe-forbidden-success.svg)](src/lib.rs)

**Early-stage open-source foundation, not production certified, and not yet published to crates.io**

</div>

> [!IMPORTANT]
> `presidio-rs` is independently governed and is not affiliated with, sponsored by, or endorsed by Microsoft. Microsoft Presidio is acknowledged as an architectural reference under its MIT license. This project is incomplete, may change before `1.0`, and does not guarantee detection of all sensitive information.

## Release status

The repository is targeted to become publicly readable on **Thursday, July 30, 2026**.

Public repository visibility is separate from:

- crates.io publication;
- an advertised launch;
- a public beta or stable release;
- production certification; and
- guaranteed maintainer response times.

The project may remain quietly public while correctness, evaluation, fuzzing, compatibility, and consumer validation continue.

- [Public repository release week](docs/planning/PUBLIC_RELEASE_WEEK.md)
- [Rebaselined roadmap](docs/planning/DEVELOPMENT_PLAN.md)
- [Release checklist](OPEN_SOURCE_RELEASE_CHECKLIST.md)

## What is implemented

The current crate provides:

- regex-based recognizers for structured PII and selected secrets;
- optional checksum validators and nearby context scoring;
- bounded open entity, recognizer, document, and metadata identifiers;
- UTF-8-safe spans and finite confidence values;
- evidence-bearing findings and candidate-preserving reports;
- authoritative recognizer IDs, versions, locales, capabilities, mechanisms, and attribution;
- exact source binding through document ID, byte length, and SHA-256 fingerprint;
- bounded request selection for entities, recognizers, locale, capabilities, confidence, input size, candidates, and issues;
- an object-safe backend-neutral `Recognizer` trait;
- validated candidate emission that enforces source, provenance, entity, confidence, and resource invariants;
- typed non-plaintext backend failures; and
- legacy replacement, redaction, masking, and deterministic hashing.

The crate itself performs no network or filesystem I/O. Its direct dependencies are `regex`, `sha2`, and optional `serde`.

## API paths

The crate preserves a legacy-compatible path while introducing an authoritative request-oriented path for new Rust consumers.

### Legacy-compatible analysis and anonymization

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

This path retains `PatternRecognizer`, `RecognizerResult`, `EntityType`, the existing overlap policy, and the current anonymization API.

### Authoritative document-aware analysis

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

`AnalysisReport::candidates()` is authoritative for this path. `legacy_compatible_results()` is a compatibility projection. When an open entity cannot be represented by the legacy taxonomy, `report.status().legacy_projection_incomplete()` is true.

Migration and compatibility references:

- [Public API status](docs/api/PUBLIC_API_STATUS.md)
- [Legacy-to-request migration guide](docs/api/MIGRATION_GUIDE.md)

## Expected use cases

Good fits include:

- sanitizing logs and telemetry before persistence;
- inspecting prompts, model responses, or tool output before release;
- redacting structured identifiers in local or offline pipelines;
- embedding PII controls in desktop, edge, sandboxed, or air-gapped software;
- protecting command-line output and diagnostic bundles;
- scanning text before it crosses a service or plugin boundary;
- adding organization-specific identifiers through strict pattern recognizers;
- integrating optional local semantic or dictionary backends; and
- preserving recognizer provenance and backend failures for product policy decisions.

## What it is not

`presidio-rs` is not currently:

- a complete reimplementation or drop-in replacement for Microsoft Presidio;
- a guarantee that all sensitive information will be detected;
- a built-in NLP or named-entity recognition system;
- able by default to reliably detect arbitrary names or prose locations;
- an OCR, image, DICOM, audio, or video redactor;
- a structured or tabular de-identification framework;
- a hosted service;
- a substitute for threat modeling, access control, review, or data governance;
- proof of compliance with any legal or regulatory obligation;
- a stable serialized request or report protocol;
- a production-certified security boundary; or
- a published crates.io package.

Detection systems produce false positives and false negatives. Consumers own coverage, thresholds, retention, failure behavior, and review policy.

## Installation

The crate is not yet published to crates.io.

Use a Git dependency when repository access is available:

```toml
[dependencies]
presidio-rs = { git = "https://github.com/MythologIQ-Labs-LLC/presidio-rs" }
```

Or use a workspace path:

```toml
[dependencies]
presidio-rs = { path = "../presidio-rs" }
```

The package is named `presidio-rs`; Rust code imports it as `presidio`. Package publication remains a separate decision targeted for review after the public alpha and beta gates.

## Request controls

`AnalysisRequest` supports:

- open entity allowlists;
- explicit recognizer allowlists;
- locale selection;
- available capability declarations;
- minimum confidence for the legacy-compatible projection;
- maximum UTF-8 input bytes;
- maximum candidates across selected recognizers; and
- maximum retained issue details.

The default request selects metadata marked default-enabled, accepts at most 1 MiB of source text, retains at most 10,000 candidates, and retains at most 100 issue details.

Legacy pattern registrations made through `RecognizerRegistry::add` have unknown provenance. They continue to run through legacy APIs but are skipped by `analyze_request`, which records a typed `LegacyRecognizersSkipped` issue instead of inventing metadata.

## Custom backends

Consumer backends implement `Recognizer` and emit candidates through `CandidateEmitter`.

Runnable extension references:

- [Strict metadata-backed pattern recognizer](examples/strict_pattern_recognizer.rs)
- [Backend-neutral custom recognizer](examples/custom_backend.rs)

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
        // Preserve original UTF-8 byte coordinates and submit results
        // through emitter.emit(...).
        let _ = (document, emitter);
        Ok(())
    }
}
```

The emitter rejects undeclared entities, invalid UTF-8 spans, invalid confidence values, and candidates beyond the remaining request limit. Accepted findings receive authoritative recognizer identity and exact document binding.

Backend errors use stable categories, bounded codes, and retryability. Applications decide whether those issues block, retry, fail open, fail closed, or require review.

## Built-in entity coverage

| Entity | Tag | Method | Validator |
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
| Person | `PERSON` | Optional backend only | Not emitted by built-ins |
| Location | `LOCATION` | Optional backend only | Not emitted by built-ins |
| NRP | `NRP` | Optional backend only | Not emitted by built-ins |

A listed entity does not imply complete country, vendor, formatting, or malformed-input coverage.

## Anonymization status

The current anonymizer consumes legacy `RecognizerResult` values and supports:

- `Replace(None)` with an entity marker;
- `Replace(Some(value))`;
- `Redact`;
- `Mask`; and
- deterministic salted SHA-256 `Hash`.

Fallible anonymization over document-bound findings and an explicit permanent resolution policy are immediate post-visibility roadmap work.

Deterministic hashing enables correlation. It does not guarantee irreversible anonymity, especially for low-entropy values or disclosed salts.

## Architecture

```text
Legacy path
&str -> pattern registry -> legacy resolution -> Vec<RecognizerResult> -> anonymizer

Authoritative path
TextDocument + AnalysisRequest
  -> strict pattern adapters and optional Arc<dyn Recognizer> backends
  -> CandidateEmitter invariant enforcement
  -> AnalysisReport
       -> source-bound candidates
       -> recognizer metadata catalog
       -> typed issues and limit status
       -> legacy-compatible projection
```

Architecture documents:

- [Target architecture](docs/architecture/ARCHITECTURE.md)
- [ADR 0001: Open-source-grade posture](docs/adr/0001-private-open-source-posture.md)
- [ADR 0002: Backend-neutral core and optional adapters](docs/adr/0002-backend-neutral-core-and-optional-adapters.md)
- [ADR 0003: Validated core types](docs/adr/0003-stage-core-types-before-engine-migration.md)
- [ADR 0004: Candidate-preserving reports](docs/adr/0004-add-candidate-preserving-analysis-report.md)
- [ADR 0005: Recognizer metadata and validated registration](docs/adr/0005-add-recognizer-metadata-and-validated-registration.md)
- [ADR 0006: Exact document binding](docs/adr/0006-bind-findings-to-text-documents.md)
- [ADR 0007: Requests and recognizer execution](docs/adr/0007-add-analysis-request-and-recognizer-trait.md)
- [Documentation index](docs/README.md)

## Security and privacy boundaries

The crate:

- performs no network or filesystem I/O;
- does not download patterns or models;
- forbids crate-local `unsafe` code;
- borrows raw source text rather than copying it into reports;
- omits plaintext from `TextDocument` debug output;
- uses bounded identifier-shaped evidence and failure codes; and
- binds request-oriented findings to exact source identity and content.

A document fingerprint is an integrity mechanism, not encryption or anonymization. Bindings may be sensitive metadata, particularly for low-entropy content. Use opaque document IDs and appropriate retention and access controls.

Before deployment at a security boundary, define required coverage, test adversarial inputs, choose fail-open or fail-closed behavior, validate Unicode handling, and establish review and regression-monitoring policy.

Report vulnerabilities confidentially to **admin@mythologiq.studio**. See [SECURITY.md](SECURITY.md).

## Development program

The accelerated roadmap targets public repository visibility on July 30, contributor-ready alpha by August 14, consumer-ready beta by September 25, and a separate package and advertised-launch decision by October 30.

- [Public repository release week](docs/planning/PUBLIC_RELEASE_WEEK.md)
- [Rebaselined development roadmap](docs/planning/DEVELOPMENT_PLAN.md)
- [Open-source foundation track](docs/planning/OPEN_SOURCE_FOUNDATION_TRACK.md)
- [Risk and assumption register](docs/planning/RISK_REGISTER.md)
- [Active Rust privacy landscape watch](docs/research/ACTIVE_LANDSCAPE_WATCH.md)
- [Parallel efforts and lessons](docs/research/PARALLEL_EFFORTS_AND_LESSONS.md)

The build-versus-adopt-versus-collaborate decision remains active. Sunk cost is not a reason to duplicate a better maintained project.

## Development commands

```bash
cargo build --all-features
cargo test --all-features
cargo test --doc --all-features
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo run --example legacy_anonymization
cargo run --example document_analysis
cargo run --example strict_pattern_recognizer
cargo run --example custom_backend
cargo publish --dry-run
cargo audit
```

Rust 1.74 is the declared minimum and is verified in CI.

## Contributing

Start with the [first contribution guide](docs/contributing/FIRST_CONTRIBUTION.md), then read [CONTRIBUTING.md](CONTRIBUTING.md). Contributions require the [Contributor License Agreement](CONTRIBUTOR_LICENSE_AGREEMENT.md), DCO sign-off, tests, documentation, attribution, and a responsible human contributor who can explain the change.

## Claims policy

Documentation distinguishes among **implemented**, **measured**, and **planned** capabilities. Performance, accuracy, security, compliance, production-readiness, and cost claims require reproducible evidence.

## License and acknowledgements

Copyright (c) 2026 MythologIQ Labs LLC and contributors.

Licensed under the [MIT License](LICENSE).

The analyzer and anonymizer concepts are informed by [Microsoft Presidio](https://github.com/microsoft/presidio), also distributed under MIT. No Microsoft Presidio source code is linked, vendored, or redistributed by this crate.
