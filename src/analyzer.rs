//! Analyzer orchestration for legacy and backend-neutral recognition paths.

use std::cmp::Ordering;
use std::collections::HashSet;
use std::sync::Arc;

use crate::context;
use crate::document::{DocumentBinding, TextDocument};
use crate::entity::EntityType;
use crate::metadata::RecognizerMetadata;
use crate::recognition::{CandidateEmitter, PatternRecognizerAdapter, Recognizer};
use crate::recognizer::PatternRecognizer;
use crate::registry::{RecognizerRegistry, RecognizerRegistryError};
use crate::report::{AnalysisIssue, AnalysisOptions, AnalysisReport, AnalysisStatus};
use crate::request::{AnalysisExecutionError, AnalysisRequest};
use crate::result::RecognizerResult;
use crate::types::{Confidence, EntityId, Evidence, Finding, MetadataId, Span};

/// Default minimum score a legacy result must reach to be returned.
pub const DEFAULT_SCORE_THRESHOLD: f32 = 0.3;

/// Runs legacy pattern analysis and backend-neutral document requests.
pub struct AnalyzerEngine {
    registry: RecognizerRegistry,
    score_threshold: f32,
    backends: Vec<Arc<dyn Recognizer>>,
}

impl Default for AnalyzerEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyzerEngine {
    /// Build an engine with the predefined pattern recognizers.
    pub fn new() -> Self {
        Self {
            registry: RecognizerRegistry::with_predefined(),
            score_threshold: DEFAULT_SCORE_THRESHOLD,
            backends: Vec::new(),
        }
    }

    /// Build an engine over a caller-supplied legacy and strict pattern registry.
    pub fn with_registry(registry: RecognizerRegistry) -> Self {
        Self {
            registry,
            score_threshold: DEFAULT_SCORE_THRESHOLD,
            backends: Vec::new(),
        }
    }

