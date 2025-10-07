use std::{collections::HashMap, sync::Arc};

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
        let mut blue_balls_occur_indexes = HashMap::<usize, Vec<usize>>::new();
        let mut red_balls_occur_indexes = HashMap::<usize, Vec<usize>>::new();
        context.prize_records.iter().for_each(|record| {
            blue_balls_occur_indexes
                .entry(record.blue_ball())
                .and_modify(|occur_indexes| {
                    occur_indexes.push(record.index());
                })
                .or_insert(vec![record.index()]);
            record.red_balls().iter().for_each(|red_ball| {
                red_balls_occur_indexes
                    .entry(*red_ball)
                    .and_modify(|occur_indexes| {
                        occur_indexes.push(record.index());
                    })
                    .or_insert(vec![record.index()]);
            });
        });
        debug!("蓝球出现索引：{:?}", blue_balls_occur_indexes);
        debug!("红球出现索引：{:?}", red_balls_occur_indexes);
        let mut blue_balls_occurs = HashMap::<usize, BallOccurInfo>::new();
        let mut red_balls_occurs = HashMap::<usize, BallOccurInfo>::new();
        blue_balls_occur_indexes
            .iter()
            .for_each(|(ball, occur_indexes)| {
                let last_occur_index = occur_indexes.first().copied().unwrap_or(usize::MAX);
                blue_balls_occurs
                    .entry(*ball)
                    .and_modify(|occur_info| {
                        occur_info.set_last_occur_index(last_occur_index);
                        occur_info.set_occur_count(occur_indexes.len());
                        if occur_indexes.len() < 2 {
                            occur_info.add_interval(occur_indexes[0]);
                        } else {
                            occur_indexes.windows(2).for_each(|two_occur_indexes| {
                                occur_info
                                    .add_interval(two_occur_indexes[1] - two_occur_indexes[0]);
                            });
                        }
                    })
                    .or_insert_with(|| {
                        let mut occur_info = BallOccurInfo::new(*ball);
                        occur_info.set_last_occur_index(last_occur_index);
                        occur_info.set_occur_count(occur_indexes.len());
                        if occur_indexes.len() < 2 {
                            occur_info.add_interval(occur_indexes[0]);
                        } else {
                            occur_indexes.windows(2).for_each(|two_occur_indexes| {
                                occur_info
                                    .add_interval(two_occur_indexes[1] - two_occur_indexes[0]);
                            });
                        }
                        occur_info
                    });
            });

        red_balls_occur_indexes
            .iter()
            .for_each(|(ball, occur_indexes)| {
                let last_occur_index = occur_indexes.first().copied().unwrap_or(usize::MAX);
                red_balls_occurs
                    .entry(*ball)
                    .and_modify(|occur_info| {
                        occur_info.set_last_occur_index(last_occur_index);
                        occur_info.set_occur_count(occur_indexes.len());
                        if occur_indexes.len() < 2 {
                            occur_info.add_interval(occur_indexes[0]);
                        } else {
                            occur_indexes.windows(2).for_each(|two_occur_indexes| {
                                occur_info
                                    .add_interval(two_occur_indexes[1] - two_occur_indexes[0]);
                            });
                        }
                    })
                    .or_insert_with(|| {
                        let mut occur_info = BallOccurInfo::new(*ball);
                        occur_info.set_last_occur_index(last_occur_index);
                        occur_info.set_occur_count(occur_indexes.len());
                        if occur_indexes.len() < 2 {
                            occur_info.add_interval(occur_indexes[0]);
                        } else {
                            occur_indexes.windows(2).for_each(|two_occur_indexes| {
                                occur_info
                                    .add_interval(two_occur_indexes[1] - two_occur_indexes[0]);
                            });
                        }
                        occur_info
                    });
            });

        let prized_blue_balls = blue_balls_occurs.keys().copied().collect::<Vec<usize>>();
        prized_blue_balls.into_iter().for_each(|blue_ball| {
            blue_balls_occurs.entry(blue_ball).and_modify(|occur_info| {
                let max_interval = occur_info
                    .intervals()
                    .iter()
                    .max()
                    .copied()
                    .unwrap_or(usize::MAX);
                let min_interval = occur_info
                    .intervals()
                    .iter()
                    .min()
                    .copied()
                    .unwrap_or(usize::MAX);
                let average_interval =
                    occur_info.intervals().iter().sum::<usize>() / occur_info.occur_count();
                let average_occur_possibility =
                    (occur_info.occur_count() as f64) / (average_interval as f64);
                occur_info.set_max_interval(max_interval);
                occur_info.set_min_interval(min_interval);
                occur_info.set_average_interval(average_interval);
                occur_info.set_average_occur_possibility(average_occur_possibility);
                occur_info.set_possible_next_occur_index(
                    (occur_info.last_occur_index() as isize) - (average_interval as isize),
                );
            });
        });
        let prized_red_balls = red_balls_occurs.keys().copied().collect::<Vec<usize>>();
        prized_red_balls.into_iter().for_each(|red_ball| {
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
                let average_interval =
                    occur_info.intervals().iter().sum::<usize>() / occur_info.occur_count();
                let average_occur_possibility =
                    (occur_info.occur_count() as f64) / (average_interval as f64);
                occur_info.set_max_interval(max_interval);
                occur_info.set_min_interval(min_interval);
                occur_info.set_average_interval(average_interval);
                occur_info.set_average_occur_possibility(average_occur_possibility);
                occur_info.set_possible_next_occur_index(
                    (occur_info.last_occur_index() as isize) - (average_interval as isize),
                );
            });
        });
        debug!("蓝球出现情况：{:?}", blue_balls_occurs);
        let blue_balls_occurs = blue_balls_occurs
            .into_iter()
            .map(|(k, v)| (k, Arc::new(v)))
            .collect::<HashMap<usize, Arc<BallOccurInfo>>>();
        context.add_attribute(PRIZED_BLUE_BALLS_OCCUR_INFO.clone(), blue_balls_occurs);
        debug!("红球出现情况：{:?}", red_balls_occurs);
        let red_balls_occurs = red_balls_occurs
            .into_iter()
            .map(|(k, v)| (k, Arc::new(v)))
            .collect::<HashMap<usize, Arc<BallOccurInfo>>>();
        context.add_attribute(PRIZED_RED_BALLS_OCCUR_INFO.clone(), red_balls_occurs);
        Ok(())
    }
}
