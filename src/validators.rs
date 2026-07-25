//! Checksum validators.
//!
//! A validator inspects the matched substring and returns:
//! - `Some(true)`  — checksum passes; the analyzer sets the score to `1.0`.
//! - `Some(false)` — checksum fails; the analyzer drops the match.
//! - `None`        — no decision; the analyzer keeps the pattern's base score.

/// Luhn (mod-10) check, used for credit-card numbers.
pub fn luhn(matched: &str) -> Option<bool> {
    let digits: Vec<u32> = matched.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() < 13 || digits.len() > 19 {
        return Some(false);
    }
    let mut sum = 0u32;
    let mut double = false;
    for &d in digits.iter().rev() {
        let mut v = d;
        if double {
            v *= 2;
            if v > 9 {
                v -= 9;
            }
        }
        sum += v;
        double = !double;
    }
    Some(sum % 10 == 0)
}

/// ISO 13616 IBAN mod-97 check.
pub fn iban_mod97(matched: &str) -> Option<bool> {
    let compact: String = matched
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();
    if compact.len() < 15 || compact.len() > 34 {
        return Some(false);
    }
    let upper = compact.to_ascii_uppercase();
    let (head, tail) = upper.split_at(4);
    let rearranged = format!("{tail}{head}");

    let mut remainder: u32 = 0;
    for c in rearranged.chars() {
        let value = if c.is_ascii_digit() {
            c as u32 - '0' as u32
        } else if c.is_ascii_uppercase() {
            c as u32 - 'A' as u32 + 10
        } else {
            return Some(false);
        };
        // Letters expand to two decimal digits (10..=35), digits to one.
        if value >= 10 {
            remainder = (remainder * 100 + value) % 97;
        } else {
            remainder = (remainder * 10 + value) % 97;
        }
    }
    Some(remainder == 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luhn_accepts_valid_card() {
        assert_eq!(luhn("4111111111111111"), Some(true));
    }

    #[test]
    fn luhn_rejects_invalid_card() {
        assert_eq!(luhn("4111111111111112"), Some(false));
    }

    #[test]
    fn iban_accepts_valid() {
        assert_eq!(iban_mod97("GB82WEST12345698765432"), Some(true));
    }

    #[test]
    fn iban_rejects_bad_checksum() {
        assert_eq!(iban_mod97("GB00WEST12345698765432"), Some(false));
    }
}
