//! Validated value types for the next-generation analysis contract.
//!
//! These types are additive. The existing [`crate::RecognizerResult`] API
//! remains available while consumers migrate toward evidence-bearing findings.

use core::fmt;

use crate::entity::EntityType;
use crate::result::RecognizerResult;

const MAX_IDENTIFIER_LEN: usize = 128;

/// An ordered, non-empty byte range.
///
/// `Span::new` validates only structural ordering. Use [`Span::new_for`] or
/// [`Span::validate_for`] before indexing a particular UTF-8 source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    /// Construct an ordered, non-empty byte range without binding it to text.
    pub fn new(start: usize, end: usize) -> Result<Self, SpanError> {
        if start > end {
            return Err(SpanError::Reversed { start, end });
        }
        if start == end {
            return Err(SpanError::Empty { offset: start });
        }
        Ok(Self { start, end })
    }

    /// Construct and validate a span against a specific UTF-8 source.
    pub fn new_for(text: &str, start: usize, end: usize) -> Result<Self, SpanError> {
        let span = Self::new(start, end)?;
        span.validate_for(text)?;
        Ok(span)
    }

    /// Start byte offset, inclusive.
    pub const fn start(self) -> usize {
        self.start
    }

    /// End byte offset, exclusive.
    pub const fn end(self) -> usize {
        self.end
    }

    /// Span length in bytes.
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// A structurally valid span is never empty.
    pub const fn is_empty(self) -> bool {
        false
    }

    /// Validate that this span can safely index the supplied UTF-8 source.
    pub fn validate_for(self, text: &str) -> Result<(), SpanError> {
        if self.end > text.len() {
            return Err(SpanError::OutOfBounds {
                start: self.start,
                end: self.end,
                text_len: text.len(),
            });
        }
        if !text.is_char_boundary(self.start) {
            return Err(SpanError::NotCharBoundary { offset: self.start });
        }
        if !text.is_char_boundary(self.end) {
            return Err(SpanError::NotCharBoundary { offset: self.end });
        }
        Ok(())
    }

    /// Return the selected substring after source validation.
    pub fn slice(self, text: &str) -> Result<&str, SpanError> {
        self.validate_for(text)?;
        Ok(&text[self.start..self.end])
    }
}

/// Failure to construct or apply a [`Span`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpanError {
    /// The start is greater than the end.
    Reversed { start: usize, end: usize },
    /// The span contains no bytes.
    Empty { offset: usize },
    /// The span exceeds the supplied text.
    OutOfBounds {
        start: usize,
        end: usize,
        text_len: usize,
    },
    /// An offset is not on a UTF-8 character boundary.
    NotCharBoundary { offset: usize },
}

impl fmt::Display for SpanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reversed { start, end } => {
                write!(f, "span start {start} is greater than end {end}")
            }
            Self::Empty { offset } => write!(f, "span at offset {offset} is empty"),
            Self::OutOfBounds {
                start,
                end,
                text_len,
            } => write!(f, "span {start}..{end} exceeds text length {text_len}"),
            Self::NotCharBoundary { offset } => {
                write!(f, "offset {offset} is not a UTF-8 character boundary")
            }
        }
    }
}

impl std::error::Error for SpanError {}

/// A finite confidence value constrained to the inclusive range `0.0..=1.0`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Confidence(f32);

impl Confidence {
    /// Construct a validated confidence value.
    pub fn new(value: f32) -> Result<Self, ConfidenceError> {
        if !value.is_finite() {
            return Err(ConfidenceError::NotFinite);
        }
        if !(0.0..=1.0).contains(&value) {
            return Err(ConfidenceError::OutOfRange { value });
        }
        Ok(Self(value))
    }

    /// Return the inner value.
    pub const fn get(self) -> f32 {
        self.0
    }
}

/// Failure to construct a [`Confidence`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfidenceError {
    /// NaN and infinity are not valid confidence values.
    NotFinite,
    /// The value is outside `0.0..=1.0`.
    OutOfRange { value: f32 },
}

impl fmt::Display for ConfidenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFinite => f.write_str("confidence must be finite"),
            Self::OutOfRange { value } => {
                write!(f, "confidence {value} is outside 0.0..=1.0")
            }
        }
    }
}

