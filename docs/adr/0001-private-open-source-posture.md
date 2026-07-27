# ADR 0001: Develop Privately with Open-Source-Grade Practices

- **Status:** Accepted
- **Date:** 2026-07-26
- **Decision owners:** MythologIQ Labs LLC maintainers

## Context

`presidio-rs` is intended to become a reusable Rust library with an independently defensible architecture, clear contribution boundaries, and credible security practices.

Making the repository public immediately would create irreversible licensing, history, naming, maintenance, and product-positioning consequences before the project has completed its architecture, provenance, evaluation, and release-readiness work.

Keeping the project conventionally proprietary during development would create a different risk: internal coupling, weak documentation, undocumented decisions, unreviewed dependencies, and a difficult public conversion later.

## Strategic objective

The primary development objective is to create a solid foundation for future open-source use.

Repository privacy is a temporary operating condition that permits architecture, naming, provenance, security, maintenance, and release boundaries to mature before public expectations become irreversible. It is not a signal that the reusable core should be designed as a proprietary or internal-only library.

Public release remains a separate controlled decision. The foundation should nevertheless be strong enough that publication, collaboration, or a clean public export can occur without first rebuilding the project around open-source expectations.

The governing operating principle is:

> Build privately, design publicly, disclose deliberately.

The detailed cross-cutting requirements and maturity gates are maintained in the [Stealth Open-Source Foundation Track](../planning/OPEN_SOURCE_FOUNDATION_TRACK.md).

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
- changelog, compatibility, governance, and architecture-decision discipline;
- continuous contributor, package, release, and clean-export readiness work; and
- a separate public-release checklist and approval decision.

Repository privacy does not permit confidential product assumptions to define the reusable library's public contract. Internal consumers may influence requirements only when those requirements are independently justified for the project.

## Development and merge implications

Material changes must improve or preserve the project's future open-source foundation.

A change should identify:

- the reusable consumer problem it solves;
- why it belongs in the core, an adapter, an example, or an application;
- public API and compatibility effects;
- security, privacy, evidence, and maintenance consequences;
- required README, guide, ADR, changelog, and migration updates; and
- whether a better existing Rust project or component changes the decision to build independently.

Feature breadth alone is not a sufficient reason to merge. Contributor usability, supportability, evidence, and release safety are first-class development outcomes.

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
- Contributor experience and clean-export readiness become continuous work.
- Publication remains a reversible strategic decision until explicitly approved.

### Costs

- Maintainers carry documentation and governance overhead before a public community exists.
- MIT licensing inside a private repository may require additional access and employment-policy clarity.
- CLA and governance language require legal review before public third-party contributions are accepted.
- Some public-facing references, URLs, and project naming may need revision before release.
- Foundation work can slow visible feature accumulation, even when it reduces future release and maintenance risk.

### Risks

- Developers may mistakenly interpret MIT files as authorization to redistribute the private repository. Repository access controls and organizational policy remain binding until approved public distribution.
- Standards may become ceremonial unless CI, reviews, and release decisions enforce them.
- The project may accumulate public promises faster than implementation evidence. Claims discipline remains mandatory.
- Private consumer urgency may attempt to displace contributor, compatibility, security, or release-readiness work.

## Alternatives considered

### Make the repository public immediately

Rejected because public licensing, history, name, and maintenance expectations would become difficult or impossible to reverse before readiness is established.

### Keep the project proprietary until feature-complete

Rejected because late conversion would preserve internal coupling and defer provenance, documentation, security, and contribution discipline until they are more expensive to repair.

### Treat open-source readiness as launch-only work

Rejected because API coherence, evidence, provenance, contributor experience, and maintainership cannot be repaired reliably in a final documentation sprint.

### Maintain separate private and public repositories immediately

Deferred. Dual-repository synchronization would add operational complexity before a stable public release boundary exists.

## Validation

This decision is considered effective when:

- the private repository adopts the documented governance baseline;
- CI gates pass;
- subsequent changes follow DCO and review requirements;
- architecture decisions are recorded for material changes;
- README, documentation, changelog, contributor, security, package, and release artifacts stay current;
- private consumers do not silently define the reusable core;
- the foundation maturity gates are reviewed during phase exits; and
- no public release occurs without completing the explicit release checklist.

## Revisit conditions

Revisit this decision when:

- the project reaches a credible public `0.1.0` boundary;
- external collaboration is materially blocked by privacy;
- commercial strategy requires a different license or contribution model;
- project naming changes;
- maintenance capacity changes substantially; or
- an existing project becomes a demonstrably better foundation for the intended consumers.
