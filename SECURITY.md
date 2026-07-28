# Security Policy

`presidio-rs` processes untrusted text and may be used at privacy or security boundaries. We take reports involving data exposure, incorrect redaction, denial of service, unsafe dependency behavior, and supply-chain integrity seriously.

## Supported versions

Until the first stable release, only the latest commit on `main` is supported. Security fixes may be released without backporting to older `0.x` versions.

After stable releases begin, this file will list supported release lines explicitly.

## Reporting a vulnerability

Do not disclose suspected vulnerabilities in a public issue, pull request, discussion, or social post before maintainers have had a reasonable opportunity to investigate.

Send confidential reports to **admin@mythologiq.studio** with the subject line `presidio-rs security report`.

GitHub private vulnerability reporting is not currently available for this repository. Email is therefore the authoritative confidential reporting path until that capability becomes available or a replacement secure intake system is documented here.

Include:

- affected version or commit;
- vulnerability class;
- reproducible input or proof of concept;
- expected and actual behavior;
- likely impact;
- affected platforms or configurations;
- suggested mitigation, if known; and
- whether the report has been shared elsewhere.

Do not include real personal data, production secrets, or customer information. Use synthetic examples. When a proof of concept requires sensitive material, first send a minimal description and request a safer transfer method.

## Response process

Maintainers will aim to:

- acknowledge a complete report promptly;
- validate impact and affected versions;
- coordinate remediation and disclosure with the reporter;
- credit the reporter when requested and appropriate; and
- publish an advisory when users need to take action.

Response timing depends on severity, reproducibility, maintainer availability, and whether downstream coordination is required. This policy does not promise a specific service-level agreement.

## Security-relevant issue classes

Please report concerns involving:

- false negatives that expose supported PII formats;
- span or Unicode errors that leave sensitive text unmodified;
- panics, excessive CPU use, memory exhaustion, or algorithmic denial of service;
- incorrect overlap resolution;
- anonymization failures or reversible transformations represented as irreversible;
- unsafe handling of pseudonymization material;
- dependency compromise or malicious package behavior;
- release artifact or provenance tampering;
- parsing discrepancies between documented and actual behavior; and
- accidental network, filesystem, or telemetry activity introduced into the crate.

A recognizer failing to detect an explicitly unsupported format is generally a feature request rather than a vulnerability. Reports showing that documented supported behavior fails in a realistic security context may still qualify.

## Disclosure expectations

We ask reporters to allow coordinated disclosure. Maintainers will not request indefinite silence and will communicate when investigation is delayed.

Good-faith research that avoids privacy harm, data destruction, service disruption, and unauthorized access will be treated respectfully. This statement is not legal authorization to test systems you do not own or have permission to assess.

## Security boundaries

Using `presidio-rs` does not guarantee that all sensitive information will be found or that an application is compliant with any law or standard. Callers remain responsible for threat modeling, coverage validation, policy, failure behavior, access control, logging, storage, and downstream handling.
