use crate::context::Relationship;
use crate::{
    BLUE_BALL_RELATIONSHIPS, Processor, ProcessorContext, RED_BALL_RELATIONSHIPS, error::Error,
};
use ssq_tool_domain::{BlueBall, RedBall};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use tracing::trace;

pub struct RelationshipProcessor;

#[async_trait::async_trait]
impl Processor for RelationshipProcessor {
    fn name(&self) -> &str {
        "RelationshipProcessor"
    }

    async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error> {
        let mut blue_ball_relationships = HashMap::new();
        BlueBall::iter().for_each(|blue_ball| {
            blue_ball_relationships.insert(blue_ball, Relationship::new_blue(blue_ball));
        });
        let mut red_ball_relationships = HashMap::new();
        RedBall::iter().for_each(|red_ball| {
            red_ball_relationships.insert(red_ball, Relationship::new_red(red_ball));
        });
        context.get_prize_records().iter().for_each(|prized_ball| {
            let prize_blue_ball = prized_ball.blue_ball;
            let prize_red_balls = prized_ball.red_balls;
            blue_ball_relationships
                .entry(prize_blue_ball)
                .and_modify(|relationship| {
                    RedBall::iter().for_each(|target_red_ball| {
                        if prize_red_balls.contains(&target_red_ball) {
                            relationship.increase_relationship_with_red(target_red_ball);
                        }
                    });
                });
            prize_red_balls.iter().for_each(|prize_red_ball| {
                red_ball_relationships
                    .entry(*prize_red_ball)
                    .and_modify(|relationship| {
                        relationship.increase_relationship_with_blue(prize_blue_ball);
                    });
                RedBall::iter().for_each(|target_red_ball| {
                    red_ball_relationships
                        .entry(target_red_ball)
                        .and_modify(|relationship| {
                            relationship.increase_relationship_with_red(*prize_red_ball);
                        });
                });
            });
        });
        trace!("蓝球相关性：{blue_ball_relationships:?}");
        context.set_attribute(&BLUE_BALL_RELATIONSHIPS, blue_ball_relationships);
        trace!("红球相关性：{red_ball_relationships:?}");
        context.set_attribute(&RED_BALL_RELATIONSHIPS, red_ball_relationships);
        Ok(())
    }
}
