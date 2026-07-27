use presidio::{
    AnalyzerEngine, Confidence, ConfidenceError, EntityId, Evidence, Finding, IdentifierError,
    RecognizerId, Span, SpanError,
};

#[test]
fn legacy_analyzer_results_convert_to_validated_findings() {
    let analyzer = AnalyzerEngine::new();
    let text = "contact jane@example.com";
    let result = analyzer
        .analyze(text, None)
        .into_iter()
        .find(|result| result.entity_type.as_tag() == "EMAIL_ADDRESS")
        .expect("email result");

    let finding = Finding::try_from(&result).expect("validated finding");

    assert_eq!(finding.entity().as_str(), "EMAIL_ADDRESS");
    assert_eq!(finding.span().slice(text), Ok("jane@example.com"));
    assert_eq!(finding.recognizer().as_str(), "legacy.pattern");
    assert_eq!(finding.evidence(), &[Evidence::LegacyResult]);
}

#[test]
fn public_value_types_enforce_invariants() {
    assert_eq!(Span::new(2, 2), Err(SpanError::Empty { offset: 2 }));
    assert_eq!(
        Confidence::new(f32::INFINITY),
        Err(ConfidenceError::NotFinite)
    );
    assert_eq!(EntityId::new(""), Err(IdentifierError::Empty));
    assert!(RecognizerId::new("acme.email:v1").is_ok());
}
