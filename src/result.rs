//! Detection results.

use crate::entity::EntityType;

/// A detected PII span: `text[start..end]` is the matched substring, `score`
/// is the confidence in `[0.0, 1.0]`.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RecognizerResult {
    pub entity_type: EntityType,
    pub start: usize,
    pub end: usize,
    pub score: f32,
}

impl RecognizerResult {
    pub fn new(entity_type: EntityType, start: usize, end: usize, score: f32) -> Self {
        Self {
            entity_type,
            start,
            end,
            score,
        }
    }
}
