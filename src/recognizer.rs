//! Recognizers: regex patterns + optional checksum validator + context words.

use crate::entity::EntityType;
use regex::Regex;

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

/// A recognizer for one [`EntityType`]: one or more regex patterns, an optional
/// checksum validator, and context words that boost the score of nearby matches.
pub struct PatternRecognizer {
    pub entity_type: EntityType,
    pub patterns: Vec<Pattern>,
    pub context: &'static [&'static str],
    pub validator: Option<Validator>,
}
