use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};
use std::fmt::Display;

pub fn non_negative<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + PartialOrd + Display + From<u8>,
{
    let val = T::deserialize(deserializer)?;
    if val < T::from(0) {
        return Err(de::Error::invalid_value(
            Unexpected::Other(&val.to_string()),
            &"a non negative value",
        ));
    }
    Ok(val)
}

pub fn non_negative_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + PartialOrd + Display + From<u8>,
{
    let val = Option::<T>::deserialize(deserializer)?;
    if let Some(ref v) = val {
        if *v < T::from(0) {
            return Err(de::Error::invalid_value(
                Unexpected::Other(&v.to_string()),
                &"a non-negative value",
            ));
        }
    }
    Ok(val)
}
