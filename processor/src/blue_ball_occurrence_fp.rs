use crate::context::ProcessorContext;
use crate::error::Error;
use crate::{Processor, BLUE_BALL_NEXT_OCCURRENCES};
use fp_growth::algorithm::FPGrowth;
use itertools::Itertools;
use ssq_tool_domain::BlueBall;
use std::collections::HashMap;
use tracing::info;

pub struct BlueBallOccurrenceFpProcessor {
    minimum_support: usize,
    occurrence_window_size: usize,
}

impl BlueBallOccurrenceFpProcessor {
    pub fn new(
        minimum_support: usize,
        occurrence_window_size: usize,
    ) -> BlueBallOccurrenceFpProcessor {
        BlueBallOccurrenceFpProcessor {
            minimum_support,
            occurrence_window_size,
        }
    }
}

#[async_trait::async_trait]
impl Processor for BlueBallOccurrenceFpProcessor {
    fn name(&self) -> &str {
        "BlueBallOccurrenceFpGrowth"
    }

    async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error> {
        let mut blue_ball_following_occurrence =
            HashMap::<BlueBall, HashMap<BlueBall, usize>>::new();

        let prized_blue_balls = context
            .get_prize_records()
            .iter()
            .sorted_by_key(|record| record.date)
            .map(|record| record.blue_ball)
            .collect::<Vec<BlueBall>>();

        prized_blue_balls.windows(2).for_each(|records| {
            let current = records[0];
            let next = records[1];
            blue_ball_following_occurrence
                .entry(current)
                .and_modify(|next_occurrences| {
                    next_occurrences
                        .entry(next)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                })
                .or_default();
        });

        info!("蓝球后续出现情况：{blue_ball_following_occurrence:?}");
        context.set_attribute(&BLUE_BALL_NEXT_OCCURRENCES, blue_ball_following_occurrence);
        Ok(())
    }
}
