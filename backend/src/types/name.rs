use std::fmt;
use std::str::FromStr;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, TS)]
pub struct Name(#[ts(as = "String")] String);

impl Name {
    pub fn new(s: &str) -> Result<Self, String> {
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        if trimmed.len() > 100 {
            return Err("Name cannot exceed 100 characters".to_string());
        }

        // Check if it contains only valid characters (letters, spaces, hyphens, apostrophes, periods)
        if !trimmed
            .chars()
            .all(|c| c.is_alphabetic() || c == ' ' || c == '-' || c == '\'' || c == '.')
        {
            return Err(
                "Name can only contain letters, spaces, hyphens, apostrophes, and periods"
                    .to_string(),
            );
        }

        // Check if it doesn't start or end with special characters
        if trimmed.starts_with(' ')
            || trimmed.ends_with(' ')
            || trimmed.starts_with('-')
            || trimmed.ends_with('-')
            || trimmed.starts_with('\'')
            || trimmed.ends_with('\'')
            || trimmed.starts_with('.')
            || trimmed.ends_with('.')
        {
            return Err("Name cannot start or end with special characters".to_string());
        }

        // Check for consecutive spaces
        if trimmed.contains("  ") {
            return Err("Name cannot contain consecutive spaces".to_string());
        }

        // Normalize the name (proper capitalization)
        let normalized = normalize_name(trimmed);

        Ok(Name(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn first_name(&self) -> Option<&str> {
        self.0.split_whitespace().next()
    }

    pub fn last_name(&self) -> Option<&str> {
        let parts: Vec<&str> = self.0.split_whitespace().collect();
        if parts.len() > 1 {
            parts.last().copied()
        } else {
            None
        }
    }

    pub fn middle_names(&self) -> Vec<&str> {
        let parts: Vec<&str> = self.0.split_whitespace().collect();
        if parts.len() > 2 {
            parts[1..parts.len() - 1].to_vec()
        } else {
            vec![]
        }
    }

    pub fn initials(&self) -> String {
        self.0
            .split_whitespace()
            .filter_map(|word| word.chars().next())
            .map(|c| c.to_uppercase().to_string())
            .collect::<Vec<_>>()
            .join(".")
    }
}

fn normalize_name(name: &str) -> String {
    name.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(_first) => {
                    let _rest: String = chars.collect();
                    // Handle special cases like "McDonald", "O'Connor", etc.
                    if word.contains('\'') {
                        word.split('\'')
                            .map(|part| capitalize_word(part))
                            .collect::<Vec<_>>()
                            .join("'")
                    } else if word.contains('-') {
                        word.split('-')
                            .map(|part| capitalize_word(part))
                            .collect::<Vec<_>>()
                            .join("-")
                    } else {
                        capitalize_word(word)
                    }
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn capitalize_word(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let mut chars = word.chars();
    let first = chars.next().unwrap().to_uppercase().to_string();
    let rest: String = chars.map(|c| c.to_lowercase().to_string()).collect();
    first + rest.as_str()
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Name {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Name::new(s)
    }
}

impl Default for Name {
    fn default() -> Self {
        Self("Unnamed".to_string())
    }
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NameVisitor;

        impl<'de> Visitor<'de> for NameVisitor {
            type Value = Name;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a valid name")
            }

            fn visit_str<E>(self, v: &str) -> Result<Name, E>
            where
                E: de::Error,
            {
                Name::new(v).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(NameVisitor)
    }
}
