//! Pure, explicit candidate resolution for the authoritative analysis path.
//!
//! Resolution never reads or transforms source plaintext. It canonicalizes a
//! caller-provided candidate collection, applies one named versioned policy,
//! and returns a separate report containing resolved output and bounded
//! non-plaintext decision evidence. Legacy analyzer behavior is intentionally
//! unaffected.

use core::cmp::Ordering;
use core::fmt;

use crate::document::DocumentBinding;
use crate::types::{Confidence, EntityId, Evidence, Finding, RecognizerId, Span};

/// Version shared by the first accepted resolution-policy contracts.
pub const RESOLUTION_POLICY_VERSION_V1: u16 = 1;
/// Default maximum number of input candidates accepted by the resolver.
pub const DEFAULT_RESOLUTION_CANDIDATE_LIMIT: usize = 10_000;
/// Default maximum number of resolved outputs produced by the resolver.
pub const DEFAULT_RESOLUTION_OUTPUT_LIMIT: usize = 10_000;
/// Default maximum number of detailed resolution decisions retained.
pub const DEFAULT_RESOLUTION_DECISION_LIMIT: usize = 10_000;

/// A named candidate-resolution policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum ResolutionPolicy {
    /// Preserve every qualifying candidate without overlap elimination.
    ReportAll,
    /// Select a deterministic non-overlapping candidate set.
    BestCandidate,
    /// Union every connected strict-overlap component for coverage-safe redaction.
    ConservativeRedaction,
}

impl ResolutionPolicy {
    /// Stable policy identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::ReportAll => "report_all",
            Self::BestCandidate => "best_candidate",
            Self::ConservativeRedaction => "conservative_redaction",
        }
    }

    /// Stable semantic version for this policy contract.
    pub const fn version(self) -> u16 {
        RESOLUTION_POLICY_VERSION_V1
    }
}

/// Bounded configuration for one resolution execution.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolutionOptions {
    policy: ResolutionPolicy,
    minimum_confidence: Confidence,
    max_candidates: usize,
    max_resolved: usize,
    max_decisions: usize,
    entity_priority: Vec<EntityId>,
    recognizer_priority: Vec<RecognizerId>,
}

impl ResolutionOptions {
    /// Construct options for one explicit policy using bounded defaults.
    pub fn new(policy: ResolutionPolicy) -> Self {
        Self {
            policy,
            minimum_confidence: Confidence::new(0.0).expect("zero confidence is valid"),
            max_candidates: DEFAULT_RESOLUTION_CANDIDATE_LIMIT,
            max_resolved: DEFAULT_RESOLUTION_OUTPUT_LIMIT,
            max_decisions: DEFAULT_RESOLUTION_DECISION_LIMIT,
            entity_priority: Vec::new(),
            recognizer_priority: Vec::new(),
        }
    }

    /// Set the minimum candidate confidence accepted by resolution.
    pub fn with_minimum_confidence(mut self, minimum: Confidence) -> Self {
        self.minimum_confidence = minimum;
        self
    }

    /// Set the maximum number of caller-provided candidates.
    pub const fn with_max_candidates(mut self, maximum: usize) -> Self {
        self.max_candidates = maximum;
        self
    }

    /// Set the maximum number of resolved outputs.
    pub const fn with_max_resolved(mut self, maximum: usize) -> Self {
        self.max_resolved = maximum;
        self
    }

    /// Set the maximum number of detailed decisions retained.
    pub const fn with_max_decisions(mut self, maximum: usize) -> Self {
        self.max_decisions = maximum;
        self
    }

    /// Set entity precedence from highest to lowest priority.
    pub fn with_entity_priority(mut self, priority: impl IntoIterator<Item = EntityId>) -> Self {
        self.entity_priority = priority.into_iter().collect();
        self
    }

    /// Set recognizer precedence from highest to lowest priority.
    pub fn with_recognizer_priority(
        mut self,
        priority: impl IntoIterator<Item = RecognizerId>,
    ) -> Self {
        self.recognizer_priority = priority.into_iter().collect();
        self
    }

    /// Selected policy.
    pub const fn policy(&self) -> ResolutionPolicy {
        self.policy
    }

    /// Minimum qualifying confidence.
    pub const fn minimum_confidence(&self) -> Confidence {
        self.minimum_confidence
    }

    /// Maximum accepted caller-provided candidates.
    pub const fn max_candidates(&self) -> usize {
        self.max_candidates
    }

    /// Maximum resolved outputs.
    pub const fn max_resolved(&self) -> usize {
        self.max_resolved
    }

    /// Maximum detailed decisions.
    pub const fn max_decisions(&self) -> usize {
        self.max_decisions
    }

    /// Entity precedence from highest to lowest priority.
    pub fn entity_priority(&self) -> &[EntityId] {
        &self.entity_priority
    }

    /// Recognizer precedence from highest to lowest priority.
    pub fn recognizer_priority(&self) -> &[RecognizerId] {
        &self.recognizer_priority
    }
}

