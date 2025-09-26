use std::error::Error;

struct Pick {
    red_balls: Vec<u8>,
    blue_ball: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rounds = 10000000;
    let mut chosen_rounds = vec![];
    for _ in 1..=5 {
        let round = rand::random_range(0..rounds);
        chosen_rounds.push(round);
    }
    let mut picks = vec![];
    for _ in 1..=rounds {
        let red_balls = generate_red_balls().await;
        let blue_ball = generate_blue_ball().await;
        let pick = Pick {
            red_balls,
            blue_ball,
        };
        picks.push(pick);
    }
    chosen_rounds.into_iter().for_each(|round_index| {
        let Pick {
            red_balls,
            blue_ball,
        } = &picks[round_index];
        println!("##########");
        println!("红球: {red_balls:?}");
        println!("蓝球: {blue_ball}");
        println!("##########");
    });
    tokio::signal::ctrl_c().await?;
    Ok(())
}

async fn generate_red_balls() -> Vec<u8> {
    let all_red_balls = 1..=33;
    let mut all_red_balls = all_red_balls.collect::<Vec<u8>>();
    let mut result = vec![];
    let mut i = 0;
    while i < 6 {
        let random_index = rand::random_range(0..all_red_balls.len());
        let red_ball = all_red_balls.remove(random_index);
        result.push(red_ball);
        i += 1;
    }
    result.sort();
    result
}

async fn generate_blue_ball() -> u8 {
    let blue_ball = rand::random_range(1..=16);
    blue_ball
}
