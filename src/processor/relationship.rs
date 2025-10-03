use std::collections::HashMap;

use tracing::info;

use crate::{
    error::Error,
    processor::{
        BLUE_BALL_RELATIONSHIPS, Context, Processor, RED_BALL_RELATIONSHIPS,
        business_obj::{BlueBallRelationship, RedBallRelationship},
    },
};

pub struct RelationshipProcessor;

#[async_trait::async_trait]
impl Processor for RelationshipProcessor {
    fn name(&self) -> &str {
        "RelationshipProcessor"
    }

    async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
        let mut blue_ball_relationships = HashMap::new();
        (1..=16).for_each(|blue_ball| {
            let blue_ball = blue_ball as u8;
            blue_ball_relationships.insert(blue_ball, BlueBallRelationship::new(blue_ball));
        });
        let mut red_ball_relationships = HashMap::new();
        (1..=33).for_each(|red_ball| {
            let red_ball = red_ball as u8;
            red_ball_relationships.insert(red_ball, RedBallRelationship::new(red_ball));
        });
        context
            .prize_record_page
            .prize_records
            .iter()
            .for_each(|prized_ball| {
                let blue_ball = prized_ball.blue;
                let red_balls = &prized_ball.red;
                blue_ball_relationships
                    .entry(blue_ball)
                    .and_modify(|relationship| {
                        (1..=33).for_each(|target_red_ball| {
                            let target_red_ball = target_red_ball as u8;
                            if red_balls.contains(&target_red_ball) {
                                relationship.increase_related_red_ball_count(target_red_ball);
                            }
                        });
                    });
                red_balls.iter().for_each(|red_ball| {
                    red_ball_relationships
                        .entry(*red_ball)
                        .and_modify(|relationship| {
                            relationship.increase_related_blue_ball_count(blue_ball);
                        });
                    (1..=33).for_each(|target_red_ball| {
                        red_ball_relationships
                            .entry(target_red_ball)
                            .and_modify(|relationship| {
                                relationship.increase_related_red_ball_count(*red_ball);
                            });
                    });
                });
            });
        info!("蓝球相关性：\n{blue_ball_relationships:#?}");
        context.add_attribute(BLUE_BALL_RELATIONSHIPS.clone(), blue_ball_relationships);
        info!("红球相关性：\n{red_ball_relationships:#?}");
        context.add_attribute(RED_BALL_RELATIONSHIPS.clone(), red_ball_relationships);
        Ok(())
    }
}
