<div align="center">

# presidio-rs

**Offline, Rust-native PII detection and anonymization**
_A small, embeddable enforcement engine for Rust applications and constrained runtimes, informed by Microsoft Presidio's model-free architecture._

![status](https://img.shields.io/badge/status-internal-blue)
![rust](https://img.shields.io/badge/rust-2021%20edition-orange)
![offline](https://img.shields.io/badge/network-none-brightgreen)
![unsafe](https://img.shields.io/badge/crate--local%20unsafe-forbidden-success)
![clippy](https://img.shields.io/badge/clippy-%E2%88%92D%20warnings-success)
![license](https://img.shields.io/badge/license-proprietary-lightgrey)

**Internal — MythologIQ-Labs-LLC. Confidential. Not for public distribution.**

</div>

---

## Table of contents

- [What it is](#what-it-is)
- [Why it exists](#why-it-exists)
- [Value proposition](#value-proposition)
  - [Where it creates value](#where-it-creates-value)
  - [What is proven today](#what-is-proven-today)
  - [What is not yet proven](#what-is-not-yet-proven)
  - [When to use upstream Presidio instead](#when-to-use-upstream-presidio-instead)
- [Highlights](#highlights)
- [Quick start](#quick-start)
- [Architecture](#architecture)
  - [Component correspondence](#component-correspondence)
  - [Detection pipeline](#detection-pipeline)
  - [The scoring model](#the-scoring-model)
- [Supported entities](#supported-entities)
- [Anonymization operators](#anonymization-operators)
- [Extending the engine](#extending-the-engine)
- [The NER gap and roadmap](#the-ner-gap-and-roadmap)
- [Design principles and non-goals](#design-principles-and-non-goals)
- [Relationship to GG-CORE and COREFORGE](#relationship-to-gg-core-and-coreforge)
- [Project layout](#project-layout)
- [Development](#development)
- [Security and privacy](#security-and-privacy)
- [Claims policy](#claims-policy)
- [License and acknowledgements](#license-and-acknowledgements)

---

## What it is

`presidio-rs` detects personally identifiable information (PII) in text and
anonymizes it. It is a small, dependency-light Rust library that implements the
**model-free portion** of Microsoft Presidio's design: pattern recognizers,
checksum validators, context-word scoring, scored spans, and configurable
anonymization operators.

The current crate runs fully offline and introduces no Python runtime, C/C++
runtime, network service, or build-time model download. It is designed to be
embedded directly into a Rust application.

```rust
use presidio::{AnalyzerEngine, AnonymizerEngine, EntityType, Operator};

let analyzer = AnalyzerEngine::new();
let text = "Email jane@acme.com about card 4111 1111 1111 1111.";

let findings = analyzer.analyze(text, None);
let clean = AnonymizerEngine::new(Operator::Replace(None)).anonymize(text, &findings);
// "Email <EMAIL_ADDRESS> about card <CREDIT_CARD>."
```

`presidio-rs` is not a complete replacement for the full Microsoft Presidio
suite. It currently does not include NLP/NER-based person or location detection,
OCR, image redaction, DICOM support, structured-data de-identification, or a
network service.

## Why it exists

Microsoft Presidio is a mature and capable PII detection framework. It is also
centered on a Python runtime and typically combines pattern recognizers with an
NLP engine. That is a sensible architecture for many applications, especially
Python services and NLP-heavy workloads.

It is not the right deployment shape for every environment.

Rust-native systems such as GG-CORE may prohibit in-process Python, localhost
services, network access, or runtime downloads. Introducing a Python sidecar or
service only to perform model-free pattern detection creates an additional
runtime, deployment unit, trust boundary, and operational dependency.

`presidio-rs` addresses that narrower problem. It treats Presidio as a design
reference rather than a linked dependency and makes the model-free detection
and anonymization path available as a directly embeddable Rust library.

The intended position is not "Presidio, but universally better." It is:

> A deterministic, embeddable PII enforcement engine for Rust-native,
> offline, and sandboxed systems that do not need or cannot host the full
> Python and NLP stack.

## Value proposition

### Where it creates value

#### In-process privacy enforcement

A Rust caller can analyze and transform text without sending plaintext through
HTTP, a sidecar container, a subprocess, or a foreign-language FFI boundary.
This is especially useful for enforcement points such as:

- agent egress gates
- prompt and response sanitation
- telemetry and log redaction
- local document processing
- command-line security tools
- offline or air-gapped workflows
- constrained plugin and sandbox runtimes

This reduces deployment complexity and narrows the library's own data flow. It
does not remove the caller's responsibility for logging, memory handling,
storage, crash dumps, telemetry, and downstream processing.

#### Small, auditable integration surface

The current direct dependency set is `regex`, `sha2`, and optional `serde`.
There are no model files, package managers, interpreters, web servers, or
runtime downloads in the current implementation.

That makes the crate easier to embed, inventory, test, and govern inside a Rust
product. It does not by itself prove a smaller final binary, a vulnerability-free
dependency tree, or lower operational cost. Those properties must be measured
in the consuming application.

#### Deterministic model-free behavior

Given the same text, registry, threshold, and operators, the analyzer follows a
fixed pipeline of pattern matching, checksum validation, context enhancement,
overlap resolution, threshold filtering, and transformation.

That makes the current behavior suitable for reproducible tests and policy
enforcement. "Deterministic" does not mean "complete" or "correct for every
input." Pattern-based detection still produces false positives and false
negatives.

#### Typed Rust integration

Entity types, scored spans, registries, validators, and anonymization operators
are represented as Rust types. This can catch some integration mistakes at
compile time and makes per-entity policy explicit in Rust callers.

Type safety improves integration discipline. It does not establish superior
PII recall, precision, or policy correctness.

#### A shared privacy primitive

Once evaluation demonstrates that it meets product requirements,
`presidio-rs` can replace duplicated in-tree detectors across GG-CORE,
COREFORGE, and related Rust components. A shared implementation can reduce
policy drift, duplicated recognizers, and inconsistent anonymization behavior.

Centralization also creates a shared failure domain. Adoption therefore depends
on measured detection quality, stable APIs, regression testing, and explicit
ownership rather than architectural preference alone.

#### A path to offline semantic detection

The architecture reserves a future NLP/NER backend without requiring Python.
An offline ONNX path could eventually add person and location recognition while
preserving local execution.

That is roadmap work, not a current capability. Adding model inference will
increase artifact size, memory use, dependency complexity, model-governance
requirements, and startup cost.

### What is proven today

The repository currently demonstrates that:

- the analyzer and anonymizer are implemented as a Rust library
- the crate requires no Python runtime
- the library itself performs no network or filesystem I/O
- crate-local `unsafe` code is forbidden with `#![forbid(unsafe_code)]`
- the direct dependency set is small
- predefined pattern recognizers, context words, and selected checksum
  validators are available
- findings are returned as scored spans
- replace, redact, mask, and salted-hash operators are implemented
- custom recognizers and validators can be registered
- person, location, and NRP entities are not emitted without a future NER
  backend

### What is not yet proven

The repository does not yet provide evidence that `presidio-rs` is:

- faster than Python Presidio
- lower-latency or more predictable under load
- lower-memory or cheaper to operate
- more accurate
- safer overall
- production-ready
- a drop-in replacement for upstream Presidio
- superior in recall or precision
- free of unsafe code in all transitive dependencies
- resistant to all denial-of-service or adversarial inputs

Those claims require a reproducible benchmark and evaluation program. Planned
evidence includes:

- span-level precision, recall, and F1 by entity type
- false-positive and false-negative analysis
- warm and cold latency measurements
- throughput and memory profiling
- artifact-size and dependency comparisons
- fuzzing and adversarial long-input tests
- recognizer regression suites
- transitive dependency and license audits
- ONNX NER quality and resource measurements when that backend exists

### When to use upstream Presidio instead

Use Microsoft Presidio, or evaluate it first, when the workload requires:

- mature NLP/NER detection for names and prose locations
- broad international entity coverage
- OCR or image redaction
- DICOM redaction
- structured or tabular de-identification
- existing Python integration
- a mature service ecosystem and deployment examples
- community recognizers and upstream compatibility

Use `presidio-rs` when the host is Rust-native, direct embedding matters, the
runtime is offline or constrained, and the current pattern-based entity
coverage matches the threat model.

## Highlights

- **Offline library operation.** The current crate performs no network or
  filesystem I/O and requires no runtime downloads.
- **Rust-native embedding.** No Python interpreter, Python binding, local HTTP
  service, or sidecar is required for the current model-free path.
- **Crate-local unsafe code forbidden.** `#![forbid(unsafe_code)]` applies to
  this crate. It is not a claim about every transitive dependency or the Rust
  standard library.
- **Small direct dependency set.** `regex`, `sha2`, and optional `serde`.
- **Presidio-informed architecture.** `AnalyzerEngine`, `RecognizerRegistry`,
  `PatternRecognizer`, context enhancement, `RecognizerResult`, and
  `AnonymizerEngine` map closely to upstream concepts.
- **Checksum validation.** Luhn and IBAN mod-97 validators can promote valid
  candidates or reject invalid candidates, reducing some pattern-only false
  positives.
- **Context-word scoring.** Nearby terms such as `"card"`, `"ssn"`, or
  `"iban"` can increase a candidate's score.
- **Composable anonymization.** `Replace`, `Redact`, `Mask`, and salted `Hash`
  can be configured globally or by entity type.
- **Explicit limitations.** NER-only entities are reserved but not emitted
  until a model backend is implemented and evaluated.

## Quick start

Add it as an internal path or Git dependency. This crate is `publish = false`
and must not be published to crates.io.

```toml
[dependencies]
presidio-rs = { git = "https://github.com/mythologiq-labs-llc/presidio-rs" }
# or, in a workspace:
presidio-rs = { path = "../presidio-rs" }
```

Detect, inspect, and anonymize:

```rust
use presidio::{AnalyzerEngine, AnonymizerEngine, EntityType, Operator};

let analyzer = AnalyzerEngine::new();
let text = "SSN 123-45-6789, IBAN GB82WEST12345698765432, key sk-ABCD1234EFGH5678IJKL.";

// 1. Analyze -> scored spans
for r in analyzer.analyze(text, None) {
    println!("{:<12} [{}..{}] score={:.2}", r.entity_type, r.start, r.end, r.score);
}

// 2. Anonymize with per-entity policy
let engine = AnonymizerEngine::new(Operator::Redact)
    .with_operator(
        EntityType::IbanCode,
        Operator::Mask {
            mask_char: '*',
            keep_last: 4,
        },
    )
    .with_operator(
        EntityType::ApiKey,
        Operator::Hash {
            salt: "tenant-42".into(),
        },
    );

let clean = engine.anonymize(text, &analyzer.analyze(text, None));
```

Scope detection to selected entities:

```rust
let only_secrets =
    analyzer.analyze(text, Some(&[EntityType::ApiKey, EntityType::CreditCard]));
```

---

## Architecture

`presidio-rs` mirrors Presidio's two-engine design: an **analyzer** identifies
PII candidates and an **anonymizer** transforms selected spans. The component
model and model-free scoring concepts map closely to upstream Presidio, while
the implementation, supported recognizers, and runtime are independent.

### Component correspondence

| Python Presidio | `presidio-rs` | Role |
|---|---|---|
| `AnalyzerEngine` | [`AnalyzerEngine`](src/analyzer.rs) | Orchestrates detection over a registry; validates, scores, deduplicates, and applies thresholds. |
| `RecognizerRegistry` | [`RecognizerRegistry`](src/registry.rs) | Holds the recognizers the analyzer runs; replace or extend it. |
| `PatternRecognizer` | [`PatternRecognizer`](src/recognizer.rs) | Defines one entity type using regex patterns, context words, and an optional validator. |
| `Pattern` | [`Pattern`](src/recognizer.rs) | A named regex with a base confidence score. |
| `EntityRecognizer.validate_result()` | [`Validator`](src/validators.rs) | Validator outcome: valid, invalid, or no decision. |
| Context-aware enhancer | [`context::enhance`](src/context.rs) | Boosts scores when a configured context word appears near a match. |
| `NlpEngine` | _reserved future backend_ | Would supply NER-only entities such as `PERSON` and `LOCATION`. **Not implemented.** |
| `RecognizerResult` | [`RecognizerResult`](src/result.rs) | A scored span containing entity type, byte offsets, and score. |
| `AnonymizerEngine` | [`AnonymizerEngine`](src/anonymizer.rs) | Applies operators to detected spans by entity policy. |
| Operators | [`Operator`](src/anonymizer.rs) | Defines replacement, redaction, masking, or hashing behavior. |

The current implementation intentionally omits an NLP engine. It implements the
model-free concepts while reserving a future offline NER integration.

### Detection pipeline

```text
                        ┌──────────────────────── AnalyzerEngine ───────────────────────┐
   text ──────────────► │                                                                │
                        │   RecognizerRegistry                                           │
                        │   ┌───────────────┐   for each recognizer:                     │
                        │   │ PatternReco 1 │──► regex.find_iter ─► base score           │
                        │   │ PatternReco 2 │──► validator(match) ─► promote|drop|keep   │
                        │   │      ...      │──► context::enhance ─► score adjustment    │
                        │   └───────────────┘                                            │
                        │                             ▼                                  │
                        │             resolve overlapping candidate spans                │
                        │                             ▼                                  │
                        │                    apply score threshold                        │
                        │                             ▼                                  │
                        └──────────────── Vec<RecognizerResult> (sorted) ────────────────┘
                                                      │
                                                      ▼
                        ┌──────────────── AnonymizerEngine ──────────────┐
                        │ per-entity Replace|Redact|Mask|Hash operator   │──► output text
                        │ applied right-to-left so offsets stay valid    │
                        └─────────────────────────────────────────────────┘
```

### The scoring model

Each candidate span receives a score in `[0.0, 1.0]` through the following
model-free pipeline:

1. **Base score.** Every `Pattern` carries an initial confidence.
2. **Validator.** A recognizer may promote a valid match, drop an invalid
   match, or leave the score unchanged.
3. **Context boost.** A configured context word within the character window
   can increase the score and apply a minimum contextual score.
4. **Overlap resolution.** Overlapping candidates are resolved by score.
5. **Threshold.** Results below the engine threshold are discarded.

The constants and concepts are aligned with Presidio's model-free context
enhancement where documented, but this project does not claim full behavioral
equivalence across all upstream versions, recognizers, NLP engines, or edge
cases.

Context can improve precision for ambiguous patterns, but it does not eliminate
false positives or guarantee that actual PII will be detected.

---

## Supported entities

| Entity (`EntityType`) | Tag | Method | Validator | Context words |
|---|---|---|---|---|
| `CreditCard` | `CREDIT_CARD` | regex | **Luhn** | credit, card, visa, … |
| `Ssn` | `US_SSN` | regex | — | ssn, social security |
| `Email` | `EMAIL_ADDRESS` | regex | — | email, contact |
| `PhoneNumber` | `PHONE_NUMBER` | regex | — | phone, tel, mobile |
| `IpAddress` | `IP_ADDRESS` | regex (v4/v6) | — | ip, address |
| `MacAddress` | `MAC_ADDRESS` | regex | — | mac, hardware |
| `IbanCode` | `IBAN_CODE` | regex | **mod-97** | iban, bank, account |
| `CryptoWallet` | `CRYPTO` | regex (ETH/BTC) | — | wallet, bitcoin, … |
| `Url` | `URL` | regex | — | — |
| `UsItin` | `US_ITIN` | regex | — | itin, taxpayer |
| `ApiKey` | `API_KEY` | regex (selected OpenAI/GitHub/Slack formats) | — | api, key, token, secret |
| `Person` | `PERSON` | **NER-only** | — | _reserved, not emitted_ |
| `Location` | `LOCATION` | **NER-only** | — | _reserved, not emitted_ |
| `Nrp` | `NRP` | **NER-only** | — | _reserved, not emitted_ |

Recognizer coverage is format-specific. A listed entity type does not imply
coverage of every national, regional, vendor, historical, or malformed variant.

## Anonymization operators

| Operator | Effect | Example output |
|---|---|---|
| `Replace(None)` | Substitute `<ENTITY_TAG>` | `<EMAIL_ADDRESS>` |
| `Replace(Some(s))` | Substitute a fixed string | `[redacted]` |
| `Redact` | Delete the span | _(empty)_ |
| `Mask { mask_char, keep_last }` | Overwrite while preserving the final N characters | `************1111` |
| `Hash { salt }` | Deterministic salted SHA-256 token | `<EMAIL_ADDRESS:9f86d0…>` |

Use one operator for all findings through the free
`anonymize(text, results, &op)` function, or configure per-entity behavior with
`AnonymizerEngine`.

The hash operator supports deterministic correlation, not irreversible
anonymity. Low-entropy values may remain guessable if an attacker obtains the
salt or can test candidate inputs. Callers must define salt or secret-pepper
management, tenant isolation, rotation, domain separation, retention, and
correlation requirements.

## Extending the engine

**Custom recognizer:**

```rust
use presidio::{
    AnalyzerEngine, EntityType, Pattern, PatternRecognizer, RecognizerRegistry,
};

let mut registry = RecognizerRegistry::with_predefined();
registry.add(PatternRecognizer {
    entity_type: EntityType::ApiKey,
    patterns: vec![Pattern::new("acme", r"\bACME-[A-Z0-9]{8}\b", 0.9)],
    context: &["key", "token"],
    validator: None,
});

let analyzer = AnalyzerEngine::with_registry(registry);
```

**Custom validator:** attach a `fn(&str) -> Option<bool>` following the
convention in [`validators`](src/validators.rs). A validator may accept, reject,
or decline to decide on a candidate.

**Empty registry:** start from `RecognizerRegistry::empty()` to run only
application-specific recognizers.

Custom recognizers require the same evaluation discipline as built-in
recognizers. A regex that compiles is not evidence that it detects the intended
entity safely or completely. Regrettably, syntax remains easier than truth.

## The NER gap and roadmap

Some PII has no fixed textual format. Person names, prose locations,
nationalities, religions, and political affiliations generally require semantic
or statistical recognition rather than regex alone.

`Person`, `Location`, and `Nrp` exist in the taxonomy but are never emitted by
the current implementation. This is a deliberate limitation.

Planned work:

1. **Offline evaluation harness.** Track span-level precision, recall, and F1 by
   entity type over a licensed, vendored, air-gapped corpus.
2. **Additional recognizers.** Add international identifiers, postal formats,
   vendor-secret formats, and checksum validators based on product need and
   measured quality.
3. **Adversarial testing.** Add fuzzing, large-input tests, regex stress cases,
   malformed Unicode coverage, and performance regression gates.
4. **Comparative benchmarks.** Measure warm and cold latency, throughput,
   memory, artifact size, and deployment complexity against explicitly defined
   Python Presidio configurations.
5. **Offline NER backend.** Evaluate an ONNX token classifier with a compatible
   tokenizer for `Person` and `Location` recognition.
6. **Model governance.** Record model identity, license, provenance, evaluation
   results, resource requirements, and upgrade policy before any NER backend is
   adopted.

The ONNX direction is a candidate architecture, not a committed model choice or
a completed capability.

## Design principles and non-goals

### Principles

- **Offline and minimal.** No network, Python runtime, C/C++ runtime, or
  build-time downloads in the current crate.
- **Values over hidden state.** The analyzer maps text and explicit
  configuration to scored spans. Logging and metrics belong to the caller.
- **Presidio as reference, not dependency.** The project uses published design
  concepts without linking, vendoring, or sidecarring upstream Presidio.
- **Evidence before superiority claims.** Architectural differences may be
  described now. Performance, accuracy, safety, and cost claims require
  measurements.
- **Honesty about limits.** Missing NER coverage and unsupported suite features
  remain visible.
- **Caller-owned policy.** Detection output is evidence for a policy decision,
  not a guarantee that data is safe to release.

### Non-goals

- Not a network service.
- Not an OCR, image, or DICOM redactor.
- Not a structured or tabular de-identifier.
- Not a Python binding.
- Not a claim of full Microsoft Presidio compatibility.
- Not a guarantee that all sensitive information will be found.
- Not a substitute for threat modeling, data classification, or human review
  where consequences require them.

## Relationship to GG-CORE and COREFORGE

GG-CORE currently ships an in-tree detector for its egress sanitizer.
`presidio-rs` is intended to become the standalone, reusable evolution of that
work.

Adoption is conditional. The crate should replace existing detectors only after
it demonstrates that it meets or exceeds their required entity coverage and
quality against a reproducible evaluation harness.

If adopted, the intended value is:

- one versioned PII API across Rust components
- shared recognizers and anonymization semantics
- fewer duplicated policy implementations
- direct embedding within constrained runtimes
- consistent regression testing and evidence
- explicit ownership of detector changes

A shared crate also concentrates risk. A false negative, breaking API change,
or flawed recognizer may affect every consumer. Releases therefore require
versioning, compatibility policy, evaluation receipts, and staged adoption.

## Project layout

```text
presidio-rs/
├── src/
│   ├── lib.rs          # crate root, public API, docs
│   ├── entity.rs       # EntityType taxonomy and tags
│   ├── recognizer.rs   # Pattern, PatternRecognizer, Validator
│   ├── registry.rs     # RecognizerRegistry and predefined recognizers
│   ├── validators.rs   # Luhn, IBAN mod-97
│   ├── context.rs      # context-word enhancer
│   ├── analyzer.rs     # detect, validate, score, deduplicate, threshold
│   ├── anonymizer.rs   # AnonymizerEngine and operators
│   └── result.rs       # RecognizerResult
├── tests/
│   └── integration.rs  # end-to-end detection and anonymization
├── Cargo.toml          # publish = false
├── LICENSE             # proprietary
└── README.md
```

## Development

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo doc --no-deps --open

cargo build --features serde
```

Optional feature:

- `serde`: derives `Serialize` and `Deserialize` for `EntityType` and
  `RecognizerResult`.

Before release or adoption, also run the evaluation, fuzzing, dependency audit,
and benchmark suites once those gates are implemented.

## Security and privacy

- **Library-local data flow.** The crate itself returns values and performs no
  network or filesystem I/O. The caller controls logs, storage, telemetry,
  memory lifecycle, and downstream transmission.
- **Crate-local unsafe prohibition.** `#![forbid(unsafe_code)]` prevents unsafe
  blocks in this crate. It does not prove that dependencies or the Rust
  standard library contain no unsafe internals.
- **Pattern limitations.** Regex, checksums, and context are fallible.
  Unsupported formats and semantic entities may pass undetected.
- **Deterministic hashing.** Salted SHA-256 permits stable correlation but may
  be vulnerable to guessing for low-entropy inputs. Treat keying and salt
  management as part of the threat model.
- **Threshold tradeoffs.** Raising the threshold may reduce false positives
  while increasing false negatives. Lowering it may do the reverse.
- **Denial-of-service review.** Large and adversarial inputs require explicit
  stress testing and input-size policy before exposing the library to
  untrusted workloads.
- **No guarantee of de-identification.** Successful transformation of detected
  spans does not prove that the resulting text is anonymous or safe for every
  use.

## Claims policy

Project documentation, release notes, and marketing material may state current
architectural facts, supported entities, implemented operators, and measured
results.

They must not state or imply superiority in speed, latency, throughput, memory,
cost, accuracy, safety, recall, precision, or production maturity unless the
claim is tied to:

1. a named comparison target and configuration
2. a versioned corpus or workload
3. a reproducible test procedure
4. recorded hardware and runtime conditions
5. published results and known limitations
6. a commit or release identity

Preferred wording before evidence exists:

- "designed for"
- "enables"
- "avoids introducing"
- "can reduce"
- "intended to"
- "not yet measured"

Rejected wording before evidence exists:

- "faster than Presidio"
- "more secure"
- "zero false positives"
- "production-ready"
- "drop-in replacement"
- "guaranteed PII protection"
- "zero unsafe code" without the crate-local qualification

This policy exists because privacy infrastructure should accumulate evidence,
not adjectives.

## License and acknowledgements

Proprietary and confidential. See [`LICENSE`](./LICENSE). Internal use within
MythologIQ-Labs-LLC, including GG-CORE and COREFORGE, only. The crate is
`publish = false`.

The architecture and scoring model are informed by
[Microsoft Presidio](https://github.com/microsoft/presidio), licensed under MIT.
No Presidio code is linked, vendored, or redistributed by this repository.
