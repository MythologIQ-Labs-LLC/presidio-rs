# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project follows semantic-versioning expectations appropriate to a pre-1.0 Rust crate.

## [Unreleased]

### Added

- Open-source-grade contribution, governance, conduct, security, and release-readiness policies.
- An open-source foundation track with contributor, consumer, security, evidence, package, governance, ecosystem, merge, and maturity gates.
- A public repository release-week runbook targeting July 30, 2026.
- Separate visibility, contributor-alpha, consumer-beta, package-publication, advertised-launch, and stable-release gates.
- CI gates for formatting, Clippy, tests, documentation, package verification, MSRV, dependency audit, and DCO sign-off.
- Standalone public-facing project documentation and expected-use-case boundaries.
- Validated structural spans, bounded open identifiers, constrained confidence values, evidence, and findings.
- Candidate-preserving `AnalysisReport` output with typed issues and deterministic candidate and issue limits.
- Authoritative recognizer metadata, strict pattern registration, stable built-in recognizer IDs, and report provenance catalogs.
- `TextDocument`, document identity, exact-content fingerprints, document-bound findings, and report validation.
- Bounded `AnalysisRequest` controls for entities, recognizers, locale, capabilities, confidence, input size, candidates, and issues.
- Object-safe backend-neutral `Recognizer` execution and validated `CandidateEmitter` output.
- Typed non-plaintext backend failures and explicit legacy-projection completeness status.
- Adversarial tests for Unicode spans, provenance, limits, document mismatch, custom backends, legacy compatibility, and failure handling.

### Changed

- Rebaselined the project from a 30-week private-first plan to public repository visibility on July 30, 2026, contributor-ready alpha by August 14, consumer-ready beta by September 25, and a package and advertised-launch decision by October 30.
- Package publication and active promotion are now explicitly separate from GitHub repository visibility.
- Package metadata remains prepared for MIT-licensed distribution without implying crates.io publication.
- Documentation separates implemented, measured, and planned capabilities.
- The governing development posture states that the open-source foundation is the primary objective.
- The release checklist now permits early public source visibility while preserving stronger later gates for evidence, compatibility, packaging, support, and stability.
- Built-in pattern recognizers now use validated metadata-backed registration.
- New request-oriented integrations use exact document binding and authoritative recognizer provenance.
- The existing analyzer, `RecognizerResult`, pattern registry, and anonymizer APIs remain available for compatibility.

### Security

- Added a coordinated vulnerability disclosure process.
- Added an explicit pre-visibility history, secret, confidential-reference, licensing, and provenance gate.
- Prevented supported custom backends from bypassing entity, span, confidence, provenance, source-binding, and candidate-limit invariants.
- Added bounded metadata and failure codes to reduce accidental plaintext capture in reports.
- Added exact-source validation before document-bound findings can be sliced or applied.

## [0.1.0] - Unreleased

Initial model-free Rust analyzer and anonymizer implementation.

[Unreleased]: https://github.com/MythologIQ-Labs-LLC/presidio-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/MythologIQ-Labs-LLC/presidio-rs/releases/tag/v0.1.0