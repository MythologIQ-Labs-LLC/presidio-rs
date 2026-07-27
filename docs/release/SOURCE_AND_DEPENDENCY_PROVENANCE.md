# Source and Dependency Provenance Inventory

## Purpose

This inventory records the known origin and redistribution posture of the project's source, algorithms, regular expressions, tests, documentation, and dependencies before public repository visibility.

It is a technical provenance record, not legal advice. MythologIQ Labs LLC remains responsible for confirming publication authority and obtaining legal review where required.

## Classification vocabulary

- **Repository-authored:** implemented specifically for this repository without identified copied source.
- **Public specification:** implemented from a publicly documented format or algorithm.
- **Design prior art:** influenced architecture or vocabulary but no source is linked or vendored.
- **Adapted:** materially follows identifiable third-party source and requires source and license attribution.
- **Copied:** reproduced source or text requiring explicit license compliance.
- **Pending attestation:** no copied source was identified in review, but the author or rights holder must still confirm origin and publication authority.

## Project source and architecture

| Material | Current classification | Known source or influence | Redistribution posture | Remaining action |
|---|---|---|---|---|
| Rust core implementation | Repository-authored, pending attestation | Project design and Rust ecosystem conventions | Intended MIT | MythologIQ Labs LLC publication-authority attestation |
| Public API and type architecture | Repository-authored with design prior art | Microsoft Presidio concepts, Rust error and trait conventions | Intended MIT | Retain acknowledgements and non-affiliation language |
| ADRs, roadmap, governance, and release documentation | Repository-authored, pending attestation | Common open-source governance and architecture practices | Intended MIT | Confirm no confidential source text was incorporated |
| CI and repository automation | Repository-authored from standard GitHub Actions composition | Official actions and tool documentation | Intended MIT | Keep actions pinned or reviewed through dependency automation |

No Microsoft Presidio source code is linked, vendored, or redistributed by the crate based on the current tree review.

## Microsoft Presidio prior art

Microsoft Presidio is treated as design prior art for analyzer, recognizer, and anonymizer concepts.

Current controls:

- the README states that the project is independently governed and not affiliated with, sponsored by, or endorsed by Microsoft;
- recognizer metadata records `prior-art.microsoft-presidio`;
- the README rejects drop-in compatibility and complete reimplementation claims; and
- no Microsoft Presidio runtime or source dependency exists in `Cargo.toml`.

The project name remains under separate review because descriptive acknowledgement and product naming create different affiliation risks.

## Built-in recognizer patterns

Built-in expressions are defined in `src/registry.rs`.

| Recognizer | Format basis | Current classification | Notes and remaining action |
|---|---|---|---|
| Credit card | Common payment-card digit grouping plus Luhn validation | Repository-authored from public format conventions, pending attestation | Confirm no expression was copied from a third-party project |
| US SSN | Publicly known digit grouping | Repository-authored from public format conventions, pending attestation | Scope and false-positive behavior remain documented limitations |
| Email | Common simplified email-address syntax | Repository-authored, pending attestation | Not a complete RFC parser |
| US phone number | Common North American formatting | Repository-authored from public format conventions, pending attestation | Locale limited and not libphonenumber-equivalent |
| IPv4 and IPv6 | Public Internet address notation | Repository-authored simplified expressions, pending attestation | Current IPv4 expression is permissive and relies on documented scope |
| MAC address | Public hexadecimal hardware-address notation | Repository-authored from public format conventions, pending attestation | Supports selected common separators |
| IBAN | ISO 13616 structure plus mod-97 validation | Repository-authored from public specification rules, pending attestation | ISO standard text is not copied into the repository |
| Cryptocurrency wallet | Public Bitcoin and Ethereum address formats | Repository-authored simplified expressions, pending attestation | Selected formats only; no completeness claim |
| URL | Common HTTP and HTTPS prefix matching | Repository-authored simplified expression, pending attestation | Not a general URL parser |
| US ITIN | Publicly documented digit ranges and grouping | Repository-authored from public format conventions, pending attestation | Jurisdiction-specific scope must remain explicit |
| API key | Public vendor token prefixes and lengths | Repository-authored from publicly observable format conventions, pending attestation | Includes OpenAI, GitHub, and Slack-shaped values; does not imply vendor endorsement |

