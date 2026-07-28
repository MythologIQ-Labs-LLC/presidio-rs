# Public Repository Release Week

> Status: Public visibility completed Tuesday, July 28, 2026. This document remains the historical release runbook.

## Decision

The repository became publicly readable on **Tuesday, July 28, 2026**, two days ahead of the original target.

This is a source-visibility release, not an advertised product launch, crates.io publication, production-readiness declaration, or promise of support beyond the documented maintainer capacity.

The release objective is to make the project safely inspectable, cloneable, buildable, and contributable while preserving honest early-stage limitations.

## Release distinctions

The project uses four separate release decisions:

1. **Public repository visibility** means the source and history are publicly readable on GitHub.
2. **Contributor-ready alpha** means an unfamiliar contributor can build, test, understand, and propose changes without private oral history.
3. **Package release** means a version is published to crates.io or another package registry.
4. **Advertised launch** means the maintainers actively promote availability and accept the resulting community and support load.

The first decision completed on July 28. The other decisions still require separate approval.

## Scope freeze

From Monday, July 27 through the visibility change, only changes required for public safety, legal clarity, documentation accuracy, repository operations, or a release-blocking correctness defect may enter `main`.

New recognizers, semantic backends, performance features, and broad API redesign are deferred until after visibility. Humans may survive four days without adding another abstraction.

## Must-pass visibility gate

The repository may become public only when all of the following are true:

