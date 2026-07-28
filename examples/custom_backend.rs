use presidio::{
    AnalysisRequest, AnalyzerEngine, CandidateEmitter, DocumentId, EntityId, Evidence, MetadataId,
    RecognitionError, RecognitionMechanism, Recognizer, RecognizerId, RecognizerMetadata,
    TextDocument,
};

struct EmployeeIdBackend {
    metadata: RecognizerMetadata,
    entity: EntityId,
}

impl EmployeeIdBackend {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let entity = EntityId::new("EMPLOYEE_ID")?;
        let metadata = RecognizerMetadata::new(
            RecognizerId::new("example.employee-id-backend")?,
            MetadataId::new("1.0.0")?,
            [entity.clone()],
            RecognitionMechanism::Structural,
        )?
        .with_default_enabled(true)
        .with_attribution(MetadataId::new("example.repository-authored")?);

        Ok(Self { metadata, entity })
    }
}

impl Recognizer for EmployeeIdBackend {
    fn metadata(&self) -> &RecognizerMetadata {
        &self.metadata
    }

    fn recognize(
        &self,
        document: &TextDocument<'_>,
        _request: &AnalysisRequest,
        emitter: &mut CandidateEmitter<'_, '_>,
    ) -> Result<(), RecognitionError> {
        let text = document.original();
        let prefix = "EMP-";

        for (start, _) in text.match_indices(prefix) {
            let end = start + prefix.len() + 6;
            if end > text.len() {
                continue;
            }

            let candidate = &text[start..end];
            if candidate[prefix.len()..]
                .bytes()
                .all(|byte| byte.is_ascii_digit())
            {
                emitter
                    .emit(
                        self.entity.clone(),
                        start,
                        end,
                        0.95,
                        std::iter::empty::<Evidence>(),
                    )
                    .map_err(|error| RecognitionError::from_candidate(&error))?;
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut analyzer = AnalyzerEngine::new();
    analyzer.add_backend(EmployeeIdBackend::new()?)?;

    let document = TextDocument::new(
        DocumentId::new("custom-backend-example")?,
        "Escalate ticket ownership to EMP-104729.",
    );
    let report = analyzer.analyze_request(&document, &AnalysisRequest::new())?;

    let finding = report
        .candidates()
        .iter()
        .find(|candidate| candidate.entity().as_str() == "EMPLOYEE_ID")
        .expect("the custom employee identifier should be detected");

    assert_eq!(finding.slice_document(&document)?, "EMP-104729");
    assert_eq!(
        finding.recognizer().expect("backend provenance"),
        &RecognizerId::new("example.employee-id-backend")?
    );

    Ok(())
}
