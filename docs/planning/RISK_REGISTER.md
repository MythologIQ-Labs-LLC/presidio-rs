# Development Risk and Assumption Register

## Use

Review this document daily through the July 30 visibility change, weekly through the August 14 contributor-alpha gate, and at every later phase exit.

Risks R-001 and R-029 are also governed by the [Active Rust Privacy Landscape Watch](../research/ACTIVE_LANDSCAPE_WATCH.md): weekly monitoring, monthly source-level review, and immediate escalation when a material ecosystem change is detected.

Risk ratings are qualitative:

- **Likelihood:** Low, Medium, High
- **Impact:** Low, Medium, High, Critical
- **State:** Open, Mitigating, Accepted, Closed, Triggered

## Active risks

| ID | Risk | Likelihood | Impact | Early warning | Mitigation and contingency | Decision point | State |
|---|---|---|---|---|---|---|---|
| R-001 | An existing Rust project is a better technical and ecosystem fit | Medium | High | Comparable API, active maintenance, stronger coverage, evidence, adoption, or collaboration potential | Maintain weekly ecosystem watch and monthly source comparison; evaluate adoption, collaboration, migration, or a narrow adapter within five working days of a material finding | Continuous | Mitigating |
| R-002 | Project differentiation is too weak | Medium | High | Positioning reduces to “Presidio in Rust” or unmeasured speed claims | Validate constrained-runtime, evidence, provenance, and consumer needs; narrow or redirect if absent | Aug 14, 2026 | Open |
| R-003 | UTF-8 normalization and source-offset mapping require substantial redesign | Medium | Critical | Findings cannot reliably map to original text; consumer examples fail | Preserve original byte semantics; prototype normalization mapping only with consumer evidence; reject approximate spans | Sep 4, 2026 | Mitigating |
| R-004 | Overlap resolution causes false-negative redaction | Medium | Critical | Nested or higher-scoring short match hides a larger sensitive span | Preserve candidates; add explicit policies and conservative union behavior; adversarial fixtures | Aug 28, 2026 | Open |
| R-005 | Anonymization silently produces unsafe output | Medium | Critical | Invalid offsets, mismatched text, skipped operations, or partial output occur | Fallible planning API, document identity, full preflight validation, explicit operation report | Sep 4, 2026 | Open |
| R-006 | Evaluation corpus overstates quality | High | High | Synthetic results greatly exceed consumer results; template leakage | Multiple corpus families, template-family splits, slice reporting, consumer fixtures | Sep 4, 2026 | Open |
| R-007 | Corpus or model licensing blocks redistribution or use | Medium | High | Unclear terms, non-commercial restrictions, personal data concerns | Provenance review before inclusion; keep unapproved data out of repository | Each corpus/model decision | Open |
| R-008 | Consumer requirements create incompatible APIs | Medium | High | Conflicting entity, error, runtime, or serialization requirements | Two distinct pilots; keep policy outside core; compatibility facade; reject narrow coupling | Sep 25, 2026 | Open |
| R-009 | Public API stabilizes before behavior is understood | High | High | Public users depend on transitional reports, identifiers, or behavior | Label alpha stability categories; migration guide; consumer fixtures; semver checks after baseline | Aug 14 and Sep 25, 2026 | Mitigating |
| R-010 | Custom recognizer interface becomes too broad or leaks internals | Medium | Medium | Trait requires mutable engine access or consumer-specific state | Keep narrow value-based trait; validate with independent implementations | Sep 25, 2026 | Mitigating |
| R-011 | Recognizer breadth creates unmaintainable rule quality | High | High | Patterns lack validators, counterexamples, locale scope, evidence, or owners | Freeze breadth during release and correctness work; require admission evidence | Ongoing | Mitigating |
| R-012 | Dependency growth undermines minimal and offline value | Medium | High | Native libraries, runtime downloads, multiple runtimes, or high MSRV enter core | Adapter crates, dependency ADR, license/source checks, feature isolation | Before each dependency | Open |
| R-013 | Rust 1.74 MSRV blocks useful dependencies | Medium | Medium | Security or parser libraries require newer compiler | Gather consumer requirements; isolate optional adapters; change only with migration rationale | Oct 30, 2026 | Open |
| R-014 | Consumer-supplied patterns cause excessive compile memory or latency | Medium | High | Large automata, slow startup, memory spikes | Pattern size limits, validation, resource tests, explicit errors | Oct 16, 2026 | Open |
| R-015 | False negatives undermine security trust | High | Critical | Consumer incidents or poor recall in required entities | Publish supported envelope; default only evaluated recognizers; conservative policies; no guarantee claims | Ongoing | Open |
| R-016 | False positives make consumers disable the library | High | High | High suppression volume, user complaints, bypass behavior | Context, validators, allowlists, slice metrics, consumer tuning outside core | Ongoing | Open |
| R-017 | Pseudonymization design is reversible or correlation-prone | Medium | Critical | Low-entropy values recoverable; salt treated as secret; cross-tenant correlation | Keep limitations prominent; design keyed replacement separately; independent review | Oct 16, 2026 | Open |
| R-018 | Semantic backend creates large burden for limited gain | High | High | Large artifact, native runtime, slow startup, modest recall improvement | Keep outside critical path; collaborate, defer, or reject | Post-Oct 2026 review | Accepted |
| R-019 | Semantic model labels do not match the PII taxonomy | High | Medium | PERSON and LOCATION improve while required sensitive entities remain missed | Map labels explicitly; evaluate by entity; avoid general PII claims | Before semantic adoption | Accepted |
| R-020 | Streaming redaction leaks values across chunk boundaries | High | Critical | Split identifiers pass undetected | Keep streaming out of initial scope; require boundary and watermark design | Before streaming work | Accepted |
| R-021 | WASM, FFI, CLI, and service requests fragment the roadmap | Medium | High | Multiple surfaces appear before Rust API evidence | Primary Rust-library surface; require demonstrated consumers and separate decisions | Ongoing | Mitigating |
| R-022 | Performance work optimizes benchmarks rather than consumer outcomes | Medium | Medium | Microbench gains do not affect real integration cost | Representative workloads, p95/p99, memory, construction, consumer benchmarks | Oct 16, 2026 | Open |
| R-023 | Maintainer capacity cannot support a public security-sensitive library | Medium | Critical | Issues or advisories wait; one owner becomes unavailable | Quiet visibility release, explicit response limits, backup ownership, narrow scope, collaborate if necessary | Jul 30 and Aug 14, 2026 | Mitigating |
| R-024 | Project name implies Microsoft affiliation or collides with existing crates | Medium | Critical | Ecosystem confusion, trademark concern, crate-name unavailability | Complete review before visibility; rename or strengthen distancing language; defer package name if needed | Jul 29, 2026 | Triggered |
| R-025 | MIT licensing is regretted after outside use | Medium | High | Commercial boundary or competitor use becomes unacceptable | Confirm distribution intent before visibility; document public/commercial boundary; legal review | Jul 29, 2026 | Triggered |
| R-026 | CLA or contribution terms are legally insufficient | Medium | High | Public contribution arrives before legal review | Do not merge third-party contribution under unreviewed terms; document temporary contribution handling | Before first external merge | Mitigating |
| R-027 | Internal consumer pressure reintroduces product coupling | High | Medium | Core gains orchestration, policy, or private identifiers used by one product | Require independent reusable justification and architecture review | Every consumer PR | Mitigating |
| R-028 | Schedule optimism consumes quality controls | High | Critical | Checks are skipped, review narrows, visibility date becomes absolute regardless of evidence | Hard safety blockers; scope freeze; clean export or rename rather than unsafe exposure; no check weakening | Daily through Jul 30 | Mitigating |
| R-029 | Parallel projects change during development | High | High | Another crate adds required capability, stronger evidence, adoption, or reusable subsystem | Weekly monitoring; monthly source review; five-day build/adopt/collaborate response | Continuous | Mitigating |
| R-030 | Real PII or secrets enter test corpora or CI artifacts | Medium | Critical | Fixtures contain live credentials or personal data | Synthetic or approved data only; secret scanning; corpus review; artifact retention controls | Before corpus merge | Open |
| R-031 | Public history exposes secrets, confidential references, or deleted proprietary material | Medium | Critical | Secret scan, manual review, or clone rehearsal finds sensitive history | Scan full intended history; remove affected material or publish a clean audited export | Jul 28, 2026 | Triggered |
| R-032 | Public users misinterpret repository visibility as production readiness | High | High | README excerpts omit caveats; downstream use treats alpha APIs as stable | Prominent status and non-goals; alpha API classifications; no launch claims; issue templates reinforce scope | Jul 30 and Aug 14, 2026 | Mitigating |
| R-033 | Quiet public release unexpectedly attracts more attention than maintainers can handle | Medium | High | Issue, PR, or security volume exceeds capacity | No advertising; triage labels and templates; response expectations; archive or restrict scope if necessary | First two public weeks | Open |
| R-034 | Public release operations are incomplete despite code readiness | Medium | High | Anonymous clone fails, links break, vulnerability reporting unavailable, branch protections absent | Clean-clone rehearsal, link check, settings verification, runbook and rollback checklist | Jul 29, 2026 | Open |
| R-035 | Acceleration leaves API migration and compatibility expectations unclear | High | High | External users depend on legacy or transitional APIs without guidance | Public API inventory, alpha classification, migration guide, downstream fixtures | Aug 14, 2026 | Open |

