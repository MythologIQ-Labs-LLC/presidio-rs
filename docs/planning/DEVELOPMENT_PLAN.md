# Rebaselined Development Roadmap

## Executive decision

`presidio-rs` became a **publicly readable GitHub repository on Tuesday, July 28, 2026**, two days ahead of the original target.

This supersedes the original 30-week private-development sequencing. The project exposed its source early and continues foundation work in public.

Public visibility is not equivalent to:

- crates.io publication;
- an advertised launch;
- production certification;
- a stable `1.0` API;
- complete evaluation evidence;
- guaranteed maintainer response times; or
- a promise that every planned capability is implemented.

Those remain separate maturity and strategy decisions.

## Rebaseline rationale

The original roadmap assumed that value types, source binding, candidate-preserving reports, recognizer metadata, request controls, backend-neutral execution, governance, CI, documentation, and release discipline would be built over several months.

Those foundations are already present on `main` substantially earlier than planned. The remaining work is no longer “prepare privately before anyone can see it.” It is now:

1. make the current repository safe and honest to expose;
2. continue correctness and contributor-readiness work in public;
3. gather evaluation and consumer evidence quickly;
4. decide package publication and active promotion separately; and
5. keep the build-versus-adopt-versus-collaborate decision active.

The accelerated schedule spends the recovered architecture time on visibility safety, correctness, evaluation, consumer validation, and maintainership rather than feature breadth.

## Operating principles

- **Public since July 28; promotion by explicit decision.**
- **Foundation before breadth.** No recognizer accumulation merely to make a feature table longer.
- **Honest maturity language.** Early-stage limitations remain prominent.
- **One current `main`.** Completed slices merge promptly after review and green gates.
- **Documentation ships with behavior.** README, guides, ADRs, changelog, and migration notes change in the same PR.
- **Consumer evidence outranks internal preference.**
- **A better existing Rust project remains a successful outcome.** Adoption, collaboration, reuse, or migration may replace independent implementation.
- **Public access does not waive security, provenance, or licensing obligations.**

## Current foundation status

Completed before the rebaseline:

- open-source-grade governance, contribution, conduct, licensing, and security documents;
- CI for formatting, Clippy, tests, documentation, packaging, MSRV, DCO, and dependency audit;
- validated spans, confidence values, and open identifiers;
- candidate-preserving reports with typed issues and resource limits;
- authoritative recognizer metadata and strict registration;
- exact `TextDocument` source binding;
- bounded `AnalysisRequest` selection and resource controls;
- object-safe backend-neutral recognizer execution;
- validated candidate emission and typed backend failures;
- current README, documentation index, ADR set, changelog, risk register, and active landscape watch;
- a stealth open-source foundation track and explicit release checklist.

Partially complete:

- API compatibility policy;
- contributor examples;
- public release operations;
- package metadata and package-content rehearsal;
- provenance and history review;
- source and documentation link verification;
- maintainer ownership and branch-protection configuration.

Not yet complete:

- explicit permanent candidate-resolution policy;
- fallible document-bound anonymization;
- reproducible quality evaluation baseline;
- sustained fuzzing and property testing;
- two materially different consumer pilots;
- downstream compile fixtures;
- semver baseline and migration guide;
- public package release;
- advertised launch;
- production-readiness evidence.

## Timeline summary

| Phase | Dates | Primary outcome |
|---|---|---|
| R0. Public visibility release | Jul 27 to Jul 28, 2026 | Completed public exposure without advertising or package publication |
| R1. Contributor-ready public alpha | Jul 28 to Aug 3, 2026 | Complete public operations, ownership, observation, and contribution-intake verification |
| R2. Correctness and evaluation sprint | Jul 28 to Aug 21, 2026 | Deliver explicit resolution, fallible anonymization, evaluation baseline, and critical fuzz targets |
| R3. Consumer and compatibility sprint | Aug 3 to Sep 4, 2026 | Integrate two distinct consumers and establish downstream compatibility evidence |
| R4. Public beta candidate | Sep 4 to Sep 18, 2026 | Harden supported scope, package contents, API migration, security, and measured claims |
| R5. Package and launch decision | Sep 18 to Oct 2, 2026 | Decide crates.io publication and whether to actively announce the project |
| R6. Post-public maturity program | Oct 5, 2026 to Feb 26, 2027 | Continue hardening, broader evaluation, governance maturity, and optional semantic feasibility |

The schedule is intentionally aggressive. Workstreams overlap, but public visibility is not allowed to erase the correctness and evidence work required for later maturity claims.

---

## R0: Public visibility release

**Dates:** July 27 to July 28, 2026  
**Status:** Completed with public source access on Tuesday, July 28

### Objective

Make the repository publicly readable, cloneable, buildable, and honestly documented without implying package release, production readiness, or an advertising commitment.

### Required work

1. Freeze non-release scope.
2. Reconcile the roadmap and release checklist.
3. Review the intended public tree and history for:
   - secrets and credentials;
   - confidential references;
   - customer, employee, or private third-party data;
   - problematic commit metadata;
   - deleted proprietary material; and
   - content that requires a clean export instead of a visibility change.
