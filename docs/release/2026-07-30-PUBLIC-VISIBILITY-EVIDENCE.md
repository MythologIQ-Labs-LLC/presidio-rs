# July 30 Public Visibility Evidence

## Status

- **Target:** publicly readable GitHub repository on Thursday, July 30, 2026
- **Evidence state:** Monday preliminary review
- **Visibility authorization:** not yet granted
- **Release owner:** Kevin Knapp
- **Current repository visibility:** internal

This document records evidence and decisions for public source visibility. It does not authorize crates.io publication, advertising, production claims, or stable API guarantees.

## Monday decision summary

### Repository history versus clean export

**Preliminary recommendation:** expose the current repository and history rather than create a clean export.

This recommendation is conditional. The current repository may be exposed only if Tuesday's complete history, object, secret, confidentiality, metadata, and provenance review passes and stale refs are removed or neutralized.

A clean export becomes mandatory if the review finds:

- secrets, credentials, private data, or confidential references in history;
- deleted proprietary or otherwise unpublishable material;
- problematic author or committer metadata that cannot be accepted;
- stale refs that expose misleading or unsafe development history;
- a naming problem that requires a different public repository identity; or
- uncertain publication or redistribution rights.

Rationale for preferring the current repository when safe:

- the merged history is scoped to this library;
- architecture decisions and adversarial review history have public value;
- existing issue and release trackers are already organized for public work;
- the current tree contains explicit early-stage, non-affiliation, security, contribution, and claims boundaries; and
- discarding the review record would reduce transparency without improving safety if the history is otherwise clean.

Tracking decision: issue #24.

## Monday blocker inventory

| Area | Severity | Owner | Due | Status | Tracking |
|---|---|---|---|---|---|
| Public project name and Presidio positioning | Critical | Kevin Knapp | Jul 28 | Open | #18 |
| Complete history, secret, confidentiality, and provenance scan | Critical | Kevin Knapp | Jul 28 | Open | #19 |
| Publication authority, license posture, and contribution intake | Critical | Kevin Knapp | Jul 28 | Open | #20 |
| Security reporting, branch protection, and stale refs | High | Kevin Knapp | Jul 29 | Open | #21 |
| Line-item source and dependency provenance inventory | Critical | Kevin Knapp | Jul 28 | Open | #22 |
| Confidential-reference review for public documentation and discussions | Critical | Kevin Knapp | Jul 28 | Open | #23 |
| Superseded PR #3 | High | Kevin Knapp | Jul 27 | Resolved | PR #3 closed and marked superseded |

No open blocker has been waived because of the Thursday date.

## Preliminary current-tree confidentiality and secret review

Repository code search was performed for representative secret and internal-reference indicators. This is an initial current-tree review, not the required complete history scan.

### No current-tree matches found

- `AKIA`
- `Bicameral`
- `Qortara`
- `krknapp@gmail.com`
- `password`

### Reviewed matches

- `ghp_`, `sk-`, and Slack-token-shaped text appear in the built-in API-key recognizer regular expressions, not as live credentials.
- `Accountable` matched ordinary governance language rather than the Accountable company or product.
- `customer` appears in security, contribution, release, and issue-template warnings that prohibit customer data.
- private-key language appears in release and security checklists rather than embedded key material.

### Limitations

GitHub code search does not establish that:

- deleted files are clean;
- stale branches and pull-request refs are clean;
- binary or large objects are clean;
- commit messages and author metadata are acceptable; or
- every token-shaped value has been detected.

Issue #19 requires a complete clone-based history and object scan with recorded commands and results.

## Pull request and ref review

PR #3 contained an obsolete report contract and 57 commits of reconstruction history. It was closed and marked superseded by merged PR #4.

Before visibility, all stale feature, backup, diagnostic, and reconstruction branches must be enumerated and removed or neutralized. Closing a pull request alone does not prove that its branch and commits are absent from the public repository surface. Issue #21 owns that work.

## Preliminary naming and ecosystem review

### Current names

- repository: `presidio-rs`
- package: `presidio-rs`
- Rust crate import: `presidio`

