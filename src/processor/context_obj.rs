use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BallOccurInfo {
    ball: usize,
    intervals: Vec<usize>,
    max_interval: usize,
    min_interval: usize,
    average_interval: f64,
    average_occur_possibility: f64,
    occur_count: usize,
    last_occur_index: usize,
    possible_next_occur_index: f64,
}

impl BallOccurInfo {
    pub fn new(ball: usize) -> Self {
        Self {
            ball,
            intervals: Vec::new(),
            max_interval: 0,
            min_interval: 0,
            average_interval: 0f64,
            average_occur_possibility: 0f64,
            occur_count: 0,
            last_occur_index: 0,
            possible_next_occur_index: 0f64,
        }
    }

    pub fn ball(&self) -> usize {
        self.ball
    }

    pub fn add_interval(&mut self, interval: usize) {
        self.intervals.push(interval);
    }

    pub fn intervals(&self) -> &[usize] {
        &self.intervals
    }

    pub fn set_max_interval(&mut self, max_interval: usize) {
        self.max_interval = max_interval;
    }

    pub fn max_interval(&self) -> usize {
        self.max_interval
    }

    pub fn min_interval(&self) -> usize {
        self.min_interval
    }

    pub fn set_min_interval(&mut self, min_interval: usize) {
        self.min_interval = min_interval;
    }

    pub fn average_interval(&self) -> f64 {
        self.average_interval
    }

    pub fn set_average_interval(&mut self, average_interval: f64) {
        self.average_interval = average_interval;
    }

    pub fn average_occur_possibility(&self) -> f64 {
        self.average_occur_possibility
    }

    pub fn set_average_occur_possibility(&mut self, occur_possibility: f64) {
        self.average_occur_possibility = occur_possibility
    }

    pub fn occur_count(&self) -> usize {
        self.occur_count
    }

    pub fn set_occur_count(&mut self, occur_count: usize) {
        self.occur_count = occur_count
    }

    pub fn set_last_occur_index(&mut self, last_occur_index: usize) {
        self.last_occur_index = last_occur_index
    }

    pub fn last_occur_index(&self) -> usize {
        self.last_occur_index
    }

    pub fn set_possible_next_occur_index(&mut self, possible_next_occur_index: f64) {
        self.possible_next_occur_index = possible_next_occur_index
    }

    pub fn possible_next_occur_index(&self) -> f64 {
        self.possible_next_occur_index
    }
}

/// The blue ball related red balls.
#[derive(Debug, Clone)]
pub struct BlueBallRelationship {
    blue_ball: usize,
    related_red_ball_counts: HashMap<usize, usize>,
}

impl BlueBallRelationship {
    pub fn new(blue_ball: usize) -> Self {
        let mut related_red_ball_counts = HashMap::new();
        (1..=33).for_each(|red_ball| {
            related_red_ball_counts.insert(red_ball, 0);
        });
        Self {
            blue_ball,
            related_red_ball_counts,
        }
    }

    pub fn blue_ball(&self) -> usize {
        self.blue_ball
    }

    pub fn related_red_ball_counts(&self) -> &HashMap<usize, usize> {
        &self.related_red_ball_counts
    }

    pub fn increase_related_red_ball_count(&mut self, red_ball: usize) {
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
    red_ball: usize,
    related_red_ball_counts: HashMap<usize, usize>,
    related_blue_ball_counts: HashMap<usize, usize>,
}

impl RedBallRelationship {
    pub fn new(red_ball: usize) -> Self {
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

    pub fn red_ball(&self) -> usize {
        self.red_ball
    }

    pub fn related_red_ball_counts(&self) -> &HashMap<usize, usize> {
        &self.related_red_ball_counts
    }

    pub fn increase_related_red_ball_count(&mut self, red_ball: usize) {
        if self.red_ball == red_ball {
            return;
        }
        self.related_red_ball_counts
            .entry(red_ball)
            .and_modify(|count| {
                *count += 1;
            });
    }

    pub fn related_blue_ball_counts(&self) -> &HashMap<usize, usize> {
        &self.related_blue_ball_counts
    }

    pub fn increase_related_blue_ball_count(&mut self, blue_ball: usize) {
        self.related_blue_ball_counts
            .entry(blue_ball)
            .and_modify(|count| {
                *count += 1;
            });
    }
}
