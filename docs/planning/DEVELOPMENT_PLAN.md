# Secure Functional Alpha Roadmap

## Executive decision

`presidio-rs` became a publicly readable GitHub repository on Tuesday, July 28, 2026.

The roadmap now distinguishes two alpha gates:

1. **Public foundation alpha by August 3:** the public project is understandable, contributable, reproducible, and operationally governed.
2. **Secure functional alpha by August 21:** the authoritative text pipeline is bounded, source-exact, explicitly resolved, atomically transformable, explainable, and supported by reproducible evaluation and adversarial evidence.

This distinction matters because a public and buildable repository is not automatically a secure transformation system. GitHub visibility remains only slightly more meaningful than turning on a porch light.

Crates.io publication, active promotion, production certification, stable `1.0` compatibility, broad locale coverage, and comprehensive PII detection remain separate decisions.

## Governing architecture

The roadmap follows:

- [Architecture](../architecture/ARCHITECTURE.md)
- [ADR 0008: Stage the secure functional alpha through an evidence-gated pipeline](../adr/0008-stage-secure-functional-alpha-through-evidence-gated-pipeline.md)
- [Presidio archaeology and secure alpha model](../research/PRESIDIO_ARCHAEOLOGY_AND_ALPHA_MODEL.md)

Material alpha decisions use the Presidio evidence ledger and are classified as adopt, adapt, reject, defer, or investigate.

Python Presidio is an evidence source, not a normative compatibility oracle.

## Operating principles

- **Public foundation before promotion.** The source is public; broader announcement remains deliberate.
- **Secure pipeline before feature breadth.** Resolution, anonymization, explainability, evaluation, and fuzzing precede recognizer expansion.
- **Evidence before defaults.** A recognizer requires locale, provenance, regression, and evaluation evidence before default enablement.
- **Resolution before anonymization.** The transformation path cannot consume undefined overlap semantics.
- **Plan before mutation.** Authoritative anonymization validates the complete operation plan before producing output.
- **Preserve raw evidence.** Threshold, context, and resolution policy do not destroy original candidates.
- **Typed failure over silent degradation.** Wrong documents, invalid spans, limits, unsupported operations, and incomplete backends are explicit.
- **Offline core.** The default crate initiates no network or filesystem I/O.
- **Typed construction before configuration files.** Serialized configuration waits for stable concepts.
- **Consumer evidence outranks internal preference.** Two materially different consumers must validate the boundaries.
- **A better Rust project remains a successful outcome.** Adoption, collaboration, reuse, or migration can replace independent implementation.
- **No check weakening for dates.** The schedule may move; security and evidence gates do not.

## Current state

### Complete

- public repository visibility and anonymous network clone verification;
- governance, contribution, conduct, licensing, and confidential security-reporting documents;
- CI for formatting, Clippy, tests, documentation, packaging, Rust 1.74, DCO, dependency advisories, history scanning, and public-clone rehearsal;
- validated spans, confidence, identifiers, and source-bound findings;
- candidate-preserving reports with typed issues and resource limits;
- authoritative recognizer metadata and strict registration;
- bounded `AnalysisRequest` controls;
- backend-neutral recognizer execution;
- validated candidate emission and typed backend failures;
- contributor examples, API status, migration guidance, package rehearsal, and public documentation;
- initial Presidio archaeology and secure-alpha model;
- architecture decision establishing the staged secure-alpha pipeline.

### Partially complete

- public foundation alpha operations and branch rules;
- Presidio primary-source decision ledger;
- threat model and secure-alpha contract;
- compact and diagnostic evidence model;
- safe default recognizer policy;
- API compatibility baseline;
- consumer-pilot identification;
- release and support ownership.

### Not complete

- permanent explicit resolution policies;
- authoritative fallible document-bound anonymization;
- boundary-aware positive and negative context policy;
- source-to-output operation mapping;
- authoritative hash or pseudonymization decision;
- reproducible evaluation baseline;
- historical Presidio regression harness;
- initial fuzz and property targets;
- downstream compile fixture;
- two materially different consumer pilots;
- beta API baseline;
- public package release or advertised launch.

