use crate::{
    domain::BusinessPrizeRecord,
    error::Error,
    processor::context_obj::{BallOccurInfo, BlueBallRelationship, RedBallRelationship},
};
use async_trait;
use std::{
    any::Any,
    borrow::Borrow,
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, LazyLock},
};
use tokio::pin;

pub mod context_obj;
pub mod occur;
pub mod relationship;
pub mod summary;

pub static BLUE_BALL_RELATIONSHIPS: LazyLock<ContextAttr<HashMap<usize, BlueBallRelationship>>> =
    LazyLock::new(|| ContextAttr::new("BLUE_BALL_RELATIONSHIPS"));
pub static RED_BALL_RELATIONSHIPS: LazyLock<ContextAttr<HashMap<usize, RedBallRelationship>>> =
    LazyLock::new(|| ContextAttr::new("RED_BALL_RELATIONSHIPS"));

pub static PRIZED_BLUE_BALLS_OCCUR_INFO: LazyLock<ContextAttr<HashMap<usize, Arc<BallOccurInfo>>>> =
    LazyLock::new(|| ContextAttr::new("PRIZED_BLUE_BALLS_OCCUR_INFO"));
pub static PRIZED_RED_BALLS_OCCUR_INFO: LazyLock<ContextAttr<HashMap<usize, Arc<BallOccurInfo>>>> =
    LazyLock::new(|| ContextAttr::new("PRIZED_RED_BALLS_OCCUR_INFO"));

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ContextAttr<T>
where
    T: Any + Send + 'static,
{
    name: String,
    _val_type: PhantomData<T>,
}

impl<T> ContextAttr<T>
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
pub struct Context {
    prize_records: Arc<Vec<BusinessPrizeRecord>>,
    attributes: HashMap<String, Box<dyn Any + Send + 'static>>,
}

impl Context {
    pub fn new(prize_records: Arc<Vec<BusinessPrizeRecord>>) -> Self {
        Self {
            attributes: HashMap::new(),
            prize_records,
        }
    }

    pub fn attribute<T>(&self, name: &ContextAttr<T>) -> Option<&T>
    where
        T: Send + 'static,
    {
        let ContextAttr { name, .. } = name;
        match self.attributes.get(name).as_ref() {
            Some(attr) => attr.downcast_ref::<T>(),
            None => None,
        }
    }

    pub fn add_attribute<T>(
        &mut self,
        name: ContextAttr<T>,
        value: T,
    ) -> Option<Box<dyn Any + Send>>
    where
        T: Send + 'static,
    {
        let ContextAttr { name, .. } = name;
        self.attributes.insert(name, Box::new(value))
    }
}

#[async_trait::async_trait]
pub trait Processor {
    /// Return the name of the processor
    fn name(&self) -> &str;

    /// Define the execut logic of the processor
    async fn execute(&mut self, context: &mut Context) -> Result<(), Error>;
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
    pub fn add_processor(&mut self, processor: Box<dyn Processor + Send>) {
        self.processors.push(processor);
    }

    /// Execute all the processors in the chain
    pub async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
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

    async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
        pin!(self);
        self.execute(context).await
    }
}
