use crate::collect_business_obj;
use crate::error::Error;
use crate::raw::PrizePage;
use ssq_tool_domain::PrBusinessObj;
use std::fs::File;
use std::path::Path;

const OFFICIAL_DATA_FILE: &str = "official_data.json";

pub(super) async fn collect_from_file(
    record_size_to_store: Option<usize>,
) -> Result<Vec<PrBusinessObj>, Error> {
    let official_data_json_path = Path::new(OFFICIAL_DATA_FILE);
    let official_data_json = File::open(official_data_json_path)?;
    let page = serde_json::from_reader::<File, PrizePage>(official_data_json)?;
    collect_business_obj(page, record_size_to_store)
}
