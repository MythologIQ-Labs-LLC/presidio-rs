//! Anonymization: transform detected spans in place.
//!
//! Mirrors Presidio's `AnonymizerEngine` + operators. Two entry points:
//! the free [`anonymize`] function (one operator for every span) and
//! [`AnonymizerEngine`] (a default operator plus per-entity overrides).

use std::collections::HashMap;

use crate::entity::EntityType;
use crate::result::RecognizerResult;
use sha2::{Digest, Sha256};

/// How to transform a detected span.
#[derive(Debug, Clone)]
pub enum Operator {
    /// Replace with `new_value`, or `<ENTITY_TAG>` when `None`.
    Replace(Option<String>),
    /// Delete the span entirely.
    Redact,
    /// Overwrite with `mask_char`, keeping the last `keep_last` characters.
    Mask { mask_char: char, keep_last: usize },
    /// Replace with a salted SHA-256 digest (deterministic pseudonym).
    Hash { salt: String },
}

/// Apply a single `op` to every span in `results`, returning the transformed
/// string. Spans are applied right-to-left so byte offsets stay valid; results
/// whose offsets are not char boundaries of `text` are skipped defensively.
pub fn anonymize(text: &str, results: &[RecognizerResult], op: &Operator) -> String {
    let mut out = text.to_string();
    for r in ordered(results) {
        apply_span(&mut out, r, op);
    }
    out
}

/// A per-entity anonymization policy: a default operator plus overrides keyed
/// by [`EntityType`]. Mirrors Presidio's `AnonymizerEngine(operators=...)`.
pub struct AnonymizerEngine {
    default: Operator,
    overrides: HashMap<EntityType, Operator>,
}

impl AnonymizerEngine {
    /// New engine that applies `default` to any entity without an override.
    pub fn new(default: Operator) -> Self {
        Self {
            default,
            overrides: HashMap::new(),
        }
    }

    /// Set the operator used for `entity` (builder style).
    pub fn with_operator(mut self, entity: EntityType, op: Operator) -> Self {
        self.overrides.insert(entity, op);
        self
    }

    /// Anonymize `text`, choosing each span's operator by entity type.
    pub fn anonymize(&self, text: &str, results: &[RecognizerResult]) -> String {
        let mut out = text.to_string();
        for r in ordered(results) {
            let op = self.overrides.get(&r.entity_type).unwrap_or(&self.default);
            apply_span(&mut out, r, op);
        }
        out
    }
}

/// Spans sorted right-to-left so in-place replacement keeps offsets valid.
fn ordered(results: &[RecognizerResult]) -> Vec<&RecognizerResult> {
    let mut spans: Vec<&RecognizerResult> = results.iter().collect();
    spans.sort_by_key(|r| std::cmp::Reverse(r.start));
    spans
}

fn apply_span(out: &mut String, r: &RecognizerResult, op: &Operator) {
    if r.end > out.len() || !out.is_char_boundary(r.start) || !out.is_char_boundary(r.end) {
        return;
    }
    let original = out[r.start..r.end].to_string();
    let replacement = render(r.entity_type, &original, op);
    out.replace_range(r.start..r.end, &replacement);
}

fn render(entity: EntityType, original: &str, op: &Operator) -> String {
    match op {
        Operator::Replace(value) => value
            .clone()
            .unwrap_or_else(|| format!("<{}>", entity.as_tag())),
        Operator::Redact => String::new(),
        Operator::Mask {
            mask_char,
            keep_last,
        } => mask(original, *mask_char, *keep_last),
        Operator::Hash { salt } => {
            let mut hasher = Sha256::new();
            hasher.update(salt.as_bytes());
            hasher.update(original.as_bytes());
            let digest = hasher.finalize();
            let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
            format!("<{}:{hex}>", entity.as_tag())
        }
    }
}

fn mask(original: &str, mask_char: char, keep_last: usize) -> String {
    let chars: Vec<char> = original.chars().collect();
    let n = chars.len();
    let keep = keep_last.min(n);
    let mut out = String::with_capacity(n);
    for (i, c) in chars.iter().enumerate() {
        if i >= n - keep {
            out.push(*c);
        } else {
            out.push(mask_char);
        }
    }
    out
}
