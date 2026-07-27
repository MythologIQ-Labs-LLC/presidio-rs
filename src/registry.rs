//! Recognizer registry and authoritative registration metadata.

use core::fmt;

use crate::entity::EntityType;
use crate::metadata::{RecognitionMechanism, RecognizerMetadata};
use crate::recognizer::{
    Pattern, PatternRecognizer, PatternRecognizerRegistrationError, PatternValidationError,
};
use crate::types::{EntityId, MetadataId, RecognizerId};
use crate::validators;

/// Holds the pattern recognizers an [`crate::AnalyzerEngine`] runs.
///
/// The legacy [`RecognizerRegistry::add`] path keeps provenance unknown for
/// compatibility. [`RecognizerRegistry::add_with_metadata`] validates and
/// records authoritative recognizer metadata.
pub struct RecognizerRegistry {
    recognizers: Vec<PatternRecognizer>,
    metadata: Vec<Option<RecognizerMetadata>>,
}

impl Default for RecognizerRegistry {
    fn default() -> Self {
        Self::with_predefined()
    }
}

impl RecognizerRegistry {
    /// An empty registry. Callers may use legacy or metadata-backed registration.
    pub fn empty() -> Self {
        Self {
            recognizers: Vec::new(),
            metadata: Vec::new(),
        }
    }

    /// A registry preloaded with metadata-backed built-in recognizers.
    pub fn with_predefined() -> Self {
        let mut registry = Self::empty();
        for (metadata, recognizer) in predefined() {
            registry
                .add_with_metadata(metadata, recognizer)
                .expect("built-in recognizer metadata must be valid");
        }
        registry
    }

    /// Register through the legacy compatibility path.
    ///
    /// This preserves existing source behavior but does not claim recognizer
    /// identity or version in reports.
    pub fn add(&mut self, recognizer: PatternRecognizer) -> &mut Self {
        self.recognizers.push(recognizer);
        self.metadata.push(None);
        self
    }

    /// Validate and register a pattern recognizer with authoritative metadata.
    pub fn add_with_metadata(
        &mut self,
        metadata: RecognizerMetadata,
        recognizer: PatternRecognizer,
    ) -> Result<&mut Self, RecognizerRegistryError> {
        recognizer
            .validate_with_metadata(&metadata)
            .map_err(RecognizerRegistryError::InvalidRecognizer)?;

        if self
            .metadata
            .iter()
            .flatten()
            .any(|existing| existing.id() == metadata.id())
        {
            return Err(RecognizerRegistryError::DuplicateRecognizerId {
                id: metadata.id().clone(),
            });
        }

        self.recognizers.push(recognizer);
        self.metadata.push(Some(metadata));
        Ok(self)
    }

    /// The registered pattern recognizers, including legacy registrations.
    pub fn recognizers(&self) -> &[PatternRecognizer] {
        &self.recognizers
    }

    /// Authoritative metadata for a recognizer index, when registered strictly.
    pub fn metadata_at(&self, index: usize) -> Option<&RecognizerMetadata> {
        self.metadata.get(index).and_then(Option::as_ref)
    }

    /// Iterate authoritative metadata without exposing legacy placeholder values.
    pub fn metadata(&self) -> impl Iterator<Item = &RecognizerMetadata> {
        self.metadata.iter().flatten()
    }

    /// Number of registered recognizers across both registration paths.
    pub fn len(&self) -> usize {
        self.recognizers.len()
    }

    /// Whether the registry holds no recognizers.
    pub fn is_empty(&self) -> bool {
        self.recognizers.is_empty()
    }
}

/// Failure to register a recognizer in the registry.
#[derive(Debug)]
pub enum RecognizerRegistryError {
    /// The recognizer and its metadata are internally inconsistent.
    InvalidRecognizer(PatternRecognizerRegistrationError),
    /// A stable recognizer ID is already registered.
    DuplicateRecognizerId { id: RecognizerId },
}

impl fmt::Display for RecognizerRegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRecognizer(error) => write!(formatter, "invalid recognizer: {error}"),
            Self::DuplicateRecognizerId { id } => {
                write!(formatter, "recognizer ID {id} is already registered")
            }
        }
    }
}

impl std::error::Error for RecognizerRegistryError {}

