use std::{fs::File, path::Path};

use crate::{domain::PrizeRecordPage, error::Error};

const OFFICIAL_DATA_FILE: &str = "official_data.json";

pub async fn generate_official_data() -> Result<PrizeRecordPage, Error> {
    let official_data_json_path = Path::new(OFFICIAL_DATA_FILE);
    let official_data_json = File::open(official_data_json_path)?;
    let page = serde_json::from_reader::<File, PrizeRecordPage>(official_data_json)?;
    Ok(page)
}
