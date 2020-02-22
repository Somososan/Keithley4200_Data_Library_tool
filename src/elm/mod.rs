use serde::{Deserialize, Serialize};

pub mod filter;

pub mod process;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "fromrust")]
pub struct ToElm {
    pub message_nr: u32,
    pub task_done: Task,
    pub measurements: Vec<crate::measurement::MeasurementCompact>,
    pub filter_options: filter::FilterOptions,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "torust", content = "content")]
pub enum FromElm {
    Init,
    Log(String),
    Filter(filter::FilterQuery),
    Process(process::ProcessQuery),
}

//{"torust":{"Log":"updated model"}}
#[derive(Debug, Serialize, Deserialize)]
pub enum Task {
    Init,
    Filtering,
    Processing,
}
