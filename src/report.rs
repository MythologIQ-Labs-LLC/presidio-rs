//! Additive, bounded report contract for candidate-preserving analysis.

use crate::metadata::RecognizerMetadata;
use crate::types::RecognizerId;
use crate::{Finding, RecognizerResult};

/// Default maximum number of accepted raw candidates processed by a report.
pub const DEFAULT_REPORT_CANDIDATE_LIMIT: usize = 10_000;
/// Default maximum number of detailed report-construction issues retained.
pub const DEFAULT_REPORT_ISSUE_LIMIT: usize = 100;

/// Resource limits for report-oriented analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalysisOptions {
    max_candidates: usize,
    max_issues: usize,
}

impl Default for AnalysisOptions {
    fn default() -> Self {
        Self {
            max_candidates: DEFAULT_REPORT_CANDIDATE_LIMIT,
            max_issues: DEFAULT_REPORT_ISSUE_LIMIT,
        }
    }
}

impl AnalysisOptions {
    /// Set the maximum accepted candidates processed before deterministic truncation.
    pub const fn with_max_candidates(mut self, maximum: usize) -> Self {
        self.max_candidates = maximum;
        self
    }

    /// Set the maximum detailed issues retained before deterministic truncation.
    pub const fn with_max_issues(mut self, maximum: usize) -> Self {
        self.max_issues = maximum;
        self
    }

    /// Maximum accepted candidates processed by the report path.
    pub const fn max_candidates(self) -> usize {
        self.max_candidates
    }

    /// Maximum detailed issues retained by the report path.
    pub const fn max_issues(self) -> usize {
        self.max_issues
    }
}

/// A typed, non-plaintext problem encountered while constructing validated findings.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub enum AnalysisIssue {
    /// A raw candidate did not produce a source-valid span.
    InvalidSpan {
        recognizer_index: usize,
        pattern_index: usize,
        start: usize,
        end: usize,
    },
    /// A raw candidate carried a non-finite or out-of-range confidence value.
    InvalidConfidence {
        recognizer_index: usize,
        pattern_index: usize,
        value: f32,
    },
    /// A legacy pattern name could not be represented as bounded evidence metadata.
    InvalidPatternMetadata {
        recognizer_index: usize,
        pattern_index: usize,
    },
}

/// Whether report processing reached configured resource limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AnalysisStatus {
    candidate_limit_reached: bool,
    issue_limit_reached: bool,
}

impl AnalysisStatus {
    pub(crate) const fn new(candidate_limit_reached: bool, issue_limit_reached: bool) -> Self {
        Self {
            candidate_limit_reached,
            issue_limit_reached,
        }
    }

    /// Whether candidate collection stopped at the configured limit.
    pub const fn candidate_limit_reached(self) -> bool {
        self.candidate_limit_reached
    }

    /// Whether additional issue details were suppressed at the configured limit.
    pub const fn issue_limit_reached(self) -> bool {
        self.issue_limit_reached
    }

    /// Whether either report collection reached a configured limit.
    pub const fn was_truncated(self) -> bool {
        self.candidate_limit_reached || self.issue_limit_reached
    }
}

/// Candidate-preserving analysis output.
///
/// `candidates` contains validated findings before thresholding or overlap
/// resolution. `legacy_compatible_results` applies the existing analyzer policy
/// to the same bounded raw candidate stream. `recognizers` contains authoritative
/// metadata only for metadata-backed recognizers that emitted raw candidates.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AnalysisReport {
    engine_version: &'static str,
    candidates: Vec<Finding>,
    recognizers: Vec<RecognizerMetadata>,
    legacy_compatible_results: Vec<RecognizerResult>,
    issues: Vec<AnalysisIssue>,
    status: AnalysisStatus,
}

impl AnalysisReport {
    pub(crate) fn new(
        engine_version: &'static str,
        candidates: Vec<Finding>,
        recognizers: Vec<RecognizerMetadata>,
        legacy_compatible_results: Vec<RecognizerResult>,
        issues: Vec<AnalysisIssue>,
        status: AnalysisStatus,
    ) -> Self {
        Self {
            engine_version,
            candidates,
            recognizers,
            legacy_compatible_results,
            issues,
            status,
        }
    }

    /// Version of the library engine that constructed this report.
    pub const fn engine_version(&self) -> &'static str {
        self.engine_version
    }

    /// Validated findings before thresholding or overlap resolution.
    pub fn candidates(&self) -> &[Finding] {
        &self.candidates
    }

    /// Metadata snapshots for authoritative recognizers represented in this report.
    pub fn recognizers(&self) -> &[RecognizerMetadata] {
        &self.recognizers
    }

    /// Find authoritative metadata by stable recognizer ID.
    pub fn recognizer_metadata(&self, id: &RecognizerId) -> Option<&RecognizerMetadata> {
        self.recognizers.iter().find(|metadata| metadata.id() == id)
    }

    /// Existing analyzer semantics projected from the same bounded raw candidates.
    pub fn legacy_compatible_results(&self) -> &[RecognizerResult] {
        &self.legacy_compatible_results
    }

    /// Typed non-fatal report-construction issues.
    pub fn issues(&self) -> &[AnalysisIssue] {
        &self.issues
    }

    /// Whether at least one detailed issue was retained.
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }

    /// Resource-limit status for this analysis.
    pub const fn status(&self) -> AnalysisStatus {
        self.status
    }
}