impl Default for ResolutionOptions {
    fn default() -> Self {
        Self::new(ResolutionPolicy::ReportAll)
    }
}

/// Entity identity represented by one resolved output.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(tag = "kind", content = "entity", rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum ResolvedEntity {
    /// Every contributing candidate had this entity identity.
    Single(EntityId),
    /// Contributors contained different entity identities.
    Mixed,
}

/// One policy-resolved output.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(tag = "kind", rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum ResolvedFinding {
    /// A canonical source candidate retained without span synthesis.
    Candidate {
        /// Canonical candidate ordinal.
        candidate: usize,
    },
    /// Union of one connected strict-overlap component.
    Union {
        /// Coverage span from the minimum start to maximum end.
        span: Span,
        /// Single shared entity or explicit mixed identity.
        entity: ResolvedEntity,
        /// Canonical candidate ordinals contributing to this union.
        candidates: Vec<usize>,
        /// Exact common source binding, when candidates are document-bound.
        document: Option<DocumentBinding>,
    },
}

impl ResolvedFinding {
    /// Canonical contributors represented by this output.
    pub fn candidates(&self) -> &[usize] {
        match self {
            Self::Candidate { candidate } => core::slice::from_ref(candidate),
            Self::Union { candidates, .. } => candidates,
        }
    }

    /// Coverage span represented by this output.
    pub fn span(&self, canonical_candidates: &[Finding]) -> Option<Span> {
        match self {
            Self::Candidate { candidate } => {
                canonical_candidates.get(*candidate).map(Finding::span)
            }
            Self::Union { span, .. } => Some(*span),
        }
    }

    /// Exact document binding represented by this output, when known.
    pub fn document_binding<'a>(
        &'a self,
        canonical_candidates: &'a [Finding],
    ) -> Option<&'a DocumentBinding> {
        match self {
            Self::Candidate { candidate } => canonical_candidates
                .get(*candidate)
                .and_then(Finding::document_binding),
            Self::Union { document, .. } => document.as_ref(),
        }
    }
}

/// Bounded, non-plaintext evidence explaining one resolution outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(tag = "kind", rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum ResolutionDecision {
    /// `ReportAll` retained every qualifying canonical candidate.
    ReportedAll { retained: usize },
    /// A canonical candidate was retained by `BestCandidate`.
    Retained { candidate: usize },
    /// An exact duplicate collapsed onto a retained canonical candidate.
    CollapsedDuplicate { candidate: usize, retained: usize },
    /// An overlapping candidate lost to a retained candidate.
    RejectedOverlap { candidate: usize, retained: usize },
    /// A connected overlap component produced one conservative union.
    ConservativeUnion {
        output: usize,
        candidates: Vec<usize>,
    },
}

/// Completeness status for a successful resolution report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ResolutionStatus {
    decision_limit_reached: bool,
}

impl ResolutionStatus {
    const fn new(decision_limit_reached: bool) -> Self {
        Self {
            decision_limit_reached,
        }
    }

    /// Whether detailed decision evidence was truncated.
    pub const fn decision_limit_reached(self) -> bool {
        self.decision_limit_reached
    }

    /// Whether the resolved output is complete.
    ///
    /// Candidate and output limits are hard errors. A successful report therefore
    /// always contains complete resolved output even when decision evidence was
    /// explicitly truncated.
    pub const fn output_complete(self) -> bool {
        true
    }

    /// Whether all detailed decision evidence was retained.
    pub const fn decision_evidence_complete(self) -> bool {
        !self.decision_limit_reached
    }
}

/// Separate candidate snapshot, resolved output, and bounded decision evidence.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ResolutionReport {
    policy: ResolutionPolicy,
    policy_version: u16,
    document: Option<DocumentBinding>,
    candidates: Vec<Finding>,
    resolved: Vec<ResolvedFinding>,
    decisions: Vec<ResolutionDecision>,
    status: ResolutionStatus,
}

impl ResolutionReport {
    /// Stable policy identifier.
    pub const fn policy_id(&self) -> &'static str {
        self.policy.id()
    }

    /// Policy variant used for this report.
    pub const fn policy(&self) -> ResolutionPolicy {
        self.policy
    }

    /// Stable policy semantic version.
    pub const fn policy_version(&self) -> u16 {
        self.policy_version
    }

    /// Exact common source binding, when candidates were document-bound.
    pub fn document_binding(&self) -> Option<&DocumentBinding> {
        self.document.as_ref()
    }

    /// Canonically ordered qualifying candidate snapshot.
    pub fn candidates(&self) -> &[Finding] {
        &self.candidates
    }

    /// Resolved outputs, ordered by source span and canonical identity.
    pub fn resolved(&self) -> &[ResolvedFinding] {
        &self.resolved
    }

    /// Retained non-plaintext decision evidence.
    pub fn decisions(&self) -> &[ResolutionDecision] {
        &self.decisions
    }

    /// Output and decision-evidence completeness status.
    pub const fn status(&self) -> ResolutionStatus {
        self.status
    }
}

