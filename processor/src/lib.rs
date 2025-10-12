use crate::context_obj::{OccurDetail, Relationship};
use crate::error::Error;
use async_trait;
use ssq_tool_domain::{Ball, BlueBall, PrBusinessObj, RedBall};
use std::any::type_name;
use std::{
    any::Any,
    borrow::Borrow,
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, LazyLock},
};

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
pub struct ProcessorContext {
    prize_records: Arc<Vec<PrBusinessObj>>,
    result_size: usize,
    attributes: HashMap<String, Box<dyn Any + Send + 'static>>,
}

impl ProcessorContext {
    pub fn new(prize_records: Arc<Vec<PrBusinessObj>>, result_size: usize) -> Self {
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
    pub async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error> {
        for processor in self.processors.iter_mut() {
            println!("executing processor: {}", processor.name());
            processor.execute(context).await?;
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
