use glob::Pattern;
use serde::{Deserialize, Deserializer, Serializer};



type Field = Option<Pattern>;

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Field, D::Error> {
    let string = String::deserialize(deserializer)?;
    Pattern::new(&string).map(Some).map_err(serde::de::Error::custom)
}

pub fn serialize<S: Serializer>(field: &Field, serializer: S) -> Result<S::Ok, S::Error> {
    match field {
        Some(pat) => serializer.serialize_str(pat.as_str()),
        None => serializer.serialize_none(),
    }
}
