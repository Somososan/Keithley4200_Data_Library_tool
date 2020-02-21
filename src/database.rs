use crate::measurement::timestamp::TimeStamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    pub files_scanned_before: Vec<String>,
    pub id_day_counter: HashMap<String, u32>,
    pub measurements: Vec<crate::measurement::Measurement>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            files_scanned_before: vec![],
            id_day_counter: HashMap::new(),
            measurements: vec![],
        }
    }

    pub fn generate_id(&mut self, time_stamp: TimeStamp) -> String {
        let string = format!(
            "{:0>4}{:0>2}{:0>2}",
            time_stamp.year, time_stamp.month, time_stamp.day
        );
        let counter = self.id_day_counter.entry(string.clone()).or_insert(0);
        *counter += 1;

        format!("{}-{}", string.as_str(), counter)
    }
}
