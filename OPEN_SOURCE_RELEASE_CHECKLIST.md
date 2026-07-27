# Open-Source Release Checklist

This checklist separates public source visibility from contributor maturity, package publication, active promotion, and stable release.

The repository is targeted to become public on **Thursday, July 30, 2026**. Only Section A must be complete for that visibility change. Later sections govern later maturity decisions.

See:

- [Public Repository Release Week](docs/planning/PUBLIC_RELEASE_WEEK.md)
- [Rebaselined Development Roadmap](docs/planning/DEVELOPMENT_PLAN.md)
- [Open-Source Foundation Track](docs/planning/OPEN_SOURCE_FOUNDATION_TRACK.md)

## A. Public repository visibility gate

### Public safety and authorization

- [ ] No known secret, credential, private key, customer data, employee data, NDA-bound material, or unauthorized confidential reference exists in the intended public tree.
- [ ] The intended public history has been scanned for secrets, confidential references, deleted proprietary files, and problematic metadata.
- [ ] A clean export has been selected instead of a visibility change when the existing history cannot be exposed safely.
- [ ] Copyright ownership and authority to expose the source are confirmed.
- [ ] The intended MIT distribution posture is confirmed for the exposed source.
- [ ] Every copied or adapted pattern, validator, algorithm, API, fixture, and documentation section has acceptable provenance and attribution.
- [ ] Test data is synthetic, public, or licensed for redistribution.
- [ ] Dependency licenses are compatible with public distribution.

### Name and positioning

- [ ] The project and crate name have been reviewed for trademark, affiliation, package-name, and ecosystem confusion.
- [ ] Microsoft and Microsoft Presidio references are accurate, limited, and accompanied by clear non-affiliation language.
- [ ] Any unresolved naming risk is documented and accepted or resolved before visibility.
- [ ] The README states that the project is early-stage, incomplete, and not production certified.
- [ ] The README does not claim complete Presidio compatibility, comprehensive PII detection, compliance, safety, or measured superiority.

### Repository operations

