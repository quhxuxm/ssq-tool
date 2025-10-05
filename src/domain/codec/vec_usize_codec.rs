use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(data: &[usize], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = data
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",")
        .to_string();
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let items = s
        .split(",")
        .map(|v| v.parse::<usize>().unwrap_or(0))
        .collect::<Vec<usize>>();
    Ok(items)
}
