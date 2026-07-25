//! The analyzer engine: runs a [`RecognizerRegistry`], applies validators and
//! context scoring, resolves overlaps, and filters by a score threshold.
//!
//! Mirrors Presidio's `AnalyzerEngine`.

use std::cmp::Ordering;

use crate::context;
use crate::entity::EntityType;
use crate::recognizer::PatternRecognizer;
use crate::registry::RecognizerRegistry;
use crate::result::RecognizerResult;

/// Default minimum score a result must reach to be returned.
pub const DEFAULT_SCORE_THRESHOLD: f32 = 0.3;

/// Runs a [`RecognizerRegistry`] over text and returns scored PII spans.
pub struct AnalyzerEngine {
    registry: RecognizerRegistry,
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
            registry: RecognizerRegistry::with_predefined(),
            score_threshold: DEFAULT_SCORE_THRESHOLD,
        }
    }

    /// Build an engine over a caller-supplied registry.
    pub fn with_registry(registry: RecognizerRegistry) -> Self {
        Self {
            registry,
            score_threshold: DEFAULT_SCORE_THRESHOLD,
        }
    }

    /// Override the score threshold (builder style).
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.score_threshold = threshold;
        self
    }

    /// The registry this engine runs.
    pub fn registry(&self) -> &RecognizerRegistry {
        &self.registry
    }

    /// Mutable access to the registry (register custom recognizers).
    pub fn registry_mut(&mut self) -> &mut RecognizerRegistry {
        &mut self.registry
    }

    /// Register an additional custom recognizer.
    pub fn add_recognizer(&mut self, recognizer: PatternRecognizer) {
        self.registry.add(recognizer);
    }

    /// Detect PII in `text`. When `entities` is `Some`, only those entity types
    /// are scanned for; `None` scans for all.
    pub fn analyze(&self, text: &str, entities: Option<&[EntityType]>) -> Vec<RecognizerResult> {
        let mut out: Vec<RecognizerResult> = Vec::new();
        for rec in self.registry.recognizers() {
            if entities.is_some_and(|f| !f.contains(&rec.entity_type)) {
                continue;
            }
            run_recognizer(text, rec, &mut out);
        }
        dedupe_overlaps(&mut out);
        out.retain(|r| r.score >= self.score_threshold);
        out.sort_by_key(|r| r.start);
        out
    }
}

fn run_recognizer(text: &str, rec: &PatternRecognizer, out: &mut Vec<RecognizerResult>) {
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
