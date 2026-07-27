use presidio::{
    AnalysisIssue, AnalysisOptions, AnalyzerEngine, EntityType, Evidence, Pattern,
    PatternRecognizer, RecognizerRegistry,
};

fn registry_with(recognizer: PatternRecognizer) -> RecognizerRegistry {
    let mut registry = RecognizerRegistry::empty();
    registry.add(recognizer);
    registry
}

#[test]
fn report_preserves_exact_overlapping_candidates_before_legacy_resolution() {
    let registry = registry_with(PatternRecognizer {
        entity_type: EntityType::Email,
        patterns: vec![
            Pattern::new("wide", r"abc123", 0.4),
            Pattern::new("narrow", r"abc", 0.9),
        ],
        context: &[],
        validator: None,
    });
    let analyzer = AnalyzerEngine::with_registry(registry);

    let report = analyzer.analyze_report("abc123", None);

    assert_eq!(report.candidates().len(), 2);
    let spans: Vec<_> = report
        .candidates()
        .iter()
        .map(|finding| (finding.span().start(), finding.span().end()))
        .collect();
    assert!(spans.contains(&(0, 6)));
    assert!(spans.contains(&(0, 3)));

    assert_eq!(report.legacy_compatible_results().len(), 1);
    assert_eq!(report.legacy_compatible_results()[0].start, 0);
    assert_eq!(report.legacy_compatible_results()[0].end, 3);
    assert_eq!(report.legacy_compatible_results()[0].score, 0.9);
    assert!(!report.status().was_truncated());
}

#[test]
fn legacy_projection_matches_analyze_across_context_validator_and_filtering() {
    let analyzer = AnalyzerEngine::new();
    let text = "email jane@example.com card 4111 1111 1111 1111 ip 10.0.0.1";

    for filter in [
        None,
        Some(&[EntityType::Email][..]),
        Some(&[EntityType::CreditCard, EntityType::IpAddress][..]),
    ] {
        let legacy = analyzer.analyze(text, filter);
        let report = analyzer.analyze_report(text, filter);
        assert_eq!(legacy.as_slice(), report.legacy_compatible_results());
    }
}

#[test]
fn equal_score_tie_behavior_matches_legacy_path() {
    let registry = registry_with(PatternRecognizer {
        entity_type: EntityType::Email,
        patterns: vec![
            Pattern::new("first", r"abc", 0.7),
            Pattern::new("second", r"abc", 0.7),
        ],
        context: &[],
        validator: None,
    });
    let analyzer = AnalyzerEngine::with_registry(registry);

    let legacy = analyzer.analyze("abc", None);
    let report = analyzer.analyze_report("abc", None);

    assert_eq!(report.candidates().len(), 2);
    assert_eq!(legacy.as_slice(), report.legacy_compatible_results());
    assert_eq!(legacy.len(), 1);
}

#[test]
fn invalid_metadata_does_not_drop_a_valid_candidate_or_invent_provenance() {
    let registry = registry_with(PatternRecognizer {
        entity_type: EntityType::Email,
        patterns: vec![Pattern::new("bad/pattern name", r"abc", 0.7)],
        context: &[],
        validator: None,
    });
    let analyzer = AnalyzerEngine::with_registry(registry);

    let report = analyzer.analyze_report("abc", None);

    assert_eq!(report.candidates().len(), 1);
    assert_eq!(report.candidates()[0].recognizer(), None);
    assert!(report.candidates()[0]
        .evidence()
        .iter()
        .all(|evidence| !matches!(evidence, Evidence::Pattern { .. })));
    assert!(matches!(
        report.issues(),
        [AnalysisIssue::InvalidPatternMetadata { .. }]
    ));
    assert_eq!(
        analyzer.analyze("abc", None).as_slice(),
        report.legacy_compatible_results()
    );
}

#[test]
fn invalid_confidence_is_reported_without_false_validated_finding() {
    let registry = registry_with(PatternRecognizer {
        entity_type: EntityType::Email,
        patterns: vec![Pattern::new("nan", r"abc", f32::NAN)],
        context: &[],
        validator: None,
    });
    let analyzer = AnalyzerEngine::with_registry(registry);

    let report = analyzer.analyze_report("abc", None);

    assert!(report.candidates().is_empty());
    assert!(report.has_issues());
    assert!(matches!(
        report.issues(),
        [AnalysisIssue::InvalidConfidence { .. }]
    ));
    assert_eq!(
        analyzer.analyze("abc", None).as_slice(),
        report.legacy_compatible_results()
    );
}

#[test]
fn candidate_collection_is_deterministically_bounded() {
    let registry = registry_with(PatternRecognizer {
        entity_type: EntityType::Email,
        patterns: vec![Pattern::new("repeated", r".", 0.7)],
        context: &[],
        validator: None,
    });
    let analyzer = AnalyzerEngine::with_registry(registry);
    let options = AnalysisOptions::default().with_max_candidates(3);

    let report = analyzer.analyze_report_with_options("abcdef", None, options);

    assert_eq!(report.candidates().len(), 3);
    assert!(report.status().candidate_limit_reached());
    assert!(report.status().was_truncated());
}

#[test]
fn issue_details_are_deterministically_bounded() {
    let registry = registry_with(PatternRecognizer {
        entity_type: EntityType::Email,
        patterns: vec![
            Pattern::new("bad/one", r"a", 0.7),
            Pattern::new("bad/two", r"b", 0.7),
        ],
        context: &[],
        validator: None,
    });
    let analyzer = AnalyzerEngine::with_registry(registry);
    let options = AnalysisOptions::default().with_max_issues(1);

    let report = analyzer.analyze_report_with_options("ab", None, options);

    assert_eq!(report.candidates().len(), 2);
    assert_eq!(report.issues().len(), 1);
    assert!(report.status().issue_limit_reached());
}

#[test]
fn report_records_engine_version_not_recognizer_version() {
    let report = AnalyzerEngine::new().analyze_report("jane@example.com", None);

    assert_eq!(report.engine_version(), env!("CARGO_PKG_VERSION"));
    assert!(report
        .candidates()
        .iter()
        .all(|finding| finding.recognizer().is_none()));
}
