use std::{fs::File, path::Path};

use crate::{domain::PrizeRecordPage, error::Error};

pub async fn collect_official_data() -> Result<PrizeRecordPage, Error> {
    let official_data_json_path = Path::new("official_data.json");
    let official_data_json = File::open(official_data_json_path)?;
    let page = serde_json::from_reader::<File, PrizeRecordPage>(official_data_json)?;
    Ok(page)
}
