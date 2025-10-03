use tracing::info;

use crate::{
    error::Error,
    processor::{
        BLUE_BALL_RELATIONSHIPS, Context, Processor, RED_BALL_RELATIONSHIPS,
        SORTED_BLUE_BALLS_COUNTS, SORTED_RED_BALLS_COUNTS,
    },
};

pub struct SummaryTopRedBallRelationshipProcessor;

#[async_trait::async_trait]
impl Processor for SummaryTopRedBallRelationshipProcessor {
    fn name(&self) -> &str {
        "SummaryTopRedBallRelationshipProcessor"
    }

    async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
        let sorted_blue_balls_counts =
            context
                .attribute(&SORTED_BLUE_BALLS_COUNTS)
                .ok_or(Error::ContextAttrNotExist(
                    SORTED_BLUE_BALLS_COUNTS.name.clone(),
                ))?;
        let sorted_red_balls_counts =
            context
                .attribute(&SORTED_RED_BALLS_COUNTS)
                .ok_or(Error::ContextAttrNotExist(
                    SORTED_RED_BALLS_COUNTS.name.clone(),
                ))?;
        let blue_ball_relationships =
            context
                .attribute(&BLUE_BALL_RELATIONSHIPS)
                .ok_or(Error::ContextAttrNotExist(
                    BLUE_BALL_RELATIONSHIPS.name.clone(),
                ))?;
        let red_ball_relationships =
            context
                .attribute(&RED_BALL_RELATIONSHIPS)
                .ok_or(Error::ContextAttrNotExist(
                    RED_BALL_RELATIONSHIPS.name.clone(),
                ))?;
        let top_blue_ball = sorted_blue_balls_counts
            .first()
            .ok_or(Error::Other("can not find top blue ball.".to_string()))?;
        let bottom_blue_ball = sorted_blue_balls_counts
            .last()
            .ok_or(Error::Other("can not find last blue ball.".to_string()))?;
        let mut top_red_balls = sorted_red_balls_counts[0..6].to_vec();
        info!("最热的6个红球和相应的出现次数是：{top_red_balls:?}");
        top_red_balls.sort_by(|v1, v2| v1.1.cmp(&v2.1));
        let top_red_balls = top_red_balls.iter().map(|v| v.0).collect::<Vec<u8>>();
        let mut bottom_red_balls = sorted_red_balls_counts[27..33].to_vec();
        info!("最冷的6个红球和相应的出现次数是：{bottom_red_balls:?}");
        bottom_red_balls.sort_by(|v1, v2| v1.1.cmp(&v2.1));
        let bottom_red_balls = bottom_red_balls.iter().map(|v| v.0).collect::<Vec<u8>>();
        let top_blue_ball_relationship =
            blue_ball_relationships
                .get(&top_blue_ball.0)
                .ok_or(Error::Other(format!(
                    "top blue ball {} relationship not exist",
                    top_blue_ball.0
                )))?;
        info!(
            "按照最热红球相关性估测1：BLUE: {}; RED: {:?}",
            top_blue_ball_relationship.blue_ball(),
            top_red_balls
        );
        info!(
            "按照最热红球相关性估测2：BLUE: {}; RED: {:?}",
            top_blue_ball_relationship.blue_ball(),
            bottom_red_balls
        );
        let bottom_blue_ball_relationship = blue_ball_relationships
            .get(&bottom_blue_ball.0)
            .ok_or(Error::Other(format!(
                "bottom blue ball {} relationship not exist",
                bottom_blue_ball.0
            )))?;

        info!(
            "按照最冷红球相关性估测1：BLUE: {}; RED: {:?}",
            bottom_blue_ball_relationship.blue_ball(),
            top_red_balls
        );
        info!(
            "按照最冷红球相关性估测2：BLUE: {}; RED: {:?}",
            bottom_blue_ball_relationship.blue_ball(),
            bottom_red_balls
        );
        Ok(())
    }
}
