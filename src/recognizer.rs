//! Pattern recognizers and their validated registration contract.

use core::fmt;
use std::collections::HashMap;

use regex::Regex;

use crate::entity::EntityType;
use crate::metadata::{RecognitionMechanism, RecognizerMetadata};
use crate::types::{Confidence, ConfidenceError, EntityId, IdentifierError, MetadataId};

const MAX_CONTEXT_TERM_LEN: usize = 128;

/// A checksum validator. See [`crate::validators`] for the return convention.
pub type Validator = fn(&str) -> Option<bool>;

/// A named regex pattern with a base confidence score in `[0.0, 1.0]`.
pub struct Pattern {
    pub name: &'static str,
    pub regex: Regex,
    pub base_score: f32,
}

impl Pattern {
    /// Construct a pattern through the legacy compatibility path.
    ///
    /// This preserves the existing API and only rejects invalid regular
    /// expressions. Use [`Pattern::try_new`] for strict construction.
    pub fn new(name: &'static str, regex: &str, base_score: f32) -> Self {
        Self {
            name,
            regex: Regex::new(regex).expect("predefined pattern must compile"),
            base_score,
        }
    }

    /// Construct a pattern with identifier, score, regex, and empty-match validation.
    pub fn try_new(
        name: &'static str,
        regex: &str,
        base_score: f32,
    ) -> Result<Self, PatternValidationError> {
        let pattern = Self {
            name,
            regex: Regex::new(regex).map_err(PatternValidationError::InvalidRegex)?,
            base_score,
        };
        pattern.validate()?;
        Ok(pattern)
    }

    /// Validate a pattern created through either constructor.
    pub fn validate(&self) -> Result<(), PatternValidationError> {
        MetadataId::new(self.name).map_err(PatternValidationError::InvalidName)?;
        Confidence::new(self.base_score).map_err(PatternValidationError::InvalidBaseScore)?;
        if self.regex.find("").is_some() {
            return Err(PatternValidationError::MatchesEmptyInput);
        }
        Ok(())
    }
}

/// Failure to construct or validate a pattern safely.
#[derive(Debug)]
pub enum PatternValidationError {
    /// The pattern name is not a bounded metadata identifier.
    InvalidName(IdentifierError),
    /// The regular expression did not compile.
    InvalidRegex(regex::Error),
    /// The configured base score is invalid.
    InvalidBaseScore(ConfidenceError),
    /// The expression can match empty input and may produce unbounded candidates.
    MatchesEmptyInput,
}

impl fmt::Display for PatternValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(error) => write!(formatter, "invalid pattern name: {error}"),
            Self::InvalidRegex(error) => write!(formatter, "invalid pattern regex: {error}"),
            Self::InvalidBaseScore(error) => write!(formatter, "invalid pattern score: {error}"),
            Self::MatchesEmptyInput => formatter.write_str("pattern must not match empty input"),
        }
    }
}

impl std::error::Error for PatternValidationError {}

/// A recognizer for one [`EntityType`]: one or more regex patterns, an optional
/// checksum validator, and context words that boost the score of nearby matches.
pub struct PatternRecognizer {
    pub entity_type: EntityType,
    pub patterns: Vec<Pattern>,
    pub context: &'static [&'static str],
    pub validator: Option<Validator>,
}

impl PatternRecognizer {
    /// Validate this recognizer against authoritative metadata before registration.
    pub fn validate_with_metadata(
        &self,
        metadata: &RecognizerMetadata,
    ) -> Result<(), PatternRecognizerRegistrationError> {
        let entity = EntityId::from(self.entity_type);
        if metadata.supported_entities() != [entity.clone()] {
            return Err(PatternRecognizerRegistrationError::EntityMismatch {
                recognizer_entity: entity,
                declared_entities: metadata.supported_entities().to_vec(),
            });
        }

        let expected_mechanism = if self.validator.is_some() {
            RecognitionMechanism::PatternWithValidation
        } else {
            RecognitionMechanism::Pattern
        };
        if metadata.mechanism() != expected_mechanism {
            return Err(PatternRecognizerRegistrationError::MechanismMismatch {
                expected: expected_mechanism,
                actual: metadata.mechanism(),
            });
        }

        if self.patterns.is_empty() {
            return Err(PatternRecognizerRegistrationError::NoPatterns);
        }

        let mut pattern_ids = HashMap::with_capacity(self.patterns.len());
        for (index, pattern) in self.patterns.iter().enumerate() {
            pattern.validate().map_err(|error| {
                PatternRecognizerRegistrationError::InvalidPattern { index, error }
            })?;
            let pattern_id = MetadataId::new(pattern.name)
                .expect("pattern validation already established a metadata identifier");
            if let Some(first_index) = pattern_ids.insert(pattern_id, index) {
                return Err(PatternRecognizerRegistrationError::DuplicatePatternId {
                    first_index,
                    duplicate_index: index,
                });
            }
        }

        let mut normalized_context = HashMap::with_capacity(self.context.len());
        for (index, term) in self.context.iter().enumerate() {
            validate_context_term(term).map_err(|error| {
                PatternRecognizerRegistrationError::InvalidContext { index, error }
            })?;
            let normalized = term.to_lowercase();
            if let Some(first_index) = normalized_context.insert(normalized, index) {
                return Err(PatternRecognizerRegistrationError::DuplicateContext {
                    first_index,
                    duplicate_index: index,
                });
            }
        }

        Ok(())
    }
}