4. Review source, fixture, regex, validator, algorithm, and documentation provenance.
5. Confirm copyright ownership and MIT distribution intent.
6. Review the name, package identity, Microsoft references, and non-affiliation language.
7. Verify public-facing documentation and links.
8. Verify issue templates, PR templates, private vulnerability reporting, Dependabot, DCO, and branch-protection expectations.
9. Run a clean-clone build, test, documentation, and package rehearsal.
10. Record the release commit, evidence, known limitations, operator, maintainers, and rollback procedure.
11. Change visibility or publish an approved clean export.
12. Verify anonymous access after the change.

### Must-pass blockers

Public visibility is blocked only by risks that make exposure unsafe or unauthorized, including:

- secrets or confidential material;
- unclear right to publish source or fixtures;
- unacceptable license or provenance problems;
- unresolved naming risk severe enough to require a rename before exposure;
- materially false or misleading public documentation;
- broken build or verification commands;
- no usable security-reporting path; or
- no accountable maintainer for the visibility operation.

Incomplete metrics, fuzzing, pilots, semantic recognition, fallible anonymization, package publication, and advertising are not visibility blockers when documented as incomplete.

### Exit criteria

- the repository or clean export is public;
- anonymous clone and documentation access work;
- the declared early-stage status is prominent;
- no package publication or announcement is implied;
- public issue and security-reporting paths work; and
- the release evidence record is committed.

Detailed runbook: [Public Repository Release Week](PUBLIC_RELEASE_WEEK.md).

---

## R1: Contributor-ready public alpha

**Dates:** July 28 to August 3, 2026

### Objectives

- Make the newly public project understandable and contributable without private context.
- Remove operational gaps exposed by public access.
- Establish the first explicit compatibility baseline.

### Work

- add a runnable strict pattern-recognizer example;
- add a minimal custom backend reference implementation;
- create a first-contribution guide with synthetic fixtures;
- add CODEOWNERS or equivalent ownership documentation;
- document public issue triage, review ownership, and security escalation;
- document the legacy versus request-oriented API migration path;
- inventory the public API and classify items as stable-for-alpha, transitional, legacy-compatible, experimental, or unsupported;
- add documentation link checking;
- add a clean package-content check and `cargo publish --dry-run` rehearsal without publishing;
- enable and verify required branch protections and checks;
- process public feedback without accepting internal-product coupling into the core.

### Exit criteria

- a fresh external contributor can build, test, understand, and extend the project from repository documentation alone;
- ownership and triage responsibilities are explicit;
- the alpha compatibility policy and migration path are documented;
- no release-week operational blocker remains open.

---

## R2: Correctness and evaluation sprint

**Dates:** July 28 to August 21, 2026

### Workstream A: explicit candidate resolution

- define supported overlap, nesting, adjacency, and equal-confidence behavior;
- preserve unresolved candidates separately from selected findings;
- make resolution policy explicit and versionable;
- add conservative redaction-union behavior where policy requires it;
- add differential tests against the legacy projection.

### Workstream B: fallible document-bound anonymization

- introduce an anonymization plan over validated document-bound findings;
- validate the entire plan before transforming text;
- reject source mismatch, invalid spans, unsupported operations, and policy conflicts explicitly;
- produce source-to-output operation records;
- avoid silent skips and partial transformation without an explicit report.

### Workstream C: evaluation baseline

- define a versioned synthetic and redistributable corpus schema;
- measure exact-span and overlap-tolerant precision, recall, and F1;
- report results by entity, recognizer, locale, and corpus family;
- add false-positive and false-negative regression fixtures;
- record corpus provenance and licensing;
- produce machine-readable evaluation receipts.

### Workstream D: fuzzing and property tests

Add initial targets for:

- span validation;
- request construction;
- candidate emission;
- resolution;
- anonymization planning and application; and
- report serialization.

### Exit criteria

- resolution behavior is explicit and tested;
- document-bound anonymization is fallible and auditable;
- a clean checkout reproduces an evaluation baseline;
- critical-path fuzz and property targets run in CI or a documented scheduled workflow;
- unsupported behavior remains documented rather than implied.

---

## R3: Consumer and compatibility sprint

**Dates:** August 3 to September 4, 2026

### Consumer pilots

Integrate at least two materially different Rust consumers, such as:

- an in-process log or generated-text sanitizer;
- a batch or CLI workflow;
- a service with custom entity definitions;
- a constrained or offline application; or
- a WASM feasibility consumer.

Two wrappers around the same application do not count.

### Work

- add downstream compile fixtures;
- run realistic workloads through the request-oriented API;
- compare integration complexity, failure handling, resource behavior, and upgrade friction;
- classify requested changes as core, adapter, consumer policy, or rejected coupling;
- add migration fixtures and examples;
- introduce `cargo-semver-checks` or equivalent public API drift detection after the alpha baseline is recorded;
- update the ecosystem comparison with any newly viable Rust alternatives.

### Exit criteria

