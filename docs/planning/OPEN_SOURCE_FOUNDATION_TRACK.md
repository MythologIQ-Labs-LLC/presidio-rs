# Stealth Open-Source Foundation Track

## Purpose

The primary strategic objective of private development is to create a credible foundation for future open-source use.

The project remains private so architecture, naming, provenance, security, maintenance, and release boundaries can be corrected before public expectations become irreversible. Privacy is a development condition, not the product goal.

Public release remains a separate explicit decision. The foundation should be strong enough that publication, collaboration, or a clean public export can occur without first rewriting the project around open-source expectations.

## Operating principle

> Build privately, design publicly, disclose deliberately.

Every merged change should be understandable, testable, attributable, supportable, and useful outside any one internal consumer.

Stealth development means:

- repository access remains restricted;
- no public launch, marketing, or availability claim is implied;
- documentation is written for an unfamiliar external Rust consumer;
- architecture avoids confidential product assumptions and internal-only coupling;
- licenses, provenance, security practices, and contribution terms are maintained continuously;
- release and package metadata remain capable of supporting a clean public export; and
- public-readiness weaknesses are treated as development work rather than postponed launch chores.

## Foundation workstreams

The following workstreams apply across every development phase.

### 1. Public API and compatibility

The project must maintain:

- coherent naming and extension boundaries;
- additive migration where practical;
- explicit deprecation and removal policy;
- documented MSRV and feature behavior;
- downstream compile fixtures for representative consumers;
- semantic-version impact assessment for public API changes; and
- migration notes whenever behavior or contracts change.

A feature is not foundation-ready merely because it compiles. An unfamiliar consumer must be able to understand what is stable, transitional, legacy-compatible, or intentionally unsupported.

### 2. Contributor experience

The repository should make a high-quality contribution possible without private oral history.

Required foundations include:

- a current README and documentation index;
- small, reviewable architecture decisions;
- runnable examples and synthetic fixtures;
- clear issue and pull-request templates;
- local development and verification commands;
- contribution, conduct, security, DCO, CLA, and attribution guidance;
- documented code ownership and review expectations; and
- a path for first-time contributors that does not require access to internal products.

### 3. Security and correctness

Security-sensitive behavior must be explicit, bounded, and adversarially tested.

The foundation should include:

- fallible APIs for operations that can fail;
- exact source and coordinate semantics;
- deterministic resource limits;
- typed non-plaintext failures;
- false-positive and false-negative fixtures;
- malformed-input, Unicode, overlap, and denial-of-service tests;
- fuzzing or property tests for critical paths;
- dependency and advisory monitoring; and
- a coordinated disclosure process with realistic maintainer response capacity.

### 4. Evidence and claims

The project must distinguish:

- implemented behavior demonstrated by source and tests;
- measured behavior supported by reproducible artifacts; and
- planned behavior that is not yet available.

Accuracy, performance, security, production-readiness, compliance, and cost claims require evidence. Architecture properties must not be presented as benchmark results.

### 5. Packaging and release discipline

Private development should continuously preserve the ability to create a clean public release.

This includes:

- intentional Cargo package contents;
- correct package, repository, documentation, license, category, and keyword metadata;
- reproducible release checks;
- changelog and release-note discipline;
- tag-to-package identity;
- docs.rs compatibility;
- a clean-history export plan;
- secret, confidential-reference, and provenance audits; and
- an explicit release approval record.

### 6. Governance and maintainership

A public security-sensitive library requires more than source availability.

The project must define:

- maintainers and review ownership;
- compatibility and release authority;
- security-triage responsibility;
- expected response times that match actual capacity;
- contribution and decision escalation paths;
- succession or archive criteria; and
- conditions under which the project should collaborate, redirect, narrow, or stop.

### 7. Ecosystem fit

The project must remain under active comparison with related Rust and Presidio-compatible efforts.

Independent implementation should continue only when it provides a defensible benefit for Rust consumers. Adoption, upstream contribution, collaboration, component reuse, or migration are valid successful outcomes.

Sunk cost, repository identity, and authorship pride are not architectural differentiators.

## Merge gate for future phases

Each material pull request should answer:

1. Which reusable consumer problem does this solve?
2. Does the change belong in the core, an optional adapter, an example, or an application?
3. What public API or behavior becomes stable, transitional, or deprecated?
4. Which security, privacy, compatibility, and maintenance risks change?
5. Which tests or evidence prove the claimed behavior?
6. Which README, guide, ADR, changelog, or migration document must change?
7. Does the change improve or weaken future public release readiness?
8. Has a better existing Rust component or project emerged?

A change that cannot answer these questions should be narrowed, deferred, or kept outside the reusable core.

## Near-term priority order

The next work should prioritize foundation completeness over feature breadth:

1. fallible anonymization over document-bound findings;
2. explicit candidate-resolution policy and overlap semantics;
3. reproducible evaluation fixtures and error analysis;
4. fuzzing and property tests for spans, resolution, and anonymization;
5. downstream compile fixtures and at least two distinct consumer pilots;
6. public API inventory, semantic-version baseline, and migration guide;
7. contributor examples and a minimal custom-backend reference implementation;
8. package and clean-export rehearsal;
9. naming, licensing, provenance, and legal review; and
10. measured private-beta readiness evidence.

Semantic recognition remains optional and outside the critical path unless consumer evidence makes it necessary.

## Foundation maturity gates

### Gate A: Contributor-ready privately

- repository standards are enforceable;
- architecture and current behavior are documented;
- local verification is reproducible;
- contribution and security processes are usable;
- core tests and dependency checks are green; and
- no internal product context is required to understand the library.

### Gate B: Consumer-ready private alpha

- at least two materially different Rust consumers compile and integrate;
- compatibility fixtures exist;
- core operations have explicit failure and resource behavior;
- supported scope and limitations are evidence-backed; and
- upgrade and migration expectations are documented.

### Gate C: Public-release candidate foundation

- provenance, history, legal, security, and package audits are complete;
- evaluation, fuzzing, and benchmark evidence support published claims;
- maintainership and incident response are realistic;
- package and documentation release rehearsals pass;
- name and ecosystem positioning are approved; and
- a clean public export or visibility-change plan is accepted.

### Gate D: Publication decision

Maintainers explicitly choose one of:

- publish the complete core;
- publish a narrower component;
- collaborate with or migrate to another project;
- continue private development; or
- stop or redirect the effort.

The decision must cite evidence. A green CI badge, elapsed time, or accumulated code volume is not approval.

## Definition of success

The stealth program succeeds when the project can be evaluated as a serious open-source Rust library before it is public:

- external consumers can understand and integrate it from documentation alone;
- maintainers can support its declared scope;
- security and correctness claims match evidence;
- the API can evolve without avoidable consumer harm;
- contribution and release processes are operational; and
- publication is a controlled strategic choice rather than a rescue project for private code.
