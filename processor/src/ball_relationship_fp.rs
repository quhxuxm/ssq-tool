use crate::context::ProcessorContext;
use crate::error::Error;
use crate::{Processor, BALL_RELATIONSHIP_FP};
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
        let mut ball_transactions = HashMap::<BlueBall, Vec<Vec<RedBall>>>::new();
        context.get_prize_records().iter().for_each(|record| {
            ball_transactions
                .entry(record.blue_ball)
                .and_modify(|txn| {
                    txn.push(record.red_balls.to_vec());
                })
                .or_insert(vec![record.red_balls.to_vec()]);
        });

        let ball_fp_growth = ball_transactions
            .into_iter()
            .map(|(blue_ball, red_ball_transactions)| {
                let fp_growth = FPGrowth::new(red_ball_transactions, self.minimum_support);
                (blue_ball, fp_growth.find_frequent_patterns())
            })
            .collect::<HashMap<BlueBall, FPResult<RedBall>>>();
        context.set_attribute(&BALL_RELATIONSHIP_FP, ball_fp_growth);
        Ok(())
    }
}
