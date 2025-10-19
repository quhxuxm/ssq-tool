use crate::context::ProcessorContext;
use crate::error::Error;
use crate::{Processor, BLUE_BALL_OCCURRENCE_FP};
use itertools::Itertools;
use ssq_tool_domain::BlueBall;
use std::collections::HashMap;
use tracing::{info, trace};

pub struct FinalResultsProcessor {
    result_size: usize,
}

impl FinalResultsProcessor {
    pub fn new(result_size: usize) -> Self {
        Self { result_size }
    }
}

#[async_trait::async_trait]
impl Processor for FinalResultsProcessor {
    fn name(&self) -> &str {
        "FinalResultsProcessor"
    }

    async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error> {
        let latest_n_blue_balls = context
            .get_prize_records()
            .iter()
            .take(self.result_size)
            .map(|record| record.blue_ball)
            .collect::<Vec<BlueBall>>();
        let blue_ball_occurrence_fp =
            context
                .get_attribute(&BLUE_BALL_OCCURRENCE_FP)
                .ok_or(Error::ContextAttrNotExist(
                    BLUE_BALL_OCCURRENCE_FP.to_string(),
                ))?;

        let blue_ball_frequent_patterns = blue_ball_occurrence_fp
            .frequent_patterns()
            .iter()
            .filter(|val| val.0.len() > 1)
            .inspect(|(pattern, support)| {
                info!("蓝球连续出现情况频繁模式: {pattern:?}，支持度: {support}");
            })
            .map(|(pattern, _)| pattern.to_vec())
            .collect::<Vec<Vec<BlueBall>>>();

        let mut most_possible_blue_balls = HashMap::<BlueBall, Vec<BlueBall>>::new();
        blue_ball_frequent_patterns.iter().for_each(|pattern| {
            pattern.iter().copied().for_each(|blue_ball| {
                most_possible_blue_balls
                    .entry(blue_ball)
                    .and_modify(|related_balls| {
                        related_balls.extend(pattern.to_vec());
                        *related_balls = related_balls
                            .iter()
                            .copied()
                            .unique()
                            .filter(|&current| blue_ball != current)
                            .collect::<Vec<BlueBall>>();
                    })
                    .or_insert(
                        pattern
                            .iter()
                            .copied()
                            .filter(|&current| blue_ball != current)
                            .collect(),
                    );
            })
        });

        trace!("可能出现的蓝球序列：{most_possible_blue_balls:?}");
        let mut result_blue_balls = Vec::<BlueBall>::new();
        latest_n_blue_balls.iter().for_each(|ball| {
            let related_blue_balls = most_possible_blue_balls
                .get(ball)
                .cloned()
                .unwrap_or(vec![]);
            result_blue_balls.extend(related_blue_balls);
        });
        let result_blue_balls = result_blue_balls
            .into_iter()
            .unique()
            .collect::<Vec<BlueBall>>();

        info!("选定的蓝球序列：{result_blue_balls:?}");
        Ok(())
    }
}
