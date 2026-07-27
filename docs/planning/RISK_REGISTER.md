# Development Risk and Assumption Register

## Use

Review this document every two weeks during active development and at every phase exit.

Risks R-001 and R-029 are also governed by the [Active Rust Privacy Landscape Watch](../research/ACTIVE_LANDSCAPE_WATCH.md): weekly monitoring, monthly source-level review, and immediate escalation when a material ecosystem change is detected.

Risk ratings are qualitative:

- **Likelihood:** Low, Medium, High
- **Impact:** Low, Medium, High, Critical
- **State:** Open, Mitigating, Accepted, Closed, Triggered

## Active risks

| ID | Risk | Likelihood | Impact | Early warning | Mitigation and contingency | Decision point | State |
|---|---|---|---|---|---|---|---|
| R-001 | An existing Rust project is a better technical and ecosystem fit | Medium | High | Comparable API, active maintenance, stronger coverage, evidence, adoption, or collaboration potential | Maintain a weekly ecosystem watch and monthly source-level comparison; immediately evaluate adoption, collaboration, migration, or a narrow adapter when a material alternative emerges | Continuous; formal review at every phase exit | Mitigating |
| R-002 | Project differentiation is too weak | Medium | High | Positioning reduces to “Presidio in Rust” or unmeasured speed claims | Validate constrained-runtime, evidence, provenance, and consumer needs; stop or narrow scope if absent | Aug 14, 2026 | Open |
| R-003 | UTF-8 normalization and source-offset mapping require substantial redesign | Medium | Critical | Findings cannot reliably map to original text; consumer examples fail | Address before recognizer expansion; prototype mapping; reject approximate spans | Sep 4, 2026 | Open |
| R-004 | Overlap resolution causes false-negative redaction | Medium | Critical | Nested or higher-scoring short match hides a larger sensitive span | Preserve candidates; add conservative union policy; adversarial fixtures | Sep 18, 2026 | Open |
| R-005 | Anonymization silently produces unsafe output | Medium | Critical | Invalid offsets, mismatched text, or skipped operations occur | Fallible planning API, document identity, preflight validation, explicit report | Sep 18, 2026 | Open |
| R-006 | Evaluation corpus overstates quality | High | High | Synthetic results greatly exceed consumer results; template leakage | Multiple corpus families, template-family splits, slice reporting, consumer fixtures | Oct 2, 2026 | Open |
| R-007 | Corpus or model licensing blocks redistribution or use | Medium | High | Unclear terms, non-commercial restrictions, personal data concerns | Provenance review before inclusion; keep unapproved data out of repository | Each corpus/model decision | Open |
| R-008 | Consumer requirements create incompatible APIs | Medium | High | One consumer needs open entities, another relies on enum exhaustiveness; conflicting error semantics | Use two distinct pilots; keep policy outside core; compatibility facade; reject narrow coupling | Dec 4, 2026 | Open |
| R-009 | Public API stabilizes before behavior is understood | High | High | Serialized reports or IDs are used downstream while semantics still change | Label experimental surfaces; version schemas; consumer fixtures; semver checks after baseline | Jan 8, 2027 | Open |
| R-010 | Custom recognizer interface becomes too broad or leaks internals | Medium | Medium | Trait requires mutable engine access or consumer-specific state | Narrow value-based trait; validate with three mechanisms; ADR before expansion | Oct 16, 2026 | Open |
| R-011 | Recognizer breadth creates unmaintainable rule quality | High | High | Many patterns lack validators, counterexamples, locale scope, or owners | Admission checklist, evaluation receipt, stable IDs, ownership, limited default set | Ongoing | Open |
| R-012 | Dependency growth undermines minimal/offline value | Medium | High | Native libraries, runtime downloads, multiple runtimes, or high MSRV enter core | Adapter crates, cargo-deny, dependency ADR, feature isolation | Before each dependency | Open |
| R-013 | Rust 1.74 MSRV blocks useful dependencies | Medium | Medium | Security or parser libraries require newer compiler | Gather consumer MSRV requirements; isolate optional adapters; change only with migration rationale | Oct 30, 2026 | Open |
| R-014 | Consumer-supplied patterns cause excessive compile memory or latency | Medium | High | Large automata, long startup, memory spikes | Pattern size limits, compile limits, validation, optional precompilation, explicit errors | Jan 8, 2027 | Open |
| R-015 | False negatives undermine security trust | High | Critical | Real consumer incidents or poor recall in required entities | Publish supported envelope; default only evaluated recognizers; conservative policies; no guarantee claims | Ongoing | Open |
| R-016 | False positives make consumers disable the library | High | High | High suppression volume, user complaints, bypass behavior | Context, validators, allowlists, slice metrics, consumer tuning outside core | Ongoing | Open |
| R-017 | Pseudonymization design is reversible or correlation-prone | Medium | Critical | Low-entropy values recoverable; salt treated as secret; cross-tenant correlation | Deprecate salted hash; keyed design; domain separation; independent review | Dec 18, 2026 | Open |
| R-018 | Semantic backend creates large operational burden for limited gain | High | High | Large artifact, native runtime, slow startup, modest recall improvement | Optional spike; explicit stop criteria; collaborate or defer | Jan 22, 2027 | Open |
| R-019 | Semantic model labels do not match PII taxonomy | High | Medium | PERSON/LOC work but sensitive domain entities remain missed | Map labels explicitly; evaluate by entity; avoid claiming general PII NER | Jan 22, 2027 | Open |
| R-020 | Streaming redaction leaks values across chunk boundaries | High | Critical | Split identifiers pass undetected | Keep streaming out of initial scope; require watermark and boundary permutation tests | Before streaming work | Accepted |
| R-021 | WASM, FFI, CLI, and service requests fragment the roadmap | Medium | High | Multiple packaging surfaces appear before Rust API stability | Primary Rust-library surface; require demonstrated consumers and separate decision | Ongoing | Mitigating |
| R-022 | Performance work optimizes benchmarks rather than consumer outcomes | Medium | Medium | Microbench improvements do not change end-to-end cost | Use representative workloads, p95/p99, memory, construction, consumer benchmarks | Jan 8, 2027 | Open |
| R-023 | Maintainer capacity cannot support a security-sensitive library | Medium | Critical | Reviews delayed, advisories ignored, single owner unavailable | Name backups, limit scope, remain private, or collaborate with larger project | Feb 12, 2027 | Open |
| R-024 | Public project name implies Microsoft affiliation or collides with existing crates | Medium | High | Ecosystem confusion, crate-name unavailability, trademark concern | Keep name provisional; legal and ecosystem review; clean rename before publication | Feb 19, 2027 | Open |
| R-025 | MIT licensing is regretted after outside use | Low while private | High | Commercial boundary becomes unclear | Keep repository private; revisit license before public distribution; legal review | Public release decision | Mitigating |
| R-026 | CLA or contribution terms are legally insufficient | Medium | High | Outside contribution proposed before legal review | Do not accept public third-party contribution under draft CLA; obtain counsel review | Before public contribution | Mitigating |
| R-027 | Internal consumer pressure reintroduces product coupling | High | Medium | Core gains orchestration, policy, or private identifiers used by one product | Require independent reusable justification and architecture review | Every consumer PR | Mitigating |
| R-028 | Schedule optimism consumes contingency | High | High | Phase work carries over repeatedly; architecture review skipped | Reforecast at each phase; protect 20% reserve; cut scope before quality | Every phase exit | Open |
| R-029 | Parallel projects change during development | High | High | Another crate adds required capability, stronger evidence, maintenance capacity, adoption, or a reusable subsystem | Weekly conditional monitoring; monthly deep source review; maintain comparison matrix; trigger build-versus-adopt review within five working days of a material change | Continuous | Mitigating |
| R-030 | Real PII or secrets enter test corpora or CI artifacts | Medium | Critical | Fixtures contain live credentials or personal data | Synthetic or approved data only; secret scanning; corpus review; artifact retention controls | Before corpus merge | Open |

