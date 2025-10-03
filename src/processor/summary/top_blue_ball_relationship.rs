use tracing::info;

use crate::{
    error::Error,
    processor::{BLUE_BALL_RELATIONSHIPS, Context, Processor, SORTED_BLUE_BALLS_COUNTS},
};

pub struct SummaryTopBlueBallRelationshipProcessor;

#[async_trait::async_trait]
impl Processor for SummaryTopBlueBallRelationshipProcessor {
    fn name(&self) -> &str {
        "SummaryTopBlueBallRelationshipProcessor"
    }

    async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
        let sorted_blue_balls_counts =
            context
                .attribute(&SORTED_BLUE_BALLS_COUNTS)
                .ok_or(Error::ContextAttrNotExist(
                    SORTED_BLUE_BALLS_COUNTS.name.clone(),
                ))?;
        let blue_ball_relationships =
            context
                .attribute(&BLUE_BALL_RELATIONSHIPS)
                .ok_or(Error::ContextAttrNotExist(
                    BLUE_BALL_RELATIONSHIPS.name.clone(),
                ))?;
        let top_blue_ball = sorted_blue_balls_counts
            .first()
            .ok_or(Error::Other("can not find top blue ball.".to_string()))?;
        info!(
            "最热的蓝球是：{}, 中奖次数：{}",
            top_blue_ball.0, top_blue_ball.1
        );
        let bottom_blue_ball = sorted_blue_balls_counts
            .last()
            .ok_or(Error::Other("can not find last blue ball.".to_string()))?;
        info!(
            "最冷的蓝球是：{}，中奖次数：{}",
            bottom_blue_ball.0, bottom_blue_ball.1
        );
        let top_blue_ball_relationship =
            blue_ball_relationships
                .get(&top_blue_ball.0)
                .ok_or(Error::Other(format!(
                    "top blue ball {} relationship not exist",
                    top_blue_ball.0
                )))?;
        let mut top_blue_ball_related_red_balls_counts = top_blue_ball_relationship
            .related_red_ball_counts()
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<(u8, usize)>>();
        top_blue_ball_related_red_balls_counts.sort_by(|v1, v2| v2.1.cmp(&v1.1));
        let top_blue_ball_chosen_red_balls = top_blue_ball_related_red_balls_counts
            .iter()
            .map(|v| v.0)
            .collect::<Vec<u8>>();
        let mut top_blue_ball_chosen_red_balls = top_blue_ball_chosen_red_balls[0..6].to_vec();
        top_blue_ball_chosen_red_balls.sort();
        info!(
            "按照最热蓝球相关性估测：BLUE: {}; RED: {:?}",
            top_blue_ball_relationship.blue_ball(),
            top_blue_ball_chosen_red_balls
        );
        let bottom_blue_ball_relationship = blue_ball_relationships
            .get(&bottom_blue_ball.0)
            .ok_or(Error::Other(format!(
                "bottom blue ball {} relationship not exist",
                bottom_blue_ball.0
            )))?;
        let mut bottom_blue_ball_related_red_balls_counts = bottom_blue_ball_relationship
            .related_red_ball_counts()
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<(u8, usize)>>();
        bottom_blue_ball_related_red_balls_counts.sort_by(|v1, v2| v2.1.cmp(&v1.1));
        let bottom_blue_ball_chosen_red_balls = bottom_blue_ball_related_red_balls_counts
            .iter()
            .map(|v| v.0)
            .collect::<Vec<u8>>();

        let mut bottom_blue_ball_chosen_red_balls =
            bottom_blue_ball_chosen_red_balls[0..6].to_vec();
        bottom_blue_ball_chosen_red_balls.sort();
        info!(
            "按照最冷蓝球具相关性估测：BLUE: {}; RED: {:?}",
            bottom_blue_ball_relationship.blue_ball(),
            bottom_blue_ball_chosen_red_balls
        );
        info!(
            "按照最热蓝球相关性混搭：BLUE：{}; RED: {:?}",
            top_blue_ball_relationship.blue_ball(),
            bottom_blue_ball_chosen_red_balls
        );
        info!(
            "按照最冷蓝球相关性混搭：BLUE：{}; RED: {:?}",
            bottom_blue_ball_relationship.blue_ball(),
            top_blue_ball_chosen_red_balls
        );
        Ok(())
    }
}
