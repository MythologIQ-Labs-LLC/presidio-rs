# Presidio Archaeology and Secure Alpha Model

## Purpose

This document defines how `presidio-rs` will learn from the mature Python Presidio project without copying its implementation mechanically or claiming behavioral compatibility that has not been proven.

The work has two linked goals:

1. extract architectural, security, evaluation, configuration, operational, and governance lessons from Presidio's history; and
2. convert those lessons into a concrete secure and functional alpha boundary for the Rust project.

The useful inheritance is not only the current Python API. It includes the decisions that changed over time, the bugs that forced those changes, the false-positive and false-negative tradeoffs, the deployment warnings, the configuration failures, the overlap fixes, the model-integration friction, the release engineering, and the governance transition.

A Rust rewrite is not automatically an improvement. The project must be able to explain which Presidio decisions it adopts, adapts, rejects, or defers, and why.

## Source-of-truth program

The archaeology program reviews primary project evidence in this order:

1. current Presidio architecture and concept documentation;
2. current source code and tests for analyzer, registry, NLP, context, anonymizer, operators, structured data, and service boundaries;
3. changelog and release notes, with special attention to breaking changes and repeated bug classes;
4. accepted and rejected design discussions;
5. issues and pull requests involving false positives, false negatives, offsets, overlap, model loading, configuration, security, performance, and compatibility;
6. Presidio Research evaluation formats, generators, metrics, and error analysis;
7. migration guides and deprecated APIs;
8. release, packaging, supply-chain, contribution, and governance history; and
9. current community-owned transition and maintenance model.

Every material lesson becomes an evidence row containing:

- subsystem;
- source URL and immutable commit or release identity where practical;
- observed problem or requirement;
- Presidio decision;
- known benefits;
- known costs or regressions;
- applicability to the Rust project;
- Rust decision: adopt, adapt, reject, defer, or investigate;
- required test or evaluation evidence; and
- owner and target issue.

## Secure alpha definition

A secure alpha does **not** guarantee that all PII will be detected. Presidio explicitly warns that automated detection has false positives and false negatives and must be combined with other protections.

For `presidio-rs`, secure alpha means:

- supported scope is explicit and measured;
- unsupported scope is explicit and does not masquerade as coverage;
- source identity and UTF-8 offsets are exact;
- invalid, ambiguous, mismatched, truncated, or unsupported operations fail explicitly;
- anonymization is planned and validated before output is produced;
- partial transformation is never presented as complete success;
- resolution policy is explicit and versioned;
- recognizer provenance and decision evidence are retained without plaintext logging by default;
- default recognizers are conservative, locale-aware, and evidence-backed;
- input, candidate, issue, output, and backend resource behavior is bounded;
- cryptographic operations have safe defaults and clear correlation or reversibility semantics;
- the core initiates no network or filesystem I/O;
- dependency, provenance, history, DCO, MSRV, and package gates remain enforced;
- public security reporting has a usable confidential path; and
- a fresh external consumer can reproduce documented behavior.

Secure alpha is a trustworthy failure and transformation boundary. It is not a claim of comprehensive detection, regulatory compliance, production certification, or stable `1.0` compatibility.

## Functional alpha boundary

### In scope

The alpha should provide:

- in-process synchronous Rust analysis of UTF-8 text;
- a bounded `TextDocument` and `AnalysisRequest` path;
- a deliberately small default set of structured recognizers;
- strict recognizer metadata, provenance, locale, country, and capability selection;
- custom pattern recognizers and backend-neutral custom recognizers;
- candidate-preserving analysis reports;
- compact and diagnostic evidence levels;
- explicit candidate resolution policies;
- fallible document-bound replacement, redaction, and masking;
- source-to-output operation records;
- reproducible evaluation on synthetic and redistributable corpora;
- false-positive and false-negative regression fixtures;
- initial fuzz and property targets;
- runnable examples and downstream compile fixtures; and
- clear migration from the legacy-compatible API.

### Experimental or restricted in alpha

The following may exist only behind explicit experimental status:

- deterministic hashing or pseudonymization;
- serialized configuration;
- semantic or NER adapters;
- batch APIs;
- report serialization as a durable wire protocol; and
- custom transformation operators.

### Out of scope for alpha

The alpha should not absorb:

