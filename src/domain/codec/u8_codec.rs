use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(data: &u8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", data);
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let item = s.parse::<u8>().unwrap_or(0);
    Ok(item)
}
