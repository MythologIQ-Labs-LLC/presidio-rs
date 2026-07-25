<div align="center">

# presidio-rs

**Offline, pure-Rust PII detection & anonymization**
_A Rust-native reimagining of [Microsoft Presidio](https://github.com/microsoft/presidio), built for sandboxed runtimes where Presidio itself cannot go._

![status](https://img.shields.io/badge/status-internal-blue)
![rust](https://img.shields.io/badge/rust-2021%20edition-orange)
![offline](https://img.shields.io/badge/network-none-brightgreen)
![unsafe](https://img.shields.io/badge/unsafe-forbidden-success)
![clippy](https://img.shields.io/badge/clippy-%E2%88%92D%20warnings-success)
![license](https://img.shields.io/badge/license-proprietary-lightgrey)

**Internal — MythologIQ-Labs-LLC. Confidential. Not for public distribution.**

</div>

---

## Table of contents

- [What it is](#what-it-is)
- [Why it exists](#why-it-exists)
- [Highlights](#highlights)
- [Quick start](#quick-start)
- [Architecture](#architecture) — _mapped to Python Presidio_
  - [Component correspondence](#component-correspondence)
  - [Detection pipeline](#detection-pipeline)
  - [The scoring model](#the-scoring-model)
- [Supported entities](#supported-entities)
- [Anonymization operators](#anonymization-operators)
- [Extending the engine](#extending-the-engine)
- [The NER gap & roadmap](#the-ner-gap--roadmap)
- [Design principles & non-goals](#design-principles--non-goals)
- [Relationship to GG-CORE / COREFORGE](#relationship-to-gg-core--coreforge)
- [Project layout](#project-layout)
- [Development](#development)
- [Security & privacy](#security--privacy)
- [License & acknowledgements](#license--acknowledgements)

---

## What it is

`presidio-rs` detects personally-identifiable information (PII) in text and
anonymizes it. It is a small, dependency-light Rust library that ports the
**model-free core** of Microsoft Presidio's design — pattern recognizers,
checksum validators, and context-word scoring — into a crate that runs **fully
offline**, with no Python, no C/C++ runtime, and no network access of any kind.

```rust
use presidio::{AnalyzerEngine, AnonymizerEngine, EntityType, Operator};

let analyzer = AnalyzerEngine::new();
let text = "Email jane@acme.com about card 4111 1111 1111 1111.";

let findings = analyzer.analyze(text, None);
let clean = AnonymizerEngine::new(Operator::Replace(None)).anonymize(text, &findings);
// "Email <EMAIL_ADDRESS> about card <CREDIT_CARD>."
```

## Why it exists

Microsoft Presidio is the industry reference for PII detection — but it is
**Python + spaCy**, and it exposes only an in-process Python API or a Flask
HTTP server. Neither fits a **sandboxed, offline, Rust** runtime such as
**GG-CORE**, whose charter forbids in-process Python (bindings run the other
way — Python calls Rust), forbids network and localhost ports (IPC is
named-pipe / Unix-socket only), and keeps the dependency surface minimal.

There is no mature Rust-native equivalent. `presidio-rs` fills that gap by
treating Presidio as a **design reference to port from**, not a component to
link, vendor, or sidecar. The result is a detector shaped like Presidio's —
same concepts, same scoring model — that a Rust sandbox can actually embed.

## Highlights

- **Zero network, zero unsafe.** `#![forbid(unsafe_code)]`; dependencies are
  just `regex`, `sha2`, and optional `serde`. No downloads, ever.
- **Presidio-faithful architecture.** `AnalyzerEngine`, `RecognizerRegistry`,
  `PatternRecognizer`, a context-aware enhancer, `RecognizerResult`, and an
  `AnonymizerEngine` with pluggable operators — the same component model as
  upstream (see [Architecture](#architecture)).
- **Checksum-validated confidence.** Luhn (cards) and IBAN mod-97 promote a
  match to full confidence or drop it — no more false positives on random digit
  runs that happen to look like a card or IBAN.
- **Context-word scoring.** A nearby keyword (`"card"`, `"ssn"`, `"iban"`)
  boosts a match's confidence, the single biggest precision lever a regex-only
  detector lacks.
- **Composable anonymization.** `Replace`, `Redact`, `Mask` (keep-last-N), and
  salted `Hash`, applied globally or per entity type.
- **Honest about the NER gap.** Names and prose locations need a model; those
  entity types exist but are never emitted until the ONNX backend lands. See
  [roadmap](#the-ner-gap--roadmap).

## Quick start

Add it as an internal path/git dependency (this crate is `publish = false` and
must never reach crates.io):

```toml
[dependencies]
presidio-rs = { git = "https://github.com/mythologiq-labs-llc/presidio-rs" }
# or, in a workspace:  presidio-rs = { path = "../presidio-rs" }
```

Detect, inspect, anonymize:

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
    .with_operator(EntityType::IbanCode, Operator::Mask { mask_char: '*', keep_last: 4 })
    .with_operator(EntityType::ApiKey, Operator::Hash { salt: "tenant-42".into() });
let clean = engine.anonymize(text, &analyzer.analyze(text, None));
```

Scope detection to specific entities by passing `Some(&[...])`:

```rust
let only_secrets = analyzer.analyze(text, Some(&[EntityType::ApiKey, EntityType::CreditCard]));
```

---

## Architecture

`presidio-rs` deliberately mirrors Presidio's two-engine design — an
**analyzer** that finds PII and an **anonymizer** that transforms it — so anyone
who knows Presidio is immediately at home, and so upstream design decisions
(scoring, context enhancement, recognizer registry) map straight across.

### Component correspondence

| Python Presidio | `presidio-rs` | Role |
|---|---|---|
| `AnalyzerEngine` | [`AnalyzerEngine`](src/analyzer.rs) | Orchestrates detection over a registry; validates, scores, dedupes, thresholds. |
| `RecognizerRegistry` | [`RecognizerRegistry`](src/registry.rs) | Holds the recognizers the analyzer runs; swap or extend it. |
| `PatternRecognizer` | [`PatternRecognizer`](src/recognizer.rs) | One entity type: regex `Pattern`s + context words + optional validator. |
| `Pattern` | [`Pattern`](src/recognizer.rs) | A named regex with a base confidence score. |
| `EntityRecognizer.validate_result()` | [`Validator`](src/validators.rs) | Checksum decision: valid → 1.0, invalid → drop, none → keep. |
| `LemmaContextAwareEnhancer` | [`context::enhance`](src/context.rs) | Boosts score when a context word sits near a match. |
| `NlpEngine` (spaCy / transformers) | _reserved_ — future ONNX NER backend | Supplies NER-only entities (`PERSON`/`LOCATION`/`NRP`). **Not yet present.** |
| `RecognizerResult` | [`RecognizerResult`](src/result.rs) | A scored span: `entity_type`, `start`, `end`, `score`. |
| `AnonymizerEngine` | [`AnonymizerEngine`](src/anonymizer.rs) | Applies operators to detected spans, per entity. |
| Operators (`replace`/`redact`/`mask`/`hash`) | [`Operator`](src/anonymizer.rs) | The transform applied to each span. |

The one intentional divergence is the **`NlpEngine`**: Presidio's NER runs a
spaCy/transformers model; ours is a reserved slot (see
[the NER gap](#the-ner-gap--roadmap)). Everything model-free is present and
faithful.

### Detection pipeline

```
                        ┌──────────────────────── AnalyzerEngine ───────────────────────┐
   text ──────────────► │                                                                │
                        │   RecognizerRegistry                                           │
                        │   ┌───────────────┐   for each recognizer:                     │
                        │   │ PatternReco 1 │──►  regex.find_iter ─► base_score          │
                        │   │ PatternReco 2 │──►  validator(match) ─► 1.0 | drop | keep   │
                        │   │      ...      │──►  context::enhance ─► +0.35 (floor 0.4)   │
                        │   └───────────────┘                                            │
                        │                             ▼                                  │
                        │            dedupe overlaps (keep highest score)                │
                        │                             ▼                                  │
                        │            filter: score ≥ threshold (default 0.3)             │
                        │                             ▼                                  │
                        └──────────────── Vec<RecognizerResult> (sorted) ────────────────┘
                                                      │
                                                      ▼
                        ┌──────────────── AnonymizerEngine ──────────────┐
                        │  per-entity Operator: Replace|Redact|Mask|Hash  │──► clean text
                        │  applied right-to-left so offsets stay valid    │
                        └─────────────────────────────────────────────────┘
```

### The scoring model

Each candidate span accrues a confidence in `[0.0, 1.0]`, exactly as Presidio
scores results:

1. **Base score** — every `Pattern` carries a starting confidence (e.g. email
   `0.7`, a bare card `0.3`, an API-key prefix `0.9`).
2. **Validator** — if the recognizer has one, a passing checksum sets the score
   to **`1.0`**; a failing checksum **drops** the match; no validator leaves the
   base score untouched. (`validators::luhn`, `validators::iban_mod97`.)
3. **Context boost** — if a recognizer context word appears within
   `CONTEXT_WINDOW_CHARS` of the match, the score gains
   `CONTEXT_SIMILARITY_FACTOR` (**+0.35**) and is floored at
   `MIN_SCORE_WITH_CONTEXT` (**0.4**) — the same constants as Presidio's
   `LemmaContextAwareEnhancer`.
4. **Overlap resolution** — among overlapping spans, the highest score wins.
5. **Threshold** — results below the engine threshold (default **0.3**) are
   discarded. Tune with `AnalyzerEngine::new().with_threshold(0.5)`.

This is what lets a loose numeric pattern (a 9-digit run) stay quiet until a
context word (`"account"`, `"passport"`) makes it a genuine hit — precision
without a model.

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
| `ApiKey` | `API_KEY` | regex (OpenAI/GitHub/Slack) | — | api, key, token, secret |
| `Person` | `PERSON` | **NER-only** | — | _reserved (see roadmap)_ |
| `Location` | `LOCATION` | **NER-only** | — | _reserved (see roadmap)_ |
| `Nrp` | `NRP` | **NER-only** | — | _reserved (see roadmap)_ |

## Anonymization operators

| Operator | Effect | Example output |
|---|---|---|
| `Replace(None)` | Substitute `<ENTITY_TAG>` | `<EMAIL_ADDRESS>` |
| `Replace(Some(s))` | Substitute a fixed string | `[redacted]` |
| `Redact` | Delete the span | _(empty)_ |
| `Mask { mask_char, keep_last }` | Overwrite, reveal last N | `************1111` |
| `Hash { salt }` | Salted SHA-256 pseudonym | `<EMAIL_ADDRESS:9f86d0…>` |

Use one operator for everything via the free `anonymize(text, results, &op)`
function, or a per-entity policy via `AnonymizerEngine`.

## Extending the engine

**Custom recognizer** — bring your own regex + context + validator:

```rust
use presidio::{AnalyzerEngine, EntityType, Pattern, PatternRecognizer, RecognizerRegistry};

let mut registry = RecognizerRegistry::with_predefined();
registry.add(PatternRecognizer {
    entity_type: EntityType::ApiKey,
    patterns: vec![Pattern::new("acme", r"\bACME-[A-Z0-9]{8}\b", 0.9)],
    context: &["key", "token"],
    validator: None,
});
let analyzer = AnalyzerEngine::with_registry(registry);
```

**Custom validator** — a `fn(&str) -> Option<bool>` (see [`validators`](src/validators.rs)
for the convention) attached to a recognizer promotes or rejects matches by
checksum.

**Empty registry** — start from `RecognizerRegistry::empty()` to run _only_ your
own recognizers.

## The NER gap & roadmap

Some PII has **no fixed format** — person names, prose locations, nationality —
and no regex can catch it. Presidio catches these only through an NLP/NER model,
and so must we. The `Person`, `Location`, and `Nrp` entity types exist in the
taxonomy but are **never emitted today**; that is a deliberate, documented gap,
not a bug.

Closing it stays true to the offline charter — a model runtime, never a Python
dependency:

1. **Offline evaluation harness** — span-level precision / recall / F1 per
   entity type over a vendored, air-gapped corpus (ai4privacy
   `pii-masking-openpii-1m`, CC-BY-4.0; Presidio-research synthetic data
   generated once, then vendored). Turn "we redact PII" into a tracked number.
2. **More recognizers** — additional international IDs, postal formats, and
   checksum validators.
3. **Offline ONNX NER backend** — a token-classifier (e.g. `dslim/distilbert-NER`,
   Apache-2.0) run via `candle-onnx` with a real WordPiece tokenizer, emitting
   `Person`/`Location`. This is the only sandbox-legal route to name/location
   coverage and aligns with GG-CORE's existing ONNX inference path.

## Design principles & non-goals

**Principles**
- **Offline and minimal.** No network, no Python, no C/C++ runtime, no
  build-time downloads. `regex` + `sha2` (+ optional `serde`) is the whole tree.
- **Values over state.** The analyzer is a pure function of text → scored spans;
  logging and metrics belong to the caller.
- **Presidio as reference, not dependency.** We port the published design; we
  never link, vendor, or sidecar Presidio.
- **Honesty about limits.** The NER gap is documented, not hidden; scores are
  calibrated, not asserted.

**Non-goals**
- Not a network service (no HTTP/gRPC surface — by design).
- Not an OCR / image / DICOM redactor (Presidio's `image-redactor` scope).
- Not a structured/tabular de-identifier (Presidio's `structured` scope).
- Not a Python binding.

## Relationship to GG-CORE / COREFORGE

GG-CORE ships a minimal in-tree detector (`security/pii_detector.rs`) for its own
egress sanitizer. `presidio-rs` is the **standalone, reusable evolution** of that
work. Once it demonstrably surpasses the in-tree detector against the evaluation
harness, GG-CORE and COREFORGE can depend on it internally — a single, tested
PII surface shared across the stack, with none of the offline/sandbox
compromises a Presidio sidecar would force.

## Project layout

```
presidio-rs/
├── src/
│   ├── lib.rs          # crate root, public API, docs
│   ├── entity.rs       # EntityType taxonomy (+ tags)
│   ├── recognizer.rs   # Pattern, PatternRecognizer, Validator
│   ├── registry.rs     # RecognizerRegistry + predefined recognizers
│   ├── validators.rs   # Luhn, IBAN mod-97
│   ├── context.rs      # context-word enhancer (Presidio-aligned constants)
│   ├── analyzer.rs     # AnalyzerEngine: detect → validate → score → dedupe → threshold
│   ├── anonymizer.rs   # AnonymizerEngine + Operators
│   └── result.rs       # RecognizerResult
├── tests/
│   └── integration.rs  # end-to-end detection + anonymization
├── Cargo.toml          # publish = false (proprietary/internal)
├── LICENSE             # proprietary — MythologIQ-Labs-LLC
└── README.md
```

## Development

```bash
cargo build                              # build
cargo test                               # unit + integration + doctests
cargo clippy --all-targets -- -D warnings   # lints as errors (CI gate)
cargo doc --no-deps --open               # API docs

cargo build --features serde             # enable serde on results/entities
```

Optional features: `serde` (derive `Serialize`/`Deserialize` on `EntityType`
and `RecognizerResult`).

## Security & privacy

- **No egress.** The library performs no I/O beyond returning values to the
  caller. It cannot phone home; there is nothing to phone home with.
- **No unsafe.** `#![forbid(unsafe_code)]` is enforced crate-wide.
- **Deterministic pseudonymization.** The `Hash` operator is salted SHA-256;
  supply a per-tenant salt to prevent cross-context correlation.
- **Recall-oriented.** Like Presidio, the defaults favor catching PII over
  avoiding false positives — tune the threshold up if precision matters more
  than recall for your surface.

## License & acknowledgements

Proprietary and confidential — see [`LICENSE`](./LICENSE). Internal use within
MythologIQ-Labs-LLC (GG-CORE, COREFORGE) only; `publish = false`.

Architecture and scoring model are informed by
[Microsoft Presidio](https://github.com/microsoft/presidio) (MIT), used here as
a design reference — no Presidio code is linked, vendored, or redistributed.
