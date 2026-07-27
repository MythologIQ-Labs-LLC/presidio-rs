use presidio::{
    AnalyzerEngine, Confidence, ConfidenceError, EntityId, Evidence, Finding, IdentifierError,
    MetadataId, RecognizerId, RecognizerResult, Span, SpanError,
};

#[test]
fn legacy_analyzer_results_convert_without_invented_provenance() {
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
    assert_eq!(finding.recognizer(), None);
    assert_eq!(finding.evidence(), &[Evidence::LegacyResult]);
}

#[test]
fn public_value_types_enforce_structural_and_source_invariants() {
    assert_eq!(Span::new(2, 2), Err(SpanError::Empty { offset: 2 }));
    assert_eq!(
        Span::new_for("aéz", 1, 2),
        Err(SpanError::NotCharBoundary { offset: 2 })
    );
    assert_eq!(
        Span::new(1, 4).expect("structural span").validate_for("x"),
        Err(SpanError::OutOfBounds {
            start: 1,
            end: 4,
            text_len: 1,
        })
    );
    assert_eq!(
        Confidence::new(f32::INFINITY),
        Err(ConfidenceError::NotFinite)
    );
}

#[test]
fn metadata_and_public_identifiers_are_bounded() {
    assert_eq!(EntityId::new(""), Err(IdentifierError::Empty));
    assert!(RecognizerId::new("acme.email:v1").is_ok());
    assert!(MetadataId::new("pattern.email:v1").is_ok());
    assert!(MetadataId::new("matched jane@example.com").is_err());
    assert!(matches!(
        MetadataId::new("x".repeat(129)),
        Err(IdentifierError::TooLong { .. })
    ));
}

#[test]
fn legacy_conversion_rejects_invalid_raw_values() {
    let reversed = RecognizerResult::new(presidio::EntityType::Email, 5, 2, 0.7);
    assert!(Finding::try_from(&reversed).is_err());

    let invalid_score = RecognizerResult::new(presidio::EntityType::Email, 0, 3, f32::NAN);
    assert!(Finding::try_from(&invalid_score).is_err());
}
