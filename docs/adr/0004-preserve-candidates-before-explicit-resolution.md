# ADR 0004: Preserve Candidates Before Explicit Resolution

- **Status:** Accepted
- **Date:** 2026-07-27

## Context

The legacy `AnalyzerEngine::analyze` method combines recognition, thresholding, overlap resolution, and sorting. Consumers cannot inspect candidates removed by overlap handling or identify the recognizer and pattern that produced a result.

`PatternRecognizer` is publicly constructed through struct literals, so adding required metadata fields directly would be source-breaking.

## Decision

Add `AnalyzerEngine::analyze_report` without changing `analyze`.

The report path validates the threshold, emits validated `Finding` values, records recognizer and pattern provenance, preserves every valid threshold-qualified candidate, records invalid candidate construction as non-fatal issues, and requires an explicit `ResolutionPolicy` before selection.

The first policies are `AllCandidates` and `HighestConfidenceNonOverlapping`. The latter reproduces legacy behavior and is not presented as universally conservative redaction policy.

`RecognizerRegistry` stores metadata beside the existing recognizer collection. Built-ins receive explicit IDs. `add_with_metadata` supports stable consumer IDs and rejects duplicates. The existing `add` method generates registry-local IDs and remains source-compatible.

## Boundaries

- The legacy analyzer and anonymizer behavior is unchanged.
- Validator rejection and below-threshold candidates are not included in reports.
- Invalid spans and confidence values are reported without matched plaintext.
- Context evidence is deferred until the enhancer returns trustworthy structured explanations.
- Generated custom IDs must not be persisted as durable configuration identifiers.

## Consequences

Recognition and resolution become separate concepts, provenance survives analysis, and existing consumers remain compatible. The temporary cost is parallel recognizer and metadata storage plus two analyzer surfaces during migration.

## Validation

- overlapping candidates survive into reports;
- explicit resolution can reproduce legacy behavior;
- findings carry recognizer and pattern provenance;
- invalid candidate scores become issues;
- invalid thresholds fail report construction;
- duplicate explicit IDs are rejected; and
- all existing and new quality gates pass.

## Follow-up

The next review should choose among `TextDocument` identity, structured context evidence, and a conservative overlap policy backed by adversarial fixtures. The legacy analyzer must not be routed through the report path until behavioral equivalence is measured.
