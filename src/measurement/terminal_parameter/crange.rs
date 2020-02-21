use crate::calamine_helper::MyRange;
use crate::measurement::testparameter::TestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CRange {
    LimitedAuto(String),
    Auto,
}

#[allow(non_snake_case)]
struct Positions {
    Sweeping: usize,
    Sampling: usize,
}

const OFFSETS: Positions = Positions {
    Sweeping: 12,
    Sampling: 9,
};

impl CRange {
    pub fn extract(column: &MyRange, test_type: &TestType) -> Option<CRange> {
        let at_offset = |offset| -> Option<CRange> {
            let string = column.it.get((offset, 0))?.get_string()?;
            if string.starts_with("Limited") {
                Some(CRange::LimitedAuto(string.rsplit("=").next()?.to_string()))
            } else if string == "Auto" {
                Some(CRange::Auto)
            } else {
                None
            }
        };
        match test_type {
            TestType::Sweeping => at_offset(OFFSETS.Sweeping),
            TestType::Sampling => at_offset(OFFSETS.Sampling),
        }
    }
}
