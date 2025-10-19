use derive_more::Display;
use serde::{Deserialize, Serialize};
use ssq_tool_domain::{BlueBall, PrBusinessObj, RedBall};
use std::{
    any::{type_name, Any},
    borrow::Borrow,
    collections::HashMap,
    marker::PhantomData,
};
use strum::IntoEnumIterator;

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

    pub fn occurance_count_by_average_interval(&self) -> usize {
        self.occurrence_count_by_average_interval
    }

    pub fn set_occurrence_count_by_average_interval(
        &mut self,
        occurrence_count_by_average_interval: usize,
    ) {
        self.occurrence_count_by_average_interval = occurrence_count_by_average_interval;
    }
}

#[derive(Debug)]
pub enum Relationship {
    Blue {
        ball: BlueBall,
        detail: HashMap<RedBall, usize>,
    },
    Red {
        ball: RedBall,
        blue_ball_detail: HashMap<BlueBall, usize>,
        red_ball_detail: HashMap<RedBall, usize>,
    },
}

impl Relationship {
    pub fn new_blue(ball: BlueBall) -> Self {
        let mut detail = HashMap::new();
        RedBall::iter().for_each(|red_ball| {
            detail.insert(red_ball, 0);
        });
        Self::Blue { ball, detail }
    }

    pub fn new_red(ball: RedBall) -> Self {
        let mut red_ball_detail = HashMap::new();
        RedBall::iter().for_each(|red_ball| {
            red_ball_detail.insert(red_ball, 0);
        });
        let mut blue_ball_detail = HashMap::new();
        BlueBall::iter().for_each(|blue_ball| {
            blue_ball_detail.insert(blue_ball, 0);
        });
        Self::Red {
            ball,
            red_ball_detail,
            blue_ball_detail,
        }
    }

    pub fn increase_relationship_with_blue(&mut self, target_ball: BlueBall) {
        if let Relationship::Red {
            blue_ball_detail, ..
        } = self
        {
            blue_ball_detail.entry(target_ball).and_modify(|count| {
                *count += 1;
            });
        }
    }

    pub fn increase_relationship_with_red(&mut self, target_ball: RedBall) {
        match self {
            Relationship::Blue { detail, .. } => {
                detail.entry(target_ball).and_modify(|count| {
                    *count += 1;
                });
            }
            Relationship::Red {
                red_ball_detail, ..
            } => {
                red_ball_detail.entry(target_ball).and_modify(|count| {
                    *count += 1;
                });
            }
        }
    }
}
