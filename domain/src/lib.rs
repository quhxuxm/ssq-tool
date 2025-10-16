use chrono::NaiveDate;
use derive_more::TryFrom;
use derive_more::{Display, From};
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use strum::{EnumCount, EnumIter};

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Display, From, JsonSchema)]
#[schemars(description = "双色球")]
pub enum Ball {
    #[display("蓝球：{_0}")]
    #[schemars(description = "蓝球")]
    Blue(BlueBall),
    #[display("红球：{_0}")]
    #[schemars(description = "红球")]
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
    JsonSchema,
)]
#[try_from(repr)]
#[repr(usize)]
#[schemars(description = "蓝球")]
pub enum BlueBall {
    #[display("1")]
    #[schemars(description = "1")]
    V1 = 1,
    #[display("2")]
    #[schemars(description = "2")]
    V2 = 2,
    #[display("3")]
    #[schemars(description = "3")]
    V3 = 3,
    #[display("4")]
    #[schemars(description = "4")]
    V4 = 4,
    #[display("5")]
    #[schemars(description = "5")]
    V5 = 5,
    #[display("6")]
    #[schemars(description = "6")]
    V6 = 6,
    #[display("7")]
    #[schemars(description = "7")]
    V7 = 7,
    #[display("8")]
    #[schemars(description = "8")]
    V8 = 8,
    #[display("9")]
    #[schemars(description = "9")]
    V9 = 9,
    #[display("10")]
    #[schemars(description = "10")]
    V10 = 10,
    #[display("11")]
    #[schemars(description = "11")]
    V11 = 11,
    #[display("12")]
    #[schemars(description = "12")]
    V12 = 12,
    #[display("13")]
    #[schemars(description = "13")]
    V13 = 13,
    #[display("14")]
    #[schemars(description = "14")]
    V14 = 14,
    #[display("15")]
    #[schemars(description = "15")]
    V15 = 15,
    #[display("16")]
    #[schemars(description = "16")]
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
    JsonSchema,
)]
#[try_from(repr)]
#[repr(usize)]
#[schemars(description = "红球")]
pub enum RedBall {
    #[display("1")]
    #[schemars(description = "1")]
    V1 = 1,
    #[display("2")]
    #[schemars(description = "2")]
    V2 = 2,
    #[display("3")]
    #[schemars(description = "3")]
    V3 = 3,
    #[display("4")]
    #[schemars(description = "4")]
    V4 = 4,
    #[display("5")]
    #[schemars(description = "5")]
    V5 = 5,
    #[display("6")]
    #[schemars(description = "6")]
    V6 = 6,
    #[display("7")]
    #[schemars(description = "7")]
    V7 = 7,
    #[display("8")]
    #[schemars(description = "8")]
    V8 = 8,
    #[display("9")]
    #[schemars(description = "9")]
    V9 = 9,
    #[display("10")]
    #[schemars(description = "10")]
    V10 = 10,
    #[display("11")]
    #[schemars(description = "11")]
    V11 = 11,
    #[display("12")]
    #[schemars(description = "12")]
    V12 = 12,
    #[display("13")]
    #[schemars(description = "13")]
    V13 = 13,
    #[display("14")]
    #[schemars(description = "14")]
    V14 = 14,
    #[display("15")]
    #[schemars(description = "15")]
    V15 = 15,
    #[display("16")]
    #[schemars(description = "16")]
    V16 = 16,
    #[display("17")]
    #[schemars(description = "17")]
    V17 = 17,
    #[display("18")]
    #[schemars(description = "18")]
    V18 = 18,
    #[display("19")]
    #[schemars(description = "19")]
    V19 = 19,
    #[display("20")]
    #[schemars(description = "20")]
    V20 = 20,
    #[display("21")]
    #[schemars(description = "21")]
    V21 = 21,
    #[display("22")]
    #[schemars(description = "22")]
    V22 = 22,
    #[display("23")]
    #[schemars(description = "23")]
    V23 = 23,
    #[display("24")]
    #[schemars(description = "24")]
    V24 = 24,
    #[display("25")]
    #[schemars(description = "25")]
    V25 = 25,
    #[display("26")]
    #[schemars(description = "26")]
    V26 = 26,
    #[display("27")]
    #[schemars(description = "27")]
    V27 = 27,
    #[display("28")]
    #[schemars(description = "28")]
    V28 = 28,
    #[display("29")]
    #[schemars(description = "29")]
    V29 = 29,
    #[display("30")]
    #[schemars(description = "30")]
    V30 = 30,
    #[display("31")]
    #[schemars(description = "31")]
    V31 = 31,
    #[display("32")]
    #[schemars(description = "32")]
    V32 = 32,
    #[display("33")]
    #[schemars(description = "33")]
    V33 = 33,
}

impl Debug for RedBall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "双色球中奖记录")]
pub struct PrBusinessObj {
    #[schemars(description = "中奖期号")]
    pub code: String,
    #[schemars(description = "中奖索引")]
    pub seq: usize,
    #[schemars(description = "中奖日期")]
    pub date: NaiveDate,
    #[schemars(description = "中奖星期")]
    pub week: String,
    #[schemars(description = "中奖蓝球")]
    pub blue_ball: BlueBall,
    #[schemars(description = "中奖红球")]
    pub red_balls: [RedBall; 6],
}
