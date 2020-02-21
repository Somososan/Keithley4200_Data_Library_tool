use crate::calamine_helper::MyRange;
use serde::{Deserialize, Serialize};

mod instrument;

mod opmode;

mod compliance;

mod measured;

mod vrange;

mod crange;

pub mod terminal;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TerminalParameter {
    pub terminal: terminal::Terminal,
    pub instrument: instrument::Instrument,
    pub operational_mode: opmode::OpMode,
    pub compliance: Option<f64>, //current limit of the terminal
    pub voltage: Option<measured::UnitMeasured>,
    pub voltage_range: Option<vrange::VRange>,
    pub current: Option<measured::UnitMeasured>,
    pub current_range: Option<crange::CRange>,
}

impl TerminalParameter {
    pub fn extract(
        column: &MyRange,
        test_type: &super::testparameter::TestType,
    ) -> Option<TerminalParameter> {
        use crate::Extract;
        let terminal = terminal::Terminal::extract(column).unwrap();
        let instrument = instrument::Instrument::extract(column).unwrap();
        let operational_mode = opmode::OpMode::extract(column, test_type).unwrap();
        let compliance = compliance::extract(column, test_type);
        let voltage = measured::extract(column, test_type, measured::UnitType::Voltage);
        let voltage_range = vrange::VRange::extract(column, test_type);
        let current = measured::extract(column, test_type, measured::UnitType::Current);
        let current_range = crange::CRange::extract(column, test_type);

        Some(TerminalParameter {
            terminal,
            instrument,
            operational_mode,
            compliance,
            voltage,
            voltage_range,
            current,
            current_range,
        })
    }
}
