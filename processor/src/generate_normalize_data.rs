use crate::context::ProcessorContext;
use crate::error::Error;
use crate::{BALL_OCCURRENCE, BALL_RELATIONSHIP_FP, Processor};
use itertools::Itertools;
use ssq_tool_domain::Ball;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use tracing::warn;

pub struct GenerateNormalizeDataProcessor {
    file_path: PathBuf,
}

impl GenerateNormalizeDataProcessor {
    pub fn new(file_path: PathBuf) -> Self {
        GenerateNormalizeDataProcessor { file_path }
    }
}
#[async_trait::async_trait]
impl Processor for GenerateNormalizeDataProcessor {
    fn name(&self) -> &str {
        "GenerateNormalizeDataProcessor"
    }

    async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error> {
        let ball_occurrence = context
            .get_attribute(&BALL_OCCURRENCE)
            .ok_or(Error::ContextAttrNotExist(BALL_OCCURRENCE.to_string()))?;
        let ball_occurrence_fp = context
            .get_attribute(&BALL_RELATIONSHIP_FP)
            .ok_or(Error::ContextAttrNotExist(BALL_RELATIONSHIP_FP.to_string()))?;
        if std::fs::remove_file(&self.file_path).is_err() {
            warn!("没有旧文件：{:?}", self.file_path)
        };
        let mut output_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;
        context.get_prize_records().iter().try_for_each(|record| {
            let blue_ball=Ball::Blue(record.blue_ball);
            let blue_ball_occurrence = ball_occurrence.get(&blue_ball).unwrap();
            let top_related_red_balls=match ball_occurrence_fp.get(&record.blue_ball){
                Some(fp_result) => {
                   fp_result.frequent_patterns().iter().sorted_by_key(|pattern| pattern.1).flat_map(|pattern|{&pattern.0}).copied().unique().take(6).sorted().collect()
                },
                None => vec![],
            };

            writeln!(
                output_file,
                "{}，期号：{}，星期：{}，蓝球：{}，红球：{:?}，总中奖注数：{}，总销售注数：{}，蓝球平均出现间隔：{}，蓝球预期出现次数：{}，蓝球实际出现次数：{}，推测关联红球：{:?}",
                record.date,
                record.code,
                record.day,
                record.blue_ball,
                record.red_balls,
                record.total_prized_tickets,
                record.total_tickets,
                blue_ball_occurrence.average_occur_interval(),
                blue_ball_occurrence.occurrence_count_by_average_interval(),
                blue_ball_occurrence.occurrence_count_by_official_data(),
                top_related_red_balls
            )?;
            Result::<(), Error>::Ok(())
        })?;
        Ok(())
    }
}
