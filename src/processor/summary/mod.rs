use crate::processor::{ProcessorChain, summary::occur_based::OccurBasedSummaryProcessor};
mod occur_based;

pub fn create_summary_processor_chain() -> ProcessorChain {
    let mut processor_chain = ProcessorChain::new("SummaryProcessorChain");
    processor_chain.add_processor(Box::new(OccurBasedSummaryProcessor));
    processor_chain
}