- no known secret, credential, private key, customer data, employee data, NDA-bound material, or confidential internal reference exists in the current tree or intended public history;
- copyright ownership and the intended MIT licensing posture are confirmed for the source being exposed;
- copied or adapted source, patterns, algorithms, documentation, and fixtures have acceptable provenance and attribution;
- the public name and Microsoft non-affiliation language are reviewed and any unresolved naming risk is documented;
- `README.md`, `LICENSE`, `SECURITY.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, and the CLA are present and internally consistent;
- the README states that the crate is early-stage, incomplete, not production certified, and not a complete Microsoft Presidio replacement;
- CI passes formatting, Clippy, tests, documentation, package verification, MSRV, DCO, and dependency audit on the release head;
- a clean anonymous clone can build, test, and generate documentation using only documented commands;
- GitHub issue, pull-request, security-reporting, and dependency-update paths are configured;
- branch protection and required-check expectations are recorded, even if an organization setting must be applied manually;
- maintainers know who can triage public issues and private vulnerability reports;
- the visibility-change and rollback procedure is written and reviewed.

A blocker in these categories requires correction, a clean-history export, a rename, or removal of the affected material before public visibility. The deadline does not authorize exposing secrets or material the project does not have the right to publish.

## Explicit non-blockers for public visibility

The following remain important but do not block public source visibility when limitations are documented honestly:

- incomplete evaluation metrics;
- incomplete fuzzing duration;
- absence of two consumer pilots;
- fallible document-bound anonymization still being roadmap work;
- an unsettled permanent overlap-resolution policy;
- no semantic NER backend;
- no crates.io package;
- no stable serialized protocol;
- no production-readiness claim;
- no advertising campaign;
- no guaranteed response-time service level.

These items block later maturity gates, not public inspection of an explicitly early-stage repository.

## Daily execution plan

### Monday, July 27

- [x] Rebaseline the roadmap around accelerated public visibility.
- [x] Freeze non-release scope.
- [x] Create the release tracking issue and assign owners.
- [x] Inventory visibility blockers and create focused owner-assigned issues.
- [x] Record a preliminary current-history versus clean-export decision.
- [x] Begin name, provenance, license, confidential-reference, and security review.
- [x] Close superseded PR #3.
- [x] Commit the preliminary release evidence record.

**Exit:** every visibility blocker has an owner, evidence requirement, and deadline.

Monday evidence: [July 30 Public Visibility Evidence](../release/2026-07-30-PUBLIC-VISIBILITY-EVIDENCE.md).

### Tuesday, July 28

- [x] Complete automated secret and confidential-history scanning.
- [x] Complete the technical source, fixture, pattern, algorithm, documentation, and dependency provenance inventory.
- [x] Review name, package identity, Microsoft references, and non-affiliation language and route the unresolved risk to a rename decision.
- [x] Verify license, CLA, contribution, conduct, governance, and security documents.
- [x] Add or prepare CODEOWNERS, issue-template configuration, pull-request template, security-reporting guidance, and branch-protection documentation.
- [x] Run `cargo package` and inspect intentional package contents.

**Exit:** the automated audit passed, provenance and package evidence are recorded, and remaining blockers are explicit human or GitHub-administration decisions.

Tuesday evidence:

- [Tuesday automated audit evidence](../release/2026-07-27-TUESDAY-AUDIT-EVIDENCE.md)
- [Source and dependency provenance inventory](../release/SOURCE_AND_DEPENDENCY_PROVENANCE.md)

### Wednesday, July 29

- [x] Add the release-candidate rehearsal and run it on the candidate branch.
- [x] Perform a fresh credential-free isolated-clone rehearsal in a clean directory.
- [x] Build documentation from the clone and verify both README-backed examples.
- [x] Verify all relative repository documentation links.
- [x] Verify required public files, issue forms, pull-request template, CODEOWNERS, and Dependabot configuration.
- [x] Prepare the visibility-change runbook, rollback steps, known-limitations boundary, and operator assignments.
- [x] Delete the allowlisted obsolete source branches identified by the audit.
- [ ] Pin the final candidate after merge and record final run identifiers and artifact digests.
- [ ] Verify private vulnerability reporting and branch protection through repository administration.
- [ ] Record the final human go or no-go authorization.

**Exit:** one pinned commit satisfies every automatable visibility gate. Remaining work is the final name, email, publication-authority, history, repository-setting, and authorization decisions.

Wednesday evidence:

- [Wednesday release-candidate evidence](../release/2026-07-30-WEDNESDAY-CANDIDATE.md)
- [Public visibility and rollback runbook](../release/PUBLIC_VISIBILITY_RUNBOOK.md)

A truly anonymous network clone cannot be tested while the repository remains internal. It must be verified immediately after the visibility change before Thursday's release is considered complete.

### Thursday, July 30

- Hold the final maintainer review.
- Confirm the release commit and evidence bundle.
- Change repository visibility to public or publish the approved clean export.
- Verify anonymous access, clone, CI badge, documentation links, issue templates, license display, security policy, and default branch.
- Confirm no package registry publication or advertising occurred unless separately approved.
- Monitor initial public activity and security-reporting paths.

**Exit:** the source is publicly accessible, accurately described, and operationally supportable at its declared early-stage maturity.

## Go or no-go record

The visibility decision must record:

- repository or export URL;
- public commit SHA;
- history and secret-scan result;
- provenance and licensing result;
- naming review result;
- CI run and package verification result;
- known limitations;
- maintainers responsible for public issue and security triage;
- branch-protection state;
- visibility-change operator;
- rollback procedure; and
- explicit confirmation that advertising and package publication were not implied.

## Immediate post-public priorities

Public visibility does not alter the foundation-first technical order:

1. explicit candidate-resolution policy;
2. fallible anonymization over document-bound findings;
3. evaluation corpus and error analysis;
4. fuzzing and property tests;
5. contributor examples and API migration guidance;
6. downstream compile fixtures and two materially different consumer pilots;
7. compatibility, package, and release hardening; and
8. evidence-based crates.io and advertised-launch decisions.

## Success criteria

Release week succeeds when the repository is public without overstating maturity, exposing confidential material, fabricating evidence, or creating an accidental support promise.

A quiet public repository is a valid outcome. Publicity is access. Promotion is strategy. The distinction is now written down so nobody has to rediscover it during a Thursday-afternoon settings change.
