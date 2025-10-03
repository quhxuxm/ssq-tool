use std::collections::HashMap;

use tracing::info;

use crate::{
    error::Error,
    processor::{Context, PRIZED_BLUE_BALLS_COUNTS, Processor},
};

pub struct CountPrizedBlueballProcessor;

#[async_trait::async_trait]
impl Processor for CountPrizedBlueballProcessor {
    fn name(&self) -> &str {
        "CountPrizedBlueballProcessor"
    }

    async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
        let mut prized_blue_ball_counts = HashMap::new();
        // initialize the prized blue ball count with 0
        (1..=16).for_each(|blue_ball| {
            prized_blue_ball_counts.entry(blue_ball).or_insert(0);
        });
        // count the prized times of each blue ball
        context
            .prize_record_page
            .prize_records
            .iter()
            .for_each(|prized_ball| {
                let blue_ball = prized_ball.blue;
                prized_blue_ball_counts.entry(blue_ball).and_modify(|ball| {
                    *ball += 1;
                });
            });
        info!("蓝球中奖次数：\n{prized_blue_ball_counts:#?}");
        context.add_attribute(PRIZED_BLUE_BALLS_COUNTS.clone(), prized_blue_ball_counts);
        Ok(())
    }
}
