use presidio::{
    AnalysisError, AnalyzerEngine, CandidateIssue, ConfidenceError, EntityType, Evidence, Pattern,
    PatternRecognizer, RecognizerId, RecognizerMetadata, RecognizerRegistry, RegistryError,
    ResolutionPolicy,
};

fn metadata(id: &str, version: &str) -> RecognizerMetadata {
    RecognizerMetadata::new(RecognizerId::new(id).expect("valid test recognizer ID"))
        .with_version(version)
}

fn recognizer(
    entity_type: EntityType,
    name: &'static str,
    regex: &str,
    score: f32,
) -> PatternRecognizer {
    PatternRecognizer {
        entity_type,
        patterns: vec![Pattern::new(name, regex, score)],
        context: &[],
        validator: None,
    }
}

#[test]
fn report_preserves_overlapping_candidates_until_resolution() {
    let mut registry = RecognizerRegistry::empty();
    registry
        .add_with_metadata(
            recognizer(EntityType::ApiKey, "long", r"ABC123", 0.8),
            metadata("test.long", "1"),
        )
        .unwrap();
    registry
        .add_with_metadata(
            recognizer(EntityType::Url, "short", r"ABC", 0.9),
            metadata("test.short", "2"),
        )
        .unwrap();

    let analyzer = AnalyzerEngine::with_registry(registry);
    let report = analyzer.analyze_report("ABC123", None).unwrap();

    assert_eq!(report.candidates().len(), 2);
    assert_eq!(report.resolve(ResolutionPolicy::AllCandidates).len(), 2);
    assert_eq!(
        report
            .resolve(ResolutionPolicy::HighestConfidenceNonOverlapping)
            .len(),
        1
    );
    assert_eq!(analyzer.analyze("ABC123", None).len(), 1);
}

#[test]
fn report_carries_recognizer_and_pattern_provenance() {
    let mut registry = RecognizerRegistry::empty();
    registry
        .add_with_metadata(
            recognizer(
                EntityType::Email,
                "customer-email",
                r"[a-z]+@[a-z]+\.[a-z]+",
                0.8,
            ),
            metadata("customer.email", "2026.07"),
        )
        .unwrap();

    let report = AnalyzerEngine::with_registry(registry)
        .analyze_report("reach jane@example.com", None)
        .unwrap();
    let finding = report.candidates().first().unwrap();

    assert_eq!(finding.recognizer().as_str(), "customer.email");
    assert_eq!(finding.recognizer_version(), Some("2026.07"));
    assert_eq!(
        finding.evidence(),
        &[Evidence::Pattern {
            pattern_id: "customer-email".to_owned(),
        }]
    );
}

#[test]
fn invalid_candidate_scores_become_non_fatal_issues() {
    let mut registry = RecognizerRegistry::empty();
    registry
        .add_with_metadata(
            recognizer(EntityType::ApiKey, "invalid-score", r"SECRET", 1.5),
            metadata("test.invalid-score", "1"),
        )
        .unwrap();

    let report = AnalyzerEngine::with_registry(registry)
        .with_threshold(0.0)
        .analyze_report("SECRET", None)
        .unwrap();

    assert!(report.candidates().is_empty());
    assert_eq!(report.issues().len(), 1);
    assert!(matches!(
        report.issues()[0].reason(),
        CandidateIssue::InvalidConfidence(ConfidenceError::OutOfRange { value })
            if (*value - 1.5).abs() < f32::EPSILON
    ));
}

#[test]
fn invalid_threshold_is_fatal() {
    let error = AnalyzerEngine::new()
        .with_threshold(f32::NAN)
        .analyze_report("text", None)
        .unwrap_err();
    assert_eq!(
        error,
        AnalysisError::InvalidThreshold(ConfidenceError::NotFinite)
    );
}

#[test]
fn duplicate_explicit_recognizer_ids_are_rejected() {
    let mut registry = RecognizerRegistry::empty();
    registry
        .add_with_metadata(
            recognizer(EntityType::Email, "first", r"FIRST", 0.8),
            metadata("duplicate.id", "1"),
        )
        .unwrap();

    let error = registry
        .add_with_metadata(
            recognizer(EntityType::Url, "second", r"SECOND", 0.8),
            metadata("duplicate.id", "2"),
        )
        .unwrap_err();

    assert_eq!(
        error,
        RegistryError::DuplicateRecognizerId {
            id: RecognizerId::new("duplicate.id").unwrap(),
        }
    );
}
