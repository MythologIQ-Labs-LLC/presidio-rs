use presidio::{AnalysisRequest, AnalyzerEngine, DocumentId, TextDocument};

fn main() {
    let document = TextDocument::new(
        DocumentId::new("request-42").expect("valid document ID"),
        "Email jane@acme.com",
    );
    let request = AnalysisRequest::new();

    let report = AnalyzerEngine::new()
        .analyze_request(&document, &request)
        .expect("bounded analysis");

    report
        .validate_for_document(&document)
        .expect("matching source document");

    let email = report
        .candidates()
        .iter()
        .find(|finding| finding.entity().as_str() == "EMAIL_ADDRESS")
        .expect("email candidate");

    assert_eq!(
        email.slice_document(&document).expect("bound source slice"),
        "jane@acme.com"
    );

    println!("{}", email.entity());
}
