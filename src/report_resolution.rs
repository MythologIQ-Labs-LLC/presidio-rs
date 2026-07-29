//! Additive integration between document-aware analysis and pure resolution.

use core::fmt;

use crate::document::{DocumentBinding, TextDocument};
use crate::report::{AnalysisReport, AnalysisStatus, ReportDocumentError};
use crate::resolution::{resolve_candidates, ResolutionError, ResolutionOptions, ResolutionReport};

/// Document-validated analysis context paired with one resolution report.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ResolvedAnalysisReport {
    engine_version: &'static str,
    document: DocumentBinding,
    analysis_status: AnalysisStatus,
    analysis_issue_count: usize,
    resolution: ResolutionReport,
}

impl ResolvedAnalysisReport {
    /// Analyzer engine version that produced the source candidate collection.
    pub const fn engine_version(&self) -> &'static str {
        self.engine_version
    }

    /// Exact source binding validated before resolution.
    pub fn document_binding(&self) -> &DocumentBinding {
        &self.document
    }

    /// Resource and compatibility status from the source analysis.
    pub const fn analysis_status(&self) -> AnalysisStatus {
        self.analysis_status
    }

    /// Number of retained non-fatal analysis issues.
    pub const fn analysis_issue_count(&self) -> usize {
        self.analysis_issue_count
    }

    /// Pure versioned resolution output.
    pub fn resolution(&self) -> &ResolutionReport {
        &self.resolution
    }

    /// Consume the integration wrapper and return the pure resolution report.
    pub fn into_resolution(self) -> ResolutionReport {
        self.resolution
    }
}

/// Failure to resolve an analysis report as an authoritative document-bound result.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AnalysisResolutionError {
    /// The report is unbound, mismatched, or contains a source-invalid span.
    ReportDocument(ReportDocumentError),
    /// Candidate collection stopped at its configured limit.
    IncompleteCandidateCollection,
    /// Pure resolution failed its configured contract.
    Resolution(ResolutionError),
}

impl fmt::Display for AnalysisResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReportDocument(error) => write!(formatter, "analysis report is not usable: {error}"),
            Self::IncompleteCandidateCollection => formatter.write_str(
                "analysis candidate collection is incomplete and cannot be authoritatively resolved",
            ),
            Self::Resolution(error) => write!(formatter, "candidate resolution failed: {error}"),
        }
    }
}

impl std::error::Error for AnalysisResolutionError {}

impl From<ReportDocumentError> for AnalysisResolutionError {
    fn from(value: ReportDocumentError) -> Self {
        Self::ReportDocument(value)
    }
}

impl From<ResolutionError> for AnalysisResolutionError {
    fn from(value: ResolutionError) -> Self {
        Self::Resolution(value)
    }
}

