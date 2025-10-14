use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp_actix_web::transport::StreamableHttpService;
use crate::error::Error;
use ssq_tool_collector::Collector;
use ssq_tool_processor::occur::OccurProcessor;
use ssq_tool_processor::relationship::RelationshipProcessor;
use ssq_tool_processor::summary::SummaryProcessor;
use ssq_tool_processor::{Processor, ProcessorChain, ProcessorContext, SUMMARIES};
use tracing::{info, level_filters::LevelFilter};
use ssq_tool_domain::PrBusinessObj;
use crate::service::occur_service::OccurMcpService;

pub mod error;
mod service;

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
        .with_max_level(LevelFilter::DEBUG)
        .init();
    info!("开始收集往期双色球数据...");
    let collector = Collector::Remote;
    let prize_record_business_objs = collector.collect(None).await?;
    let mut processor_chain = generate_processor_chain();
    info!("双色球分析链构建完成...");
    let mut context = ProcessorContext::new(Arc::new(prize_record_business_objs), 5);
    info!("开始分析双色球数据...");
    processor_chain.execute(&mut context).await?;
    let summarise = context
        .get_attribute(&SUMMARIES)
        .ok_or(Error::NoSummarise)?;
    summarise.iter().for_each(|record| {
        println!("{record}");
    });
    Ok(())
}

async fn create_mcp_server(prize_record_business_objs:Vec<PrBusinessObj>)->Result<(), Error>{
    let http_service = StreamableHttpService::builder()
        .service_factory(Arc::new(|| Ok(OccurMcpService::new())))
        .session_manager(Arc::new(LocalSessionManager::default()))
        .stateful_mode(true)
        .build();
    HttpServer::new(move || {
        App::new()
            // Mount MCP service at custom path
            .service(web::scope("/api/v1/mcp").service(http_service.clone().scope()))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}
