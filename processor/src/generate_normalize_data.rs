use crate::context::ProcessorContext;
use crate::error::Error;
use crate::Processor;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use tracing::{error, warn};

pub struct GenerateNormalizeDataProcessor {
    file_path: PathBuf,
}

impl GenerateNormalizeDataProcessor {
    pub fn new(file_path: PathBuf) -> Self {
        GenerateNormalizeDataProcessor { file_path }
    }
}
#[async_trait::async_trait]
impl Processor for GenerateNormalizeDataProcessor {
    fn name(&self) -> &str {
        "GenerateNormalizeDataProcessor"
    }

    async fn execute(&mut self, context: &mut ProcessorContext) -> Result<(), Error> {
        if let Err(e) = std::fs::remove_file(&self.file_path) {
            warn!("没有旧文件：{:?}", self.file_path)
        };
        let mut output_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;
        context.get_prize_records().iter().try_for_each(|record| {
            writeln!(
                output_file,
                "期号：{}，蓝球：{}，红球：{:?}",
                record.code, record.blue_ball, record.red_balls
            )?;

            Result::<(), Error>::Ok(())
        })?;
        Ok(())
    }
}
