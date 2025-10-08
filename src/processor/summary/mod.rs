use crate::processor::summary::relationship_based::RelationshipBasedProcessor;
use crate::processor::{summary::occur_based::OccurBasedSummaryProcessor, ProcessorChain};

mod occur_based;
mod relationship_based;
pub fn create_summary_processor_chain() -> ProcessorChain {
    ProcessorChain::new("SummaryProcessorChain")
        .add_processor(Box::new(OccurBasedSummaryProcessor))
        .add_processor(Box::new(RelationshipBasedProcessor))
}
