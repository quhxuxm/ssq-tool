use std::collections::HashMap;

use polars::{frame::DataFrame, prelude::NamedFrom, series::Series};

use crate::domain::PrizeRecordPage;

pub fn generate_data_storage(prize_record_page: PrizeRecordPage) {
    let prize_records = prize_record_page.prize_records;
    let mut redballs = HashMap::new();
    for prize_record in prize_records.into_iter() {
        let redball_records = prize_record.red;
        for i in 0..6 {
            redballs.entry(i).and_modify(|i| {});
            redballs.insert(i, redball_records[i]);
        }
    }

    //let redball1_column = Series::new("redball1".into(), _)
    todo!()
}
