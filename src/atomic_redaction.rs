//! Fail-closed, source-bound redaction over a complete resolved analysis.
//!
//! No partial transformed text is returned on failure. This is distinct from
//! the legacy best-effort anonymizer and deliberately supports redaction only.

use core::fmt;

use crate::document::{DocumentBindingError, TextDocument};
use crate::report_resolution::ResolvedAnalysisReport;
use crate::resolution::{ResolutionPolicy, ResolvedFinding};
use crate::types::SpanError;

/// Failure to construct complete redacted output.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AtomicRedactionError {
    /// The source identity, byte count, or fingerprint changed.
    Document(DocumentBindingError),
    /// Only conservative union coverage is accepted for fail-closed redaction.
    UnsupportedPolicy,
    /// Expected a union output from the conservative resolution policy.
    UnexpectedOutput,
    /// A resolved span is invalid for the original UTF-8 source.
    InvalidSpan(SpanError),
    /// Outputs are overlapping or not monotonically ordered.
    OverlappingOutputs,
}

impl fmt::Display for AtomicRedactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Document(error) => write!(f, "document mismatch: {error}"),
            Self::UnsupportedPolicy => f.write_str("conservative redaction policy required"),
            Self::UnexpectedOutput => f.write_str("unexpected resolved output shape"),
            Self::InvalidSpan(error) => write!(f, "invalid resolved span: {error}"),
            Self::OverlappingOutputs => {
                f.write_str("resolved output ranges overlap or are unordered")
            }
        }
    }
}

impl std::error::Error for AtomicRedactionError {}

/// Produce complete redacted UTF-8 text from source-validated conservative
/// resolution. Every output is validated before creating any transformed text.
///
/// The caller must create the report using `AnalysisReport::resolve_for_document`
/// and `ResolutionPolicy::ConservativeRedaction`. Missing detections are not
/// discoverable here; detection coverage is a separate assurance concern.
pub fn redact_resolved_document(
    document: &TextDocument<'_>,
    resolved: &ResolvedAnalysisReport,
) -> Result<String, AtomicRedactionError> {
    resolved
        .document_binding()
        .validate_document(document)
        .map_err(AtomicRedactionError::Document)?;
    if resolved.resolution().policy() != ResolutionPolicy::ConservativeRedaction {
        return Err(AtomicRedactionError::UnsupportedPolicy);
    }

    let source = document.original();
    let mut previous_end = 0;
    // Prevalidate all ranges before building the output.
    for output in resolved.resolution().resolved() {
        let span = match output {
            ResolvedFinding::Union { span, .. } => *span,
            _ => return Err(AtomicRedactionError::UnexpectedOutput),
        };
        span.validate_for(source)
            .map_err(AtomicRedactionError::InvalidSpan)?;
        if span.start() < previous_end {
            return Err(AtomicRedactionError::OverlappingOutputs);
        }
        previous_end = span.end();
    }

    let mut redacted = String::with_capacity(source.len());
    let mut cursor = 0;
    for output in resolved.resolution().resolved() {
        if let ResolvedFinding::Union { span, .. } = output {
            redacted.push_str(&source[cursor..span.start()]);
            redacted.push_str("[REDACTED]");
            cursor = span.end();
        }
    }
    redacted.push_str(&source[cursor..]);
    Ok(redacted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AnalysisRequest, AnalyzerEngine, DocumentId, ResolutionOptions};

    #[test]
    fn redacts_validated_multibyte_source() {
        let source = "Café jane@example.com and 202-555-0142";
        let doc = TextDocument::new(DocumentId::new("source").unwrap(), source);
        let report = AnalyzerEngine::new()
            .analyze_request(&doc, &AnalysisRequest::new())
            .unwrap();
        let resolved = report
            .resolve_for_document(
                &doc,
                &ResolutionOptions::new(ResolutionPolicy::ConservativeRedaction),
            )
            .unwrap();
        let output = redact_resolved_document(&doc, &resolved).unwrap();
        assert!(!output.contains("jane@example.com"));
        assert!(output.contains("Café"));
        assert!(output.contains("[REDACTED]"));
    }

    #[test]
    fn rejects_different_document_without_partial_output() {
        let doc = TextDocument::new(DocumentId::new("source").unwrap(), "jane@example.com");
        let report = AnalyzerEngine::new()
            .analyze_request(&doc, &AnalysisRequest::new())
            .unwrap();
        let resolved = report
            .resolve_for_document(
                &doc,
                &ResolutionOptions::new(ResolutionPolicy::ConservativeRedaction),
            )
            .unwrap();
        let other = TextDocument::new(
            DocumentId::new("source").unwrap(),
            "mary@example.com",
        );
        assert!(matches!(
            redact_resolved_document(&other, &resolved),
            Err(AtomicRedactionError::Document(_))
        ));
    }

    #[test]
    fn rejects_report_all_policy() {
        let doc = TextDocument::new(DocumentId::new("source").unwrap(), "jane@example.com");
        let report = AnalyzerEngine::new()
            .analyze_request(&doc, &AnalysisRequest::new())
            .unwrap();
        let resolved = report
            .resolve_for_document(
                &doc,
                &ResolutionOptions::new(ResolutionPolicy::ReportAll),
            )
            .unwrap();
        assert_eq!(
            redact_resolved_document(&doc, &resolved),
            Err(AtomicRedactionError::UnsupportedPolicy)
        );
    }
}
