//! Recognizer registry — the collection of recognizers the analyzer runs.
//!
//! Mirrors Presidio's `RecognizerRegistry`: it holds the recognizers, and the
//! [`crate::AnalyzerEngine`] iterates them on each `analyze` call. Swap or
//! extend the registry to change what gets detected.

use core::fmt;

use crate::entity::EntityType;
use crate::recognizer::{Pattern, PatternRecognizer, RecognizerMetadata};
use crate::types::RecognizerId;
use crate::validators;

/// Holds the set of [`PatternRecognizer`]s an [`crate::AnalyzerEngine`] runs.
pub struct RecognizerRegistry {
    recognizers: Vec<PatternRecognizer>,
    metadata: Vec<RecognizerMetadata>,
}

impl Default for RecognizerRegistry {
    fn default() -> Self {
        Self::with_predefined()
    }
}

impl RecognizerRegistry {
    /// An empty registry — bring your own recognizers.
    pub fn empty() -> Self {
        Self {
            recognizers: Vec::new(),
            metadata: Vec::new(),
        }
    }

    /// A registry preloaded with the built-in recognizers.
    pub fn with_predefined() -> Self {
        let recognizers = predefined();
        let metadata = predefined_metadata();
        debug_assert_eq!(recognizers.len(), metadata.len());
        Self {
            recognizers,
            metadata,
        }
    }

    /// Register a recognizer using generated registry-local metadata.
    pub fn add(&mut self, recognizer: PatternRecognizer) -> &mut Self {
        let id = RecognizerId::new(format!(
            "custom.{}.{}",
            recognizer.entity_type.as_tag(),
            self.recognizers.len()
        ))
        .expect("generated recognizer identifier is valid");
        let metadata = RecognizerMetadata::new(id).with_version(env!("CARGO_PKG_VERSION"));
        self.recognizers.push(recognizer);
        self.metadata.push(metadata);
        self
    }

    /// Register a recognizer with explicit stable metadata.
    pub fn add_with_metadata(
        &mut self,
        recognizer: PatternRecognizer,
        metadata: RecognizerMetadata,
    ) -> Result<&mut Self, RegistryError> {
        if self
            .metadata
            .iter()
            .any(|existing| existing.id() == metadata.id())
        {
            return Err(RegistryError::DuplicateRecognizerId {
                id: metadata.id().clone(),
            });
        }
        self.recognizers.push(recognizer);
        self.metadata.push(metadata);
        Ok(self)
    }

    /// The registered recognizers.
    pub fn recognizers(&self) -> &[PatternRecognizer] {
        &self.recognizers
    }

    /// Metadata aligned with [`Self::recognizers`].
    pub fn metadata(&self) -> &[RecognizerMetadata] {
        &self.metadata
    }

    /// Iterate registered recognizers together with provenance metadata.
    pub fn entries(&self) -> impl Iterator<Item = (&PatternRecognizer, &RecognizerMetadata)> + '_ {
        self.recognizers.iter().zip(self.metadata.iter())
    }

    /// Number of registered recognizers.
    pub fn len(&self) -> usize {
        self.recognizers.len()
    }

    /// Whether the registry holds no recognizers.
    pub fn is_empty(&self) -> bool {
        self.recognizers.is_empty()
    }
}

/// Failure to register recognizer metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// A stable recognizer identifier is already registered.
    DuplicateRecognizerId { id: RecognizerId },
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateRecognizerId { id } => {
                write!(f, "recognizer identifier {id} is already registered")
            }
        }
    }
}

impl std::error::Error for RegistryError {}

