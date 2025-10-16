use chrono::NaiveDate;
use derive_more::TryFrom;
use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use strum::{EnumCount, EnumIter};

#[derive(
    Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Display, From, Deserialize, Serialize,
)]
#[serde(rename = "双色球")]
pub enum Ball {
    #[display("蓝球：{_0}")]
    #[serde(rename = "蓝球")]
    Blue(BlueBall),
    #[display("红球：{_0}")]
    #[serde(rename = "红球")]
    Red(RedBall),
}

impl Debug for Ball {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(
    Hash,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Display,
    TryFrom,
    EnumIter,
    EnumCount,
    Serialize,
    Deserialize,
)]
#[try_from(repr)]
#[repr(usize)]
#[serde(rename = "蓝球")]
pub enum BlueBall {
    #[display("1")]
    #[serde(rename = "1")]
    V1 = 1,
    #[display("2")]
    #[serde(rename = "2")]
    V2 = 2,
    #[display("3")]
    #[serde(rename = "3")]
    V3 = 3,
    #[display("4")]
    #[serde(rename = "4")]
    V4 = 4,
    #[display("5")]
    #[serde(rename = "5")]
    V5 = 5,
    #[display("6")]
    #[serde(rename = "6")]
    V6 = 6,
    #[display("7")]
    #[serde(rename = "7")]
    V7 = 7,
    #[display("8")]
    #[serde(rename = "8")]
    V8 = 8,
    #[display("9")]
    #[serde(rename = "9")]
    V9 = 9,
    #[display("10")]
    #[serde(rename = "10")]
    V10 = 10,
    #[display("11")]
    #[serde(rename = "11")]
    V11 = 11,
    #[display("12")]
    #[serde(rename = "12")]
    V12 = 12,
    #[display("13")]
    #[serde(rename = "13")]
    V13 = 13,
    #[display("14")]
    #[serde(rename = "14")]
    V14 = 14,
    #[display("15")]
    #[serde(rename = "15")]
    V15 = 15,
    #[display("16")]
    #[serde(rename = "16")]
    V16 = 16,
}

impl Debug for BlueBall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(
    Hash,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Display,
    TryFrom,
    EnumIter,
    EnumCount,
    Serialize,
    Deserialize,
)]
#[try_from(repr)]
#[repr(usize)]
#[serde(rename = "红球")]
pub enum RedBall {
    #[display("1")]
    #[serde(rename = "1")]
    V1 = 1,
    #[display("2")]
    #[serde(rename = "2")]
    V2 = 2,
    #[display("3")]
    #[serde(rename = "3")]
    V3 = 3,
    #[display("4")]
    #[serde(rename = "4")]
    V4 = 4,
    #[display("5")]
    #[serde(rename = "5")]
    V5 = 5,
    #[display("6")]
    #[serde(rename = "6")]
    V6 = 6,
    #[display("7")]
    #[serde(rename = "7")]
    V7 = 7,
    #[display("8")]
    #[serde(rename = "8")]
    V8 = 8,
    #[display("9")]
    #[serde(rename = "9")]
    V9 = 9,
    #[display("10")]
    #[serde(rename = "10")]
    V10 = 10,
    #[display("11")]
    #[serde(rename = "11")]
    V11 = 11,
    #[display("12")]
    #[serde(rename = "12")]
    V12 = 12,
    #[display("13")]
    #[serde(rename = "13")]
    V13 = 13,
    #[display("14")]
    #[serde(rename = "14")]
    V14 = 14,
    #[display("15")]
    #[serde(rename = "15")]
    V15 = 15,
    #[display("16")]
    #[serde(rename = "16")]
    V16 = 16,
    #[display("17")]
    #[serde(rename = "17")]
    V17 = 17,
    #[display("18")]
    #[serde(rename = "18")]
    V18 = 18,
    #[display("19")]
    #[serde(rename = "19")]
    V19 = 19,
    #[display("20")]
    #[serde(rename = "20")]
    V20 = 20,
    #[display("21")]
    #[serde(rename = "21")]
    V21 = 21,
    #[display("22")]
    #[serde(rename = "22")]
    V22 = 22,
    #[display("23")]
    #[serde(rename = "23")]
    V23 = 23,
    #[display("24")]
    #[serde(rename = "24")]
    V24 = 24,
    #[display("25")]
    #[serde(rename = "25")]
    V25 = 25,
    #[display("26")]
    #[serde(rename = "26")]
    V26 = 26,
    #[display("27")]
    #[serde(rename = "27")]
    V27 = 27,
    #[display("28")]
    #[serde(rename = "28")]
    V28 = 28,
    #[display("29")]
    #[serde(rename = "29")]
    V29 = 29,
    #[display("30")]
    #[serde(rename = "30")]
    V30 = 30,
    #[display("31")]
    #[serde(rename = "31")]
    V31 = 31,
    #[display("32")]
    #[serde(rename = "32")]
    V32 = 32,
    #[display("33")]
    #[serde(rename = "33")]
    V33 = 33,
}

impl Debug for RedBall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "中奖记录")]
pub struct PrBusinessObj {
    #[serde(rename = "中奖期号")]
    pub code: String,
    #[serde(rename = "中奖索引")]
    pub seq: usize,
    #[serde(rename = "中奖日期")]
    pub date: NaiveDate,
    #[serde(rename = "中奖星期")]
    pub day: String,
    #[serde(rename = "蓝球")]
    pub blue_ball: BlueBall,
    #[serde(rename = "红球")]
    pub red_balls: [RedBall; 6],
}
