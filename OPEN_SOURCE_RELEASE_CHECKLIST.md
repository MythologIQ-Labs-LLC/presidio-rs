# Open-Source Release Checklist

This repository is developed privately with open-source-grade practices. Public release is a separate decision and must not happen merely because the files look ready.

Complete this checklist before changing repository visibility, publishing a crate, or exporting the project into a public repository.

## Strategic decision

- [ ] The project has a clear independent reason to exist.
- [ ] Expected users and supported use cases are documented.
- [ ] The public name has been reviewed for trademark, affiliation, package-name, and ecosystem confusion.
- [ ] The maintainers accept the commercial rights granted by the MIT License, including private forks and competitor use.
- [ ] The public/private product boundary is documented.
- [ ] Maintenance capacity and response expectations are realistic.

## Legal and provenance

- [ ] Counsel has reviewed the MIT licensing decision and Contributor License Agreement.
- [ ] Copyright ownership of the existing source is confirmed.
- [ ] Every dependency license is compatible with MIT distribution.
- [ ] Every copied or adapted pattern, validator, algorithm, API, and documentation section has provenance and attribution.
- [ ] Test datasets and fixtures are synthetic or licensed for redistribution.
- [ ] No confidential, NDA-bound, customer, employee, or third-party private material is present.
- [ ] Git history has been reviewed for secrets, confidential references, deleted proprietary files, and problematic metadata.
- [ ] A clean public-history export has been considered instead of changing this repository's visibility.

## Product and architecture

- [ ] The public API has been reviewed for coherent naming and extension boundaries.
- [ ] Anonymization failure behavior is explicit and tested.
- [ ] Unicode, byte offsets, normalization, and overlap behavior are documented and tested.
- [ ] Supported entities and formats are precisely bounded.
- [ ] Unsupported semantic PII is clearly documented.
- [ ] Security-sensitive defaults have received independent review.
- [ ] No internal consumer silently defines the reusable core architecture.

## Evidence

- [ ] Precision, recall, and F1 are measured on a redistributable or reproducible corpus.
- [ ] False-positive and false-negative regression fixtures exist.
- [ ] Adversarial long-input, malformed Unicode, and denial-of-service cases are tested.
- [ ] Fuzzing or property tests cover critical span and anonymization paths.
- [ ] Benchmarks clearly define hardware, software, data, warmup, and comparison configurations.
- [ ] Documentation separates implemented, measured, and planned capabilities.
- [ ] No superiority, compliance, production-readiness, or safety claim exceeds the evidence.

## Repository readiness

- [ ] CI passes on the intended public release commit.
- [ ] DCO enforcement is active.
- [ ] The CLA acceptance mechanism is operational.
- [ ] `README.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, and `GOVERNANCE.md` are current.
- [ ] Issue and pull request templates are usable.
- [ ] Dependency update automation is configured.
- [ ] Security advisories or private vulnerability reporting are enabled.
- [ ] Branch protection and required checks are configured.
- [ ] Maintainer and review ownership are explicit.
- [ ] The changelog and release notes describe limitations and compatibility.

## Package release

- [ ] The crate name is available and appropriate.
- [ ] `cargo package` contains only intended files.
- [ ] `cargo publish --dry-run` passes.
- [ ] Package metadata, repository URL, documentation URL, license, categories, and keywords are correct.
- [ ] The minimum supported Rust version is tested.
- [ ] docs.rs feature behavior is configured and verified.
- [ ] The release tag and crate contents correspond to the same commit.

## Final approval

Record the decision in an architecture or release decision document containing:

- approved public name;
- approved license;
- release commit;
- provenance-audit result;
- security-review result;
- known limitations;
- maintainers responsible for the release; and
- rollback or incident-response plan.

Public release requires an explicit maintainer approval recorded in the repository. Silence, enthusiasm, or a green badge does not count.