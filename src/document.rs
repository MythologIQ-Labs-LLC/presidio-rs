//! Borrowed source documents and content-bound identity.

use core::fmt;

use sha2::{Digest, Sha256};

use crate::types::{DocumentId, Span, SpanError};

/// SHA-256 fingerprint of the original UTF-8 document bytes.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DocumentFingerprint([u8; 32]);

impl DocumentFingerprint {
    /// Compute a fingerprint from the exact original UTF-8 bytes.
    pub fn from_text(text: &str) -> Self {
        let digest = Sha256::digest(text.as_bytes());
        let mut bytes = [0_u8; 32];
        bytes.copy_from_slice(&digest);
        Self(bytes)
    }

    /// Borrow the raw digest bytes.
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Render the fingerprint as lowercase hexadecimal.
    pub fn to_hex(self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut output = String::with_capacity(64);
        for byte in self.0 {
            output.push(HEX[(byte >> 4) as usize] as char);
            output.push(HEX[(byte & 0x0f) as usize] as char);
        }
        output
    }

    /// Whether this fingerprint describes the supplied exact UTF-8 text.
    pub fn matches_text(self, text: &str) -> bool {
        self == Self::from_text(text)
    }
}

impl fmt::Debug for DocumentFingerprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("DocumentFingerprint")
            .field(&self.to_hex())
            .finish()
    }
}

impl fmt::Display for DocumentFingerprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_hex())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for DocumentFingerprint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

/// Serializable identity and content fingerprint for one exact source document.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DocumentBinding {
    id: DocumentId,
    byte_len: usize,
    fingerprint: DocumentFingerprint,
}

impl DocumentBinding {
    /// Bind a caller-controlled document ID to exact UTF-8 content.
    pub fn for_text(id: DocumentId, text: &str) -> Self {
        Self {
            id,
            byte_len: text.len(),
            fingerprint: DocumentFingerprint::from_text(text),
        }
    }

    /// Caller-controlled stable document identifier.
    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    /// Length of the original UTF-8 text in bytes.
    pub const fn byte_len(&self) -> usize {
        self.byte_len
    }

    /// Fingerprint of the original UTF-8 bytes.
    pub const fn fingerprint(&self) -> DocumentFingerprint {
        self.fingerprint
    }

    /// Validate exact text content without checking a caller-controlled ID.
    pub fn validate_text(&self, text: &str) -> Result<(), DocumentBindingError> {
        if self.byte_len != text.len() {
            return Err(DocumentBindingError::LengthMismatch {
                expected: self.byte_len,
                actual: text.len(),
            });
        }
        if !self.fingerprint.matches_text(text) {
            return Err(DocumentBindingError::FingerprintMismatch);
        }
        Ok(())
    }

    /// Validate identity and exact content against a document.
    pub fn validate_document(
        &self,
        document: &TextDocument<'_>,
    ) -> Result<(), DocumentBindingError> {
        if &self.id != document.id() {
            return Err(DocumentBindingError::IdMismatch {
                expected: self.id.clone(),
                actual: document.id().clone(),
            });
        }
        self.validate_text(document.original())
    }
}

/// Borrowed original UTF-8 source paired with a durable content binding.
pub struct TextDocument<'a> {
    original: &'a str,
    binding: DocumentBinding,
}

impl<'a> TextDocument<'a> {
    /// Create a document from a validated caller-controlled ID and exact text.
    pub fn new(id: DocumentId, original: &'a str) -> Self {
        Self {
            original,
            binding: DocumentBinding::for_text(id, original),
        }
    }

    /// Exact original UTF-8 source.
    pub const fn original(&self) -> &'a str {
        self.original
    }

    /// Caller-controlled stable document identifier.
    pub fn id(&self) -> &DocumentId {
        self.binding.id()
    }

    /// Identity, length, and fingerprint of the exact source.
    pub fn binding(&self) -> &DocumentBinding {
        &self.binding
    }

    /// Original text length in bytes.
    pub const fn len(&self) -> usize {
        self.original.len()
    }

    /// Whether the original source contains no bytes.
    pub const fn is_empty(&self) -> bool {
        self.original.is_empty()
    }

    /// Validate and slice an original-coordinate span.
    pub fn slice(&self, span: Span) -> Result<&'a str, SpanError> {
        span.validate_for(self.original)?;
        Ok(&self.original[span.start()..span.end()])
    }
}

impl fmt::Debug for TextDocument<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TextDocument")
            .field("binding", &self.binding)
            .finish_non_exhaustive()
    }
}

/// Failure to validate a document binding against a source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentBindingError {
    /// The caller-controlled document IDs differ.
    IdMismatch {
        expected: DocumentId,
        actual: DocumentId,
    },
    /// The exact UTF-8 byte lengths differ.
    LengthMismatch { expected: usize, actual: usize },
    /// The SHA-256 content fingerprints differ.
    FingerprintMismatch,
}

impl fmt::Display for DocumentBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdMismatch { expected, actual } => {
                write!(
                    formatter,
                    "document ID mismatch: expected {expected}, found {actual}"
                )
            }
            Self::LengthMismatch { expected, actual } => write!(
                formatter,
                "document byte length mismatch: expected {expected}, found {actual}"
            ),
            Self::FingerprintMismatch => {
                formatter.write_str("document content fingerprint mismatch")
            }
        }
    }
}

impl std::error::Error for DocumentBindingError {}

/// Failure to apply or inspect a finding against a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindingDocumentError {
    /// The finding was produced by an unbound legacy or string-only analysis path.
    UnboundFinding,
    /// The supplied document does not match the finding's bound source.
    Document(DocumentBindingError),
    /// The finding span is invalid for the bound source.
    Span(SpanError),
}

impl fmt::Display for FindingDocumentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnboundFinding => formatter.write_str("finding is not bound to a document"),
            Self::Document(error) => write!(formatter, "finding document mismatch: {error}"),
            Self::Span(error) => write!(formatter, "finding span is invalid for document: {error}"),
        }
    }
}

impl std::error::Error for FindingDocumentError {}

impl From<DocumentBindingError> for FindingDocumentError {
    fn from(value: DocumentBindingError) -> Self {
        Self::Document(value)
    }
}

impl From<SpanError> for FindingDocumentError {
    fn from(value: SpanError) -> Self {
        Self::Span(value)
    }
}
