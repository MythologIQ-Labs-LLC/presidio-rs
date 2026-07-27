use presidio::{
    anonymize, AnalyzerEngine, AnonymizerEngine, EntityType, Operator, Pattern, PatternRecognizer,
    RecognizerRegistry,
};

#[test]
fn detects_and_replaces_email_and_card() {
    let analyzer = AnalyzerEngine::new();
    let text = "Reach jane.doe@acme.com or card 4111 1111 1111 1111 today.";
    let results = analyzer.analyze(text, None);

    assert!(results.iter().any(|r| r.entity_type == EntityType::Email));
    assert!(results
        .iter()
        .any(|r| r.entity_type == EntityType::CreditCard && r.score >= 0.99));

    let clean = anonymize(text, &results, &Operator::Replace(None));
    assert!(!clean.contains("jane.doe@acme.com"));
    assert!(!clean.contains("4111"));
    assert!(clean.contains("<EMAIL_ADDRESS>"));
    assert!(clean.contains("<CREDIT_CARD>"));
}

#[test]
fn luhn_rejects_invalid_card() {
    let analyzer = AnalyzerEngine::new();
    let text = "bad card 4111 1111 1111 1112 here";
    let results = analyzer.analyze(text, None);
    assert!(!results
        .iter()
        .any(|r| r.entity_type == EntityType::CreditCard));
}

#[test]
fn context_word_boosts_ssn_score() {
    let analyzer = AnalyzerEngine::new();
    let results = analyzer.analyze("SSN: 123-45-6789", None);
    let ssn = results
        .iter()
        .find(|r| r.entity_type == EntityType::Ssn)
        .expect("ssn detected");
    assert!(
        ssn.score >= 0.4,
        "context should lift score, got {}",
        ssn.score
    );
}

#[test]
fn iban_checksum_validates() {
    let analyzer = AnalyzerEngine::new();
    let text = "IBAN GB82WEST12345698765432 please";
    let results = analyzer.analyze(text, None);
    assert!(results
        .iter()
        .any(|r| r.entity_type == EntityType::IbanCode && r.score >= 0.99));
}

#[test]
fn mask_keeps_last_four() {
    let analyzer = AnalyzerEngine::new();
    let text = "card 4111 1111 1111 1111";
    let results = analyzer.analyze(text, None);
    let masked = anonymize(
        text,
        &results,
        &Operator::Mask {
            mask_char: '*',
            keep_last: 4,
        },
    );
    assert!(masked.contains("1111"));
    assert!(masked.contains('*'));
}

#[test]
fn entity_filter_scopes_detection() {
    let analyzer = AnalyzerEngine::new();
    let text = "jane@acme.com and 4111 1111 1111 1111";
    let only_email = analyzer.analyze(text, Some(&[EntityType::Email]));
    assert!(only_email
        .iter()
        .all(|r| r.entity_type == EntityType::Email));
    assert!(!only_email.is_empty());
}

#[test]
fn hash_operator_is_deterministic() {
    let analyzer = AnalyzerEngine::new();
    let text = "contact jane@acme.com";
    let results = analyzer.analyze(text, None);
    let op = Operator::Hash { salt: "s".into() };
    let a = anonymize(text, &results, &op);
    let b = anonymize(text, &results, &op);
    assert_eq!(a, b);
    assert!(!a.contains("jane@acme.com"));
}

#[test]
fn custom_recognizer_via_registry() {
    // An empty registry with a single custom recognizer detects only that entity.
    let mut registry = RecognizerRegistry::empty();
    registry.add(PatternRecognizer {
        entity_type: EntityType::ApiKey,
        patterns: vec![Pattern::new("acme-key", r"\bACME-[A-Z0-9]{8}\b", 0.9)],
        context: &["key", "token"],
        validator: None,
    });
    let analyzer = AnalyzerEngine::with_registry(registry);
    let results = analyzer.analyze("token ACME-12ABCD34 here", None);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].entity_type, EntityType::ApiKey);
    assert!(results[0].score >= 0.9);
}

#[test]
fn anonymizer_engine_applies_per_entity_operators() {
    let analyzer = AnalyzerEngine::new();
    let text = "jane@acme.com paid with card 4111 1111 1111 1111";
    let results = analyzer.analyze(text, None);

    let engine = AnonymizerEngine::new(Operator::Redact)
        .with_operator(EntityType::Email, Operator::Replace(Some("[email]".into())))
        .with_operator(
            EntityType::CreditCard,
            Operator::Mask {
                mask_char: '#',
                keep_last: 4,
            },
        );
    let out = engine.anonymize(text, &results);

    assert!(out.contains("[email]")); // email -> replace override
    assert!(out.contains("1111")); // card -> mask keeps last 4
    assert!(out.contains('#'));
    assert!(!out.contains("jane@acme.com"));
}

#[test]
fn predefined_registry_is_populated() {
    let registry = RecognizerRegistry::with_predefined();
    assert!(!registry.is_empty());
    assert_eq!(registry.len(), registry.recognizers().len());
}
