# Multi-Phase Development Plan

## Executive decision

`presidio-rs` will be developed through a **30-week private program** beginning **August 3, 2026** and ending with a deliberate release-strategy decision on **February 26, 2027**.

The plan prioritizes correctness, architectural durability, evaluation, and multi-consumer validation before feature breadth. Public release is not an automatic phase outcome.

## Planning assumptions

The baseline timeline assumes:

- one primary Rust engineer contributing approximately 70% to 90% of a full-time schedule;
- one maintainer or senior reviewer contributing approximately 10% to 20%;
- periodic privacy, security, legal, and product review;
- access to at least two materially different Rust consumers for pilot integrations;
- no requirement to ship a production semantic NER backend on the critical path; and
- 20% capacity reserved across the program for discovery, redesign, defects, and unknown unknowns.

### Estimated effort

| Work category | Estimated effort |
|---|---:|
| Core architecture and implementation | 14 to 17 engineer-weeks |
| Evaluation, testing, and hardening | 7 to 9 engineer-weeks |
| Consumer pilots and compatibility | 4 to 6 engineer-weeks |
| Semantic-backend feasibility work | 2 to 4 engineer-weeks |
| Documentation, governance, and release work | 3 to 4 engineer-weeks |
| Review and specialist input | 5 to 7 reviewer-weeks, distributed |
| **Total** | **35 to 47 person-weeks across 30 calendar weeks** |

This is not a 47-week serial plan. Activities overlap, specialist effort is intermittent, and the primary engineer remains the pacing resource.

### Capacity scenarios

| Staffing shape | Reasonable calendar expectation |
|---|---|
| One primary engineer plus part-time reviewer | 28 to 34 weeks |
| Two experienced Rust engineers plus reviewer | 20 to 24 weeks |
| One engineer with frequent competing priorities | 36 to 44 weeks |

The 30-week baseline should be treated as a target range, not a contractual delivery date.

## Program outcomes

By the end of the plan, the project should have:

1. a defensible standalone architecture;
2. byte-accurate and fallible analysis and anonymization contracts;
3. extensible recognizer and evidence models;
4. a reproducible quality-evaluation harness;
5. explicit dependency, compatibility, and security controls;
6. at least two real Rust consumer pilots;
7. measured performance and quality evidence for supported scope;
8. a documented decision on semantic recognition;
9. a stable private beta release boundary; and
10. an explicit decision to publish, remain private, collaborate, rename, or redirect.

## Architecture work is continuous

Architecture is not confined to Phase 0.

Every phase includes:

- a written assumption review;
- at least one architecture checkpoint;
- an ADR for material decisions;
- a consumer-impact assessment;
- a risk-register update;
- a review of evidence gathered during the phase; and
- explicit confirmation that the next phase remains the best use of effort.

### Standing architecture cadence

| Cadence | Activity |
|---|---|
| Weekly | 45-minute architecture review during active implementation |
| Every two weeks | Risk, assumption, and unknowns review |
| Every four weeks | Downstream consumer and compatibility review |
| Every eight weeks | External landscape and parallel-project review |
| Phase exit | Architecture, evidence, and readiness gate |
| Before new heavy dependency | Time-boxed spike and ADR |
| Before public API break | Consumer compile test and migration analysis |

## Timeline summary

| Phase | Dates | Primary outcome |
|---|---|---|
| 0. Discovery and strategic validation | Aug 3 to Aug 14, 2026 | Decide whether to build, adopt, collaborate, or narrow scope |
| 1. Correctness and core architecture | Aug 17 to Sep 18, 2026 | Establish durable value, offset, error, and pipeline foundations |
| 2. Evaluation and evidence foundation | Aug 31 to Oct 2, 2026 | Create reproducible quality baseline and regression harness |
| 3. Extensibility and recognizer maturity | Sep 21 to Oct 30, 2026 | Introduce recognizer contract, metadata, locale, and evidence |
| 4. Multi-consumer private alpha | Oct 19 to Dec 4, 2026 | Integrate with at least two different Rust consumers |
| 5. Hardening and compatibility | Nov 16, 2026 to Jan 8, 2027 | Fuzz, benchmark, secure, and stabilize the supported core |
| 6. Semantic recognition feasibility | Dec 7, 2026 to Jan 22, 2027 | Make an evidence-based adopt, defer, or reject decision |
| 7. Private beta and release candidate | Jan 11 to Feb 12, 2027 | Produce a supportable private beta with measured claims |
| 8. Strategy and publication decision | Feb 15 to Feb 26, 2027 | Decide public release, continued private development, or redirection |

