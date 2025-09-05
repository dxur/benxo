use std::fmt;
use std::str::FromStr;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, TS)]
pub struct Password(#[ts(as = "String")] String);

impl Password {
    pub fn new(s: &str) -> Result<Self, PasswordError> {
        if s.trim().is_empty() {
            return Err(PasswordError::Empty);
        }

        if s.len() < 4 {
            return Err(PasswordError::TooShort);
        }

        if s.len() > 128 {
            return Err(PasswordError::TooLong);
        }

        Ok(Password(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn masked(&self) -> String {
        "*".repeat(self.0.len())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn has_lowercase(&self) -> bool {
        self.0.chars().any(|c| c.is_lowercase())
    }

    pub fn has_uppercase(&self) -> bool {
        self.0.chars().any(|c| c.is_uppercase())
    }

    pub fn has_digit(&self) -> bool {
        self.0.chars().any(|c| c.is_ascii_digit())
    }

    pub fn has_special_char(&self) -> bool {
        self.0
            .chars()
            .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?/".contains(c))
    }

    pub fn strength(&self) -> PasswordStrength {
        let mut score = 0;

        let s = &self.0;

        if s.len() >= 8 {
            score += 1;
        }
        if s.len() >= 12 {
            score += 1;
        }
        if s.len() >= 16 {
            score += 1;
        }

        if self.has_lowercase() {
            score += 1;
        }
        if self.has_uppercase() {
            score += 1;
        }
        if self.has_digit() {
            score += 1;
        }
        if self.has_special_char() {
            score += 1;
        }

        if has_mixed_case_words(s) {
            score += 1;
        }

        if has_sequential_chars(s) {
            score -= 1;
        }

        if has_repeated_chars(s) {
            score -= 1;
        }

        match score {
            i if i <= 2 => PasswordStrength::Weak,
            3..=5 => PasswordStrength::Medium,
            6..=7 => PasswordStrength::Strong,
            _ => PasswordStrength::VeryStrong,
        }
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.masked())
    }
}

impl FromStr for Password {
    type Err = PasswordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Password::new(s)
    }
}

impl Serialize for Password {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PasswordVisitor;

        impl<'de> Visitor<'de> for PasswordVisitor {
            type Value = Password;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a valid password string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Password, E>
            where
                E: de::Error,
            {
                Password::new(v).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(PasswordVisitor)
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PasswordError {
    #[error("Password cannot be empty")]
    Empty,
    #[error("Password must be at least 4 characters long")]
    TooShort,
    #[error("Password cannot exceed 128 characters")]
    TooLong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl fmt::Display for PasswordStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordStrength::Weak => write!(f, "Weak"),
            PasswordStrength::Medium => write!(f, "Medium"),
            PasswordStrength::Strong => write!(f, "Strong"),
            PasswordStrength::VeryStrong => write!(f, "Very Strong"),
        }
    }
}

fn has_sequential_chars(s: &str) -> bool {
    let chars: Vec<_> = s.chars().collect();
    for window in chars.windows(3) {
        if let [a, b, c] = window {
            let a = *a as u32;
            let b = *b as u32;
            let c = *c as u32;

            if (b == a + 1 && c == b + 1) || (b == a - 1 && c == b - 1) {
                return true;
            }
        }
    }
    false
}

fn has_repeated_chars(s: &str) -> bool {
    let mut last = None;
    let mut count = 1;

    for c in s.chars() {
        if Some(c) == last {
            count += 1;
            if count > 2 {
                return true;
            }
        } else {
            count = 1;
        }
        last = Some(c);
    }
    false
}

fn has_mixed_case_words(s: &str) -> bool {
    s.split_whitespace().any(|word| {
        word.chars().any(|c| c.is_lowercase()) && word.chars().any(|c| c.is_uppercase())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_password() {
        let result = Password::new("");
        assert_eq!(result.unwrap_err(), PasswordError::Empty);
    }

    #[test]
    fn test_whitespace_password() {
        let result = Password::new("   ");
        assert_eq!(result.unwrap_err(), PasswordError::Empty);
    }

    #[test]
    fn test_too_short_password() {
        let result = Password::new("abc");
        assert_eq!(result.unwrap_err(), PasswordError::TooShort);
    }

    #[test]
    fn test_too_long_password() {
        let result = Password::new(&"a".repeat(129));
        assert_eq!(result.unwrap_err(), PasswordError::TooLong);
    }

    #[test]
    fn test_valid_password_creation() {
        let p = Password::new("1234").unwrap();
        assert_eq!(p.as_str(), "1234");
    }

    #[test]
    fn test_strength_levels() {
        let weak = Password::new("1234").unwrap();
        let medium = Password::new("Password123").unwrap();
        let strong = Password::new("My@Good1234!").unwrap();
        let very_strong = Password::new("ExTr3m3lY$tr0ngP@55w0rd!").unwrap();

        assert_eq!(weak.strength(), PasswordStrength::Weak);
        assert_eq!(medium.strength(), PasswordStrength::Medium);
        assert_eq!(strong.strength(), PasswordStrength::Strong);
        assert_eq!(very_strong.strength(), PasswordStrength::VeryStrong);
    }

    #[test]
    fn test_helper_methods() {
        let p = Password::new("Aa1!xyz").unwrap();

        assert!(p.has_lowercase());
        assert!(p.has_uppercase());
        assert!(p.has_digit());
        assert!(p.has_special_char());
        assert_eq!(p.len(), 7);
        assert_eq!(p.masked(), "*******");
    }

    #[test]
    fn test_repeated_characters() {
        let repeated = Password::new("aaaBBB111").unwrap();
        assert!(super::has_repeated_chars(repeated.as_str()));
    }

    #[test]
    fn test_sequential_characters() {
        let seq1 = Password::new("abcDEF123").unwrap();
        let seq2 = Password::new("zyx654").unwrap();

        assert!(super::has_sequential_chars(seq1.as_str()));
        assert!(super::has_sequential_chars(seq2.as_str()));
    }

    #[test]
    fn test_non_sequential_characters() {
        let non_seq = Password::new("a1b2c3!").unwrap();
        assert!(!super::has_sequential_chars(non_seq.as_str()));
    }

    #[test]
    fn test_mixed_case_words() {
        let mixed = Password::new("ThisIsMixed").unwrap();
        assert!(super::has_mixed_case_words(mixed.as_str()));
    }

    #[test]
    fn test_display_masked() {
        let p = Password::new("secret123").unwrap();
        assert_eq!(format!("{}", p), "*********");
    }

    #[test]
    fn test_serde_serialization() {
        let p = Password::new("Abc123!@#").unwrap();
        let json = serde_json::to_string(&p).unwrap();
        assert_eq!(json, "\"Abc123!@#\"");
    }

    #[test]
    fn test_serde_deserialization() {
        let json = "\"Abc123!@#\"";
        let p: Password = serde_json::from_str(json).unwrap();
        assert_eq!(p.as_str(), "Abc123!@#");
    }
}
