use std::{fmt, str::FromStr};

use bson::oid::ObjectId;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, TS)]
pub struct Id(#[ts(as = "String")] ObjectId);

impl Id {
    pub fn new() -> Self {
        Self(ObjectId::new())
    }

    pub fn into_inner(self) -> ObjectId {
        self.0
    }
}

impl From<ObjectId> for Id {
    fn from(value: ObjectId) -> Self {
        Self(value)
    }
}

impl Into<ObjectId> for Id {
    fn into(self) -> ObjectId {
        self.0
    }
}

impl AsRef<ObjectId> for Id {
    fn as_ref(&self) -> &ObjectId {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.to_hex().fmt(f)
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.to_hex().as_str())
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IdVisitor;

        impl<'de> Visitor<'de> for IdVisitor {
            type Value = Id;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a valid Id")
            }

            fn visit_str<E>(self, v: &str) -> Result<Id, E>
            where
                E: de::Error,
            {
                ObjectId::from_str(v).map(Id).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(IdVisitor)
    }
}
