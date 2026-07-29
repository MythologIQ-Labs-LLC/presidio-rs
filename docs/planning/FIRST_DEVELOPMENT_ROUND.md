# First Secure-Alpha Development Round

## Status

**Complete through PRs #51, #52, and #53.**

The repository now contains the Presidio-informed resolution contract, the pure versioned resolver, and exact-document `AnalysisReport` integration. Legacy analysis and anonymization behavior remain unchanged.

## Purpose

This document records the first legitimate implementation round after the public architecture rebaseline.

The round did not add recognizer breadth. It established the first missing authoritative contract in the secure functional alpha pipeline: explicit, deterministic candidate resolution while preserving the original candidate evidence.

## Why resolution came first

The request-oriented path already produced validated, document-bound `Finding` candidates and preserved them in `AnalysisReport`.

The legacy-compatible projection still applies a private overlap-deduplication function with transitional behavior. That behavior:

- is not represented by a public policy identity;
- does not distinguish containment from partial intersection;
- does not define all equal-confidence ties;
- considers only the most recently retained overlapping result;
- does not produce decision evidence;
- cannot express conservative union behavior; and
- returns only selected legacy results rather than the reasons for selection or rejection.

Authoritative anonymization could not safely begin until overlap and selection behavior was explicit. Otherwise the transformation layer would consume an accidental algorithm as if it were policy, a traditional software-development maneuver with an unusually reliable regret yield.

## Delivered outcome

The completed additive subsystem:

- accepts validated `Finding` candidates;
- never mutates or discards the caller-owned source candidate collection;
- returns resolved findings separately;
- identifies the exact resolution policy and version;
- behaves deterministically across input order;
- explains keep, reject, duplicate-collapse, and merge decisions using bounded non-plaintext evidence;
- supports `ReportAll`, `BestCandidate`, and `ConservativeRedaction` version 1;
- retains the legacy projection unchanged;
- validates exact document identity before report-integrated resolution;
- refuses candidate-truncated analysis rather than claiming complete resolution;
- preserves analyzer version, analysis status, issue count, and source binding; and
- gives the future anonymizer one explicit document-aware resolved-input contract.

## Delivered pull requests

### PR #51: evidence and contract freeze

Delivered:

- ADR 0009;
- Presidio resolution decision ledger;
- normative resolution conformance matrix;
- strict-overlap and adjacency rules;
- canonical candidate ordinals;
- deterministic precedence;
- mixed-entity union semantics;
- document-binding and limit behavior; and
- API and migration expectations.

### PR #52: pure resolution engine

Delivered:

- `resolve_candidates`;
- `ResolutionPolicy` and stable version-1 identities;
- `ResolutionOptions`;
- `ResolutionReport`;
- `ResolvedFinding` and `ResolvedEntity`;
- `ResolutionDecision` and `ResolutionStatus`;
- `ResolutionError`;
- hard candidate and resolved-output limits;
- explicit decision-evidence truncation;
- permutation-heavy conflict tests;
- a runnable resolution example; and
- public-clone rehearsal coverage.

### PR #53: analysis-report integration

Delivered:

- `AnalysisReport::resolve_for_document`;
- `ResolvedAnalysisReport`;
- `AnalysisResolutionError`;
- exact document validation before resolution;
- fail-closed candidate-completeness enforcement;
- retained analyzer and analysis-status context;
- open-entity integration coverage;
- repeated-resolution and legacy-parity coverage;
- a downstream public-API fixture; and
- integrated README, migration, API-status, changelog, and example updates.

## Version-1 policies

### `ReportAll`

- retains every qualifying candidate;
- preserves exact duplicates;
- applies deterministic canonical ordering;
- records that no conflict elimination occurred.

### `BestCandidate`

- applies explicit entity and recognizer priority where configured;
- then applies confidence, span length, source position, entity, recognizer, and canonical-ordinal precedence;
- handles duplicates, containment, partial intersection, chained overlap, and ties;
- never depends on caller input order.

### `ConservativeRedaction`

- merges connected strict-overlap components;
- keeps merely adjacent spans separate;
- retains every source candidate contributing to a union;
- reports `Single(entity)` only when all contributors agree;
- reports `Mixed` rather than inventing a false entity identity; and
- preserves exact common source binding.

## Evidence and tests

The round includes coverage for:

- disjoint spans;
- exact duplicates;
- same-span different entities;
- same-span different recognizers;
- full containment;
- partial intersection;
- chained overlaps involving three or more candidates;
- adjacency;
- equal and different confidence;
- explicit entity and recognizer priority;
- input-order permutations;
- Unicode-boundary-safe spans;
- open entity identifiers;
- matching, mismatched, bound, and unbound document states;
- candidate and output limits;
- decision-evidence truncation;
- deterministic repeated execution;
- raw-candidate immutability;
- legacy-projection preservation; and
- a clean downstream consumer workflow.

Differences from Python Presidio and the existing Rust compatibility path are classified through the archaeology model rather than automatically treated as defects.

## Explicit non-goals retained

This round did not:

- transform document text;
- change existing legacy analyzer or anonymizer output;
- add hashing or pseudonymization;
- add serialized configuration;
- add semantic recognition;
- expand the default recognizer set;
- claim drop-in compatibility with Python Presidio; or
- publish a crate.

## Exit evidence

The round is complete when the exact PR #53 head passes:

- formatting and Clippy;
- unit, integration, and doctests;
- rustdoc with warnings denied;
- Rust 1.74;
- DCO;
- dependency advisories;
- full history, provenance, and secret scanning;
- unauthenticated public-clone rehearsal;
- all documented examples;
- package inspection; and
- `cargo publish --dry-run` without publication.

PR #53 carries the final evidence for this round and closes issue #42 when merged.

## Next development round

The next legitimate development round is **fallible document-bound anonymization**:

1. define an explicit anonymization request and operator policy;
2. validate a complete transformation plan from `ResolvedAnalysisReport` or its contained `ResolutionReport`;
3. reject wrong-document, invalid-span, unresolved-conflict, unsupported-operator, and output-limit conditions;
4. calculate source-to-output offsets before mutation;
5. execute atomically only after the full plan validates;
6. return transformed text, operation records, policy identity, versions, status, and bounded non-plaintext evidence;
7. preserve legacy anonymization unchanged; and
8. leave hashing unavailable on the authoritative path until issue #37 establishes reviewed pseudonymization semantics.
