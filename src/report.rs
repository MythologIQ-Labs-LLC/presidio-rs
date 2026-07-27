//! Additive, bounded report contract for candidate-preserving analysis.

use core::fmt;

use crate::document::{DocumentBinding, DocumentBindingError, FindingDocumentError, TextDocument};
use crate::metadata::RecognizerMetadata;
use crate::recognition::RecognitionError;
use crate::types::{RecognizerId, Span, SpanError};
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

/// A typed, non-plaintext problem encountered during analysis.
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
    /// A backend-neutral recognizer returned a typed execution failure.
    RecognitionFailed {
        recognizer: RecognizerId,
        error: RecognitionError,
    },
    /// Legacy pattern registrations were skipped by the authoritative request path.
    LegacyRecognizersSkipped { count: usize },
}

/// Whether report processing reached configured limits or compatibility boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AnalysisStatus {
    candidate_limit_reached: bool,
    issue_limit_reached: bool,
    legacy_projection_incomplete: bool,
}

impl AnalysisStatus {
    pub(crate) const fn new(
        candidate_limit_reached: bool,
        issue_limit_reached: bool,
        legacy_projection_incomplete: bool,
    ) -> Self {
        Self {
            candidate_limit_reached,
            issue_limit_reached,
            legacy_projection_incomplete,
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

    /// Whether open entity identifiers prevented a complete legacy projection.
    pub const fn legacy_projection_incomplete(self) -> bool {
        self.legacy_projection_incomplete
    }

    /// Whether either bounded report collection reached a configured limit.
    pub const fn was_truncated(self) -> bool {
        self.candidate_limit_reached || self.issue_limit_reached
    }
}

/// Candidate-preserving analysis output.
///
/// `candidates` contains validated findings before thresholding or overlap
/// resolution. `legacy_compatible_results` applies the existing analyzer policy
/// where open entity identifiers can be represented by the legacy taxonomy.
/// `recognizers` contains authoritative metadata for represented recognizers.
/// Document-aware analysis binds the report and each finding to exact source bytes.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AnalysisReport {
    engine_version: &'static str,
    document: Option<DocumentBinding>,
    candidates: Vec<Finding>,
    recognizers: Vec<RecognizerMetadata>,
    legacy_compatible_results: Vec<RecognizerResult>,
    issues: Vec<AnalysisIssue>,
    status: AnalysisStatus,
}

impl AnalysisReport {
    pub(crate) fn new(
        engine_version: &'static str,
        document: Option<DocumentBinding>,
        candidates: Vec<Finding>,
        recognizers: Vec<RecognizerMetadata>,
        legacy_compatible_results: Vec<RecognizerResult>,
        issues: Vec<AnalysisIssue>,
        status: AnalysisStatus,
    ) -> Self {
        Self {
            engine_version,
            document,
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

    /// Exact source binding, when produced through document-aware analysis.
    pub fn document_binding(&self) -> Option<&DocumentBinding> {
        self.document.as_ref()
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

    /// Legacy-compatible projection from the same bounded candidate stream.
    pub fn legacy_compatible_results(&self) -> &[RecognizerResult] {
        &self.legacy_compatible_results
    }

    /// Typed non-fatal analysis issues.
    pub fn issues(&self) -> &[AnalysisIssue] {
        &self.issues
    }

    /// Whether at least one detailed issue was retained.
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }

    /// Resource and compatibility status for this analysis.
    pub const fn status(&self) -> AnalysisStatus {
        self.status
    }

    /// Validate report identity, candidate bindings, and legacy spans for a document.
    pub fn validate_for_document(
        &self,
        document: &TextDocument<'_>,
    ) -> Result<(), ReportDocumentError> {
        let binding = self
            .document
            .as_ref()
            .ok_or(ReportDocumentError::UnboundReport)?;
        binding
            .validate_document(document)
            .map_err(ReportDocumentError::Document)?;

        for (index, finding) in self.candidates.iter().enumerate() {
            finding
                .validate_for_document(document)
                .map_err(|error| ReportDocumentError::Candidate { index, error })?;
        }

        for (index, result) in self.legacy_compatible_results.iter().enumerate() {
            Span::new_for(document.original(), result.start, result.end)
                .map_err(|error| ReportDocumentError::LegacySpan { index, error })?;
        }

        Ok(())
    }
}

/// Failure to validate an analysis report against a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportDocumentError {
    /// The report came from a legacy string-only analysis path.
    UnboundReport,
    /// The supplied document does not match the report's source binding.
    Document(DocumentBindingError),
    /// One candidate is missing or violates the report's source binding.
    Candidate {
        index: usize,
        error: FindingDocumentError,
    },
    /// One legacy-compatible span cannot safely index the bound document.
    LegacySpan { index: usize, error: SpanError },
}

impl fmt::Display for ReportDocumentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnboundReport => {
                formatter.write_str("analysis report is not bound to a document")
            }
            Self::Document(error) => write!(formatter, "report document mismatch: {error}"),
            Self::Candidate { index, error } => {
                write!(
                    formatter,
                    "candidate {index} does not match document: {error}"
                )
            }
            Self::LegacySpan { index, error } => {
                write!(
                    formatter,
                    "legacy result {index} has invalid document span: {error}"
                )
            }
        }
    }
}

impl std::error::Error for ReportDocumentError {}
