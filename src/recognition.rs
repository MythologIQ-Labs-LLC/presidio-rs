//! Backend-neutral recognizer execution and validated candidate emission.

use core::fmt;

use crate::context;
use crate::document::{DocumentBinding, TextDocument};
use crate::metadata::RecognizerMetadata;
use crate::recognizer::PatternRecognizer;
use crate::request::AnalysisRequest;
use crate::types::{
    Confidence, ConfidenceError, EntityId, Evidence, Finding, MetadataId, Span, SpanError,
};

/// Backend-neutral recognizer interface.
///
/// Implementations receive an exact source document, a validated request, and a
/// candidate emitter that enforces source coordinates, declared entities,
/// confidence bounds, provenance, and candidate limits.
pub trait Recognizer: Send + Sync {
    /// Authoritative recognizer metadata.
    fn metadata(&self) -> &RecognizerMetadata;

    /// Whether this recognizer is selected and executable for the request.
    fn supports(&self, request: &AnalysisRequest) -> bool {
        request.selects(self.metadata())
    }

    /// Analyze a document and emit validated candidates.
    fn recognize(
        &self,
        document: &TextDocument<'_>,
        request: &AnalysisRequest,
        emitter: &mut CandidateEmitter<'_, '_>,
    ) -> Result<(), RecognitionError>;
}

/// Validated candidate sink scoped to one recognizer and one exact document.
pub struct CandidateEmitter<'metadata, 'text> {
    text: &'text str,
    binding: DocumentBinding,
    metadata: &'metadata RecognizerMetadata,
    maximum: usize,
    findings: Vec<Finding>,
    limit_reached: bool,
}

impl<'metadata, 'text> CandidateEmitter<'metadata, 'text> {
    pub(crate) fn new(
        document: &'text TextDocument<'_>,
        metadata: &'metadata RecognizerMetadata,
        maximum: usize,
    ) -> Self {
        Self {
            text: document.original(),
            binding: document.binding().clone(),
            metadata,
            maximum,
            findings: Vec::new(),
            limit_reached: false,
        }
    }

    /// Emit one candidate using original UTF-8 byte coordinates.
    pub fn emit(
        &mut self,
        entity: EntityId,
        start: usize,
        end: usize,
        score: f32,
        evidence: impl IntoIterator<Item = Evidence>,
    ) -> Result<EmissionStatus, CandidateEmissionError> {
        if self.findings.len() >= self.maximum {
            self.limit_reached = true;
            return Ok(EmissionStatus::LimitReached);
        }

        if !self.metadata.supported_entities().contains(&entity) {
            return Err(CandidateEmissionError::UndeclaredEntity { entity });
        }

        let span = Span::new_for(self.text, start, end).map_err(CandidateEmissionError::Span)?;
        let confidence = Confidence::new(score).map_err(CandidateEmissionError::Confidence)?;
        let finding = Finding::new(entity, span, confidence)
            .with_recognizer(self.metadata.id().clone())
            .with_document_binding(self.binding.clone())
            .with_evidence(evidence);
        self.findings.push(finding);
        Ok(EmissionStatus::Accepted)
    }

    /// Remaining candidate capacity for this recognizer invocation.
    pub fn remaining(&self) -> usize {
        self.maximum.saturating_sub(self.findings.len())
    }

    /// Whether an emission attempt reached the configured candidate limit.
    pub const fn limit_reached(&self) -> bool {
        self.limit_reached
    }

    pub(crate) fn into_findings(self) -> Vec<Finding> {
        self.findings
    }
}

/// Outcome of one validated candidate-emission attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmissionStatus {
    /// The candidate was accepted.
    Accepted,
    /// The configured candidate capacity was already exhausted.
    LimitReached,
}

/// Candidate data violated the emitter's source or metadata contract.
#[derive(Debug, Clone, PartialEq)]
pub enum CandidateEmissionError {
    /// The recognizer emitted an entity not declared by its metadata.
    UndeclaredEntity { entity: EntityId },
    /// The candidate span was not valid for the exact source document.
    Span(SpanError),
    /// The candidate confidence was non-finite or outside `0.0..=1.0`.
    Confidence(ConfidenceError),
}

impl fmt::Display for CandidateEmissionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UndeclaredEntity { entity } => {
                write!(formatter, "recognizer emitted undeclared entity {entity}")
            }
            Self::Span(error) => write!(formatter, "invalid candidate span: {error}"),
            Self::Confidence(error) => write!(formatter, "invalid candidate confidence: {error}"),
        }
    }
}

