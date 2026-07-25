//! Context-word enhancement.
//!
//! Mirrors Presidio's `LemmaContextAwareEnhancer`: a match whose surrounding
//! window contains one of the recognizer's context words has its score boosted
//! by [`CONTEXT_SIMILARITY_FACTOR`] and floored at [`MIN_SCORE_WITH_CONTEXT`].
//! Lemmatization is approximated here by lowercase substring matching.

/// Score added when a context word is found near a match.
pub const CONTEXT_SIMILARITY_FACTOR: f32 = 0.35;
/// Score floor applied after a context hit.
pub const MIN_SCORE_WITH_CONTEXT: f32 = 0.4;
/// Characters of window inspected on each side of a match.
pub const CONTEXT_WINDOW_CHARS: usize = 40;

fn floor_boundary(s: &str, mut i: usize) -> usize {
    if i >= s.len() {
        return s.len();
    }
    while i > 0 && !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

fn ceil_boundary(s: &str, mut i: usize) -> usize {
    if i >= s.len() {
        return s.len();
    }
    while i < s.len() && !s.is_char_boundary(i) {
        i += 1;
    }
    i
}

/// Returns the (possibly boosted) score for a match at `start..end`.
///
/// `start` and `end` must be char boundaries (they are, when produced by a
/// regex match on `text`).
pub fn enhance(text: &str, start: usize, end: usize, score: f32, context: &[&str]) -> f32 {
    if context.is_empty() {
        return score;
    }
    let pre = floor_boundary(text, start.saturating_sub(CONTEXT_WINDOW_CHARS));
    let post = ceil_boundary(text, (end + CONTEXT_WINDOW_CHARS).min(text.len()));
    let prefix = &text[pre..start];
    let suffix = &text[end..post];
    let hay = format!("{prefix} {suffix}").to_lowercase();
    if context.iter().any(|w| hay.contains(w.to_lowercase().as_str())) {
        (score + CONTEXT_SIMILARITY_FACTOR).clamp(MIN_SCORE_WITH_CONTEXT, 1.0)
    } else {
        score
    }
}
