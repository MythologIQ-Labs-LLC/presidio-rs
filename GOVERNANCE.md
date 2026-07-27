# Project Governance

`presidio-rs` is maintained by MythologIQ Labs LLC as an independently governed Rust project.

The repository is currently private. Its engineering, documentation, review, and release practices are intentionally designed to support a future public community without requiring a rushed visibility change.

## Principles

Project decisions prioritize:

1. correctness at privacy and security boundaries;
2. explicit limitations over inflated capability claims;
3. reproducible evidence over intuition or benchmark theater;
4. small, stable, composable interfaces;
5. offline and embeddable operation for the model-free core;
6. clear provenance for code, patterns, datasets, models, and documentation;
7. compatibility and migration discipline; and
8. maintainable scope over feature accumulation.

## Roles

### Maintainers

Maintainers are appointed by MythologIQ Labs LLC. Maintainers may:

- set project direction and release scope;
- review and merge changes;
- approve public API and dependency changes;
- manage security reports and disclosures;
- appoint or remove other maintainers;
- decline contributions that do not fit the project; and
- enforce contribution and conduct policies.

Maintainer authority carries a duty to explain material decisions, avoid unsupported claims, and protect contributor attribution.

### Contributors

Contributors submit changes under the project's contribution terms. Contribution does not automatically grant maintainer status or decision authority.

Consistent, high-quality contributors may be invited to take on broader review or maintenance responsibilities.

## Decision making

Routine changes may be approved by one maintainer after required checks pass.

The following require explicit maintainer review and a written rationale in the pull request or an architecture decision record:

- public API changes;
- breaking behavior changes;
- new runtime or build dependencies;
- cryptographic or pseudonymization changes;
- normalization, offset, overlap, or anonymization semantics;
- default recognizers or confidence-score changes;
- model-runtime or semantic-recognition integrations;
- MSRV changes;
- license, CLA, governance, or security-policy changes; and
- claims about measured performance, accuracy, security, or compliance.

Maintainers may require an issue or design document before accepting a large implementation.

## Architecture decisions

Material architectural decisions should be recorded under `docs/adr/` using a short decision record containing:

- context;
- decision;
- alternatives considered;
- consequences;
- evidence and assumptions; and
- follow-up validation.

An accepted decision may be revisited when evidence or requirements change. Architecture is a maintained decision system, not sacred sediment.

The active target architecture and development program are maintained in:

- [Target architecture](docs/architecture/ARCHITECTURE.md);
- [Multi-phase development plan](docs/planning/DEVELOPMENT_PLAN.md);
- [Development risk and assumption register](docs/planning/RISK_REGISTER.md); and
- [Parallel efforts and architectural lessons](docs/research/PARALLEL_EFFORTS_AND_LESSONS.md).

During active development, architecture is reviewed weekly, risks and assumptions every two weeks, consumer compatibility every four weeks, and the parallel-project landscape every eight weeks.

## Releases

Until `1.0.0`, the project follows Rust and Cargo semantic-versioning expectations while reserving normal `0.x` flexibility. Maintainers should still avoid unnecessary breaking changes.

A release requires:

- passing CI on the release commit;
- an updated changelog;
- package-content verification;
- dependency and license review;
- security review proportionate to the change;
- documentation matching actual capability;
- explicit classification of implemented, measured, and planned claims; and
- release notes describing compatibility and known limitations.

A release must not claim production readiness solely because tests pass.

## Security response

Security reports follow [SECURITY.md](SECURITY.md). Maintainers may temporarily restrict discussion or delay public details while coordinating a fix.

Security fixes should receive independent review whenever practical.

## Compatibility

Public APIs, serialized formats, entity identifiers, score semantics, and anonymization output can become dependencies for downstream users. Changes to these surfaces require a compatibility assessment.

Deprecation is preferred over immediate removal when practical. Breaking changes must be documented with migration guidance.

## Claims and evidence

The project uses three claim states:

- **implemented:** demonstrated by the current source and tests;
- **measured:** supported by reproducible evaluation or benchmark artifacts; and
- **planned:** proposed but not implemented.

Maintainers should reject documentation or promotion that blurs these states.

## Commercial and project independence

MythologIQ Labs LLC may build commercial services, support, integrations, or products around the project. The core project's technical claims and contribution attribution must remain accurate regardless of commercial interests.

The project must not be architected around an undisclosed internal consumer. Product-specific policy and orchestration should remain outside the reusable core unless independently justified by the project's public scope.

## Amendments

Maintainers may update this governance document through normal review. Material changes to contributor rights, licensing, security reporting, or decision authority require an explicit rationale.
