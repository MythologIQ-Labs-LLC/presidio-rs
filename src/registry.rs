//! Recognizer registry — the collection of recognizers the analyzer runs.
//!
//! Mirrors Presidio's `RecognizerRegistry`: it holds the recognizers, and the
//! [`crate::AnalyzerEngine`] iterates them on each `analyze` call. Swap or
//! extend the registry to change what gets detected.

use crate::entity::EntityType;
use crate::recognizer::{Pattern, PatternRecognizer};
use crate::validators;

/// Holds the set of [`PatternRecognizer`]s an [`crate::AnalyzerEngine`] runs.
pub struct RecognizerRegistry {
    recognizers: Vec<PatternRecognizer>,
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
        }
    }

    /// A registry preloaded with the built-in recognizers.
    pub fn with_predefined() -> Self {
        Self {
            recognizers: predefined(),
        }
    }

    /// Register a recognizer (builder-friendly, returns `&mut Self`).
    pub fn add(&mut self, recognizer: PatternRecognizer) -> &mut Self {
        self.recognizers.push(recognizer);
        self
    }

    /// The registered recognizers.
    pub fn recognizers(&self) -> &[PatternRecognizer] {
        &self.recognizers
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

/// The built-in recognizers. Format-structured entities only — the NER-only
/// trio (`Person`/`Location`/`Nrp`) is reserved for a future model backend.
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
