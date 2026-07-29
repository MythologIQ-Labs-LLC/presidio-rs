# Migration Guide: Legacy Analysis to Document-Aware Requests

The legacy API remains available. Migration is recommended when consumers need exact source binding, open entities, authoritative recognizer provenance, typed backend failures, explicit resource limits, or versioned resolution.

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
use presidio::{
    AnalysisRequest, AnalyzerEngine, DocumentId, ResolutionOptions,
    ResolutionPolicy, TextDocument,
};

let document = TextDocument::new(
    DocumentId::new("request-42").expect("valid document ID"),
    "Email jane@acme.com",
);
let report = AnalyzerEngine::new()
    .analyze_request(&document, &AnalysisRequest::new())
    .expect("bounded analysis");

let resolved = report
    .resolve_for_document(
        &document,
        &ResolutionOptions::new(ResolutionPolicy::ConservativeRedaction),
    )
    .expect("document-bound resolution");

for output in resolved.resolution().resolved() {
    println!("{output:?}");
}
```

The request-oriented path preserves validated candidates and records status, issues, recognizer metadata, exact document binding, legacy-projection completeness, and a separate versioned resolution report.

## Resolution migration state

ADR 0009 defines three implemented additive policies:

- `report_all/v1` preserves all qualifying candidates;
- `best_candidate/v1` performs deterministic precedence-based selection; and
- `conservative_redaction/v1` unions connected strict-overlap components without inventing a mixed source entity.

`AnalysisReport::resolve_for_document` validates the exact source, rejects candidate-truncated analysis, retains analyzer version and analysis status, and returns a `ResolvedAnalysisReport` containing the pure `ResolutionReport`.

Semantics are documented in:

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

### 4. Inspect authoritative candidates

Use `report.candidates()` rather than `legacy_compatible_results()` when raw evidence, provenance, or evaluation matters.

Use `finding.slice_document(&document)` instead of slicing directly from unchecked offsets.

### 5. Resolve against the exact document

Choose one named policy and resolve through the report:

```rust
let integrated = report.resolve_for_document(
    &document,
    &ResolutionOptions::new(ResolutionPolicy::BestCandidate),
)?;
let resolution = integrated.resolution();
```

This call:

1. validates report and candidate bindings against the exact document;
2. rejects an unbound or mismatched report;
3. rejects analysis that stopped at the candidate limit;
4. preserves raw candidates separately;
5. applies the selected policy and version;
6. retains analyzer version, analysis status, issue count, and document binding; and
7. returns bounded non-plaintext decision evidence.

Do not overwrite the raw candidate collection. Do not infer resolution from vector order. Do not reproduce legacy overlap behavior in application code and later label it `BestCandidate`.

`ConservativeRedaction` is the intended safe input for irreversible coverage-oriented redaction. `BestCandidate` is a precision-oriented selection policy and can intentionally select a contained higher-confidence span over a larger lower-confidence span.

The lower-level `resolve_candidates` function remains available for validated `Finding` collections outside `AnalysisReport`, but report-based consumers should prefer `resolve_for_document`.

### 6. Inspect status and issues

A successful `analyze_request` can still return retained issues or issue-detail truncation. `ResolvedAnalysisReport::analysis_status()` and `analysis_issue_count()` preserve that context.

Candidate truncation is fail-closed by `resolve_for_document`. Pure resolution candidate and output limits are hard errors. Decision-evidence truncation is explicit through `ResolutionStatus` while resolved output remains complete.

Applications must still decide whether recognizer failures or retained issues block, require review, reduce confidence, or are acceptable for their use case.

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
- [`examples/resolution_policies.rs`](../../examples/resolution_policies.rs)

### 8. Delay transformation migration until the fallible API lands

The current anonymizer accepts legacy `RecognizerResult` values. Fallible atomic anonymization over `ResolvedAnalysisReport` or its contained `ResolutionReport` is the next secure-alpha development round.

Until that API exists:

- keep existing anonymization on the legacy path where compatibility is required;
- use request-oriented analysis and resolution for inspection, provenance, evaluation and policy decisions;
- do not manually transform overlapping candidates;
- do not pass unresolved request-oriented candidates into the legacy anonymizer; and
- do not present the legacy compatibility projection as the permanent transformation contract.

## Mixed-mode migration

A consumer may run both paths temporarily:

1. use `analyze_request` to collect authoritative candidates and evidence;
2. resolve with one explicit policy through `resolve_for_document`;
3. compare `legacy_compatible_results()` with the existing `analyze` output;
4. record open-entity, threshold, ordering, containment, partial-overlap, and resolution differences;
5. retain the existing anonymizer for current production behavior; and
6. migrate transformation only after the document-bound fallible anonymizer is available.

This allows evidence gathering and policy evaluation without changing transformation behavior in the same deployment.

## Compatibility checklist

Before switching a consumer:

- [ ] document IDs are opaque and stable for the source lifetime;
- [ ] every report is resolved through the exact document;
- [ ] open entities are handled without assuming legacy representability;
- [ ] candidate and issue limits are appropriate for expected input size;
- [ ] backend failure and truncation policy is explicit;
- [ ] recognizer metadata is retained where provenance matters;
- [ ] Unicode byte offsets are tested with realistic synthetic fixtures;
- [ ] existing overlap and anonymization behavior is regression-tested;
- [ ] one explicit resolution policy is selected;
- [ ] conservative redaction is used where coverage matters more than single-candidate precision; and
- [ ] logs do not expose plaintext, source fingerprints, or sensitive identifiers unnecessarily.

## Deprecation posture

No legacy API is deprecated by this guide. Deprecation requires a separately reviewed decision after the replacement analysis, resolution, and transformation paths have consumer evidence, migration coverage, and a documented support window.
