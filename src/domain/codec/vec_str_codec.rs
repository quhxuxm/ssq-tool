use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(data: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = data.join(",").to_string();
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let items = s.split(",").map(|v| v.to_string()).collect::<Vec<String>>();
    Ok(items)
}
