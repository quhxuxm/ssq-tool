mod codec;

use std::collections::HashMap;

use chrono::NaiveDate;
use codec::date_codec;
use codec::prize_grade_codec;

use codec::red_balls_codec;
use codec::u64_codec;
use codec::usize_codec;
use codec::vec_str_codec;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

#[derive(
    Debug, Serialize_repr, Deserialize_repr, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord,
)]
#[repr(u8)]
pub(super) enum PrizeGradeType {
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
    Level5 = 5,
    Level6 = 6,
    Level7 = 7,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct PrizePage {
    #[serde(rename = "state")]
    pub response_state: u8,
    pub message: String,
    pub total: usize,
    #[serde(rename = "pageNum")]
    pub page_num: usize,
    #[serde(rename = "pageNo")]
    pub page_no: usize,
    #[serde(rename = "pageSize")]
    pub page_size: usize,
    #[serde(rename = "Tflag")]
    pub t_flag: u8,
    #[serde(rename = "result")]
    pub prize_records: Vec<PrizeRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct PrizeGrade {
    #[serde(rename = "type")]
    pub prize_type: PrizeGradeType,
    #[serde(rename = "typenum", with = "usize_codec")]
    pub prize_type_number: usize,
    #[serde(rename = "typemoney")]
    pub prize_type_money: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct PrizeRecord {
    pub name: String,
    pub code: String,
    #[serde(rename = "detailsLink")]
    pub details_link: String,
    #[serde(rename = "videoLink")]
    pub video_link: String,
    #[serde(with = "date_codec")]
    pub date: NaiveDate,
    pub week: String,
    #[serde(with = "red_balls_codec")]
    pub red: [usize; 6],
    #[serde(with = "usize_codec")]
    pub blue: usize,
    #[serde(with = "usize_codec")]
    pub blue2: usize,
    #[serde(with = "u64_codec")]
    pub sales: u64,
    #[serde(rename = "poolmoney")]
    pub pool_money: String,
    #[serde(with = "vec_str_codec")]
    pub content: Vec<String>,
    #[serde(rename = "addmoney", with = "usize_codec")]
    pub add_money: usize,
    #[serde(rename = "addmoney2", with = "usize_codec")]
    pub add_money2: usize,
    pub msg: String,
    pub z2add: String,
    pub m2add: String,
    #[serde(rename = "prizegrades", with = "prize_grade_codec")]
    pub prize_grades: HashMap<PrizeGradeType, PrizeGrade>,
}