Phases overlap deliberately. Evaluation begins before the core redesign is complete, and consumer pilots begin before every feature is finished.

---

## Phase 0: Discovery and strategic validation

**Dates:** August 3 to August 14, 2026  
**Estimated effort:** 1.5 to 2.5 engineer-weeks plus stakeholder interviews

### Objectives

- Validate that the project has a useful and differentiated scope.
- Identify potential Rust consumers and their actual constraints.
- Compare build, adopt, collaborate, fork, and narrow-scope options.
- Establish the first architecture and risk baselines.

### Work

1. Inventory likely Rust consumers without assuming any one product owns the design.
2. Conduct short consumer interviews covering:
   - input sizes and throughput;
   - required entity types;
   - locale and country needs;
   - sync, async, batch, streaming, WASM, and FFI expectations;
   - acceptable dependencies and MSRV;
   - error and fail-open or fail-closed behavior;
   - observability and privacy requirements;
   - serialized output requirements; and
   - upgrade and compatibility expectations.
3. Evaluate parallel efforts including:
   - Microsoft Presidio and Presidio Research;
   - active Rust Presidio ports;
   - `redact-core` and related NER crates;
   - `cloakrs-core`;
   - secret-scanning projects such as Gitleaks; and
   - relevant Rust quality and supply-chain tools.
4. Build a capability and differentiation matrix.
5. Decide whether the best direction is:
   - continue independently;
   - collaborate with or contribute to an existing project;
   - depend on an existing core and build a narrower adapter;
   - focus on evaluation, governance, or constrained-runtime differentiation; or
   - stop before sunk cost becomes a product strategy.
6. Record the target architecture and first public API principles.
7. Establish a maintained assumption ledger.

### Required architecture decisions

- Core scope and differentiator.
- Build-versus-adopt-versus-collaborate decision.
- Primary consumer surface.
- Initial MSRV posture.
- One-crate versus workspace starting shape.
- Naming strategy for private development and future publication.

### Exit criteria

- At least two plausible consumers identified.
- A written competitive and collaboration assessment completed.
- A decision to continue has a clear reason beyond “Rust version of Presidio.”
- Target architecture is accepted or revised.
- Critical unknowns are scheduled as spikes rather than assumed away.

### Stop or redirect triggers

Pause or redirect if:

- an existing project already satisfies the required constraints with acceptable quality and governance;
- no distinct consumer need survives direct interviews;
- differentiation depends entirely on unmeasured performance claims;
- the project name or contribution model creates unacceptable legal or ecosystem risk; or
- maintenance capacity is insufficient for security-sensitive library ownership.

---

## Phase 1: Correctness and core architecture

**Dates:** August 17 to September 18, 2026  
**Estimated effort:** 4 to 5 engineer-weeks

### Objectives

- Repair correctness risks before expanding recognizer breadth.
- Establish stable internal concepts that can support multiple consumers.
- Preserve compatibility where reasonable through transition adapters.

### Workstreams

#### 1. Value and error types

Implement validated types for:

- `Span`;
- `Confidence`;
- `EntityId`;
- `RecognizerId`;
- `DocumentId`;
- `Finding`;
- `AnalysisReport`;
- `AnonymizationPlan`;
- `AnonymizationReport`; and
- typed error classifications.

#### 2. Original-text coordinate model

- Introduce `TextDocument`.
- Define original UTF-8 byte-offset semantics.
- Add normalization-to-source mapping tests.
- Prevent findings from being applied to the wrong text.
- Add configurable input and finding limits.

#### 3. Candidate preservation and resolution

- Preserve all qualifying candidates before resolution.
- Add explicit resolution policies.
- Define nested, adjacent, and equal-score behavior.
- Add conservative redaction span union.

#### 4. Fallible anonymization

- Replace silent skips with typed errors or explicit warnings.
- Separate planning from transformation.
- Validate all transformations before mutation.
- Add source-to-output operation records.

#### 5. Context correctness

- Replace substring matching with boundary-aware handling.
- Add negative context design or explicitly defer it.
- Record context evidence.

