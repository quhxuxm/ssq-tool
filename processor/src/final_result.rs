use crate::context::ProcessorContext;
use crate::error::Error;
use crate::{Processor, BLUE_BALL_OCCURRENCE_FP};
use tracing::info;

pub struct FinalResultsProcessor;

#[async_trait::async_trait]
impl Processor for FinalResultsProcessor {
    fn name(&self) -> &str {
        "FinalResultsProcessor"
    }

    async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error> {
        let blue_ball_occurrence_fp =
            context
                .get_attribute(&BLUE_BALL_OCCURRENCE_FP)
                .ok_or(Error::ContextAttrNotExist(
                    BLUE_BALL_OCCURRENCE_FP.to_string(),
                ))?;
        blue_ball_occurrence_fp
            .frequent_patterns()
            .iter()
            .filter(|val| val.0.len() > 1)
            .for_each(|(pattern, support)| {
                info!("蓝球连续出现情况频繁模式: {pattern:?}，支持度: {support}");
            });
        Ok(())
    }
}
