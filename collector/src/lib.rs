pub mod error;
mod local;
mod raw;
mod remote;

use crate::error::Error;
use crate::local::collect_from_file;
use crate::raw::{PrizePage, PrizeRecord};
use crate::remote::collect_from_network;
use ssq_tool_domain::PrBusinessObj;
use tracing::info;

pub enum Collector {
    Local,
    Remote,
}

impl Collector {
    pub async fn collect(
        &self,
        recent_record_size: Option<usize>,
    ) -> Result<Vec<PrBusinessObj>, Error> {
        match self {
            Collector::Local => collect_from_file(recent_record_size).await,
            Collector::Remote => collect_from_network(recent_record_size).await,
        }
    }
}

fn collect_business_obj(
    prize_page: PrizePage,
    recent_record_size: Option<usize>,
) -> Result<Vec<PrBusinessObj>, Error> {
    let PrizePage {
        total,
        prize_records,
        ..
    } = prize_page;
    let record_size = recent_record_size.unwrap_or(total);

    let mut seq = 0;
    let mut business_objs = Vec::new();
    for prize_record in prize_records {
        let PrizeRecord {
            code,
            date,
            red,
            blue,
            week,
            ..
        } = prize_record;
        let day = format!("星期{week}");
        let red_balls = [
            red[0].try_into()?,
            red[1].try_into()?,
            red[2].try_into()?,
            red[3].try_into()?,
            red[4].try_into()?,
            red[5].try_into()?,
        ];
        let blue_ball = blue.try_into()?;
        let business_obj = PrBusinessObj {
            code,
            seq,
            date,
            blue_ball,
            red_balls,
            day,
        };
        business_objs.push(business_obj);
        seq += 1;
        if seq >= record_size {
            break;
        }
    }
    info!("共收集{}条中奖记录...", business_objs.len());
    Ok(business_objs)
}