### Architecture checkpoints

- Week 1: value-type and compatibility strategy.
- Week 3: text and offset mapping review.
- Week 5: anonymization and resolution review.

### Exit criteria

- Invalid spans cannot silently pass through anonymization.
- Findings refer to original input coordinates.
- Candidate evidence survives until explicit resolution.
- Existing simple use cases remain available through migration shims or documented changes.
- New core invariants have unit, property, and integration tests.

### Key risks

- API churn can consume the entire phase.
- Unicode offset mapping may expose deeper design problems.
- Compatibility shims may obscure the cleaner target API.
- Over-generalized types may create needless complexity.

### Mitigation

Time-box abstractions, maintain concrete consumer fixtures, and prefer a small complete contract over speculative flexibility.

---

## Phase 2: Evaluation and evidence foundation

**Dates:** August 31 to October 2, 2026  
**Estimated effort:** 3 to 4 engineer-weeks, overlapping Phase 1

### Objectives

- Measure the current implementation before replacing it.
- Establish repeatable quality and error-analysis infrastructure.
- Prevent recognizer changes from being approved solely by anecdote.

### Work

1. Define a versioned corpus schema containing:
   - source text;
   - expected entity ID;
   - expected original-text spans;
   - locale and country;
   - source and license metadata;
   - synthetic-template family where applicable; and
   - allowed ambiguity.
2. Build exact-span and relaxed-overlap metrics:
   - precision;
   - recall;
   - F1;
   - false positives;
   - false negatives;
   - confusion by entity;
   - result by recognizer; and
   - result by locale or corpus slice.
3. Create regression fixtures for known failures.
4. Separate synthetic templates across train, tuning, and evaluation partitions to avoid template leakage.
5. Establish corpus provenance and privacy review.
6. Record baseline results for the current engine.
7. Add machine-readable evaluation receipts suitable for release evidence.

### Architecture decisions

- Whether evaluation tooling lives in the main crate, a sibling crate, or external tooling.
- Stable corpus and report schemas.
- Required evidence for default recognizer changes.
- How ambiguous or overlapping ground truth is represented.

### Exit criteria

- A clean checkout can reproduce the baseline.
- Results are available by entity and recognizer.
- New recognizer PRs have a defined evidence requirement.
- Corpus licensing and provenance are documented.
- Test and evaluation data do not contain accidental real secrets or sensitive personal data.

### Risk

Synthetic data can create impressive metrics that fail on real-world text. The plan therefore requires multiple corpus families and explicit reporting by slice rather than one blended score.

---

## Phase 3: Extensibility and recognizer maturity

**Dates:** September 21 to October 30, 2026  
**Estimated effort:** 4 to 5 engineer-weeks

### Objectives

- Make the analyzer backend-neutral.
- Support external Rust consumers without forcing them to fork the registry.
- Add recognizer maturity only where evaluation justifies it.

### Work

1. Introduce a `Recognizer: Send + Sync` contract.
2. Convert `PatternRecognizer` into one implementation.
3. Add recognizer metadata:
   - stable ID;
   - version;
   - supported entities;
   - locale and country;
   - mechanism;
   - capability requirements;
   - attribution; and
   - evaluation receipt.
4. Make analyzer construction immutable through a builder.
5. Add registry filtering by entity, recognizer, locale, and capability.
6. Add positive and negative context support if evaluation supports it.
7. Add rule prefilters to avoid running irrelevant patterns unnecessarily.
8. Add explicit allowlist and deny-list concepts without hiding detections in diagnostic mode.
9. Evaluate mature parsing dependencies for high-complexity entities such as international phone numbers.
10. Add a limited set of new recognizers chosen from consumer demand and evaluation value, not checklist accumulation.

### Architecture checkpoints

- Recognizer object safety, ownership, and concurrency.
- Open entity identifiers versus enum compatibility.
- Configuration timing and schema-version strategy.
- Dependency acceptance for parser-backed recognizers.

### Exit criteria

- A consumer can implement and register a recognizer without modifying the crate.
- Recognizers are explainable and versioned.
- The analyzer is safely shareable for concurrent callers.
- Default recognizer changes are evidence-backed.
- Country and locale filtering semantics are defined.

### Risk

A plugin interface can become an unstable dumping ground. Keep the contract narrow, pass values rather than internal engine state, and avoid exposing implementation details merely to support one experimental recognizer.

