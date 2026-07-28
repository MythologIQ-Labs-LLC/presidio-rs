# Public Visibility and Rollback Runbook

## Purpose

This runbook governs the controlled change from an internal repository to publicly readable source. It does not authorize crates.io publication, advertising, production claims, or stable API guarantees.

## Accountable operators

Until additional maintainers are appointed:

- release owner: Kevin Knapp;
- visibility-change operator: Kevin Knapp;
- public issue triage: Kevin Knapp;
- private security triage: Kevin Knapp; and
- rollback operator: Kevin Knapp.

Concentrating these roles in one person is a capacity risk, not an ideal governance model. The public alpha gate must add backup ownership.

## Preconditions

Do not change visibility until all conditions are recorded in the release evidence:

1. the final public project, repository, package, and crate names are approved;
2. the intended public commit is pinned by an immutable recorded SHA and candidate branch;
3. the full history and current tree pass the public-visibility audit;
4. the fresh-clone release-candidate rehearsal passes on the pinned tree;
5. MythologIQ Labs LLC records publication authority and MIT-distribution intent;
6. the maintainer accepts public exposure of commit metadata or approves a clean export or rewrite;
7. the current-history versus clean-export decision is final;
8. nonessential branches are deleted or explicitly retained;
9. GitHub private vulnerability reporting is enabled and tested;
10. `main` branch protection and required checks are configured and recorded;
11. issue templates, pull-request template, CODEOWNERS, Dependabot, DCO, security policy, governance, license, and contribution rules are present; and
12. the final go or no-go record explicitly separates visibility from package publication and promotion.

## Candidate pinning

After all Wednesday changes merge:

1. record the latest `main` SHA;
2. create `release/public-candidate-2026-07-30` at that exact SHA;
3. do not move the candidate branch after evidence is recorded;
4. run the visibility audit and release-candidate rehearsal against that tree; and
5. require any later code or documentation change to create a new candidate SHA and rerun both gates.

The candidate branch is an operational pin, not a release channel or compatibility promise.

## Visibility procedure

The visibility-change operator should:

1. confirm the approved repository name and URL;
2. confirm the pinned candidate SHA is the current default-branch tree or the approved clean-export tree;
3. capture the final repository visibility and settings state;
4. change repository visibility to public using GitHub organization controls;
5. avoid creating a GitHub Release, tag intended as a package release, or crates.io publication unless separately approved;
6. avoid posting an announcement unless the promotion decision is separately approved; and
7. record the exact time, operator, repository URL, and public SHA in the go or no-go evidence.

## Immediate public verification

From an unauthenticated environment:

1. open the repository URL;
2. verify `README.md`, MIT license display, default branch, CI badge, security policy, contribution guide, issue forms, and pull-request template;
3. clone the repository without credentials;
4. check out the recorded public SHA;
5. run the documented build, test, doctest, rustdoc, example, link, and package commands;
6. verify private vulnerability reporting is offered to an unauthenticated reporter where GitHub supports it;
7. verify protected-branch and required-check behavior through a harmless test branch if needed; and
8. confirm no package or promotion action occurred accidentally.

## Initial monitoring

For the first public operating window:

- review new issues and pull requests for confidential data before engaging;
- route suspected vulnerabilities to private reporting;
- enforce the interim rule that outside contributions are not merged;
- watch CI, Dependabot, audit, and repository settings for unexpected failures;
- preserve screenshots, logs, and commit SHAs for any incident; and
- correct misleading maturity claims promptly.

## Rollback triggers

Consider immediate rollback for:

- exposed credentials, private data, confidential material, or unauthorized source;
- a material publication-rights or trademark problem;
- incorrect repository identity or history;
- broken security-reporting controls;
- compromised repository settings or branch protection;
- evidence that the public tree differs from the approved candidate; or
- a severe release-blocking correctness or supply-chain issue that makes continued visibility unsafe.

Normal bugs, incomplete roadmap items, or lack of advertising are not automatic visibility rollback triggers.

## Rollback procedure

The rollback operator should:

1. change repository visibility back to internal or private when organization policy permits;
2. disable contribution merges and pause automated release actions;
3. revoke and rotate any exposed credentials immediately;
4. open a private incident record with timeline, affected refs, actors, and evidence;
5. preserve the exposed public SHA, logs, and audit artifacts;
6. determine whether a history rewrite, clean export, rename, deletion request, advisory, or notification is required;
7. assume public clones, caches, forks, and search indexes may persist after visibility is reduced; and
8. publish a corrected repository only after a new evidence review and explicit authorization.

Reducing visibility is containment, not erasure. The rollback record must not claim that previously public material has ceased to exist everywhere.

## Separate decisions

The following require their own approval after source visibility:

- contributor-ready public alpha;
- acceptance of outside contributions;
- crates.io publication;
- GitHub Release creation;
- advertised launch;
- production-readiness claims; and
- stable API or support commitments.