/// Failure to register a pattern recognizer under authoritative metadata.
#[derive(Debug)]
pub enum PatternRecognizerRegistrationError {
    /// Pattern recognizers emit exactly one legacy entity, and metadata disagrees.
    EntityMismatch {
        recognizer_entity: EntityId,
        declared_entities: Vec<EntityId>,
    },
    /// Metadata mechanism does not match validator presence.
    MechanismMismatch {
        expected: RecognitionMechanism,
        actual: RecognitionMechanism,
    },
    /// A registered recognizer must contain at least one pattern.
    NoPatterns,
    /// A pattern failed strict validation.
    InvalidPattern {
        index: usize,
        error: PatternValidationError,
    },
    /// Two patterns use the same stable identifier.
    DuplicatePatternId {
        first_index: usize,
        duplicate_index: usize,
    },
    /// A context term is unsafe or unusable.
    InvalidContext {
        index: usize,
        error: ContextValidationError,
    },
    /// Two context terms are equivalent under current case handling.
    DuplicateContext {
        first_index: usize,
        duplicate_index: usize,
    },
}

impl fmt::Display for PatternRecognizerRegistrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EntityMismatch {
                recognizer_entity,
                declared_entities,
            } => write!(
                formatter,
                "recognizer emits {recognizer_entity}, metadata declares {declared_entities:?}"
            ),
            Self::MechanismMismatch { expected, actual } => write!(
                formatter,
                "recognizer mechanism mismatch: expected {expected:?}, found {actual:?}"
            ),
            Self::NoPatterns => formatter.write_str("pattern recognizer must contain a pattern"),
            Self::InvalidPattern { index, error } => {
                write!(formatter, "invalid pattern at index {index}: {error}")
            }
            Self::DuplicatePatternId {
                first_index,
                duplicate_index,
            } => write!(
                formatter,
                "duplicate pattern identifier at indexes {first_index} and {duplicate_index}"
            ),
            Self::InvalidContext { index, error } => {
                write!(formatter, "invalid context term at index {index}: {error}")
            }
            Self::DuplicateContext {
                first_index,
                duplicate_index,
            } => write!(
                formatter,
                "duplicate context terms at indexes {first_index} and {duplicate_index}"
            ),
        }
    }
}

impl std::error::Error for PatternRecognizerRegistrationError {}

/// Failure to validate a context term.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextValidationError {
    /// Empty or whitespace-only terms cannot affect context scoring predictably.
    Empty,
    /// Terms are bounded to prevent oversized configuration and diagnostics.
    TooLong { actual: usize, maximum: usize },
    /// Control characters are not valid context metadata.
    ControlCharacter,
}

impl fmt::Display for ContextValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("context term must not be empty"),
            Self::TooLong { actual, maximum } => write!(
                formatter,
                "context term length {actual} exceeds maximum {maximum}"
            ),
            Self::ControlCharacter => {
                formatter.write_str("context term must not contain control characters")
            }
        }
    }
}

impl std::error::Error for ContextValidationError {}

fn validate_context_term(term: &str) -> Result<(), ContextValidationError> {
    if term.trim().is_empty() {
        return Err(ContextValidationError::Empty);
    }
    if term.len() > MAX_CONTEXT_TERM_LEN {
        return Err(ContextValidationError::TooLong {
            actual: term.len(),
            maximum: MAX_CONTEXT_TERM_LEN,
        });
    }
    if term.chars().any(char::is_control) {
        return Err(ContextValidationError::ControlCharacter);
    }
    Ok(())
}
