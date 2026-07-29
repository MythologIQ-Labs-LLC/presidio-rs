# Resolution Conformance Matrix

## Status

- **Contract:** ADR 0009
- **Policy versions:** `report_all/v1`, `best_candidate/v1`, `conservative_redaction/v1`
- **Primary implementation issue:** #41
- **Report integration issue:** #42

This matrix is normative for version-1 candidate resolution. Test implementations may use synthetic entity and recognizer identifiers, but expected spans, selected candidates, unions, and decision classes must match.

## Notation

A candidate is written as:

```text
name: [start,end) entity confidence recognizer
```

All offsets are UTF-8 byte offsets. Adjacency means `left.end == right.start`. Strict overlap means both `left.start < right.end` and `right.start < left.end`.

`BestCandidate` precedence is:

1. configured entity priority;
2. configured recognizer priority;
3. confidence descending;
4. span length descending;
5. start ascending;
6. end ascending;
7. entity ID ascending;
8. recognizer ID ascending; and
9. canonical ordinal ascending.

## Required cases

| ID | Candidates | `ReportAll/v1` | `BestCandidate/v1` | `ConservativeRedaction/v1` |
|---|---|---|---|---|
| R01 | `a:[0,4) A .8 r1`, `b:[6,9) B .7 r2` | retain `a,b` | retain `a,b` | unions `[0,4) A`, `[6,9) B` |
| R02 | two exact copies of `a:[0,4) A .8 r1` | retain both | retain one, collapse duplicate | union `[0,4) A`, contributors both |
| R03 | `a:[0,4) A .8 r1`, `b:[0,4) B .8 r1` | retain both | retain lexical entity winner absent priority | union `[0,4) Mixed` |
| R04 | `a:[0,4) A .8 r1`, `b:[0,4) A .8 r2` | retain both | retain lexical recognizer winner absent priority | union `[0,4) A` |
| R05 | `outer:[0,10) A .6 r1`, `inner:[2,6) B .9 r2` | retain both | retain `inner` | union `[0,10) Mixed` |
| R06 | `outer:[0,10) A .9 r1`, `inner:[2,6) B .6 r2` | retain both | retain `outer` | union `[0,10) Mixed` |
| R07 | `left:[0,6) A .7 r1`, `right:[4,10) B .8 r2` | retain both | retain `right` | union `[0,10) Mixed` |
| R08 | same as R07 with caller order reversed | same canonical result | same canonical result | same canonical result |
| R09 | `a:[0,5) A .8 r1`, `bridge:[4,9) B .7 r2`, `c:[8,12) C .8 r3` | retain all | retain `a,c`, reject `bridge` | union `[0,12) Mixed` |
| R10 | same as R09 with `bridge` confidence `.95` | retain all | retain `bridge`, reject `a,c` | union `[0,12) Mixed` |
| R11 | `a:[0,4) A .8 r1`, `b:[4,8) B .9 r2` | retain both | retain both | two separate unions |
| R12 | same span and confidence, entity priority `[B,A]` | retain both | retain `B` | union `Mixed` |
| R13 | same span, entity, confidence, recognizer priority `[r2,r1]` | retain both | retain `r2` | union single entity |
| R14 | same span and IDs, confidence `.9` versus `.8` | retain both | retain `.9` | union single entity, both contributors |
| R15 | overlapping open entities `customer.secret` and `tenant.reference` | retain both | apply normal precedence | union `Mixed` |
| R16 | candidates bound to the same document | preserve bindings | selected candidate preserves binding | union preserves common binding |
| R17 | overlapping candidates bound to different documents | report construction error | resolution error | resolution error |
| R18 | one bound and one unbound overlapping candidate | report construction error | resolution error | resolution error |
| R19 | Unicode source where spans align to valid byte boundaries | retain valid spans | resolve normally | union on valid byte boundaries |
| R20 | candidate below minimum confidence | exclude | exclude | exclude |
| R21 | zero qualifying candidates | complete empty report | complete empty report | complete empty report |
| R22 | resolved-output limit too small | explicit incomplete or error status | explicit incomplete or error status | explicit incomplete or error status |
| R23 | decision-evidence limit too small | output may remain complete only when evidence truncation is explicit | output may remain complete only when evidence truncation is explicit | output may remain complete only when evidence truncation is explicit |
| R24 | candidate limit exceeded before resolution | reject or mark source incomplete; never claim complete resolution | same | same |

## Permutation requirements

For R02 through R14, tests must execute every permutation of the input candidate vector when the case contains four or fewer candidates.

The following outputs must remain identical across permutations:

- canonical candidate content and ordinals;
- resolved finding content and order;
- decision class and canonical references;
- status flags;
- policy identity and version; and
- document binding.

## Property requirements

### All policies

- Resolution does not mutate the caller's candidate slice.
- Every candidate reference is within the canonical candidate table.
- Every output span is a valid source span.
- Repeated resolution with the same configuration is equal.
- Debug and serialized decision evidence contain no matched source plaintext.

### `ReportAll/v1`

- Output cardinality equals qualifying candidate cardinality.
- Exact duplicates remain distinct.
- No overlap rejection or merge decision is emitted.

### `BestCandidate/v1`

- No two retained output candidates overlap.
- Every rejected overlap references at least one retained conflicting candidate.
- A retained candidate is never lower in precedence than an overlapping rejected candidate considered before it.
- Exact duplicates produce one retained semantic candidate.

### `ConservativeRedaction/v1`

- No two union outputs overlap.
- Adjacent outputs remain separate.
- Every qualifying candidate span is fully covered by exactly one union output.
- Each union span equals the minimum start and maximum end of its connected overlap component.
- A union reports `Single(entity)` only when every contributor has that same entity.

## Differential classifications

When Python Presidio, the legacy Rust projection, and a version-1 policy differ, the fixture must record one classification:

- intentional Rust safety improvement;
- intentional scope difference;
- Python behavior worth matching;
- Rust defect;
- upstream defect or disputed behavior;
- taxonomy mismatch; or
- unresolved evidence gap.

Initial expected classifications:

| Case | Difference | Classification |
|---|---|---|
| Equal-score full overlap | Python documents arbitrary selection; Rust uses a total tie-break | intentional Rust safety improvement |
| Lower-confidence outer containment | Python anonymizer chooses larger span; `BestCandidate/v1` chooses confidence, `ConservativeRedaction/v1` covers union | intentional Rust policy split |
| Partial intersection | Python transformation concatenates separate operator output; Rust resolves before transformation | intentional Rust safety improvement |
| Chained overlap | Legacy Rust compares against the last retained result; version-1 policies operate on the complete conflict graph | Rust legacy compatibility difference |
| Adjacency | Version-1 policies keep spans separate | explicit Rust scope decision |

## Required implementation evidence

The #41 PR must include:

- table-driven tests corresponding to every row;
- permutation tests;
- explicit policy identity and version assertions;
- limit and status tests;
- Rust 1.74 verification;
- optional-serde tests when the feature is enabled; and
- confirmation that legacy analyzer output is unchanged.
