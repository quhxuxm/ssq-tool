use crate::error::Error;
use crate::processor::{Context, Processor};

pub struct RelationshipBasedProcessor;

#[async_trait::async_trait]
impl Processor for RelationshipBasedProcessor{
    fn name(&self) -> &str {
        "RelationshipBasedProcessor"
    }

    async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
        todo!()
    }
}