## Critical path

```text
Presidio archaeology and secure-alpha contract
        |
        v
Resolution policy and retained candidate evidence
        |
        v
Fallible anonymization plan and atomic execution
        |
        v
Explainability, context, and safe defaults
        |
        v
Evaluation receipts, historical regressions, and fuzzing
        |
        v
Two consumer validations and compatibility baseline
        |
        v
Hardened beta, package decision, and promotion decision
```

### Work that may run in parallel

- Presidio archaeology can proceed with alpha operations and corpus design.
- Corpus schema, provenance, and historical fixture collection can begin before resolution code lands.
- Fuzz harness scaffolding can begin before target APIs stabilize, then be rebound to final contracts.
- Consumer selection and compile-fixture setup can begin during secure-alpha implementation.
- Landscape review continues throughout every phase.

### Work that must remain ordered

- Authoritative anonymization does not precede explicit resolution semantics.
- Default recognizer expansion does not precede evaluation receipts and regression evidence.
- Durable serialized configuration does not precede stable policy and identifier contracts.
- Semantic-model adoption does not precede model-free alpha evidence and consumer justification.
- Crates.io publication does not precede secure-alpha and consumer evidence.

## Timeline summary

| Phase | Dates | Primary outcome |
|---|---|---|
| R0. Public visibility | Jul 27 to Jul 28, 2026 | Completed safe public source exposure without package publication or promotion |
| R1. Public foundation alpha | Jul 28 to Aug 3, 2026 | Finish public operations, ownership, branch controls, intake verification, and observation |
| R2. Secure functional alpha | Jul 28 to Aug 21, 2026 | Deliver the complete bounded analysis and transformation boundary with evaluation and adversarial evidence |
| R3. Consumer validation and beta | Aug 3 to Sep 4, 2026 | Validate two distinct consumers, compatibility boundaries, and beta acceptance |
| R4. Hardened public beta | Sep 4 to Sep 18, 2026 | Stabilize supported scope, performance evidence, security review, and package readiness |
| R5. Package and promotion decision | Sep 18 to Oct 2, 2026 | Decide crates.io publication and active promotion independently |
| R6. Maturity program | Oct 5, 2026 to Feb 26, 2027 | Broaden evidence, governance, integrations, locales, and optional capability feasibility |

The schedule is aggressive. The architecture narrows the alpha so speed is gained through focus rather than by hiding unfinished security work behind a larger feature table.

---

## R0: Public visibility

**Dates:** July 27 to July 28, 2026  
**Status:** Complete

### Outcome

The repository is public, anonymously cloneable, buildable, documented, and clear that package publication and production readiness have not occurred.

### Evidence

- full history, secret, provenance, license, and package audits;
- anonymous GitHub network-clone rehearsal;
- public documentation and examples;
- confidential security-reporting fallback;
- package and publication dry runs;
- immutable release-candidate evidence.

---

## R1: Public foundation alpha

**Dates:** July 28 to August 3, 2026  
**Primary issues:** #13 and #21

### Objectives

- complete operational controls for the public repository;
- verify contribution and confidential-security intake;
- observe early public feedback;
- establish a public alpha API and ownership baseline; and
- prepare consumer pilots without claiming the secure transformation gate is complete.

### Remaining work

- configure and verify the `main` branch ruleset and required checks;
- test the confidential reporting mailbox from an external sender;
- exercise public issue and pull-request intake without merging outside contributions;
- record GitHub licensing and organization-policy limitations;
- observe public activity through August 3;
- identify two materially different pilot consumers and acceptance boundaries;
- establish the initial downstream compile fixture; and
- reassess build, adopt, collaborate, narrow, and stop alternatives.

### Exit criteria

- an external contributor can understand and exercise the project without private context;
- branch controls are enforceable;
- ownership, triage, security intake, and contribution boundaries work;
- the public alpha API status is recorded; and
- no public observation requires withdrawal, clean export, or immediate rename.

