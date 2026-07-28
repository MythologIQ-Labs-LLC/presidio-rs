# First Contribution Guide

This guide is for contributors who have no private project context. A successful first contribution should be small, synthetic, attributable, and independently verifiable.

## Current contribution boundary

Public issues and pull requests may be opened for discussion and review. Outside contributions are not merged until the Contributor License Agreement and its acceptance process receive the required approval. DCO sign-off is still required on every commit.

Do not submit customer data, employee data, real credentials, production logs, confidential formats, or examples copied from private systems.

## Development setup

Requirements:

- Git;
- Rust 1.74 or newer;
- Cargo; and
- a Unix-like shell for repository scripts.

Run the same core checks used by CI:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --doc --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo package --allow-dirty --no-verify --list
```

The release rehearsal also executes the runnable examples:

```bash
cargo run --example legacy_anonymization
cargo run --example document_analysis
cargo run --example strict_pattern_recognizer
cargo run --example custom_backend
```

## Good first changes

Suitable first contributions include:

- clarifying documentation without changing claims;
- reducing a synthetic failing input into a regression fixture;
- improving an error message while preserving its stable machine-readable code;
- adding tests for malformed Unicode, span validation, resource limits, or metadata validation;
- improving a repository script or example without expanding the core API; and
- documenting prior art, public specifications, or compatibility implications.

Avoid starting with:

- new default-enabled recognizers;
- broad API redesign;
- semantic-model dependencies;
- application-specific policy;
- compatibility-breaking serialization changes; or
- accuracy, performance, security, or compliance claims without reproducible evidence.

## Contribution workflow

1. Search existing issues and pull requests.
2. Open or select a narrowly scoped issue.
3. State the consumer problem and why it belongs in the reusable library.
4. Use only synthetic or redistributable fixtures.
5. Add the smallest test that fails before the change and passes afterward.
6. Update the README, guide, ADR, migration notes, or changelog when behavior or expectations change.
7. Run the local checks above.
8. Sign every commit:

```bash
git commit -s -m "fix: describe the change"
```

9. Complete the pull-request template honestly. Mark inapplicable sections as `N/A` with a reason rather than deleting them.

## Evidence expectations

A pull request should identify:

- the exact behavior changed;
- the synthetic input or fixture used;
- the expected and actual output;
- affected public APIs;
- security, privacy, Unicode, resource, and compatibility implications;
- prior art or public specifications consulted;
- documentation updated; and
- commands used for validation.

Passing CI is necessary but not sufficient. Reviewers may reject changes that are difficult to explain, application-specific, weakly evidenced, unsafe at privacy boundaries, or expensive to maintain.

## Where changes belong

Use the core crate for deterministic, reusable contracts such as validated spans, source binding, candidate emission, metadata, request limits, and typed failures.

Use an external adapter or consumer project for model runtimes, network access, organization policy, storage, hosted services, user interfaces, or domain-specific orchestration.

When uncertain, open an issue before building the abstraction. The repository prefers a delayed interface over a permanent wrong one, a rare moment of institutional maturity.

## Security reports

Do not open a public issue for a suspected vulnerability. Follow [`SECURITY.md`](../../SECURITY.md) and use GitHub private vulnerability reporting when available.
