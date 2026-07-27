use presidio::{
    AnalyzerEngine, EntityId, EntityType, MetadataId, Pattern, PatternRecognizer,
    PatternRecognizerRegistrationError, PatternValidationError, RecognitionMechanism, RecognizerId,
    RecognizerMetadata, RecognizerMetadataError, RecognizerRegistry, RecognizerRegistryError,
};

fn metadata(
    id: &str,
    version: &str,
    entity_type: EntityType,
    mechanism: RecognitionMechanism,
) -> RecognizerMetadata {
    RecognizerMetadata::new(
        RecognizerId::new(id).expect("valid recognizer ID"),
        MetadataId::new(version).expect("valid version"),
        [EntityId::from(entity_type)],
        mechanism,
    )
    .expect("valid metadata")
}

fn email_recognizer(pattern_name: &'static str) -> PatternRecognizer {
    PatternRecognizer {
        entity_type: EntityType::Email,
        patterns: vec![Pattern::try_new(pattern_name, r"abc", 0.7).expect("valid pattern")],
        context: &[],
        validator: None,
    }
}

#[test]
fn strict_pattern_construction_rejects_unsafe_configuration() {
    assert!(matches!(
        Pattern::try_new("bad/name", r"abc", 0.7),
        Err(PatternValidationError::InvalidName(_))
    ));
    assert!(matches!(
        Pattern::try_new("invalid-regex", r"[", 0.7),
        Err(PatternValidationError::InvalidRegex(_))
    ));
    assert!(matches!(
        Pattern::try_new("nan", r"abc", f32::NAN),
        Err(PatternValidationError::InvalidBaseScore(_))
    ));
    assert!(matches!(
        Pattern::try_new("empty", r"", 0.7),
        Err(PatternValidationError::MatchesEmptyInput)
    ));
}

#[test]
fn metadata_rejects_empty_and_duplicate_dimensions() {
    let id = RecognizerId::new("custom.metadata").expect("valid ID");
    let version = MetadataId::new("1.0.0").expect("valid version");
    assert_eq!(
        RecognizerMetadata::new(
            id.clone(),
            version.clone(),
            [],
            RecognitionMechanism::Pattern,
        ),
        Err(RecognizerMetadataError::NoSupportedEntities)
    );

    let entity = EntityId::from(EntityType::Email);
    assert!(matches!(
        RecognizerMetadata::new(
            id,
            version,
            [entity.clone(), entity],
            RecognitionMechanism::Pattern,
        ),
        Err(RecognizerMetadataError::DuplicateEntity { .. })
    ));

    let locale = MetadataId::new("en-US").expect("valid locale");
    let metadata = metadata(
        "custom.locales",
        "1.0.0",
        EntityType::Email,
        RecognitionMechanism::Pattern,
    );
    assert!(matches!(
        metadata.with_supported_locales([locale.clone(), locale]),
        Err(RecognizerMetadataError::DuplicateLocale { .. })
    ));
}

#[test]
fn registration_rejects_entity_and_mechanism_mismatch() {
    let mut registry = RecognizerRegistry::empty();
    let entity_mismatch = metadata(
        "custom.wrong-entity",
        "1.0.0",
        EntityType::Ssn,
        RecognitionMechanism::Pattern,
    );
    assert!(matches!(
        registry.add_with_metadata(entity_mismatch, email_recognizer("email-entity")),
        Err(RecognizerRegistryError::InvalidRecognizer(
            PatternRecognizerRegistrationError::EntityMismatch { .. }
        ))
    ));

    let mechanism_mismatch = metadata(
        "custom.wrong-mechanism",
        "1.0.0",
        EntityType::Email,
        RecognitionMechanism::PatternWithValidation,
    );
    assert!(matches!(
        registry.add_with_metadata(mechanism_mismatch, email_recognizer("email-mechanism")),
        Err(RecognizerRegistryError::InvalidRecognizer(
            PatternRecognizerRegistrationError::MechanismMismatch { .. }
        ))
    ));
}

#[test]
fn registry_rejects_duplicate_recognizer_ids() {
    let mut registry = RecognizerRegistry::empty();
    registry
        .add_with_metadata(
            metadata(
                "custom.duplicate",
                "1.0.0",
                EntityType::Email,
                RecognitionMechanism::Pattern,
            ),
            email_recognizer("first"),
        )
        .expect("first registration");

    assert!(matches!(
        registry.add_with_metadata(
            metadata(
                "custom.duplicate",
                "2.0.0",
                EntityType::Email,
                RecognitionMechanism::Pattern,
            ),
            email_recognizer("second"),
        ),
        Err(RecognizerRegistryError::DuplicateRecognizerId { .. })
    ));
}

#[test]
fn custom_metadata_is_preserved_separately_from_engine_version() {
    let custom_metadata = metadata(
        "custom.email",
        "7.4.2",
        EntityType::Email,
        RecognitionMechanism::Pattern,
    )
    .with_supported_locales([MetadataId::new("en-US").expect("valid locale")])
    .expect("unique locale")
    .with_evaluation_receipt(MetadataId::new("eval.custom-email.v1").expect("valid receipt"));

    let mut registry = RecognizerRegistry::empty();
    registry
        .add_with_metadata(custom_metadata, email_recognizer("custom-email"))
        .expect("valid registration");
    let analyzer = AnalyzerEngine::with_registry(registry);
    let report = analyzer.analyze_report("abc", None);
    let finding = report.candidates().first().expect("custom finding");
    let recognizer_id = finding.recognizer().expect("authoritative recognizer ID");
    let metadata = report
        .recognizer_metadata(recognizer_id)
        .expect("metadata snapshot");

    assert_eq!(report.engine_version(), env!("CARGO_PKG_VERSION"));
    assert_eq!(recognizer_id.as_str(), "custom.email");
    assert_eq!(metadata.version().as_str(), "7.4.2");
    assert_eq!(metadata.supported_locales()[0].as_str(), "en-US");
    assert_eq!(
        metadata
            .evaluation_receipt()
            .expect("evaluation receipt")
            .as_str(),
        "eval.custom-email.v1"
    );
}

#[test]
fn legacy_registration_keeps_provenance_unknown() {
    let mut registry = RecognizerRegistry::empty();
    registry.add(email_recognizer("legacy-email"));
    let report = AnalyzerEngine::with_registry(registry).analyze_report("abc", None);

    assert_eq!(report.candidates().len(), 1);
    assert_eq!(report.candidates()[0].recognizer(), None);
    assert!(report.recognizers().is_empty());
}