This phase is a public-project gate, not the secure functional alpha gate.

---

## R2: Secure functional alpha

**Dates:** July 28 to August 21, 2026  
**Primary issues:** #14, #33, #34, #35, #36, and #37

### Gate definition

The authoritative text pipeline must be bounded, source-exact, deterministic under named policies, failure-safe, auditable without plaintext by default, and reproducibly evaluated.

### R2A: Presidio archaeology and alpha contract

**Target:** August 7  
**Issues:** #33 and #34

#### Work

- complete the first primary-source Presidio decision ledger for alpha-critical subsystems;
- map upstream architecture, failures, fixes, security changes, configuration evolution, evaluation practices, and governance lessons;
- classify each lesson as adopt, adapt, reject, defer, or investigate;
- freeze the supported and unsupported secure-alpha scope;
- define caller responsibilities and fail-open or fail-closed boundaries;
- document the threat model;
- map every alpha requirement to an automated test, manual verification, or explicit risk acceptance.

#### Exit criteria

- resolution, anonymization, context, default, cryptographic, and evaluation decisions reference relevant evidence;
- unsupported behavior is explicit;
- the alpha contract can be reviewed by a consumer, contributor, and security reviewer.

### R2B: Explicit candidate resolution

**Target:** August 7  
**Issue:** #14

#### Work

- define `ReportAll`, `BestCandidate`, and `ConservativeRedaction` semantics;
- specify full overlap, containment, partial intersection, adjacency, equal confidence, duplicates, entity priority, recognizer priority, and stable tie-breaking;
- preserve original candidates separately from resolved findings;
- version policy identity;
- record decision evidence;
- add legacy differential tests and selected Python Presidio historical fixtures;
- record the contract in an ADR and migration notes.

#### Exit criteria

- resolution is deterministic, versioned, documented, and independently testable;
- no authoritative anonymization work depends on implicit legacy sorting.

### R2C: Fallible document-bound anonymization

**Target:** August 14  
**Issues:** #14 and #37

#### Work

- introduce an anonymization policy and complete plan-validation stage;
- validate source identity, fingerprint, spans, resolution, operator support, and output limits before transformation;
- execute transformations atomically;
- return source-to-output operation records;
- represent planning and execution failures explicitly;
- prevent silent skips and partial success;
- support replacement, redaction, and validated masking;
- disable or explicitly restrict deterministic hashing on the authoritative path until reviewed semantics exist;
- add source-mismatch, Unicode, overlap, whitespace, end-of-text, low-entropy, and repeated-value tests.

#### Exit criteria

- a failed plan produces no successful transformed result;
- applied operations are auditable;
- the authoritative path has safe or unavailable cryptographic semantics.

### R2D: Explainability, context, and safe defaults

**Target:** August 14  
**Issue:** #35

#### Work

- define compact and diagnostic report levels;
- record recognizer, validator, context, threshold, allowlist, denylist, limit, and resolution evidence;
- keep diagnostics plaintext-free by default;
- separate context enhancement from recognition mechanics;
- support positive and negative boundary-aware context;
- define proximity, locale, case, and optional cross-entity behavior;
- inventory every default-enabled recognizer;
- require locale, country, provenance, regressions, and evaluation receipt identity for default enablement;
- disable weak or unevaluated defaults.

#### Exit criteria

- a consumer can explain why a candidate exists and why it was selected or rejected;
- every score change is attributable;
- default behavior is conservative and evidence-backed.

### R2E: Evaluation, historical regressions, fuzzing, and downstream fixture

**Target:** August 21  
**Issues:** #14 and #36

#### Work

- define a versioned synthetic and redistributable corpus schema;
- split synthetic template families to reduce evaluation leakage;
- measure exact-span and overlap-tolerant precision, recall, and F1;
- report by entity, recognizer, locale, country, and corpus family;
- retain false-positive and false-negative regressions;
- reproduce relevant Presidio failure families;
- classify Rust and Python differences;
- record corpus provenance and licensing;
- produce machine-readable evaluation receipts;
- add fuzz and property targets for spans, requests, candidate emission, context, resolution, anonymization, source-to-output mapping, and report serialization;
- retain minimized failures;
- add a downstream compile fixture for the authoritative path.