- a hosted HTTP service;
- authentication or authorization infrastructure;
- OCR, image, DICOM, audio, or video redaction;
- structured or tabular policy orchestration;
- automatic model downloads;
- runtime telemetry containing plaintext;
- a general compliance platform;
- streaming redaction without explicit boundary guarantees;
- stable cross-language bindings; or
- a claim of drop-in Microsoft Presidio compatibility.

## Presidio lessons that directly shape alpha

### 1. Analyzer and anonymizer remain separate contracts

Presidio separates detection from transformation. `presidio-rs` should retain this boundary and make the handoff stronger through exact document binding, validated resolution, and a fallible anonymization plan.

**Rust decision:** adopt and strengthen.

### 2. Recognizer registry, recognition mechanics, NLP capability, and context are separate concerns

Presidio's analyzer architecture separates `RecognizerRegistry`, `EntityRecognizer`, `NlpEngine`, and `ContextAwareEnhancer`. This separation enabled multiple recognizer mechanisms and model integrations, but configuration and lifecycle complexity grew around them.

**Rust decision:** adopt the separation, but keep the core contract smaller and immutable after construction. Heavy NLP lifecycle belongs in optional adapters.

### 3. Detection is domain-specific and must be evaluated by the consumer

Presidio describes itself as a customizable SDK and warns that every detection approach trades false positives against false negatives. Its research project separates evaluation and error analysis from the runtime.

**Rust decision:** make evaluation receipts and per-entity error analysis a release requirement. Never market entity presence in an enum or table as proof of useful coverage.

### 4. Safe defaults matter more than recognizer count

Presidio later disabled many country-specific recognizers by default because enabling them broadly caused false positives outside their intended context.

**Rust decision:** default-enable only recognizers with measured value in the declared locale and scope. Country-specific and weak-pattern recognizers remain opt-in.

### 5. Context must include negative evidence and explain score changes

Presidio introduced context enhancement, later expanded recognizer-level and cross-entity context, and continues to evolve negative-context support. Context is powerful but can create opaque score inflation and new false positives.

**Rust decision:** context is a separate, versioned policy stage with positive and negative evidence, boundary-aware matching, proximity, locale behavior, and diagnostic receipts. Context scores are heuristic unless calibrated by evaluation.

### 6. Span alignment is a first-class correctness problem

Model and tokenizer integrations can skip or misalign spans. Presidio users have reported annotations being dropped when external pipelines could not align model spans with source text.

**Rust decision:** all public spans remain byte offsets into original UTF-8. Normalized, tokenized, or model views must map exactly back to source coordinates or return a typed failure. Approximate spans are not acceptable for transformation.

### 7. Overlap behavior cannot remain an incidental sort

Presidio's anonymizer documents different behavior for full overlap, containment, and partial intersection, and its changelog records repeated conflict-handling fixes.

**Rust decision:** resolution is a named, versioned policy with preserved candidates. The core must support at least report-all, best-candidate, and conservative-redaction union behavior. Equal-score and partial-intersection behavior must be deterministic.

### 8. Anonymization must be atomic and auditable

Presidio accepts analyzer results as transformation input and has evolved overlap handling over time. The Rust project can improve this boundary by validating the complete operation plan before modifying text.

**Rust decision:** no authoritative anonymization output until document identity, spans, resolution, operators, and policy conflicts all validate. Reports include applied operations, source spans, output spans, warnings, and rejected operations.

### 9. Hashing and pseudonymization require explicit security semantics

Presidio changed its hash operator to use random salt by default because deterministic outputs were vulnerable to brute-force and dictionary attacks. That security improvement broke previous referential-integrity behavior unless callers explicitly managed salt.

**Rust decision:** the current deterministic salted SHA-256 behavior is not acceptable as an authoritative secure-alpha default. The project must either disable it on the document-aware path or replace it with reviewed semantics such as keyed HMAC with secret wrappers, domain separation, rotation guidance, and explicit correlation behavior.

### 10. Configuration becomes a compatibility surface

Presidio has repeatedly evolved YAML, model configuration, recognizer loading, and analyzer configuration. Its changelog includes configuration parsing and model-configuration fixes, and a unified analyzer configuration remains active work.

**Rust decision:** programmatic typed construction remains primary through alpha. Serialized configuration is deferred until concepts stabilize, then requires schema versions, validation, fingerprints, unknown-field rules, and migrations.

### 11. Model loading and lifecycle affect memory, startup, and embedding

Presidio users requested the ability to reuse loaded spaCy models rather than load duplicates, and Presidio later refactored model and NER integration.

