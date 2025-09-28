use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!(
        "{}",
        data.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let items = s
        .split(",")
        .map(|v| v.parse::<u8>().unwrap_or(0))
        .collect::<Vec<u8>>();
    Ok(items)
}