The patterns are short factual expressions over public formats, but shortness does not erase provenance. Maintainer attestation remains required before issue #22 can close.

## Validators and algorithms

| Material | Public basis | Current classification | Notes |
|---|---|---|---|
| Luhn validator | Public mod-10 checksum algorithm | Repository-authored from public algorithm rules, pending attestation | Implementation in `src/validators.rs`; no copied source identified |
| IBAN mod-97 validator | ISO 13616 validation procedure | Repository-authored from public algorithm rules, pending attestation | Implementation in `src/validators.rs`; no copied source identified |
| SHA-256 document fingerprinting | Standard cryptographic hash through `sha2` | Repository-authored composition using dependency API | Integrity and identity mechanism, not encryption or anonymization |
| Deterministic salted hash anonymization | SHA-256 composition | Repository-authored, pending security review | Correlation and low-entropy reversal risks are explicitly documented |
| Candidate resolution compatibility behavior | Existing project behavior | Repository-authored | Permanent security-oriented policy remains roadmap work |

## Fixtures and tests

Current tests and examples use synthetic or standard test values, including:

- `jane@example.com`;
- Luhn test number `4111111111111111`;
- IBAN test value `GB82WEST12345698765432`; and
- historical sequential-digit GitHub-token placeholders.

No real personal data or live credentials have been identified in the current tree or first automated history scan.

The historical fake GitHub-token examples are fingerprint-allowlisted in `.gitleaksignore`. Future token findings remain blocking.

Before new corpora or evaluation datasets are merged, each dataset must record source, license, redistribution right, generation method, and privacy review.

## Direct dependencies

| Dependency | Role | Declared license metadata | Project posture |
|---|---|---|---|
| `regex` | Pattern compilation and matching | MIT OR Apache-2.0 ecosystem metadata | Accepted current dependency |
| `sha2` | SHA-256 hashing | MIT OR Apache-2.0 ecosystem metadata | Accepted current dependency |
| `serde` | Optional serialization | MIT OR Apache-2.0 ecosystem metadata | Optional accepted dependency |

## Transitive dependency review

The first automated Cargo metadata review found:

- no dependency with missing license metadata;
- license expressions including MIT, Apache-2.0, `MIT OR Apache-2.0`, `Unlicense OR MIT`, and `(MIT OR Apache-2.0) AND Unicode-3.0`; and
- no network, model-runtime, native-library, or filesystem dependency introduced by the current crate graph.

This review validates declared metadata, not every license text or source package. Later public-beta hardening should add `cargo-deny` with explicit allow, ban, source, and advisory policy.

## Documentation sources

Documentation is repository-authored unless a source is explicitly cited.

Named external references include:

- Microsoft Presidio and its MIT license;
- the Developer Certificate of Origin;
- Keep a Changelog;
- semantic versioning expectations;
- GitHub security and contribution mechanisms; and
- public format or algorithm references where required by future recognizer contribution review.

External links and conventional document structure do not grant permission to copy source text. No substantial copied documentation has been identified in the current review.

## Publication authority still required

This inventory cannot itself prove ownership.

Before visibility, MythologIQ Labs LLC must affirm that:

1. it owns or is authorized to publish the existing repository-authored source and documentation;
2. no employee, contractor, client, or third party retains conflicting rights;
3. no NDA-bound or confidential material is included;
4. MIT is the intended distribution license for the exposed source; and
5. every identified third-party influence is compatible with the intended distribution.

## Review result

**Technical provenance status: provisionally acceptable, pending maintainer and rights-holder attestation.**

No copied or vendored third-party source has been identified. The remaining blocking work is explicit publication-authority attestation, final naming, manual review of public discussions and refs, and confirmation of the corrected automated audit.
