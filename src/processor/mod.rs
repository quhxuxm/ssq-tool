use crate::{domain::PrizeRecordPage, error::Error};
use async_trait;
use std::{any::Any, sync::Arc};

pub mod storage;

pub struct Context {
    pub prize_record_page: Arc<PrizeRecordPage>,
    pub previous_result: Option<Box<dyn Any>>,
}

impl Context {
    pub fn previous_result<T>(&self) -> Option<&T> {
        let previous_result = self.previous_result()?;
        let previous_result = previous_result as &dyn Any;
        let previous_resutl = previous_result.downcast_ref::<T>();
        previous_resutl
    }
}

#[async_trait::async_trait]
pub trait Processor {
    async fn execute(&mut self, context: &mut Context) -> Result<(), Error>;
}

#[derive(Default)]
pub struct ProcessorChain {
    processors: Vec<Box<dyn Processor>>,
}

impl ProcessorChain {
    pub fn add_processor(&mut self, processor: Box<dyn Processor>) {
        self.processors.push(processor);
    }

    async fn execute(&mut self, mut context: Context) -> Result<(), Error> {
        for processor in self.processors.iter_mut() {
            processor.execute(&mut context).await?;
        }
        Ok(())
    }
}

impl From<Vec<Box<dyn Processor>>> for ProcessorChain {
    fn from(processors: Vec<Box<dyn Processor>>) -> Self {
        Self { processors }
    }
}