fn predefined() -> Vec<(RecognizerMetadata, PatternRecognizer)> {
    vec![
        registered(
            "builtin.credit-card",
            EntityType::CreditCard,
            RecognitionMechanism::PatternWithValidation,
            &[],
            vec![strict_pattern("cc", r"\b(?:\d{4}[-\s]?){3}\d{4}\b", 0.3)],
            &["credit", "card", "visa", "mastercard", "amex"],
            Some(validators::luhn),
        ),
        registered(
            "builtin.us-ssn",
            EntityType::Ssn,
            RecognitionMechanism::Pattern,
            &["en-US"],
            vec![strict_pattern(
                "ssn",
                r"\b\d{3}[-\s]?\d{2}[-\s]?\d{4}\b",
                0.3,
            )],
            &["ssn", "social security"],
            None,
        ),
        registered(
            "builtin.email",
            EntityType::Email,
            RecognitionMechanism::Pattern,
            &[],
            vec![strict_pattern(
                "email",
                r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b",
                0.7,
            )],
            &["email", "e-mail", "contact"],
            None,
        ),
        registered(
            "builtin.us-phone-number",
            EntityType::PhoneNumber,
            RecognitionMechanism::Pattern,
            &["en-US"],
            vec![strict_pattern(
                "phone",
                r"\b(?:\+?1[-.\s]?)?\(?[0-9]{3}\)?[-.\s]?[0-9]{3}[-.\s]?[0-9]{4}\b",
                0.4,
            )],
            &["phone", "tel", "call", "mobile", "cell"],
            None,
        ),
        registered(
            "builtin.ip-address",
            EntityType::IpAddress,
            RecognitionMechanism::Pattern,
            &[],
            vec![
                strict_pattern("ipv4", r"\b(?:\d{1,3}\.){3}\d{1,3}\b", 0.6),
                strict_pattern("ipv6", r"\b(?:[a-fA-F0-9]{1,4}:){7}[a-fA-F0-9]{1,4}\b", 0.6),
            ],
            &["ip", "address"],
            None,
        ),
        registered(
            "builtin.mac-address",
            EntityType::MacAddress,
            RecognitionMechanism::Pattern,
            &[],
            vec![strict_pattern(
                "mac",
                r"\b(?:[a-fA-F0-9]{2}[:-]){5}[a-fA-F0-9]{2}\b",
                0.7,
            )],
            &["mac", "hardware"],
            None,
        ),
        registered(
            "builtin.iban",
            EntityType::IbanCode,
            RecognitionMechanism::PatternWithValidation,
            &[],
            vec![strict_pattern(
                "iban",
                r"\b[A-Z]{2}\d{2}[A-Z0-9]{11,30}\b",
                0.3,
            )],
            &["iban", "bank", "account"],
            Some(validators::iban_mod97),
        ),
        registered(
            "builtin.crypto-wallet",
            EntityType::CryptoWallet,
            RecognitionMechanism::Pattern,
            &[],
            vec![
                strict_pattern("eth", r"\b0x[a-fA-F0-9]{40}\b", 0.6),
                strict_pattern(
                    "btc",
                    r"\b(?:bc1[a-z0-9]{25,90}|[13][a-km-zA-HJ-NP-Z1-9]{25,34})\b",
                    0.5,
                ),
            ],
            &["wallet", "crypto", "bitcoin", "ethereum", "btc", "eth"],
            None,
        ),
        registered(
            "builtin.url",
            EntityType::Url,
            RecognitionMechanism::Pattern,
            &[],
            vec![strict_pattern("url", r"https?://[^\s]+", 0.5)],
            &[],
            None,
        ),
        registered(
            "builtin.us-itin",
            EntityType::UsItin,
            RecognitionMechanism::Pattern,
            &["en-US"],
            vec![strict_pattern(
                "itin",
                r"\b9\d{2}[-\s]?[7-9]\d[-\s]?\d{4}\b",
                0.3,
            )],
            &["itin", "taxpayer"],
            None,
        ),
        registered(
            "builtin.api-key",
            EntityType::ApiKey,
            RecognitionMechanism::Pattern,
            &[],
            vec![
                strict_pattern("openai", r"\bsk-[a-zA-Z0-9]{20,}\b", 0.9),
                strict_pattern("github", r"\bghp_[a-zA-Z0-9]{36}\b", 0.9),
                strict_pattern("slack", r"\bxox[baprs]-[a-zA-Z0-9-]{10,}\b", 0.9),
            ],
            &["api", "key", "token", "secret"],
            None,
        ),
    ]
}

fn registered(
    id: &str,
    entity_type: EntityType,
    mechanism: RecognitionMechanism,
    locales: &[&str],
    patterns: Vec<Pattern>,
    context: &'static [&'static str],
    validator: Option<crate::recognizer::Validator>,
) -> (RecognizerMetadata, PatternRecognizer) {
    let metadata = RecognizerMetadata::new(
        RecognizerId::new(id).expect("built-in recognizer ID must be valid"),
        MetadataId::new(env!("CARGO_PKG_VERSION")).expect("crate version must be valid metadata"),
        [EntityId::from(entity_type)],
        mechanism,
    )
    .expect("built-in recognizer metadata must be valid")
    .with_supported_locales(
        locales
            .iter()
            .map(|locale| MetadataId::new(*locale).expect("built-in locale must be valid")),
    )
    .expect("built-in locales must be unique")
    .with_default_enabled(true)
    .with_attribution(
        MetadataId::new("prior-art.microsoft-presidio")
            .expect("built-in attribution must be valid"),
    );

    (
        metadata,
        PatternRecognizer {
            entity_type,
            patterns,
            context,
            validator,
        },
    )
}

fn strict_pattern(name: &'static str, regex: &str, score: f32) -> Pattern {
    Pattern::try_new(name, regex, score).unwrap_or_else(|error: PatternValidationError| {
        panic!("invalid built-in pattern {name}: {error}")
    })
}
