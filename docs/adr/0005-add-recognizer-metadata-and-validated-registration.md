# ADR 0005: Add Recognizer Metadata and Validated Registration

- **Status:** Accepted
- **Date:** 2026-07-27
- **Decision owners:** MythologIQ Labs LLC maintainers

## Context

The candidate-report slice deliberately left recognizer identity unknown because the existing `PatternRecognizer` structure contains patterns, context, and an optional validator but no authoritative identity, version, locale, mechanism, capability, attribution, or evaluation metadata.

Deriving identity from pattern names was rejected because one recognizer may contain multiple patterns, names may collide, lossy normalization is unsafe, and custom recognizers do not share the crate version.

The project also needs to support future non-pattern Rust consumers and recognition backends. Metadata therefore cannot be defined in terms of regular-expression implementation details.

At the same time, a complete public execution trait would currently be premature. The intended trait depends on contracts that do not yet exist, including `TextDocument`, `AnalysisRequest`, source-coordinate mapping, and typed recognition errors. Publishing a thinner trait now would create a misleading compatibility surface that would need to break later.

## Decision

Introduce backend-neutral `RecognizerMetadata` with:

- stable recognizer ID;
- independent recognizer version;
- supported open entity identifiers;
- supported locale or country identifiers;
- recognition mechanism;
- required capability identifiers;
- default-enabled status;
- source or prior-art attribution identifier; and
- optional evaluation receipt identifier.

Metadata construction validates that at least one entity is declared and that entity, locale, and capability lists contain no duplicates.

Introduce a strict metadata-backed registration path for `PatternRecognizer` while retaining the existing registration path for source compatibility.

## Registration paths

### Legacy registration

`RecognizerRegistry::add(PatternRecognizer)` remains available.

Legacy registrations:

- preserve existing source behavior;
- may contain historical configuration that does not satisfy new invariants;
- do not receive authoritative recognizer identity or version; and
- continue to surface report-construction issues when invalid candidate metadata or scores are encountered.

Unknown provenance remains unknown.

### Metadata-backed registration

`RecognizerRegistry::add_with_metadata(metadata, recognizer)` validates:

- metadata declares exactly the entity emitted by the pattern recognizer;
- recognition mechanism agrees with validator presence;
- at least one pattern exists;
- pattern IDs are bounded and unique;
- base scores are finite and within `0.0..=1.0`;
- regular expressions do not match empty input;
- context terms are non-empty, bounded, free of control characters, and unique under current case handling; and
- recognizer IDs are unique within the registry.

`Pattern::try_new` provides strict construction. `Pattern::new` remains as a compatibility constructor and continues to validate only regex compilation.

## Built-in recognizers

Built-in recognizers use metadata-backed registration.

Their recognizer version is explicitly the crate version because the built-in rule definitions ship and release with the crate. This does not imply that consumer-provided recognizers share the crate version.

The current US phone-number, SSN, and ITIN recognizers declare `en-US` support. Locale-agnostic structured recognizers leave the locale list empty.

Built-ins declare a stable prior-art attribution identifier for Microsoft Presidio without claiming affiliation or endorsement.

## Report provenance

Each validated finding produced by a metadata-backed recognizer carries its stable recognizer ID.

`AnalysisReport` contains one metadata snapshot for each metadata-backed recognizer that emitted at least one raw candidate. Consumers can resolve a finding's recognizer ID through `AnalysisReport::recognizer_metadata` to inspect version, locales, mechanism, capabilities, attribution, and evaluation receipt.

The metadata catalog is stored once per report rather than repeating the same metadata on every finding.

Legacy findings retain no recognizer ID and produce no fabricated metadata entry.

## Backend-neutral boundary

`RecognizerMetadata` is backend-neutral. Pattern, structural, dictionary, semantic, and custom mechanisms can all use the same metadata contract.

This ADR does **not** publish the final backend-neutral execution trait. That trait will be introduced only after `TextDocument`, `AnalysisRequest`, candidate emission, capability selection, and recognition error semantics are defined well enough to support multiple implementations without immediate redesign.

## Security and resource considerations

Strict registration rejects expressions that match empty input because they can generate candidates at every position and create disproportionate resource use.

Metadata and evidence fields remain bounded identifiers rather than arbitrary free-form strings. This limits accidental plaintext capture and serialized growth, although callers remain responsible for assigning identifiers rather than sensitive values.

Registration failure is explicit and fallible. Invalid custom configuration is not silently accepted into the authoritative metadata path.

## Consequences

### Positive

- Reports gain accurate built-in and custom recognizer provenance.
- Engine version and recognizer version are separate concepts.
- Built-in metadata becomes inspectable by downstream Rust consumers.
- Invalid strict registrations fail before analysis.
- Stable recognizer IDs can support future configuration, filtering, evaluation, and compatibility tests.
- The metadata model can be reused by future semantic or structural backends.

### Costs

- The registry temporarily maintains both legacy and metadata-backed registration paths.
- Pattern recognizers still use the existing closed `EntityType` during execution.
- Locale identifiers are declared metadata, not yet request-time locale negotiation.
- Evaluation receipt identifiers are optional until the evaluation program produces durable receipts.
- The final backend-neutral execution trait remains deferred.

## Alternatives considered

### Add metadata fields directly to `PatternRecognizer`

Rejected because the public struct is constructed with struct literals. Adding required fields would break existing consumers.

### Continue deriving IDs from pattern names

Rejected because the result would be collision-prone, pattern-specific rather than recognizer-specific, and falsely authoritative.

### Store full metadata on every finding

Rejected because it duplicates identical data and expands report size unnecessarily. Findings carry an ID and the report carries a metadata catalog.

### Publish a metadata-only `Recognizer` trait

Rejected because a trait that exposes metadata but cannot express execution does not provide a meaningful backend boundary.

### Publish the final execution trait now

Rejected because document, request, capability, candidate, and error contracts remain unsettled.

## Validation

This decision is effective when:

- built-in recognizers register through the strict metadata path;
- strict pattern construction rejects invalid IDs, regexes, scores, and empty matches;
- registration rejects entity and mechanism mismatch;
- duplicate recognizer IDs fail deterministically;
- built-in findings carry authoritative recognizer IDs;
- reports expose metadata snapshots for participating recognizers;
- custom recognizer versions remain distinct from engine version;
- legacy registrations retain unknown provenance; and
- formatting, Clippy, tests, documentation, package verification, Rust 1.74, DCO, and dependency audit pass.

## Follow-up

The next architecture slice should introduce `TextDocument` and document identity. Once source binding and request semantics exist, the project can define the backend-neutral execution trait and migrate registry storage toward immutable trait objects without inventing incomplete interfaces.
