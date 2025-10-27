use crate::context::ProcessorContext;
use crate::error::Error;
use crate::{
    FinalProcessorChainResult, Processor,
    BLUE_BALL_AND_RED_BALL_RELATIONSHIP_FP, BLUE_BALL_FOLLOWING_OCCURRENCES, FINAL_PROCESSOR_CHAIN_RESULTS,
};
use itertools::Itertools;
use ssq_tool_domain::{BlueBall, RedBall};
use tracing::info;

pub struct FinalResultsProcessor {
    final_result_size: usize,
}

impl FinalResultsProcessor {
    pub fn new(final_result_size: usize) -> Self {
        Self { final_result_size }
    }
}

#[async_trait::async_trait]
impl Processor for FinalResultsProcessor {
    fn name(&self) -> &str {
        "FinalResultsProcessor"
    }

    async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error> {
        // 对中奖记录按照从最近到最早进行排序
        let sorted_blue_balls = context
            .get_prize_records()
            .iter()
            .sorted_by_key(|record| record.date)
            .rev()
            .map(|record| record.blue_ball)
            .collect::<Vec<BlueBall>>();

        let blue_ball_following_occurrences = context
            .get_attribute(&BLUE_BALL_FOLLOWING_OCCURRENCES)
            .ok_or(Error::ContextAttrNotExist(
                BLUE_BALL_FOLLOWING_OCCURRENCES.to_string(),
            ))?;
        // 最近一次中奖蓝球
        let last_occur_blue_ball = sorted_blue_balls[0];
        let following_blue_ball_occurrences =
            blue_ball_following_occurrences.get(&last_occur_blue_ball);
        info!(
            "最后一次中奖的篮球 {last_occur_blue_ball} 后续可能出现的蓝球情况：{following_blue_ball_occurrences:?}"
        );
        let mut result_blue_balls = Vec::<BlueBall>::new();
        if let Some(possible_blue_balls) = following_blue_ball_occurrences {
            // 按照出现次数从大到小排列
            possible_blue_balls
                .iter()
                .sorted_by_key(|kv| kv.1)
                .rev()
                .for_each(|(ball, _)| {
                    result_blue_balls.push(*ball);
                });
        }
        info!(
            "最后一次中奖的篮球 {last_occur_blue_ball} 后续可能出现的蓝球序列：{result_blue_balls:?}"
        );
        let result_blue_balls = result_blue_balls
            .into_iter()
            .unique()
            .take(self.final_result_size)
            .collect::<Vec<BlueBall>>();
        let red_ball_occurrence_fp = context
            .get_attribute(&BLUE_BALL_AND_RED_BALL_RELATIONSHIP_FP)
            .ok_or(Error::ContextAttrNotExist(
                BLUE_BALL_AND_RED_BALL_RELATIONSHIP_FP.to_string(),
            ))?;

        let mut final_results = Vec::<FinalProcessorChainResult>::new();
        result_blue_balls.iter().try_for_each(|blue_ball| {
            let top_related_red_balls = red_ball_occurrence_fp
                .get(blue_ball)
                .map(|red_ball_fp_result| {
                    let red_ball_occurrence_fp_pattern = red_ball_fp_result.frequent_patterns();
                    let red_ball_occurrence_fp_ave_support = red_ball_occurrence_fp_pattern
                        .iter()
                        .map(|pattern| pattern.1)
                        .sum::<usize>()
                        / red_ball_occurrence_fp_pattern.len();
                    red_ball_occurrence_fp_pattern
                        .iter()
                        .sorted_by_key(|pattern| pattern.1)
                        .rev()
                        .filter(|pattern| pattern.1 > red_ball_occurrence_fp_ave_support)
                        .take(6)
                        .flat_map(|pattern| &pattern.0)
                        .copied()
                        .unique()
                        .sorted()
                        .take(6)
                        .collect::<Vec<RedBall>>()
                })
                .ok_or(Error::OtherFailure(format!(
                    "没有找到蓝球出现情况：{blue_ball}"
                )))?;
            let one_result =
                FinalProcessorChainResult::new(*blue_ball, top_related_red_balls.to_vec());
            final_results.push(one_result);

            Ok::<(), Error>(())
        })?;
        context.set_attribute(&FINAL_PROCESSOR_CHAIN_RESULTS, final_results);
        Ok(())
    }
}
