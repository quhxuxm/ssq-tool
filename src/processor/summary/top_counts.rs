use tracing::info;

use crate::{
    error::Error,
    processor::{
        Context, PRIZED_BLUE_BALLS_COUNTS, PRIZED_RED_BALLS_COUNTS, Processor,
        SORTED_BLUE_BALLS_COUNTS, SORTED_RED_BALLS_COUNTS,
    },
};

pub struct SummaryTopCountsProcessor;

#[async_trait::async_trait]
impl Processor for SummaryTopCountsProcessor {
    fn name(&self) -> &str {
        "SummaryTopCountsProcessor"
    }

    async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
        let prized_blue_balls_counts =
            context
                .attribute(&PRIZED_BLUE_BALLS_COUNTS)
                .ok_or(Error::ContextAttrNotExist(
                    PRIZED_BLUE_BALLS_COUNTS.name.clone(),
                ))?;
        let mut result_blue_balls_and_counts = prized_blue_balls_counts
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<(u8, i32)>>();
        result_blue_balls_and_counts.sort_by(|v1, v2| v2.1.cmp(&v1.1));
        info!("sorted blue balls and counts: {result_blue_balls_and_counts:?}");
        context.add_attribute(
            SORTED_BLUE_BALLS_COUNTS.clone(),
            result_blue_balls_and_counts,
        );
        let prized_red_balls_counts =
            context
                .attribute(&PRIZED_RED_BALLS_COUNTS)
                .ok_or(Error::ContextAttrNotExist(
                    PRIZED_RED_BALLS_COUNTS.name.clone(),
                ))?;
        let mut result_red_balls_and_counts = prized_red_balls_counts
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<(u8, i32)>>();
        result_red_balls_and_counts.sort_by(|v1, v2| v2.1.cmp(&v1.1));
        info!("sorted red balls and counts: {result_red_balls_and_counts:?}");
        context.add_attribute(SORTED_RED_BALLS_COUNTS.clone(), result_red_balls_and_counts);
        Ok(())
    }
}
