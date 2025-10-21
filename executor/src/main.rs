use crate::error::Error;
use crate::service::ssq_mcp_service::SsqMcpService;
use actix_web::{web, App, HttpServer};
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp_actix_web::transport::StreamableHttpService;
use ssq_tool_collector::Collector;
use ssq_tool_domain::PrBusinessObj;
use ssq_tool_processor::ball_occurrence::BallOccurrenceProcessor;
use ssq_tool_processor::ball_relationship_fp::BallRelationshipFpProcessor;
use ssq_tool_processor::blue_ball_occurrence_fp::BlueBallOccurrenceFpProcessor;
use ssq_tool_processor::final_result::FinalResultsProcessor;
use ssq_tool_processor::generate_normalize_data::GenerateNormalizeDataProcessor;
use ssq_tool_processor::{context::ProcessorContext, Processor, ProcessorChain};
use std::sync::{Arc, OnceLock};
use tracing::{error, info, level_filters::LevelFilter};

pub mod error;
mod service;

static OFFICIAL_PRIZE_RECORD_BUSINESS_OBJ: OnceLock<Vec<PrBusinessObj>> = OnceLock::new();

fn generate_processor_chain() -> ProcessorChain {
    let processors: Vec<Box<dyn Processor + Send>> = vec![
        Box::new(GenerateNormalizeDataProcessor::new("./generate.txt".into())),
        Box::new(BallOccurrenceProcessor),
        Box::new(BallRelationshipFpProcessor::new(10)),
        Box::new(BlueBallOccurrenceFpProcessor::new(480, 10)),
        Box::new(FinalResultsProcessor::new(10, 5)),
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
    OFFICIAL_PRIZE_RECORD_BUSINESS_OBJ
        .set(prize_record_business_objs)
        .map_err(|_| {
            error!("把往期双色球数据放进全局变量失败.");
            Error::Other("把往期双色球数据放进全局变量失败.".to_string())
        })?;
    command_line().await?;
    // create_mcp_server().await?;
    Ok(())
}

async fn command_line() -> Result<(), Error> {
    let mut processor_chain = generate_processor_chain();
    info!("双色球分析链构建完成...");
    let pr_bus_objs = OFFICIAL_PRIZE_RECORD_BUSINESS_OBJ
        .get()
        .ok_or(Error::Other(
            "无法从全局变量中取得往期双色球数据，执行命令行失败".to_string(),
        ))?
        .clone();
    let mut context = ProcessorContext::new(&pr_bus_objs, 5);
    info!("开始分析双色球数据...");
    processor_chain.execute(&mut context).await?;
    // let final_processor_chain_results = context
    //     .get_attribute(&FINAL_PROCESSOR_CHAIN_RESULTS)
    //     .ok_or(Error::NoFinalProcessorChainResults)?;
    // final_processor_chain_results.iter().for_each(|record| {
    //     println!("{record}");
    // });
    Ok(())
}

#[allow(unused)]
async fn create_mcp_server() -> Result<(), Error> {
    let service = Arc::new(|| {
        let ssq_mcp_service = SsqMcpService::new(OFFICIAL_PRIZE_RECORD_BUSINESS_OBJ.get().ok_or(
            std::io::Error::other(
                "无法从全局变量中取得往期双色球数据，创建 MCP 服务失败.".to_string(),
            ),
        )?);
        Ok(ssq_mcp_service)
    });
    let ssq_mcp_service = StreamableHttpService::builder()
        .service_factory(service.clone())
        .session_manager(Arc::new(LocalSessionManager::default()))
        .stateful_mode(true)
        .build();
    HttpServer::new(move || {
        App::new()
            // Mount MCP service at custom path
            .service(web::scope("/ssq/mcp").service(ssq_mcp_service.clone().scope()))
    })
    .bind("127.0.0.1:20080")?
    .run()
    .await?;
    Ok(())
}
