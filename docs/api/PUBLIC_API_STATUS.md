# Public API Status

The crate is pre-1.0. This inventory communicates maintainer intent for the public alpha; it is not a promise that every API will remain source-compatible forever.

## Status definitions

- **Alpha target:** the preferred integration surface for new consumers. Changes require tests, documentation, changelog and migration guidance.
- **Legacy-compatible:** retained for existing consumers while the target path matures. New designs should not depend on undocumented behavior.
- **Transitional:** useful during migration but not intended as the final long-term contract.
- **Experimental:** exposed for evaluation. Shape and semantics may change with evidence.
- **Accepted contract, not implemented:** semantics are frozen by ADR, but the Rust API has not yet landed.
- **Unsupported contract:** behavior consumers must not treat as stable or authoritative.

## Alpha target

### Source and value contracts

- `DocumentId`
- `DocumentFingerprint`
- `DocumentBinding`
- `TextDocument`
- `Span`
- `Confidence`
- `EntityId`
- `RecognizerId`
- `MetadataId`
- `Evidence`
- `Finding`

Expected guarantees:

- original UTF-8 byte coordinates;
- explicit source binding for document-aware findings;
- finite confidence in `0.0..=1.0`;
- bounded identifier-shaped metadata; and
- fallible validation rather than silent correction.

### Request-oriented analysis

- `AnalysisRequest`
- `AnalyzerEngine::analyze_request`
- `AnalysisReport::candidates`
- `AnalysisReport::validate_for_document`
- `AnalysisStatus`
- `AnalysisIssue`
- `AnalysisExecutionError`

`AnalysisReport::candidates()` is authoritative on this path. `legacy_compatible_results()` is only a compatibility projection and may be incomplete for open entities.

### Recognizer extension contract

- `RecognizerMetadata`
- `RecognitionMechanism`
- `Recognizer`
- `CandidateEmitter`
- `EmissionStatus`
- `CandidateEmissionError`
- `RecognitionError`
- `RecognitionErrorKind`
- `RecognizerRegistry::add_with_metadata`
- `AnalyzerEngine::add_recognizer_with_metadata`
- `AnalyzerEngine::add_backend`

Recognizers must preserve original source coordinates, declare every emitted entity, use authoritative metadata, and submit candidates through the emitter.

## Experimental resolution API

ADR 0009 freezes the version-1 semantics for:

- `report_all/v1`;
- `best_candidate/v1`; and
- `conservative_redaction/v1`.

The pure additive implementation exposes:

- `resolve_candidates`;
- `ResolutionPolicy`;
- `ResolutionOptions`;
- `ResolutionReport`;
- `ResolvedFinding` and `ResolvedEntity`;
- `ResolutionDecision`;
- `ResolutionStatus`; and
- `ResolutionError`.

Document-aware integration adds:

- `AnalysisReport::resolve_for_document`;
- `ResolvedAnalysisReport`; and
- `AnalysisResolutionError`.

`resolve_for_document` validates the report against the exact `TextDocument`, refuses candidate-truncated analysis, retains analyzer version and source analysis status, and returns the separate pure `ResolutionReport`. Raw candidates and the legacy-compatible projection remain independently inspectable and unchanged.

The resolver enforces hard candidate and output limits, reports decision-evidence truncation explicitly, and never transforms document text. The next development round is fallible atomic anonymization over the document-validated resolved contract.

`legacy_compatible_results()` remains compatibility output only and does not implement one of the accepted policies.

The conformance contract is documented in [`docs/testing/RESOLUTION_CONFORMANCE_MATRIX.md`](../testing/RESOLUTION_CONFORMANCE_MATRIX.md).

## Legacy-compatible

- `AnalyzerEngine::analyze`
- `RecognizerResult`
- `EntityType`
- `AnonymizerEngine`
- `Operator`
- free function `anonymize`
- `Pattern::new`
- `RecognizerRegistry::add`
- `AnalyzerEngine::add_recognizer`

These APIs remain supported during alpha, but they do not provide exact document binding, open entities, authoritative provenance, typed backend failures, or the full request resource contract.

The current legacy overlap and anonymization behavior is compatibility behavior, not the permanent resolution policy.

## Transitional

- `AnalyzerEngine::analyze_report`
- `AnalyzerEngine::analyze_report_with_options`
- `AnalyzerEngine::analyze_document`
- `AnalyzerEngine::analyze_document_with_options`
- `AnalysisOptions`
- `legacy_compatible_results`

These APIs helped stage candidate preservation and document binding before `AnalysisRequest`. They remain useful, but new integrations should prefer `analyze_request`.

## Other experimental surfaces

- optional `serde` serialization for metadata, findings, reports, requests, statuses, failures, and resolution values;
- recognizer evaluation receipts;
- capability and locale selection semantics beyond the documented current behavior; and
- any future transformation-record, evaluation-corpus or semantic-adapter interfaces until separately promoted.

Serialization is one-way for several validated types and is not a stable wire protocol.

## Unsupported contracts

Consumers must not rely on:

- struct debug formatting;
- iteration order unless explicitly documented;
- private module layout;
- exact error prose rather than typed variants or stable codes;
- regex internals or current built-in pattern text;
- complete PII coverage;
- all candidates being safe to transform without resolution;
- `legacy_compatible_results()` representing open entities completely;
- SHA-256 document fingerprints providing secrecy or anonymity;
- deterministic hashing being irreversible anonymity; or
- pre-1.0 serialized request, report, or resolution compatibility.

## Change policy during public alpha

A material public API change must include:

1. tests for old and new behavior where compatibility is claimed;
2. a changelog entry;
3. updated API inventory status;
4. migration guidance;
5. security and privacy impact analysis;
6. MSRV and feature impact; and
7. evidence that the change serves at least one reusable consumer problem.

Breaking an alpha-target contract requires an explicit ADR or equivalent decision record. Legacy-compatible APIs may be deprecated only after the replacement is documented, exercised, and available for a reasonable migration period.
