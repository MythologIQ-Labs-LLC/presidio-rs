# First Secure-Alpha Development Round

## Purpose

This document defines the first legitimate implementation round after the public architecture rebaseline.

The round does not add recognizer breadth. It establishes the first missing authoritative contract in the secure functional alpha pipeline: explicit, deterministic candidate resolution while preserving the original candidate evidence.

## Why resolution is first

The current request-oriented path already produces validated, document-bound `Finding` candidates and preserves them in `AnalysisReport`.

The current legacy-compatible projection still applies a private overlap-deduplication function with transitional behavior. That behavior:

- is not represented by a public policy identity;
- does not distinguish containment from partial intersection;
- does not define all equal-confidence ties;
- considers only the most recently retained overlapping result;
- does not produce decision evidence;
- cannot express conservative union behavior; and
- returns only selected legacy results rather than the reasons for selection or rejection.

Authoritative anonymization cannot safely begin until overlap and selection behavior is explicit. Otherwise the transformation layer would consume an accidental algorithm as if it were policy, a traditional software-development maneuver with an unusually reliable regret yield.

## Round objective

Deliver an additive, source-preserving resolution subsystem that:

- accepts validated `Finding` candidates;
- never mutates or discards the source candidate collection;
- returns resolved findings separately;
- identifies the exact resolution policy and version;
- behaves deterministically across input order;
- explains every keep, reject, merge, or tie decision using bounded non-plaintext evidence;
- supports the secure-alpha policies `ReportAll`, `BestCandidate`, and `ConservativeRedaction`;
- retains the legacy projection unchanged until differential evidence supports migration; and
- gives the future anonymizer a validated, explicit input contract.

## Preconditions

Before the resolution API is declared stable-for-alpha:

1. Issue #33 must contain the Presidio evidence relevant to overlap, conflict handling, duplicates, score ties, containment, partial intersections, and anonymizer behavior.
2. Issue #34 must freeze the supported resolution scope and threat assumptions.
3. The implementation PR must include the relevant ADR, migration, README, API-status, and changelog changes.

Research and contract work may proceed in parallel with internal type design, but public policy semantics are not frozen until the evidence review is complete.

## Engineering slices

### Slice 1: resolution value model and pure policy engine

Create an additive module containing the smallest complete public contract, expected to include concepts equivalent to:

- `ResolutionPolicy`;
- `ResolutionPolicyId` or another stable policy identity;
- `ResolvedFinding`;
- `ResolutionDecision`;
- `ResolutionReport`; and
- `ResolutionError` where invalid source candidates or unsupported policy state can still be represented.

The exact names remain subject to implementation review. The required semantics do not.

The resolver must be pure with respect to source text and must not perform anonymization. It operates only on already validated findings and policy metadata.

### Slice 2: policy semantics

#### `ReportAll`

- retain every qualifying candidate;
- apply deterministic ordering;
- record that no conflict elimination occurred.

#### `BestCandidate`

- select findings using documented precedence;
- define full overlap, containment, partial intersection, duplicate, and equal-confidence behavior;
- use a total deterministic tie-break order;
- never depend on input vector order.

The initial precedence should use only stable evidence already present in the model. Entity or recognizer priority must not be invented implicitly. If priority is needed, it must be explicit policy input.

#### `ConservativeRedaction`

- merge the union of overlapping qualifying spans for transformation safety;
- define whether merely adjacent spans remain separate;
- retain all source candidates contributing to a merged span;
- avoid inventing a false single entity identity when multiple entities contributed; and
- produce bounded merge evidence suitable for a later anonymization report.

### Slice 3: report integration

Add an authoritative additive entry point, such as resolving an `AnalysisReport` or a validated candidate slice under an explicit policy.

Integration requirements:

- `AnalysisReport::candidates()` remains unchanged and authoritative as raw evidence;
- resolved findings are returned through a separate object or method;
- document binding is preserved;
- engine, policy, and policy-version identity is retained;
- candidate and decision limits are explicit;
- no legacy-compatible result silently changes in this round; and
- serialization remains additive and pre-1.0 experimental unless explicitly documented otherwise.

### Slice 4: differential and adversarial evidence

The round requires a table-driven matrix covering at least:

- disjoint spans;
- exact duplicates;
- same-span different entities;
- same-span different recognizers;
- full containment;
- partial intersection;
- chained overlaps involving three or more candidates;
- adjacency;
- equal confidence;
- different confidence;
- input-order permutations;
- Unicode-boundary-safe spans;
- open entity identifiers;
- document binding preservation; and
- deterministic repeated execution.

The matrix must compare:

- current legacy projection;
- `ReportAll`;
- `BestCandidate`; and
- `ConservativeRedaction`.

Differences are classified using the Presidio archaeology model rather than automatically treated as defects.

## Explicit non-goals

This round does not:

- implement document transformation;
- change the existing legacy analyzer or anonymizer output;
- add hashing or pseudonymization;
- add serialized configuration;
- add semantic recognition;
- expand the default recognizer set;
- claim compatibility with Python Presidio; or
- publish a crate.

## Pull-request structure

The preferred implementation sequence is three reviewable PRs:

1. **Evidence and contract freeze:** complete the overlap decision ledger, threat assumptions, ADR, and test matrix specification.
2. **Pure resolution engine:** implement value types, three policies, deterministic behavior, and table-driven tests without analyzer integration.
3. **Analysis-report integration:** expose the additive authoritative entry point, preserve document binding, add examples and migration guidance, and run downstream compile evidence.

The second PR is the first substantive Rust implementation. It should remain small enough that reviewers can reason about every conflict case without scrolling through unrelated analyzer refactors.

## Exit criteria

The round is complete when:

- all three policies have explicit versioned semantics;
- original candidates remain available unchanged;
- resolved findings and decision evidence are separate and bounded;
- behavior is deterministic across input order;
- all required overlap and tie families have tests;
- legacy behavior is captured in differential fixtures but not silently replaced;
- the public API status and migration documents describe the additive contract;
- the public-clone, history, CI, MSRV, DCO, dependency, package, and documentation gates pass; and
- the next anonymization round can consume a defined `ResolutionReport` rather than private overlap logic.

## Next round

After this round, the next legitimate development round is fallible document-bound anonymization:

1. validate a complete transformation plan from resolved findings;
2. reject wrong-document, invalid-span, unresolved-conflict, unsupported-operator, and output-limit conditions;
3. execute atomically;
4. return transformed text and source-to-output operation records; and
5. leave hashing unavailable on the authoritative path until issue #37 is resolved.
