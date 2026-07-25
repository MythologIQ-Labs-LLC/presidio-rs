//! Anonymization operators: transform detected spans in place.

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

/// Apply `op` to every span in `results`, returning the transformed string.
///
/// Spans are applied right-to-left so byte offsets stay valid. Results whose
/// offsets do not land on char boundaries of `text` are skipped defensively.
pub fn anonymize(text: &str, results: &[RecognizerResult], op: &Operator) -> String {
    let mut spans: Vec<&RecognizerResult> = results.iter().collect();
    spans.sort_by_key(|r| std::cmp::Reverse(r.start)); // right-to-left

    let mut out = text.to_string();
    for r in spans {
        if r.end > out.len() || !out.is_char_boundary(r.start) || !out.is_char_boundary(r.end) {
            continue;
        }
        let original = out[r.start..r.end].to_string();
        let replacement = render(r.entity_type, &original, op);
        out.replace_range(r.start..r.end, &replacement);
    }
    out
}

fn render(entity: EntityType, original: &str, op: &Operator) -> String {
    match op {
        Operator::Replace(value) => value
            .clone()
            .unwrap_or_else(|| format!("<{}>", entity.as_tag())),
        Operator::Redact => String::new(),
        Operator::Mask { mask_char, keep_last } => mask(original, *mask_char, *keep_last),
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
