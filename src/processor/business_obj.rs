use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BallOccurInterval {
    ball: u8,
    intervals: Vec<u32>,
}

impl BallOccurInterval {
    pub fn new(ball: u8) -> Self {
        Self {
            ball,
            intervals: Vec::new(),
        }
    }

    pub fn ball(&self) -> u8 {
        self.ball
    }

    pub fn add_interval(&mut self, interval: u32) {
        self.intervals.push(interval);
    }
}

/// The blue ball related red balls.
#[derive(Debug, Clone)]
pub struct BlueBallRelationship {
    blue_ball: u8,
    related_red_ball_counts: HashMap<u8, usize>,
}

impl BlueBallRelationship {
    pub fn new(blue_ball: u8) -> Self {
        let mut related_red_ball_counts = HashMap::new();
        (1..=33).for_each(|red_ball| {
            related_red_ball_counts.insert(red_ball, 0);
        });
        Self {
            blue_ball,
            related_red_ball_counts,
        }
    }

    pub fn blue_ball(&self) -> u8 {
        self.blue_ball
    }

    pub fn related_red_ball_counts(&self) -> &HashMap<u8, usize> {
        &self.related_red_ball_counts
    }

    pub fn increase_related_red_ball_count(&mut self, red_ball: u8) {
        self.related_red_ball_counts
            .entry(red_ball)
            .and_modify(|count| {
                *count += 1;
            });
    }
}

/// The red ball related red balls and blue balls.
#[derive(Debug, Clone)]
pub struct RedBallRelationship {
    red_ball: u8,
    related_red_ball_counts: HashMap<u8, usize>,
    related_blue_ball_counts: HashMap<u8, usize>,
}

impl RedBallRelationship {
    pub fn new(red_ball: u8) -> Self {
        let mut related_red_ball_counts = HashMap::new();
        let mut related_blue_ball_counts = HashMap::new();
        (1..=33).for_each(|target_red_ball| {
            if red_ball == target_red_ball {
                return;
            }
            related_red_ball_counts.insert(target_red_ball, 0);
        });
        (1..=16).for_each(|target_blue_ball| {
            related_blue_ball_counts.insert(target_blue_ball, 0);
        });
        Self {
            red_ball,
            related_red_ball_counts,
            related_blue_ball_counts,
        }
    }

    pub fn red_ball(&self) -> u8 {
        self.red_ball
    }

    pub fn related_red_ball_counts(&self) -> &HashMap<u8, usize> {
        &self.related_red_ball_counts
    }

    pub fn increase_related_red_ball_count(&mut self, red_ball: u8) {
        if self.red_ball == red_ball {
            return;
        }
        self.related_red_ball_counts
            .entry(red_ball)
            .and_modify(|count| {
                *count += 1;
            });
    }

    pub fn related_blue_ball_counts(&self) -> &HashMap<u8, usize> {
        &self.related_blue_ball_counts
    }

    pub fn increase_related_blue_ball_count(&mut self, blue_ball: u8) {
        self.related_blue_ball_counts
            .entry(blue_ball)
            .and_modify(|count| {
                *count += 1;
            });
    }
}