---

## Phase 4: Multi-consumer private alpha

**Dates:** October 19 to December 4, 2026  
**Estimated effort:** 4 to 6 engineer-weeks across the library and consumer teams

### Objectives

- Prove that the library is genuinely reusable.
- Discover API requirements that internal unit tests cannot reveal.
- Validate migration and compatibility behavior.

### Consumer selection

Pilot at least two consumers with materially different needs, for example:

- a synchronous in-process sanitizer for logs or generated text;
- a batch-processing or command-line workflow;
- a WASM feasibility consumer;
- a high-throughput service; or
- a consumer requiring custom entity definitions.

Two wrappers around the same application do not count as two pilots. Humans are remarkably creative when satisfying metrics, so the distinction is explicit.

### Work

1. Create downstream compile and integration fixtures.
2. Define consumer-specific policy outside the core crate.
3. Run the existing and new detectors in parallel where possible.
4. Compare:
   - detection differences;
   - latency and memory;
   - failure behavior;
   - integration complexity;
   - logging and observability needs; and
   - upgrade friction.
5. Record consumer-requested changes and classify them as:
   - broadly reusable core requirement;
   - optional adapter;
   - consumer-owned policy; or
   - rejected coupling.
6. Produce migration notes and sample integrations.
7. Establish a downstream compatibility test mechanism.

### Architecture checkpoints

- Consumer API review after the first pilot.
- Generalization review before accepting product-specific features.
- Compatibility and serialization review after the second pilot.

### Exit criteria

- At least two distinct consumers run realistic workloads.
- No consumer requires a private fork of the core.
- Failure and error semantics work at actual boundaries.
- Consumer-specific policy remains outside the reusable core.
- Adoption costs and missing capabilities are documented honestly.

### Stop or redesign triggers

- Consumers require mutually incompatible public contracts.
- The trait model causes unacceptable performance or ergonomics.
- The source-offset model fails under real normalization or tokenization needs.
- Most value comes from a consumer-specific adapter rather than the core.

---

## Phase 5: Hardening and compatibility

**Dates:** November 16, 2026 to January 8, 2027  
**Estimated effort:** 5 to 6 engineer-weeks

### Objectives

- Make supported behavior resilient under malformed, large, adversarial, and evolving inputs.
- Establish release-quality compatibility and dependency controls.

### Workstreams

#### Fuzzing and property testing

Add targets for:

- analyzer input;
- custom pattern construction;
- Unicode and normalization mapping;
- overlap resolution;
- anonymization planning;
- masking and replacement;
- report serialization; and
- streaming experiments if present.

#### Adversarial testing

Test:

- large inputs;
- maximum finding counts;
- dense overlapping matches;
- malformed or unusual Unicode;
- boundary-crossing identifiers;
- low-entropy pseudonymization inputs;
- invalid consumer-provided patterns;
- pathological configuration; and
- panic and resource-exhaustion behavior.

#### Performance

Measure with reproducible inputs:

- cold analyzer construction;
- warm p50, p95, and p99 latency;
- throughput;
- allocations and peak memory;
- effect of recognizer count;
- effect of input size;
- batch behavior;
- artifact size; and
- optional feature cost.

#### Supply chain

Add and enforce:

- `cargo-deny` for licenses, advisories, bans, and sources;
- `cargo-audit` or equivalent advisory review;
- dependency rationale and ownership;
- feature-matrix CI;
- MSRV CI; and
- dependency-update review.

#### Compatibility

- Add consumer compile fixtures.
- Introduce `cargo-semver-checks` after a versioned baseline exists.
- Define serialized schema-version policy.
- Add migration guidance for breaking changes.

### Architecture decisions

- Input and resource limit defaults.
- Dependency acceptance policy.
- Feature-flag structure.
- Pseudonymization replacement for salted hashing.
- Whether tracing integration belongs in core or an adapter.

### Exit criteria

- No known panics on untrusted text within supported limits.
- Fuzzing has sustained runs with retained regression corpus.
- Performance measurements are reproducible and published internally.
- Dependency and license policies pass.
- Downstream compile fixtures pass.
- Claims documentation distinguishes measured results from expectations.

### Risk

Hardening often reveals architecture defects late. The timeline includes overlap with consumer pilots and preserves contingency rather than pretending this is merely a test-writing phase.

