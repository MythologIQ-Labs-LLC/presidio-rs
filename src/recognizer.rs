//! Recognizers: regex patterns + optional checksum validator + context words.

use regex::Regex;

use crate::entity::EntityType;
use crate::types::RecognizerId;

/// A checksum validator. See [`crate::validators`] for the return convention.
pub type Validator = fn(&str) -> Option<bool>;

/// A named regex pattern with a base confidence score in `[0.0, 1.0]`.
pub struct Pattern {
    pub name: &'static str,
    pub regex: Regex,
    pub base_score: f32,
}

impl Pattern {
    pub fn new(name: &'static str, regex: &str, base_score: f32) -> Self {
        Self {
            name,
            regex: Regex::new(regex).expect("predefined pattern must compile"),
            base_score,
        }
    }
}

/// Stable metadata for a registered recognizer.
///
/// Metadata is stored by [`crate::RecognizerRegistry`] beside the existing
/// [`PatternRecognizer`] value. Keeping it out of `PatternRecognizer` preserves
/// compatibility for consumers that construct recognizers with struct literals.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RecognizerMetadata {
    id: RecognizerId,
    version: Option<String>,
}

impl RecognizerMetadata {
    /// Construct metadata with a stable recognizer identifier.
    pub fn new(id: RecognizerId) -> Self {
        Self { id, version: None }
    }

    /// Attach a recognizer implementation or rule-set version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        let version = version.into();
        self.version = (!version.trim().is_empty()).then_some(version);
        self
    }

    /// Stable recognizer identifier.
    pub fn id(&self) -> &RecognizerId {
        &self.id
    }

    /// Optional recognizer implementation or rule-set version.
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}

/// A recognizer for one [`EntityType`]: one or more regex patterns, an optional
/// checksum validator, and context words that boost the score of nearby matches.
pub struct PatternRecognizer {
    pub entity_type: EntityType,
    pub patterns: Vec<Pattern>,
    pub context: &'static [&'static str],
    pub validator: Option<Validator>,
}
