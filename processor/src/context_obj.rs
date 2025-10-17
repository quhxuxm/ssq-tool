use ssq_tool_domain::{BlueBall, RedBall};
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Default)]
pub struct OccurDetail {
    average_interval: usize,
    occurrence_count: usize,
    latest_occur_seq: usize,
    count_based_on_average_interval: usize,
}

impl OccurDetail {
    pub fn average_interval(&self) -> usize {
        self.average_interval
    }

    pub fn set_average_interval(&mut self, average_interval: usize) {
        self.average_interval = average_interval;
    }

    pub fn occurrence_count(&self) -> usize {
        self.occurrence_count
    }

    pub fn set_occurrence_count(&mut self, occurrence_count: usize) {
        self.occurrence_count = occurrence_count
    }

    pub fn set_latest_occur_seq(&mut self, latest_occur_seq: usize) {
        self.latest_occur_seq = latest_occur_seq
    }

    pub fn latest_occur_seq(&self) -> usize {
        self.latest_occur_seq
    }

    pub fn count_based_on_average_interval(&self) -> usize {
        self.count_based_on_average_interval
    }

    pub fn set_count_based_on_average_interval(&mut self, count_based_on_average_interval: usize) {
        self.count_based_on_average_interval = count_based_on_average_interval;
    }
}

#[derive(Debug)]
pub enum Relationship {
    Blue {
        ball: BlueBall,
        detail: HashMap<RedBall, usize>,
    },
    Red {
        ball: RedBall,
        blue_ball_detail: HashMap<BlueBall, usize>,
        red_ball_detail: HashMap<RedBall, usize>,
    },
}

impl Relationship {
    pub fn new_blue(ball: BlueBall) -> Self {
        let mut detail = HashMap::new();
        RedBall::iter().for_each(|red_ball| {
            detail.insert(red_ball, 0);
        });
        Self::Blue { ball, detail }
    }

    pub fn new_red(ball: RedBall) -> Self {
        let mut red_ball_detail = HashMap::new();
        RedBall::iter().for_each(|red_ball| {
            red_ball_detail.insert(red_ball, 0);
        });
        let mut blue_ball_detail = HashMap::new();
        BlueBall::iter().for_each(|blue_ball| {
            blue_ball_detail.insert(blue_ball, 0);
        });
        Self::Red {
            ball,
            red_ball_detail,
            blue_ball_detail,
        }
    }

    pub fn increase_relationship_with_blue(&mut self, target_ball: BlueBall) {
        if let Relationship::Red {
            blue_ball_detail, ..
        } = self
        {
            blue_ball_detail.entry(target_ball).and_modify(|count| {
                *count += 1;
            });
        }
    }

    pub fn increase_relationship_with_red(&mut self, target_ball: RedBall) {
        match self {
            Relationship::Blue { detail, .. } => {
                detail.entry(target_ball).and_modify(|count| {
                    *count += 1;
                });
            }
            Relationship::Red {
                red_ball_detail, ..
            } => {
                red_ball_detail.entry(target_ball).and_modify(|count| {
                    *count += 1;
                });
            }
        }
    }
}
