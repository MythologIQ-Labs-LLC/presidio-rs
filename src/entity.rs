//! PII entity taxonomy.

use core::fmt;

/// A category of personally-identifiable information.
///
/// The format-structured variants are detectable by regex plus (optionally) a
/// checksum validator. The [`EntityType::Person`], [`EntityType::Location`],
/// and [`EntityType::Nrp`] variants have no regex form; they are reserved for a
/// future offline model (NER) backend and are never emitted by the current
/// [`crate::AnalyzerEngine`] legacy path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum EntityType {
    CreditCard,
    Ssn,
    Email,
    PhoneNumber,
    IpAddress,
    MacAddress,
    IbanCode,
    CryptoWallet,
    Url,
    UsItin,
    ApiKey,
    /// NER-only — reserved for a future model backend.
    Person,
    /// NER-only — reserved for a future model backend.
    Location,
    /// NER-only (nationality / religion / political group).
    Nrp,
}

impl EntityType {
    /// Canonical uppercase tag, e.g. `CREDIT_CARD`. Stable identifier used in
    /// replacement markers and serialized output.
    pub fn as_tag(&self) -> &'static str {
        match self {
            EntityType::CreditCard => "CREDIT_CARD",
            EntityType::Ssn => "US_SSN",
            EntityType::Email => "EMAIL_ADDRESS",
            EntityType::PhoneNumber => "PHONE_NUMBER",
            EntityType::IpAddress => "IP_ADDRESS",
            EntityType::MacAddress => "MAC_ADDRESS",
            EntityType::IbanCode => "IBAN_CODE",
            EntityType::CryptoWallet => "CRYPTO",
            EntityType::Url => "URL",
            EntityType::UsItin => "US_ITIN",
            EntityType::ApiKey => "API_KEY",
            EntityType::Person => "PERSON",
            EntityType::Location => "LOCATION",
            EntityType::Nrp => "NRP",
        }
    }

    /// Convert a canonical open entity identifier into the legacy taxonomy.
    pub fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "CREDIT_CARD" => Some(Self::CreditCard),
            "US_SSN" => Some(Self::Ssn),
            "EMAIL_ADDRESS" => Some(Self::Email),
            "PHONE_NUMBER" => Some(Self::PhoneNumber),
            "IP_ADDRESS" => Some(Self::IpAddress),
            "MAC_ADDRESS" => Some(Self::MacAddress),
            "IBAN_CODE" => Some(Self::IbanCode),
            "CRYPTO" => Some(Self::CryptoWallet),
            "URL" => Some(Self::Url),
            "US_ITIN" => Some(Self::UsItin),
            "API_KEY" => Some(Self::ApiKey),
            "PERSON" => Some(Self::Person),
            "LOCATION" => Some(Self::Location),
            "NRP" => Some(Self::Nrp),
            _ => None,
        }
    }
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_tag())
    }
}
