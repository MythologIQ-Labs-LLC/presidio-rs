# ADR 0009: Version explicit candidate-resolution policies

- **Status:** Accepted
- **Date:** 2026-07-28
- **Decision owners:** MythologIQ Labs LLC maintainers
- **Related issues:** #14, #33, #34, #36, #40, #41, #42

## Context

The request-oriented analyzer preserves validated source-bound candidates in `AnalysisReport::candidates()`. The legacy-compatible projection still applies a private overlap-deduplication function whose behavior is not named, versioned, explained, or suitable as the permanent input contract for document transformation.

Python Presidio provides useful evidence but not one universal policy. Its current anonymizer documentation distinguishes full overlap, containment, and partial intersection. It chooses the higher score for full overlap, chooses the larger span for containment even when the score is lower, and transforms partially intersecting findings separately. Equal-score full overlaps are documented as arbitrary. Presidio's release history also records repeated changes to duplicate removal, same-entity merging, conflict handling, result ordering, whitespace merging, and end-of-text behavior.

Those choices are valuable evidence about failure classes. They are not copied as the Rust contract because the authoritative Rust path must be deterministic, input-order invariant, source-bound, policy-explicit, and suitable for a later atomic anonymization plan.

## Decision

The authoritative path will expose three named policies with stable identifiers and version `1`:

- `report_all/v1`
- `best_candidate/v1`
- `conservative_redaction/v1`

The policy API may be idiomatic Rust, but these semantics are normative.

### Common rules

1. Resolution operates on validated `Finding` values and never reads or copies source plaintext.
2. The resolver creates a canonical candidate table before applying policy. Decision references use canonical ordinals, not caller vector indexes.
3. Canonical ordering is independent of caller input order.
4. Two spans conflict only when they overlap strictly: `left.start < right.end && right.start < left.end`.
5. Adjacent spans do not conflict and remain separate under every version-1 policy.
6. Open `EntityId` and `RecognizerId` values remain supported.
7. Document bindings are preserved on selected candidates. A conservative union is valid only when all contributing candidates have the same binding state and, when bound, the same exact binding.
8. Candidate, resolved-output, and decision-evidence limits are explicit. A limit condition is reported and never disguised as complete success.
9. Raw candidates remain available independently from policy output.
10. Legacy `dedupe_overlaps` behavior remains unchanged and is not relabeled as one of the new policies.

### Canonical candidate ordering

Candidates are ordered by this total semantic key:

1. span start ascending;
2. span end ascending;
3. entity identifier ascending;
4. recognizer identifier ascending, with a present identifier before an absent identifier;
5. confidence descending;
6. stable evidence representation ascending; and
7. exact document-binding representation ascending.

Candidates equal under the complete key are exact duplicates. They remain distinct canonical entries so evidence can preserve cardinality, but version-1 policies may collapse them when the policy requires selection.

### `ReportAll` version 1

- Retain every candidate that satisfies the configured minimum confidence.
- Preserve exact duplicates.
- Return candidates in canonical order.
- Record summary evidence that no conflict elimination occurred.
- Never merge, select, or reject because of overlap.

This policy is for inspection, evaluation, and callers that own their own policy.

### `BestCandidate` version 1

Candidates are considered in deterministic precedence order:

1. explicitly listed entity priority;
2. explicitly listed recognizer priority;
3. higher confidence;
4. longer span;
5. earlier start;
6. earlier end;
7. entity identifier ascending;
8. recognizer identifier ascending; and
9. canonical ordinal ascending.

A listed priority outranks an unlisted value. Earlier entries in a configured priority list rank higher.

The resolver greedily retains a candidate when it does not overlap any already retained candidate. A candidate that overlaps a retained candidate is rejected with evidence identifying the retained conflicting candidate. Exact duplicates collapse to one retained candidate.

Consequences:

- Higher confidence can beat a larger containing span.
- Two non-overlapping candidates in a chained conflict can both survive if a competing bridge candidate ranks lower.
- The policy is deterministic and input-order invariant.
- The policy is not the safe default for irreversible redaction because selecting one candidate can leave bytes from a lower-ranked overlapping candidate outside the selected span.

