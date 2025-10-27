use crate::context::ProcessorContext;
use crate::error::Error;
use crate::{Processor, BLUE_BALL_FOLLOWING_OCCURRENCES};
use itertools::Itertools;
use ssq_tool_domain::BlueBall;
use std::collections::HashMap;
use tracing::info;

#[derive(Default)]
pub struct BlueBallFollowingOccurrenceProcessor;

#[async_trait::async_trait]
impl Processor for BlueBallFollowingOccurrenceProcessor {
    fn name(&self) -> &str {
        "BlueBallFollowingOccurrenceProcessor"
    }

    async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error> {
        let mut blue_ball_following_occurrences =
            HashMap::<BlueBall, HashMap<BlueBall, usize>>::new();

        //对蓝球的出现情况进行日期从小到大排序
        let prized_blue_balls = context
            .get_prize_records()
            .iter()
            .sorted_by_key(|record| record.date)
            .map(|record| record.blue_ball)
            .collect::<Vec<BlueBall>>();

        prized_blue_balls.windows(2).for_each(|records| {
            let current = records[0];
            let next = records[1];
            blue_ball_following_occurrences
                .entry(current)
                .and_modify(|next_occurrences| {
                    next_occurrences
                        .entry(next)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                })
                .or_default();
        });

        info!("蓝球后续出现情况：{blue_ball_following_occurrences:?}");
        context.set_attribute(
            &BLUE_BALL_FOLLOWING_OCCURRENCES,
            blue_ball_following_occurrences,
        );
        Ok(())
    }
}
