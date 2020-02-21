use crate::calamine_helper::MyRange;
use crate::measurement::testparameter::TestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum VRange {
    BestFixed,
}

#[allow(non_snake_case)]
struct Positions {
    Sweeping: usize,
    Sampling: usize,
}

const OFFSETS: Positions = Positions {
    Sweeping: 13,
    Sampling: 10,
};

impl VRange {
    pub fn extract(column: &MyRange, test_type: &TestType) -> Option<VRange> {
        let at_offset = |offset| match column.it.get((offset, 0))?.get_string() {
            Some("Best Fixed") => Some(VRange::BestFixed),
            _ => None,
        };

        match test_type {
            TestType::Sweeping => at_offset(OFFSETS.Sweeping),
            TestType::Sampling => at_offset(OFFSETS.Sampling),
        }
    }
}
