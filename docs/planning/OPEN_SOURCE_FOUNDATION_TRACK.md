# Open-Source Foundation Track

## Purpose

The primary strategic objective is to create a credible open-source Rust foundation.

The repository is targeted to become publicly readable on **Thursday, July 30, 2026**. Public visibility accelerates feedback and collaboration, but it does not convert incomplete evidence into production readiness or create an automatic package-release or advertising commitment.

The project must be safe and authorized to expose on July 30, then continue foundation work in public.

## Operating principle

> Build deliberately, expose honestly, promote only by decision.

Every merged change should be understandable, testable, attributable, supportable, and useful outside any one internal consumer.

The accelerated open-source posture means:

- repository visibility becomes public after the release-week gate;
- no launch campaign, package publication, or production claim is implied;
- documentation is written for an unfamiliar external Rust consumer;
- architecture avoids confidential product assumptions and internal-only coupling;
- licenses, provenance, security practices, and contribution terms remain continuously maintained;
- public weaknesses are visible and tracked instead of hidden behind repository access controls; and
- release maturity is communicated through explicit gates rather than one overloaded word such as “public.”

## Release decisions are separate

The project distinguishes:

1. **Public repository visibility**: source and history are publicly readable.
2. **Contributor-ready public alpha**: unfamiliar contributors can build, test, understand, and extend the project.
3. **Consumer-ready public beta**: multiple consumers validate the supported contracts and compatibility behavior.
4. **Package release**: a version is published to crates.io or another registry.
5. **Advertised launch**: maintainers actively promote the project and accept the resulting attention and support load.
6. **Stable release**: long-term compatibility expectations are explicitly accepted.

Only public repository visibility is committed for July 30.

## Foundation workstreams

### 1. Public API and compatibility

The project must maintain:

- coherent naming and extension boundaries;
- additive migration where practical;
- explicit deprecation and removal policy;
- documented MSRV and feature behavior;
- downstream compile fixtures for representative consumers;
- semantic-version impact assessment for public API changes; and
- migration notes whenever behavior or contracts change.

A feature is not foundation-ready merely because it compiles. An unfamiliar consumer must understand what is stable-for-alpha, transitional, legacy-compatible, experimental, deprecated, or unsupported.

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

The project distinguishes:

- implemented behavior demonstrated by source and tests;
- measured behavior supported by reproducible artifacts; and
- planned behavior that is not yet available.

Accuracy, performance, security, production readiness, compliance, and cost claims require evidence. Architecture properties must not be presented as benchmark results.

Public visibility does not authorize stronger claims. It makes weak claims easier for strangers to notice, which is useful if somewhat rude.

### 5. Packaging and release discipline

Public source development should continuously preserve the ability to create a clean package and release.

This includes:

- intentional Cargo package contents;
- correct package, repository, documentation, license, category, and keyword metadata;
- reproducible release checks;
- changelog and release-note discipline;
- tag-to-package identity;
- docs.rs compatibility;
- a clean-history or clean-export plan;
- secret, confidential-reference, and provenance audits; and
- explicit package and launch approval records.

### 6. Governance and maintainership

A public security-sensitive library requires more than source availability.

The project must define:

- maintainers and review ownership;
- compatibility and release authority;
- public issue and security-triage responsibility;
- expected response times that match actual capacity;
- contribution and decision escalation paths;
- succession or archive criteria; and
- conditions under which the project should collaborate, redirect, narrow, or stop.

### 7. Ecosystem fit

The project remains under active comparison with related Rust and Presidio-compatible efforts.

Independent implementation should continue only when it provides a defensible benefit for Rust consumers. Adoption, upstream contribution, collaboration, component reuse, or migration are valid successful outcomes.

Sunk cost, repository identity, and authorship pride are not architectural differentiators.

## Merge gate for future phases

Each material pull request should answer:

