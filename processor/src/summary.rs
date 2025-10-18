use crate::context::Relationship;
use crate::{
    BALL_OCCURS, BLUE_BALL_RELATIONSHIPS, Processor, ProcessorContext, SUMMARIES, SummaryRecord,
    error::Error,
};
use itertools::Itertools;
use rand::prelude::SliceRandom;
use ssq_tool_domain::{Ball, BlueBall, RedBall};
use tracing::{debug, info};

pub struct SummaryProcessor;

#[async_trait::async_trait]
impl Processor for SummaryProcessor {
    fn name(&self) -> &str {
        "SummaryProcessor"
    }

    async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error> {
        let ball_occurs = context
            .get_attribute(&BALL_OCCURS)
            .ok_or(Error::ContextAttrNotExist(BALL_OCCURS.to_string()))?;
        let mut blue_ball_occurs = Vec::new();
        let mut red_ball_occurs = Vec::new();
        ball_occurs.iter().for_each(|(k, v)| match k {
            Ball::Blue(blue_ball) => {
                blue_ball_occurs.push((*blue_ball, v));
            }
            Ball::Red(red_ball) => {
                red_ball_occurs.push((*red_ball, v));
            }
        });
        let blue_ball_occurs = blue_ball_occurs;

        // 平均出现次数和实际出现次数的差值从大到小排列，截取前result size个元素
        let blue_ball_occurs_sorted_by_diff_of_occurrence = blue_ball_occurs
            .iter()
            .sorted_by(|v1, v2| {
                let v2_diff = v2.1.occurance_count_by_average_interval() as isize
                    - v2.1.occurrence_count_by_official_data() as isize;
                let v1_diff = v1.1.occurance_count_by_average_interval() as isize
                    - v1.1.occurrence_count_by_official_data() as isize;
                v2_diff.cmp(&v1_diff)
            })
            .take(context.expect_result_size())
            .inspect(|v| debug!("蓝球，按照出现次数差距收集数据：{}, {:?}", v.0, v.1))
            .map(|v| v.0)
            .collect::<Vec<BlueBall>>();
        // 出现次数从大到小排列，截取前result size个元素
        let blue_ball_occurs_sorted_by_occurrence_count = blue_ball_occurs
            .iter()
            .sorted_by(|v1, v2| {
                v2.1.occurrence_count_by_official_data()
                    .cmp(&v1.1.occurrence_count_by_official_data())
            })
            .take(context.expect_result_size())
            .inspect(|v| debug!("蓝球，按照出现次数收集数据：{}, {:?}", v.0, v.1))
            .map(|v| v.0)
            .collect::<Vec<BlueBall>>();
        // 出现平均间隔从小到大排列，截取前result size个元素
        let blue_ball_occurs_sorted_by_average_interval = blue_ball_occurs
            .iter()
            .sorted_by_key(|v| v.1.average_occur_interval())
            .take(context.expect_result_size())
            .inspect(|v| debug!("蓝球，按照平均出现间隔收集数据：{}, {:?}", v.0, v.1))
            .map(|v| v.0)
            .collect::<Vec<BlueBall>>();

        let mut result_blue_balls = Vec::new();
        result_blue_balls.extend(blue_ball_occurs_sorted_by_average_interval);
        result_blue_balls.extend(blue_ball_occurs_sorted_by_diff_of_occurrence);
        result_blue_balls.extend(blue_ball_occurs_sorted_by_occurrence_count);
        let mut result_blue_balls = result_blue_balls
            .into_iter()
            .unique()
            .collect::<Vec<BlueBall>>();
        debug!("候选蓝球：{result_blue_balls:?}");
        let mut rng = rand::rng();
        result_blue_balls.shuffle(&mut rng);
        let result_blue_balls = result_blue_balls
            .into_iter()
            .take(context.expect_result_size())
            .sorted()
            .collect::<Vec<BlueBall>>();

        debug!("选中的蓝球：{result_blue_balls:?}");

        let red_ball_num = 6;

        let red_ball_occurs = red_ball_occurs;

        // 最后出现序列号从大到小排列，截取前result size个元素
        let red_ball_occurs_sorted_by_diff_of_occurrence = red_ball_occurs
            .iter()
            .sorted_by(|v1, v2| {
                let v2_diff = v2.1.occurance_count_by_average_interval() as isize
                    - v2.1.occurrence_count_by_official_data() as isize;
                let v1_diff = v1.1.occurance_count_by_average_interval() as isize
                    - v1.1.occurrence_count_by_official_data() as isize;
                v2_diff.cmp(&v1_diff)
            })
            .take(red_ball_num)
            .inspect(|v| debug!("红球，按照出现次数差距收集数据：{}, {:?}", v.0, v.1))
            .map(|v| v.0)
            .collect::<Vec<RedBall>>();
        // 出现次数从大到小排列，截取前result size个元素
        let red_ball_occurs_sorted_by_occurrence_count = red_ball_occurs
            .iter()
            .sorted_by(|v1, v2| {
                v2.1.occurrence_count_by_official_data()
                    .cmp(&v1.1.occurrence_count_by_official_data())
            })
            .take(red_ball_num)
            .inspect(|v| debug!("红球，按照出现次数收集数据：{}, {:?}", v.0, v.1))
            .map(|v| v.0)
            .collect::<Vec<RedBall>>();
        // 出现平均间隔从小到大排列，截取前result size个元素
        let red_ball_occurs_sorted_by_average_interval = red_ball_occurs
            .iter()
            .sorted_by_key(|v| v.1.average_occur_interval())
            .take(red_ball_num)
            .inspect(|v| debug!("红球，按照平均出现间隔收集数据：{}, {:?}", v.0, v.1))
            .map(|v| v.0)
            .collect::<Vec<RedBall>>();
        let blue_ball_relationship =
            context
                .get_attribute(&BLUE_BALL_RELATIONSHIPS)
                .ok_or(Error::ContextAttrNotExist(
                    BLUE_BALL_RELATIONSHIPS.to_string(),
                ))?;

        let mut result_red_balls = Vec::new();
        result_red_balls.extend(red_ball_occurs_sorted_by_average_interval);
        result_red_balls.extend(red_ball_occurs_sorted_by_diff_of_occurrence);
        result_red_balls.extend(red_ball_occurs_sorted_by_occurrence_count);
        let result_red_balls = result_red_balls
            .into_iter()
            .sorted()
            .collect::<Vec<RedBall>>();
        debug!("候选红球：{result_red_balls:?}");
        let mut rng = rand::rng();
        let mut summaries = Vec::new();
        result_blue_balls.into_iter().for_each(|blue_ball| {
            let mut candidate_red_balls = result_red_balls.clone();
            let blue_ball_relationship = blue_ball_relationship.get(&blue_ball);
            let related_red_balls = match blue_ball_relationship {
                Some(Relationship::Blue { detail, .. }) => detail
                    .iter()
                    .sorted_by(|v1, v2| v2.1.cmp(v1.1))
                    .map(|v| *v.0)
                    .take(red_ball_num)
                    .sorted()
                    .collect::<Vec<RedBall>>(),
                _ => vec![],
            };
            debug!("选出与蓝球：{blue_ball} 相关的红球：{related_red_balls:?}");
            candidate_red_balls.extend(related_red_balls);
            let mut candidate_red_balls = candidate_red_balls
                .into_iter()
                .unique()
                .sorted()
                .collect::<Vec<RedBall>>();
            debug!("蓝球：{blue_ball} 所有候选的红球：{candidate_red_balls:?}");
            candidate_red_balls.shuffle(&mut rng);
            let candidate_red_balls = candidate_red_balls
                .iter()
                .copied()
                .take(red_ball_num)
                .sorted()
                .collect::<Vec<RedBall>>();
            info!("红球：{candidate_red_balls:?}；蓝球：{blue_ball}");
            summaries.push(SummaryRecord::new(
                blue_ball,
                [
                    candidate_red_balls[0],
                    candidate_red_balls[1],
                    candidate_red_balls[2],
                    candidate_red_balls[3],
                    candidate_red_balls[4],
                    candidate_red_balls[5],
                ],
            ))
        });
        context.set_attribute(&SUMMARIES, summaries);
        Ok(())
    }
}
