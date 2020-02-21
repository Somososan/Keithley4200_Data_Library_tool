use crate::calamine_helper::MyRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestParameter {
    pub test_type: TestType,
    pub measurement_speed: MeasurementSpeed,
    pub ad_aperture: Option<f64>,
    pub filter_factor: Option<f64>,
    pub interval_time: Option<f64>,
    pub sweep_delay_time: Option<f64>,
    pub hold_time: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TestType {
    Sampling,
    Sweeping,
}

impl std::fmt::Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestType::Sampling => write!(f, "Sampling"),
            TestType::Sweeping => write!(f, "Sweeping"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MeasurementSpeed {
    Fast,
    Normal,
    Quiet,
    Custom,
}

impl std::fmt::Display for MeasurementSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MeasurementSpeed::Fast => write!(f, "Fast"),
            MeasurementSpeed::Normal => write!(f, "Normal"),
            MeasurementSpeed::Quiet => write!(f, "Quiet"),
            MeasurementSpeed::Custom => write!(f, "Custom"),
        }
    }
}

impl crate::Extract for TestParameter {
    fn extract(sheet: &MyRange) -> Option<TestParameter> {
        match sheet.it.get((4, 1))?.get_string()? {
            "Sweeping" => {
                let test_type = TestType::Sweeping;
                let measurement_speed = match sheet.it.get((5, 1))?.get_string()? {
                    "Fast" => MeasurementSpeed::Fast,
                    "Normal" => MeasurementSpeed::Normal,
                    "Quiet" => MeasurementSpeed::Quiet,
                    _ => MeasurementSpeed::Custom,
                };
                let ad_aperture = None;
                let filter_factor = None;
                let interval_time = None;
                let sweep_delay_time = sheet.it.get((6, 1))?.get_string()?.parse::<f64>().ok();
                let hold_time = sheet.it.get((7, 1))?.get_string()?.parse::<f64>().ok()?;

                Some(TestParameter {
                    test_type,
                    measurement_speed,
                    ad_aperture,
                    filter_factor,
                    interval_time,
                    sweep_delay_time,
                    hold_time,
                })
            }
            "Sampling" => {
                let test_type = TestType::Sampling;
                let measurement_speed = match sheet.it.get((5, 1))?.get_string()? {
                    "Fast" => MeasurementSpeed::Fast,
                    "Normal" => MeasurementSpeed::Normal,
                    "Quiet" => MeasurementSpeed::Quiet,
                    _ => MeasurementSpeed::Custom,
                };
                let ad_aperture = None;
                let filter_factor = None;
                let interval_time = sheet.it.get((6, 1))?.get_string()?.parse::<f64>().ok();
                let sweep_delay_time = None;
                let hold_time = sheet.it.get((7, 1))?.get_string()?.parse::<f64>().ok()?;

                Some(TestParameter {
                    test_type,
                    measurement_speed,
                    ad_aperture,
                    filter_factor,
                    interval_time,
                    sweep_delay_time,
                    hold_time,
                })
            }
            _ => None,
        }
    }
}
