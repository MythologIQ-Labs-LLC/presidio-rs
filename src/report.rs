//! Report-oriented analysis results and explicit resolution policies.

use core::cmp::Ordering;
use core::fmt;

use crate::types::{Confidence, ConfidenceError, Finding, RecognizerId, SpanError};

/// A report containing every threshold-qualified candidate before resolution.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisReport {
    threshold: Confidence,
    candidates: Vec<Finding>,
    issues: Vec<AnalysisIssue>,
}

impl AnalysisReport {
    pub(crate) fn new(
        threshold: Confidence,
        mut candidates: Vec<Finding>,
        issues: Vec<AnalysisIssue>,
    ) -> Self {
        sort_findings(&mut candidates);
        Self {
            threshold,
            candidates,
            issues,
        }
    }

    /// Score threshold applied before candidates entered this report.
    pub const fn threshold(&self) -> Confidence {
        self.threshold
    }

    /// Every valid, threshold-qualified finding before overlap resolution.
    pub fn candidates(&self) -> &[Finding] {
        &self.candidates
    }

    /// Non-fatal candidate construction issues encountered during analysis.
    pub fn issues(&self) -> &[AnalysisIssue] {
        &self.issues
    }

    /// Whether analysis completed without non-fatal issues.
    pub fn is_clean(&self) -> bool {
        self.issues.is_empty()
    }

    /// Resolve report candidates using an explicit policy.
    pub fn resolve(&self, policy: ResolutionPolicy) -> Vec<Finding> {
        let mut findings = self.candidates.clone();
        sort_findings(&mut findings);
        match policy {
            ResolutionPolicy::AllCandidates => findings,
            ResolutionPolicy::HighestConfidenceNonOverlapping => {
                highest_confidence_non_overlapping(findings)
            }
        }
    }
}

/// Explicit policy for converting unresolved candidates into selected findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResolutionPolicy {
    /// Return all threshold-qualified candidates, including overlaps.
    AllCandidates,
    /// Keep the highest-confidence candidate among spans encountered as overlapping.
    HighestConfidenceNonOverlapping,
}

/// Fatal failure to construct a report.
#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisError {
    /// The analyzer's configured score threshold is invalid.
    InvalidThreshold(ConfidenceError),
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidThreshold(error) => write!(f, "invalid analysis threshold: {error}"),
        }
    }
}

impl std::error::Error for AnalysisError {}

/// Non-fatal issue encountered while constructing a candidate finding.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisIssue {
    recognizer: RecognizerId,
    pattern_id: String,
    reason: CandidateIssue,
}

impl AnalysisIssue {
    pub(crate) fn new(
        recognizer: RecognizerId,
        pattern_id: impl Into<String>,
        reason: CandidateIssue,
    ) -> Self {
        Self {
            recognizer,
            pattern_id: pattern_id.into(),
            reason,
        }
    }

    pub fn recognizer(&self) -> &RecognizerId {
        &self.recognizer
    }

    pub fn pattern_id(&self) -> &str {
        &self.pattern_id
    }

    pub fn reason(&self) -> &CandidateIssue {
        &self.reason
    }
}

/// Reason a candidate could not be represented safely.
#[derive(Debug, Clone, PartialEq)]
pub enum CandidateIssue {
    InvalidSpan(SpanError),
    InvalidConfidence(ConfidenceError),
}

fn sort_findings(findings: &mut [Finding]) {
    findings.sort_by(|left, right| {
        left.span()
            .start()
            .cmp(&right.span().start())
            .then_with(|| {
                right
                    .confidence()
                    .get()
                    .partial_cmp(&left.confidence().get())
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| right.span().end().cmp(&left.span().end()))
            .then_with(|| left.entity().as_str().cmp(right.entity().as_str()))
            .then_with(|| left.recognizer().as_str().cmp(right.recognizer().as_str()))
    });
}

fn highest_confidence_non_overlapping(findings: Vec<Finding>) -> Vec<Finding> {
    let mut kept: Vec<Finding> = Vec::with_capacity(findings.len());
    for finding in findings {
        match kept.last_mut() {
            Some(last) if finding.span().start() < last.span().end() => {
                if finding.confidence().get() > last.confidence().get() {
                    *last = finding;
                }
            }
            _ => kept.push(finding),
        }
    }
    kept
}