**Rust decision:** optional semantic adapters receive model handles or immutable model identities from consumers. The core does not own runtime downloads, global model caches, devices, or async runtimes.

### 12. Structured data, images, and services are separate products

Presidio grew distinct modules for text, structured data, and image redaction. Structured-data support required its own design discussion because field context, tabular policy, and free-text cells are different from plain text.

**Rust decision:** do not widen the alpha core. Structured data, OCR, service deployment, and platform adapters remain separate work with their own threat models and dependency graphs.

### 13. Service security belongs at the service boundary

Presidio's API containers intentionally omit built-in authentication and warn against direct exposure without a gateway or equivalent infrastructure.

**Rust decision:** the core crate remains network-free. Any future service wrapper must define authentication, authorization, rate limiting, request limits, observability, and deployment hardening separately. Example servers must not imply safe internet exposure.

### 14. Explainability is operationally necessary

Presidio includes decision-process tracing to understand why findings were produced. This supports tuning, debugging, and false-positive analysis.

**Rust decision:** diagnostic reports must explain recognizer, pattern or model identity, validator result, context changes, thresholding, resolution, and limits without copying matched plaintext by default.

### 15. Batch processing is not merely a loop

Presidio distinguishes batch engines and model batch processing because setup, model inference, errors, and throughput differ from single-text analysis.

**Rust decision:** batch support follows the single-document correctness contract. It is not an alpha blocker and must not weaken per-document identity, limits, errors, or receipts.

### 16. Supply-chain and release operations are part of security

Presidio's recent releases include pinned CI actions, pinned build tooling, dependency updates for vulnerabilities, and trusted publishing improvements.

**Rust decision:** continue immutable action pinning, dependency advisory checks, provenance scans, dry-run publication, package inspection, and eventually trusted crates.io publishing. The current green source tree is not a substitute for artifact identity.

### 17. Governance and ownership transitions are architectural events

Presidio is transitioning from Microsoft ownership to a community-owned Data Privacy Stack project. Repository, package, documentation, support, and release identities are moving with it.

**Rust decision:** stable IDs, neutral documentation, maintainer succession, domain ownership, release authority, and project naming must be treated as durable operational contracts rather than README decoration.

## Alpha design tree

```text
Can the input be represented exactly as a bounded TextDocument?
  no  -> typed rejection
  yes -> select recognizers by explicit metadata and request policy
           |
           v
       Can each recognizer produce exact source spans and bounded evidence?
         no  -> typed backend issue; caller policy decides whether analysis is incomplete
         yes -> preserve validated candidates
                  |
                  v
              Apply explicit threshold and context policy
                  |
                  v
              Apply named resolution policy
                report-all | best-candidate | conservative-redaction | custom
                  |
                  v
              Validate complete anonymization plan
                source identity | spans | overlaps | operator support | output bounds
                  |
             no   |   yes
          error   v
              execute atomically and return an auditable report
```

## Alpha must-pass gates

### Correctness

- deterministic overlap, nesting, adjacency, and tie behavior;
- preserved source candidates and recorded resolution decisions;
- exact UTF-8 and document identity validation;
- atomic transformation plan and explicit errors;
- no silent truncation, skipped operation, or approximate span;
- differential tests against the legacy path where behavior is intentionally compatible.

### Security

- bounded input, candidate, issue, output, and regex behavior;
- fuzzing for spans, requests, resolution, transformation, and serialization;
- no plaintext diagnostic logging by default;
- authoritative hash or pseudonymization behavior either corrected or disabled;
- dependency advisories, supply-chain controls, package inspection, and provenance gates;
- threat model and caller-responsibility documentation;
- confidential vulnerability intake tested.

### Functional behavior

- documented default entity scope;
- runnable analysis and anonymization examples;
- custom pattern and custom backend examples;
- locale and recognizer selection;
- compact and diagnostic reports;
- migration guidance and downstream compile fixtures.

### Evaluation

- versioned corpus schema;
- exact-span and overlap-tolerant metrics;
- per-entity and per-recognizer results;
- false-positive and false-negative fixtures;
- template-family separation for synthetic data;
- provenance and licensing records;
- machine-readable receipts tied to engine and recognizer versions.

### Operations

- protected `main` and required checks;
- reproducible public clone and package dry run;
- release and rollback ownership;
- public intake and security-reporting paths;
- explicit alpha support posture and known limitations.

