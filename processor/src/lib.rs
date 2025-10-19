use crate::context::{OccurrenceDetail, ProcessorContext, ProcessorContextAttr, Relationship};
use crate::error::Error;
use derive_more::Display;
use ssq_tool_domain::{Ball, BlueBall, RedBall};
use std::{
    borrow::Borrow,
    collections::HashMap,
    sync::{Arc, LazyLock},
};
use tracing::debug;

pub mod context;
pub mod error;
pub mod prepare;

pub mod summary;

pub static BLUE_BALL_RELATIONSHIPS: LazyLock<
    ProcessorContextAttr<HashMap<BlueBall, Relationship>>,
> = LazyLock::new(|| ProcessorContextAttr::new("BLUE_BALL_RELATIONSHIPS"));
pub static RED_BALL_RELATIONSHIPS: LazyLock<ProcessorContextAttr<HashMap<RedBall, Relationship>>> =
    LazyLock::new(|| ProcessorContextAttr::new("RED_BALL_RELATIONSHIPS"));

pub static BALL_OCCURS: LazyLock<Arc<ProcessorContextAttr<HashMap<Ball, OccurrenceDetail>>>> =
    LazyLock::new(|| Arc::new(ProcessorContextAttr::new("BALL_OCCURS")));

pub static SUMMARIES: LazyLock<ProcessorContextAttr<Vec<SummaryResult>>> =
    LazyLock::new(|| ProcessorContextAttr::new("SUMMARIES"));

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Display)]
#[display("红球：{red_balls:?}; 蓝球：{blue_ball}")]
pub struct SummaryResult {
    blue_ball: BlueBall,
    red_balls: [RedBall; 6],
}

impl SummaryResult {
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
