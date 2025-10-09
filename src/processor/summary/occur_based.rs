use std::{cmp::Ordering, sync::Arc};

use tracing::{debug, info};

use crate::processor::{MOST_POSSIBLE_OCCUR_BLUE_BALLS, MOST_POSSIBLE_OCCUR_RED_BALLS};
use crate::{
    error::Error,
    processor::{
        Context, PRIZED_BLUE_BALLS_OCCUR_INFO, PRIZED_RED_BALLS_OCCUR_INFO, Processor,
        context_obj::BallOccurInfo,
    },
};

pub struct OccurBasedSummaryProcessor;

#[async_trait::async_trait]
impl Processor for OccurBasedSummaryProcessor {
    fn name(&self) -> &str {
        "OccurBasedSummaryProcessor"
    }

    async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
        let prized_blue_balls_occur_info =
            context
                .attribute(&PRIZED_BLUE_BALLS_OCCUR_INFO)
                .ok_or(Error::ContextAttrNotExist(
                    PRIZED_BLUE_BALLS_OCCUR_INFO.name.clone(),
                ))?;
        let mut most_possible_occur_blue_balls = prized_blue_balls_occur_info
            .values()
            .cloned()
            .collect::<Vec<Arc<BallOccurInfo>>>();
        Self::sort_ball_occurs(&mut most_possible_occur_blue_balls);
        debug!("按照概率可能出现的蓝球是：{most_possible_occur_blue_balls:?}");
        let prized_red_balls_occur_info =
            context
                .attribute(&PRIZED_RED_BALLS_OCCUR_INFO)
                .ok_or(Error::ContextAttrNotExist(
                    PRIZED_RED_BALLS_OCCUR_INFO.name.clone(),
                ))?;

        let mut most_possible_occur_red_balls = prized_red_balls_occur_info
            .values()
            .cloned()
            .collect::<Vec<Arc<BallOccurInfo>>>();
        Self::sort_ball_occurs(&mut most_possible_occur_red_balls);

        debug!("按照概率可能出现的红球是：{most_possible_occur_red_balls:?}");
        let mut top_n_blue_balls = most_possible_occur_blue_balls
            .iter()
            .take(5)
            .map(|occur_info| occur_info.ball())
            .collect::<Vec<usize>>();
        top_n_blue_balls.sort();
        let mut top_6_red_balls = most_possible_occur_red_balls
            .iter()
            .take(6)
            .map(|occur_info| occur_info.ball())
            .collect::<Vec<usize>>();
        top_6_red_balls.sort();
        top_n_blue_balls.iter().for_each(|blue| {
            info!("根据出现规律选出，蓝球：{blue}; 红球：{top_6_red_balls:?}");
        });

        context.add_attribute(
            MOST_POSSIBLE_OCCUR_BLUE_BALLS.clone(),
            most_possible_occur_blue_balls,
        );
        context.add_attribute(
            MOST_POSSIBLE_OCCUR_RED_BALLS.clone(),
            most_possible_occur_red_balls,
        );

        Ok(())
    }
}

impl OccurBasedSummaryProcessor {
    fn sort_ball_occurs(most_possible_occur_balls: &mut [Arc<BallOccurInfo>]) {
        most_possible_occur_balls.sort_by(|v1, v2| {
            let ordering = v2
                .possible_next_occur_index()
                .cmp(&v1.possible_next_occur_index());
            match ordering {
                Ordering::Equal => v2
                    .average_occur_possibility()
                    .total_cmp(&v1.average_occur_possibility()),
                other => other,
            }
        });
    }
}