/// Failure to produce a complete resolved output.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResolutionError {
    /// Caller-provided candidate count exceeded the configured bound.
    CandidateLimitExceeded { actual: usize, maximum: usize },
    /// Complete resolved output would exceed the configured bound.
    ResolvedLimitExceeded { required: usize, maximum: usize },
    /// Candidates did not share one consistent binding state and exact binding.
    InconsistentDocumentBinding,
}

impl fmt::Display for ResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CandidateLimitExceeded { actual, maximum } => write!(
                formatter,
                "resolution candidate count {actual} exceeds maximum {maximum}"
            ),
            Self::ResolvedLimitExceeded { required, maximum } => write!(
                formatter,
                "resolution requires {required} outputs, exceeding maximum {maximum}"
            ),
            Self::InconsistentDocumentBinding => formatter
                .write_str("resolution candidates do not share one consistent document binding"),
        }
    }
}

impl std::error::Error for ResolutionError {}

/// Resolve validated findings under one explicit versioned policy.
///
/// The caller's slice is never mutated. The report owns a canonical candidate
/// snapshot so decision references remain stable independently of caller vector
/// order. Candidate and output bounds fail before a partial report can be
/// returned. Decision evidence alone may be explicitly truncated.
pub fn resolve_candidates(
    candidates: &[Finding],
    options: &ResolutionOptions,
) -> Result<ResolutionReport, ResolutionError> {
    if candidates.len() > options.max_candidates {
        return Err(ResolutionError::CandidateLimitExceeded {
            actual: candidates.len(),
            maximum: options.max_candidates,
        });
    }

    let document = consistent_document_binding(candidates)?;
    let mut canonical: Vec<Finding> = candidates
        .iter()
        .filter(|candidate| candidate.confidence() >= options.minimum_confidence)
        .cloned()
        .collect();
    canonical.sort_by(compare_canonical_candidates);

    let mut decisions = DecisionSink::new(options.max_decisions);
    let resolved = match options.policy {
        ResolutionPolicy::ReportAll => resolve_report_all(&canonical, options, &mut decisions)?,
        ResolutionPolicy::BestCandidate => {
            resolve_best_candidate(&canonical, options, &mut decisions)?
        }
        ResolutionPolicy::ConservativeRedaction => {
            resolve_conservative(&canonical, options, document.as_ref(), &mut decisions)?
        }
    };

    Ok(ResolutionReport {
        policy: options.policy,
        policy_version: options.policy.version(),
        document,
        candidates: canonical,
        resolved,
        decisions: decisions.decisions,
        status: ResolutionStatus::new(decisions.limit_reached),
    })
}

fn resolve_report_all(
    candidates: &[Finding],
    options: &ResolutionOptions,
    decisions: &mut DecisionSink,
) -> Result<Vec<ResolvedFinding>, ResolutionError> {
    enforce_resolved_limit(candidates.len(), options.max_resolved)?;
    decisions.push(ResolutionDecision::ReportedAll {
        retained: candidates.len(),
    });
    Ok((0..candidates.len())
        .map(|candidate| ResolvedFinding::Candidate { candidate })
        .collect())
}

fn resolve_best_candidate(
    candidates: &[Finding],
    options: &ResolutionOptions,
    decisions: &mut DecisionSink,
) -> Result<Vec<ResolvedFinding>, ResolutionError> {
    let mut precedence: Vec<usize> = (0..candidates.len()).collect();
    precedence.sort_by(|left, right| compare_precedence(*left, *right, candidates, options));

    let mut selected = Vec::new();
    for candidate in precedence {
        if let Some(retained) = selected
            .iter()
            .copied()
            .find(|retained| candidates[candidate] == candidates[*retained])
        {
            decisions.push(ResolutionDecision::CollapsedDuplicate {
                candidate,
                retained,
            });
            continue;
        }

        if let Some(retained) = selected.iter().copied().find(|retained| {
            spans_overlap(candidates[candidate].span(), candidates[*retained].span())
        }) {
            decisions.push(ResolutionDecision::RejectedOverlap {
                candidate,
                retained,
            });
            continue;
        }

        selected.push(candidate);
        decisions.push(ResolutionDecision::Retained { candidate });
    }

    enforce_resolved_limit(selected.len(), options.max_resolved)?;
    selected.sort_unstable();
    Ok(selected
        .into_iter()
        .map(|candidate| ResolvedFinding::Candidate { candidate })
        .collect())
}

