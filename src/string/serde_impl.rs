use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S: Serializer>(value: &String, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_str(value)
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    Ok(String::deserialize(deserializer)?)
}