fn metadata(id: &str) -> RecognizerMetadata {
    RecognizerMetadata::new(RecognizerId::new(id).expect("built-in recognizer identifier is valid"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

fn predefined_metadata() -> Vec<RecognizerMetadata> {
    vec![
        metadata("builtin.credit-card"),
        metadata("builtin.us-ssn"),
        metadata("builtin.email"),
        metadata("builtin.us-phone"),
        metadata("builtin.ip-address"),
        metadata("builtin.mac-address"),
        metadata("builtin.iban"),
        metadata("builtin.crypto-wallet"),
        metadata("builtin.url"),
        metadata("builtin.us-itin"),
        metadata("builtin.api-key"),
    ]
}

fn predefined() -> Vec<PatternRecognizer> {
    vec![
        PatternRecognizer {
            entity_type: EntityType::CreditCard,
            patterns: vec![Pattern::new("cc", r"\b(?:\d{4}[-\s]?){3}\d{4}\b", 0.3)],
            context: &["credit", "card", "visa", "mastercard", "amex"],
            validator: Some(validators::luhn),
        },
        PatternRecognizer {
            entity_type: EntityType::Ssn,
            patterns: vec![Pattern::new("ssn", r"\b\d{3}[-\s]?\d{2}[-\s]?\d{4}\b", 0.3)],
            context: &["ssn", "social security"],
            validator: None,
        },
        PatternRecognizer {
            entity_type: EntityType::Email,
            patterns: vec![Pattern::new(
                "email",
                r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b",
                0.7,
            )],
            context: &["email", "e-mail", "contact"],
            validator: None,
        },
        PatternRecognizer {
            entity_type: EntityType::PhoneNumber,
            patterns: vec![Pattern::new(
                "phone",
                r"\b(?:\+?1[-.\s]?)?\(?[0-9]{3}\)?[-.\s]?[0-9]{3}[-.\s]?[0-9]{4}\b",
                0.4,
            )],
            context: &["phone", "tel", "call", "mobile", "cell"],
            validator: None,
        },
        PatternRecognizer {
            entity_type: EntityType::IpAddress,
            patterns: vec![
                Pattern::new("ipv4", r"\b(?:\d{1,3}\.){3}\d{1,3}\b", 0.6),
                Pattern::new("ipv6", r"\b(?:[a-fA-F0-9]{1,4}:){7}[a-fA-F0-9]{1,4}\b", 0.6),
            ],
            context: &["ip", "address"],
            validator: None,
        },
        PatternRecognizer {
            entity_type: EntityType::MacAddress,
            patterns: vec![Pattern::new(
                "mac",
                r"\b(?:[a-fA-F0-9]{2}[:-]){5}[a-fA-F0-9]{2}\b",
                0.7,
            )],
            context: &["mac", "hardware"],
            validator: None,
        },
        PatternRecognizer {
            entity_type: EntityType::IbanCode,
            patterns: vec![Pattern::new(
                "iban",
                r"\b[A-Z]{2}\d{2}[A-Z0-9]{11,30}\b",
                0.3,
            )],
            context: &["iban", "bank", "account"],
            validator: Some(validators::iban_mod97),
        },
        PatternRecognizer {
            entity_type: EntityType::CryptoWallet,
            patterns: vec![
                Pattern::new("eth", r"\b0x[a-fA-F0-9]{40}\b", 0.6),
                Pattern::new(
                    "btc",
                    r"\b(?:bc1[a-z0-9]{25,90}|[13][a-km-zA-HJ-NP-Z1-9]{25,34})\b",
                    0.5,
                ),
            ],
            context: &["wallet", "crypto", "bitcoin", "ethereum", "btc", "eth"],
            validator: None,
        },
        PatternRecognizer {
            entity_type: EntityType::Url,
            patterns: vec![Pattern::new("url", r"https?://[^\s]+", 0.5)],
            context: &[],
            validator: None,
        },
        PatternRecognizer {
            entity_type: EntityType::UsItin,
            patterns: vec![Pattern::new(
                "itin",
                r"\b9\d{2}[-\s]?[7-9]\d[-\s]?\d{4}\b",
                0.3,
            )],
            context: &["itin", "taxpayer"],
            validator: None,
        },
        PatternRecognizer {
            entity_type: EntityType::ApiKey,
            patterns: vec![
                Pattern::new("openai", r"\bsk-[a-zA-Z0-9]{20,}\b", 0.9),
                Pattern::new("github", r"\bghp_[a-zA-Z0-9]{36}\b", 0.9),
                Pattern::new("slack", r"\bxox[baprs]-[a-zA-Z0-9-]{10,}\b", 0.9),
            ],
            context: &["api", "key", "token", "secret"],
            validator: None,
        },
    ]
}
