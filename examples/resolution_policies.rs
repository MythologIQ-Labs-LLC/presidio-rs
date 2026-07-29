use presidio::{
    AnalysisRequest, AnalyzerEngine, DocumentId, ResolutionOptions, ResolutionPolicy,
    ResolvedFinding, TextDocument,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document = TextDocument::new(
        DocumentId::new("example-request")?,
        "Email jane@example.com or call 202-555-0142.",
    );
    let analysis = AnalyzerEngine::new().analyze_request(&document, &AnalysisRequest::new())?;

    println!("raw candidates: {}", analysis.candidates().len());

    for policy in [
        ResolutionPolicy::ReportAll,
        ResolutionPolicy::BestCandidate,
        ResolutionPolicy::ConservativeRedaction,
    ] {
        let integrated =
            analysis.resolve_for_document(&document, &ResolutionOptions::new(policy))?;
        let report = integrated.resolution();
        println!(
            "{}/v{}: {} resolved outputs",
            report.policy_id(),
            report.policy_version(),
            report.resolved().len()
        );

        for resolved in report.resolved() {
            match resolved {
                ResolvedFinding::Candidate { candidate } => {
                    let finding = &report.candidates()[*candidate];
                    println!(
                        "  candidate {} {} {}..{}",
                        candidate,
                        finding.entity(),
                        finding.span().start(),
                        finding.span().end()
                    );
                }
                ResolvedFinding::Union {
                    span,
                    entity,
                    candidates,
                    ..
                } => println!(
                    "  union {:?} {}..{} from {:?}",
                    entity,
                    span.start(),
                    span.end(),
                    candidates
                ),
                _ => println!("  future resolution output"),
            }
        }
    }

    Ok(())
}
