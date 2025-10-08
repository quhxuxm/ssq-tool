use crate::error::Error;
use crate::processor::context_obj::BallOccurInfo;
use crate::processor::{
    // RED_BALL_RELATIONSHIPS,
    Context,
    Processor,
    BLUE_BALL_RELATIONSHIPS,
    MOST_POSSIBLE_OCCUR_BLUE_BALLS,
    MOST_POSSIBLE_OCCUR_RED_BALLS,
};
use rand::{rng, Rng};
use std::collections::HashSet;
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
        let mut top_n_blue_balls = most_possible_occur_blue_balls
            .iter()
            .take(5)
            .cloned()
            .collect::<Vec<Arc<BallOccurInfo>>>();
        top_n_blue_balls.sort_by_key(|v1| v1.ball());
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
            let mut red_balls_set = HashSet::new();
            red_balls.iter().for_each(|ball| {
                red_balls_set.insert(*ball);
            });
            let mut red_balls = red_balls_set.into_iter().collect::<Vec<usize>>();
            red_balls.sort();
            debug!("根据关联关系生成待选红球：{:?}", red_balls);
            let mut chosen_red_balls = Vec::new();
            loop {
                let index = rng().random_range(0..red_balls.len());
                let red_ball = red_balls.remove(index);
                chosen_red_balls.push(red_ball);
                if chosen_red_balls.len() == 6 {
                    break;
                }
            }
            chosen_red_balls.sort();
            info!(
                "根据关联关系选出，蓝球：{}，红球：{:?}",
                blue_ball.ball(),
                chosen_red_balls
            );
        });
        Ok(())
    }
}
