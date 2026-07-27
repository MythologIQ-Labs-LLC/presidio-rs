//! Backend-neutral recognizer metadata and provenance.

use core::fmt;
use std::collections::HashSet;

use crate::types::{EntityId, MetadataId, RecognizerId};

/// High-level mechanism used by a recognizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub enum RecognitionMechanism {
    /// Regular-expression or equivalent pattern matching.
    Pattern,
    /// Pattern matching followed by checksum or structural validation.
    PatternWithValidation,
    /// Structural parsing without a pattern-first contract.
    Structural,
    /// Dictionary, gazetteer, or exact-term matching.
    Dictionary,
    /// Statistical or machine-learning recognition.
    Semantic,
    /// A consumer-defined mechanism not represented by another variant.
    Custom,
}

/// Stable metadata describing a recognizer independently of its execution backend.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RecognizerMetadata {
    id: RecognizerId,
    version: MetadataId,
    supported_entities: Vec<EntityId>,
    supported_locales: Vec<MetadataId>,
    mechanism: RecognitionMechanism,
    required_capabilities: Vec<MetadataId>,
    default_enabled: bool,
    attribution: Option<MetadataId>,
    evaluation_receipt: Option<MetadataId>,
}

impl RecognizerMetadata {
    /// Construct the required recognizer metadata.
    pub fn new(
        id: RecognizerId,
        version: MetadataId,
        supported_entities: impl IntoIterator<Item = EntityId>,
        mechanism: RecognitionMechanism,
    ) -> Result<Self, RecognizerMetadataError> {
        let supported_entities: Vec<_> = supported_entities.into_iter().collect();
        if supported_entities.is_empty() {
            return Err(RecognizerMetadataError::NoSupportedEntities);
        }
        ensure_unique_entities(&supported_entities)?;

        Ok(Self {
            id,
            version,
            supported_entities,
            supported_locales: Vec::new(),
            mechanism,
            required_capabilities: Vec::new(),
            default_enabled: false,
            attribution: None,
            evaluation_receipt: None,
        })
    }

    /// Replace the supported locale or country identifiers.
    pub fn with_supported_locales(
        mut self,
        locales: impl IntoIterator<Item = MetadataId>,
    ) -> Result<Self, RecognizerMetadataError> {
        let locales: Vec<_> = locales.into_iter().collect();
        ensure_unique_metadata(&locales, MetadataDimension::Locale)?;
        self.supported_locales = locales;
        Ok(self)
    }

    /// Replace the capability identifiers required to execute this recognizer.
    pub fn with_required_capabilities(
        mut self,
        capabilities: impl IntoIterator<Item = MetadataId>,
    ) -> Result<Self, RecognizerMetadataError> {
        let capabilities: Vec<_> = capabilities.into_iter().collect();
        ensure_unique_metadata(&capabilities, MetadataDimension::Capability)?;
        self.required_capabilities = capabilities;
        Ok(self)
    }

    /// Set whether the recognizer is enabled by default.
    pub const fn with_default_enabled(mut self, default_enabled: bool) -> Self {
        self.default_enabled = default_enabled;
        self
    }

    /// Attach a stable source or prior-art attribution identifier.
    pub fn with_attribution(mut self, attribution: MetadataId) -> Self {
        self.attribution = Some(attribution);
        self
    }

    /// Attach the evaluation receipt that supports this recognizer version.
    pub fn with_evaluation_receipt(mut self, receipt: MetadataId) -> Self {
        self.evaluation_receipt = Some(receipt);
        self
    }

    /// Stable recognizer identifier.
    pub fn id(&self) -> &RecognizerId {
        &self.id
    }

    /// Recognizer implementation or rule-set version.
    pub fn version(&self) -> &MetadataId {
        &self.version
    }

    /// Open entity identifiers this recognizer can emit.
    pub fn supported_entities(&self) -> &[EntityId] {
        &self.supported_entities
    }

    /// Locale or country identifiers supported by this recognizer.
    pub fn supported_locales(&self) -> &[MetadataId] {
        &self.supported_locales
    }

    /// Detection mechanism.
    pub const fn mechanism(&self) -> RecognitionMechanism {
        self.mechanism
    }

    /// Capabilities that must be available before execution.
    pub fn required_capabilities(&self) -> &[MetadataId] {
        &self.required_capabilities
    }

    /// Whether the recognizer participates without explicit opt-in.
    pub const fn default_enabled(&self) -> bool {
        self.default_enabled
    }

    /// Source or prior-art attribution identifier, when declared.
    pub fn attribution(&self) -> Option<&MetadataId> {
        self.attribution.as_ref()
    }

    /// Evaluation receipt identifier, when available.
    pub fn evaluation_receipt(&self) -> Option<&MetadataId> {
        self.evaluation_receipt.as_ref()
    }
}

/// Failure to construct internally consistent recognizer metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecognizerMetadataError {
    /// A recognizer must declare at least one entity it can emit.
    NoSupportedEntities,
    /// An entity appears more than once.
    DuplicateEntity { entity: EntityId },
    /// A locale identifier appears more than once.
    DuplicateLocale { locale: MetadataId },
    /// A required capability appears more than once.
    DuplicateCapability { capability: MetadataId },
}

impl fmt::Display for RecognizerMetadataError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSupportedEntities => {
                formatter.write_str("recognizer metadata must declare a supported entity")
            }
            Self::DuplicateEntity { entity } => {
                write!(formatter, "duplicate supported entity {entity}")
            }
            Self::DuplicateLocale { locale } => {
                write!(formatter, "duplicate supported locale {locale}")
            }
            Self::DuplicateCapability { capability } => {
                write!(formatter, "duplicate required capability {capability}")
            }
        }
    }
}

impl std::error::Error for RecognizerMetadataError {}

#[derive(Debug, Clone, Copy)]
enum MetadataDimension {
    Locale,
    Capability,
}

fn ensure_unique_entities(entities: &[EntityId]) -> Result<(), RecognizerMetadataError> {
    let mut seen = HashSet::with_capacity(entities.len());
    for entity in entities {
        if !seen.insert(entity) {
            return Err(RecognizerMetadataError::DuplicateEntity {
                entity: entity.clone(),
            });
        }
    }
    Ok(())
}

fn ensure_unique_metadata(
    values: &[MetadataId],
    dimension: MetadataDimension,
) -> Result<(), RecognizerMetadataError> {
    let mut seen = HashSet::with_capacity(values.len());
    for value in values {
        if !seen.insert(value) {
            return Err(match dimension {
                MetadataDimension::Locale => RecognizerMetadataError::DuplicateLocale {
                    locale: value.clone(),
                },
                MetadataDimension::Capability => RecognizerMetadataError::DuplicateCapability {
                    capability: value.clone(),
                },
            });
        }
    }
    Ok(())
}
