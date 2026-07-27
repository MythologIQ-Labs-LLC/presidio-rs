//! Validated request selection and resource limits for document-aware analysis.

use core::fmt;
use std::collections::HashSet;

use crate::metadata::RecognizerMetadata;
use crate::report::{DEFAULT_REPORT_CANDIDATE_LIMIT, DEFAULT_REPORT_ISSUE_LIMIT};
use crate::types::{Confidence, EntityId, MetadataId, RecognizerId};

/// Default maximum input size accepted by the request-oriented API: 1 MiB.
pub const DEFAULT_ANALYSIS_INPUT_LIMIT: usize = 1_048_576;

/// Backend-neutral request for document-aware recognition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AnalysisRequest {
    entities: Vec<EntityId>,
    recognizers: Vec<RecognizerId>,
    locale: Option<MetadataId>,
    available_capabilities: Vec<MetadataId>,
    minimum_confidence: Confidence,
    max_input_bytes: usize,
    max_candidates: usize,
    max_issues: usize,
}

impl Default for AnalysisRequest {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            recognizers: Vec::new(),
            locale: None,
            available_capabilities: Vec::new(),
            minimum_confidence: Confidence::new(0.3).expect("default confidence is valid"),
            max_input_bytes: DEFAULT_ANALYSIS_INPUT_LIMIT,
            max_candidates: DEFAULT_REPORT_CANDIDATE_LIMIT,
            max_issues: DEFAULT_REPORT_ISSUE_LIMIT,
        }
    }
}

impl AnalysisRequest {
    /// Construct the default bounded request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict recognition to these open entity identifiers.
    pub fn with_entities(
        mut self,
        entities: impl IntoIterator<Item = EntityId>,
    ) -> Result<Self, AnalysisRequestError> {
        let entities: Vec<_> = entities.into_iter().collect();
        ensure_unique_entities(&entities)?;
        self.entities = entities;
        Ok(self)
    }

    /// Explicitly select recognizers by stable identifier.
    ///
    /// When this list is empty, only metadata marked default-enabled is selected.
    pub fn with_recognizers(
        mut self,
        recognizers: impl IntoIterator<Item = RecognizerId>,
    ) -> Result<Self, AnalysisRequestError> {
        let recognizers: Vec<_> = recognizers.into_iter().collect();
        ensure_unique_recognizers(&recognizers)?;
        self.recognizers = recognizers;
        Ok(self)
    }

    /// Set the requested locale or country identifier.
    pub fn with_locale(mut self, locale: MetadataId) -> Self {
        self.locale = Some(locale);
        self
    }

    /// Declare capabilities available to recognizers for this request.
    pub fn with_available_capabilities(
        mut self,
        capabilities: impl IntoIterator<Item = MetadataId>,
    ) -> Result<Self, AnalysisRequestError> {
        let capabilities: Vec<_> = capabilities.into_iter().collect();
        ensure_unique_capabilities(&capabilities)?;
        self.available_capabilities = capabilities;
        Ok(self)
    }

    /// Set the minimum confidence used by the legacy-compatible projection.
    pub const fn with_minimum_confidence(mut self, minimum: Confidence) -> Self {
        self.minimum_confidence = minimum;
        self
    }

    /// Set the maximum accepted UTF-8 input length in bytes.
    pub fn with_max_input_bytes(mut self, maximum: usize) -> Result<Self, AnalysisRequestError> {
        ensure_nonzero(maximum, LimitDimension::InputBytes)?;
        self.max_input_bytes = maximum;
        Ok(self)
    }

    /// Set the maximum candidate count across all selected recognizers.
    pub fn with_max_candidates(mut self, maximum: usize) -> Result<Self, AnalysisRequestError> {
        ensure_nonzero(maximum, LimitDimension::Candidates)?;
        self.max_candidates = maximum;
        Ok(self)
    }

    /// Set the maximum detailed issues retained in the report.
    pub fn with_max_issues(mut self, maximum: usize) -> Result<Self, AnalysisRequestError> {
        ensure_nonzero(maximum, LimitDimension::Issues)?;
        self.max_issues = maximum;
        Ok(self)
    }

    /// Requested entity allowlist. Empty means all declared entities.
    pub fn entities(&self) -> &[EntityId] {
        &self.entities
    }

    /// Explicit recognizer allowlist. Empty means default-enabled recognizers.
    pub fn recognizers(&self) -> &[RecognizerId] {
        &self.recognizers
    }

    /// Requested locale or country identifier.
    pub fn locale(&self) -> Option<&MetadataId> {
        self.locale.as_ref()
    }

