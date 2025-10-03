use std::collections::HashMap;

use tracing::{debug, info};

use crate::{
    domain::PrizeRecordCode,
    error::Error,
    processor::{
        BLUE_BALL_RELATIONSHIPS, Context, Processor, RED_BALL_RELATIONSHIPS,
        business_obj::{BallOccurInterval, BlueBallRelationship, RedBallRelationship},
    },
};

pub struct BallOccurIntervalProcessor;

#[async_trait::async_trait]
impl Processor for BallOccurIntervalProcessor {
    fn name(&self) -> &str {
        "BallOccurIntervalProcessor"
    }

    async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
        let mut blue_balls_occurs = HashMap::<u8, Vec<PrizeRecordCode>>::new();
        (1..=16).for_each(|blue_ball| {
            blue_balls_occurs.insert(blue_ball, vec![]);
        });
        for record in &context.prize_record_page.prize_records {
            blue_balls_occurs
                .entry(record.blue)
                .and_modify(|v| {
                    v.push(record.code);
                })
                .or_insert(vec![record.code]);
        }
        let mut red_balls_occurs = HashMap::<u8, Vec<PrizeRecordCode>>::new();
        (1..=33).for_each(|red_ball| {
            red_balls_occurs.insert(red_ball, vec![]);
        });
        for record in &context.prize_record_page.prize_records {
            record.red.iter().for_each(|red_ball| {
                red_balls_occurs
                    .entry(*red_ball)
                    .and_modify(|v| {
                        v.push(record.code);
                    })
                    .or_insert(vec![record.code]);
            });
        }
        debug!("蓝球出现期数：{:#?}", blue_balls_occurs);
        debug!("红球出现期数：{:#?}", red_balls_occurs);
        let mut blue_balls_accur_intervals = HashMap::<u8, Vec<i32>>::new();
        blue_balls_occurs
            .iter_mut()
            .for_each(|(k, v)| v.windows(2).map(|v1, v2| {}));
        Ok(())
    }
}
