use std::collections::HashMap;

use super::date_codec;
use super::prize_grade_codec;
use super::u64_codec;
use super::usize_codec;
use super::vec_str_codec;
use super::vec_usize_codec;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

#[derive(
    Debug, Serialize_repr, Deserialize_repr, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord,
)]
#[repr(u8)]
pub enum OfficialPrizeGradeType {
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
    Level5 = 5,
    Level6 = 6,
    Level7 = 7,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfficialPrizeRecordPage {
    pub state: u8,
    pub message: String,
    pub total: u64,
    #[serde(rename = "pageNum")]
    pub page_num: u64,
    #[serde(rename = "pageNo")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    #[serde(rename = "Tflag")]
    pub t_flag: u8,
    #[serde(rename = "result")]
    pub prize_records: Vec<OfficialPrizeRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfficialPrizeGrade {
    #[serde(rename = "type")]
    pub prize_type: OfficialPrizeGradeType,
    #[serde(rename = "typenum", with = "u64_codec")]
    pub prize_type_number: u64,
    #[serde(rename = "typemoney")]
    pub prize_type_money: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfficialPrizeRecord {
    pub name: String,
    pub code: String,
    #[serde(rename = "detailsLink")]
    pub details_link: String,
    #[serde(rename = "videoLink")]
    pub video_link: String,
    #[serde(with = "date_codec")]
    pub date: NaiveDate,
    pub week: String,
    #[serde(with = "vec_usize_codec")]
    pub red: Vec<usize>,
    #[serde(with = "usize_codec")]
    pub blue: usize,
    #[serde(with = "usize_codec")]
    pub blue2: usize,
    pub sales: String,
    pub poolmoney: String,
    #[serde(with = "vec_str_codec")]
    pub content: Vec<String>,
    #[serde(with = "u64_codec")]
    pub addmoney: u64,
    #[serde(with = "u64_codec")]
    pub addmoney2: u64,
    pub msg: String,
    pub z2add: String,
    pub m2add: String,
    #[serde(rename = "prizegrades", with = "prize_grade_codec")]
    pub prize_grades: HashMap<OfficialPrizeGradeType, OfficialPrizeGrade>,
}
