use presidio::{
    AnalysisExecutionError, AnalysisIssue, AnalysisRequest, CandidateEmitter, DocumentId,
    EmissionStatus, EntityId, MetadataId, Pattern, PatternRecognizer, RecognitionError,
    RecognitionErrorKind, RecognitionMechanism, Recognizer, RecognizerId, RecognizerMetadata,
    RecognizerRegistry, TextDocument,
};

#[derive(Clone, Copy)]
enum Behavior {
    EmitKnown,
    EmitCustom,
    EmitMany,
    Fail,
    EmitUndeclared,
}

struct TestBackend {
    metadata: RecognizerMetadata,
    behavior: Behavior,
}

impl Recognizer for TestBackend {
    fn metadata(&self) -> &RecognizerMetadata {
        &self.metadata
    }

    fn recognize(
        &self,
        document: &TextDocument<'_>,
        _request: &AnalysisRequest,
        emitter: &mut CandidateEmitter<'_, '_>,
    ) -> Result<(), RecognitionError> {
        match self.behavior {
            Behavior::EmitKnown => {
                if let Some(start) = document.original().find("mail") {
                    emitter
                        .emit(
                            EntityId::new("EMAIL_ADDRESS").expect("valid entity"),
                            start,
                            start + 4,
                            0.8,
                            [],
                        )
                        .map_err(|error| RecognitionError::from_candidate(&error))?;
                }
                Ok(())
            }
            Behavior::EmitCustom => {
                if let Some(start) = document.original().find("SECRET") {
                    emitter
                        .emit(
                            EntityId::new("CUSTOM_SECRET").expect("valid entity"),
                            start,
                            start + 6,
                            0.9,
                            [],
                        )
                        .map_err(|error| RecognitionError::from_candidate(&error))?;
                }
                Ok(())
            }
            Behavior::EmitMany => {
                for (start, _) in document.original().match_indices('x') {
                    let status = emitter
                        .emit(
                            EntityId::new("CUSTOM_SECRET").expect("valid entity"),
                            start,
                            start + 1,
                            0.7,
                            [],
                        )
                        .map_err(|error| RecognitionError::from_candidate(&error))?;
                    if status == EmissionStatus::LimitReached {
                        break;
                    }
                }
                Ok(())
            }
            Behavior::Fail => Err(RecognitionError::new(
                RecognitionErrorKind::BackendUnavailable,
                MetadataId::new("backend.model-unavailable").expect("valid code"),
                true,
            )),
            Behavior::EmitUndeclared => {
                let error = emitter
                    .emit(
                        EntityId::new("OTHER_SECRET").expect("valid entity"),
                        0,
                        1,
                        0.7,
                        [],
                    )
                    .expect_err("undeclared entity must fail");
                Err(RecognitionError::from_candidate(&error))
            }
        }
    }
}

fn metadata(id: &str, entity: &str, default_enabled: bool) -> RecognizerMetadata {
    RecognizerMetadata::new(
        RecognizerId::new(id).expect("valid recognizer ID"),
        MetadataId::new("1.0.0").expect("valid version"),
        [EntityId::new(entity).expect("valid entity")],
        RecognitionMechanism::Custom,
    )
    .expect("valid metadata")
    .with_default_enabled(default_enabled)
}

fn document<'a>(text: &'a str) -> TextDocument<'a> {
    TextDocument::new(
        DocumentId::new("analysis-request-test").expect("valid document ID"),
        text,
    )
}

#[test]
fn default_request_executes_authoritative_builtin_recognizers() {
    let source = document("email jane@example.com");
    let report = presidio::AnalyzerEngine::new()
        .analyze_request(&source, &AnalysisRequest::new())
        .expect("bounded request");

    assert!(report
        .candidates()
        .iter()
        .any(|finding| finding.entity().as_str() == "EMAIL_ADDRESS"));
    assert!(report
        .candidates()
        .iter()
        .all(|finding| finding.document_binding() == Some(source.binding())));
    assert!(!report.status().legacy_projection_incomplete());
}

#[test]
fn request_rejects_input_before_backend_execution() {
    let source = document("too large");
    let request = AnalysisRequest::new()
        .with_max_input_bytes(3)
        .expect("positive limit");

    assert_eq!(
        presidio::AnalyzerEngine::new().analyze_request(&source, &request),
        Err(AnalysisExecutionError::InputTooLarge {
            actual: source.len(),
            maximum: 3,
        })
    );
}

#[test]
fn explicit_custom_backend_emits_open_entity_with_incomplete_legacy_projection() {
    let backend = TestBackend {
        metadata: metadata("custom.secret", "CUSTOM_SECRET", false),
        behavior: Behavior::EmitCustom,
    };
    let mut analyzer = presidio::AnalyzerEngine::with_registry(RecognizerRegistry::empty());
    analyzer.add_backend(backend).expect("unique backend");
    let request = AnalysisRequest::new()
        .with_recognizers([RecognizerId::new("custom.secret").expect("valid ID")])
        .expect("unique recognizer selection");
    let source = document("value SECRET here");

    let report = analyzer
        .analyze_request(&source, &request)
        .expect("custom analysis");

    assert_eq!(report.candidates().len(), 1);
    assert_eq!(report.candidates()[0].entity().as_str(), "CUSTOM_SECRET");
    assert_eq!(report.legacy_compatible_results().len(), 0);
    assert!(report.status().legacy_projection_incomplete());
    assert_eq!(
        report.candidates()[0]
            .recognizer()
            .expect("recognizer provenance")
            .as_str(),
        "custom.secret"
    );
}

