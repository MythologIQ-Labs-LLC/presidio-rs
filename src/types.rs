//! Validated value types for the next-generation analysis contract.
//!
//! These types are additive. The existing [`crate::RecognizerResult`] API
//! remains available while consumers migrate toward evidence-bearing findings.

use core::fmt;

use crate::entity::EntityType;
use crate::result::RecognizerResult;

const MAX_IDENTIFIER_LEN: usize = 128;

/// A validated, non-empty byte range into original UTF-8 input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    /// Construct a non-empty ordered span.
    pub fn new(start: usize, end: usize) -> Result<Self, SpanError> {
        if start > end {
            return Err(SpanError::Reversed { start, end });
        }
        if start == end {
            return Err(SpanError::Empty { offset: start });
        }
        Ok(Self { start, end })
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

    /// A validated span is never empty.
    pub const fn is_empty(self) -> bool {
        false
    }

    /// Validate that this span can safely index the supplied original text.
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

    /// Return the matched substring after validating bounds and UTF-8 boundaries.
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

/// Stable open identifier for a detected entity category.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct EntityId(String);

impl EntityId {
    /// Construct a validated entity identifier.
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

impl From<EntityType> for EntityId {
    fn from(value: EntityType) -> Self {
        Self(value.as_tag().to_owned())
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Stable identifier for the recognizer that produced a finding.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RecognizerId(String);

impl RecognizerId {
    /// Construct a validated recognizer identifier.
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

impl fmt::Display for RecognizerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "kind", rename_all = "snake_case")
)]
#[non_exhaustive]
pub enum Evidence {
    /// Finding converted from the legacy result contract.
    LegacyResult,
    /// A named pattern matched. Matched plaintext is intentionally omitted.
    Pattern { pattern_id: String },
    /// A checksum or structural validator produced a decision.
    Validator {
        validator_id: String,
        accepted: bool,
    },
    /// Nearby context influenced the score.
    Context {
        context_id: String,
        distance_bytes: usize,
        positive: bool,
    },
}

/// Evidence-bearing detection result for the target analysis contract.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Finding {
    entity: EntityId,
    span: Span,
    confidence: Confidence,
    recognizer: RecognizerId,
    recognizer_version: Option<String>,
    evidence: Vec<Evidence>,
    locale: Option<String>,
}

impl Finding {
    /// Construct a finding from validated components.
    pub fn new(
        entity: EntityId,
        span: Span,
        confidence: Confidence,
        recognizer: RecognizerId,
    ) -> Self {
        Self {
            entity,
            span,
            confidence,
            recognizer,
            recognizer_version: None,
            evidence: Vec::new(),
            locale: None,
        }
    }

    /// Attach a recognizer version.
    pub fn with_recognizer_version(mut self, version: impl Into<String>) -> Self {
        let version = version.into();
        self.recognizer_version = (!version.trim().is_empty()).then_some(version);
        self
    }

    /// Attach non-plaintext evidence.
    pub fn with_evidence(mut self, evidence: impl IntoIterator<Item = Evidence>) -> Self {
        self.evidence.extend(evidence);
        self
    }

    /// Attach locale metadata.
    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        let locale = locale.into();
        self.locale = (!locale.trim().is_empty()).then_some(locale);
        self
    }

    /// Detected entity identifier.
    pub fn entity(&self) -> &EntityId {
        &self.entity
    }

    /// Original-text byte span.
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Validated confidence.
    pub const fn confidence(&self) -> Confidence {
        self.confidence
    }

    /// Recognizer identifier.
    pub fn recognizer(&self) -> &RecognizerId {
        &self.recognizer
    }

    /// Optional recognizer version.
    pub fn recognizer_version(&self) -> Option<&str> {
        self.recognizer_version.as_deref()
    }

    /// Non-plaintext evidence.
    pub fn evidence(&self) -> &[Evidence] {
        &self.evidence
    }

    /// Optional locale metadata.
    pub fn locale(&self) -> Option<&str> {
        self.locale.as_deref()
    }
}

/// Failure to convert a legacy result into a validated [`Finding`].
#[derive(Debug, Clone, PartialEq)]
pub enum FindingConversionError {
    /// The legacy span is invalid.
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
        let recognizer = RecognizerId::new("legacy.pattern")
            .expect("the built-in legacy recognizer identifier is valid");

        Ok(Self::new(entity, span, confidence, recognizer)
            .with_recognizer_version(env!("CARGO_PKG_VERSION"))
            .with_evidence([Evidence::LegacyResult]))
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::de::Error as _;
    use serde::{Deserialize, Deserializer};

    impl<'de> Deserialize<'de> for Span {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            #[derive(Deserialize)]
            struct RawSpan {
                start: usize,
                end: usize,
            }

            let raw = RawSpan::deserialize(deserializer)?;
            Span::new(raw.start, raw.end).map_err(D::Error::custom)
        }
    }

    impl<'de> Deserialize<'de> for Confidence {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value = f32::deserialize(deserializer)?;
            Confidence::new(value).map_err(D::Error::custom)
        }
    }

    impl<'de> Deserialize<'de> for EntityId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value = String::deserialize(deserializer)?;
            EntityId::new(value).map_err(D::Error::custom)
        }
    }

    impl<'de> Deserialize<'de> for RecognizerId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value = String::deserialize(deserializer)?;
            RecognizerId::new(value).map_err(D::Error::custom)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_rejects_empty_and_reversed_ranges() {
        assert_eq!(Span::new(3, 3), Err(SpanError::Empty { offset: 3 }));
        assert_eq!(
            Span::new(4, 2),
            Err(SpanError::Reversed { start: 4, end: 2 })
        );
    }

    #[test]
    fn span_validates_utf8_boundaries() {
        let text = "aéz";
        let valid = Span::new(1, 3).expect("valid span");
        assert_eq!(valid.slice(text), Ok("é"));

        let split_codepoint = Span::new(1, 2).expect("ordered span");
        assert_eq!(
            split_codepoint.validate_for(text),
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
        assert_eq!(Confidence::new(0.75).expect("valid").get(), 0.75);
    }

    #[test]
    fn stable_identifiers_accept_namespaces_and_reject_whitespace() {
        assert_eq!(
            EntityId::new("acme.customer-id:v1")
                .expect("valid identifier")
                .as_str(),
            "acme.customer-id:v1"
        );
        assert!(RecognizerId::new("bad recognizer").is_err());
    }

    #[test]
    fn legacy_result_converts_without_changing_legacy_api() {
        let legacy = RecognizerResult::new(EntityType::Email, 0, 4, 0.8);
        let finding = Finding::try_from(&legacy).expect("valid conversion");

        assert_eq!(finding.entity().as_str(), "EMAIL_ADDRESS");
        assert_eq!(finding.span(), Span::new(0, 4).expect("valid span"));
        assert_eq!(finding.confidence().get(), 0.8);
        assert_eq!(finding.recognizer().as_str(), "legacy.pattern");
        assert_eq!(finding.evidence(), &[Evidence::LegacyResult]);
    }
}
