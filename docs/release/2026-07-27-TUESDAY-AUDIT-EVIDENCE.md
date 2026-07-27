# Tuesday Public-Visibility Audit Evidence

## Scope

This record advances the July 30, 2026 public-repository visibility review one day early. It covers automated repository-history scanning, current-tree scanning, Git object and ref inventory, package inspection, dependency-license metadata, author metadata, preliminary naming review, and the remaining human approvals.

It does not authorize public visibility, crates.io publication, advertising, production claims, or stable API guarantees.

## Automated audit implementation

Pull request #26 introduced a read-only GitHub Actions workflow at `.github/workflows/public-visibility-audit.yml`.

The workflow:

- fetches complete branch and tag history plus available pull-request refs;
- inventories refs, commits, objects, author metadata, and unreachable objects;
- downloads the current Gitleaks release from its official GitHub release page and verifies the published checksum;
- scans fetched history and the current working tree;
- records selected unrelated internal-reference paths for manual review;
- inspects `cargo package` contents;
- records Cargo dependency license expressions; and
- uploads the complete evidence as a retained workflow artifact.

The workflow has read-only repository permissions.

## First audit run and corrections

The first complete run scanned 35 fetched refs, 126 commits, and 523 Git objects.

It found no current-tree secret, two historical findings, no dependency with missing license metadata, and eight selected internal-reference paths.

Both historical findings referred to the same deliberately synthetic GitHub-token example in old README commits. The value used a `ghp_` prefix followed by 36 sequential digits solely to exercise the API-key recognizer. It is not a credential.

The exact historical fingerprints are allowlisted in `.gitleaksignore`:

- `cbdafb5e5569e347574975a872499158d1110522:README.md:github-pat:128`
- `090c04f1a8ce2448c48979b9772163d8a7df0c17:README.md:github-pat:128`

The allowlist is fingerprint-scoped. It does not suppress future GitHub-token findings in the README or elsewhere.

The first run also wrote generated audit artifacts inside the repository workspace before calling `cargo package`. Cargo therefore listed the scanner's own output as package content. The workflow was corrected to write evidence under `/tmp/public-visibility-audit`.

A draft of this evidence document briefly repeated the token-shaped test value literally. That branch history was discarded and rebuilt as one signed commit from `main`, rather than adding another scanner exception for release prose.

## Authoritative corrected run

Workflow run `30311002537` completed successfully against the rebuilt evidence branch.

The retained artifact has digest:

```text
sha256:4e5191a08bff781c086c926269569f4e3f584015d808119fa1e8152ef303902a
```

Results:

- fetched refs: **37**;
- commits scanned: **131**;
- Git objects inventoried: **548**;
- Gitleaks historical findings: **0**;
- Gitleaks current-tree findings: **0**;
- selected internal-reference paths requiring manual review: **20**;
- Cargo package files: **53**; and
- dependencies with missing Cargo license metadata: **0**.

The 20 internal-reference path matches are release-evidence and audit-workflow revisions that contain the selected search vocabulary. They do not identify unrelated internal product source or documentation.

## Package contents

The corrected package contains 53 intentional files:

- package metadata, lockfile, license, README, changelog, governance, contribution, conduct, CLA, and security documents;
- documentation index, ADRs, architecture, planning, release-evidence, risk, and research records;
- Rust source modules; and
- integration and architecture tests.

`.github/` remains excluded from the crate package. Audit artifacts are not packaged.

The package list includes `.gitleaksignore` so the exact historical synthetic-token classification remains available to downstream source reviewers. Package publication remains a later decision.

## Dependency-license metadata

The audit found no dependency with missing Cargo license metadata.

Observed license expressions include:

- MIT;
- Apache-2.0;
- `MIT OR Apache-2.0`;
- `Unlicense OR MIT`; and
- `(MIT OR Apache-2.0) AND Unicode-3.0`.

This establishes that package metadata declares licenses. It is not legal advice and does not replace review of license texts, source restrictions, dependency provenance, or a future `cargo-deny` policy.

## Author and committer metadata

The fetched history contains these author email identities:

- `49699333+dependabot[bot]@users.noreply.github.com`;
- `krknapp@gmail.com`.

Public visibility would expose the maintainer's personal email through Git commit and DCO metadata. This is not a software defect, but it requires an explicit maintainer privacy decision.

Accepted options are:

1. accept the historical email as public project metadata;
2. rewrite or clean-export history using an approved public identity; or
3. retain a clean export that omits the existing commit history.

No option is selected by this document.

## Confidential-reference review

The automated selected-term scan did not identify unrelated internal project source or documentation.

The matched paths are release-evidence and audit-workflow files that themselves list the search terms. They require only confirmation that the references describe prohibited content rather than expose it.

Issue and pull-request discussions still require manual review because public visibility includes more than the default branch tree.

## Ref and branch findings

The audit confirmed that numerous stale remote branches remain reachable, including backup, superseded feature, documentation, reconstruction, and Dependabot branches.

Examples include:

- `backup/candidate-report-adversarial-remediation`;
- `backup/core-value-types-adversarial-remediation`;
- `chore/open-source-readiness`;
- `docs/public-by-july-30-rebaseline`;
- `docs/stealth-open-source-foundation`;
- `feat/analysis-report`;
- `feat/analysis-request-recognizer-trait`;
- `feat/candidate-analysis-report`;
- `feat/core-value-types`; and
- `feat/recognizer-metadata-contract`.

A complete ref list is retained in the workflow artifact. All nonessential branches must be deleted or explicitly retained before visibility. Pull-request refs controlled by GitHub may remain reachable even after their branches are deleted; the final history-versus-clean-export decision must account for that behavior.

## Naming review

The preliminary recommendation is to rename the project before public visibility.

Reasons:

- `presidio-rs` places a Microsoft-associated product name directly in the repository and package identity;
- several active Rust crates already use Presidio-derived names;
- a disclaimer reduces affiliation confusion but does not create a distinct project identity; and
- renaming before public visibility is materially cheaper than renaming after links, forks, issues, and package references accumulate.

A final neutral name has not been approved. Issue #18 remains a hard blocker.

## Publication and contribution posture

The repository currently declares MIT licensing under MythologIQ Labs LLC and requires DCO sign-off.

Before visibility, MythologIQ Labs LLC must affirm that it owns or is authorized to publish all existing source and documentation under the intended terms.

Until the Contributor License Agreement and acceptance mechanism receive the required legal and maintainer approval:

- public issues may be opened;
- public pull requests may be opened and reviewed;
- DCO remains mandatory; and
- outside contributions must not be merged.

This interim rule is added to `CONTRIBUTING.md` in the same evidence change.

## Remaining human-only blockers

Automation cannot decide:

1. the final neutral public name;
2. whether the personal email in Git history is accepted or removed;
3. MythologIQ Labs LLC's publication-authority and MIT-distribution attestation;
4. whether the current repository history or a clean export will be published;
5. whether GitHub private vulnerability reporting is enabled and usable;
6. branch-protection and required-check configuration;
7. the final stale-ref deletion set; and
8. the release, issue-triage, security-triage, visibility-change, and rollback operators.

## Current release state

**Automated Tuesday visibility audit: PASS.**

**Overall public-visibility decision: NO-GO pending the human-only decisions above.**

The Tuesday work began early and converted repository safety, package, and dependency questions into reproducible evidence. The remaining blockers are explicit decisions and GitHub administration, not technical work that benefits from waiting until Tuesday.