## Assumption ledger

| ID | Assumption | Confidence | Evidence needed | If false | Revisit |
|---|---|---|---|---|---|
| A-001 | Pattern-based detection is independently useful without NER | Medium | Public feedback and two consumer pilots | Narrow or redirect project | Aug 14 and Sep 25, 2026 |
| A-002 | Synchronous core APIs fit primary Rust consumers | Medium | Consumer runtime inventory | Add adapter or reconsider contract | Sep 25, 2026 |
| A-003 | Original UTF-8 byte offsets are the correct external coordinate system | High | Normalization and consumer tests | Rework report contract | Sep 4, 2026 |
| A-004 | One primary crate is simpler during stabilization | High | Dependency and release review | Split if real adapter boundaries emerge | Every architecture review |
| A-005 | Rust 1.74 remains a useful MSRV | Medium | Consumer requirements and dependency matrix | Raise MSRV with migration plan | Oct 30, 2026 |
| A-006 | Two distinct consumer pilots can complete by Sep 25 | Medium | Named pilot owners and windows | Reforecast beta confidence | Aug 14, 2026 |
| A-007 | Consumer-specific policy can remain outside the core | Medium | Pilot design review | Revisit boundary or create policy crate | Sep 25, 2026 |
| A-008 | Evaluation corpora can be licensed and stored safely | Medium | Legal and provenance review | Use generated or externally hosted fixtures | Sep 4, 2026 |
| A-009 | The backend-neutral recognizer trait is ergonomic and fast enough | Medium | Independent implementations and benchmarks | Use enum dispatch, generics, or revised interface | Sep 25, 2026 |
| A-010 | No demonstrated `no_std` requirement exists | High | Consumer inventory | Add a separate feasibility phase | Every consumer review |
| A-011 | WASM is optional rather than critical | Medium | Consumer inventory | Add targeted platform work | Sep 25, 2026 |
| A-012 | Public source visibility can safely precede beta maturity when limitations are explicit | Medium | Release-week audit, public feedback, and absence of critical exposure defects | Use clean export, temporarily revert visibility, or narrow exposed scope | Jul 30 and Aug 14, 2026 |
| A-013 | The current public name is acceptable for source visibility even if package naming remains unresolved | Low | Naming and affiliation review | Rename before visibility or package publication | Jul 29, 2026 |
| A-014 | Semantic recognition can remain off the critical path | High | Consumer requirements | Reforecast around model capability | Sep 25, 2026 |
| A-015 | Maintainer capacity can absorb a quiet public release | Medium | Named ownership and first two weeks of public activity | Narrow scope, communicate limits, recruit help, or collaborate | Jul 30 and Aug 14, 2026 |
| A-016 | No existing or emerging Rust project makes independent development inferior | Low | Weekly watch, monthly comparison, consumer migration analysis | Adopt, collaborate, narrow, redirect, or stop | Weekly and every phase exit |

## Escalation rule

A risk requires immediate architecture or release review when:

- a Critical-impact risk moves to High likelihood;
- public exposure may include a secret, confidential item, or material without publication rights;
- naming or affiliation risk may require a rename before visibility;
- a consumer needs a breaking contract change;
- a new native, network, model, or cryptographic dependency is proposed;
- a real data leak or unsafe transformation is discovered;
- a material landscape finding could provide a better path; or
- the accelerated schedule is used to justify weakening a required check.

## Closure evidence

Do not mark a risk closed because activity occurred. Record evidence such as:

- full-history scan result;
- clean-export comparison;
- provenance inventory;
- naming or legal review;
- passing clean-clone rehearsal;
- evaluation receipt;
- regression fixture;
- accepted ADR;
- consumer acceptance;
- dependency audit;
- benchmark result;
- fuzzing artifact; or
- documented decision to remove the capability or material from scope.