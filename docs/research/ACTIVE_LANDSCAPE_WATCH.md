# Active Rust Privacy Landscape Watch

## Status

- **State:** Active
- **Started:** 2026-07-27
- **Cadence:** Weekly watch, monthly deep review, immediate escalation on material change
- **Applies to:** The build-versus-adopt-versus-collaborate decision for `presidio-rs`

## Purpose

The possibility that an existing Rust project is a better technical, ecosystem, or maintenance choice is not a one-time Phase 0 question. It is a standing strategic risk throughout development.

The project will actively watch the Rust ecosystem for developments that could:

- eliminate the need for independent implementation;
- provide a reusable component or compatible dependency;
- create a credible collaboration or contribution path;
- invalidate planned architecture or roadmap work;
- materially weaken project differentiation;
- establish a stronger ecosystem standard;
- change licensing, maintenance, or security assumptions; or
- justify narrowing, pausing, redirecting, or stopping the project.

The goal is not to defend sunk cost. The goal is to produce the best sustainable outcome for Rust consumers.

## Watch scope

Monitor projects and releases related to:

- Rust-native PII detection and anonymization;
- Microsoft Presidio ports or compatibility layers;
- redaction and privacy-enforcement libraries;
- secret scanners and mature rule engines;
- recognizer registries and entity-taxonomy projects;
- tokenization, normalization, and source-offset mapping;
- NER and semantic-recognition runtimes suitable for Rust;
- pseudonymization and privacy-preserving transformation;
- evaluation corpora and PII benchmarking tools;
- WASM, embedded, `no_std`, FFI, and constrained-runtime privacy tooling; and
- Rust supply-chain, fuzzing, semver, and security tooling relevant to the project.

Initial named projects include Microsoft Presidio, Presidio Research, `presidio-analyzer`, `redact-core`, `cloakrs-core`, Gitleaks, and newly discovered projects with comparable scope.

## Cadence

### Weekly watch

Perform a lightweight scan for:

- new projects or crates;
- significant releases;
- major architecture changes;
- new recognizers, NER capabilities, or supported platforms;
- maintenance or governance changes;
- adoption signals;
- security advisories;
- license changes;
- repository archival or abandonment; and
- collaboration opportunities.

A weekly scan should produce no notification or planning churn when nothing material changed.

### Monthly deep review

Re-score the strongest alternatives against the maintained comparison criteria. Review source where necessary rather than relying on README claims.

### Immediate event-triggered review

Start an architecture review without waiting for the next cadence when a material trigger is detected.

## Comparison criteria

Each credible alternative should be assessed against:

1. architecture and separation of concerns;
2. correctness and span semantics;
3. recognizer extensibility;
4. entity and locale coverage;
5. evaluation quality and reproducibility;
6. error and provenance reporting;
7. anonymization safety;
8. dependency graph and build complexity;
9. offline and constrained-runtime suitability;
10. MSRV and platform support;
11. API stability and semver discipline;
12. license and provenance;
13. security posture;
14. maintenance activity and bus factor;
15. ecosystem adoption;
16. documentation quality;
17. collaboration feasibility; and
18. migration cost for known Rust consumers.

Claims about speed, memory, accuracy, or safety must be independently reproducible before they influence the decision.

## Material-change triggers

A finding is material when one or more of the following is true:

- another project satisfies the project’s required consumer constraints with acceptable quality;
- an alternative reaches materially stronger adoption or maintenance capacity;
- a project implements a planned high-cost capability before `presidio-rs` does;
- a reusable crate can replace a planned internal subsystem;
- an alternative offers stronger evaluation, provenance, or compatibility evidence;
- a license or governance change creates or removes a collaboration path;
- a project becomes abandoned, archived, compromised, or otherwise unsuitable;
- a new ecosystem standard makes the planned API or taxonomy strategically isolated;
- continued independent work would primarily duplicate maintained functionality; or
- the expected cost of migration becomes lower than the expected cost of continued independent development.

## Required response to a material finding

Within five working days:

1. record the evidence and source revision;
2. update the comparison matrix and risk register;
3. identify affected roadmap work;
4. estimate adopt, collaborate, integrate, migrate, continue, and stop options;
5. consult impacted Rust consumers;
6. hold an architecture review; and
7. record the decision in an ADR or a documented no-change rationale.

Affected implementation work should pause when continuing it could create substantial avoidable duplication or lock-in.

## Decision outcomes

A review may conclude:

- continue independently;
- adopt an existing project;
- contribute upstream;
- collaborate on shared infrastructure;
- depend on a narrower reusable component;
- maintain a compatibility adapter;
- merge or transfer work;
- narrow the project’s scope;
- defer a capability; or
- stop independent development.

Stopping or collaborating is a successful risk response when it produces a better outcome than preserving repository identity.

## Watch record

Material findings should be appended below or linked to an issue or ADR.

| Date | Project or event | Materiality | Decision or next action |
|---|---|---|---|
| 2026-07-27 | Standing watch established | High | Weekly conditional monitoring enabled; monthly source-level review required |
