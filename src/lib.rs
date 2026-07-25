//! # presidio-rs (internal)
//!
//! Offline, pure-Rust PII detection and anonymization — a Rust-native
//! reimagining of Microsoft Presidio's analyzer/anonymizer design.
//!
//! - **Zero network access.** No downloads, no runtime calls; the entire engine
//!   is regex + arithmetic + hashing.
//! - **Detection** = regex recognizers, checksum validators (Luhn, IBAN mod-97),
//!   and context-word scoring (a nearby keyword boosts a match's confidence).
//! - **Anonymization** = replace / redact / mask / hash operators.
//! - **NER gap** is explicit: [`EntityType::Person`], [`EntityType::Location`],
//!   and [`EntityType::Nrp`] have no regex form and are reserved for a future
//!   offline model backend (ONNX token-classifier). The current engine never
//!   emits them.
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
//! Internal — MythologIQ-Labs-LLC. Consumed by GG-CORE / COREFORGE; not for
//! public distribution.

#![forbid(unsafe_code)]

mod analyzer;
mod anonymizer;
pub mod context;
mod entity;
mod recognizer;
mod result;
pub mod validators;

pub use analyzer::{AnalyzerEngine, DEFAULT_SCORE_THRESHOLD};
pub use anonymizer::{anonymize, Operator};
pub use entity::EntityType;
pub use recognizer::{Pattern, PatternRecognizer, Validator};
pub use result::RecognizerResult;