impl AnalysisReport {
    /// Validate this report against an exact document and resolve its raw candidates.
    ///
    /// Resolution is refused when candidate collection reached its configured
    /// limit. Detailed issue truncation remains visible through
    /// [`ResolvedAnalysisReport::analysis_status`], but does not silently alter
    /// the complete candidate collection.
    pub fn resolve_for_document(
        &self,
        document: &TextDocument<'_>,
        options: &ResolutionOptions,
    ) -> Result<ResolvedAnalysisReport, AnalysisResolutionError> {
        self.validate_for_document(document)?;
        if self.status().candidate_limit_reached() {
            return Err(AnalysisResolutionError::IncompleteCandidateCollection);
        }

        let resolution = resolve_candidates(self.candidates(), options)?;
        Ok(ResolvedAnalysisReport {
            engine_version: self.engine_version(),
            document: document.binding().clone(),
            analysis_status: self.status(),
            analysis_issue_count: self.issues().len(),
            resolution,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AnalysisRequest, AnalyzerEngine, DocumentId, EntityId, ResolutionPolicy, TextDocument,
    };

    #[test]
    fn request_report_resolves_for_the_exact_document() {
        let document = TextDocument::new(
            DocumentId::new("doc-1").unwrap(),
            "Email jane@example.com and call 202-555-0142.",
        );
        let analysis = AnalyzerEngine::new()
            .analyze_request(&document, &AnalysisRequest::new())
            .expect("analysis succeeds");
        let resolved = analysis
            .resolve_for_document(
                &document,
                &ResolutionOptions::new(ResolutionPolicy::ConservativeRedaction),
            )
            .expect("resolution succeeds");

        assert_eq!(resolved.engine_version(), env!("CARGO_PKG_VERSION"));
        assert_eq!(resolved.document_binding(), document.binding());
        assert_eq!(
            resolved.resolution().document_binding(),
            Some(document.binding())
        );
        assert_eq!(resolved.resolution().candidates(), analysis.candidates());
    }

    #[test]
    fn mismatched_document_is_rejected_before_resolution() {
        let original = TextDocument::new(DocumentId::new("doc-1").unwrap(), "Email a@b.com");
        let other = TextDocument::new(DocumentId::new("doc-2").unwrap(), "Email a@b.com");
        let analysis = AnalyzerEngine::new()
            .analyze_request(&original, &AnalysisRequest::new())
            .expect("analysis succeeds");

        assert!(matches!(
            analysis
                .resolve_for_document(&other, &ResolutionOptions::new(ResolutionPolicy::ReportAll)),
            Err(AnalysisResolutionError::ReportDocument(_))
        ));
    }

    #[test]
    fn unbound_legacy_report_cannot_masquerade_as_authoritative() {
        let document = TextDocument::new(DocumentId::new("doc-1").unwrap(), "Email a@b.com");
        let analysis = AnalyzerEngine::new().analyze_report(document.original(), None);

        assert_eq!(
            analysis.resolve_for_document(
                &document,
                &ResolutionOptions::new(ResolutionPolicy::ReportAll)
            ),
            Err(AnalysisResolutionError::ReportDocument(
                ReportDocumentError::UnboundReport
            ))
        );
    }

    #[test]
    fn truncated_candidate_collection_is_rejected() {
        let document =
            TextDocument::new(DocumentId::new("doc-1").unwrap(), "a@b.com c@d.com e@f.com");
        let request = AnalysisRequest::new().with_max_candidates(1).unwrap();
        let analysis = AnalyzerEngine::new()
            .analyze_request(&document, &request)
            .expect("bounded analysis succeeds");
        assert!(analysis.status().candidate_limit_reached());

        assert_eq!(
            analysis.resolve_for_document(
                &document,
                &ResolutionOptions::new(ResolutionPolicy::ReportAll)
            ),
            Err(AnalysisResolutionError::IncompleteCandidateCollection)
        );
    }

    #[test]
    fn repeated_resolution_preserves_raw_candidates_and_legacy_projection() {
        let document = TextDocument::new(
            DocumentId::new("doc-1").unwrap(),
            "Email jane@example.com and call 202-555-0142.",
        );
        let analyzer = AnalyzerEngine::new();
        let analysis = analyzer
            .analyze_request(&document, &AnalysisRequest::new())
            .expect("analysis succeeds");
        let candidates = analysis.candidates().to_vec();
        let legacy = analysis.legacy_compatible_results().to_vec();
        let options = ResolutionOptions::new(ResolutionPolicy::BestCandidate);

        let first = analysis
            .resolve_for_document(&document, &options)
            .expect("first resolution succeeds");
        let second = analysis
            .resolve_for_document(&document, &options)
            .expect("second resolution succeeds");

        assert_eq!(first, second);
        assert_eq!(analysis.candidates(), candidates);
        assert_eq!(analysis.legacy_compatible_results(), legacy);
    }

    #[test]
    fn open_entities_remain_supported_through_report_integration() {
        struct OpenEntityBackend {
            metadata: crate::RecognizerMetadata,
        }

        impl crate::Recognizer for OpenEntityBackend {
            fn metadata(&self) -> &crate::RecognizerMetadata {
                &self.metadata
            }

            fn recognize(
                &self,
                document: &TextDocument<'_>,
                _request: &AnalysisRequest,
                emitter: &mut crate::CandidateEmitter<'_, '_>,
            ) -> Result<(), crate::RecognitionError> {
                emitter
                    .emit(
                        EntityId::new("CUSTOM_SECRET").unwrap(),
                        crate::Span::new_for(document.original(), 0, 4).unwrap(),
                        crate::Confidence::new(0.9).unwrap(),
                        [],
                    )
                    .expect("candidate accepted");
                Ok(())
            }
        }

        let metadata = crate::RecognizerMetadata::builder(
            crate::RecognizerId::new("custom.open").unwrap(),
            "1",
            crate::RecognitionMechanism::Custom,
        )
        .with_supported_entities([EntityId::new("CUSTOM_SECRET").unwrap()])
        .build()
        .unwrap();
        let mut analyzer = AnalyzerEngine::new();
        analyzer
            .add_backend(OpenEntityBackend { metadata })
            .expect("backend registration");
        let document = TextDocument::new(DocumentId::new("doc-1").unwrap(), "ABCD");
        let request =
            AnalysisRequest::new().with_entities([EntityId::new("CUSTOM_SECRET").unwrap()]);
        let analysis = analyzer
            .analyze_request(&document, &request)
            .expect("analysis succeeds");
        let resolved = analysis
            .resolve_for_document(
                &document,
                &ResolutionOptions::new(ResolutionPolicy::ReportAll),
            )
            .expect("resolution succeeds");

        assert_eq!(
            resolved.resolution().candidates()[0].entity().as_str(),
            "CUSTOM_SECRET"
        );
    }
}
