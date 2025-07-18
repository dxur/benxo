use email_address::EmailAddress;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, TS)]
pub struct Email(#[ts(as = "String")] EmailAddress);

impl Email {
    pub fn new(s: &str) -> Result<Self, String> {
        EmailAddress::from_str(s)
            .map(Email)
            .map_err(|e| e.to_string())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn local_part(&self) -> &str {
        self.0.local_part()
    }

    pub fn domain(&self) -> &str {
        self.0.domain()
    }

    pub fn into_inner(self) -> EmailAddress {
        self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Email {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Email::new(s)
    }
}

impl Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EmailVisitor;

        impl<'de> Visitor<'de> for EmailVisitor {
            type Value = Email;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a valid email address")
            }

            fn visit_str<E>(self, v: &str) -> Result<Email, E>
            where
                E: de::Error,
            {
                Email::new(v).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(EmailVisitor)
    }
}
