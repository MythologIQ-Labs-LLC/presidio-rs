# ADR 0006: Bind Findings and Reports to Exact Text Documents

- **Status:** Accepted
- **Date:** 2026-07-27
- **Decision owners:** MythologIQ Labs LLC maintainers

## Context

A structurally valid byte span cannot prove which source text it belongs to. The current string-based analyzer APIs validate that offsets are legal for the string being analyzed, but once a finding leaves that call there is no durable protection against applying it to a different string.

A caller-controlled document ID alone is insufficient. Consumers may accidentally reuse an ID after text changes, or may derive IDs from mutable application records. In either case, the ID can remain equal while the source bytes differ.

The backend-neutral recognizer trait also needs a document contract that can later own normalized views, tokenized views, offset mappings, and input limits without changing every recognizer signature.

## Decision

Introduce:

- `DocumentId`, a validated caller-controlled opaque identifier;
- `DocumentFingerprint`, a SHA-256 digest of the exact original UTF-8 bytes;
- `DocumentBinding`, which combines document ID, byte length, and fingerprint; and
- `TextDocument<'a>`, which borrows the exact original text and owns its binding.

All externally visible spans remain byte offsets into the original UTF-8 source.

`AnalyzerEngine::analyze_document` and `analyze_document_with_options` are added beside the existing string APIs. Document-aware analysis:

- attaches the exact `DocumentBinding` to every validated `Finding`;
- attaches the same binding once at report level;
- preserves the existing candidate and legacy-compatible projections; and
- allows the entire report or an individual finding to be validated against a supplied document before slicing or applying spans.

The existing `analyze`, `analyze_report`, and `analyze_report_with_options` APIs remain source compatible. Their findings and reports remain explicitly unbound rather than receiving a fabricated or process-local document identity.

## Binding semantics

A binding matches a document only when all three dimensions agree:

1. caller-controlled `DocumentId`;
2. original UTF-8 byte length; and
3. SHA-256 fingerprint of the exact original UTF-8 bytes.

This detects:

- applying findings to a different application record;
- reusing a document ID after content mutation;
- same-length text substitutions; and
- accidental offset reuse across different source versions.

`Finding::slice_document` validates the binding and UTF-8 span before returning a substring.

`AnalysisReport::validate_for_document` validates:

- report-level identity and content binding;
- every candidate's binding and span; and
- every legacy-compatible result span against the same source.

## Privacy boundary

`TextDocument` does not implement serialization. Its custom `Debug` implementation omits original plaintext.

`DocumentBinding` and `DocumentFingerprint` may be serialized as part of a report. The fingerprint is an integrity and identity mechanism, not encryption, anonymization, or proof that content is non-sensitive. A SHA-256 digest of low-entropy or guessable text may be susceptible to offline guessing.

Consumers should:

- use opaque document IDs rather than embedding PII in IDs;
- treat bindings and reports as potentially sensitive metadata;
- apply appropriate retention and access controls; and
- avoid treating a fingerprint as a safe public representation of source content.

A future wire-format decision may define configurable omission, keyed fingerprints, or consumer-supplied binding strategies if real use cases require them.

## Compatibility boundary

Adding an optional document binding to `Finding` and `AnalysisReport` is additive for Rust source consumers because their fields remain private and construction APIs remain available.

The serialized shape is still pre-stable. The project continues to provide serialization without a durable deserialization or schema-version promise. Public wire compatibility requires a separate ADR.

Legacy `RecognizerResult` values remain structurally unbound. Their spans can be validated only in the context of a document-bound `AnalysisReport` or through direct caller validation.

## Deliberate deferrals

This slice does not yet add:

- normalized or tokenized document views;
- normalized-to-original offset maps;
- request-time input-size policy;
- locale or capability selection;
- the final backend-neutral recognizer execution trait;
- persistent deserialization of bindings or reports; or
- fallible anonymization over document-bound findings.

Those contracts should build on `TextDocument` rather than bypass it.

## Security and correctness consequences

### Positive

- Findings can be rejected when applied to the wrong text.
- Same-ID source mutation is detected.
- Original-coordinate UTF-8 slicing is centralized and fallible.
- Reports carry enough identity to validate their legacy compatibility projection.
- Raw text remains borrowed and is not copied into report structures.
- The next recognizer-trait phase has a real source contract to depend on.

### Costs

- Document-aware findings repeat the binding in memory and serialized output.
- SHA-256 computation adds linear work per document.
- Fingerprints may be sensitive metadata for low-entropy input.
- Callers must create and manage stable opaque document IDs.
- Existing string APIs remain less safe until consumers migrate.

## Alternatives considered

### Bind only by document ID

Rejected because IDs can be reused after source mutation and therefore cannot prove exact content identity.

### Bind only by content fingerprint

Rejected because two application records may intentionally contain identical text but still require distinct ownership, retention, or audit identity.

### Store the original text in each report

Rejected because it duplicates sensitive plaintext, increases retention risk, and is unnecessary for identity validation.

### Generate random IDs inside the analyzer

Rejected because process-local random IDs do not map reliably to consumer records, retries, or persisted workflows. The consumer owns record identity.

### Replace existing string APIs immediately

Rejected because it would create an unnecessary breaking change before consumer pilots demonstrate migration requirements.

## Validation

This decision is effective when:

- a document binding combines ID, byte length, and exact-content fingerprint;
- same-ID changed content is rejected;
- same-content different-ID documents are rejected;
- document-aware findings safely slice original UTF-8 text;
- string-only reports remain explicitly unbound;
- report validation checks candidates and legacy-compatible spans;
- debug output does not contain original plaintext; and
- formatting, Clippy, tests, documentation, package verification, Rust 1.74, DCO, and dependency audit pass.

## Follow-up

The next architectural slice should introduce `AnalysisRequest` and backend-neutral candidate emission around `TextDocument`. Only then should the analyzer registry migrate toward immutable trait objects and runtime mutation begin deprecation.
