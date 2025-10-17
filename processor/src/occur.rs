use std::collections::HashMap;

use crate::context_obj::OccurDetail;
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
        let total_prized_record_num = context.prize_records.len();
        context.prize_records.iter().for_each(|record| {
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
            let latest_occur_seq = occur_seqs.first().copied().unwrap_or(usize::MAX);
            let occurrence_count = occur_seqs.len();
            let all_intervals = if occurrence_count < 2 {
                vec![occur_seqs[0]]
            } else {
                occur_seqs
                    .windows(2)
                    .map(|v| v[1] - v[0])
                    .collect::<Vec<usize>>()
            };

            let average_interval = all_intervals.iter().sum::<usize>() / occurrence_count;
            let count_base_on_average_interval = total_prized_record_num / (average_interval + 1);
            ball_occurs
                .entry(*ball)
                .and_modify(|occur_info| {
                    occur_info.set_occurrence_count(occurrence_count);
                    occur_info.set_average_interval(average_interval);
                    occur_info.set_latest_occur_seq(latest_occur_seq);
                    occur_info.set_count_based_on_average_interval(count_base_on_average_interval);
                })
                .or_insert_with(|| {
                    let mut occur_info = OccurDetail::default();
                    occur_info.set_occurrence_count(occurrence_count);
                    occur_info.set_average_interval(average_interval);
                    occur_info.set_latest_occur_seq(latest_occur_seq);
                    occur_info.set_count_based_on_average_interval(count_base_on_average_interval);
                    occur_info
                });
        });
        ball_occurs.iter().for_each(|(k, v)| match k {
            Ball::Blue(ball) => {
                trace!("蓝球：{ball}，出现信息：{v:?}")
            }
            Ball::Red(ball) => {
                trace!("红球：{ball}，出现信息：{v:?}")
            }
        });
        context.set_attribute(&BALL_OCCURS, ball_occurs);
        Ok(())
    }
}
