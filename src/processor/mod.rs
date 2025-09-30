use crate::{domain::PrizeRecordPage, error::Error};
use async_trait;
use std::{any::Any, collections::HashMap, sync::Arc};

pub mod prepare;

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

    pub fn attribute<T>(&self, name: &str) -> Option<&T>
    where
        T: Send + 'static,
    {
        match self.attributes.get(name).as_ref() {
            Some(attr) => attr.downcast_ref::<T>(),
            None => None,
        }
    }

    pub fn add_attribute<T>(&mut self, name: &str, value: T) -> Option<Box<dyn Any + Send>>
    where
        T: Send + 'static,
    {
        self.attributes.insert(name.to_owned(), Box::new(value))
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