---

## Phase 6: Semantic recognition feasibility

**Dates:** December 7, 2026 to January 22, 2027  
**Estimated effort:** 2 to 4 engineer-weeks

### Critical-path status

This phase is **not required** for a useful pattern-based private beta. It is a controlled feasibility program.

### Objectives

- Determine whether semantic recognition belongs in the project.
- Avoid committing to a runtime, model, or artifact strategy based on familiarity alone.

### Candidate work

1. Define the semantic recognizer adapter contract.
2. Evaluate at least two viable implementation paths when available:
   - native Candle model implementation;
   - ONNX-based runtime such as Candle ONNX, Tract, or ONNX Runtime; or
   - collaboration with an existing Rust NER project.
3. Evaluate:
   - model and dataset license;
   - source-offset alignment;
   - supported labels and languages;
   - artifact size;
   - initialization and inference latency;
   - memory;
   - platform support;
   - MSRV impact;
   - transitive native dependencies;
   - maintenance burden; and
   - measurable quality gain over the model-free core.
4. Produce one thin adapter spike rather than a complete product integration.
5. Compare build versus reuse of existing Rust semantic implementations.

### Required decision

Choose one:

- adopt a semantic backend;
- collaborate with or depend on an existing implementation;
- retain only the adapter seam and defer implementation;
- reject semantic recognition from this project; or
- move semantic recognition to a separately governed project.

### Exit criteria

- Decision is supported by evaluation and resource evidence.
- No model or runtime enters the default dependency graph accidentally.
- Model provenance and update policy are documented if adopted.
- Public claims remain scoped to actual evaluated labels and languages.

### Stop triggers

- License or provenance is unsuitable.
- Source-offset mapping cannot be made reliable.
- Quality gain does not justify operational cost.
- A maintained existing implementation is clearly superior to building another.

---

## Phase 7: Private beta and release candidate

**Dates:** January 11 to February 12, 2027  
**Estimated effort:** 3 to 4 engineer-weeks

### Objectives

- Produce a supportable private beta release.
- Freeze a coherent compatibility boundary long enough to gather real usage evidence.
- Prepare, but do not presume, public release.

### Work

1. Select the supported entity, locale, platform, and feature matrix.
2. Remove or clearly mark experimental APIs.
3. Complete API documentation and examples.
4. Add release notes and migration notes.
5. Run full evaluation, benchmarks, fuzzing, and supply-chain checks.
6. Run consumer acceptance against pinned release candidates.
7. Review naming, licensing, CLA, provenance, and public history strategy.
8. Obtain independent architecture and security review where practical.
9. Define maintenance ownership and response expectations.
10. Produce a release evidence bundle containing:
    - source commit;
    - dependency lock and audit result;
    - evaluation receipt;
    - benchmark receipt;
    - supported matrix;
    - known limitations;
    - consumer acceptance; and
    - unresolved risks.

### Exit criteria

- Private consumers can pin and upgrade the release candidate.
- Supported behavior is documented and measured.
- Known limitations are explicit.
- No critical security or correctness issue remains open.
- Maintenance capacity is named.
- Public-release checklist has evidence for every completed item.

---

## Phase 8: Strategy and publication decision

**Dates:** February 15 to February 26, 2027  
**Estimated effort:** 1 to 2 engineer-weeks plus leadership, legal, and maintainer review

### Possible outcomes

1. Publish a clean, renamed open-source repository.
2. Continue private development with public-grade practices.
3. Publish only a smaller core or evaluation toolkit.
4. Collaborate with or contribute to another project.
5. Maintain a private fork while standardizing compatibility with another ecosystem.
6. Stop or archive the effort if differentiation and maintenance do not justify continuation.

### Decision inputs

- consumer adoption and satisfaction;
- measured quality;
- operational cost;
- ecosystem differentiation;
- collaboration opportunities;
- maintenance burden;
- naming and trademark posture;
- legal review;
- provenance review;
- public versus commercial boundary; and
- unresolved security risks.

### Public release is approved only when

- the project name is independently defensible;
- source and history export are audited;
- licensing and contribution terms are reviewed;
- quality and performance claims are reproducible;
- a maintainer can support public issues and security reports;
- the supported scope is clear; and
- publication creates more strategic value than ongoing private development.

---

