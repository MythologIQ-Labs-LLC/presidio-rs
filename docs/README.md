# Project Documentation

## Architecture

- [Target architecture](architecture/ARCHITECTURE.md)
- [ADR 0001: Develop with open-source-grade practices](adr/0001-private-open-source-posture.md)
- [ADR 0002: Backend-neutral core with optional capability adapters](adr/0002-backend-neutral-core-and-optional-adapters.md)
- [ADR 0003: Stage validated core types before engine migration](adr/0003-stage-core-types-before-engine-migration.md)
- [ADR 0004: Add candidate-preserving analysis reports](adr/0004-add-candidate-preserving-analysis-report.md)
- [ADR 0005: Add recognizer metadata and validated registration](adr/0005-add-recognizer-metadata-and-validated-registration.md)
- [ADR 0006: Bind findings and reports to exact text documents](adr/0006-bind-findings-to-text-documents.md)
- [ADR 0007: Add analysis requests and a backend-neutral recognizer trait](adr/0007-add-analysis-request-and-recognizer-trait.md)

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

The immediate post-visibility architecture work is explicit candidate resolution followed by fallible anonymization over document-bound findings.

## API and migration

- [Public API status](api/PUBLIC_API_STATUS.md)
- [Migration from legacy analysis to document-aware requests](api/MIGRATION_GUIDE.md)
- [Strict pattern recognizer example](../examples/strict_pattern_recognizer.rs)
- [Custom backend example](../examples/custom_backend.rs)

## Contributor onboarding

- [First contribution guide](contributing/FIRST_CONTRIBUTION.md)
- [Contribution policy](../CONTRIBUTING.md)
- [Code of Conduct](../CODE_OF_CONDUCT.md)
- [Contributor License Agreement](../CONTRIBUTOR_LICENSE_AGREEMENT.md)

## Development planning

- [Public repository release week](planning/PUBLIC_RELEASE_WEEK.md)
- [Rebaselined development roadmap](planning/DEVELOPMENT_PLAN.md)
- [Open-source foundation track](planning/OPEN_SOURCE_FOUNDATION_TRACK.md)
- [Development risk and assumption register](planning/RISK_REGISTER.md)

## Release evidence and operations

- [July 30 public visibility evidence](release/2026-07-30-PUBLIC-VISIBILITY-EVIDENCE.md)
- [Tuesday automated audit evidence](release/2026-07-27-TUESDAY-AUDIT-EVIDENCE.md)
- [Wednesday release-candidate evidence](release/2026-07-30-WEDNESDAY-CANDIDATE.md)
- [Public visibility and rollback runbook](release/PUBLIC_VISIBILITY_RUNBOOK.md)
- [Source and dependency provenance inventory](release/SOURCE_AND_DEPENDENCY_PROVENANCE.md)

The repository became public on July 28, 2026. Evidence records remain authoritative for the exact tested commits and release operations.

## Accelerated release baseline

The repository became publicly readable on **Tuesday, July 28, 2026**, two days ahead of the original target.

Public visibility remains separate from contributor-ready alpha, consumer-ready beta, crates.io publication, an advertised launch, production certification, and stable API commitments.

Current target gates:

- public repository visibility: completed July 28, 2026;
- contributor-ready public alpha: August 3, 2026;
- correctness, evaluation, and initial fuzzing: August 21, 2026;
- consumer-ready public beta: September 4, 2026; and
- package and advertised-launch decision: October 2, 2026.

The original February 26, 2027 horizon remains a broader maturity checkpoint rather than the first possible publication date.

## Research and external landscape

- [Active Rust privacy landscape watch](research/ACTIVE_LANDSCAPE_WATCH.md)
- [Parallel efforts and architectural lessons](research/PARALLEL_EFFORTS_AND_LESSONS.md)

The build-versus-adopt-versus-collaborate question remains active. The Rust privacy landscape is watched weekly and escalated immediately when another project could invalidate planned work or provide a better path for consumers.

## Governance and release

- [Project governance](../GOVERNANCE.md)
- [Security policy](../SECURITY.md)
- [Open-source release checklist](../OPEN_SOURCE_RELEASE_CHECKLIST.md)

Public access does not weaken architecture, security, provenance, evidence, documentation, package, compatibility, or maintainership standards. It changes when the work is visible, not what responsible completion requires.
