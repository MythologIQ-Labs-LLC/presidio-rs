# Parallel Efforts and Architectural Lessons

## Purpose

This review identifies lessons from related privacy, PII, secret-scanning, and Rust library efforts. It is not an endorsement of their claims or a decision to copy their designs.

The landscape is expected to change. Review it at least every eight weeks while `presidio-rs` remains in active architectural development.

**Reviewed:** July 27, 2026

## Decision principle

Before implementing a major capability, ask:

1. Does an existing maintained project already solve it?
2. Can we depend on, contribute to, or collaborate with that project?
3. Is our requirement materially different?
4. Is the difference architectural, operational, legal, or merely aesthetic?
5. Can we validate the difference with consumers and measurements?

A Rust rewrite is not automatically differentiation. Crabs are abundant; maintenance capacity is not.

## 1. Microsoft Presidio

Reference: [microsoft/presidio](https://github.com/microsoft/presidio)

### Relevant design patterns

- A common recognizer abstraction supports pattern, rule, and model-backed detection.
- `RecognizerRegistry` separates recognizer discovery from analyzer orchestration.
- NLP is represented as a capability rather than embedded into each pattern recognizer.
- Context enhancement is a separate concern.
- Recognizers have names, versions, supported entities, and language metadata.
- Country and language filtering constrain recognizer selection.
- Decision-process tracing helps explain why a finding exists.
- Batch processing is considered separately from single-text analysis.
- Configuration has evolved into a meaningful public surface.

### Lessons to adopt

- Use one backend-neutral recognizer contract.
- Preserve recognizer identity and version in findings.
- Treat language and country as first-class metadata.
- Make diagnostic evidence available without requiring plaintext logging.
- Keep analyzer orchestration separate from recognition mechanics.
- Expect configuration and metadata to become compatibility surfaces.

### Lessons not to copy blindly

- Do not reproduce Python class boundaries when Rust ownership and trait design suggest a smaller contract.
- Do not claim behavioral compatibility without a compatibility suite.
- Do not require an NLP stack for model-free consumers.
- Do not inherit service and deployment assumptions irrelevant to an in-process Rust library.

## 2. Presidio Research

Reference: [microsoft/presidio-research](https://github.com/microsoft/presidio-research)

### Relevant design patterns

- Evaluation and data-science tooling live outside the primary runtime.
- The project supports synthetic data generation, standardized sample representations, model and recognizer evaluation, and error analysis.
- Synthetic dataset splitting accounts for template leakage, preventing related generated sentences from appearing across training and evaluation partitions.
- Results are analyzed by entity and error category rather than only through one aggregate score.

### Lessons to adopt

- Keep evaluation tooling separable from the runtime dependency graph.
- Use a stable corpus schema that can represent spans, labels, source, and ambiguity.
- Split synthetic templates by family, not merely by generated row.
- Require per-entity precision, recall, F1, false positives, and false negatives.
- Build error-analysis tooling, not only a pass or fail metric.
- Treat corpus licenses and provenance as release evidence.

### Unknowns

- Which public datasets can legally and safely be vendored or redistributed?
- How much synthetic evaluation predicts real consumer text?
- Which entity taxonomies can be mapped without hiding mismatches?

## 3. Active Rust Presidio ports

### `presidio-analyzer`

References:

- [docs.rs/presidio-analyzer](https://docs.rs/presidio-analyzer)
- Repository link from the crate metadata should be inspected during Phase 0.

As of July 2026, this project presents:

- a Rust analyzer architecture mirroring Presidio;
- a general `EntityRecognizer` abstraction;
- pattern recognizers and validators;
- an NLP abstraction and NER recognizer;
- country-specific modules;
- gazetteer support; and
- phone recognition backed by a dedicated phone-number library.

### Strategic implication

This is the strongest reason to perform a build-versus-adopt-versus-collaborate decision immediately.

`presidio-rs` should not spend months recreating an equivalent feature list without answering:

- Is the existing API sound?
- Is the license compatible?
- Is maintenance credible?
- Does it satisfy offline, dependency, MSRV, provenance, and consumer constraints?
- Would contribution or a focused fork create more value than a separate project?
- Is our differentiator evaluation rigor, minimal core scope, explainability, constrained-runtime support, governance, or something else?

### Required Phase 0 action

Perform a source-level review and produce a scored comparison covering:

- architecture;
- correctness;
- tests and evaluation;
- dependency graph;
- MSRV;
- platform support;
- maintenance activity;
- API stability;
- provenance;
- security posture; and
- collaboration feasibility.

## 4. `redact-core`

References:

- [docs.rs/redact-core](https://docs.rs/redact-core)
- [censgate/redact](https://github.com/censgate/redact)

The project presents a broad PII platform with pattern detection, anonymization, policy, optional NER, multiple platforms, and a larger dependency surface.

### Lessons to adopt

- Isolate heavier NER and platform capabilities from the core.
- Treat multi-platform support as a matrix requiring real tests.
- Consider policy-aware anonymization without absorbing full application policy into the detector.
- Compare actual package size, build time, dependencies, and supported targets.

### Cautions

Public descriptions include strong production, performance, memory, and replacement claims. Those claims must be independently reproduced before they inform project positioning or architectural decisions.

The lesson is not that the claims are false. The lesson is that `presidio-rs` must never adopt a claim because another README used confident typography.

### Strategic implication

A broad all-in-one feature race is unlikely to be the best differentiation. A smaller explainable core, stronger evaluation receipts, and clearer dependency boundaries may be more defensible.

## 5. `cloakrs-core`

Reference: [docs.rs/cloakrs-core](https://docs.rs/cloakrs-core)

The public API includes core types, a recognizer trait, registry, scanner, masking strategies, confidence, locale, entity, and span concepts.

### Lessons to adopt

- Traits and public value objects are natural Rust extension points.
- Locale belongs in the core data model when recognizer coverage varies by region.
- Recognition and masking strategies can remain separate.
- Stable recognizer IDs improve configuration and diagnostics.

### Questions for Phase 0

- How does it handle overlap and source offsets?
- How are recognizers versioned?
- What is its evaluation discipline?
- Is there an opportunity to collaborate on shared types, corpus formats, or benchmarks?

## 6. Gitleaks and mature rule-driven scanners

Reference: [gitleaks/gitleaks](https://github.com/gitleaks/gitleaks)

Although Gitleaks scans secrets rather than general PII, its rule ecosystem exposes mature problems that `presidio-rs` will eventually encounter.

### Relevant design patterns

- Stable rule IDs.
- Keywords used as prefilters before regex execution.
- Entropy as optional supporting evidence.
- Rule-specific and global allowlists.
- Allowlist targeting of match, line, path, or secret.
- Rule tags and descriptions.
- Finding fingerprints for durable suppression.
- Config extension and rule override behavior.
- Composite or proximity-based rules.
- Multiple report formats.
- Backward-compatible configuration migrations followed by deprecation.

### Lessons to adopt

- Every recognizer needs a stable ID and version.
- Prefilters can reduce unnecessary regex work.
- Allowlists must be explicit, explainable, and visible in diagnostic reports.
- Suppression should target stable fingerprints rather than fragile line numbers when possible.
- Composite evidence can outperform one giant regex.
- Configuration changes require migration and deprecation policy.
- Rule metadata and reporting formats matter to downstream automation.

### Lessons to avoid

- Do not import file-path and Git-history concepts into a text-only core.
- Do not add entropy as a universal proxy for sensitivity.
- Do not allow suppression mechanisms to erase evidence during evaluation.

## 7. Rust PII interface crates

Reference: [docs.rs/pii](https://docs.rs/pii)

This crate describes a deterministic, auditable pipeline with an `NlpEngine`, recognizers, policy, capabilities, stable byte offsets, controlled degradation, and optional Candle-based NER.

### Strategic implication

The ecosystem is converging on similar abstractions:

- analyzer;
- recognizer interface;
- NLP capability seam;
- byte spans;
- policy or anonymization stage; and
- optional model adapters.

Convergence is useful evidence for the target architecture, but it also increases duplication risk. Phase 0 should investigate whether shared interfaces or collaboration are realistic.

## 8. Rust quality and compatibility tooling

### `cargo-semver-checks`

References:

- [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks)
- [Cargo SemVer compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)

Lesson:

- Public Rust APIs have many non-obvious compatibility rules.
- Add automated semantic-version checks after the first versioned baseline.
- Do not rely on reviewer memory to catch every breaking change.
- Serialized formats and behavioral semantics still require additional project-specific compatibility tests.

### `cargo-fuzz`

Reference: [rust-fuzz/cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz)

Lesson:

- Parser, span, Unicode, overlap, anonymization, and configuration boundaries are well suited to coverage-guided fuzzing.
- Retain minimized failures as regression cases.
- Seed fuzz targets from representative integration fixtures without allowing generated fuzz cases to contaminate canonical test data.

### `cargo-deny`

Reference: [EmbarkStudios/cargo-deny](https://github.com/EmbarkStudios/cargo-deny)

Lesson:

- Dependency governance includes licenses, advisories, duplicate versions, banned crates, build scripts, and source restrictions.
- A private project preparing for future publication should establish allowed licenses and sources before the dependency graph becomes large.
- Exceptions must include reasons and owners.

### Criterion

Reference: [criterion.rs](https://github.com/criterion-rs/criterion.rs)

Lesson:

- Use statistics-driven benchmarks for regression detection.
- Keep quality evaluation separate from microbenchmarks.
- Report full workload definitions and environment, not only attractive throughput numbers.

### Rust `regex`

Reference: [rust-lang/regex](https://github.com/rust-lang/regex)

Lesson:

- The regex engine avoids traditional catastrophic backtracking, but resource limits still matter for untrusted pattern construction and very large automata.
- Predefined patterns should compile at build or construction time with tests.
- Consumer-supplied patterns need explicit size and compilation limits.
- Linear-time matching does not mean constant memory or acceptable latency for unlimited input.

## Common lessons across the landscape

### 1. Extensibility is normal; stable evidence is rarer

Many projects provide recognizer traits and registries. Fewer make recognizer provenance, version, evaluation receipt, and decision evidence first-class.

Potential differentiation:

- evaluation-backed recognizer admission;
- explainable findings;
- immutable model and rule identity;
- reproducible release evidence; and
- conservative, explicit security-boundary semantics.

### 2. Broad feature lists create dependency and maintenance gravity

NER, OCR, encryption, servers, CLI, WASM, mobile, and FFI are individually plausible and collectively dangerous to an early library.

Use optional adapters and split only at real dependency or release boundaries.

### 3. Rule quality is a product, not a file of regexes

A mature recognizer needs:

- stable identity;
- source and attribution;
- examples and counterexamples;
- structural validation;
- context behavior;
- allowlist behavior;
- evaluation results;
- locale scope;
- performance characteristics; and
- maintenance ownership.

### 4. Evaluation must precede superiority claims

Related projects frequently advertise speed, memory, accuracy, or production readiness. `presidio-rs` should compare configurations through reproducible harnesses and resist inherited marketing language.

### 5. Configuration becomes permanent quickly

Once consumers store rule IDs, entity names, thresholds, and operator settings, those strings become an API. Delay serialized configuration until the concepts stabilize, then version it explicitly.

### 6. Multi-consumer support requires downstream testing

A library can compile perfectly while breaking consumer assumptions about:

- output tags;
- score meaning;
- overlap resolution;
- feature defaults;
- serialized fields;
- MSRV; and
- error behavior.

Maintain real or fixture downstream consumers in CI.

## Build, adopt, collaborate, or stop scorecard

During Phase 0, score each credible project from 1 to 5 on:

| Dimension | Weight |
|---|---:|
| Correctness and source-offset semantics | 5 |
| Evaluation quality and transparency | 5 |
| Offline and embeddable operation | 4 |
| Dependency and supply-chain posture | 4 |
| Extensibility and consumer fit | 4 |
| Maintenance activity and governance | 4 |
| API stability and migration discipline | 3 |
| MSRV and platform compatibility | 3 |
| License and provenance | 5 |
| Collaboration feasibility | 3 |
| Differentiation opportunity | 4 |

Possible conclusions:

- **Adopt:** use the project directly and contribute missing capabilities upstream.
- **Collaborate:** align corpus, interfaces, recognizers, or benchmarks while retaining separate implementations.
- **Depend:** build a narrow adapter or policy layer around the existing project.
- **Continue independently:** requirements are materially distinct and defensible.
- **Stop:** another project is better and differentiation is insufficient.

## Landscape risks not to ignore

1. The ecosystem may standardize around another crate name or API before publication.
2. A competitor may improve faster because it accepts native dependencies or a higher MSRV.
3. Model and dataset licenses may eliminate an apparently ideal semantic option.
4. Parallel projects may share copied patterns with unclear provenance.
5. An inactive-looking project may resume development after this plan begins.
6. Consumer demand may favor a CLI or service rather than a library.
7. A broad Rust port may win discoverability while a technically cleaner core remains obscure.
8. Collaboration may be cheaper technically but harder organizationally.
9. Project naming may imply affiliation and require a clean public rename.
10. Existing projects' benchmarks may use incomparable configurations.

## Scheduled revisit dates

- August 14, 2026: Phase 0 build/adopt/collaborate decision.
- October 9, 2026: architecture and ecosystem refresh before recognizer expansion completes.
- December 4, 2026: consumer-pilot and collaboration reassessment.
- January 22, 2027: semantic-backend and ecosystem decision.
- February 19, 2027: publication and positioning review.
