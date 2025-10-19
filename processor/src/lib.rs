use crate::context::{OccurrenceDetail, ProcessorContext, ProcessorContextAttr};
use crate::error::Error;
use derive_more::Display;

use ::fp_growth::algorithm::FPResult;
use ssq_tool_domain::{Ball, BlueBall, RedBall};
use std::{
    borrow::Borrow,
    collections::HashMap,
    sync::{Arc, LazyLock},
};
use tracing::debug;

pub mod ball_occurrence;
pub mod ball_relationship_fp;
pub mod blue_ball_occurrence_fp;
pub mod context;
pub mod error;
pub mod final_result;

pub static BALL_OCCURRENCE: LazyLock<Arc<ProcessorContextAttr<HashMap<Ball, OccurrenceDetail>>>> =
    LazyLock::new(|| Arc::new(ProcessorContextAttr::new("BALL_OCCURRENCE")));
pub static BALL_RELATIONSHIP_FP: LazyLock<
    ProcessorContextAttr<HashMap<BlueBall, FPResult<RedBall>>>,
> = LazyLock::new(|| ProcessorContextAttr::new("BALL_RELATIONSHIP_FP_RESULT"));
pub static BLUE_BALL_OCCURRENCE_FP: LazyLock<ProcessorContextAttr<FPResult<BlueBall>>> =
    LazyLock::new(|| ProcessorContextAttr::new("BLUR_BALL_OCCURRENCE_FP_RESULT"));
pub static FINAL_PROCESSOR_CHAIN_RESULTS: LazyLock<
    ProcessorContextAttr<Vec<FinalProcessorChainResult>>,
> = LazyLock::new(|| ProcessorContextAttr::new("CUSTOMIZE_SUMMARIES"));

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Display)]
#[display("红球：{red_balls:?}; 蓝球：{blue_ball}")]
pub struct FinalProcessorChainResult {
    blue_ball: BlueBall,
    red_balls: [RedBall; 6],
}

impl FinalProcessorChainResult {
    pub fn new(blue_ball: BlueBall, red_balls: [RedBall; 6]) -> Self {
        Self {
            blue_ball,
            red_balls,
        }
    }

    pub fn blue_ball(&self) -> &BlueBall {
        &self.blue_ball
    }

    pub fn red_ball(&self) -> &[RedBall] {
        &self.red_balls
    }
}

#[async_trait::async_trait]
pub trait Processor {
    /// Return the name of the processor
    fn name(&self) -> &str;

    /// Define the execut logic of the processor
    async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error>;
}

pub struct ProcessorChain {
    name: String,
    processors: Vec<Box<dyn Processor + Send>>,
}

impl ProcessorChain {
    pub fn new(name: impl Borrow<str>) -> Self {
        Self {
            name: name.borrow().to_owned(),
            processors: Default::default(),
        }
    }

    /// Add a processor to the chain
    pub fn add_processor(mut self, processor: Box<dyn Processor + Send>) -> Self {
        self.processors.push(processor);
        self
    }

    /// Execute all the processors in the chain
    pub async fn execute<'a>(&mut self, context: &mut ProcessorContext<'a>) -> Result<(), Error> {
        for processor in self.processors.iter_mut() {
            debug!("开始执行: {}", processor.name());
            processor.execute(context).await?;
            debug!("成功执行: {}", processor.name());
        }
        Ok(())
    }
}

impl From<Vec<Box<dyn Processor + Send>>> for ProcessorChain {
    fn from(processors: Vec<Box<dyn Processor + Send>>) -> Self {
        Self {
            processors,
            name: "ProcessorChain".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl Processor for ProcessorChain {
    fn name(&self) -> &str {
        &self.name
    }

    async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error> {
        self.execute(context).await
    }
}
