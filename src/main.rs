use std::sync::Arc;

use crate::{
    error::Error,
    processor::{Processor, ProcessorChain, prepare::PrepareProcessor},
};

pub mod domain;
pub mod error;
pub mod official;
pub mod processor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let prize_record_page = official::generate_official_data().await?;
    let processors: Vec<Box<dyn Processor>> = vec![Box::new(PrepareProcessor)];
    let mut processor_chain = ProcessorChain::from(processors);
    let mut context = processor::Context::new(Arc::new(prize_record_page));
    processor_chain.execute(&mut context).await?;
    Ok(())
}
