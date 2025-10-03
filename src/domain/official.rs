use std::collections::HashMap;
use std::ops::Deref;
use std::ops::Sub;

use crate::error::Error;

use super::date_codec;
use super::prize_grade_codec;
use super::u8_codec;
use super::u64_codec;
use super::vec_str_codec;
use super::vec_u8_codec;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

#[derive(
    Debug, Serialize_repr, Deserialize_repr, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord,
)]
#[repr(u8)]
pub enum PrizeGradeType {
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
    Level5 = 5,
    Level6 = 6,
    Level7 = 7,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrizeRecordPage {
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
    pub prize_records: Vec<PrizeRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrizeGrade {
    #[serde(rename = "type")]
    pub prize_type: PrizeGradeType,
    #[serde(rename = "typenum", with = "u64_codec")]
    pub prize_type_number: u64,
    #[serde(rename = "typemoney")]
    pub prize_type_money: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[serde(try_from = "String")]
pub struct PrizeRecordCode(pub i32);

impl TryFrom<String> for PrizeRecordCode {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let raw_code = value.parse::<i32>()?;
        Ok(Self(raw_code))
    }
}

impl Deref for PrizeRecordCode {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Sub for PrizeRecordCode {
    type Output = i32;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrizeRecord {
    pub name: String,
    pub code: PrizeRecordCode,
    #[serde(rename = "detailsLink")]
    pub details_link: String,
    #[serde(rename = "videoLink")]
    pub video_link: String,
    #[serde(with = "date_codec")]
    pub date: NaiveDate,
    pub week: String,
    #[serde(with = "vec_u8_codec")]
    pub red: Vec<u8>,
    #[serde(with = "u8_codec")]
    pub blue: u8,
    #[serde(with = "u8_codec")]
    pub blue2: u8,
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
    pub prize_grades: HashMap<PrizeGradeType, PrizeGrade>,
}
