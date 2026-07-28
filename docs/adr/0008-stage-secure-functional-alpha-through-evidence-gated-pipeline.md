# ADR 0008: Stage the secure functional alpha through an evidence-gated pipeline

- **Status:** Accepted
- **Date:** 2026-07-28
- **Decision owners:** MythologIQ Labs LLC maintainers
- **Related issues:** #14, #33, #34, #35, #36, #37

## Context

The repository is publicly readable and already provides validated spans, bounded identifiers, source-bound findings, candidate-preserving reports, recognizer metadata, bounded analysis requests, and backend-neutral recognizer execution.

Those capabilities establish a public foundation, but they do not yet form a secure functional alpha. Candidate resolution remains transitional, the authoritative document-bound anonymization path is not implemented, diagnostic decision tracing is incomplete, default recognizer policy is not yet evidence-gated, evaluation is not reproducible, and the legacy deterministic hash operator has security-sensitive correlation semantics.

Microsoft Presidio provides useful architecture, release, issue, and evaluation history. Its value is not limited to the current Python API. Repeated changes involving overlaps, context, false-positive defaults, tokenizer alignment, configuration, model lifecycle, anonymization, hashing, packaging, and governance provide evidence about where mature systems fail.

The Rust project must learn from that history without treating Python behavior as a normative oracle or copying Python class boundaries into Rust.

## Decision

The project will distinguish two alpha gates:

1. **Public foundation alpha:** the repository is public, buildable, documented, contributable, and operationally governed.
2. **Secure functional alpha:** the authoritative text pipeline provides a bounded, explicit, reproducible, and failure-safe analysis and transformation contract.

The secure functional alpha is delivered through this ordered pipeline:

```text
Presidio evidence and decision ledger
        |
        v
Bounded TextDocument and AnalysisRequest
        |
        v
Explicit recognizer selection and safe defaults
        |
        v
Validated source-bound candidate collection
        |
        v
Boundary-aware context and threshold policy
        |
        v
Named and versioned resolution policy
        |
        v
Complete anonymization-plan validation
        |
        v
Atomic transformation and auditable report
        |
        v
Evaluation receipts, regressions, fuzzing, and consumer evidence
```

### Architectural rules

1. Presidio archaeology is a design input for material alpha decisions. Each relevant lesson is classified as adopt, adapt, reject, defer, or investigate.
2. Resolution is implemented before authoritative anonymization because anonymization requires a defined set of resolved findings.
3. Original candidates remain available independently from resolved findings.
4. Context, thresholding, allowlists, denylists, and resolution are explicit policy stages and produce bounded evidence.
5. The authoritative anonymizer validates the complete plan before producing output. Source mismatch, invalid spans, unresolved conflicts, unsupported operators, and output-limit failures return errors rather than partial success.
6. Default recognizers require locale and country applicability, provenance, regression evidence, and an evaluation receipt. Recognizer count is not a maturity metric.
7. The authoritative alpha path supports replacement, redaction, and masking. Deterministic hashing is disabled or explicitly experimental until issue #37 establishes reviewed semantics.
8. Programmatic typed construction remains the primary configuration API. Serialized configuration is deferred until the underlying contracts stabilize.
9. Semantic recognition, model lifecycle, runtime downloads, structured data, OCR, hosted services, streaming, and stable cross-language bindings remain outside the secure functional alpha.
10. Evaluation tooling remains separable from the runtime dependency graph and treats Python Presidio as a comparison source rather than a compatibility oracle.
11. The secure functional alpha gate requires reproducible evaluation, historical regression fixtures, initial fuzz and property targets, and at least one downstream compile fixture.
12. Broader recognizer expansion begins only after the secure functional alpha contract is green and measured.

## Secure functional alpha contract

The gate requires:

- explicit supported and unsupported scope;
- exact original UTF-8 byte offsets and document identity;
- bounded input, candidates, issues, backend work, and output;
- conservative evidence-backed defaults;
- compact and diagnostic reports without plaintext by default;
- deterministic and versioned resolution behavior;
- atomic document-bound anonymization;
- source-to-output operation records;
- safe or unavailable cryptographic transformation semantics;
- reproducible evaluation and claims evidence;
- retained historical regressions;
- initial fuzz and property testing; and
- a clean external consumer path.

The gate does not claim complete PII detection, production certification, regulatory compliance, drop-in Presidio compatibility, stable `1.0` compatibility, or support for every locale and entity.

## Consequences

### Positive

- Correctness and security risks are addressed before feature breadth.
- Presidio history becomes actionable evidence instead of informal inspiration.
- The transformation boundary becomes stronger than the legacy compatibility path.
- Default behavior remains small, explainable, and measurable.
- Consumers can distinguish raw detection evidence from policy decisions.
- Evaluation and compatibility work begin early enough to influence architecture.

### Costs

- New recognizers and semantic features are delayed.
- The alpha requires additional documentation, ADRs, fixtures, and evaluation infrastructure.
- Some Python Presidio behavior will intentionally differ and must be explained.
- The legacy API remains transitional while the authoritative pipeline matures.
- Hashing and pseudonymization may be unavailable in the authoritative alpha until reviewed semantics exist.

## Rejected alternatives

### Port Presidio feature breadth first

Rejected because recognizer breadth would increase false-positive, maintenance, configuration, and evaluation burden before the transformation boundary is trustworthy.

### Treat Python Presidio as the compatibility oracle

Rejected because some Python behavior reflects historical constraints, service assumptions, or defects that Rust should not reproduce.

### Implement anonymization before resolution

Rejected because the anonymizer would consume undefined overlap and selection behavior.

### Include serialized configuration in the alpha

Rejected because configuration would freeze unstable recognizer, resolution, context, and operator contracts into a second public API.

### Retain deterministic hashing as an ordinary alpha operator

Rejected because correlation, brute-force, domain separation, secret management, and rotation semantics are not currently adequate for the authoritative path.

## Follow-up

- #33 completes the Presidio decision ledger.
- #34 freezes and verifies the secure functional alpha contract.
- #35 delivers decision tracing, context evidence, and safe defaults.
- #36 builds differential learning and historical regression evidence.
- #37 resolves authoritative hashing and pseudonymization behavior.
- #14 implements the secure functional alpha pipeline and closes only when its exit evidence is reproducible.