#[test]
fn legacy_pattern_registration_is_skipped_but_legacy_api_still_runs_it() {
    let mut registry = RecognizerRegistry::empty();
    registry.add(PatternRecognizer {
        entity_type: presidio::EntityType::ApiKey,
        patterns: vec![Pattern::new("legacy", r"LEGACY-[0-9]+", 0.9)],
        context: &[],
        validator: None,
    });
    let analyzer = presidio::AnalyzerEngine::with_registry(registry);
    let source = document("LEGACY-42");

    assert_eq!(analyzer.analyze(source.original(), None).len(), 1);
    let report = analyzer
        .analyze_request(&source, &AnalysisRequest::new())
        .expect("request report");
    assert!(report.candidates().is_empty());
    assert!(matches!(
        report.issues(),
        [AnalysisIssue::LegacyRecognizersSkipped { count: 1 }]
    ));
}

#[test]
fn typed_backend_failure_is_preserved_without_plaintext() {
    let backend = TestBackend {
        metadata: metadata("custom.failure", "CUSTOM_SECRET", true),
        behavior: Behavior::Fail,
    };
    let mut analyzer = presidio::AnalyzerEngine::with_registry(RecognizerRegistry::empty());
    analyzer.add_backend(backend).expect("unique backend");
    let source = document("SECRET");

    let report = analyzer
        .analyze_request(&source, &AnalysisRequest::new())
        .expect("report despite backend failure");

    assert!(matches!(
        report.issues(),
        [AnalysisIssue::RecognitionFailed { recognizer, error }]
            if recognizer.as_str() == "custom.failure"
                && error.kind() == RecognitionErrorKind::BackendUnavailable
                && error.code().as_str() == "backend.model-unavailable"
                && error.retryable()
    ));
}

#[test]
fn candidate_limit_is_global_and_observable() {
    let backend = TestBackend {
        metadata: metadata("custom.many", "CUSTOM_SECRET", true),
        behavior: Behavior::EmitMany,
    };
    let mut analyzer = presidio::AnalyzerEngine::with_registry(RecognizerRegistry::empty());
    analyzer.add_backend(backend).expect("unique backend");
    let request = AnalysisRequest::new()
        .with_max_candidates(2)
        .expect("positive limit");
    let source = document("xxxxx");

    let report = analyzer
        .analyze_request(&source, &request)
        .expect("bounded report");

    assert_eq!(report.candidates().len(), 2);
    assert!(report.status().candidate_limit_reached());
}

#[test]
fn locale_and_capability_requirements_control_selection() {
    let backend_metadata = metadata("custom.capability", "EMAIL_ADDRESS", true)
        .with_supported_locales([MetadataId::new("en-US").expect("valid locale")])
        .expect("unique locale")
        .with_required_capabilities([MetadataId::new("model.local").expect("valid capability")])
        .expect("unique capability");
    let backend = TestBackend {
        metadata: backend_metadata,
        behavior: Behavior::EmitKnown,
    };
    let mut analyzer = presidio::AnalyzerEngine::with_registry(RecognizerRegistry::empty());
    analyzer.add_backend(backend).expect("unique backend");
    let source = document("mail");

    let missing_capability = analyzer
        .analyze_request(&source, &AnalysisRequest::new())
        .expect("empty report");
    assert!(missing_capability.candidates().is_empty());

    let selected = AnalysisRequest::new()
        .with_locale(MetadataId::new("en-US").expect("valid locale"))
        .with_available_capabilities([MetadataId::new("model.local").expect("valid capability")])
        .expect("unique capability");
    assert_eq!(
        analyzer
            .analyze_request(&source, &selected)
            .expect("selected report")
            .candidates()
            .len(),
        1
    );

    let wrong_locale = AnalysisRequest::new()
        .with_locale(MetadataId::new("fr-FR").expect("valid locale"))
        .with_available_capabilities([MetadataId::new("model.local").expect("valid capability")])
        .expect("unique capability");
    assert!(analyzer
        .analyze_request(&source, &wrong_locale)
        .expect("empty locale report")
        .candidates()
        .is_empty());
}

#[test]
fn undeclared_candidate_becomes_typed_recognition_issue() {
    let backend = TestBackend {
        metadata: metadata("custom.invalid", "CUSTOM_SECRET", true),
        behavior: Behavior::EmitUndeclared,
    };
    let mut analyzer = presidio::AnalyzerEngine::with_registry(RecognizerRegistry::empty());
    analyzer.add_backend(backend).expect("unique backend");
    let source = document("x");

    let report = analyzer
        .analyze_request(&source, &AnalysisRequest::new())
        .expect("report with issue");

    assert!(matches!(
        report.issues(),
        [AnalysisIssue::RecognitionFailed { error, .. }]
            if error.kind() == RecognitionErrorKind::InvalidCandidate
                && error.code().as_str() == "candidate.undeclared-entity"
    ));
}
