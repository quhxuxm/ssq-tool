use std::collections::HashMap;

use crate::context::OccurDetail;
use crate::{BALL_OCCURS, Processor, ProcessorContext, error::Error};
use ssq_tool_domain::Ball;
use tracing::trace;

pub struct OccurProcessor;

#[async_trait::async_trait]
impl Processor for OccurProcessor {
    fn name(&self) -> &str {
        "OccurProcessor"
    }

    async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error> {
        let mut balls_occur_seq = HashMap::<Ball, Vec<usize>>::new();
        let total_prized_record_num = context.get_prize_records().len();
        context.get_prize_records().iter().for_each(|record| {
            balls_occur_seq
                .entry(record.blue_ball.into())
                .and_modify(|seqs| {
                    seqs.push(record.seq);
                })
                .or_insert(vec![record.seq]);
            record.red_balls.iter().for_each(|red_ball| {
                balls_occur_seq
                    .entry((*red_ball).into())
                    .and_modify(|seqs| {
                        seqs.push(record.seq);
                    })
                    .or_insert(vec![record.seq]);
            });
        });
        balls_occur_seq.iter().for_each(|(k, v)| match k {
            Ball::Blue(ball) => {
                trace!("红球 {ball} 出现索引：{v:?}");
            }
            Ball::Red(ball) => {
                trace!("蓝球 {ball} 出现索引：{v:?}");
            }
        });
        let mut ball_occurs = HashMap::<Ball, OccurDetail>::new();
        balls_occur_seq.iter().for_each(|(ball, occur_seqs)| {
            let latest_occur_seq = occur_seqs[0];
            let occurrence_count = occur_seqs.len();
            let all_intervals = if occurrence_count < 2 {
                // 如果只有一个元素
                vec![latest_occur_seq]
            } else {
                // 如果有多于一个元素
                occur_seqs
                    .windows(2)
                    .map(|v| v[1] - v[0])
                    .collect::<Vec<usize>>()
            };
            let average_interval = all_intervals.iter().sum::<usize>() / occurrence_count;
            trace!("当前双色球：{ball}，平均出现间隔：{average_interval}");
            let occurence_count_by_average_interval =
                total_prized_record_num / (average_interval + 1);
            trace!("当前双色球：{ball}，按照平均出现间隔计算应该出现的次数是：{occurence_count_by_average_interval}");
            ball_occurs
                .entry(*ball)
                .and_modify(|occur_info| {
                    occur_info.set_occurrence_count_by_official_data(occurrence_count);
                    occur_info.set_average_occur_interval(average_interval);
                    occur_info.set_latest_occur_seq(latest_occur_seq);
                    occur_info.set_occurrence_count_by_average_interval(
                        occurence_count_by_average_interval,
                    );
                })
                .or_insert_with(|| {
                    let mut occur_info = OccurDetail::default();
                    occur_info.set_occurrence_count_by_official_data(occurrence_count);
                    occur_info.set_average_occur_interval(average_interval);
                    occur_info.set_latest_occur_seq(latest_occur_seq);
                    occur_info.set_occurrence_count_by_average_interval(
                        occurence_count_by_average_interval,
                    );
                    occur_info
                });
        });
        context.set_attribute(&BALL_OCCURS, ball_occurs);
        Ok(())
    }
}