fn resolve_conservative(
    candidates: &[Finding],
    options: &ResolutionOptions,
    document: Option<&DocumentBinding>,
    decisions: &mut DecisionSink,
) -> Result<Vec<ResolvedFinding>, ResolutionError> {
    let components = overlap_components(candidates);
    enforce_resolved_limit(components.len(), options.max_resolved)?;

    let mut resolved = Vec::with_capacity(components.len());
    for component in components {
        let first = component[0];
        let start = candidates[first].span().start();
        let end = component
            .iter()
            .map(|candidate| candidates[*candidate].span().end())
            .max()
            .expect("component is non-empty");
        let span = Span::new(start, end).expect("candidate union is a non-empty ordered span");
        let first_entity = candidates[first].entity();
        let entity = if component
            .iter()
            .all(|candidate| candidates[*candidate].entity() == first_entity)
        {
            ResolvedEntity::Single(first_entity.clone())
        } else {
            ResolvedEntity::Mixed
        };

        let output = resolved.len();
        resolved.push(ResolvedFinding::Union {
            span,
            entity,
            candidates: component.clone(),
            document: document.cloned(),
        });
        decisions.push(ResolutionDecision::ConservativeUnion {
            output,
            candidates: component,
        });
    }

    Ok(resolved)
}

fn overlap_components(candidates: &[Finding]) -> Vec<Vec<usize>> {
    let mut components = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let mut maximum_end = 0;

    for (candidate, finding) in candidates.iter().enumerate() {
        if current.is_empty() {
            current.push(candidate);
            maximum_end = finding.span().end();
            continue;
        }

        if finding.span().start() < maximum_end {
            current.push(candidate);
            maximum_end = maximum_end.max(finding.span().end());
        } else {
            components.push(core::mem::take(&mut current));
            current.push(candidate);
            maximum_end = finding.span().end();
        }
    }

    if !current.is_empty() {
        components.push(current);
    }
    components
}

fn enforce_resolved_limit(required: usize, maximum: usize) -> Result<(), ResolutionError> {
    if required > maximum {
        Err(ResolutionError::ResolvedLimitExceeded { required, maximum })
    } else {
        Ok(())
    }
}

fn consistent_document_binding(
    candidates: &[Finding],
) -> Result<Option<DocumentBinding>, ResolutionError> {
    let expected = candidates
        .first()
        .and_then(Finding::document_binding)
        .cloned();
    for candidate in candidates.iter().skip(1) {
        if candidate.document_binding() != expected.as_ref() {
            return Err(ResolutionError::InconsistentDocumentBinding);
        }
    }
    Ok(expected)
}

fn spans_overlap(left: Span, right: Span) -> bool {
    left.start() < right.end() && right.start() < left.end()
}

fn compare_precedence(
    left: usize,
    right: usize,
    candidates: &[Finding],
    options: &ResolutionOptions,
) -> Ordering {
    let left_candidate = &candidates[left];
    let right_candidate = &candidates[right];

    priority_rank(options.entity_priority(), left_candidate.entity())
        .cmp(&priority_rank(
            options.entity_priority(),
            right_candidate.entity(),
        ))
        .then_with(|| {
            recognizer_priority_rank(options.recognizer_priority(), left_candidate.recognizer())
                .cmp(&recognizer_priority_rank(
                    options.recognizer_priority(),
                    right_candidate.recognizer(),
                ))
        })
        .then_with(|| {
            right_candidate
                .confidence()
                .get()
                .partial_cmp(&left_candidate.confidence().get())
                .unwrap_or(Ordering::Equal)
        })
        .then_with(|| {
            right_candidate
                .span()
                .len()
                .cmp(&left_candidate.span().len())
        })
        .then_with(|| {
            left_candidate
                .span()
                .start()
                .cmp(&right_candidate.span().start())
        })
        .then_with(|| {
            left_candidate
                .span()
                .end()
                .cmp(&right_candidate.span().end())
        })
        .then_with(|| {
            left_candidate
                .entity()
                .as_str()
                .cmp(right_candidate.entity().as_str())
        })
        .then_with(|| {
            compare_optional_recognizer(left_candidate.recognizer(), right_candidate.recognizer())
        })
        .then_with(|| left.cmp(&right))
}

fn priority_rank(priority: &[EntityId], value: &EntityId) -> usize {
    priority
        .iter()
        .position(|candidate| candidate == value)
        .unwrap_or(usize::MAX)
}

fn recognizer_priority_rank(priority: &[RecognizerId], value: Option<&RecognizerId>) -> usize {
    value
        .and_then(|value| priority.iter().position(|candidate| candidate == value))
        .unwrap_or(usize::MAX)
}

fn compare_canonical_candidates(left: &Finding, right: &Finding) -> Ordering {
    left.span()
        .start()
        .cmp(&right.span().start())
        .then_with(|| left.span().end().cmp(&right.span().end()))
        .then_with(|| left.entity().as_str().cmp(right.entity().as_str()))
        .then_with(|| compare_optional_recognizer(left.recognizer(), right.recognizer()))
        .then_with(|| {
            right
                .confidence()
                .get()
                .partial_cmp(&left.confidence().get())
                .unwrap_or(Ordering::Equal)
        })
        .then_with(|| compare_evidence_slices(left.evidence(), right.evidence()))
        .then_with(|| compare_optional_binding(left.document_binding(), right.document_binding()))
}

