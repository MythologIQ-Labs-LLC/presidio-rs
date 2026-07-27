//! The analyzer engine: runs a [`RecognizerRegistry`], applies validators and
//! context scoring, resolves overlaps, and filters by a score threshold.
//!
//! Mirrors Presidio's `AnalyzerEngine`.

use std::cmp::Ordering;

use crate::context;
use crate::entity::EntityType;
use crate::recognizer::PatternRecognizer;
use crate::registry::RecognizerRegistry;
use crate::report::{AnalysisIssue, AnalysisOptions, AnalysisReport, AnalysisStatus};
use crate::result::RecognizerResult;
use crate::types::{Confidence, EntityId, Evidence, Finding, MetadataId, Span};

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
        let collection = self.collect_raw_candidates(text, entities, None);
        self.project_legacy_results(&collection.candidates)
    }

    /// Analyze text while preserving validated candidates before legacy policy.
    pub fn analyze_report(&self, text: &str, entities: Option<&[EntityType]>) -> AnalysisReport {
        self.analyze_report_with_options(text, entities, AnalysisOptions::default())
    }

    /// Analyze text with explicit report resource limits.
    pub fn analyze_report_with_options(
        &self,
        text: &str,
        entities: Option<&[EntityType]>,
        options: AnalysisOptions,
    ) -> AnalysisReport {
        let collection =
            self.collect_raw_candidates(text, entities, Some(options.max_candidates()));
        let legacy_compatible_results = self.project_legacy_results(&collection.candidates);
        let mut findings = Vec::with_capacity(collection.candidates.len());
        let mut issues = Vec::new();
        let mut issue_limit_reached = false;

        for candidate in &collection.candidates {
            let span = match Span::new_for(text, candidate.start, candidate.end) {
                Ok(span) => span,
                Err(_) => {
                    push_issue(
                        &mut issues,
                        &mut issue_limit_reached,
                        options.max_issues(),
                        AnalysisIssue::InvalidSpan {
                            recognizer_index: candidate.recognizer_index,
                            pattern_index: candidate.pattern_index,
                            start: candidate.start,
                            end: candidate.end,
                        },
                    );
                    continue;
                }
            };

            let confidence = match Confidence::new(candidate.score) {
                Ok(confidence) => confidence,
                Err(_) => {
                    push_issue(
                        &mut issues,
                        &mut issue_limit_reached,
                        options.max_issues(),
                        AnalysisIssue::InvalidConfidence {
                            recognizer_index: candidate.recognizer_index,
                            pattern_index: candidate.pattern_index,
                            value: candidate.score,
                        },
                    );
                    continue;
                }
            };

            let mut evidence = Vec::new();
            match MetadataId::new(candidate.pattern_name) {
                Ok(pattern_id) => evidence.push(Evidence::Pattern { pattern_id }),
                Err(_) => push_issue(
                    &mut issues,
                    &mut issue_limit_reached,
                    options.max_issues(),
                    AnalysisIssue::InvalidPatternMetadata {
                        recognizer_index: candidate.recognizer_index,
                        pattern_index: candidate.pattern_index,
                    },
                ),
            }
            if candidate.validator_accepted {
                evidence.push(Evidence::LegacyValidatorAccepted);
            }

            findings.push(
                Finding::new(EntityId::from(candidate.entity_type), span, confidence)
                    .with_evidence(evidence),
            );
        }

        findings.sort_by(|left, right| {
            left.span()
                .start()
                .cmp(&right.span().start())
                .then_with(|| left.span().end().cmp(&right.span().end()))
        });

        AnalysisReport::new(
            env!("CARGO_PKG_VERSION"),
            findings,
            legacy_compatible_results,
            issues,
            AnalysisStatus::new(collection.limit_reached, issue_limit_reached),
        )
    }

    fn collect_raw_candidates(
        &self,
        text: &str,
        entities: Option<&[EntityType]>,
        limit: Option<usize>,
    ) -> RawCollection {
        let mut candidates = Vec::new();

        for (recognizer_index, recognizer) in self.registry.recognizers().iter().enumerate() {
            if entities.is_some_and(|filter| !filter.contains(&recognizer.entity_type)) {
                continue;
            }

            for (pattern_index, pattern) in recognizer.patterns.iter().enumerate() {
                for matched in pattern.regex.find_iter(text) {
                    let mut score = pattern.base_score;
                    let mut validator_accepted = false;
                    if let Some(validate) = recognizer.validator {
                        match validate(matched.as_str()) {
                            Some(true) => {
                                score = 1.0;
                                validator_accepted = true;
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

                    if limit.is_some_and(|maximum| candidates.len() >= maximum) {
                        return RawCollection {
                            candidates,
                            limit_reached: true,
                        };
                    }

                    candidates.push(RawCandidate {
                        entity_type: recognizer.entity_type,
                        start: matched.start(),
                        end: matched.end(),
                        score,
                        recognizer_index,
                        pattern_index,
                        pattern_name: pattern.name,
                        validator_accepted,
                    });
                }
            }
        }

        RawCollection {
            candidates,
            limit_reached: false,
        }
    }

    fn project_legacy_results(&self, candidates: &[RawCandidate]) -> Vec<RecognizerResult> {
        let mut results: Vec<RecognizerResult> = candidates
            .iter()
            .map(|candidate| {
                RecognizerResult::new(
                    candidate.entity_type,
                    candidate.start,
                    candidate.end,
                    candidate.score,
                )
            })
            .collect();
        dedupe_overlaps(&mut results);
        results.retain(|result| result.score >= self.score_threshold);
        results.sort_by_key(|result| result.start);
        results
    }
}

#[derive(Debug, Clone)]
struct RawCandidate {
    entity_type: EntityType,
    start: usize,
    end: usize,
    score: f32,
    recognizer_index: usize,
    pattern_index: usize,
    pattern_name: &'static str,
    validator_accepted: bool,
}

struct RawCollection {
    candidates: Vec<RawCandidate>,
    limit_reached: bool,
}

fn push_issue(
    issues: &mut Vec<AnalysisIssue>,
    limit_reached: &mut bool,
    maximum: usize,
    issue: AnalysisIssue,
) {
    if issues.len() < maximum {
        issues.push(issue);
    } else {
        *limit_reached = true;
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