## Cross-phase workstreams

### 1. Consumer advisory loop

Maintain a lightweight consumer group representing distinct integration shapes. Every four weeks, review:

- API ergonomics;
- feature requests;
- compatibility risk;
- unsupported use cases;
- performance needs; and
- policy leakage into the core.

### 2. Assumption ledger

Track assumptions with:

- statement;
- owner;
- confidence;
- evidence;
- validation date;
- consequence if false; and
- scheduled revisit.

Examples:

- consumers need Rust 1.74 support;
- sync APIs are sufficient;
- original byte offsets meet consumer needs;
- model-free detection provides useful value independently;
- no consumer requires `no_std`;
- configuration can wait until the programmatic API stabilizes.

### 3. Architecture runway

Reserve approximately 15% of implementation capacity for design spikes, migration, and structural cleanup. This prevents feature delivery from consuming all capacity until architecture changes become impossible.

### 4. Landscape monitoring

Every eight weeks, update the parallel-efforts review. Record whether another project has:

- closed a capability gap;
- adopted a better architecture;
- changed license or maintenance status;
- established the likely ecosystem standard; or
- created a collaboration opportunity.

### 5. Claims governance

Every benchmark, quality, security, or compatibility claim must include:

- exact version or commit;
- input corpus or workload;
- configuration;
- environment;
- metric definition;
- reproduction command; and
- known limitations.

## Unknown-unknown strategy

The plan assumes that important requirements and defects remain undiscovered.

Controls include:

1. **20% schedule contingency** rather than a zero-slack critical path.
2. **Early real-consumer pilots** before API stabilization.
3. **Build-versus-adopt checkpoint** before major investment.
4. **Architecture spikes** for model runtime, Unicode mapping, cryptography, and streaming.
5. **Independent review** before security-sensitive releases.
6. **Dual-run comparison** against existing consumer detectors where possible.
7. **Adversarial testing** designed to invalidate assumptions, not merely confirm examples.
8. **Phase stop criteria** so sunk cost does not force continuation.
9. **Monthly assumption review** with explicit confidence changes.
10. **No semantic backend on the core critical path.**

## Highest risks to the timeline

| Risk | Likelihood | Impact | Timeline treatment |
|---|---|---|---|
| Existing Rust project proves better fit | Medium | High | Phase 0 build/adopt checkpoint |
| Unicode and source-offset correctness requires redesign | Medium | High | Address in Phase 1 before breadth |
| Consumer requirements conflict | Medium | High | Two distinct pilots and policy-boundary review |
| Evaluation corpus is misleading or legally unusable | Medium | High | Provenance review and multiple corpus slices |
| API redesign creates migration burden | High | Medium | Compatibility facade and consumer fixtures |
| NER runtime or model is unsuitable | High | Medium | Optional Phase 6 with stop criteria |
| Maintainer capacity is lower than assumed | Medium | High | Reforecast at every phase exit |
| Dependency or MSRV conflict | Medium | Medium | Dependency spikes and feature isolation |
| False negatives undermine trust | High | High | Evidence-first scope and explicit limitations |
| Public naming or trademark problem | Medium | High | Keep private name provisional; resolve before publication |
| Security issue in anonymization or pseudonymization | Medium | High | Fallible API, crypto ADR, independent review |
| Overengineering delays usable value | Medium | Medium | Concrete consumer fixtures and phase exit gates |

## Delivery confidence

- **High confidence:** 30 weeks is sufficient to create a materially stronger pattern-based private beta if staffing assumptions hold.
- **Medium confidence:** two consumer pilots can complete in the proposed window because consumer availability is outside the repository's control.
- **Low-to-medium confidence:** a production-worthy semantic backend belongs inside the same 30-week program. It is deliberately optional.
- **Medium confidence:** the project should ultimately be published. The plan preserves publication as an evidence-based decision rather than an emotional reward for finishing the roadmap.

## Immediate next actions

1. Complete Phase 0 consumer and parallel-project assessment.
2. Record ADR 0002 for the backend-neutral core and optional-adapter direction.
3. Create the assumption ledger and risk register.
4. Define the evaluation corpus schema before adding recognizer breadth.
5. Identify two pilot consumers and secure participation windows.
6. Convert the phases into issues only after Phase 0 confirms the direction, avoiding a backlog composed mostly of assumptions.