impl std::error::Error for ConfidenceError {}

macro_rules! identifier_type {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize))]
        pub struct $name(String);

        impl $name {
            /// Construct a validated, bounded identifier.
            pub fn new(value: impl Into<String>) -> Result<Self, IdentifierError> {
                let value = value.into();
                validate_identifier(&value)?;
                Ok(Self(value))
            }

            /// Borrow the canonical identifier.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

identifier_type!(
    EntityId,
    "Stable open identifier for a detected entity category."
);
identifier_type!(RecognizerId, "Stable identifier for a recognizer.");
identifier_type!(
    MetadataId,
    "Bounded identifier for non-plaintext evidence metadata."
);

impl From<EntityType> for EntityId {
    fn from(value: EntityType) -> Self {
        Self(value.as_tag().to_owned())
    }
}

/// Failure to construct a stable identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentifierError {
    /// The identifier is empty.
    Empty,
    /// The identifier exceeds the supported byte length.
    TooLong { actual: usize, maximum: usize },
    /// The first character must be ASCII alphanumeric.
    InvalidStart { character: char },
    /// A later character is not allowed.
    InvalidCharacter { index: usize, character: char },
}

impl fmt::Display for IdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("identifier must not be empty"),
            Self::TooLong { actual, maximum } => {
                write!(f, "identifier length {actual} exceeds maximum {maximum}")
            }
            Self::InvalidStart { character } => write!(
                f,
                "identifier must start with an ASCII letter or digit, found {character:?}"
            ),
            Self::InvalidCharacter { index, character } => write!(
                f,
                "identifier contains unsupported character {character:?} at byte {index}"
            ),
        }
    }
}

impl std::error::Error for IdentifierError {}

fn validate_identifier(value: &str) -> Result<(), IdentifierError> {
    if value.is_empty() {
        return Err(IdentifierError::Empty);
    }
    if value.len() > MAX_IDENTIFIER_LEN {
        return Err(IdentifierError::TooLong {
            actual: value.len(),
            maximum: MAX_IDENTIFIER_LEN,
        });
    }

    let mut chars = value.char_indices();
    let (_, first) = chars.next().expect("non-empty identifier");
    if !first.is_ascii_alphanumeric() {
        return Err(IdentifierError::InvalidStart { character: first });
    }

    for (index, character) in chars {
        let allowed =
            character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.' | ':' | '+');
        if !allowed {
            return Err(IdentifierError::InvalidCharacter { index, character });
        }
    }
    Ok(())
}

/// Evidence explaining why a recognizer emitted a finding.
///
/// Evidence metadata is represented by validated, length-bounded identifiers.
/// Matched plaintext is intentionally not part of this contract.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(tag = "kind", rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum Evidence {
    /// Finding converted from the legacy result contract.
    LegacyResult,
    /// A legacy validator accepted a candidate, but its identity is unavailable.
    LegacyValidatorAccepted,
    /// A named pattern matched.
    Pattern { pattern_id: MetadataId },
    /// A checksum or structural validator produced a decision.
    Validator {
        validator_id: MetadataId,
        accepted: bool,
    },
    /// Nearby context influenced the score.
    Context {
        context_id: MetadataId,
        distance_bytes: usize,
        positive: bool,
    },
}

/// Evidence-bearing detection result for the target analysis contract.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Finding {
    entity: EntityId,
    span: Span,
    confidence: Confidence,
    recognizer: Option<RecognizerId>,
    evidence: Vec<Evidence>,
}

impl Finding {
    /// Construct a finding without inventing recognizer provenance.
    pub fn new(entity: EntityId, span: Span, confidence: Confidence) -> Self {
        Self {
            entity,
            span,
            confidence,
            recognizer: None,
            evidence: Vec::new(),
        }
    }

    /// Attach authoritative recognizer identity when it is actually known.
    pub fn with_recognizer(mut self, recognizer: RecognizerId) -> Self {
        self.recognizer = Some(recognizer);
        self
    }

    /// Attach non-plaintext evidence metadata.
    pub fn with_evidence(mut self, evidence: impl IntoIterator<Item = Evidence>) -> Self {
        self.evidence.extend(evidence);
        self
    }

