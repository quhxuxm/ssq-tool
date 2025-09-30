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
        let mut redball_column_data: Vec<Vec<bool>> = vec![vec![]; 33];
        let mut blueball_column_data: Vec<Vec<bool>> = vec![vec![]; 16];
        prize_record_page
            .prize_records
            .iter()
            .for_each(|prize_record| {
                let redballs = prize_record.red.as_slice();
                let blueball = prize_record.blue;
                blueball_column_data
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, v)| {
                        v.push(i == blueball as usize);
                    });
                redball_column_data
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, v)| {
                        v.push(redballs.contains(&(i as u8)));
                    });
            });

        println!("BLUE: {blueball_column_data:?}");
        println!("RED: {redball_column_data:?}");
        Ok(())
    }
}
