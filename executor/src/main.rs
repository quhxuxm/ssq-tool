use std::sync::Arc;

use crate::error::Error;
use ssq_tool_collector::Collector;
use ssq_tool_processor::occur::OccurProcessor;
use ssq_tool_processor::relationship::RelationshipProcessor;
use ssq_tool_processor::summary::SummaryProcessor;
use ssq_tool_processor::{Processor, ProcessorChain, ProcessorContext};
use tracing::{info, level_filters::LevelFilter};

pub mod error;

fn generate_processor_chain() -> ProcessorChain {
    let processors: Vec<Box<dyn Processor + Send>> = vec![
        Box::new(RelationshipProcessor),
        Box::new(OccurProcessor),
        Box::new(SummaryProcessor),
    ];
    ProcessorChain::from(processors)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let tracing_file_appender = tracing_appender::rolling::daily("./log", "ssq.log");
    let (tracing_file_writer, _tracing_file_writer_guard) =
        tracing_appender::non_blocking(tracing_file_appender);
    tracing_subscriber::fmt()
        .with_writer(tracing_file_writer)
        .with_ansi(false)
        // .with_line_number(true)
        // .with_file(true)
        .with_level(true)
        // .with_thread_names(true)
        // .with_thread_ids(true)
        .with_max_level(LevelFilter::INFO)
        .init();
    info!("开始收集往期双色球数据...");
    let collector = Collector::Local;
    let prize_record_business_objs = collector.collect(None).await?;
    let mut processor_chain = generate_processor_chain();
    info!("双色球分析链构建完成...");
    let mut context = ProcessorContext::new(Arc::new(prize_record_business_objs), 5);
    info!("开始分析双色球数据...");
    processor_chain.execute(&mut context).await?;
    Ok(())
}