fn compare_optional_recognizer(
    left: Option<&RecognizerId>,
    right: Option<&RecognizerId>,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => left.as_str().cmp(right.as_str()),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn compare_optional_binding(
    left: Option<&DocumentBinding>,
    right: Option<&DocumentBinding>,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => left
            .id()
            .as_str()
            .cmp(right.id().as_str())
            .then_with(|| left.byte_len().cmp(&right.byte_len()))
            .then_with(|| {
                left.fingerprint()
                    .as_bytes()
                    .cmp(right.fingerprint().as_bytes())
            }),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn compare_evidence_slices(left: &[Evidence], right: &[Evidence]) -> Ordering {
    for (left, right) in left.iter().zip(right) {
        let ordering = compare_evidence(left, right);
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    left.len().cmp(&right.len())
}

fn compare_evidence(left: &Evidence, right: &Evidence) -> Ordering {
    evidence_rank(left)
        .cmp(&evidence_rank(right))
        .then_with(|| match (left, right) {
            (Evidence::LegacyResult, Evidence::LegacyResult)
            | (Evidence::LegacyValidatorAccepted, Evidence::LegacyValidatorAccepted) => {
                Ordering::Equal
            }
            (Evidence::Pattern { pattern_id: left }, Evidence::Pattern { pattern_id: right }) => {
                left.as_str().cmp(right.as_str())
            }
            (
                Evidence::Validator {
                    validator_id: left_id,
                    accepted: left_accepted,
                },
                Evidence::Validator {
                    validator_id: right_id,
                    accepted: right_accepted,
                },
            ) => left_id
                .as_str()
                .cmp(right_id.as_str())
                .then_with(|| left_accepted.cmp(right_accepted)),
            (
                Evidence::Context {
                    context_id: left_id,
                    distance_bytes: left_distance,
                    positive: left_positive,
                },
                Evidence::Context {
                    context_id: right_id,
                    distance_bytes: right_distance,
                    positive: right_positive,
                },
            ) => left_id
                .as_str()
                .cmp(right_id.as_str())
                .then_with(|| left_distance.cmp(right_distance))
                .then_with(|| left_positive.cmp(right_positive)),
            _ => Ordering::Equal,
        })
}

const fn evidence_rank(evidence: &Evidence) -> u8 {
    match evidence {
        Evidence::LegacyResult => 0,
        Evidence::LegacyValidatorAccepted => 1,
        Evidence::Pattern { .. } => 2,
        Evidence::Validator { .. } => 3,
        Evidence::Context { .. } => 4,
    }
}

struct DecisionSink {
    maximum: usize,
    decisions: Vec<ResolutionDecision>,
    limit_reached: bool,
}

impl DecisionSink {
    fn new(maximum: usize) -> Self {
        Self {
            maximum,
            decisions: Vec::new(),
            limit_reached: false,
        }
    }

    fn push(&mut self, decision: ResolutionDecision) {
        if self.decisions.len() < self.maximum {
            self.decisions.push(decision);
        } else {
            self.limit_reached = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::DocumentBinding;
    use crate::types::{DocumentId, MetadataId};

    fn entity(value: &str) -> EntityId {
        EntityId::new(value).expect("valid entity")
    }

    fn recognizer(value: &str) -> RecognizerId {
        RecognizerId::new(value).expect("valid recognizer")
    }

    fn confidence(value: f32) -> Confidence {
        Confidence::new(value).expect("valid confidence")
    }

    fn finding(name: &str, start: usize, end: usize, score: f32, source: &str) -> Finding {
        Finding::new(
            entity(name),
            Span::new(start, end).expect("valid span"),
            confidence(score),
        )
        .with_recognizer(recognizer(source))
        .with_evidence([Evidence::Pattern {
            pattern_id: MetadataId::new(format!("pattern.{source}")).expect("valid pattern ID"),
        }])
    }

    fn bound_finding(
        name: &str,
        start: usize,
        end: usize,
        score: f32,
        source: &str,
        document: &DocumentBinding,
    ) -> Finding {
        finding(name, start, end, score, source).with_document_binding(document.clone())
    }

    fn resolve(
        candidates: &[Finding],
        policy: ResolutionPolicy,
    ) -> Result<ResolutionReport, ResolutionError> {
        resolve_candidates(candidates, &ResolutionOptions::new(policy))
    }

    fn candidate_ordinals(report: &ResolutionReport) -> Vec<usize> {
        report
            .resolved()
            .iter()
            .map(|resolved| match resolved {
                ResolvedFinding::Candidate { candidate } => *candidate,
                ResolvedFinding::Union { .. } => panic!("expected candidate output"),
            })
            .collect()
    }

    fn union_spans(report: &ResolutionReport) -> Vec<Span> {
        report
            .resolved()
            .iter()
            .map(|resolved| match resolved {
                ResolvedFinding::Union { span, .. } => *span,
                ResolvedFinding::Candidate { .. } => panic!("expected union output"),
            })
            .collect()
    }

    fn permutations<T: Clone>(values: &[T]) -> Vec<Vec<T>> {
        fn visit<T: Clone>(remaining: Vec<T>, prefix: Vec<T>, output: &mut Vec<Vec<T>>) {
            if remaining.is_empty() {
                output.push(prefix);
                return;
            }
            for index in 0..remaining.len() {
                let mut next_remaining = remaining.clone();
                let value = next_remaining.remove(index);
                let mut next_prefix = prefix.clone();
                next_prefix.push(value);
                visit(next_remaining, next_prefix, output);
            }
        }

        let mut output = Vec::new();
        visit(values.to_vec(), Vec::new(), &mut output);
        output
    }

    fn assert_permutation_invariant(candidates: &[Finding], options: &ResolutionOptions) {
        let expected = resolve_candidates(candidates, options).expect("baseline resolution");
        for permutation in permutations(candidates) {
            assert_eq!(
                resolve_candidates(&permutation, options).expect("permutation resolution"),
                expected
            );
        }
    }

    #[test]
    fn report_all_preserves_disjoint_candidates_and_duplicates() {
        let first = finding("A", 0, 4, 0.8, "r1");
        let second = finding("B", 6, 9, 0.7, "r2");
        let report = resolve(
            &[second.clone(), first.clone(), first.clone()],
            ResolutionPolicy::ReportAll,
        )
        .expect("resolution succeeds");

        assert_eq!(report.policy_id(), "report_all");
        assert_eq!(report.policy_version(), 1);
        assert_eq!(report.candidates().len(), 3);
        assert_eq!(candidate_ordinals(&report), vec![0, 1, 2]);
        assert_eq!(
            report.decisions(),
            &[ResolutionDecision::ReportedAll { retained: 3 }]
        );
    }

    #[test]
    fn report_all_filters_by_minimum_confidence() {
        let options = ResolutionOptions::new(ResolutionPolicy::ReportAll)
            .with_minimum_confidence(confidence(0.75));
        let report = resolve_candidates(
            &[finding("A", 0, 4, 0.8, "r1"), finding("B", 6, 9, 0.7, "r2")],
            &options,
        )
        .expect("resolution succeeds");

        assert_eq!(report.candidates().len(), 1);
        assert_eq!(report.candidates()[0].entity().as_str(), "A");
    }

    #[test]
    fn best_candidate_collapses_exact_duplicates() {
        let candidate = finding("A", 0, 4, 0.8, "r1");
        let report = resolve(
            &[candidate.clone(), candidate],
            ResolutionPolicy::BestCandidate,
        )
        .expect("resolution succeeds");

        assert_eq!(candidate_ordinals(&report), vec![0]);
        assert!(report.decisions().iter().any(|decision| matches!(
            decision,
            ResolutionDecision::CollapsedDuplicate {
                candidate: 1,
                retained: 0
            }
        )));
    }

    #[test]
    fn best_candidate_uses_lexical_entity_tie_break_without_priority() {
        let report = resolve(
            &[finding("B", 0, 4, 0.8, "r1"), finding("A", 0, 4, 0.8, "r1")],
            ResolutionPolicy::BestCandidate,
        )
        .expect("resolution succeeds");

        let selected = candidate_ordinals(&report)[0];
        assert_eq!(report.candidates()[selected].entity().as_str(), "A");
    }

    #[test]
    fn best_candidate_honors_entity_priority() {
        let options = ResolutionOptions::new(ResolutionPolicy::BestCandidate)
            .with_entity_priority([entity("B"), entity("A")]);
        let report = resolve_candidates(
            &[finding("A", 0, 4, 0.8, "r1"), finding("B", 0, 4, 0.8, "r1")],
            &options,
        )
        .expect("resolution succeeds");

        let selected = candidate_ordinals(&report)[0];
        assert_eq!(report.candidates()[selected].entity().as_str(), "B");
    }

    #[test]
    fn best_candidate_honors_recognizer_priority() {
        let options = ResolutionOptions::new(ResolutionPolicy::BestCandidate)
            .with_recognizer_priority([recognizer("r2"), recognizer("r1")]);
        let report = resolve_candidates(
            &[finding("A", 0, 4, 0.8, "r1"), finding("A", 0, 4, 0.8, "r2")],
            &options,
        )
        .expect("resolution succeeds");

        let selected = candidate_ordinals(&report)[0];
        assert_eq!(
            report.candidates()[selected]
                .recognizer()
                .expect("recognizer")
                .as_str(),
            "r2"
        );
    }

    #[test]
    fn best_candidate_prefers_confidence_over_containing_span() {
        let report = resolve(
            &[
                finding("OUTER", 0, 10, 0.6, "r1"),
                finding("INNER", 2, 6, 0.9, "r2"),
            ],
            ResolutionPolicy::BestCandidate,
        )
        .expect("resolution succeeds");

        let selected = candidate_ordinals(&report)[0];
        assert_eq!(report.candidates()[selected].entity().as_str(), "INNER");
    }

    #[test]
    fn best_candidate_prefers_longer_span_after_equal_confidence() {
        let report = resolve(
            &[
                finding("OUTER", 0, 10, 0.9, "r1"),
                finding("INNER", 2, 6, 0.9, "r2"),
            ],
            ResolutionPolicy::BestCandidate,
        )
        .expect("resolution succeeds");

        let selected = candidate_ordinals(&report)[0];
        assert_eq!(report.candidates()[selected].entity().as_str(), "OUTER");
    }

    #[test]
    fn best_candidate_handles_partial_intersection() {
        let report = resolve(
            &[
                finding("A", 0, 6, 0.7, "r1"),
                finding("B", 4, 10, 0.8, "r2"),
            ],
            ResolutionPolicy::BestCandidate,
        )
        .expect("resolution succeeds");

        let selected = candidate_ordinals(&report)[0];
        assert_eq!(report.candidates()[selected].entity().as_str(), "B");
    }

    #[test]
    fn best_candidate_keeps_non_overlapping_ends_of_lower_priority_bridge() {
        let report = resolve(
            &[
                finding("A", 0, 5, 0.8, "r1"),
                finding("BRIDGE", 4, 9, 0.7, "r2"),
                finding("C", 8, 12, 0.8, "r3"),
            ],
            ResolutionPolicy::BestCandidate,
        )
        .expect("resolution succeeds");

        let entities: Vec<&str> = candidate_ordinals(&report)
            .into_iter()
            .map(|candidate| report.candidates()[candidate].entity().as_str())
            .collect();
        assert_eq!(entities, vec!["A", "C"]);
    }

    #[test]
    fn best_candidate_high_priority_bridge_rejects_both_ends() {
        let report = resolve(
            &[
                finding("A", 0, 5, 0.8, "r1"),
                finding("BRIDGE", 4, 9, 0.95, "r2"),
                finding("C", 8, 12, 0.8, "r3"),
            ],
            ResolutionPolicy::BestCandidate,
        )
        .expect("resolution succeeds");

        let selected = candidate_ordinals(&report)[0];
        assert_eq!(report.candidates()[selected].entity().as_str(), "BRIDGE");
    }

    #[test]
    fn adjacent_candidates_remain_separate_for_selection_and_union() {
        let candidates = [finding("A", 0, 4, 0.8, "r1"), finding("B", 4, 8, 0.9, "r2")];
        let best = resolve(&candidates, ResolutionPolicy::BestCandidate).expect("best succeeds");
        let conservative = resolve(&candidates, ResolutionPolicy::ConservativeRedaction)
            .expect("conservative succeeds");

        assert_eq!(best.resolved().len(), 2);
        assert_eq!(
            union_spans(&conservative),
            vec![Span::new(0, 4).unwrap(), Span::new(4, 8).unwrap()]
        );
    }

    #[test]
    fn conservative_unions_containment_as_mixed() {
        let report = resolve(
            &[
                finding("OUTER", 0, 10, 0.6, "r1"),
                finding("INNER", 2, 6, 0.9, "r2"),
            ],
            ResolutionPolicy::ConservativeRedaction,
        )
        .expect("resolution succeeds");

        assert_eq!(union_spans(&report), vec![Span::new(0, 10).unwrap()]);
        assert!(matches!(
            &report.resolved()[0],
            ResolvedFinding::Union {
                entity: ResolvedEntity::Mixed,
                candidates,
                ..
            } if candidates == &vec![0, 1]
        ));
    }

    #[test]
    fn conservative_preserves_single_entity_identity() {
        let report = resolve(
            &[
                finding("A", 0, 6, 0.7, "r1"),
                finding("A", 4, 10, 0.8, "r2"),
            ],
            ResolutionPolicy::ConservativeRedaction,
        )
        .expect("resolution succeeds");

        assert!(matches!(
            &report.resolved()[0],
            ResolvedFinding::Union {
                entity: ResolvedEntity::Single(value),
                ..
            } if value.as_str() == "A"
        ));
    }

    #[test]
    fn conservative_unions_chained_overlap_component() {
        let report = resolve(
            &[
                finding("A", 0, 5, 0.8, "r1"),
                finding("B", 4, 9, 0.7, "r2"),
                finding("C", 8, 12, 0.8, "r3"),
            ],
            ResolutionPolicy::ConservativeRedaction,
        )
        .expect("resolution succeeds");

        assert_eq!(union_spans(&report), vec![Span::new(0, 12).unwrap()]);
    }

    #[test]
    fn open_entity_identifiers_resolve_normally() {
        let report = resolve(
            &[
                finding("customer.secret", 0, 5, 0.9, "custom.one"),
                finding("tenant.reference", 3, 8, 0.8, "custom.two"),
            ],
            ResolutionPolicy::ConservativeRedaction,
        )
        .expect("resolution succeeds");

        assert!(matches!(
            &report.resolved()[0],
            ResolvedFinding::Union {
                entity: ResolvedEntity::Mixed,
                ..
            }
        ));
    }

    #[test]
    fn matching_document_binding_is_preserved() {
        let document = DocumentBinding::for_text(DocumentId::new("doc-1").unwrap(), "abcdefghij");
        let report = resolve(
            &[
                bound_finding("A", 0, 6, 0.7, "r1", &document),
                bound_finding("B", 4, 10, 0.8, "r2", &document),
            ],
            ResolutionPolicy::ConservativeRedaction,
        )
        .expect("resolution succeeds");

        assert_eq!(report.document_binding(), Some(&document));
        assert_eq!(
            report.resolved()[0].document_binding(report.candidates()),
            Some(&document)
        );
    }

    #[test]
    fn different_document_bindings_are_rejected() {
        let first = DocumentBinding::for_text(DocumentId::new("doc-1").unwrap(), "abcdefghij");
        let second = DocumentBinding::for_text(DocumentId::new("doc-2").unwrap(), "abcdefghij");
        assert_eq!(
            resolve(
                &[
                    bound_finding("A", 0, 6, 0.7, "r1", &first),
                    bound_finding("B", 4, 10, 0.8, "r2", &second),
                ],
                ResolutionPolicy::BestCandidate,
            ),
            Err(ResolutionError::InconsistentDocumentBinding)
        );
    }

    #[test]
    fn mixed_bound_and_unbound_candidates_are_rejected() {
        let document = DocumentBinding::for_text(DocumentId::new("doc-1").unwrap(), "abcdefghij");
        assert_eq!(
            resolve(
                &[
                    bound_finding("A", 0, 6, 0.7, "r1", &document),
                    finding("B", 4, 10, 0.8, "r2"),
                ],
                ResolutionPolicy::BestCandidate,
            ),
            Err(ResolutionError::InconsistentDocumentBinding)
        );
    }

    #[test]
    fn empty_candidate_set_returns_complete_empty_output() {
        for policy in [
            ResolutionPolicy::ReportAll,
            ResolutionPolicy::BestCandidate,
            ResolutionPolicy::ConservativeRedaction,
        ] {
            let report = resolve(&[], policy).expect("empty resolution succeeds");
            assert!(report.candidates().is_empty());
            assert!(report.resolved().is_empty());
            assert!(report.status().output_complete());
        }
    }

    #[test]
    fn candidate_limit_is_a_hard_error() {
        let options = ResolutionOptions::new(ResolutionPolicy::ReportAll).with_max_candidates(1);
        assert_eq!(
            resolve_candidates(
                &[finding("A", 0, 2, 0.8, "r1"), finding("B", 3, 5, 0.8, "r2"),],
                &options,
            ),
            Err(ResolutionError::CandidateLimitExceeded {
                actual: 2,
                maximum: 1
            })
        );
    }

    #[test]
    fn resolved_limit_is_a_hard_error() {
        let options = ResolutionOptions::new(ResolutionPolicy::ReportAll).with_max_resolved(1);
        assert_eq!(
            resolve_candidates(
                &[finding("A", 0, 2, 0.8, "r1"), finding("B", 3, 5, 0.8, "r2"),],
                &options,
            ),
            Err(ResolutionError::ResolvedLimitExceeded {
                required: 2,
                maximum: 1
            })
        );
    }

    #[test]
    fn decision_limit_truncates_evidence_not_output() {
        let options = ResolutionOptions::new(ResolutionPolicy::BestCandidate).with_max_decisions(0);
        let report = resolve_candidates(
            &[finding("A", 0, 2, 0.8, "r1"), finding("B", 3, 5, 0.8, "r2")],
            &options,
        )
        .expect("resolution succeeds");

        assert_eq!(report.resolved().len(), 2);
        assert!(report.decisions().is_empty());
        assert!(report.status().decision_limit_reached());
        assert!(report.status().output_complete());
        assert!(!report.status().decision_evidence_complete());
    }

    #[test]
    fn input_candidates_are_not_mutated() {
        let candidates = vec![
            finding("B", 4, 10, 0.8, "r2"),
            finding("A", 0, 6, 0.7, "r1"),
        ];
        let before = candidates.clone();
        let _ = resolve(&candidates, ResolutionPolicy::ConservativeRedaction)
            .expect("resolution succeeds");
        assert_eq!(candidates, before);
    }

    #[test]
    fn policies_are_invariant_across_input_permutations() {
        let candidates = vec![
            finding("A", 0, 5, 0.8, "r1"),
            finding("BRIDGE", 4, 9, 0.7, "r2"),
            finding("C", 8, 12, 0.8, "r3"),
        ];

        for policy in [
            ResolutionPolicy::ReportAll,
            ResolutionPolicy::BestCandidate,
            ResolutionPolicy::ConservativeRedaction,
        ] {
            assert_permutation_invariant(&candidates, &ResolutionOptions::new(policy));
        }
    }

    #[test]
    fn unicode_aligned_byte_spans_resolve_without_coordinate_changes() {
        let report = resolve(
            &[finding("A", 1, 3, 0.8, "r1"), finding("B", 3, 4, 0.8, "r2")],
            ResolutionPolicy::ReportAll,
        )
        .expect("resolution succeeds");
        assert_eq!(report.candidates()[0].span(), Span::new(1, 3).unwrap());
        assert_eq!(report.candidates()[1].span(), Span::new(3, 4).unwrap());
    }
}