1. Which reusable consumer problem does this solve?
2. Does the change belong in the core, an optional adapter, an example, or an application?
3. What public API or behavior becomes stable-for-alpha, transitional, experimental, deprecated, or unsupported?
4. Which security, privacy, compatibility, and maintenance risks change?
5. Which tests or evidence prove the claimed behavior?
6. Which README, guide, ADR, changelog, migration document, or evaluation receipt must change?
7. Does the change improve or weaken future package and launch readiness?
8. Has a better existing Rust component or project emerged?

A change that cannot answer these questions should be narrowed, deferred, or kept outside the reusable core.

## Near-term priority order

The next work prioritizes public safety and foundation completeness over feature breadth:

1. complete the July 30 visibility safety, history, provenance, naming, documentation, and operations gate;
2. verify anonymous clone, CI, security reporting, templates, and branch protection after visibility;
3. add contributor examples, ownership documentation, API inventory, and migration guidance;
4. define explicit candidate-resolution policy and overlap semantics;
5. add fallible anonymization over document-bound findings;
6. create reproducible evaluation fixtures and error analysis;
7. add fuzzing and property tests for spans, requests, resolution, and anonymization;
8. add downstream compile fixtures and two distinct consumer pilots;
9. rehearse package publication without publishing; and
10. decide crates.io publication and advertising from evidence.

Semantic recognition remains optional and outside the critical path unless consumer evidence makes it necessary.

## Foundation maturity gates

### Gate V: Public visibility safety

Target: **July 30, 2026**

- the intended public tree and history contain no known secrets or unauthorized confidential material;
- source, fixtures, patterns, documentation, and dependencies have acceptable licensing and provenance;
- naming and non-affiliation language are reviewed;
- public-facing documentation is accurate about early-stage limitations;
- CI and documented local verification pass;
- public issue, contribution, and security-reporting paths are usable;
- maintainers and the visibility-change operator are identified; and
- the visibility or clean-export runbook and rollback plan are recorded.

This gate permits public inspection. It does not certify package, beta, production, or support maturity.

### Gate A: Contributor-ready public alpha

Target: **August 14, 2026**

- repository standards are enforceable;
- architecture and current behavior are documented;
- local verification is reproducible;
- runnable extension examples exist;
- contribution and security processes are usable;
- ownership and triage responsibilities are explicit;
- core tests and dependency checks are green; and
- no internal product context is required to understand the library.

### Gate B: Consumer-ready public beta

Target: **September 25, 2026**

- at least two materially different Rust consumers compile and integrate;
- compatibility fixtures exist;
- resolution and anonymization have explicit failure and resource behavior;
- supported scope and limitations are evidence-backed;
- evaluation and initial fuzz evidence exist; and
- upgrade and migration expectations are documented.

### Gate C: Package and advertised-launch candidate

Target: **October 30, 2026**

- provenance, history, legal, security, and package reviews are complete;
- evaluation, fuzzing, and benchmark evidence support intended claims;
- maintainership and incident response are realistic;
- package and documentation release rehearsals pass;
- name and ecosystem positioning are approved; and
- package publication and active promotion are decided separately.

### Gate D: Stable-release decision

No fixed date is implied.

Maintainers may choose to:

- publish or continue a pre-1.0 crate;
- begin a `1.0` stability program;
- publish a narrower component;
- collaborate with or migrate to another project;
- remain quietly public; or
- stop or redirect the effort.

The decision must cite evidence. A public repository, green CI badge, elapsed time, or accumulated code volume does not create stability.

## Definition of success

The accelerated program succeeds when:

- the repository becomes public without exposing unauthorized or confidential material;
- external consumers can understand and evaluate it from documentation alone;
- maintainers can support the declared early-stage scope;
- security and correctness claims match evidence;
- the API evolves without avoidable consumer harm;
- contribution and release processes become operational on the compressed roadmap; and
- package publication and promotion remain controlled strategic choices rather than accidental consequences of a visibility setting.