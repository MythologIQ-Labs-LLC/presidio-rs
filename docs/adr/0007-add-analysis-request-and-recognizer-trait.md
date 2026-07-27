# ADR 0007: Add Analysis Requests and a Backend-Neutral Recognizer Trait

- **Status:** Accepted
- **Date:** 2026-07-27
- **Decision owners:** MythologIQ Labs LLC maintainers

## Context

The library now has validated findings, candidate-preserving reports, authoritative recognizer metadata, and exact source-document binding. The analyzer still executes pattern recognizers through implementation-specific loops, however, and the existing string APIs cannot express locale, capability, recognizer selection, input limits, or backend failures.

Publishing a recognizer trait before the document contract existed would have created an interface unable to guarantee source coordinates or document identity. Those prerequisites now exist.

The new execution boundary must support future structural, dictionary, semantic, and consumer-defined recognizers without forcing heavy dependencies into the default crate. It must also preserve existing Rust consumers of `PatternRecognizer`, `RecognizerRegistry`, and the string-based analyzer APIs.

## Decision

Introduce `AnalysisRequest` as the validated request contract for document-aware backend execution.

The request owns:

- an optional open entity allowlist;
- an optional explicit recognizer allowlist;
- an optional locale or country identifier;
- capabilities available to recognizers;
- the minimum confidence for the legacy-compatible projection;
- a maximum original-input byte length;
- a global maximum candidate count; and
- a maximum retained issue count.

The default request selects metadata marked default-enabled and applies a 1 MiB input limit, the existing 10,000-candidate limit, and the existing 100-issue limit.

Introduce the object-safe `Recognizer: Send + Sync` trait:

```rust
pub trait Recognizer: Send + Sync {
    fn metadata(&self) -> &RecognizerMetadata;

    fn supports(&self, request: &AnalysisRequest) -> bool;

    fn recognize(
        &self,
        document: &TextDocument<'_>,
        request: &AnalysisRequest,
        emitter: &mut CandidateEmitter<'_, '_>,
    ) -> Result<(), RecognitionError>;
}
```

The analyzer provides `analyze_request(document, request)` as a new fallible entry point.

## Candidate emission

Recognizers do not construct `Finding` values directly. They emit candidate primitives through `CandidateEmitter`.

The emitter enforces:

- the entity was declared by recognizer metadata;
- the span is ordered, non-empty, in bounds, and on UTF-8 boundaries for the exact document;
- confidence is finite and within `0.0..=1.0`;
- the authoritative recognizer ID is attached;
- the exact document binding is attached; and
- the remaining global candidate capacity is respected.

This prevents custom backends from bypassing core source, provenance, and resource invariants.

## Pattern-recognizer adaptation

Strict metadata-backed pattern recognizers execute through a borrowed `PatternRecognizerAdapter` implementing the same backend-neutral trait.

The adapter is constructed at request time from the existing pattern registry. This gives built-in and strict custom pattern recognizers the same execution contract as arbitrary backends without changing the public fields or storage layout of `PatternRecognizer`.

Legacy `RecognizerRegistry::add(PatternRecognizer)` registrations have no authoritative metadata and therefore cannot participate safely in the new request path. They are skipped and reported through one typed `LegacyRecognizersSkipped` issue. They remain fully available through the existing legacy analyzer APIs.

Unknown provenance is not upgraded merely to preserve feature parity.

## Backend storage

Arbitrary trait implementations are stored as `Arc<dyn Recognizer>` on `AnalyzerEngine` during this phase.

This is a transitional choice. It avoids changing the legacy registry's public pattern-oriented behavior while real consumer pilots exercise the trait. Duplicate recognizer IDs are rejected across strict pattern metadata and custom backends.

A later immutable builder and registry ADR may unify storage after the trait contract and downstream requirements stabilize.

## Request selection

A recognizer is selected when:

- it is explicitly named, or its metadata is default-enabled when no explicit recognizer list exists;
- at least one declared entity intersects the request entity allowlist, when supplied;
- its locale list is empty or contains the requested locale, when supplied; and
- every required capability is present in the request.