- two distinct consumers compile and integrate without private forks;
- compatibility fixtures pass in CI;
- reusable versus consumer-owned boundaries have survived real use;
- adoption cost and missing capabilities are documented honestly;
- no better existing Rust project has invalidated the independent path.

---

## R4: Public beta candidate

**Dates:** September 4 to September 18, 2026

### Objectives

- Stabilize a supportable public beta boundary.
- Convert implementation claims into measured evidence.
- Rehearse package and release operations without assuming publication.

### Work

- freeze the beta API surface long enough for consumer verification;
- complete sustained fuzzing and retained regression corpora;
- add reproducible latency, throughput, allocation, memory, and package-size benchmarks;
- enforce license, source, ban, and advisory policy with `cargo-deny` or equivalent;
- verify feature combinations and MSRV;
- complete package metadata and docs.rs configuration;
- perform a clean-history or clean-export rehearsal;
- run an independent security and architecture review where practical;
- define realistic public issue and vulnerability response expectations;
- prepare beta release notes, migration notes, supported matrix, and known limitations.

### Exit criteria

- the supported beta scope is explicit and measured;
- no critical correctness or security issue remains open;
- package and documentation rehearsals pass;
- two consumers accept the pinned beta candidate;
- maintainer capacity matches the public support posture.

---

## R5: Package and advertised-launch decision

**Dates:** September 18 to October 2, 2026

### Separate decisions

Maintainers decide independently whether to:

1. publish a `0.1.0` crate;
2. tag a GitHub-only beta without package publication;
3. actively announce and promote the project;
4. remain quietly public while continuing evidence work;
5. publish a narrower component;
6. collaborate with or migrate to another project; or
7. rename or redirect before package publication.

### Decision evidence

- consumer adoption and feedback;
- evaluation results;
- fuzzing and security evidence;
- API stability and migration cost;
- package and docs rehearsal;
- ecosystem differentiation;
- naming and legal review;
- provenance review;
- maintainer and security-response capacity; and
- unresolved risks.

A public repository does not create an obligation to publish a crate or organize a launch campaign.

---

## R6: Post-public maturity program

**Dates:** October 5, 2026 to February 26, 2027

This period remains available for:

- broader locale and entity evaluation;
- additional consumer integrations;
- performance and resource optimization;
- governance and maintainer succession maturity;
- serialized schema decisions;
- pseudonymization improvements;
- no-std, WASM, FFI, streaming, or async feasibility driven by consumers;
- semantic recognition feasibility outside the default dependency graph;
- collaboration with related Rust projects; and
- a later `1.0` stability program when evidence supports it.

The February 26 date becomes a maturity review rather than the first possible publication decision.

## Cross-cutting controls

### Active ecosystem watch

Continue weekly monitoring and immediate escalation when another Rust project:

- satisfies the consumer requirements more completely;
- provides a reusable component that should replace planned work;
- becomes the likely ecosystem standard;
- develops stronger maintenance or evaluation capacity; or
- creates a credible collaboration path.

### Claims discipline

Every quality, performance, security, compatibility, or production-readiness claim must identify:

- commit or version;
- corpus or workload;
- configuration;
- environment;
- metric definition;
- reproduction command; and
- limitations.

### Documentation gate

Every material PR must update the relevant combination of:

- README;
- API documentation;
- ADR;
- guide or example;
- changelog;
- migration notes;
- evaluation receipts; and
- risk register.

### Main-branch discipline

- Keep `main` current through small reviewed merges.
- Do not preserve long-lived feature stacks without a specific reason.
- Merge only after required gates pass or an explicit documented exception is approved.
- Do not weaken checks to satisfy the accelerated schedule.

## Highest schedule risks

| Risk | Likelihood | Impact | Treatment |
|---|---|---|---|
| History contains confidential or unpublishable material | Medium | Critical | Complete history scan; use clean export if necessary |
| Name or Microsoft association creates material confusion | Medium | High | Review before visibility; rename or add stronger distancing language |
| Provenance of patterns or fixtures is incomplete | Medium | Critical | Audit and remove, replace, or attribute affected material |
| Public access creates unsupported maturity assumptions | High | High | Prominent early-stage status and explicit non-goals |
| Maintainer capacity is overwhelmed by attention | Medium | High | Quiet release, no advertising, triage policy, realistic response language |
| Resolution or anonymization defects undermine trust | Medium | High | Make them immediate post-public critical path |
| Evaluation evidence remains weak | High | High | Complete baseline by September 4 before broader claims |
| A better Rust project emerges | Medium | High | Weekly landscape watch and five-day architecture response |
| Aggressive schedule causes checks to be bypassed | Medium | Critical | Hard visibility blockers and no check weakening |

## Definition of success

The aggressive rebaseline succeeds when:

- the repository is public by July 30 without exposing unauthorized or confidential material;
- documentation accurately states what is and is not ready;
- contributors can understand the project without private context;
- correctness and evaluation work continues on a compressed public roadmap;
- package publication and advertising remain deliberate choices; and
- speed improves learning without purchasing it through hidden security, provenance, or maintenance debt.