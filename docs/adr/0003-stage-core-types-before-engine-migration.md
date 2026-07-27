# ADR 0003: Stage Validated Core Types Before Engine Migration

- **Status:** Accepted
- **Date:** 2026-07-27
- **Decision owners:** MythologIQ Labs LLC maintainers

## Context

The current analyzer returns `RecognizerResult`, which exposes a closed `EntityType`, raw `usize` offsets, and an unconstrained `f32` score. Existing consumers may already compile against that contract.

The target architecture requires source-valid spans, constrained confidence values, open entity identifiers, honest recognizer provenance, and bounded non-plaintext evidence metadata. Replacing the analyzer return type immediately would combine vocabulary design, analyzer orchestration, resolution behavior, anonymization changes, and consumer migration in one high-risk change.

## Decision

Introduce the target value types additively before changing analyzer behavior.

The first slice adds:

- `Span` as an ordered, non-empty structural byte range;
- source-aware validation through `Span::new_for` and `Span::validate_for`;
- `Confidence` constrained to finite values in `0.0..=1.0`;
- open, validated `EntityId` and `RecognizerId` values;
- bounded `MetadataId` values for evidence metadata;
- serialize-only, evidence-bearing `Finding`; and
- fallible conversion from `RecognizerResult` to `Finding`.

The existing `AnalyzerEngine::analyze`, `RecognizerResult`, `EntityType`, and anonymization APIs remain unchanged during this slice.

## Source-binding boundary

A `Span` created without source text proves only ordering and non-emptiness. It does not claim that the offsets belong to a particular document or fall on UTF-8 character boundaries.

Consumers must validate a structural span against the intended source before indexing it. A later `TextDocument` contract will bind findings to document identity and original-text coordinates so findings cannot be applied to the wrong input silently.

## Provenance boundary

Legacy `RecognizerResult` values do not contain recognizer identity or recognizer version. Conversion therefore leaves recognizer provenance unknown and records only `Evidence::LegacyResult`.

The library version must not be represented as a recognizer version. Engine, recognizer, policy, and report-schema versions are separate concepts and will be modeled separately when their contracts are introduced.

## Evidence and privacy

Evidence does not copy matched plaintext. Pattern, validator, and context metadata use validated, length-bounded `MetadataId` values rather than arbitrary strings.

This reduces accidental plaintext capture and unbounded serialized output, but callers remain responsible for assigning identifiers that describe metadata rather than sensitive values.

## Serialization boundary

The first contract is serialize-only. Deserialization and durable interchange are deferred until the project defines an explicit report-schema version, unknown-field and unknown-variant behavior, and validation rules for reconstructed aggregate objects.

Rust source extensibility mechanisms such as `#[non_exhaustive]` do not by themselves create a forward-compatible wire format.

## Compatibility strategy

1. Existing consumers continue to use the legacy API without source changes.
2. New consumers may compile against the target value types.
3. The analyzer gains report-oriented APIs additively rather than silently changing the existing method.
4. Legacy APIs are deprecated only after consumer pilots and migration fixtures demonstrate an acceptable path.
5. Removal, if ever justified, requires a separate ADR and semantic-version assessment.

## Consequences

### Positive

- Core invariants become executable before orchestration changes.
- Downstream Rust consumers can test the future contract early.
- Invalid legacy spans and scores become explicit conversion failures.
- Unknown provenance remains unknown instead of becoming confidently false.
- Evidence metadata has deterministic identifier and size constraints.

### Costs

- The crate temporarily exposes both legacy and target result models.
- Structural spans still require source validation at use sites until `TextDocument` exists.
- Conversion cannot reconstruct provenance that the legacy analyzer never recorded.
- Deserialization, document identity, locale, and formal version types remain deferred.

## Alternatives considered

### Replace `RecognizerResult` immediately

Rejected because it creates a breaking change before the new semantics have been validated with consumers.

### Claim source validity from structural offsets

Rejected because a byte range cannot prove which document it belongs to without source or document identity.

### Invent legacy recognizer provenance

Rejected because fabricated audit data is more dangerous than explicitly unknown data.

### Publish a deserialize contract immediately

Rejected because the aggregate schema, version policy, and unknown-variant behavior are not yet designed.

## Validation

This decision is effective when:

- structural and source-aware span behavior are separately tested;
- invalid legacy spans and scores fail conversion;
- legacy conversion leaves recognizer provenance unknown;
- evidence metadata is typed and bounded;
- serde-enabled builds expose serialization without accepting unvalidated aggregate deserialization;
- the public crate exports the types; and
- CI passes formatting, Clippy, tests, documentation, package, MSRV, DCO, and dependency checks.

## Follow-up

The next architectural slice should introduce an additive candidate report from one shared recognition pipeline. It must preserve legacy behavior through an explicitly named compatibility projection, avoid invented provenance, use typed status and issue semantics, and impose deterministic report limits.
