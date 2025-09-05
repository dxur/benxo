use std::fmt;
use std::str::FromStr;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, TS)]
pub struct PhoneNumber(#[ts(as = "String")] String);

impl PhoneNumber {
    pub fn new(s: &str) -> Result<Self, String> {
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err("Phone number cannot be empty".to_string());
        }

        // Clean the input - remove spaces, dashes, etc but keep + and digits
        let cleaned: String = trimmed
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '+')
            .collect();

        // Validate based on format
        if cleaned.starts_with("+213") {
            // International format: +213XXXXXXXXX (12 characters total)
            if cleaned.len() != 13 {
                return Err("International format must be +213 followed by 9 digits".to_string());
            }
            let local_part = &cleaned[4..]; // Skip "+213"
            validate_local_digits(local_part)?;
            Ok(PhoneNumber(cleaned))
        } else if cleaned.starts_with("0") {
            // Local format: 0XXXXXXXXX (10 digits total)
            if cleaned.len() != 10 {
                return Err("Local format must be 10 digits starting with 0".to_string());
            }
            let local_part = &cleaned[1..]; // Skip "0"
            validate_local_digits(local_part)?;
            Ok(PhoneNumber(cleaned))
        } else {
            return Err("Phone number must start with +213 or 0".to_string());
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_mobile(&self) -> bool {
        let local_digits = self.local_digits();
        let first_digit = local_digits.chars().next().unwrap();
        matches!(first_digit, '5' | '6' | '7')
    }

    pub fn is_landline(&self) -> bool {
        let local_digits = self.local_digits();
        let first_digit = local_digits.chars().next().unwrap();
        matches!(first_digit, '2' | '3' | '4' | '8')
    }

    pub fn local_digits(&self) -> &str {
        if self.0.starts_with("+213") {
            &self.0[4..]
        } else {
            &self.0[1..] // Skip the "0"
        }
    }

    pub fn to_international(&self) -> String {
        if self.0.starts_with("+213") {
            self.0.clone()
        } else {
            format!("+213{}", &self.0[1..])
        }
    }

    pub fn to_local(&self) -> String {
        if self.0.starts_with("+213") {
            format!("0{}", &self.0[4..])
        } else {
            self.0.clone()
        }
    }

    pub fn operator(&self) -> Option<&'static str> {
        let local = self.local_digits();
        if local.len() >= 3 {
            match &local[0..3] {
                "555" | "556" | "557" | "558" | "559" => Some("Djezzy"),
                "660" | "661" | "662" | "663" | "664" | "665" | "666" | "667" | "668" | "669" => {
                    Some("Mobilis")
                }
                "770" | "771" | "772" | "773" | "774" | "775" | "776" | "777" | "778" | "779" => {
                    Some("Ooredoo")
                }
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

fn validate_local_digits(digits: &str) -> Result<(), String> {
    if digits.len() != 9 {
        return Err("Local part must be exactly 9 digits".to_string());
    }

    if !digits.chars().all(|c| c.is_ascii_digit()) {
        return Err("Local part must contain only digits".to_string());
    }

    let first_digit = digits.chars().next().unwrap();
    if !matches!(first_digit, '2' | '3' | '4' | '5' | '6' | '7' | '8') {
        return Err(
            "Invalid Algerian phone number - first digit must be 2, 3, 4, 5, 6, 7, or 8"
                .to_string(),
        );
    }

    Ok(())
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PhoneNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PhoneNumber::new(s)
    }
}

impl Serialize for PhoneNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Always serialize as international format
        let international = self.to_international();
        serializer.serialize_str(&international)
    }
}

impl<'de> Deserialize<'de> for PhoneNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PhoneNumberVisitor;

        impl<'de> Visitor<'de> for PhoneNumberVisitor {
            type Value = PhoneNumber;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a valid Algerian phone number")
            }

            fn visit_str<E>(self, v: &str) -> Result<PhoneNumber, E>
            where
                E: de::Error,
            {
                PhoneNumber::new(v).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(PhoneNumberVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_format() {
        let phone = PhoneNumber::new("0662666666").unwrap();
        assert_eq!(phone.as_str(), "0662666666");
        assert_eq!(phone.local_digits(), "662666666");
        assert!(phone.is_mobile());
        assert_eq!(phone.operator(), Some("Mobilis"));
        assert_eq!(phone.to_international(), "+213662666666");
    }

    #[test]
    fn test_international_format() {
        let phone = PhoneNumber::new("+213662666666").unwrap();
        assert_eq!(phone.as_str(), "+213662666666");
        assert_eq!(phone.local_digits(), "662666666");
        assert_eq!(phone.to_local(), "0662666666");
    }

    #[test]
    fn test_landline() {
        let phone = PhoneNumber::new("0231234567").unwrap();
        assert!(phone.is_landline());
        assert!(!phone.is_mobile());
    }

    #[test]
    fn test_invalid_formats() {
        assert!(PhoneNumber::new("066266666").is_err()); // Too short
        assert!(PhoneNumber::new("06626666666").is_err()); // Too long
        assert!(PhoneNumber::new("0162666666").is_err()); // Invalid first digit
        assert!(PhoneNumber::new("abc").is_err()); // Invalid characters
    }
}
