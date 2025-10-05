use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct BusinessPrizeRecord {
    code: String,
    index: usize,
    red_balls: Vec<usize>,
    blue_ball: usize,
    date: NaiveDate,
}

impl BusinessPrizeRecord {
    pub fn new(
        code: String,
        index: usize,
        red_balls: Vec<usize>,
        blue_ball: usize,
        date: NaiveDate,
    ) -> Self {
        Self {
            code,
            index,
            red_balls,
            blue_ball,
            date,
        }
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn red_balls(&self) -> &[usize] {
        &self.red_balls
    }

    pub fn blue_ball(&self) -> usize {
        self.blue_ball
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }
}