#### Exit criteria

- a clean checkout reproduces the evaluation baseline;
- major historical failure classes have regressions or explicit exclusions;
- critical-path fuzzing runs through CI or a documented scheduled workflow;
- the downstream fixture compiles and exercises the secure-alpha path;
- no quality or compatibility claim exceeds the evidence.

### Secure functional alpha exit gate

The phase closes only when:

- supported and unsupported scope is explicit;
- document identity and UTF-8 offsets are exact;
- resources are bounded;
- defaults are conservative and measured;
- candidates and policy decisions remain distinguishable;
- resolution is versioned and deterministic;
- anonymization is atomic and auditable;
- cryptographic semantics are safe or disabled;
- diagnostic evidence avoids plaintext by default;
- evaluation and historical regressions are reproducible;
- initial fuzz and property evidence exists;
- a downstream compile fixture passes; and
- no critical correctness or security issue remains unresolved.

---

## R3: Consumer validation and beta

**Dates:** August 3 to September 4, 2026  
**Primary issue:** #15

### Consumer requirement

Integrate at least two materially different Rust consumers. Two wrappers around the same private application do not count.

Candidate shapes include:

- in-process log or generated-text sanitizer;
- batch or CLI workflow;
- service boundary with custom entities;
- constrained offline application; or
- WASM feasibility consumer.

### August 3 to August 21

- name pilot owners and acceptance criteria;
- exercise realistic synthetic or redistributable workloads;
- record integration complexity, missing capabilities, and requested coupling;
- classify requested changes as core, optional adapter, consumer policy, or rejected coupling;
- add compile and migration fixtures;
- introduce `cargo-semver-checks` or equivalent drift detection after an alpha baseline exists.

### August 21 to September 4

- revalidate both consumers against the secure-alpha resolution and anonymization contracts;
- compare failure handling, resource behavior, latency, memory, and upgrade friction;
- confirm both consumers compile without private forks;
- update the Rust ecosystem comparison;
- publish a beta compatibility and limitation report.

### Exit criteria

- two distinct consumers integrate without private forks;
- compatibility fixtures pass in CI;
- reusable and consumer-owned boundaries survive real use;
- adoption cost and missing capabilities are explicit;
- no better Rust project invalidates the independent path.

---

## R4: Hardened public beta

**Dates:** September 4 to September 18, 2026

### Work

- freeze the beta API long enough for consumer acceptance;
- complete sustained fuzzing and retained regression corpora;
- add reproducible latency, throughput, allocation, memory, and package-size benchmarks;
- enforce dependency license, source, advisory, and ban policy;
- verify feature combinations and Rust 1.74;
- complete package metadata and docs.rs configuration;
- rehearse signed tag, package, documentation, and release-note generation without publishing;
- complete independent architecture and security review where practical;
- define realistic issue and vulnerability-response expectations;
- prepare supported matrix, migration notes, and known limitations.

### Exit criteria

- the beta scope is explicit and measured;
- no critical correctness or security issue remains open;
- package and documentation rehearsals pass;
- both consumers accept a pinned candidate;
- maintainer capacity matches the public posture.

---

## R5: Package and promotion decision

**Dates:** September 18 to October 2, 2026  
**Primary issue:** #16

### Package decision

Choose one:

- publish a `0.1.0` crate;
- publish a prerelease crate;
- remain GitHub-only;
- publish a narrower component;
- rename before publication; or
- collaborate with or migrate to another project.

### Promotion decision

Choose independently:

- actively announce and promote;
- communicate only to selected consumers and contributors;
- remain quietly public; or
- defer promotion pending stronger evidence or maintainer capacity.

### Required evidence

