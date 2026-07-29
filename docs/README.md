# Project Documentation

## Architecture

- [Active architecture](architecture/ARCHITECTURE.md)
- [ADR 0001: Develop with open-source-grade practices](adr/0001-private-open-source-posture.md)
- [ADR 0002: Backend-neutral core with optional capability adapters](adr/0002-backend-neutral-core-and-optional-adapters.md)
- [ADR 0003: Stage validated core types before engine migration](adr/0003-stage-core-types-before-engine-migration.md)
- [ADR 0004: Add candidate-preserving analysis reports](adr/0004-add-candidate-preserving-analysis-report.md)
- [ADR 0005: Add recognizer metadata and validated registration](adr/0005-add-recognizer-metadata-and-validated-registration.md)
- [ADR 0006: Bind findings and reports to exact text documents](adr/0006-bind-findings-to-text-documents.md)
- [ADR 0007: Add analysis requests and a backend-neutral recognizer trait](adr/0007-add-analysis-request-and-recognizer-trait.md)
- [ADR 0008: Stage the secure functional alpha through an evidence-gated pipeline](adr/0008-stage-secure-functional-alpha-through-evidence-gated-pipeline.md)
- [ADR 0009: Version explicit candidate-resolution policies](adr/0009-version-explicit-candidate-resolution.md)

## Implemented architecture status

The current `main` branch includes:

- structurally validated spans and bounded open identifiers;
- candidate-preserving reports with typed issues and deterministic limits;
- authoritative recognizer metadata and strict pattern registration;
- exact document identity and source-content binding;
- bounded `AnalysisRequest` selection and resource controls;
- an object-safe backend-neutral `Recognizer` trait;
- validated candidate emission; and
- typed non-plaintext backend failures.

The legacy analyzer and anonymizer APIs remain available. The request-oriented path is additive and is the target integration surface for new Rust consumers.

The active implementation round is explicit candidate resolution. See [First Secure-Alpha Development Round](planning/FIRST_DEVELOPMENT_ROUND.md).

The secure functional alpha critical path is:

1. Presidio evidence and alpha contract;
2. explicit candidate resolution;
3. fallible document-bound anonymization;
4. explainability, context, and conservative defaults;
5. reproducible evaluation, historical regressions, fuzzing, and a downstream fixture; and
6. two materially different consumer validations.

## API and migration

- [Public API status](api/PUBLIC_API_STATUS.md)
- [Migration from legacy analysis to document-aware requests](api/MIGRATION_GUIDE.md)
- [Resolution conformance matrix](testing/RESOLUTION_CONFORMANCE_MATRIX.md)
- [Strict pattern recognizer example](../examples/strict_pattern_recognizer.rs)
- [Custom backend example](../examples/custom_backend.rs)

## Contributor onboarding

- [First contribution guide](contributing/FIRST_CONTRIBUTION.md)
- [Contribution policy](../CONTRIBUTING.md)
- [Code of Conduct](../CODE_OF_CONDUCT.md)
- [Contributor License Agreement](../CONTRIBUTOR_LICENSE_AGREEMENT.md)

## Development planning

- [Secure functional alpha roadmap](planning/DEVELOPMENT_PLAN.md)
- [First secure-alpha development round](planning/FIRST_DEVELOPMENT_ROUND.md)
- [Public repository release week](planning/PUBLIC_RELEASE_WEEK.md)
- [Open-source foundation track](planning/OPEN_SOURCE_FOUNDATION_TRACK.md)
- [Development risk and assumption register](planning/RISK_REGISTER.md)

## Release evidence and operations

- [July 30 public visibility evidence](release/2026-07-30-PUBLIC-VISIBILITY-EVIDENCE.md)
- [Tuesday automated audit evidence](release/2026-07-27-TUESDAY-AUDIT-EVIDENCE.md)
- [Wednesday release-candidate evidence](release/2026-07-30-WEDNESDAY-CANDIDATE.md)
- [Public visibility and rollback runbook](release/PUBLIC_VISIBILITY_RUNBOOK.md)
- [Source and dependency provenance inventory](release/SOURCE_AND_DEPENDENCY_PROVENANCE.md)

The repository became public on July 28, 2026. Evidence records remain authoritative for the exact tested commits and release operations.

## Current roadmap gates

Public visibility remains separate from a secure transformation boundary, crates.io publication, active promotion, production certification, and stable API commitments.

- public repository visibility: completed July 28, 2026;
- public foundation alpha: August 3, 2026;
- secure functional alpha: August 21, 2026;
- consumer-validated public beta: September 4, 2026;
- hardened public beta: September 18, 2026; and
- package and promotion decision: October 2, 2026.

The February 26, 2027 horizon remains a broader maturity checkpoint.

## Research and external landscape

- [Presidio archaeology and secure alpha model](research/PRESIDIO_ARCHAEOLOGY_AND_ALPHA_MODEL.md)
- [Presidio resolution decision ledger](research/PRESIDIO_RESOLUTION_DECISION_LEDGER.md)
- [Active Rust privacy landscape watch](research/ACTIVE_LANDSCAPE_WATCH.md)
- [Parallel efforts and architectural lessons](research/PARALLEL_EFFORTS_AND_LESSONS.md)

The Presidio archaeology program converts upstream architecture, changelog, issues, failures, fixes, security changes, evaluation practices, and governance history into explicit adopt, adapt, reject, defer, or investigate decisions for the Rust project.

The build-versus-adopt-versus-collaborate question remains active. The Rust privacy landscape is watched continuously and escalated when another project could invalidate planned work or provide a better path for consumers.

## Governance and release

- [Project governance](../GOVERNANCE.md)
- [Security policy](../SECURITY.md)
- [Open-source release checklist](../OPEN_SOURCE_RELEASE_CHECKLIST.md)

Public access does not weaken architecture, security, provenance, evidence, documentation, package, compatibility, or maintainership standards. It changes when the work is visible, not what responsible completion requires.