### `ConservativeRedaction` version 1

- Build connected components using strict span overlap.
- Merge every component into the union span from the minimum start to maximum end.
- Keep adjacent components separate.
- Preserve canonical references to every contributing candidate.
- If all contributors share one entity identifier, expose that entity as `Single`.
- If contributors contain different entity identifiers, expose `Mixed` and do not invent a synthetic source entity.
- Do not assign one confidence value to a mixed union. Source candidate confidences remain in the candidate table.

This policy is the intended input to a later fail-closed redaction plan because every byte covered by any qualifying candidate in a conflict component remains covered by the union.

## Presidio evidence decisions

| Evidence | Rust decision | Reason |
|---|---|---|
| Full overlap chooses higher score; equal scores may be arbitrary | **Adapt** | Keep confidence precedence but add a total deterministic tie-break order. |
| Containment chooses the larger span even with lower score | **Reject for BestCandidate; adopt through ConservativeRedaction** | A single selection policy should not silently override declared confidence. Conservative union provides the coverage-safe option. |
| Partial intersections are transformed separately and concatenated | **Reject as a resolution contract** | Transformation behavior must follow an explicit resolved plan, not manufacture adjacent replacement output from overlapping source spans. |
| Duplicate and same-entity handling changed repeatedly | **Adopt as regression taxonomy** | Exact duplicates, same-span conflicts, and same-entity overlaps require dedicated fixtures. |
| Sorting fixes were required for whitespace merging | **Adapt** | Canonical ordering is part of the resolver contract and is tested across input permutations. |
| End-of-text and overlap bugs appeared in prior releases | **Adopt as adversarial fixtures** | Boundary and terminal-span cases are mandatory. |

## Conformance matrix

The normative table-driven matrix is maintained in [`docs/testing/RESOLUTION_CONFORMANCE_MATRIX.md`](../testing/RESOLUTION_CONFORMANCE_MATRIX.md).

Every implementation must cover:

- disjoint spans;
- exact duplicates;
- same span with different entities;
- same span with different recognizers;
- containment in both caller input orders;
- partial intersection in both caller input orders;
- chained overlap with three or more candidates;
- adjacency;
- equal and unequal confidence;
- explicit entity and recognizer priorities;
- every permutation of small conflict sets;
- Unicode-safe byte spans;
- open identifiers; and
- matching and mismatched document bindings.

## Consequences

### Positive

- The future anonymizer receives one explicit resolved-input contract.
- Evaluation can compare three policies without mutating raw evidence.
- The safe redaction posture is distinct from precision-oriented selection.
- Cross-language or Python comparisons can classify differences rather than hiding them.
- Input ordering no longer controls security behavior.

### Costs

- Reports may duplicate a canonical candidate snapshot to provide stable references.
- Best-candidate behavior intentionally differs from Python Presidio containment behavior.
- Conservative unions require an explicit mixed-entity representation.
- Decision evidence and limits add API surface before anonymization begins.

## Rejected alternatives

### Rename the existing legacy deduplication function as `BestCandidate`

Rejected because its semantics are incomplete, compare only against the last retained result, and are not stable across all conflict graphs.

### Copy Python Presidio conflict behavior exactly

Rejected because equal-score behavior is documented as arbitrary, partial-intersection behavior is coupled to transformation, and containment precedence is unsuitable as the only Rust policy.

### Merge adjacent spans conservatively

Rejected for version 1. Adjacency is not overlap, and silently merging it would remove intervening semantic boundaries. A future policy version may add explicit adjacency rules.

### Implement anonymization before resolution

Rejected because transformation would consume undefined overlap semantics and would be forced to recreate private resolver logic.

## Follow-up

- #40 closes when the evidence ledger and conformance matrix are merged.
- #41 implements the pure resolver without analyzer integration.
- #42 integrates the resolver additively with `AnalysisReport`.
- The following round builds fallible document-bound anonymization over `ResolutionReport`.
