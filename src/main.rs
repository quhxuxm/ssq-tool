use std::sync::Arc;

use tracing::{info, level_filters::LevelFilter};

use crate::{
    error::Error,
    processor::{
        Processor, ProcessorChain, occur::BallOccurProcessor, relationship::RelationshipProcessor,
        summary::create_summary_processor_chain,
    },
};

pub mod collector;
pub mod domain;
pub mod error;
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
        .with_max_level(LevelFilter::INFO)
        .init();
    info!("开始收集往期双色球数据...");
    let prize_record_page = collector::collect_business_data(None).await?;
    info!("往期双色球数据收集完成...");
    let processors: Vec<Box<dyn Processor + Send>> = vec![
        Box::new(RelationshipProcessor),
        Box::new(BallOccurProcessor),
        Box::new(create_summary_processor_chain()),
    ];
    let mut processor_chain = ProcessorChain::from(processors);
    info!("双色球分析链构建完成...");
    let mut context = processor::Context::new(Arc::new(prize_record_page));
    info!("开始分析双色球数据...");
    processor_chain.execute(&mut context).await?;
    Ok(())
}
