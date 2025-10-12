use chrono::NaiveDate;
use derive_more::TryFrom;
use derive_more::{Display, From};
use std::fmt::{Debug, Formatter};
use strum::{EnumCount, EnumIter};

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Display, From)]
pub enum Ball {
    #[display("蓝球：{_0}")]
    Blue(BlueBall),
    #[display("红球：{_0}")]
    Red(RedBall),
}

impl Debug for Ball {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(
    Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Display, TryFrom, EnumIter, EnumCount,
)]
#[try_from(repr)]
#[repr(usize)]
pub enum BlueBall {
    #[display("1")]
    V1 = 1,
    #[display("2")]
    V2 = 2,
    #[display("3")]
    V3 = 3,
    #[display("4")]
    V4 = 4,
    #[display("5")]
    V5 = 5,
    #[display("6")]
    V6 = 6,
    #[display("7")]
    V7 = 7,
    #[display("8")]
    V8 = 8,
    #[display("9")]
    V9 = 9,
    #[display("10")]
    V10 = 10,
    #[display("11")]
    V11 = 11,
    #[display("12")]
    V12 = 12,
    #[display("13")]
    V13 = 13,
    #[display("14")]
    V14 = 14,
    #[display("15")]
    V15 = 15,
    #[display("16")]
    V16 = 16,
}

impl Debug for BlueBall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(
    Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Display, TryFrom, EnumIter, EnumCount,
)]
#[try_from(repr)]
#[repr(usize)]
pub enum RedBall {
    #[display("1")]
    V1 = 1,
    #[display("2")]
    V2 = 2,
    #[display("3")]
    V3 = 3,
    #[display("4")]
    V4 = 4,
    #[display("5")]
    V5 = 5,
    #[display("6")]
    V6 = 6,
    #[display("7")]
    V7 = 7,
    #[display("8")]
    V8 = 8,
    #[display("9")]
    V9 = 9,
    #[display("10")]
    V10 = 10,
    #[display("11")]
    V11 = 11,
    #[display("12")]
    V12 = 12,
    #[display("13")]
    V13 = 13,
    #[display("14")]
    V14 = 14,
    #[display("15")]
    V15 = 15,
    #[display("16")]
    V16 = 16,
    #[display("17")]
    V17 = 17,
    #[display("18")]
    V18 = 18,
    #[display("19")]
    V19 = 19,
    #[display("20")]
    V20 = 20,
    #[display("21")]
    V21 = 21,
    #[display("22")]
    V22 = 22,
    #[display("23")]
    V23 = 23,
    #[display("24")]
    V24 = 24,
    #[display("25")]
    V25 = 25,
    #[display("26")]
    V26 = 26,
    #[display("27")]
    V27 = 27,
    #[display("28")]
    V28 = 28,
    #[display("29")]
    V29 = 29,
    #[display("30")]
    V30 = 30,
    #[display("31")]
    V31 = 31,
    #[display("32")]
    V32 = 32,
    #[display("33")]
    V33 = 33,
}

impl Debug for RedBall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(Debug)]
pub struct PrBusinessObj {
    pub code: String,
    pub seq: usize,
    pub date: NaiveDate,
    pub blue_ball: BlueBall,
    pub red_balls: [RedBall; 6],
}
