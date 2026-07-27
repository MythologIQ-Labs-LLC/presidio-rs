# ADR 0001: Develop Privately with Open-Source-Grade Practices

- **Status:** Accepted
- **Date:** 2026-07-26
- **Decision owners:** MythologIQ Labs LLC maintainers

## Context

`presidio-rs` is intended to become a reusable Rust library with an independently defensible architecture, clear contribution boundaries, and credible security practices.

Making the repository public immediately would create irreversible licensing, history, naming, maintenance, and product-positioning consequences before the project has completed its architecture, provenance, evaluation, and release-readiness work.

Keeping the project conventionally proprietary during development would create a different risk: internal coupling, weak documentation, undocumented decisions, unreviewed dependencies, and a difficult public conversion later.

## Decision

The project will remain private while being developed according to open-source-grade engineering and community standards.

The repository will use:

- the MIT License for project source on the development branch and after approved merge;
- public-safe, standalone documentation;
- explicit expected use cases, non-goals, and evidence boundaries;
- CI-enforced formatting, linting, tests, documentation, package integrity, MSRV, dependency auditing, and DCO;
- a Contributor License Agreement and DCO process;
- attribution and provenance requirements;
- security disclosure and conduct policies;
- changelog, compatibility, governance, and architecture-decision discipline; and
- a separate public-release checklist and approval decision.

Repository privacy does not permit confidential product assumptions to define the reusable library's public contract. Internal consumers may influence requirements only when those requirements are independently justified for the project.

## Public release boundary

Public release is not implied by this decision.

A future visibility change or clean public export requires explicit approval after reviewing:

- project name and ecosystem positioning;
- licensing and contributor terms;
- repository and source provenance;
- confidential history and metadata;
- architecture and public API stability;
- security and adversarial testing;
- detection evaluation and claims;
- maintenance capacity; and
- the public versus commercial product boundary.

The existing private repository should not automatically be made public. A clean, audited public export may be preferable.

## Consequences

### Positive

- Public-ready habits begin before public scrutiny.
- Internal coupling and undocumented assumptions are easier to detect.
- Licensing, attribution, and contribution expectations are established early.
- CI and release discipline improve private development immediately.
- Publication remains a reversible strategic decision until explicitly approved.

### Costs

- Maintainers carry documentation and governance overhead before a public community exists.
- MIT licensing inside a private repository may require additional access and employment-policy clarity.
- CLA and governance language require legal review before public third-party contributions are accepted.
- Some public-facing references, URLs, and project naming may need revision before release.

### Risks

- Developers may mistakenly interpret MIT files as authorization to redistribute the private repository. Repository access controls and organizational policy remain binding until approved public distribution.
- Standards may become ceremonial unless CI, reviews, and release decisions enforce them.
- The project may accumulate public promises faster than implementation evidence. Claims discipline remains mandatory.

## Alternatives considered

### Make the repository public immediately

Rejected because public licensing, history, name, and maintenance expectations would become difficult or impossible to reverse before readiness is established.

### Keep the project proprietary until feature-complete

Rejected because late conversion would preserve internal coupling and defer provenance, documentation, security, and contribution discipline until they are more expensive to repair.

### Maintain separate private and public repositories immediately

Deferred. Dual-repository synchronization would add operational complexity before a stable public release boundary exists.

## Validation

This decision is considered effective when:

- the private repository adopts the documented governance baseline;
- CI gates pass;
- subsequent changes follow DCO and review requirements;
- architecture decisions are recorded for material changes; and
- no public release occurs without completing the explicit release checklist.

## Revisit conditions

Revisit this decision when:

- the project reaches a credible public `0.1.0` boundary;
- external collaboration is materially blocked by privacy;
- commercial strategy requires a different license or contribution model;
- project naming changes; or
- maintenance capacity changes substantially.