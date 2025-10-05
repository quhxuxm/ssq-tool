use std::{fs::File, path::Path};

use crate::{
    domain::{BusinessPrizeRecord, OfficialPrizeRecordPage},
    error::Error,
};

const OFFICIAL_DATA_FILE: &str = "official_data.json";

pub async fn collect_business_data() -> Result<Vec<BusinessPrizeRecord>, Error> {
    let official_data_json_path = Path::new(OFFICIAL_DATA_FILE);
    let official_data_json = File::open(official_data_json_path)?;
    let page = serde_json::from_reader::<File, OfficialPrizeRecordPage>(official_data_json)?;
    let mut index = 0usize;
    let business_records = page
        .prize_records
        .iter()
        .map(|prize_record| {
            let business_prize_record = BusinessPrizeRecord::new(
                prize_record.code.clone(),
                index,
                prize_record.red.clone(),
                prize_record.blue,
                prize_record.date,
            );
            index += 1;
            business_prize_record
        })
        .collect::<Vec<BusinessPrizeRecord>>();
    Ok(business_records)
}
