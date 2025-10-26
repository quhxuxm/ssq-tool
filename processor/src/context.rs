use derive_more::Display;
use serde::{Deserialize, Serialize};
use ssq_tool_domain::PrBusinessObj;
use std::{
    any::{Any, type_name},
    borrow::Borrow,
    collections::HashMap,
    marker::PhantomData,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Display)]
#[display("{name}")]
pub struct ProcessorContextAttr<T>
where
    T: Any + Send + 'static,
{
    name: String,
    _val_type: PhantomData<T>,
}

impl<T> ProcessorContextAttr<T>
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

static PROCESSOR_CONTEXT_ATTR_KEY_PREFIX: &str = "$__PROCESSOR_CTX_ATTR__$";

pub struct ProcessorContext<'a> {
    prize_records: &'a [PrBusinessObj],
    expect_result_size: usize,
    attributes: HashMap<String, Box<dyn Any + Send + 'static>>,
}

impl<'a> ProcessorContext<'a> {
    pub fn new(prize_records: &'a [PrBusinessObj], expect_result_size: usize) -> Self {
        Self {
            attributes: HashMap::new(),
            expect_result_size,
            prize_records,
        }
    }

    pub fn get_prize_records(&self) -> &'a [PrBusinessObj] {
        self.prize_records
    }

    pub fn get_attribute<T>(&self, name: &ProcessorContextAttr<T>) -> Option<&T>
    where
        T: Send + 'static,
    {
        let ProcessorContextAttr { name, .. } = name;
        let attr_key = format!(
            "{PROCESSOR_CONTEXT_ATTR_KEY_PREFIX}_{name}_[{}]",
            type_name::<T>()
        );
        match self.attributes.get(&attr_key).as_ref() {
            Some(attr) => attr.downcast_ref::<T>(),
            None => None,
        }
    }

    pub fn set_attribute<T>(
        &mut self,
        attr: &ProcessorContextAttr<T>,
        value: T,
    ) -> Option<Box<dyn Any + Send>>
    where
        T: Send + 'static,
    {
        let ProcessorContextAttr { name, .. } = &attr;
        let attr_key = format!(
            "{PROCESSOR_CONTEXT_ATTR_KEY_PREFIX}_{name}_[{}]",
            type_name::<T>()
        );
        self.attributes.insert(attr_key, Box::new(value))
    }

    pub fn expect_result_size(&self) -> usize {
        self.expect_result_size
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "出现细节")]
pub struct OccurrenceDetail {
    #[serde(rename = "平均出现间隔")]
    average_occur_interval: usize,
    #[serde(rename = "官方数据中的出现次数")]
    occurrence_count_by_official_data: usize,
    #[serde(rename = "最后一次出现的索引")]
    latest_occur_seq: usize,
    #[serde(rename = "按照平均出现间隔计算的出现次数")]
    occurrence_count_by_average_interval: usize,
}

impl OccurrenceDetail {
    pub fn average_occur_interval(&self) -> usize {
        self.average_occur_interval
    }

    pub fn set_average_occur_interval(&mut self, average_occur_interval: usize) {
        self.average_occur_interval = average_occur_interval;
    }

    pub fn occurrence_count_by_official_data(&self) -> usize {
        self.occurrence_count_by_official_data
    }

    pub fn set_occurrence_count_by_official_data(
        &mut self,
        occurrence_count_by_official_data: usize,
    ) {
        self.occurrence_count_by_official_data = occurrence_count_by_official_data
    }

    pub fn set_latest_occur_seq(&mut self, latest_occur_seq: usize) {
        self.latest_occur_seq = latest_occur_seq
    }

    pub fn latest_occur_seq(&self) -> usize {
        self.latest_occur_seq
    }

    pub fn occurrence_count_by_average_interval(&self) -> usize {
        self.occurrence_count_by_average_interval
    }

    pub fn set_occurrence_count_by_average_interval(
        &mut self,
        occurrence_count_by_average_interval: usize,
    ) {
        self.occurrence_count_by_average_interval = occurrence_count_by_average_interval;
    }
}