    /// Override the legacy score threshold.
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.score_threshold = threshold;
        self
    }

    /// The pattern registry this engine runs.
    pub fn registry(&self) -> &RecognizerRegistry {
        &self.registry
    }

    /// Mutable access to the pattern registry.
    pub fn registry_mut(&mut self) -> &mut RecognizerRegistry {
        &mut self.registry
    }

    /// Register an additional custom pattern recognizer through the legacy path.
    pub fn add_recognizer(&mut self, recognizer: PatternRecognizer) {
        self.registry.add(recognizer);
    }

    /// Validate and register a pattern recognizer with authoritative metadata.
    pub fn add_recognizer_with_metadata(
        &mut self,
        metadata: RecognizerMetadata,
        recognizer: PatternRecognizer,
    ) -> Result<(), RecognizerRegistryError> {
        if self.backend_id_exists(metadata.id()) {
            return Err(RecognizerRegistryError::DuplicateRecognizerId {
                id: metadata.id().clone(),
            });
        }
        self.registry.add_with_metadata(metadata, recognizer)?;
        Ok(())
    }

    /// Register an owned backend-neutral recognizer.
    pub fn add_backend<R>(&mut self, recognizer: R) -> Result<(), RecognizerRegistryError>
    where
        R: Recognizer + 'static,
    {
        self.add_shared_backend(Arc::new(recognizer))
    }

    /// Register a shareable backend-neutral recognizer.
    pub fn add_shared_backend(
        &mut self,
        recognizer: Arc<dyn Recognizer>,
    ) -> Result<(), RecognizerRegistryError> {
        let id = recognizer.metadata().id();
        if self.registry_id_exists(id) || self.backend_id_exists(id) {
            return Err(RecognizerRegistryError::DuplicateRecognizerId { id: id.clone() });
        }
        self.backends.push(recognizer);
        Ok(())
    }

    /// Iterate custom backend-neutral recognizers.
    pub fn backends(&self) -> impl Iterator<Item = &dyn Recognizer> {
        self.backends.iter().map(AsRef::as_ref)
    }

    /// Detect PII through the legacy pattern API.
    pub fn analyze(&self, text: &str, entities: Option<&[EntityType]>) -> Vec<RecognizerResult> {
        let collection = self.collect_raw_candidates(text, entities, None);
        self.project_legacy_results(&collection.candidates)
    }

    /// Analyze text while preserving validated, but source-unbound, candidates.
    pub fn analyze_report(&self, text: &str, entities: Option<&[EntityType]>) -> AnalysisReport {
        self.analyze_report_with_options(text, entities, AnalysisOptions::default())
    }

    /// Analyze text with explicit report resource limits without document identity.
    pub fn analyze_report_with_options(
        &self,
        text: &str,
        entities: Option<&[EntityType]>,
        options: AnalysisOptions,
    ) -> AnalysisReport {
        self.build_report(text, None, entities, options)
    }

    /// Analyze an identity- and content-bound source document through legacy patterns.
    pub fn analyze_document(
        &self,
        document: &TextDocument<'_>,
        entities: Option<&[EntityType]>,
    ) -> AnalysisReport {
        self.analyze_document_with_options(document, entities, AnalysisOptions::default())
    }

    /// Analyze a source document with explicit legacy report resource limits.
    pub fn analyze_document_with_options(
        &self,
        document: &TextDocument<'_>,
        entities: Option<&[EntityType]>,
        options: AnalysisOptions,
    ) -> AnalysisReport {
        self.build_report(
            document.original(),
            Some(document.binding()),
            entities,
            options,
        )
    }

    /// Execute a validated backend-neutral request against an exact document.
    pub fn analyze_request(
        &self,
        document: &TextDocument<'_>,
        request: &AnalysisRequest,
    ) -> Result<AnalysisReport, AnalysisExecutionError> {
        if document.len() > request.max_input_bytes() {
            return Err(AnalysisExecutionError::InputTooLarge {
                actual: document.len(),
                maximum: request.max_input_bytes(),
            });
        }

        let mut findings = Vec::new();
        let mut recognizers = Vec::new();
        let mut seen_recognizers = HashSet::new();
        let mut issues = Vec::new();
        let mut issue_limit_reached = false;
        let mut candidate_limit_reached = false;
        let mut legacy_skipped = 0;

        for (index, pattern) in self.registry.recognizers().iter().enumerate() {
            let Some(metadata) = self.registry.metadata_at(index) else {
                legacy_skipped += 1;
                continue;
            };
            let adapter = PatternRecognizerAdapter::new(metadata, pattern);
            if !adapter.supports(request) {
                continue;
            }
            if self.execute_backend(
                &adapter,
                document,
                request,
                &mut findings,
                &mut recognizers,
                &mut seen_recognizers,
                &mut issues,
                &mut issue_limit_reached,
            ) {
                candidate_limit_reached = true;
                break;
            }
        }

        if !candidate_limit_reached {
            for backend in &self.backends {
                if !backend.supports(request) {
                    continue;
                }
                if self.execute_backend(
                    backend.as_ref(),
                    document,
                    request,
                    &mut findings,
                    &mut recognizers,
                    &mut seen_recognizers,
                    &mut issues,
                    &mut issue_limit_reached,
                ) {
                    candidate_limit_reached = true;
                    break;
                }
            }
        }

        if legacy_skipped > 0 {
            push_issue(
                &mut issues,
                &mut issue_limit_reached,
                request.max_issues(),
                AnalysisIssue::LegacyRecognizersSkipped {
                    count: legacy_skipped,
                },
            );
        }

        findings.sort_by(|left, right| {
            left.span()
                .start()
                .cmp(&right.span().start())
                .then_with(|| left.span().end().cmp(&right.span().end()))
                .then_with(|| left.entity().as_str().cmp(right.entity().as_str()))
        });

        let (legacy_compatible_results, legacy_projection_incomplete) =
            self.project_findings(&findings, request.minimum_confidence().get());

        Ok(AnalysisReport::new(
            env!("CARGO_PKG_VERSION"),
            Some(document.binding().clone()),
            findings,
            recognizers,
            legacy_compatible_results,
            issues,
            AnalysisStatus::new(
                candidate_limit_reached,
                issue_limit_reached,
                legacy_projection_incomplete,
            ),
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn execute_backend(
        &self,
        recognizer: &dyn Recognizer,
        document: &TextDocument<'_>,
        request: &AnalysisRequest,
        findings: &mut Vec<Finding>,
        recognizers: &mut Vec<RecognizerMetadata>,
        seen_recognizers: &mut HashSet<crate::RecognizerId>,
        issues: &mut Vec<AnalysisIssue>,
        issue_limit_reached: &mut bool,
    ) -> bool {
        let remaining = request.max_candidates().saturating_sub(findings.len());
        if remaining == 0 {
            return true;
        }

        let metadata = recognizer.metadata();
        if seen_recognizers.insert(metadata.id().clone()) {
            recognizers.push(metadata.clone());
        }

        let mut emitter = CandidateEmitter::new(document, metadata, remaining);
        let result = recognizer.recognize(document, request, &mut emitter);
        let limit_reached = emitter.limit_reached();
        findings.extend(emitter.into_findings());

        if let Err(error) = result {
            push_issue(
                issues,
                issue_limit_reached,
                request.max_issues(),
                AnalysisIssue::RecognitionFailed {
                    recognizer: metadata.id().clone(),
                    error,
                },
            );
        }

        limit_reached || findings.len() >= request.max_candidates()
    }

    fn build_report(
        &self,
        text: &str,
        document: Option<&DocumentBinding>,
        entities: Option<&[EntityType]>,
        options: AnalysisOptions,
    ) -> AnalysisReport {
        let collection =
            self.collect_raw_candidates(text, entities, Some(options.max_candidates()));
        let legacy_compatible_results = self.project_legacy_results(&collection.candidates);
        let recognizers = self.collect_used_metadata(&collection.candidates);
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

            let mut finding = Finding::new(EntityId::from(candidate.entity_type), span, confidence)
                .with_evidence(evidence);
            if let Some(metadata) = self.registry.metadata_at(candidate.recognizer_index) {
                finding = finding.with_recognizer(metadata.id().clone());
            }
            if let Some(binding) = document {
                finding = finding.with_document_binding(binding.clone());
            }
            findings.push(finding);
        }

        findings.sort_by(|left, right| {
            left.span()
                .start()
                .cmp(&right.span().start())
                .then_with(|| left.span().end().cmp(&right.span().end()))
        });

        AnalysisReport::new(
            env!("CARGO_PKG_VERSION"),
            document.cloned(),
            findings,
            recognizers,
            legacy_compatible_results,
            issues,
            AnalysisStatus::new(collection.limit_reached, issue_limit_reached, false),
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

    fn collect_used_metadata(&self, candidates: &[RawCandidate]) -> Vec<RecognizerMetadata> {
        let mut seen = HashSet::new();
        let mut metadata = Vec::new();
        for candidate in candidates {
            if seen.insert(candidate.recognizer_index) {
                if let Some(recognizer) = self.registry.metadata_at(candidate.recognizer_index) {
                    metadata.push(recognizer.clone());
                }
            }
        }
        metadata
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

    fn project_findings(
        &self,
        findings: &[Finding],
        minimum_confidence: f32,
    ) -> (Vec<RecognizerResult>, bool) {
        let mut incomplete = false;
        let mut results = Vec::new();
        for finding in findings {
            if let Some(entity_type) = EntityType::from_tag(finding.entity().as_str()) {
                results.push(RecognizerResult::new(
                    entity_type,
                    finding.span().start(),
                    finding.span().end(),
                    finding.confidence().get(),
                ));
            } else {
                incomplete = true;
            }
        }
        dedupe_overlaps(&mut results);
        results.retain(|result| result.score >= minimum_confidence);
        results.sort_by_key(|result| result.start);
        (results, incomplete)
    }

    fn registry_id_exists(&self, id: &crate::RecognizerId) -> bool {
        self.registry.metadata().any(|metadata| metadata.id() == id)
    }

    fn backend_id_exists(&self, id: &crate::RecognizerId) -> bool {
        self.backends
            .iter()
            .any(|backend| backend.metadata().id() == id)
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
