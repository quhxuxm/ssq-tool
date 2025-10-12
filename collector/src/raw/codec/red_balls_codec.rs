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

pub fn deserialize<'de, D>(deserializer: D) -> Result<[usize; 6], D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let items = s
        .split(",")
        .map(|v| v.parse::<usize>().unwrap_or(0))
        .collect::<Vec<usize>>();
    let array = [items[0], items[1], items[2], items[3], items[4], items[5]];
    Ok(array)
}
