//! The analyzer engine: runs recognizers, applies validators and context
//! scoring, removes overlaps, and filters by a score threshold.

use std::cmp::Ordering;

use crate::context;
use crate::entity::EntityType;
use crate::recognizer::{Pattern, PatternRecognizer};
use crate::result::RecognizerResult;
use crate::validators;

/// Default minimum score a result must reach to be returned.
pub const DEFAULT_SCORE_THRESHOLD: f32 = 0.3;

/// Runs a set of [`PatternRecognizer`]s over text and returns scored spans.
pub struct AnalyzerEngine {
    recognizers: Vec<PatternRecognizer>,
    score_threshold: f32,
}

impl Default for AnalyzerEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyzerEngine {
    /// Build an engine with the predefined recognizers.
    pub fn new() -> Self {
        Self {
            recognizers: predefined(),
            score_threshold: DEFAULT_SCORE_THRESHOLD,
        }
    }

    /// Override the score threshold (builder style).
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.score_threshold = threshold;
        self
    }

    /// Register an additional custom recognizer.
    pub fn add_recognizer(&mut self, recognizer: PatternRecognizer) {
        self.recognizers.push(recognizer);
    }

    /// Detect PII in `text`. When `entities` is `Some`, only those entity types
    /// are scanned for; `None` scans for all.
    pub fn analyze(&self, text: &str, entities: Option<&[EntityType]>) -> Vec<RecognizerResult> {
        let mut out: Vec<RecognizerResult> = Vec::new();
        for rec in &self.recognizers {
            if entities.is_some_and(|f| !f.contains(&rec.entity_type)) {
                continue;
            }
            self.run_recognizer(text, rec, &mut out);
        }
        dedupe_overlaps(&mut out);
        out.retain(|r| r.score >= self.score_threshold);
        out.sort_by_key(|r| r.start);
        out
    }

    fn run_recognizer(&self, text: &str, rec: &PatternRecognizer, out: &mut Vec<RecognizerResult>) {
        for pat in &rec.patterns {
            for m in pat.regex.find_iter(text) {
                let mut score = pat.base_score;
                if let Some(validate) = rec.validator {
                    match validate(m.as_str()) {
                        Some(true) => score = 1.0,
                        Some(false) => continue,
                        None => {}
                    }
                }
                score = context::enhance(text, m.start(), m.end(), score, rec.context);
                out.push(RecognizerResult::new(
                    rec.entity_type,
                    m.start(),
                    m.end(),
                    score,
                ));
            }
        }
    }
}

/// Keep the highest-scoring result among overlapping spans.
fn dedupe_overlaps(results: &mut Vec<RecognizerResult>) {
    results.sort_by(|a, b| {
        a.start
            .cmp(&b.start)
            .then_with(|| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal))
    });
    let mut kept: Vec<RecognizerResult> = Vec::with_capacity(results.len());
    for r in results.drain(..) {
        match kept.last_mut() {
            Some(last) if r.start < last.end => {
                if r.score > last.score {
                    *last = r;
                }
            }
            _ => kept.push(r),
        }
    }
    *results = kept;
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
            patterns: vec![Pattern::new("iban", r"\b[A-Z]{2}\d{2}[A-Z0-9]{11,30}\b", 0.3)],
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
            patterns: vec![Pattern::new("itin", r"\b9\d{2}[-\s]?[7-9]\d[-\s]?\d{4}\b", 0.3)],
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
