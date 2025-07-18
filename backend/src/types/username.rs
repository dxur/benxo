use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, TS)]
pub struct Username(#[ts(as = "String")] String);

impl Username {
    pub fn new(s: &str) -> Result<Self, String> {
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        if trimmed.len() < 3 {
            return Err("Username must be at least 3 characters long".to_string());
        }

        if trimmed.len() > 30 {
            return Err("Username cannot exceed 30 characters".to_string());
        }

        // Check if it starts with alphanumeric
        if !trimmed.chars().next().unwrap().is_alphanumeric() {
            return Err("Username must start with a letter or number".to_string());
        }

        // Check if it contains only valid characters (alphanumeric, underscore, hyphen)
        if !trimmed
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(
                "Username can only contain letters, numbers, underscores, and hyphens".to_string(),
            );
        }

        // Check if it doesn't end with underscore or hyphen
        if trimmed.ends_with('_') || trimmed.ends_with('-') {
            return Err("Username cannot end with underscore or hyphen".to_string());
        }

        // Check for consecutive special characters
        if trimmed.contains("__")
            || trimmed.contains("--")
            || trimmed.contains("_-")
            || trimmed.contains("-_")
        {
            return Err("Username cannot contain consecutive special characters".to_string());
        }

        Ok(Username(trimmed.to_string()))
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
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Username {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Username::new(s)
    }
}

impl Serialize for Username {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Username {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsernameVisitor;

        impl<'de> Visitor<'de> for UsernameVisitor {
            type Value = Username;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a valid username")
            }

            fn visit_str<E>(self, v: &str) -> Result<Username, E>
            where
                E: de::Error,
            {
                Username::new(v).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(UsernameVisitor)
    }
}
