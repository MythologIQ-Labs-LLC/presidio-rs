# presidio-rs

**Internal — MythologIQ-Labs-LLC. Confidential. Not for public distribution.**

Offline, pure-Rust PII detection and anonymization — a Rust-native reimagining
of [Microsoft Presidio](https://github.com/microsoft/presidio)'s analyzer /
anonymizer design, built for environments where Presidio itself cannot go:
Rust-only, sandboxed, and **zero network access**.

## Why this exists

Presidio is Python + spaCy and exposes only an in-process Python API or a Flask
HTTP server. Neither fits a sandboxed offline Rust runtime like **GG-CORE**
(no in-process Python; named-pipe/Unix-socket IPC only; network denied). There
is no mature Rust-native equivalent. `presidio-rs` fills that gap: it ports the
model-free parts of Presidio's design — pattern recognizers, checksum
validators, and context-word scoring — into a small, dependency-light,
fully-offline crate.

## What it does today (v0.1, model-free core)

- **Recognizers** (`src/analyzer.rs`) for credit cards, US SSN/ITIN, email,
  phone, IPv4/IPv6, MAC, IBAN, crypto wallets (ETH/BTC), URLs, and API keys
  (OpenAI/GitHub/Slack).
- **Checksum validators** (`src/validators.rs`): Luhn (cards) and IBAN mod-97 —
  a passing checksum promotes a match to full confidence; a failing one drops it.
- **Context-word scoring** (`src/context.rs`): a keyword near a match (e.g.
  "card", "SSN", "iban") boosts confidence (`+0.35`, floor `0.4`), mirroring
  Presidio's `LemmaContextAwareEnhancer`. This is the main precision lever a
  regex-only detector lacks.
- **Anonymizer operators** (`src/anonymizer.rs`): `Replace`, `Redact`, `Mask`
  (keep-last-N), and salted `Hash` (deterministic pseudonym).

## The NER gap (deliberate)

`PERSON`, `LOCATION`, and `NRP` (nationality/religion/political) have **no regex
form** — Presidio catches them only via an NLP/NER model, and so must we. Those
`EntityType` variants exist but are never emitted today. Closing the gap is a
roadmap item, not a Python dependency:

## Roadmap

1. **Offline evaluation harness** — span-level precision/recall/F1 per entity
   type over a vendored, air-gapped corpus (ai4privacy `pii-masking-openpii-1m`,
   CC-BY-4.0; presidio-research synthetic data generated once then vendored).
   Turn "we redact PII" into a tracked number.
2. **More international recognizers** — additional national IDs, postal formats,
   and validators (extend `validators.rs` / `analyzer.rs`).
3. **Offline ONNX NER backend** — a token-classifier (e.g. `dslim/distilbert-NER`,
   Apache-2.0) run via candle-onnx with a real WordPiece tokenizer, emitting
   `Person`/`Location`. This is the only sandbox-legal route to name/location
   coverage and aligns with GG-CORE's existing ONNX path.

## Usage

```rust
use presidio::{anonymize, AnalyzerEngine, Operator};

let analyzer = AnalyzerEngine::new();
let text = "Email jane@acme.com about card 4111 1111 1111 1111.";
let found = analyzer.analyze(text, None);
let clean = anonymize(text, &found, &Operator::Replace(None));
// -> "Email <EMAIL_ADDRESS> about card <CREDIT_CARD>."
```

## Design principles

- **Offline, no network, no C/C++ runtime.** Dependencies: `regex`, `sha2`
  (hashing), and optional `serde` (result serialization) — nothing else.
- **Values over state.** The analyzer is a pure function of text → scored spans;
  effects (logging, metrics) belong to the caller.
- **Presidio as reference, not dependency.** We port the published design; we do
  not link, vendor, or sidecar Presidio.

## Relationship to GG-CORE

GG-CORE's in-tree `security/pii_detector.rs` is the minimal detector for its own
egress sanitizer. `presidio-rs` is the standalone, reusable evolution of that
work; GG-CORE (and COREFORGE) can depend on it internally once it surpasses the
in-tree detector against the evaluation harness.

## License

Proprietary and confidential. See [`LICENSE`](./LICENSE). `publish = false` —
this crate must never be pushed to crates.io.
