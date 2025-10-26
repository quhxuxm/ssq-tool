use crate::context::ProcessorContext;
use crate::error::Error;
use crate::{
    BLUE_BALL_AND_RED_BALL_RELATIONSHIP_FP, Processor, RED_BALL_AND_RED_BALL_RELATIONSHIP_FP,
};
use fp_growth::algorithm::{FPGrowth, FPResult};
use ssq_tool_domain::{BlueBall, RedBall};
use std::collections::HashMap;

pub struct BallRelationshipFpProcessor {
    minimum_support: usize,
}

impl BallRelationshipFpProcessor {
    pub fn new(minimum_support: usize) -> Self {
        BallRelationshipFpProcessor { minimum_support }
    }
}
#[async_trait::async_trait]
impl Processor for BallRelationshipFpProcessor {
    fn name(&self) -> &str {
        "BallRelationshipFpProcessor"
    }

    async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error> {
        let mut blue_ball_and_red_ball_transactions = HashMap::<BlueBall, Vec<Vec<RedBall>>>::new();
        context.get_prize_records().iter().for_each(|record| {
            blue_ball_and_red_ball_transactions
                .entry(record.blue_ball)
                .and_modify(|txn| {
                    txn.push(record.red_balls.to_vec());
                })
                .or_insert(vec![record.red_balls.to_vec()]);
        });

        let mut red_ball_and_red_ball_transactions = HashMap::<RedBall, Vec<Vec<RedBall>>>::new();
        context.get_prize_records().iter().for_each(|record| {
            record.red_balls.iter().for_each(|red_ball| {
                red_ball_and_red_ball_transactions
                    .entry(*red_ball)
                    .and_modify(|txn| {
                        txn.push(
                            record
                                .red_balls
                                .iter()
                                .copied()
                                .filter(|current_red_ball| current_red_ball != red_ball)
                                .collect::<Vec<RedBall>>(),
                        );
                    })
                    .or_insert(vec![
                        record
                            .red_balls
                            .iter()
                            .copied()
                            .filter(|current_red_ball| current_red_ball != red_ball)
                            .collect::<Vec<RedBall>>(),
                    ]);
            });
        });

        let blue_ball_and_red_ball_fp_growth = blue_ball_and_red_ball_transactions
            .into_iter()
            .map(|(blue_ball, red_ball_transactions)| {
                let fp_growth = FPGrowth::new(red_ball_transactions, self.minimum_support);
                (blue_ball, fp_growth.find_frequent_patterns())
            })
            .collect::<HashMap<BlueBall, FPResult<RedBall>>>();

        let red_ball_and_red_ball_fp_growth = red_ball_and_red_ball_transactions
            .into_iter()
            .map(|(red_ball, red_ball_transactions)| {
                let fp_growth = FPGrowth::new(red_ball_transactions, self.minimum_support);
                (red_ball, fp_growth.find_frequent_patterns())
            })
            .collect::<HashMap<RedBall, FPResult<RedBall>>>();

        context.set_attribute(
            &BLUE_BALL_AND_RED_BALL_RELATIONSHIP_FP,
            blue_ball_and_red_ball_fp_growth,
        );
        context.set_attribute(
            &RED_BALL_AND_RED_BALL_RELATIONSHIP_FP,
            red_ball_and_red_ball_fp_growth,
        );
        Ok(())
    }
}
