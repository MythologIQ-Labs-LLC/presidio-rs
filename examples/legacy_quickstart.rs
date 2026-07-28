use presidio::{AnalyzerEngine, AnonymizerEngine, Operator};

fn main() {
    let analyzer = AnalyzerEngine::new();
    let text = "Email jane@acme.com about card 4111 1111 1111 1111.";

    let findings = analyzer.analyze(text, None);
    let clean =
        AnonymizerEngine::new(Operator::Replace(None)).anonymize(text, &findings);

    assert_eq!(
        clean,
        "Email <EMAIL_ADDRESS> about card <CREDIT_CARD>."
    );

    println!("{clean}");
}
