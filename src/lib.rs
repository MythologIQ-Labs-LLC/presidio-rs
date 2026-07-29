//! # presidio-rs
//!
//! Offline, Rust-native PII detection and anonymization inspired by the
//! model-free architecture of Microsoft Presidio.
//!
//! The crate currently provides:
//!
//! - pattern-based detection for structured identifiers and secrets
//! - checksum validation for supported entity types
//! - nearby context-word scoring
//! - replace, redact, mask, and deterministic hash operators
//! - custom recognizer and validator registration
//! - additive validated value types for the next-generation finding contract
//! - bounded candidate-preserving analysis reports
//! - validated recognizer metadata and provenance
//! - document identity and exact source-content binding
//! - backend-neutral recognizer execution through validated analysis requests
//! - pure, explicit, versioned candidate-resolution policies
//! - document-validated analysis-to-resolution integration
//!
//! It performs no network or filesystem I/O and requires no Python runtime.
//! Person names, prose locations, and other semantic entities are not detected
//! by the current model-free implementation unless a consumer supplies a backend.
//!
//! ```
//! use presidio::{anonymize, AnalyzerEngine, Operator};
//!
//! let analyzer = AnalyzerEngine::new();
//! let text = "Email jane@acme.com about card 4111 1111 1111 1111.";
//! let found = analyzer.analyze(text, None);
//! let clean = anonymize(text, &found, &Operator::Replace(None));
//! assert!(!clean.contains("jane@acme.com"));
//! assert!(clean.contains("<EMAIL_ADDRESS>"));
//! ```
//!
//! Request-oriented analysis binds findings to exact source bytes and applies
//! deterministic selection and resource limits:
//!
//! ```
//! use presidio::{AnalysisRequest, AnalyzerEngine, DocumentId, TextDocument};
//!
//! let document = TextDocument::new(
//!     DocumentId::new("request-42").expect("valid document ID"),
//!     "Email jane@acme.com",
//! );
//! let request = AnalysisRequest::new();
//! let report = AnalyzerEngine::new()
//!     .analyze_request(&document, &request)
//!     .expect("bounded analysis");
//! report.validate_for_document(&document).expect("matching source");
//! ```
//!
//! Document-aware reports can be resolved only after exact source validation:
//!
//! ```
//! use presidio::{
//!     AnalysisRequest, AnalyzerEngine, DocumentId, ResolutionOptions,
//!     ResolutionPolicy, TextDocument,
//! };
//!
//! let document = TextDocument::new(
//!     DocumentId::new("request-42").expect("valid document ID"),
//!     "Email jane@acme.com",
//! );
//! let report = AnalyzerEngine::new()
//!     .analyze_request(&document, &AnalysisRequest::new())?;
//! let resolved = report.resolve_for_document(
//!     &document,
//!     &ResolutionOptions::new(ResolutionPolicy::ConservativeRedaction),
//! )?;
//! assert!(resolved.resolution().status().output_complete());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! This is an independent open-source project. It is not affiliated with or
//! endorsed by Microsoft. See the repository README for scope, limitations,
//! security guidance, and acknowledgements.

#![forbid(unsafe_code)]

mod analyzer;
mod anonymizer;
pub mod context;
mod document;
mod entity;
mod metadata;
mod recognition;
mod recognizer;
mod registry;
mod report;
mod report_resolution;
mod request;
mod resolution;
mod result;
mod types;
pub mod validators;

pub use analyzer::{AnalyzerEngine, DEFAULT_SCORE_THRESHOLD};
pub use anonymizer::{anonymize, AnonymizerEngine, Operator};
pub use document::{
    DocumentBinding, DocumentBindingError, DocumentFingerprint, FindingDocumentError, TextDocument,
};
pub use entity::EntityType;
pub use metadata::{RecognitionMechanism, RecognizerMetadata, RecognizerMetadataError};
pub use recognition::{
    CandidateEmissionError, CandidateEmitter, EmissionStatus, RecognitionError,
    RecognitionErrorKind, Recognizer,
};
pub use recognizer::{
    ContextValidationError, Pattern, PatternRecognizer, PatternRecognizerRegistrationError,
    PatternValidationError, Validator,
};
pub use registry::{RecognizerRegistry, RecognizerRegistryError};
pub use report::{
    AnalysisIssue, AnalysisOptions, AnalysisReport, AnalysisStatus, ReportDocumentError,
    DEFAULT_REPORT_CANDIDATE_LIMIT, DEFAULT_REPORT_ISSUE_LIMIT,
};
pub use report_resolution::{AnalysisResolutionError, ResolvedAnalysisReport};
pub use request::{
    AnalysisExecutionError, AnalysisRequest, AnalysisRequestError, LimitDimension,
    DEFAULT_ANALYSIS_INPUT_LIMIT,
};
pub use resolution::{
    resolve_candidates, ResolutionDecision, ResolutionError, ResolutionOptions, ResolutionPolicy,
    ResolutionReport, ResolutionStatus, ResolvedEntity, ResolvedFinding,
    DEFAULT_RESOLUTION_CANDIDATE_LIMIT, DEFAULT_RESOLUTION_DECISION_LIMIT,
    DEFAULT_RESOLUTION_OUTPUT_LIMIT, RESOLUTION_POLICY_VERSION_V1,
};
pub use result::RecognizerResult;
pub use types::{
    Confidence, ConfidenceError, DocumentId, EntityId, Evidence, Finding, FindingConversionError,
    IdentifierError, MetadataId, RecognizerId, Span, SpanError,
};