    /// Detected entity identifier.
    pub fn entity(&self) -> &EntityId {
        &self.entity
    }

    /// Structurally validated byte span.
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Validated confidence.
    pub const fn confidence(&self) -> Confidence {
        self.confidence
    }

    /// Authoritative recognizer identifier, when known.
    pub fn recognizer(&self) -> Option<&RecognizerId> {
        self.recognizer.as_ref()
    }

    /// Non-plaintext evidence metadata.
    pub fn evidence(&self) -> &[Evidence] {
        &self.evidence
    }
}

/// Failure to convert a legacy result into a validated [`Finding`].
#[derive(Debug, Clone, PartialEq)]
pub enum FindingConversionError {
    /// The legacy span is structurally invalid.
    Span(SpanError),
    /// The legacy score is invalid.
    Confidence(ConfidenceError),
}

impl fmt::Display for FindingConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Span(error) => write!(f, "invalid legacy span: {error}"),
            Self::Confidence(error) => write!(f, "invalid legacy confidence: {error}"),
        }
    }
}

impl std::error::Error for FindingConversionError {}

impl From<SpanError> for FindingConversionError {
    fn from(value: SpanError) -> Self {
        Self::Span(value)
    }
}

impl From<ConfidenceError> for FindingConversionError {
    fn from(value: ConfidenceError) -> Self {
        Self::Confidence(value)
    }
}

impl TryFrom<&RecognizerResult> for Finding {
    type Error = FindingConversionError;

    fn try_from(value: &RecognizerResult) -> Result<Self, Self::Error> {
        let entity = EntityId::from(value.entity_type);
        let span = Span::new(value.start, value.end)?;
        let confidence = Confidence::new(value.score)?;

        Ok(Self::new(entity, span, confidence).with_evidence([Evidence::LegacyResult]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_distinguishes_structural_and_source_validation() {
        let structural = Span::new(1, 3).expect("ordered span");
        assert_eq!(structural.slice("aéz"), Ok("é"));
        assert_eq!(
            structural.validate_for("x"),
            Err(SpanError::OutOfBounds {
                start: 1,
                end: 3,
                text_len: 1,
            })
        );
        assert_eq!(
            Span::new_for("aéz", 1, 2),
            Err(SpanError::NotCharBoundary { offset: 2 })
        );
    }

    #[test]
    fn confidence_rejects_non_finite_and_out_of_range_values() {
        assert_eq!(Confidence::new(f32::NAN), Err(ConfidenceError::NotFinite));
        assert_eq!(
            Confidence::new(1.1),
            Err(ConfidenceError::OutOfRange { value: 1.1 })
        );
    }

    #[test]
    fn metadata_identifiers_are_bounded_and_not_free_form() {
        assert!(MetadataId::new("pattern.email:v1").is_ok());
        assert!(MetadataId::new("matched jane@example.com").is_err());
        assert_eq!(
            MetadataId::new("x".repeat(MAX_IDENTIFIER_LEN + 1)),
            Err(IdentifierError::TooLong {
                actual: MAX_IDENTIFIER_LEN + 1,
                maximum: MAX_IDENTIFIER_LEN,
            })
        );
    }

    #[test]
    fn legacy_conversion_keeps_provenance_unknown() {
        let legacy = RecognizerResult::new(EntityType::Email, 0, 4, 0.8);
        let finding = Finding::try_from(&legacy).expect("valid conversion");

        assert_eq!(finding.entity().as_str(), "EMAIL_ADDRESS");
        assert_eq!(finding.recognizer(), None);
        assert_eq!(finding.evidence(), &[Evidence::LegacyResult]);
    }

    #[test]
    fn legacy_conversion_rejects_invalid_primitives() {
        let reversed = RecognizerResult::new(EntityType::Email, 4, 2, 0.8);
        assert!(matches!(
            Finding::try_from(&reversed),
            Err(FindingConversionError::Span(SpanError::Reversed { .. }))
        ));

        let invalid_score = RecognizerResult::new(EntityType::Email, 0, 4, f32::NAN);
        assert_eq!(
            Finding::try_from(&invalid_score),
            Err(FindingConversionError::Confidence(
                ConfidenceError::NotFinite
            ))
        );
    }
}
