use crate::error::Error;
use crate::processor::context_obj::BallOccurInfo;
use crate::processor::{
    Context,
    Processor,
    // RED_BALL_RELATIONSHIPS,
    BLUE_BALL_RELATIONSHIPS,
    MOST_POSSIBLE_OCCUR_BLUE_BALLS,
    MOST_POSSIBLE_OCCUR_RED_BALLS,
};
use itertools::Itertools;
use rand::rng;
use rand::seq::SliceRandom;
use std::sync::Arc;
use tracing::{debug, info};

pub struct RelationshipBasedProcessor;

#[async_trait::async_trait]
impl Processor for RelationshipBasedProcessor {
    fn name(&self) -> &str {
        "RelationshipBasedProcessor"
    }

    async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
        // let red_ball_relationships =
        //     context
        //         .attribute(&RED_BALL_RELATIONSHIPS)
        //         .ok_or(Error::ContextAttrNotExist(
        //             RED_BALL_RELATIONSHIPS.name.clone(),
        //         ))?;
        let blue_ball_relationships =
            context
                .attribute(&BLUE_BALL_RELATIONSHIPS)
                .ok_or(Error::ContextAttrNotExist(
                    BLUE_BALL_RELATIONSHIPS.name.clone(),
                ))?;
        let most_possible_occur_blue_balls =
            context.attribute(&MOST_POSSIBLE_OCCUR_BLUE_BALLS).ok_or(
                Error::ContextAttrNotExist(MOST_POSSIBLE_OCCUR_BLUE_BALLS.name.clone()),
            )?;
        let most_possible_occur_red_balls = context
            .attribute(&MOST_POSSIBLE_OCCUR_RED_BALLS)
            .ok_or(Error::ContextAttrNotExist(
                MOST_POSSIBLE_OCCUR_RED_BALLS.name.clone(),
            ))?;
        let mut top_n_blue_balls_by_next_index = most_possible_occur_blue_balls
            .clone()
            .into_iter()
            .collect::<Vec<Arc<BallOccurInfo>>>();
        top_n_blue_balls_by_next_index.sort_by(|v1, v2| {
            v2.possible_next_occur_index()
                .cmp(&v1.possible_next_occur_index())
        });
        let mut top_n_blue_balls_by_next_index = top_n_blue_balls_by_next_index
            .into_iter()
            .take(5)
            .collect::<Vec<Arc<BallOccurInfo>>>();
        top_n_blue_balls_by_next_index.sort_by_key(|v1| v1.ball());

        let mut top_n_blue_balls_by_possibility = most_possible_occur_blue_balls
            .clone()
            .into_iter()
            .collect::<Vec<Arc<BallOccurInfo>>>();
        top_n_blue_balls_by_possibility.sort_by(|v1, v2| {
            v2.average_occur_possibility()
                .total_cmp(&v1.average_occur_possibility())
        });
        let mut top_n_blue_balls_by_possibility = top_n_blue_balls_by_possibility
            .into_iter()
            .take(5)
            .collect::<Vec<Arc<BallOccurInfo>>>();
        top_n_blue_balls_by_possibility.sort_by_key(|v1| v1.ball());

        let mut top_n_blue_balls = top_n_blue_balls_by_next_index;
        top_n_blue_balls.extend(top_n_blue_balls_by_possibility);

        let mut top_n_blue_balls = top_n_blue_balls
            .iter()
            .unique_by(|blue_ball| blue_ball.ball())
            .cloned()
            .collect::<Vec<Arc<BallOccurInfo>>>();
        let mut rng = rng();
        top_n_blue_balls.shuffle(&mut rng);
        let top_n_blue_balls_for_log = top_n_blue_balls
            .iter()
            .map(|blue_ball| blue_ball.ball())
            .collect::<Vec<usize>>();
        debug!("根据出现可能性和下次出现索引生成待选蓝球：{top_n_blue_balls_for_log:?}",);
        let mut top_n_blue_balls = top_n_blue_balls
            .into_iter()
            .take(5)
            .collect::<Vec<Arc<BallOccurInfo>>>();
        top_n_blue_balls.sort_by_key(|blue_ball| blue_ball.ball());

        top_n_blue_balls.iter().for_each(|blue_ball| {
            let blue_ball_relationship = blue_ball_relationships.get(&blue_ball.ball());
            let blue_ball_relationship = match blue_ball_relationship {
                None => return,
                Some(blue_ball_relationship) => blue_ball_relationship,
            };
            let mut sorted_related_red_ball_counts = blue_ball_relationship
                .related_red_ball_counts()
                .iter()
                .map(|(k, v)| (*k, *v))
                .collect::<Vec<(usize, usize)>>();
            sorted_related_red_ball_counts.sort_by(|v1, v2| v2.1.cmp(&v1.1));
            let mut red_balls = sorted_related_red_ball_counts
                .iter()
                .take(6)
                .map(|(ball, _)| *ball)
                .collect::<Vec<usize>>();
            red_balls.extend(
                most_possible_occur_red_balls
                    .iter()
                    .map(|occur| occur.ball())
                    .take(6),
            );

            let mut red_balls = red_balls.into_iter().unique().collect::<Vec<usize>>();
            red_balls.sort();
            debug!("根据关联关系生成待选红球：{:?}", red_balls);

            red_balls.shuffle(&mut rng);
            let mut red_balls = red_balls.into_iter().take(6).collect::<Vec<usize>>();

            red_balls.sort();
            info!(
                "根据关联关系选出，蓝球：{}，红球：{:?}",
                blue_ball.ball(),
                red_balls
            );
        });
        Ok(())
    }
}
