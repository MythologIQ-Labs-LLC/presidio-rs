# ADR 0003: Stage Validated Core Types Before Engine Migration

- **Status:** Accepted
- **Date:** 2026-07-27
- **Decision owners:** MythologIQ Labs LLC maintainers

## Context

The current analyzer returns `RecognizerResult`, which exposes a closed `EntityType`, raw `usize` offsets, and an unconstrained `f32` score. Existing consumers may already compile against that contract.

The target architecture requires validated original-text spans, constrained confidence values, open entity and recognizer identifiers, recognizer provenance, and non-plaintext evidence. Replacing the analyzer return type immediately would combine vocabulary design, analyzer orchestration, resolution behavior, anonymization changes, and consumer migration in one high-risk change.

## Decision

Introduce the target value types additively before changing analyzer behavior.

The first slice adds:

- `Span` with ordered, non-empty range validation and UTF-8 source-boundary checks;
- `Confidence` constrained to finite values in `0.0..=1.0`;
- open, validated `EntityId` and `RecognizerId` values;
- non-plaintext `Evidence`;
- evidence-bearing `Finding`; and
- fallible conversion from `RecognizerResult` to `Finding`.

The existing `AnalyzerEngine::analyze`, `RecognizerResult`, `EntityType`, and anonymization APIs remain unchanged during this slice.

## Compatibility strategy

1. Existing consumers continue to use the legacy API without source changes.
2. New consumers may begin compiling against the target value types.
3. The analyzer will later gain a report-oriented API rather than silently changing the existing method.
4. Legacy APIs will be deprecated only after consumer pilots and migration fixtures demonstrate an acceptable path.
5. Removal, if ever justified, requires a separate ADR and semantic-version assessment.

## Evidence and privacy

Evidence values must not require retaining or logging matched plaintext. Pattern, validator, and context identifiers may be recorded, but the sensitive matched value remains available only through the caller-controlled original text and validated span.

## Consequences

### Positive

- Core invariants become executable before orchestration changes.
- Downstream Rust consumers can test the future contract early.
- Invalid legacy spans and scores become explicit conversion failures.
- The engine migration can be reviewed in smaller, reversible slices.

### Costs

- The crate temporarily exposes both legacy and target result models.
- Conversion cannot reconstruct provenance that the legacy analyzer never recorded.
- Some target metadata, including document identity and formal locale types, remains deferred.

## Alternatives considered

### Replace `RecognizerResult` immediately

Rejected because it creates a breaking change before the new semantics have been validated with consumers.

### Keep raw primitives until the analyzer rewrite

Rejected because raw offsets and scores allow invalid state to spread into every new subsystem.

### Hide the target types internally

Rejected because early consumer compile tests are part of the architecture validation strategy.

## Validation

This decision is effective when:

- all new value-type invariants are unit tested;
- the public crate exports the types;
- at least one integration test converts real analyzer output into a validated `Finding`;
- existing integration tests remain unchanged and pass; and
- CI passes formatting, Clippy, tests, documentation, package, MSRV, and dependency checks.

## Follow-up

The next architectural slice should introduce an additive `AnalysisReport` API that preserves all candidates and recognizer provenance before explicit resolution. It must not change overlap or anonymization semantics implicitly.
