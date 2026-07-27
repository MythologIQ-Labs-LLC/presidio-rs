<div align="center">

# presidio-rs

**Offline, Rust-native PII detection and anonymization**

A small, embeddable library for identifying and transforming structured sensitive data without requiring Python, a network service, or runtime model downloads.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.74%2B-orange.svg)](Cargo.toml)
[![CI](https://github.com/MythologIQ-Labs-LLC/presidio-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/MythologIQ-Labs-LLC/presidio-rs/actions/workflows/ci.yml)
[![unsafe forbidden](https://img.shields.io/badge/crate--local%20unsafe-forbidden-success.svg)](src/lib.rs)

**Project status: early-stage and not yet production certified**

</div>

> [!IMPORTANT]
> `presidio-rs` is an independent open-source project. It is not affiliated with, sponsored by, or endorsed by Microsoft. Microsoft Presidio is acknowledged as a design reference under its MIT license.

## What it does

`presidio-rs` analyzes UTF-8 text for supported categories of personally identifiable information and secrets, returns scored byte spans, and can replace, redact, mask, or deterministically hash those spans.

The current engine is model-free. It combines:

- compiled regular-expression recognizers
- optional checksum validators
- nearby context-word scoring
- configurable score thresholds
- overlap resolution
- per-entity anonymization operators
- custom recognizer and validator registration

The library performs no network or filesystem I/O. Its current direct dependencies are `regex`, `sha2`, and optional `serde`.

```rust
use presidio::{AnalyzerEngine, AnonymizerEngine, EntityType, Operator};

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

## Expected use cases

`presidio-rs` is intended for applications that need a small, local, Rust-native detection layer for structured PII and secrets.

Good fits include:

- sanitizing application logs and telemetry before persistence
- inspecting prompts, model responses, or tool output before release
- redacting structured identifiers in local or offline processing pipelines
- embedding PII controls in desktop, edge, sandboxed, or air-gapped software
- protecting command-line output and generated diagnostic bundles
- scanning text fields before they cross a service or plugin boundary
- adding organization-specific identifiers through custom recognizers
- applying deterministic masking or replacement policies by entity type

The library is especially useful when introducing a Python runtime, HTTP sidecar, or model-serving process would be disproportionate to the required pattern-based detection scope.

## What it is not

`presidio-rs` is not currently:

- a complete reimplementation or drop-in replacement for Microsoft Presidio
- a guarantee that all sensitive information will be detected
- an NLP or named-entity recognition system
- able to reliably detect arbitrary person names or prose locations
- an OCR, image, DICOM, audio, or video redactor
- a structured or tabular de-identification framework
- a hosted API or network service
- a substitute for data classification, threat modeling, access control, or human review
- evidence by itself that an application satisfies a legal or regulatory obligation

Detection systems produce false positives and false negatives. Applications must choose thresholds, entity coverage, failure behavior, and review requirements appropriate to their own risks.

## Why a Rust-native implementation

Microsoft Presidio is mature and appropriate for many Python and NLP-heavy environments. `presidio-rs` addresses a narrower deployment need: direct embedding of model-free detection and anonymization into Rust software.

The current architecture can provide:

- no Python interpreter or language bridge
- no local HTTP service or sidecar requirement
- no runtime model download
- a small direct dependency set
- typed Rust integration
- deterministic behavior for fixed input and configuration
- direct control over recognizers and anonymization policy

These are architectural properties, not benchmark results. The project does not currently claim superior speed, memory use, accuracy, safety, or operating cost compared with Microsoft Presidio or other tools.

## Installation

The crate metadata is ready for publication, but until the first crates.io release, use the Git repository:

```toml
[dependencies]
presidio-rs = { git = "https://github.com/MythologIQ-Labs-LLC/presidio-rs" }
```

In a workspace:

```toml
[dependencies]
presidio-rs = { path = "../presidio-rs" }
```

The package name is `presidio-rs`; the Rust library is imported as `presidio`.

## Quick start

### Analyze selected entities

```rust
use presidio::{AnalyzerEngine, EntityType};

let analyzer = AnalyzerEngine::new();
let text = "Contact jane@example.com and use token ghp_012345678901234567890123456789012345.";

let findings = analyzer.analyze(
    text,
    Some(&[EntityType::Email, EntityType::ApiKey]),
);

for finding in findings {
    println!(
        "{} [{}..{}] score={:.2}",
        finding.entity_type,
        finding.start,
        finding.end,
        finding.score
    );
}
```

### Apply different policies by entity

```rust
use presidio::{AnalyzerEngine, AnonymizerEngine, EntityType, Operator};

let analyzer = AnalyzerEngine::new();
let text = "jane@example.com paid with card 4111 1111 1111 1111";
let findings = analyzer.analyze(text, None);

let anonymizer = AnonymizerEngine::new(Operator::Redact)
    .with_operator(
        EntityType::Email,
        Operator::Replace(Some("[email]".into())),
    )
    .with_operator(
        EntityType::CreditCard,
        Operator::Mask {
            mask_char: '*',
            keep_last: 4,
        },
    );

let output = anonymizer.anonymize(text, &findings);
```

### Register a custom recognizer

```rust
use presidio::{
    AnalyzerEngine, EntityType, Pattern, PatternRecognizer, RecognizerRegistry,
};

let mut registry = RecognizerRegistry::empty();
registry.add(PatternRecognizer {
    entity_type: EntityType::ApiKey,
    patterns: vec![Pattern::new(
        "acme-key",
        r"\bACME-[A-Z0-9]{8}\b",
        0.9,
    )],
    context: &["key", "token"],
    validator: None,
});

let analyzer = AnalyzerEngine::with_registry(registry);
let findings = analyzer.analyze("token ACME-12ABCD34", None);
```

## Supported entities

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
| Person | `PERSON` | Reserved for future semantic backend | Not emitted |
| Location | `LOCATION` | Reserved for future semantic backend | Not emitted |
| NRP | `NRP` | Reserved for future semantic backend | Not emitted |

A listed entity does not imply coverage of every country, vendor, formatting variation, or malformed representation.

## Anonymization operators

| Operator | Behavior |
|---|---|
| `Replace(None)` | Replaces a span with its entity tag, such as `<EMAIL_ADDRESS>` |
| `Replace(Some(value))` | Replaces a span with a caller-provided value |
| `Redact` | Removes the span |
| `Mask` | Masks characters while optionally preserving the final characters |
| `Hash` | Produces a deterministic salted SHA-256 token |

The current `Hash` operator enables deterministic correlation. It does not guarantee irreversible anonymity. Low-entropy values can remain guessable, especially if the salt is disclosed or candidate values can be tested. Applications are responsible for key or salt management, tenant separation, rotation, domain separation, retention, and correlation policy.

## Architecture

The project follows a two-stage model:

```text
text
  -> AnalyzerEngine
       -> RecognizerRegistry
       -> pattern matching
       -> optional checksum validation
       -> context score adjustment
       -> overlap resolution
       -> threshold filtering
  -> Vec<RecognizerResult>
  -> AnonymizerEngine
       -> per-entity operator selection
       -> right-to-left span transformation
  -> transformed text
```

The public concepts are informed by Microsoft Presidio's analyzer and anonymizer architecture, but this implementation is independent and does not claim complete behavioral compatibility.

## Security and privacy boundaries

The crate itself:

- performs no network or filesystem I/O
- does not download patterns or models
- forbids crate-local `unsafe` code with `#![forbid(unsafe_code)]`
- returns findings and transformed strings to the caller

These properties do not control what the calling application logs, stores, transmits, caches, or includes in crash dumps. They also do not establish that transitive dependencies contain no unsafe code.

Before using the crate at a security boundary:

- define required entity coverage
- test representative and adversarial inputs
- determine acceptable false-negative and false-positive rates
- choose explicit score thresholds
- decide whether detector errors fail open or fail closed
- validate Unicode and span-handling behavior
- define handling for unrecognized semantic PII
- monitor regressions when recognizers change

Please report suspected vulnerabilities through GitHub private vulnerability reporting. Do not disclose security-sensitive findings in a public issue. See [SECURITY.md](SECURITY.md).

## Current limitations and roadmap

The project has not yet published comparative performance or quality evidence. Planned work includes:

1. an offline span-level precision, recall, and F1 evaluation harness
2. false-positive and false-negative regression corpora
3. additional international and checksum-backed recognizers
4. fuzzing, malformed Unicode tests, and adversarial long-input tests
5. latency, throughput, memory, and artifact-size benchmarks
6. stronger fallible anonymization and span validation APIs
7. explicit finding provenance and conflict-resolution policy
8. evaluation of an optional offline semantic recognizer backend

A semantic backend is roadmap work, not a current capability or committed model choice.

## Development

```bash
cargo build --all-features
cargo test --all-features
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --no-deps --all-features
```

The minimum supported Rust version is declared in `Cargo.toml`.

## Contributing

Contributions are welcome, including:

- recognizers and validators with evidence-backed test cases
- false-positive and false-negative fixtures
- international format support
- API and documentation improvements
- fuzzing and property tests
- evaluation and benchmarking infrastructure
- security and correctness fixes

All contributions require:

- acceptance of the [Contributor License Agreement](CONTRIBUTOR_LICENSE_AGREEMENT.md)
- DCO sign-off on every commit
- appropriate tests and documentation
- attribution for adapted algorithms, datasets, APIs, or prior art
- a responsible human contributor who can explain and defend the change

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

## Claims policy

Project documentation and release notes distinguish among:

- **implemented:** visible in source and tests
- **measured:** supported by a reproducible benchmark or evaluation artifact
- **planned:** proposed but not implemented

Performance, accuracy, security, compliance, and cost claims must not be presented as measured facts without reproducible evidence.

## License and acknowledgements

Copyright (c) 2026 MythologIQ Labs LLC and contributors.

Licensed under the [MIT License](LICENSE).

The analyzer and anonymizer concepts are informed by [Microsoft Presidio](https://github.com/microsoft/presidio), which is also distributed under the MIT License. No Microsoft Presidio source code is linked, vendored, or redistributed by this crate.

Microsoft and Microsoft Presidio remain the property of their respective owners. Their names are used only to identify the referenced project.
