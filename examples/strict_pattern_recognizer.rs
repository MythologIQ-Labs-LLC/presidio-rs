use presidio::{
    AnalysisRequest, AnalyzerEngine, DocumentId, EntityId, EntityType, MetadataId, Pattern,
    PatternRecognizer, RecognitionMechanism, RecognizerId, RecognizerMetadata, RecognizerRegistry,
    TextDocument,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = RecognizerMetadata::new(
        RecognizerId::new("example.internal-api-token")?,
        MetadataId::new("1.0.0")?,
        [EntityId::from(EntityType::ApiKey)],
        RecognitionMechanism::Pattern,
    )?
    .with_default_enabled(true)
    .with_attribution(MetadataId::new("example.repository-authored")?);

    let recognizer = PatternRecognizer {
        entity_type: EntityType::ApiKey,
        patterns: vec![Pattern::try_new(
            "internal-api-token",
            r"\bcorp_[A-Za-z0-9]{16}\b",
            0.9,
        )?],
        context: &["token", "credential"],
        validator: None,
    };

    let mut registry = RecognizerRegistry::empty();
    registry.add_with_metadata(metadata, recognizer)?;

    let document = TextDocument::new(
        DocumentId::new("strict-pattern-example")?,
        "Rotate token corp_A1B2C3D4E5F6G7H8 before deployment.",
    );
    let report = AnalyzerEngine::with_registry(registry)
        .analyze_request(&document, &AnalysisRequest::new())?;

    let finding = report
        .candidates()
        .first()
        .expect("the example token should be detected");

    assert_eq!(finding.entity().as_str(), "API_KEY");
    assert_eq!(finding.slice_document(&document)?, "corp_A1B2C3D4E5F6G7H8");
    assert_eq!(
        finding.recognizer().expect("strict provenance"),
        &RecognizerId::new("example.internal-api-token")?
    );

    Ok(())
}
