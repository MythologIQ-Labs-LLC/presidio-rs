# Parallel Efforts and Architectural Lessons

## Purpose

This review identifies lessons from related privacy, PII, secret-scanning, and Rust library efforts. It is not an endorsement of their claims or a decision to copy their designs.

The landscape is expected to change. This document is the deep-review baseline for the [Active Rust Privacy Landscape Watch](ACTIVE_LANDSCAPE_WATCH.md). Monitor material changes weekly, perform a source-level review at least monthly, and trigger an immediate architecture review when an alternative could alter the build-versus-adopt-versus-collaborate decision.

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
