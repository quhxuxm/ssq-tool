#[tokio::main]
async fn main() {
    let rounds = 5;
    for i in 1..=rounds {
        println!("### running in the round {i}.");
        let red_balls = generate_red_balls().await;
        let blue_ball = generate_blue_ball().await;
        println!("red balls: {red_balls:?}");
        println!("blue ball: {blue_ball}");
        println!("### complete round {i}")
    }
}

async fn generate_red_balls() -> Vec<u8> {
    let mut all_red_balls = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33,
    ];
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
    let blue_ball = rand::random_range(0..16) + 1;
    blue_ball
}
