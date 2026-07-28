# Wednesday Release-Candidate Evidence

## Status

- target visibility date: Thursday, July 30, 2026;
- work started: Monday, July 27, 2026, two days early;
- repository visibility: internal;
- release state: candidate preparation, not authorized for public visibility;
- release owner: Kevin Knapp.

## Wednesday objective

Produce one pinned tree that can be made publicly readable without additional code changes after the remaining human decisions and GitHub settings are completed.

## Repository-surface verification

This change establishes or verifies:

- issue forms for bugs and feature requests;
- blank-issue prevention and private-security routing;
- a public pull-request template covering API compatibility, security, privacy, provenance, evidence, documentation, DCO, and contribution intake;
- CODEOWNERS for the repository, security, governance, release, source, and tests;
- Dependabot configuration for Cargo and GitHub Actions;
- README maturity, non-affiliation, package, support, and roadmap boundaries;
- security, contribution, conduct, governance, license, CLA, changelog, architecture, and release documents;
- runnable examples corresponding to both documented API paths; and
- a repository-local relative Markdown link verifier.

## Release-candidate rehearsal

`.github/workflows/release-candidate-rehearsal.yml` performs a read-only rehearsal on pull requests and manual runs.

The workflow:

1. verifies required public files;
2. parses repository YAML;
3. verifies relative Markdown links;
4. creates an isolated clone of the exact candidate commit without persisted GitHub credentials;
5. runs formatting, Clippy, build, tests, doctests, and rustdoc;
6. runs the two README-backed examples;
7. builds and inspects the Cargo package;
8. rejects repository-only or generated package content; and
9. uploads candidate identity, toolchain, package, and checksum evidence.

Because the repository is still internal, this is a credential-free local clone rehearsal rather than a truly anonymous network clone. The anonymous network clone must be verified immediately after the visibility change.

## Canonical formatting correction

The first rehearsal reached the isolated clone and passed required-file, YAML, and Markdown-link checks, but rejected the new examples at the formatting gate.

A one-run formatter was added only to `release/wednesday-candidate`. It ran `cargo fmt --all`, committed the canonical result with DCO sign-off, and deleted its own workflow file in the same commit. The branch returned immediately to read-only CI, visibility-audit, and rehearsal workflows.

The final candidate checks must run from a normal maintainer-authored commit after that bootstrap so GitHub does not treat a workflow-authored commit as requiring separate action approval.

## Candidate pin procedure

After this change merges and all required checks pass:

1. record the resulting `main` SHA;
2. create `release/public-candidate-2026-07-30` at that exact SHA;
3. do not move the candidate branch;
4. record the CI, visibility-audit, and rehearsal run identifiers and artifact digests; and
5. replace the pin only if a later change is approved and all evidence is rerun.

## Stale branch disposition

The Tuesday audit identified obsolete backup, bootstrap, documentation, feature, reconstruction, and completed release branches. These should be deleted before visibility:

- `backup/candidate-report-adversarial-remediation`;
- `backup/core-value-types-adversarial-remediation`;
- `chore/open-source-readiness`;
- `docs/public-by-july-30-rebaseline`;
- `docs/stealth-open-source-foundation`;
- `feat/analysis-report`;
- `feat/analysis-request-recognizer-trait`;
- `feat/candidate-analysis-report`;
- `feat/core-value-types`;
- `feat/recognizer-metadata-contract`;
- `feat/text-document-binding`;
- `release/public-visibility-monday`;
- `release/tuesday-evidence`; and
- `release/tuesday-visibility-audit`.

Open Dependabot branches remain active until their pull requests are reviewed or closed. GitHub-managed pull-request refs may remain reachable after source branches are deleted and must be considered in the history-versus-clean-export decision.

## Accountable operators

Pending additional maintainers, Kevin Knapp is recorded as:

- release owner;
- visibility-change operator;
- public issue triage owner;
- private security triage owner; and
- rollback operator.

This removes ambiguity for Thursday but remains a public-alpha capacity risk.

## Remaining human and administrative blockers

Public visibility remains blocked until all are recorded:

1. final public project, repository, package, and crate naming decision;
2. personal commit-email privacy decision;
3. MythologIQ Labs LLC publication-authority and MIT-distribution attestation;
4. current-history versus clean-export decision;
5. stale source-branch deletion or explicit retention;
6. private vulnerability reporting enabled and tested;
7. `main` branch protection and required checks configured;
8. final candidate SHA and evidence identifiers; and
9. final go or no-go authorization.

## Advertising and package boundary

This candidate does not authorize:

- crates.io publication;
- a GitHub package release;
- an advertised launch;
- production certification;
- stable API guarantees; or
- guaranteed maintainer response times.

A quietly public source repository remains the default Thursday outcome unless promotion receives separate approval.
