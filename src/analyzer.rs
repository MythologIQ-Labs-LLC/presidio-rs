//! The analyzer engine: runs a [`RecognizerRegistry`], applies validators and
//! context scoring, resolves overlaps, and filters by a score threshold.

use std::cmp::Ordering;

use crate::context;
use crate::entity::EntityType;
use crate::recognizer::{PatternRecognizer, RecognizerMetadata};
use crate::registry::RecognizerRegistry;
use crate::report::{AnalysisError, AnalysisIssue, AnalysisReport, CandidateIssue};
use crate::result::RecognizerResult;
use crate::types::{Confidence, EntityId, Evidence, Finding, Span};

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

    /// Mutable access to the registry.
    pub fn registry_mut(&mut self) -> &mut RecognizerRegistry {
        &mut self.registry
    }

    /// Register an additional custom recognizer.
    pub fn add_recognizer(&mut self, recognizer: PatternRecognizer) {
        self.registry.add(recognizer);
    }

    /// Detect PII and apply the legacy overlap behavior.
    pub fn analyze(&self, text: &str, entities: Option<&[EntityType]>) -> Vec<RecognizerResult> {
        let mut out = Vec::new();
        for recognizer in self.registry.recognizers() {
            if entities.is_some_and(|filter| !filter.contains(&recognizer.entity_type)) {
                continue;
            }
            run_recognizer(text, recognizer, &mut out);
        }
        dedupe_overlaps(&mut out);
        out.retain(|result| result.score >= self.score_threshold);
        out.sort_by_key(|result| result.start);
        out
    }

    /// Analyze text into an unresolved, evidence-bearing report.
    pub fn analyze_report(
        &self,
        text: &str,
        entities: Option<&[EntityType]>,
    ) -> Result<AnalysisReport, AnalysisError> {
        let threshold =
            Confidence::new(self.score_threshold).map_err(AnalysisError::InvalidThreshold)?;
        let mut candidates = Vec::new();
        let mut issues = Vec::new();

        for (recognizer, metadata) in self.registry.entries() {
            if entities.is_some_and(|filter| !filter.contains(&recognizer.entity_type)) {
                continue;
            }
            run_recognizer_report(
                text,
                recognizer,
                metadata,
                threshold,
                &mut candidates,
                &mut issues,
            );
        }

        Ok(AnalysisReport::new(threshold, candidates, issues))
    }
}

fn run_recognizer(text: &str, recognizer: &PatternRecognizer, out: &mut Vec<RecognizerResult>) {
    for pattern in &recognizer.patterns {
        for matched in pattern.regex.find_iter(text) {
            let mut score = pattern.base_score;
            if let Some(validate) = recognizer.validator {
                match validate(matched.as_str()) {
                    Some(true) => score = 1.0,
                    Some(false) => continue,
                    None => {}
                }
            }
            score = context::enhance(
                text,
                matched.start(),
                matched.end(),
                score,
                recognizer.context,
            );
            out.push(RecognizerResult::new(
                recognizer.entity_type,
                matched.start(),
                matched.end(),
                score,
            ));
        }
    }
}

fn run_recognizer_report(
    text: &str,
    recognizer: &PatternRecognizer,
    metadata: &RecognizerMetadata,
    threshold: Confidence,
    candidates: &mut Vec<Finding>,
    issues: &mut Vec<AnalysisIssue>,
) {
    for pattern in &recognizer.patterns {
        for matched in pattern.regex.find_iter(text) {
            let mut score = pattern.base_score;
            let mut evidence = vec![Evidence::Pattern {
                pattern_id: pattern.name.to_owned(),
            }];

            if let Some(validate) = recognizer.validator {
                match validate(matched.as_str()) {
                    Some(true) => {
                        score = 1.0;
                        evidence.push(Evidence::Validator {
                            validator_id: format!("{}.validator", metadata.id()),
                            accepted: true,
                        });
                    }
                    Some(false) => continue,
                    None => {}
                }
            }

            score = context::enhance(
                text,
                matched.start(),
                matched.end(),
                score,
                recognizer.context,
            );

            let span = match Span::new(matched.start(), matched.end()).and_then(|span| {
                span.validate_for(text)?;
                Ok(span)
            }) {
                Ok(span) => span,
                Err(error) => {
                    issues.push(AnalysisIssue::new(
                        metadata.id().clone(),
                        pattern.name,
                        CandidateIssue::InvalidSpan(error),
                    ));
                    continue;
                }
            };

            let confidence = match Confidence::new(score) {
                Ok(confidence) => confidence,
                Err(error) => {
                    issues.push(AnalysisIssue::new(
                        metadata.id().clone(),
                        pattern.name,
                        CandidateIssue::InvalidConfidence(error),
                    ));
                    continue;
                }
            };

            if confidence < threshold {
                continue;
            }

            let mut finding = Finding::new(
                EntityId::from(recognizer.entity_type),
                span,
                confidence,
                metadata.id().clone(),
            )
            .with_evidence(evidence);
            if let Some(version) = metadata.version() {
                finding = finding.with_recognizer_version(version);
            }
            candidates.push(finding);
        }
    }
}

fn dedupe_overlaps(results: &mut Vec<RecognizerResult>) {
    results.sort_by(|left, right| {
        left.start.cmp(&right.start).then_with(|| {
            right
                .score
                .partial_cmp(&left.score)
                .unwrap_or(Ordering::Equal)
        })
    });
    let mut kept: Vec<RecognizerResult> = Vec::with_capacity(results.len());
    for result in results.drain(..) {
        match kept.last_mut() {
            Some(last) if result.start < last.end => {
                if result.score > last.score {
                    *last = result;
                }
            }
            _ => kept.push(result),
        }
    }
    *results = kept;
}
