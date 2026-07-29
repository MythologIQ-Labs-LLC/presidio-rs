# Presidio Resolution Decision Ledger

## Scope

This is the alpha-critical resolution slice of issue #33's broader Presidio archaeology program. It records the upstream evidence used to freeze ADR 0009 and the implementation matrix for issues #40 through #42.

Python Presidio is treated as a mature evidence source. Its behavior is not automatically normative for Rust.

## Source set

Primary upstream evidence reviewed for this slice:

- current Presidio Anonymizer documentation, section **Handling overlaps between entities**;
- Presidio changelog and releases;
- Presidio PR #1092, recorded as the initial logic check for merging two entities;
- Presidio PR #1196, recorded as improved conflict handling in `AnonymizerEngine`;
- Presidio PR #1588, recorded as sorting analyzer results by start and end for correct whitespace merging;
- release history entries for duplicate removal, same-entity merging, end-of-sentence behavior, and overlap fixes.

The current documentation was reviewed on July 28, 2026. Release and PR numbers provide stable historical identities where available.

## Decision ledger

| ID | Subsystem | Upstream identity | Observed problem | Presidio decision | Benefit and cost | Rust applicability | Rust decision | Required evidence | Issue |
|---|---|---|---|---|---|---|---|---|---|
| RES-001 | Full overlap | Current anonymizer docs | Two findings cover the same or overlapping source region | Prefer higher score; equal-score selection is arbitrary | Simple, but equal-score behavior is nondeterministic | Authoritative output must be repeatable and audit-friendly | **Adapt** | Total tie-break tests and permutation invariance | #40, #41 |
| RES-002 | Containment | Current anonymizer docs | One finding is fully contained in another | Prefer the larger span even when its score is lower | Maximizes transformed coverage, but overrides confidence semantics | Selection and safe redaction are distinct caller goals | **Reject for `BestCandidate`; adopt through `ConservativeRedaction`** | Lower-confidence outer-span fixture | #40, #41 |
| RES-003 | Partial intersection | Current anonymizer docs | Findings overlap without containment | Transform each separately and concatenate operator output | Avoids dropping either entity, but couples conflict handling to mutation and can create synthetic adjacency | Rust resolution must finish before transformation | **Reject** | Partial-intersection matrix and future anonymization-plan test | #40, #41, #42 |
| RES-004 | Exact duplicates | Changelog 2.2.23 and duplicate-removal history | Multiple recognizers or passes emit the same result | Remove duplicates | Reduces redundant output; historical fixes show this is easy to get wrong | Raw evidence must retain cardinality while selection may collapse duplicates | **Adapt** | `ReportAll` preserves duplicates; `BestCandidate` collapses them | #41 |
| RES-005 | Same-entity overlap | Changelog 2.2.32 and PR #1092 | Two findings of the same entity overlap or touch | Merge behavior was revised over time | Can reduce fragmented output but risks hidden span expansion | Must distinguish overlap from adjacency and preserve evidence | **Adapt** | Same-entity overlap and adjacency fixtures | #40, #41 |
| RES-006 | General conflict handling | PR #1196, changelog 2.2.352 | Existing pairwise logic did not cover all conflicts safely | Improve `AnonymizerEngine` conflict logic | Mature behavior, but still part of transformation implementation | Rust needs a pure resolver with a complete conflict graph | **Adapt** | Chained-overlap and full permutation tests | #41 |
| RES-007 | Ordering | PR #1588 and release 2.2.359 history | Input order affected whitespace and merge behavior | Sort analyzer results by start and end | Fixes one ordering class, but ordering remains implicit | Ordering must be normative and independent of caller order | **Adopt and strengthen** | Canonical candidate table and total ordering tests | #41 |
| RES-008 | End boundary | Changelog 2.2.25 | Entities at the end of a sentence were mishandled | Correct terminal-span behavior | Prevents missed or malformed transformation | UTF-8 source boundaries and document length are hard invariants | **Adopt as regression** | End-of-document span fixtures | #36, #41, #42 |
| RES-009 | Whitespace merging | PR #1588 | Incorrect result order caused whitespace to merge incorrectly | Sort before transformation | Repairs output layout, but indicates mutation was making policy decisions | Rust will resolve first and transform from a complete plan | **Reject coupling; adopt fixture** | Whitespace-separated and adjacent fixtures | #36, future anonymization issue |
| RES-010 | Equal confidence | Current anonymizer docs | Scores do not distinguish conflicting findings | Selection is arbitrary | Easy implementation, unsuitable for reproducible security behavior | Rust requires a stable total order | **Reject** | Same-span equal-score fixtures with entity and recognizer ties | #40, #41 |
| RES-011 | Conflict evidence | Current architecture and docs | Consumers cannot inspect why one overlap won | Behavior described but per-decision evidence is not the central contract | Simpler API, weaker auditability | Secure alpha requires bounded explanation without plaintext | **Adapt** | Typed retained, rejected, duplicate, and union decisions | #35, #41, #42 |
| RES-012 | Mixed entities | Current partial-intersection and containment behavior | One source region can contain findings with different entity types | Operators are applied according to selected or separate findings | Maintains operator-specific behavior, but no neutral union identity | Conservative coverage must not invent one source entity | **Adapt** | `Single` versus `Mixed` union identity tests | #40, #41 |
| RES-013 | Adjacent spans | No stable upstream rule suitable for the Rust contract | Two spans touch but do not overlap | Behavior can be affected by merge and whitespace logic | Merging may be convenient but silently broadens scope | Version 1 needs a narrow mathematical definition | **Defer adjacency merging** | Adjacent spans remain separate under all policies | #40, #41 |
| RES-014 | Chained overlap | Repeated conflict-handling changes imply pairwise complexity | A overlaps B, B overlaps C, A does not overlap C | Existing behavior is implementation-dependent across stages | Pairwise code is easy to reason about locally and wrong globally | Complete connected components or precedence selection are required | **Adapt** | Bridge-candidate cases with every permutation | #41 |
| RES-015 | Legacy compatibility | Current `presidio-rs` private `dedupe_overlaps` | Legacy output keeps a locally highest-scoring result against the last retained result | Preserve established crate behavior | Avoids breaking consumers, but is not a permanent policy | New resolution must be additive and differential-tested | **Adopt as compatibility boundary** | Before-and-after legacy parity test | #36, #41, #42 |

## Frozen alpha decisions

The following questions are no longer open for version 1:

- Adjacent spans remain separate.
- Strict overlap is the only conflict relation.
- `ReportAll` preserves every qualifying candidate, including duplicates.
- `BestCandidate` is deterministic greedy selection using the total precedence order in ADR 0009.
- `ConservativeRedaction` unions connected overlap components.
- Mixed-entity unions are represented as `Mixed`, never as a fabricated entity ID.
- Decision references use canonical candidate ordinals.
- Caller input order cannot affect output.
- Wrong or inconsistent document bindings are errors.
- Resolution limits are explicit and cannot produce an unmarked partial-success claim.
- Legacy output remains unchanged.

## Evidence gaps retained for later work

This slice does not settle:

- positive and negative context scoring;
- model-token to source-byte alignment;
- locale and country default enablement;
- structured-data conflict policy;
- image or OCR coordinate behavior;
- transformation operator composition;
- hashing or pseudonymization;
- streaming resolution; or
- cross-language wire compatibility.

Those remain governed by issues #33 through #37 and the secure-alpha roadmap.
