use std::collections::HashMap;

use tracing::info;

use crate::{
    error::Error,
    processor::{Context, PRIZED_RED_BALLS_COUNTS, Processor},
};

pub struct CountPrizedRedballProcessor;

#[async_trait::async_trait]
impl Processor for CountPrizedRedballProcessor {
    fn name(&self) -> &str {
        "CountPrizedRedballProcessor"
    }

    async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
        let mut prized_red_ball_counts = HashMap::new();
        // init the red ball prized count with 0
        (1..=33).for_each(|red_ball| {
            prized_red_ball_counts.entry(red_ball).or_insert(0);
        });
        // count the prized times of each red ball
        context
            .prize_record_page
            .prize_records
            .iter()
            .for_each(|prized_ball| {
                let red_balls = prized_ball.red.clone();
                red_balls.iter().for_each(|red_ball| {
                    prized_red_ball_counts.entry(*red_ball).and_modify(|ball| {
                        *ball += 1;
                    });
                });
            });
        info!("红球中奖次数：\n{prized_red_ball_counts:#?}");
        context.add_attribute(PRIZED_RED_BALLS_COUNTS.clone(), prized_red_ball_counts);
        Ok(())
    }
}
