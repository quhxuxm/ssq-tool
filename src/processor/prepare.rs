use crate::{
    error::Error,
    processor::{Context, Processor},
};

pub struct PrepareProcessor;

#[async_trait::async_trait]
impl Processor for PrepareProcessor {
    fn name(&self) -> &str {
        "PrepareProcessor"
    }

    async fn execute(&mut self, context: &mut Context) -> Result<(), Error> {
        let Context {
            prize_record_page, ..
        } = context;
        for prize_record in &prize_record_page.prize_records {
            let redballs = prize_record.red.as_slice();
            let blueball = prize_record.blue;
            println!("RED: {redballs:?} ; BLUE: {blueball}");
        }

        Ok(())
    }
}