## Assumption ledger

| ID | Assumption | Confidence | Evidence needed | If false | Revisit |
|---|---|---|---|---|---|
| A-001 | Pattern-based detection is independently useful without NER | Medium | Two consumer interviews and pilot results | Narrow or redirect project | Aug 14 and Dec 4, 2026 |
| A-002 | Synchronous core APIs fit primary Rust consumers | Medium | Consumer runtime inventory | Add adapter or reconsider contract | Aug 14, 2026 |
| A-003 | Original UTF-8 byte offsets are the correct external coordinate system | High | Normalization and consumer tests | Rework report contract | Sep 4, 2026 |
| A-004 | One primary crate is simpler during stabilization | High | Dependency and release review | Split earlier if heavy adapters arrive | Every architecture review |
| A-005 | Rust 1.74 remains a useful MSRV | Medium | Consumer requirements and dependency matrix | Raise MSRV with migration plan | Oct 30, 2026 |
| A-006 | Two distinct consumer pilots can be scheduled | Medium | Named pilot owners and windows | Reforecast compatibility confidence | Sep 18, 2026 |
| A-007 | Consumer-specific policy can stay outside the core | Medium | Pilot design review | Revisit boundary or create policy crate | Dec 4, 2026 |
| A-008 | Evaluation corpora can be licensed and stored safely | Medium | Legal and provenance review | Use generated or externally hosted fixtures | Oct 2, 2026 |
| A-009 | A backend-neutral recognizer trait is ergonomic and fast enough | Medium | Three implementations and benchmarks | Use enum dispatch, generics, or revised interface | Oct 30, 2026 |
| A-010 | No demonstrated `no_std` requirement exists | High | Consumer inventory | Add feasibility phase, not an incidental feature | Every consumer review |
| A-011 | WASM is optional rather than critical | Medium | Consumer inventory | Add targeted platform phase | Dec 4, 2026 |
| A-012 | Public release can wait until after private beta | High | Strategy review | Establish controlled collaboration mechanism | Feb 2027 |
| A-013 | The current name is temporary for private development | High | Naming review | Rename earlier if integrations make the name sticky | Oct 30, 2026 |
| A-014 | Semantic recognition can remain off the critical path | High | Consumer requirements | Reforecast program around model capability | Aug 14, 2026 |
| A-015 | Maintainer and reviewer capacity remains available for 30 weeks | Medium | Confirmed ownership calendar | Reduce scope or extend schedule | Every phase exit |
| A-016 | No existing or emerging Rust project makes independent development the inferior path | Low | Weekly watch, monthly deep comparison, consumer migration analysis | Adopt, collaborate, narrow, redirect, or stop | Weekly and every phase exit |

## Escalation rule

A risk requires immediate architecture review when any of the following occurs:

- a Critical-impact risk moves to High likelihood;
- a phase exit depends on an unvalidated assumption with Low confidence;
- a consumer needs a breaking contract change;
- a new native, network, model, or cryptographic dependency is proposed;
- a real data-leak or unsafe transformation is discovered;
- a material landscape-watch finding could invalidate planned work, reduce differentiation, or provide a better consumer path; or
- the schedule consumes more than half of remaining contingency before Phase 5.

## Closure evidence

Do not mark a risk closed because work was performed. Record the evidence that makes the risk no longer material, such as:

- evaluation receipt;
- passing regression fixture;
- accepted ADR;
- consumer acceptance;
- legal review;
- dependency audit;
- benchmark result;
- fuzzing artifact; or
- documented decision to remove the capability from scope.