Unsupported recognizers are skipped without being treated as failures. A selected recognizer that cannot execute returns a typed `RecognitionError`.

## Failure semantics

Input-size rejection is fatal and returns `AnalysisExecutionError::InputTooLarge` before recognizer execution.

Recognizer failures are non-fatal report issues containing:

- recognizer ID;
- stable failure category;
- bounded non-plaintext error code; and
- retryability.

This preserves evidence while leaving fail-open, fail-closed, block, review, or retry decisions to the consumer.

Candidate-emission contract violations become typed invalid-candidate recognition failures.

## Legacy compatibility projection

Backend-neutral recognizers may emit open entity identifiers outside the legacy `EntityType` taxonomy.

The request report projects representable findings into `RecognizerResult`. Unrepresentable open entities remain in the authoritative candidate list and set `AnalysisStatus::legacy_projection_incomplete()`.

The legacy projection must never be interpreted as the complete authoritative result when that status is true.

## Compatibility

The following remain unchanged:

- `AnalyzerEngine::analyze`;
- `AnalyzerEngine::analyze_report`;
- `AnalyzerEngine::analyze_document`;
- `RecognizerResult`;
- `PatternRecognizer` struct literals;
- legacy and strict pattern registration; and
- existing anonymization behavior.

The new path is additive. Consumers can migrate to `AnalysisRequest` without being forced to rewrite existing pattern integrations immediately.

## Security and resource consequences

### Positive

- Custom backends cannot emit unbound or invalid findings through the supported contract.
- Input and candidate growth are bounded before or during execution.
- Backend failures use stable non-plaintext codes rather than arbitrary messages.
- Locale and capability requirements are explicit and inspectable.
- Open entity identifiers are preserved without falsifying the legacy projection.
- Heavy semantic runtimes remain optional consumer-supplied backends.

### Costs

- The analyzer temporarily owns two recognizer storage models.
- Legacy registrations do not participate in the request path.
- A recognizer may emit partial candidates before returning an error; the report preserves both.
- The default 1 MiB limit may require adjustment for real consumers.
- Capability declarations do not provision or verify external resources themselves.
- Request selection currently performs small linear scans rather than indexed lookup.

## Deliberate deferrals

This slice does not yet add:

- an immutable `AnalyzerBuilder`;
- runtime deprecation of mutable registration;
- normalized views or source-offset maps;
- asynchronous recognizers;
- cancellation or deadlines;
- per-recognizer candidate budgets;
- calibrated confidence across recognizer families;
- fallible anonymization over document-bound findings; or
- a stable serialized request or report schema.

These require evidence from multiple Rust consumers rather than speculative API surface.

## Alternatives considered

### Return `Vec<Finding>` directly from each recognizer

Rejected because recognizers could bypass document, entity, confidence, provenance, and candidate-limit validation.

### Require legacy pattern registrations to synthesize metadata

Rejected because fabricated IDs and versions would recreate the provenance flaw corrected in earlier phases.

### Replace the registry with trait objects immediately

Rejected because it would create a broad compatibility and storage migration before consumer pilots validate the trait.

### Treat every backend failure as fatal

Rejected because consumers own fail-open and fail-closed policy. The report must preserve failures without deciding product policy.

### Permit arbitrary error strings

Rejected because error messages can accidentally capture sensitive source content and create unstable serialized contracts.

## Validation

This decision is effective when:

- default requests execute authoritative built-in recognizers;
- strict pattern recognizers and custom backends use the same trait and emitter contract;
- oversized input is rejected before execution;
- candidate limits are global and observable;
- open custom entities remain authoritative while marking the legacy projection incomplete;
- locale and capability requirements affect selection deterministically;
- legacy registrations remain operational through old APIs and are explicitly skipped by the new path;
- backend and invalid-candidate failures appear as typed issues;
- duplicate recognizer IDs are rejected across both storage models; and
- formatting, Clippy, tests, documentation, package verification, Rust 1.74, DCO, and dependency audit pass.

## Follow-up

The next architectural slice should add fallible anonymization over document-bound findings and explicit resolution policy. Immutable builder and registry unification should wait until at least two materially different Rust consumers exercise the request and recognizer contracts.
