use std::collections::HashMap;

use tracing::debug;

use crate::{
    error::Error,
    processor::{
        Context, PRIZED_BLUE_BALLS_OCCUR_INFO, PRIZED_RED_BALLS_OCCUR_INFO, Processor,
        context_obj::BallOccurInfo,
    },
};

pub struct BallOccurProcessor;

#[async_trait::async_trait]
impl Processor for BallOccurProcessor {
    fn name(&self) -> &str {
        "BallOccurProcessor"
    }

    async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
        let mut blue_balls_occur_indexes = HashMap::new();
        (1..=16).for_each(|blue_ball| {
            blue_balls_occur_indexes.insert(blue_ball, vec![]);
        });
        let mut red_balls_occur_indexes = HashMap::new();
        (1..=33).for_each(|red_ball| {
            red_balls_occur_indexes.insert(red_ball, vec![]);
        });
        context.prize_records.iter().for_each(|record| {
            blue_balls_occur_indexes
                .entry(record.blue_ball())
                .and_modify(|occur_indexes| {
                    occur_indexes.push(record.index());
                });
            record.red_balls().iter().for_each(|red_ball| {
                red_balls_occur_indexes
                    .entry(*red_ball)
                    .and_modify(|occur_indexes| {
                        occur_indexes.push(record.index());
                    });
            });
        });
        debug!("蓝球出现索引：{:?}", blue_balls_occur_indexes);
        debug!("红球出现索引：{:?}", red_balls_occur_indexes);
        let mut blue_balls_occurs = HashMap::new();
        (1..=16).for_each(|blue_ball| {
            blue_balls_occurs.insert(blue_ball, BallOccurInfo::new(blue_ball));
        });
        let mut red_balls_occurs = HashMap::new();
        (1..=33).for_each(|red_ball| {
            red_balls_occurs.insert(red_ball, BallOccurInfo::new(red_ball));
        });
        blue_balls_occur_indexes
            .iter()
            .for_each(|(ball, occur_indexes)| {
                blue_balls_occurs.entry(*ball).and_modify(|occur_info| {
                    let last_occur_index = occur_indexes.first().copied().unwrap_or(0);
                    occur_info.set_last_occur_index(last_occur_index);
                    occur_info.set_occur_count(occur_indexes.len());
                    occur_indexes.windows(2).for_each(|two_occur_indexes| {
                        occur_info.add_interval(two_occur_indexes[1] - two_occur_indexes[0]);
                    });
                });
            });

        red_balls_occur_indexes
            .iter()
            .for_each(|(ball, occur_indexes)| {
                red_balls_occurs.entry(*ball).and_modify(|occur_info| {
                    let last_occur_index = occur_indexes.first().copied().unwrap_or(0);
                    occur_info.set_last_occur_index(last_occur_index);
                    occur_info.set_occur_count(occur_indexes.len());
                    occur_indexes.windows(2).for_each(|two_occur_indexes| {
                        occur_info.add_interval(two_occur_indexes[1] - two_occur_indexes[0]);
                    });
                });
            });
        (1..=16).for_each(|blue_ball| {
            blue_balls_occurs.entry(blue_ball).and_modify(|occur_info| {
                let max_interval = occur_info
                    .intervals()
                    .iter()
                    .max()
                    .copied()
                    .unwrap_or(0usize);
                let min_interval = occur_info
                    .intervals()
                    .iter()
                    .min()
                    .copied()
                    .unwrap_or(0usize);
                let average_interval = (occur_info.intervals().iter().sum::<usize>() as f64)
                    / (occur_info.occur_count() as f64);
                let average_occur_possibility =
                    (occur_info.occur_count() as f64) / average_interval;
                occur_info.set_max_interval(max_interval);
                occur_info.set_min_interval(min_interval);
                occur_info.set_average_interval(average_interval);
                occur_info.set_average_occur_possibility(average_occur_possibility);
                occur_info.set_possible_next_occur_index(
                    (occur_info.last_occur_index() as f64) - average_interval,
                );
            });
        });
        (1..=33).for_each(|red_ball| {
            red_balls_occurs.entry(red_ball).and_modify(|occur_info| {
                let max_interval = occur_info
                    .intervals()
                    .iter()
                    .max()
                    .copied()
                    .unwrap_or(0usize);
                let min_interval = occur_info
                    .intervals()
                    .iter()
                    .min()
                    .copied()
                    .unwrap_or(0usize);
                let average_interval = (occur_info.intervals().iter().sum::<usize>() as f64)
                    / (occur_info.occur_count() as f64);
                let average_occur_possibility =
                    (occur_info.occur_count() as f64) / average_interval;
                occur_info.set_max_interval(max_interval);
                occur_info.set_min_interval(min_interval);
                occur_info.set_average_interval(average_interval);
                occur_info.set_average_occur_possibility(average_occur_possibility);
                occur_info.set_possible_next_occur_index(
                    (occur_info.last_occur_index() as f64) - average_interval,
                );
            });
        });
        debug!("蓝球出现情况：{:?}", blue_balls_occurs);
        context.add_attribute(PRIZED_BLUE_BALLS_OCCUR_INFO.clone(), blue_balls_occurs);
        debug!("红球出现情况：{:?}", red_balls_occurs);
        context.add_attribute(PRIZED_RED_BALLS_OCCUR_INFO.clone(), red_balls_occurs);
        Ok(())
    }
}
