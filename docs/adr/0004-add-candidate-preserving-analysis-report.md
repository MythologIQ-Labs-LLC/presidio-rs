# ADR 0004: Add a Bounded Candidate-Preserving Analysis Report

- **Status:** Accepted
- **Date:** 2026-07-27
- **Decision owners:** MythologIQ Labs LLC maintainers

## Context

The legacy `AnalyzerEngine::analyze` method resolves overlaps and applies the configured threshold before returning results. That behavior is useful for existing consumers but discards candidate-level evidence before downstream policy can inspect it.

Changing `analyze` directly would be a breaking semantic change. Implementing a second recognition path would also create permanent drift risk because validator, context, scoring, filtering, and rejection behavior could diverge between the two APIs.

The current `PatternRecognizer` contract does not expose authoritative recognizer identity, recognizer version, locale, or mechanism metadata. Derived identity would be collision-prone and falsely authoritative.

## Decision

Add `AnalyzerEngine::analyze_report` and `analyze_report_with_options` alongside the existing API.

Both legacy and report APIs consume one internal raw-candidate collection pipeline. The report contains:

- validated candidate `Finding` values before thresholding and overlap resolution;
- an explicitly named `legacy_compatible_results` projection derived from the same bounded raw candidates;
- typed, non-plaintext report-construction issues;
- engine version separated from recognizer version; and
- explicit status indicating whether candidate or issue limits were reached.

The existing `analyze` method remains source and behavior compatible and uses the same internal recognition path without report limits.

## Candidate and compatibility boundary

`AnalysisReport::candidates` contains only findings that satisfy the new span and confidence invariants. A raw candidate that cannot satisfy those invariants produces a typed issue rather than a false validated finding.

`legacy_compatible_results` applies the current legacy threshold and overlap behavior to the report's bounded raw candidate stream. When candidate collection reaches its configured limit, the projection is partial and `AnalysisStatus` makes that explicit.

The compatibility projection is not the permanent security resolution policy. Future policy-oriented resolution requires a separate ADR.

## Provenance boundary

This slice does not assign `RecognizerId` or recognizer version because `PatternRecognizer` does not provide them.

Valid pattern names may be retained as bounded `MetadataId` evidence. Invalid legacy pattern metadata produces a typed issue but does not cause an otherwise valid candidate to be discarded.

The crate version is recorded only as `engine_version`. A later backend-neutral recognizer contract will add authoritative recognizer metadata.

## Shared recognition pipeline

Regex iteration, validator decisions, context enhancement, and raw score production occur exactly once. The legacy result list and the report are projections of the same raw candidates.

Differential tests compare `analyze` and `legacy_compatible_results` across built-in recognition, filtering, validators, context, overlaps, equal-score ties, and adversarial custom recognizers.

## Resource limits

Report-oriented analysis has deterministic limits for:

- accepted raw candidates processed; and
- detailed issues retained.

Reaching a limit is never silent. `AnalysisStatus` reports candidate truncation and issue-detail truncation separately.

These limits protect the report API from empty or highly repetitive regular expressions and other configurations that could produce large candidate or diagnostic collections. They do not silently change the existing legacy API in this additive slice.

## Security semantics

The report exposes `has_issues` rather than `is_complete`. An empty issue list means only that no report-construction issue was retained. It does not mean that all PII was detected, that recognition is exhaustive, or that output is safe to release.

The aggregate report is serialize-only until a versioned wire schema and validated deserialization policy exist.

## Consequences

### Positive

- Candidate preservation is available without changing existing consumers.
- Recognition behavior cannot drift between two implementations.
- Compatibility is measurable through differential tests.
- Unknown recognizer provenance remains unknown.
- Invalid metadata does not drop valid detections.
- Resource truncation and construction issues are observable.

### Costs

- The report may contain fewer validated findings than raw legacy candidates when legacy configuration contains invalid scores or spans.
- The report temporarily carries a legacy compatibility projection rather than a final resolver output.
- Built-in and custom recognizer identity remains unavailable until the recognizer contract is upgraded.
- The legacy API remains unbounded until a separately reviewed compatibility and migration decision is made.

## Alternatives considered

### Duplicate the legacy recognizer loop

Rejected because compatibility would depend on maintaining two implementations indefinitely.

### Derive recognizer IDs from entity and pattern names

Rejected because the result describes patterns rather than recognizers, sanitization can collide, custom recognizers can share names, and long names can make report behavior diverge from legacy behavior.

### Treat the crate version as recognizer version

Rejected because engine and recognizer version are different provenance dimensions.

### Call the report `complete` when it has no issues

Rejected because consumers could interpret completeness as exhaustive PII detection or a safe-release signal.

## Validation

This decision is effective when:

- formatting, Clippy, tests, documentation, package, MSRV, DCO, and dependency audit pass;
- report and legacy APIs use one raw-candidate pipeline;
- exact overlapping candidates are preserved before legacy resolution;
- differential tests prove compatibility for supported mechanisms and tie behavior;
- invalid confidence and metadata produce typed issues without fabricated provenance;
- candidate and issue limits are deterministic and tested; and
- the report exposes serialization without unvalidated deserialization.

## Follow-up

The next architectural slice should introduce explicit backend-neutral recognizer metadata and construction validation. Only after that contract exists should reports carry authoritative recognizer IDs, versions, locales, mechanisms, and evaluation receipt identifiers.
