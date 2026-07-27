# Project Documentation

## Architecture

- [Target architecture](architecture/ARCHITECTURE.md)
- [ADR 0001: Develop privately with open-source-grade practices](adr/0001-private-open-source-posture.md)
- [ADR 0002: Backend-neutral core with optional capability adapters](adr/0002-backend-neutral-core-and-optional-adapters.md)
- [ADR 0003: Stage validated core types before engine migration](adr/0003-stage-core-types-before-engine-migration.md)
- [ADR 0004: Add candidate-preserving analysis reports](adr/0004-add-candidate-preserving-analysis-report.md)
- [ADR 0005: Add recognizer metadata and validated registration](adr/0005-add-recognizer-metadata-and-validated-registration.md)
- [ADR 0006: Bind findings and reports to exact text documents](adr/0006-bind-findings-to-text-documents.md)
- [ADR 0007: Add analysis requests and a backend-neutral recognizer trait](adr/0007-add-analysis-request-and-recognizer-trait.md)

## Implemented architecture status

The private development branch now includes:

- structurally validated spans and bounded open identifiers;
- candidate-preserving reports with typed issues and deterministic limits;
- authoritative recognizer metadata and strict pattern registration;
- exact document identity and source-content binding;
- bounded `AnalysisRequest` selection and resource controls;
- an object-safe backend-neutral `Recognizer` trait;
- validated candidate emission; and
- typed non-plaintext backend failures.

The existing legacy analyzer and anonymizer APIs remain available. The request-oriented path is additive and is the target integration surface for new Rust consumers.

The next architectural slice is fallible anonymization over document-bound findings with an explicit resolution policy.

## Development planning

- [Stealth open-source foundation track](planning/OPEN_SOURCE_FOUNDATION_TRACK.md)
- [Multi-phase development plan](planning/DEVELOPMENT_PLAN.md)
- [Development risk and assumption register](planning/RISK_REGISTER.md)

The open-source foundation is the primary development objective. Repository privacy controls timing and disclosure; it does not change the intended contributor, consumer, API, security, evidence, package, and maintainership standards.

## Research and external landscape

- [Active Rust privacy landscape watch](research/ACTIVE_LANDSCAPE_WATCH.md)
- [Parallel efforts and architectural lessons](research/PARALLEL_EFFORTS_AND_LESSONS.md)

## Governance and release

- [Project governance](../GOVERNANCE.md)
- [Security policy](../SECURITY.md)
- [Contribution guide](../CONTRIBUTING.md)
- [Open-source release checklist](../OPEN_SOURCE_RELEASE_CHECKLIST.md)

## Current planning baseline

The active planning baseline is a 30-week private development program from August 3, 2026 through February 26, 2027.

The target outcome is a measured, multi-consumer open-source foundation developed in private, followed by an explicit publication, collaboration, continued-private-development, or redirection decision. Public release is not assumed or automatic.

Architecture is reviewed continuously throughout the program. Each phase includes design review, risk review, evidence review, consumer impact assessment, foundation-readiness assessment, and stop or redirect criteria.

The build-versus-adopt-versus-collaborate question remains active throughout development. The external Rust privacy landscape is watched weekly, reviewed deeply each month, and escalated immediately when a material change could invalidate planned work or provide a better path for consumers.
