use crate::error::Error;

pub mod domain;
pub mod error;
pub mod official;
pub mod processor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let prize_record_page = official::generate_official_data().await?;
    let prize_records = prize_record_page.prize_records;
    for record in prize_records {
        let red_balls = record.red;
        let blue_ball = record.blue;
        let code = record.code;
        let date = record.date;
        let prize = record.prize_grades;
        println!(
            "CODE: {code}; DATE: {date}; RED: {red_balls:?}; BLUE: {blue_ball}; PRIZE: {prize:?}"
        );
    }
    Ok(())
}