- [ ] CI passes formatting, Clippy, tests, documentation, package verification, MSRV, DCO, and dependency audit on the visibility commit.
- [ ] A fresh anonymous clone can build, test, and generate documentation with documented commands.
- [ ] `README.md`, `LICENSE`, `SECURITY.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, the CLA, documentation index, ADRs, and changelog are current.
- [ ] Issue and pull-request templates are usable.
- [ ] Dependency update automation is configured.
- [ ] Private vulnerability reporting or an equivalent confidential reporting path is enabled and verified.
- [ ] Branch protection and required-check expectations are configured or have an assigned immediate post-visibility action.
- [ ] Maintainers responsible for public issue and security triage are named.
- [ ] Visibility-change and rollback steps are recorded.

### Visibility approval record

- [ ] The public commit SHA or clean-export identity is recorded.
- [ ] History and secret-scan results are recorded.
- [ ] Provenance and licensing results are recorded.
- [ ] Naming review result is recorded.
- [ ] CI and clean-clone results are recorded.
- [ ] Known limitations are recorded.
- [ ] Visibility operator and responsible maintainers are recorded.
- [ ] The decision explicitly states that crates.io publication and advertising are not implied.

**Gate A result:** Public source access is authorized. This gate does not certify contributor readiness, package maturity, beta quality, production readiness, or support guarantees.

## B. Contributor-ready public alpha gate

Target: **August 14, 2026**

- [ ] A first-time contributor can build, test, understand, and propose a change without internal product context.
- [ ] Runnable strict-pattern and custom-backend examples exist.
- [ ] Code ownership, review ownership, issue triage, and security escalation are documented.
- [ ] Public APIs are inventoried as stable-for-alpha, transitional, legacy-compatible, experimental, deprecated, or unsupported.
- [ ] Legacy-to-request-oriented migration guidance exists.
- [ ] Documentation links are checked automatically or through a documented release check.
- [ ] `cargo package` contents are reviewed.
- [ ] `cargo publish --dry-run` has been rehearsed without publishing.
- [ ] Branch protection and required checks are active.
- [ ] Public contribution and security-reporting paths have been exercised.

## C. Consumer-ready public beta gate

Target: **September 25, 2026**

### Product and architecture

- [ ] Candidate resolution and overlap behavior are explicit, versionable, and tested.
- [ ] Document-bound anonymization is fallible, validates the complete plan, and produces an operation report.
- [ ] Unicode, byte offsets, source binding, normalization boundaries, and resource limits are documented and tested.
- [ ] Supported entities and formats are precisely bounded.
- [ ] Unsupported semantic PII is clearly documented.
- [ ] No internal consumer silently defines the reusable core architecture.

### Evidence and compatibility

- [ ] Precision, recall, and F1 are measured on a reproducible and redistributable corpus.
- [ ] False-positive and false-negative regression fixtures exist.
- [ ] Initial fuzzing or property tests cover span, request, resolution, and anonymization paths.
- [ ] Adversarial long-input, malformed Unicode, overlap, and denial-of-service cases are tested.
- [ ] At least two materially different Rust consumers compile and integrate.
- [ ] Downstream compile fixtures pass in CI.
- [ ] Migration and upgrade expectations are documented.
- [ ] API drift or semver checks are active against the alpha baseline.

## D. Package publication gate

Target decision: **October 30, 2026**

- [ ] The crate name is available and approved.
- [ ] Package metadata, repository URL, documentation URL, license, categories, and keywords are correct.
- [ ] `cargo package` contains only intended files.
- [ ] `cargo publish --dry-run` passes from the release commit.
- [ ] docs.rs feature behavior is configured and verified.
- [ ] The release tag and crate contents correspond to the same commit.
- [ ] A clean checkout of the package artifact reproduces tests and documentation.
- [ ] Dependency license, source, advisory, and ban policies pass.
- [ ] Evaluation, fuzzing, benchmark, and security evidence support every package-level claim.
- [ ] Maintainer and vulnerability-response capacity are realistic for a published security-sensitive crate.
- [ ] Package publication is explicitly approved in a release decision record.

A public GitHub repository does not require a crates.io package.

## E. Advertised launch gate

This gate has no automatic date.

- [ ] The intended audience and launch purpose are defined.
- [ ] Maintainers accept the expected issue, support, contribution, and security-report volume.
- [ ] The README, release notes, examples, and known limitations are ready for increased scrutiny.
- [ ] Public claims are backed by reproducible evidence.
- [ ] A triage and communication plan exists.
- [ ] Launch channels, messaging, and responsible maintainers are approved.
- [ ] Package publication status is stated accurately.

A repository may remain quietly public indefinitely. Access and promotion are separate strategic decisions.

## F. Stable-release gate

No stable-release date is implied by visibility, package publication, or advertising.

Before accepting a stable compatibility commitment:

- [ ] public API and serialized-format policies are complete;
- [ ] deprecation and removal policy has been exercised successfully;
- [ ] consumer adoption and upgrade evidence are sufficient;
- [ ] sustained fuzzing, evaluation, benchmarks, and security review support the declared scope;
- [ ] maintainership, incident response, and succession plans are operational; and
- [ ] the build-versus-adopt-versus-collaborate decision still favors this project.

## Decision record requirements

Every release decision must record:

- decision type: visibility, contributor alpha, consumer beta, package, advertised launch, or stable release;
- approved name, license, scope, and commit;
- repository, export, tag, or package identity;
- completed and deferred checklist items;
- provenance, security, quality, and compatibility evidence appropriate to the gate;
- known limitations;
- responsible maintainers;
- response and incident-management expectations; and
- rollback, archive, migration, or redirection plan.

Silence, enthusiasm, elapsed time, accumulated code, a visibility setting, or a green badge does not substitute for the decision appropriate to each gate.