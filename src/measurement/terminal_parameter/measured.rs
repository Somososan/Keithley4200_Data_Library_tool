use super::super::testparameter::TestType;
use crate::calamine_helper::MyRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum UnitMeasured {
    Measured,
    Programmed,
}

pub enum UnitType {
    Voltage,
    Current,
}

#[allow(non_snake_case)]
struct Unit {
    Voltage: usize,
    Current: usize,
}

#[allow(non_snake_case)]
struct Positions {
    Sweeping: Unit,
    Sampling: Unit,
}

const OFFSETS: Positions = Positions {
    Sweeping: Unit {
        Voltage: 11,
        Current: 10,
    },
    Sampling: Unit {
        Voltage: 8,
        Current: 7,
    },
};

pub fn extract(column: &MyRange, test_type: &TestType, unit: UnitType) -> Option<UnitMeasured> {
    let extract_at = |offset| match column.it.get((offset, 0))?.get_string()? {
        "Measured" => Some(UnitMeasured::Measured),
        "Programmed" => Some(UnitMeasured::Programmed),
        _ => None,
    };

    match test_type {
        TestType::Sweeping => match unit {
            UnitType::Voltage => extract_at(OFFSETS.Sweeping.Voltage),
            UnitType::Current => extract_at(OFFSETS.Sweeping.Current),
        },
        TestType::Sampling => match unit {
            UnitType::Voltage => extract_at(OFFSETS.Sampling.Voltage),
            UnitType::Current => extract_at(OFFSETS.Sampling.Current),
        },
    }
}
