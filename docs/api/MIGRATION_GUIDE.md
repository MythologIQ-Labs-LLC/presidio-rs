# Migration Guide: Legacy Analysis to Document-Aware Requests

The legacy API remains available. Migration is recommended when consumers need exact source binding, open entities, authoritative recognizer provenance, typed backend failures, or explicit resource limits.

## Legacy shape

```rust
use presidio::AnalyzerEngine;

let analyzer = AnalyzerEngine::new();
let results = analyzer.analyze("Email jane@acme.com", None);
```

This returns `Vec<RecognizerResult>` after the current legacy resolution behavior.

Limitations:

- the source has no document identity;
- findings are not bound to exact source bytes;
- recognizer provenance may be unknown;
- entities are limited to `EntityType`;
- backend failures are not represented;
- resource limits are not part of the request; and
- overlap behavior is applied before the caller can inspect all candidates.

## Target shape

```rust
use presidio::{AnalysisRequest, AnalyzerEngine, DocumentId, TextDocument};

let document = TextDocument::new(
    DocumentId::new("request-42").expect("valid document ID"),
    "Email jane@acme.com",
);
let request = AnalysisRequest::new();
let report = AnalyzerEngine::new()
    .analyze_request(&document, &request)
    .expect("bounded analysis");

report
    .validate_for_document(&document)
    .expect("matching source document");

for finding in report.candidates() {
    let matched = finding
        .slice_document(&document)
        .expect("validated source binding");
    println!("{}: {matched}", finding.entity());
}
```

The request-oriented path preserves validated candidates and records status, issues, recognizer metadata, exact document binding, and legacy-projection completeness.

## Resolution migration state

ADR 0009 has accepted the version-1 semantics for three future additive policies:

- `report_all/v1` preserves all qualifying candidates;
- `best_candidate/v1` performs deterministic precedence-based selection; and
- `conservative_redaction/v1` unions connected overlap components without inventing a mixed source entity.

The implementation is tracked by #41 and #42. Until those issues merge, the crate does not expose a public resolution API. Consumers should continue to treat `AnalysisReport::candidates()` as raw authoritative evidence and `legacy_compatible_results()` as compatibility output.

Accepted semantics are documented in:

- [ADR 0009](../adr/0009-version-explicit-candidate-resolution.md); and
- [Resolution conformance matrix](../testing/RESOLUTION_CONFORMANCE_MATRIX.md).

## Step-by-step migration

### 1. Assign document identity

Use an opaque, bounded `DocumentId` that does not expose customer or subject information.

```rust
let document = TextDocument::new(DocumentId::new("job-7f31")?, source_text);
```

`TextDocument` borrows plaintext. Findings and reports retain only the document binding, including byte length and content fingerprint.

### 2. Replace entity enums with open identifiers where appropriate

Built-in entities remain available through `EntityType`, but request filters and custom backends use `EntityId`.

```rust
use presidio::{EntityId, EntityType};

let built_in = EntityId::from(EntityType::Email);
let custom = EntityId::new("EMPLOYEE_ID")?;
```

Do not assume every open entity can be represented by `RecognizerResult`.

### 3. Build an explicit request

Start with `AnalysisRequest::new()`, which selects default-enabled strict recognizers and applies bounded defaults. Add explicit filters or limits only when the application owns their policy.

Check the current rustdoc for available entity, recognizer, locale, capability, confidence, input, candidate and issue controls.

### 4. Consume authoritative candidates

Use `report.candidates()` rather than `legacy_compatible_results()`.

Before transformation or release, validate the report against the exact source document:

```rust
report.validate_for_document(&document)?;
```

Use `finding.slice_document(&document)` instead of slicing directly from unchecked offsets.

### 5. Resolve candidates explicitly when the API lands

The accepted migration sequence is:

1. inspect or retain the raw candidate collection;
2. choose one named policy and version;
3. resolve into a separate report;
4. validate that report against the same document; and
5. pass only the resolved report into the future document-bound anonymizer.

Do not overwrite the raw candidate collection. Do not infer resolution from vector order. Do not reproduce legacy overlap behavior in application code and later label it `BestCandidate`.

`ConservativeRedaction` is the intended safe input for irreversible coverage-oriented redaction. `BestCandidate` is a precision-oriented selection policy and can intentionally select a contained higher-confidence span over a larger lower-confidence span.

### 6. Inspect status and issues

A successful `analyze_request` can still return a report with retained issues or limit status. Applications must decide whether those conditions fail closed, require review, reduce confidence, or are acceptable for their use case.

Do not interpret a lack of retained issue details as proof that analysis was exhaustive when an issue limit was reached.

Future resolution status will similarly distinguish complete output from candidate, resolved-output, or decision-evidence limits.

### 7. Register recognizers through authoritative paths

Replace legacy registration:

```rust
registry.add(pattern_recognizer);
```

with strict metadata-backed registration:

```rust
registry.add_with_metadata(metadata, pattern_recognizer)?;
```

For non-pattern implementations, implement `Recognizer` and register through `AnalyzerEngine::add_backend`.

See:

- [`examples/strict_pattern_recognizer.rs`](../../examples/strict_pattern_recognizer.rs)
- [`examples/custom_backend.rs`](../../examples/custom_backend.rs)

### 8. Delay transformation migration until the fallible API lands

The current anonymizer accepts legacy `RecognizerResult` values. Fallible anonymization over a document-bound `ResolutionReport` remains the round after #42.

Until that API exists:

- keep existing anonymization on the legacy path where compatibility is required;
- use request-oriented reports for inspection, provenance, evaluation and policy decisions;
- do not manually transform overlapping candidates without an explicit application-owned policy;
- do not pass unresolved request-oriented candidates into the legacy anonymizer; and
- do not present the legacy compatibility projection as the permanent transformation contract.

## Mixed-mode migration

A consumer may run both paths temporarily:

1. use `analyze_request` to collect authoritative candidates and evidence;
2. compare `legacy_compatible_results()` with the existing `analyze` output;
3. record any open-entity, threshold, ordering, containment, or partial-overlap differences;
4. retain the existing anonymizer for current production behavior;
5. adopt explicit resolution after #42; and
6. migrate transformation only after the document-bound fallible anonymizer is available.

This allows evidence gathering without changing output behavior in the same deployment.

## Compatibility checklist

Before switching a consumer:

- [ ] document IDs are opaque and stable for the source lifetime;
- [ ] every report is validated against the exact document before use;
- [ ] open entities are handled without assuming legacy representability;
- [ ] candidate and issue limits are appropriate for expected input size;
- [ ] backend failure and truncation policy is explicit;
- [ ] recognizer metadata is retained where provenance matters;
- [ ] Unicode byte offsets are tested with realistic synthetic fixtures;
- [ ] existing overlap and anonymization behavior is regression-tested;
- [ ] an explicit resolution policy is selected when the API is available;
- [ ] conservative redaction is used where coverage matters more than single-candidate precision; and
- [ ] logs do not expose plaintext, source fingerprints, or sensitive identifiers unnecessarily.

## Deprecation posture

No legacy API is deprecated by this guide. Deprecation requires a separately reviewed decision after the replacement analysis, resolution, and transformation paths have consumer evidence, migration coverage, and a documented support window.
