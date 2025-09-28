use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(data: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", data);
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let item = s.parse::<u64>().unwrap_or(0);
    Ok(item)
}
