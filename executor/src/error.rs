use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Processor(#[from] ssq_tool_processor::error::Error),
    #[error(transparent)]
    Collector(#[from] ssq_tool_collector::error::Error),
    #[error("没有分析出最终结果")]
    NoFinalProcessorChainResults,
    #[error(transparent)]
    IoFailure(#[from] std::io::Error),
    #[error("其他错误：{0}")]
    Other(String),
}
