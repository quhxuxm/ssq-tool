use crate::context::ProcessorContext;
use crate::error::Error;
use crate::{Processor, BLUE_BALL_OCCURRENCE_FP};
use fp_growth::algorithm::FPGrowth;
use ssq_tool_domain::BlueBall;
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
        let blue_ball_occurrence_transactions_by_window_size = context
            .get_prize_records()
            .windows(self.occurrence_window_size)
            .map(|records| {
                records
                    .iter()
                    .map(|record| record.blue_ball)
                    .collect::<Vec<BlueBall>>()
            })
            .collect::<Vec<Vec<BlueBall>>>();
        let fp_growth = FPGrowth::new(
            blue_ball_occurrence_transactions_by_window_size,
            self.minimum_support,
        );
        let fp_result = fp_growth.find_frequent_patterns();
        info!(
            "蓝球出现情况频繁模式基础参数，最小支持度：{}，每 {} 期内可能出现的 {} 个频繁模式",
            self.minimum_support,
            self.occurrence_window_size,
            fp_result.frequent_patterns_num()
        );
        context.set_attribute(&BLUE_BALL_OCCURRENCE_FP, fp_result);
        Ok(())
    }
}