impl std::error::Error for CandidateEmissionError {}

/// Stable category of recognizer execution failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub enum RecognitionErrorKind {
    /// A candidate violated the validated emission contract.
    InvalidCandidate,
    /// Required backend resources are unavailable.
    BackendUnavailable,
    /// Recognizer configuration is invalid for execution.
    Configuration,
    /// A recognizer-specific resource limit was reached.
    ResourceLimit,
    /// An unexpected recognizer implementation failure occurred.
    Internal,
}

/// Non-plaintext typed recognizer execution failure.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RecognitionError {
    kind: RecognitionErrorKind,
    code: MetadataId,
    retryable: bool,
}

impl RecognitionError {
    /// Construct a typed failure from a stable non-plaintext code.
    pub fn new(kind: RecognitionErrorKind, code: MetadataId, retryable: bool) -> Self {
        Self {
            kind,
            code,
            retryable,
        }
    }

    /// Convert a candidate-emission violation into a stable recognizer failure.
    pub fn from_candidate(error: &CandidateEmissionError) -> Self {
        let code = match error {
            CandidateEmissionError::UndeclaredEntity { .. } => "candidate.undeclared-entity",
            CandidateEmissionError::Span(_) => "candidate.invalid-span",
            CandidateEmissionError::Confidence(_) => "candidate.invalid-confidence",
        };
        Self::new(
            RecognitionErrorKind::InvalidCandidate,
            MetadataId::new(code).expect("built-in recognition error code is valid"),
            false,
        )
    }

    /// Failure category.
    pub const fn kind(&self) -> RecognitionErrorKind {
        self.kind
    }

    /// Stable non-plaintext error code.
    pub fn code(&self) -> &MetadataId {
        &self.code
    }

    /// Whether retrying with the same request may be meaningful.
    pub const fn retryable(&self) -> bool {
        self.retryable
    }
}

impl fmt::Display for RecognitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "recognizer failure {:?} ({}, retryable={})",
            self.kind, self.code, self.retryable
        )
    }
}

impl std::error::Error for RecognitionError {}

/// Borrowed adapter that executes a strictly registered pattern recognizer
/// through the backend-neutral trait.
pub(crate) struct PatternRecognizerAdapter<'a> {
    metadata: &'a RecognizerMetadata,
    recognizer: &'a PatternRecognizer,
}

impl<'a> PatternRecognizerAdapter<'a> {
    pub(crate) const fn new(
        metadata: &'a RecognizerMetadata,
        recognizer: &'a PatternRecognizer,
    ) -> Self {
        Self {
            metadata,
            recognizer,
        }
    }
}

impl Recognizer for PatternRecognizerAdapter<'_> {
    fn metadata(&self) -> &RecognizerMetadata {
        self.metadata
    }

    fn recognize(
        &self,
        document: &TextDocument<'_>,
        _request: &AnalysisRequest,
        emitter: &mut CandidateEmitter<'_, '_>,
    ) -> Result<(), RecognitionError> {
        let text = document.original();
        for pattern in &self.recognizer.patterns {
            let pattern_id = MetadataId::new(pattern.name).map_err(|_| {
                RecognitionError::new(
                    RecognitionErrorKind::Configuration,
                    MetadataId::new("pattern.invalid-metadata")
                        .expect("built-in error code is valid"),
                    false,
                )
            })?;

            for matched in pattern.regex.find_iter(text) {
                let mut score = pattern.base_score;
                let mut validator_accepted = false;
                if let Some(validate) = self.recognizer.validator {
                    match validate(matched.as_str()) {
                        Some(true) => {
                            score = 1.0;
                            validator_accepted = true;
                        }
                        Some(false) => continue,
                        None => {}
                    }
                }

                score = context::enhance(
                    text,
                    matched.start(),
                    matched.end(),
                    score,
                    self.recognizer.context,
                );

                let mut evidence = vec![Evidence::Pattern {
                    pattern_id: pattern_id.clone(),
                }];
                if validator_accepted {
                    evidence.push(Evidence::LegacyValidatorAccepted);
                }

                let status = emitter
                    .emit(
                        EntityId::from(self.recognizer.entity_type),
                        matched.start(),
                        matched.end(),
                        score,
                        evidence,
                    )
                    .map_err(|error| RecognitionError::from_candidate(&error))?;
                if status == EmissionStatus::LimitReached {
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}