    /// Capabilities available for this request.
    pub fn available_capabilities(&self) -> &[MetadataId] {
        &self.available_capabilities
    }

    /// Minimum confidence for the compatibility projection.
    pub const fn minimum_confidence(&self) -> Confidence {
        self.minimum_confidence
    }

    /// Maximum accepted input length in bytes.
    pub const fn max_input_bytes(&self) -> usize {
        self.max_input_bytes
    }

    /// Maximum candidate count across selected recognizers.
    pub const fn max_candidates(&self) -> usize {
        self.max_candidates
    }

    /// Maximum retained issue details.
    pub const fn max_issues(&self) -> usize {
        self.max_issues
    }

    /// Whether authoritative recognizer metadata is selected and executable.
    pub fn selects(&self, metadata: &RecognizerMetadata) -> bool {
        if self.recognizers.is_empty() {
            if !metadata.default_enabled() {
                return false;
            }
        } else if !self.recognizers.iter().any(|id| id == metadata.id()) {
            return false;
        }

        if !self.entities.is_empty()
            && !metadata
                .supported_entities()
                .iter()
                .any(|entity| self.entities.contains(entity))
        {
            return false;
        }

        if let Some(locale) = &self.locale {
            let supported = metadata.supported_locales();
            if !supported.is_empty() && !supported.contains(locale) {
                return false;
            }
        }

        metadata
            .required_capabilities()
            .iter()
            .all(|capability| self.available_capabilities.contains(capability))
    }
}

/// Failure to construct a deterministic analysis request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalysisRequestError {
    /// An entity appears more than once.
    DuplicateEntity { entity: EntityId },
    /// A recognizer appears more than once.
    DuplicateRecognizer { recognizer: RecognizerId },
    /// A capability appears more than once.
    DuplicateCapability { capability: MetadataId },
    /// A resource limit must be greater than zero.
    ZeroLimit { dimension: LimitDimension },
}

impl fmt::Display for AnalysisRequestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateEntity { entity } => write!(formatter, "duplicate entity {entity}"),
            Self::DuplicateRecognizer { recognizer } => {
                write!(formatter, "duplicate recognizer {recognizer}")
            }
            Self::DuplicateCapability { capability } => {
                write!(formatter, "duplicate capability {capability}")
            }
            Self::ZeroLimit { dimension } => {
                write!(formatter, "analysis limit {dimension} must be greater than zero")
            }
        }
    }
}

impl std::error::Error for AnalysisRequestError {}

/// Resource dimension associated with an invalid request limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum LimitDimension {
    /// Original UTF-8 bytes.
    InputBytes,
    /// Emitted candidates.
    Candidates,
    /// Retained issue details.
    Issues,
}

impl fmt::Display for LimitDimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputBytes => formatter.write_str("input_bytes"),
            Self::Candidates => formatter.write_str("candidates"),
            Self::Issues => formatter.write_str("issues"),
        }
    }
}

/// Fatal failure before recognizer execution can produce a report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalysisExecutionError {
    /// The original document exceeds the request's configured limit.
    InputTooLarge { actual: usize, maximum: usize },
}

impl fmt::Display for AnalysisExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputTooLarge { actual, maximum } => write!(
                formatter,
                "document length {actual} bytes exceeds request limit {maximum} bytes"
            ),
        }
    }
}

impl std::error::Error for AnalysisExecutionError {}

fn ensure_unique_entities(values: &[EntityId]) -> Result<(), AnalysisRequestError> {
    let mut seen = HashSet::with_capacity(values.len());
    for value in values {
        if !seen.insert(value) {
            return Err(AnalysisRequestError::DuplicateEntity {
                entity: value.clone(),
            });
        }
    }
    Ok(())
}

fn ensure_unique_recognizers(values: &[RecognizerId]) -> Result<(), AnalysisRequestError> {
    let mut seen = HashSet::with_capacity(values.len());
    for value in values {
        if !seen.insert(value) {
            return Err(AnalysisRequestError::DuplicateRecognizer {
                recognizer: value.clone(),
            });
        }
    }
    Ok(())
}

fn ensure_unique_capabilities(values: &[MetadataId]) -> Result<(), AnalysisRequestError> {
    let mut seen = HashSet::with_capacity(values.len());
    for value in values {
        if !seen.insert(value) {
            return Err(AnalysisRequestError::DuplicateCapability {
                capability: value.clone(),
            });
        }
    }
    Ok(())
}

fn ensure_nonzero(value: usize, dimension: LimitDimension) -> Result<(), AnalysisRequestError> {
    if value == 0 {
        Err(AnalysisRequestError::ZeroLimit { dimension })
    } else {
        Ok(())
    }
}
