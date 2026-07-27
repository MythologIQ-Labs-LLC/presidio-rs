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
//!
//! It performs no network or filesystem I/O and requires no Python runtime.
//! Person names, prose locations, and other semantic entities are not detected
//! by the current model-free implementation.
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
//! This is an independent open-source project. It is not affiliated with or
//! endorsed by Microsoft. See the repository README for scope, limitations,
//! security guidance, and acknowledgements.

#![forbid(unsafe_code)]

mod analyzer;
mod anonymizer;
pub mod context;
mod entity;
mod metadata;
mod recognizer;
mod registry;
mod report;
mod result;
mod types;
pub mod validators;

pub use analyzer::{AnalyzerEngine, DEFAULT_SCORE_THRESHOLD};
pub use anonymizer::{anonymize, AnonymizerEngine, Operator};
pub use entity::EntityType;
pub use metadata::{RecognitionMechanism, RecognizerMetadata, RecognizerMetadataError};
pub use recognizer::{
    ContextValidationError, Pattern, PatternRecognizer, PatternRecognizerRegistrationError,
    PatternValidationError, Validator,
};
pub use registry::{RecognizerRegistry, RecognizerRegistryError};
pub use report::{
    AnalysisIssue, AnalysisOptions, AnalysisReport, AnalysisStatus, DEFAULT_REPORT_CANDIDATE_LIMIT,
    DEFAULT_REPORT_ISSUE_LIMIT,
};
pub use result::RecognizerResult;
pub use types::{
    Confidence, ConfidenceError, EntityId, Evidence, Finding, FindingConversionError,
    IdentifierError, MetadataId, RecognizerId, Span, SpanError,
};