### Initial finding

Naming risk is **material and unresolved**.

As of July 27, 2026, active Rust crates include:

- `presidio-analyzer`
- `presidio-anonymizer`
- `presidio-server`

Those crates explicitly present themselves as Rust ports or Presidio-style implementations. The current `presidio-rs` name can therefore create ecosystem confusion even with the existing non-affiliation notice.

Microsoft's trademark guidance requires use of Microsoft brand assets and product names to avoid source or sponsorship confusion. Truthful compatibility or reference language belongs in descriptive text rather than a name that may appear official.

Current mitigating language:

- the README states that the project is independently governed and not affiliated with, sponsored by, or endorsed by Microsoft;
- the README identifies Microsoft Presidio as design prior art;
- the README states that Microsoft Presidio source is not linked, vendored, or redistributed; and
- project documentation rejects claims of drop-in compatibility or complete Presidio implementation.

These controls reduce but do not eliminate naming risk. Issue #18 requires a signed decision to rename, retain with accepted risk and stronger positioning, or publish a clean export under a neutral name.

Reference sources:

- https://github.com/microsoft/presidio
- https://www.microsoft.com/trademarks
- https://docs.rs/crate/presidio-analyzer/latest
- https://docs.rs/crate/presidio-anonymizer/latest
- https://docs.rs/crate/presidio-server/latest

## Preliminary license and publication-authority review

### Confirmed in repository

- `LICENSE` contains the standard MIT license with MythologIQ Labs LLC copyright.
- `Cargo.toml` declares `MIT` and identifies MythologIQ Labs LLC as author.
- Direct dependencies are `regex`, `sha2`, and optional `serde`.
- DCO sign-off is enforced by CI for post-bootstrap pull requests.
- The Contributor License Agreement preserves contributor ownership while granting copyright and patent licenses.

### Unresolved before visibility

- MythologIQ Labs LLC must affirm copyright ownership or publication authorization for every existing contribution.
- Direct and transitive dependency licenses require machine-readable verification.
- Source, regular-expression, validator, fixture, algorithm, and documentation provenance must be inventoried.
- The CLA states that legal review is required before relying on it for public third-party contributions or dual licensing.

### Recommended interim contribution rule

After visibility, public issues and pull requests may be opened, but no outside contribution should be merged until the CLA and its acceptance mechanism receive the required legal and maintainer approval. DCO remains mandatory.

Issue #20 owns the authority and contribution decision. Issue #22 owns the detailed provenance inventory.

## Preliminary security-reporting review

`SECURITY.md`:

- directs reporters away from public disclosure;
- prefers GitHub private vulnerability reporting or a private security advisory;
- describes security-relevant failure classes;
- avoids promising a fixed response SLA; and
- warns against submitting real personal data, production secrets, or customer information.

Before visibility, GitHub private vulnerability reporting or an equivalent confidential path must be enabled and exercised. Public issue triage and security triage owners must be named. Issue #21 owns this work.

## Monday checklist result

- [x] Rebaselined roadmap and release-week runbook merged.
- [x] Visibility blockers inventoried and assigned.
- [x] Preliminary current-history versus clean-export decision recorded.
- [x] Name, licensing, provenance, confidentiality, and security reviews begun.
- [x] Superseded PR #3 closed.
- [ ] Full history and object scan completed.
- [ ] Final name decision completed.
- [ ] Publication authority and license review completed.
- [ ] Provenance inventory completed.
- [ ] Stale refs removed or neutralized.

## Current go or no-go state

**NO-GO pending Tuesday evidence.**

This is the expected Monday state. Public visibility is not blocked by incomplete product maturity, but it remains blocked by unresolved safety, authorization, provenance, naming, and repository-operation questions.

## Tuesday entry criteria

Tuesday begins with:

1. full clone-based history and object scanning;
2. line-item provenance and dependency-license review;
3. final name and positioning decision;
4. publication-authority confirmation;
5. contribution-intake rule confirmation;
6. stale-ref inventory; and
7. confidential-reference review of issues, pull requests, ADRs, and planning documents.
