use crate::calamine_helper::MyRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Instrument {
    SMU1,
    SMU2,
    SMU3,
    SMU4,
    GNDU,
    PMU1,
    PMU2,
    PMU3,
    PMU4,
}

impl crate::Extract for Instrument {
    fn extract(column: &MyRange) -> Option<Instrument> {
        match column.it.get((1, 0))?.get_string()? {
            "SMU1" => Some(Instrument::SMU1),
            "SMU2" => Some(Instrument::SMU2),
            "SMU3" => Some(Instrument::SMU3),
            "SMU4" => Some(Instrument::SMU4),
            "GNDU" => Some(Instrument::GNDU),
            "PMU1" => Some(Instrument::PMU1),
            "PMU2" => Some(Instrument::PMU2),
            "PMU3" => Some(Instrument::PMU3),
            "PMU4" => Some(Instrument::PMU4),
            _ => None,
        }
    }
}
