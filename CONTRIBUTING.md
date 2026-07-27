# Contributing to presidio-rs

Thank you for contributing to `presidio-rs`. The project is preparing for public source visibility while its contributor-ready alpha process is still being completed.

This project handles security-sensitive text processing. Changes should be small, explainable, tested, and honest about their limits.

## Interim public contribution intake

After the repository becomes public, anyone may open issues and pull requests for discussion and review.

Until the Contributor License Agreement and its acceptance mechanism receive the required legal and maintainer approval:

- outside contributions will not be merged;
- maintainers may review, discuss, or request revisions without implying acceptance;
- every commit must still pass DCO sign-off; and
- contributors must not submit confidential, proprietary, customer, employee, or private third-party material.

This restriction applies to contribution acceptance, not public participation. The repository will record when the restriction is lifted.

## Contribution terms

By submitting a contribution, you agree to:

1. the [Contributor License Agreement](CONTRIBUTOR_LICENSE_AGREEMENT.md);
2. license your contribution under the project's [MIT License](LICENSE);
3. certify every commit under the [Developer Certificate of Origin](https://developercertificate.org/) using a `Signed-off-by` trailer;
4. disclose relevant third-party code, algorithms, datasets, model artifacts, and prior art; and
5. take responsibility for the contribution, including any AI-assisted content.

To sign off a commit:

```bash
git commit -s -m "feat: add validated recognizer"
```

This appends:

```text
Signed-off-by: Your Name <you@example.com>
```

Use an identity you are authorized to use. Do not add another person's sign-off. Commit author and sign-off identities become public repository metadata when the repository is public.

## Development setup

Prerequisites:

- Git
- Rust 1.74 or newer
- Cargo

```bash
git clone https://github.com/MythologIQ-Labs-LLC/presidio-rs.git
cd presidio-rs
cargo build --all-features
cargo test --all-features
```

Before submitting a pull request, run:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps --all-features
cargo package --allow-dirty
```

## What belongs in this repository

Good contributions include:

- recognizers and validators supported by evidence-backed fixtures;
- false-positive and false-negative regression cases;
- international structured-identifier support;
- anonymization correctness and safety improvements;
- Unicode, offset, overlap, and malformed-input handling;
- fuzzing, property testing, evaluation, and benchmarks;
- public API and documentation improvements; and
- dependency, security, and supply-chain improvements.

A proposed change should solve a clear user problem and fit the project's model-free, embeddable Rust library scope. Hosted services, product-specific policy, unrelated compliance frameworks, and application-specific orchestration generally belong outside the core crate.

## Recognizer contributions

A recognizer is not accepted merely because its regex compiles.

Recognizer pull requests should include:

- a precise description of the entity and supported formats;
- authoritative format or checksum references;
- positive fixtures;
- near-miss and invalid fixtures;
- false-positive regression cases;
- locale or jurisdiction boundaries;
- confidence-score rationale;
- context words, if used, with boundary-sensitive tests; and
- attribution and license information for adapted patterns or datasets.

Do not use real personal data in fixtures. Use synthetic values that are clearly designated for testing.

## Security-sensitive changes

Changes involving cryptography, anonymization, span coordinates, normalization, overlap resolution, untrusted input, dependencies, or release artifacts receive heightened review.

Such pull requests should explain:

- the threat or failure mode addressed;
- security assumptions;
- failure behavior;
- compatibility impact;
- independent verification performed; and
- remaining limitations.

Do not report suspected vulnerabilities in a normal issue. Follow [SECURITY.md](SECURITY.md).

## Attribution and prior art

All contributions must identify relevant prior work.

Attribution is required when a contribution copies, closely adapts, or materially follows another project's:

- code;
- regex or validation logic;
- API design;
- scoring method;
- benchmark or dataset;
- documentation structure; or
- architectural pattern.

Include sources and licenses in the pull request and, where appropriate, in code comments or documentation. Rewriting code does not erase provenance.

## AI-assisted contributions

AI tools may assist development, but a responsible human must review and own every submission.

The contributor must be able to explain:

- what changed;
- why the design is appropriate;
- how it was verified;
- what limitations remain; and
- what third-party material influenced the result.

Do not submit autonomous, unreviewed agent output. Do not use AI tools to conceal copied or license-incompatible material. For security-sensitive changes, tests must include independent reasoning rather than merely checking an AI-generated implementation against AI-generated expectations.

## Pull request expectations

Keep pull requests focused. Each pull request should include:

- the problem being solved;
- the chosen approach and meaningful alternatives;
- tests and verification performed;
- compatibility or migration impact;
- security and privacy implications;
- prior art and third-party sources; and
- CLA acceptance confirmation.

Public API changes require documentation and a compatibility assessment. Breaking changes require explicit maintainer approval and should not be mixed with unrelated refactoring.

## Claims discipline

Documentation, benchmarks, and release notes must distinguish among:

- **implemented:** visible in source and tests;
- **measured:** supported by reproducible evidence; and
- **planned:** proposed but not implemented.

Do not claim superior speed, accuracy, safety, compliance, memory use, or cost without a reproducible comparison that defines configurations, data, environment, and methodology.

## Review and acceptance

Maintainers may request changes, decline contributions that do not fit the project direction, or require additional evidence for security-sensitive behavior. Passing CI is necessary but not sufficient for acceptance.

By participating, contributors agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md).
