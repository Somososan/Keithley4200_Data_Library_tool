use crate::calamine_helper::MyRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct OpMode {
    pub op_type: OpModeType,
    pub bias: Option<f64>,
    pub start: Option<f64>,
    pub stop: Option<f64>,
    pub stepsize: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum OpModeType {
    VoltageBias,
    VoltageLinearSweep, // (start,stop,stepsize)
    VoltageStep,        // (start,stop,stepsize)
    CurrentBias,
    CurrentLinearSweep, // (start,stop,stepsize)
    CurrentStep,        // (start,stop,stepsize)
    Common,
    Floating,
}

struct Bias {
    bias_or_start: usize,
    stop: usize,
    stepsize: usize,
}

#[allow(non_snake_case)]
struct Positions {
    Sweeping: Bias,
    Sampling: Bias,
}

const OFFSETS: Positions = Positions {
    Sweeping: Bias {
        bias_or_start: 5,
        stop: 6,
        stepsize: 7,
    },
    Sampling: Bias {
        bias_or_start: 4,
        stop: 4,
        stepsize: 4,
    },
};

impl OpMode {
    pub fn extract(
        column: &MyRange,
        test_type: &crate::measurement::testparameter::TestType,
    ) -> Option<OpMode> {
        use crate::measurement::testparameter::TestType;
        let extract_at = |offset| {
            column
                .it
                .get((offset, 0))?
                .get_string()?
                .parse::<f64>()
                .ok()
        };
        let op_type = match column.it.get((3, 0))?.get_string()? {
            "Voltage Bias" => Some(OpModeType::VoltageBias),
            "Current Bias" => Some(OpModeType::CurrentBias),
            "Voltage Linear Sweep" => Some(OpModeType::VoltageLinearSweep),
            "Current Linear Sweep" => Some(OpModeType::CurrentLinearSweep),
            "Voltage Step" => Some(OpModeType::VoltageStep),
            "Current Step" => Some(OpModeType::CurrentStep),
            "Common" => Some(OpModeType::Common),
            "Floating" => Some(OpModeType::Floating),
            _ => None,
        }
        .unwrap();

        let offset: Bias = match test_type {
            TestType::Sweeping => OFFSETS.Sweeping,
            TestType::Sampling => OFFSETS.Sampling,
        };

        match op_type {
            OpModeType::VoltageBias | OpModeType::CurrentBias => {
                let bias = extract_at(offset.bias_or_start);
                let start = None;
                let stop = None;
                let stepsize = None;
                Some(OpMode {
                    op_type,
                    bias,
                    start,
                    stop,
                    stepsize,
                })
            }
            OpModeType::Common | OpModeType::Floating => {
                let bias = None;
                let start = None;
                let stop = None;
                let stepsize = None;
                Some(OpMode {
                    op_type,
                    bias,
                    start,
                    stop,
                    stepsize,
                })
            }
            _ => {
                let bias = None;
                let start = extract_at(offset.bias_or_start);
                let stop = extract_at(offset.stop);
                let stepsize = extract_at(offset.stepsize);
                Some(OpMode {
                    op_type,
                    bias,
                    start,
                    stop,
                    stepsize,
                })
            }
        }
    }
}