- secure-alpha acceptance;
- two-consumer acceptance;
- evaluation and regression results;
- fuzzing and security evidence;
- API stability and migration cost;
- performance and package evidence;
- ecosystem differentiation;
- name, legal, and provenance review;
- maintainer and security-response capacity; and
- unresolved risks and explicit acceptances.

A public repository does not create an obligation to publish a crate or organize a launch.

---

## R6: Maturity program

**Dates:** October 5, 2026 to February 26, 2027

This period remains available for:

- broader locale and entity evaluation;
- recognizer expansion supported by evaluation receipts;
- additional consumer integrations;
- performance and resource optimization;
- governance and maintainer succession;
- durable serialized configuration and report-schema decisions;
- reviewed pseudonymization capabilities;
- semantic-adapter feasibility;
- WASM, FFI, streaming, batch, or `no_std` work driven by consumers; and
- a later `1.0` program when evidence supports it.

February 26 remains a maturity review rather than the first possible publication date.

## Cross-cutting controls

### Presidio archaeology

Before a material alpha decision is declared stable, review relevant current source, tests, changelog entries, issues, pull requests, migration history, research practices, and security changes.

Record the outcome in the decision ledger and link the implementation evidence.

### Active Rust landscape watch

Escalate immediately when another Rust project:

- satisfies consumer requirements more completely;
- offers a reusable component that should replace planned work;
- becomes the likely ecosystem standard;
- develops stronger maintenance or evaluation capacity; or
- creates a credible collaboration path.

### Claims discipline

Every quality, performance, security, compatibility, or production-readiness claim identifies:

- commit or version;
- corpus or workload;
- configuration and policies;
- environment;
- metric definition;
- reproduction command; and
- limitations.

### Documentation gate

Every material PR updates the relevant combination of:

- README;
- API documentation;
- ADR;
- guide or example;
- changelog;
- migration notes;
- evaluation receipts;
- Presidio decision ledger; and
- risk register.

### Main-branch discipline

- keep `main` current through small reviewed merges;
- avoid long-lived feature stacks without a specific reason;
- merge only after required gates pass or an explicit exception is approved;
- preserve exact tested heads where release evidence requires it; and
- do not weaken checks to satisfy the schedule.

## Highest schedule and architecture risks

| Risk | Likelihood | Impact | Treatment |
|---|---|---|---|
| Presidio archaeology becomes broad research without decision output | Medium | High | Time-box by subsystem; require ledger rows and linked implementation issues |
| Resolution semantics remain ambiguous | Medium | Critical | Freeze versioned policies before authoritative anonymization |
| Partial or wrong-document transformation is possible | Medium | Critical | Validate complete document-bound plans and execute atomically |
| Weak recognizers become defaults | High | High | Require locale, regressions, provenance, and evaluation receipt identity |
| Diagnostics expose sensitive plaintext | Medium | Critical | Plaintext-free compact and diagnostic contracts by default |
| Deterministic hashes create linkability or brute-force risk | High | High | Disable or replace on authoritative path before secure alpha |
| Evaluation work trails implementation | High | High | Build corpus and historical fixtures in parallel from the start |
| Fuzzing produces evidence too late to affect design | Medium | High | Scaffold targets during implementation and retain minimized failures |
| Consumer pilots request application-specific coupling | Medium | High | Classify requests as core, adapter, policy, or rejected coupling |
| Aggressive dates encourage bypassing gates | Medium | Critical | Hard exit criteria and no check weakening |
| A better Rust project emerges | Medium | High | Active landscape watch and rapid adopt or collaborate decision |

## Definition of success

The roadmap succeeds when:

- the public project is understandable and responsibly operated;
- the secure functional alpha provides a trustworthy failure and transformation boundary;
- Presidio lessons are converted into decisions and regressions rather than copied mechanically;
- default behavior is small, conservative, and measured;
- two distinct consumers validate the reusable boundaries;
- package publication and promotion remain evidence-based choices; and
- speed increases learning without purchasing hidden correctness, security, provenance, or maintenance debt.
