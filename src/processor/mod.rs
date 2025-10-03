use crate::{
    domain::PrizeRecordPage,
    error::Error,
    processor::business_obj::{BlueBallRelationship, RedBallRelationship},
};
use async_trait;
use std::{
    any::Any,
    borrow::Borrow,
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, LazyLock},
};

pub mod business_obj;
pub mod count_prized_blueball;
pub mod count_prized_redball;
pub mod occur_interval;
pub mod relationship;
pub mod summary;

pub const PRIZED_BLUE_BALLS_COUNTS: LazyLock<ContextAttr<HashMap<u8, i32>>> =
    LazyLock::new(|| ContextAttr::new("PRIZED_BLUE_BALLS_COUNT"));
pub const PRIZED_RED_BALLS_COUNTS: LazyLock<ContextAttr<HashMap<u8, i32>>> =
    LazyLock::new(|| ContextAttr::new("PRIZED_RED_BALLS_COUNT"));
pub const BLUE_BALL_RELATIONSHIPS: LazyLock<ContextAttr<HashMap<u8, BlueBallRelationship>>> =
    LazyLock::new(|| ContextAttr::new("BLUE_BALL_RELATIONSHIPS"));
pub const RED_BALL_RELATIONSHIPS: LazyLock<ContextAttr<HashMap<u8, RedBallRelationship>>> =
    LazyLock::new(|| ContextAttr::new("RED_BALL_RELATIONSHIPS"));

pub const SORTED_BLUE_BALLS_COUNTS: LazyLock<ContextAttr<Vec<(u8, i32)>>> =
    LazyLock::new(|| ContextAttr::new("SORTED_BLUE_BALLS_COUNTS"));
pub const SORTED_RED_BALLS_COUNTS: LazyLock<ContextAttr<Vec<(u8, i32)>>> =
    LazyLock::new(|| ContextAttr::new("SORTED_RED_BALLS_COUNTS"));

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
    prize_record_page: Arc<PrizeRecordPage>,
    attributes: HashMap<String, Box<dyn Any + Send + 'static>>,
}

impl Context {
    pub fn new(prize_record_page: Arc<PrizeRecordPage>) -> Self {
        Self {
            attributes: HashMap::new(),
            prize_record_page,
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

#[derive(Default)]
pub struct ProcessorChain {
    processors: Vec<Box<dyn Processor>>,
}

impl ProcessorChain {
    /// Add a processor to the chain
    pub fn add_processor(&mut self, processor: Box<dyn Processor>) {
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

impl From<Vec<Box<dyn Processor>>> for ProcessorChain {
    fn from(processors: Vec<Box<dyn Processor>>) -> Self {
        Self { processors }
    }
}
