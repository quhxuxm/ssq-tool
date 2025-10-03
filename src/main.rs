use std::sync::Arc;

use tracing::info;

use crate::{
    error::Error,
    processor::{
        Processor, ProcessorChain,
        count_prized_blueball::CountPrizedBlueballProcessor,
        count_prized_redball::CountPrizedRedballProcessor,
        occur_interval::BallOccurIntervalProcessor,
        relationship::RelationshipProcessor,
        summary::{
            top_blue_ball_relationship::SummaryTopBlueBallRelationshipProcessor,
            top_counts::SummaryTopCountsProcessor,
            top_red_ball_relationship::SummaryTopRedBallRelationshipProcessor,
        },
    },
};

pub mod domain;
pub mod error;
pub mod official;
pub mod processor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let tracing_file_appender = tracing_appender::rolling::daily("./log", "ssq.log");
    let (tracing_file_writer, _tracing_file_writer_guard) =
        tracing_appender::non_blocking(tracing_file_appender);
    tracing_subscriber::fmt()
        .with_writer(tracing_file_writer)
        .with_ansi(false)
        .with_line_number(true)
        .with_file(true)
        .with_level(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .init();
    info!("开始收集往期双色球数据...");
    let prize_record_page = official::generate_official_data().await?;
    info!("往期双色球数据收集完成...");
    let processors: Vec<Box<dyn Processor>> = vec![
        Box::new(CountPrizedBlueballProcessor),
        Box::new(CountPrizedRedballProcessor),
        Box::new(RelationshipProcessor),
        Box::new(SummaryTopCountsProcessor),
        Box::new(SummaryTopBlueBallRelationshipProcessor),
        Box::new(SummaryTopRedBallRelationshipProcessor),
        Box::new(BallOccurIntervalProcessor),
    ];
    let mut processor_chain = ProcessorChain::from(processors);
    info!("双色球分析链构建完成...");
    let mut context = processor::Context::new(Arc::new(prize_record_page));
    info!("开始分析双色球数据...");
    processor_chain.execute(&mut context).await?;
    Ok(())
}
