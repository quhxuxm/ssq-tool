use crate::context_obj::{OccurDetail, Relationship};
use crate::error::Error;
use async_trait;
use derive_more::Display;
use ssq_tool_domain::{Ball, BlueBall, PrBusinessObj, RedBall};
use std::any::type_name;
use std::{
    any::Any,
    borrow::Borrow,
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, LazyLock},
};
use tracing::debug;

pub mod context_obj;
pub mod error;
pub mod occur;
pub mod relationship;
pub mod summary;

pub static BLUE_BALL_RELATIONSHIPS: LazyLock<
    ProcessorContextAttr<HashMap<BlueBall, Relationship>>,
> = LazyLock::new(|| ProcessorContextAttr::new("BLUE_BALL_RELATIONSHIPS"));
pub static RED_BALL_RELATIONSHIPS: LazyLock<ProcessorContextAttr<HashMap<RedBall, Relationship>>> =
    LazyLock::new(|| ProcessorContextAttr::new("RED_BALL_RELATIONSHIPS"));

pub static BALL_OCCURS: LazyLock<Arc<ProcessorContextAttr<HashMap<Ball, OccurDetail>>>> =
    LazyLock::new(|| Arc::new(ProcessorContextAttr::new("BALL_OCCURS")));

pub static SUMMARIES: LazyLock<ProcessorContextAttr<Vec<SummaryRecord>>> =
    LazyLock::new(|| ProcessorContextAttr::new("SUMMARIES"));

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Display)]
#[display("红球：{red_balls:?}; 蓝球：{blue_ball}")]
pub struct SummaryRecord {
    blue_ball: BlueBall,
    red_balls: [RedBall; 6],
}

impl SummaryRecord {
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ProcessorContextAttr<T>
where
    T: Any + Send + 'static,
{
    name: String,
    _val_type: PhantomData<T>,
}

impl<T> ProcessorContextAttr<T>
where
    T: Any + Send + 'static,
{
    pub fn new(name: impl Borrow<str>) -> Self {
        Self {
            name: name.borrow().to_owned(),
            _val_type: PhantomData,
        }
    }
}

/// The context of the processor and processor chain

const PROCESSOR_CONTEXT_ATTR_KEY_PREFIX: &str = "$__PROCESSOR_CTX_ATTR__$";
pub struct ProcessorContext<'a> {
    prize_records: &'a [PrBusinessObj],
    result_size: usize,
    attributes: HashMap<String, Box<dyn Any + Send + 'static>>,
}

impl<'a> ProcessorContext<'a> {
    pub fn new(prize_records: &'a [PrBusinessObj], result_size: usize) -> Self {
        Self {
            attributes: HashMap::new(),
            result_size,
            prize_records,
        }
    }

    pub fn get_attribute<T>(&self, name: &ProcessorContextAttr<T>) -> Option<&T>
    where
        T: Send + 'static,
    {
        let ProcessorContextAttr { name, .. } = name;
        let attr_key = format!(
            "{PROCESSOR_CONTEXT_ATTR_KEY_PREFIX}_{name}_[{}]",
            type_name::<T>()
        );
        match self.attributes.get(&attr_key).as_ref() {
            Some(attr) => attr.downcast_ref::<T>(),
            None => None,
        }
    }

    pub fn set_attribute<T>(
        &mut self,
        attr: &ProcessorContextAttr<T>,
        value: T,
    ) -> Option<Box<dyn Any + Send>>
    where
        T: Send + 'static,
    {
        let ProcessorContextAttr { name, .. } = &attr;
        let attr_key = format!(
            "{PROCESSOR_CONTEXT_ATTR_KEY_PREFIX}_{name}_[{}]",
            type_name::<T>()
        );
        self.attributes.insert(attr_key, Box::new(value))
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
