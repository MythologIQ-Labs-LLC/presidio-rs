use presidio::{
    AnalysisRequest, AnalyzerEngine, DocumentId, ResolutionOptions, ResolutionPolicy,
    ResolvedFinding, TextDocument,
};

#[test]
fn downstream_consumer_can_analyze_validate_and_resolve() {
    let document = TextDocument::new(
        DocumentId::new("downstream-example").expect("valid document ID"),
        "Email jane@example.com or call 202-555-0142.",
    );
    let analysis = AnalyzerEngine::new()
        .analyze_request(&document, &AnalysisRequest::new())
        .expect("bounded analysis");

    let integrated = analysis
        .resolve_for_document(
            &document,
            &ResolutionOptions::new(ResolutionPolicy::ConservativeRedaction),
        )
        .expect("document-bound resolution");

    assert_eq!(integrated.document_binding(), document.binding());
    assert_eq!(integrated.engine_version(), env!("CARGO_PKG_VERSION"));
    assert!(integrated.resolution().status().output_complete());
    assert_eq!(
        integrated.resolution().candidates(),
        analysis.candidates()
    );
    assert!(integrated
        .resolution()
        .resolved()
        .iter()
        .all(|finding| matches!(finding, ResolvedFinding::Union { .. })));
}

#[test]
fn downstream_consumer_retains_raw_evidence_across_policies() {
    let document = TextDocument::new(
        DocumentId::new("downstream-example").expect("valid document ID"),
        "Email jane@example.com or call 202-555-0142.",
    );
    let analysis = AnalyzerEngine::new()
        .analyze_request(&document, &AnalysisRequest::new())
        .expect("bounded analysis");
    let original_candidates = analysis.candidates().to_vec();

    for policy in [
        ResolutionPolicy::ReportAll,
        ResolutionPolicy::BestCandidate,
        ResolutionPolicy::ConservativeRedaction,
    ] {
        let integrated = analysis
            .resolve_for_document(&document, &ResolutionOptions::new(policy))
            .expect("resolution succeeds");
        assert_eq!(integrated.resolution().candidates(), original_candidates);
    }

    assert_eq!(analysis.candidates(), original_candidates);
}
