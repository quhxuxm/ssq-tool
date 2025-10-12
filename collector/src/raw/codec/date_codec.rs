use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serializer};

const FORMAT: &str = "%Y-%m-%d";

pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let additional_part_start_index = s.find("(");
    match additional_part_start_index {
        Some(additional_part_start_index) => {
            let date_part = &s[0..additional_part_start_index];
            Ok(NaiveDate::parse_from_str(date_part, FORMAT).map_err(serde::de::Error::custom)?)
        }
        None => Ok(NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?),
    }
}