## Presidio archaeology workstreams

### A. Architecture and lifecycle

Review analyzer, registry, recognizer, NLP engine, context enhancer, anonymizer, operator, batch, structured, image, and service boundaries.

Deliverable: subsystem dependency map and adopt/adapt/reject ledger.

### B. Behavioral history

Trace repeated changes involving duplicates, overlap, whitespace merging, end-of-text entities, context, allowlists, weak patterns, country defaults, token alignment, and score handling.

Deliverable: regression taxonomy and required Rust test families.

### C. Security history

Review hash and encryption changes, security advisories, service authentication warnings, secret scanning, dependency controls, action pinning, trusted publishing, and vulnerability handling.

Deliverable: Rust threat model deltas and secure-default decisions.

### D. Configuration and compatibility

Review V1-to-V2 migration, YAML recognizer loading, NLP configuration, language and country filtering, deprecations, package splits, API response changes, and current unified-configuration work.

Deliverable: typed construction policy, deferred serialized schema requirements, and compatibility-risk register.

### E. Evaluation and claims

Review Presidio Research schemas, synthetic generation, template leakage controls, metrics, error analysis, and comparison practices.

Deliverable: `presidio-rs` evaluation schema and claim-evidence policy.

### F. Ecosystem and governance

Review package identity, module boundaries, contribution workflow, issue history, maintenance patterns, and transition to Data Privacy Stack.

Deliverable: maintainership, naming, collaboration, and migration lessons.

## Differential learning harness

The project should build a test-only, non-runtime differential harness capable of running selected synthetic fixtures through:

- the current Python Presidio release;
- `presidio-rs` legacy-compatible analysis;
- `presidio-rs` request-oriented analysis; and
- each Rust resolution policy.

The harness does not make Python Presidio a normative oracle. Differences are classified as:

- intentional Rust safety improvement;
- intentional scope difference;
- Python behavior worth matching;
- Rust defect;
- Python defect or disputed behavior;
- taxonomy mismatch; or
- unresolved evidence gap.

Fixtures should prioritize known historical failure classes rather than only common happy-path examples.

## Immediate sequence

1. Complete the Presidio evidence ledger and subsystem map.
2. Freeze the secure-alpha scope and unsupported claims.
3. Specify resolution behavior using Presidio overlap history as input, not as an unquestioned contract.
4. Implement fallible document-bound anonymization.
5. Correct or disable authoritative deterministic hashing.
6. Add compact and diagnostic decision tracing.
7. Establish conservative default recognizers and locale rules.
8. Build evaluation fixtures, differential learning cases, and initial fuzz targets in parallel.
9. Revalidate through two materially different Rust consumers.
10. Reassess whether independent development, collaboration, adoption, narrowing, or migration creates the most value.

## Stop and redirect conditions

Independent development should pause when evidence shows that:

- an existing Rust project satisfies the core requirements with lower maintenance risk;
- collaboration can produce shared contracts or evaluation assets faster;
- Presidio compatibility requirements conflict with the safer Rust contract;
- alpha security depends on capabilities outside available maintainer capacity;
- evaluation cannot substantiate useful default detection scope; or
- governance, naming, or support obligations exceed sustainable ownership.

Stopping, narrowing, collaborating, or migrating is a successful evidence-based outcome. Sunk cost remains incapable of maintaining a privacy library.

## Primary references

- Presidio documentation: <https://microsoft.github.io/presidio/>
- Presidio analyzer architecture: <https://microsoft.github.io/presidio/analyzer/>
- Presidio anonymizer and overlap behavior: <https://microsoft.github.io/presidio/anonymizer/>
- Presidio supported entities: <https://microsoft.github.io/presidio/supported_entities/>
- Presidio FAQ and deployment warning: <https://github.com/data-privacy-stack/presidio/blob/main/docs/faq.md>
- Presidio changelog: <https://github.com/data-privacy-stack/presidio/blob/main/CHANGELOG.md>
- Presidio Research: <https://github.com/microsoft/presidio-research>
- Structured-data design discussion: <https://github.com/microsoft/presidio/discussions/714>
- Span-alignment issue: <https://github.com/microsoft/presidio/issues/1262>
- Loaded-model reuse issue: <https://github.com/microsoft/presidio/issues/822>
- Weak driver-license pattern issue: <https://github.com/microsoft/presidio/issues/1063>
