use presidio::{
    AnalyzerEngine, DocumentBindingError, DocumentId, FindingDocumentError, ReportDocumentError,
    TextDocument,
};

fn document<'a>(id: &str, text: &'a str) -> TextDocument<'a> {
    TextDocument::new(DocumentId::new(id).expect("valid document ID"), text)
}

#[test]
fn document_aware_reports_bind_candidates_to_exact_source() {
    let source = document("request-42", "email jane@example.com");
    let report = AnalyzerEngine::new().analyze_document(&source, None);

    report
        .validate_for_document(&source)
        .expect("report must match source");
    assert_eq!(report.document_binding(), Some(source.binding()));

    let email = report
        .candidates()
        .iter()
        .find(|finding| finding.entity().as_str() == "EMAIL_ADDRESS")
        .expect("email finding");
    assert_eq!(email.document_binding(), Some(source.binding()));
    assert_eq!(
        email.slice_document(&source).expect("bound source slice"),
        "jane@example.com"
    );
}

#[test]
fn reused_id_with_changed_same_length_content_is_rejected() {
    let original = document("request-42", "email jane@example.com");
    let changed = document("request-42", "email john@example.com");
    let report = AnalyzerEngine::new().analyze_document(&original, None);
    let finding = report.candidates().first().expect("email finding");

    assert!(matches!(
        report.validate_for_document(&changed),
        Err(ReportDocumentError::Document(
            DocumentBindingError::FingerprintMismatch
        ))
    ));
    assert!(matches!(
        finding.slice_document(&changed),
        Err(FindingDocumentError::Document(
            DocumentBindingError::FingerprintMismatch
        ))
    ));
}

#[test]
fn matching_content_with_different_document_id_is_rejected() {
    let first = document("request-42", "email jane@example.com");
    let second = document("request-43", "email jane@example.com");
    let report = AnalyzerEngine::new().analyze_document(&first, None);

    assert!(matches!(
        report.validate_for_document(&second),
        Err(ReportDocumentError::Document(
            DocumentBindingError::IdMismatch { .. }
        ))
    ));
}

#[test]
fn changed_length_is_rejected_before_fingerprint_comparison() {
    let first = document("request-42", "email jane@example.com");
    let shorter = document("request-42", "email a@b.co");
    let report = AnalyzerEngine::new().analyze_document(&first, None);

    assert!(matches!(
        report.validate_for_document(&shorter),
        Err(ReportDocumentError::Document(
            DocumentBindingError::LengthMismatch { .. }
        ))
    ));
}

#[test]
fn string_only_reports_and_legacy_findings_remain_explicitly_unbound() {
    let source = document("request-42", "email jane@example.com");
    let report = AnalyzerEngine::new().analyze_report(source.original(), None);
    let finding = report.candidates().first().expect("email finding");

    assert_eq!(report.document_binding(), None);
    assert_eq!(finding.document_binding(), None);
    assert_eq!(
        report.validate_for_document(&source),
        Err(ReportDocumentError::UnboundReport)
    );
    assert_eq!(
        finding.slice_document(&source),
        Err(FindingDocumentError::UnboundFinding)
    );
}

#[test]
fn original_utf8_offsets_remain_valid_for_document_slicing() {
    let source = document("unicode-1", "préfixe jane@example.com suffixe");
    let report = AnalyzerEngine::new().analyze_document(&source, None);
    let email = report
        .candidates()
        .iter()
        .find(|finding| finding.entity().as_str() == "EMAIL_ADDRESS")
        .expect("email finding");

    assert_eq!(
        email.slice_document(&source).expect("UTF-8-safe slice"),
        "jane@example.com"
    );
}

#[test]
fn text_document_debug_output_omits_original_plaintext() {
    let source = document("request-42", "email jane@example.com");
    let rendered = format!("{source:?}");

    assert!(rendered.contains("request-42"));
    assert!(!rendered.contains("jane@example.com"));
    assert!(!rendered.contains("email "));
}
