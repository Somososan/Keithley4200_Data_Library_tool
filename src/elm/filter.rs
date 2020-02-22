//neasurement imports
use crate::measurement::device::process::Process;
use crate::measurement::testparameter::MeasurementSpeed;
use crate::measurement::testparameter::TestType;
use crate::measurement::timestamp::Date;
use crate::measurement::MeasurementCompact;

use boolinator::Boolinator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterQuery {
    sheet_names: Vec<String>,
    widths: Vec<String>,
    lengths: Vec<String>,
    temps: Vec<String>,
    wafer: String,
    dies: Vec<String>,
    test_type: String,
    measurement_speeds: Vec<String>,
    dates_between: (Option<Date>, Option<Date>),
}

impl FilterQuery {
    pub fn filter(&self, measurements: Vec<MeasurementCompact>) -> Vec<MeasurementCompact> {
        //filter functions
        let f_sheetname =
            |measurement: &MeasurementCompact| self.sheet_names.contains(&measurement.sheet_name);
        let f_width = |measurement: &MeasurementCompact| {
            self.widths
                .contains(&measurement.device.width.unwrap_or(0.001).to_string())
        };
        let f_length = |measurement: &MeasurementCompact| {
            self.lengths
                .contains(&measurement.device.length.unwrap_or(0.001).to_string())
        };
        let f_temp = |measurement: &MeasurementCompact| {
            let temp_without_k: Vec<String> = self
                .temps
                .clone()
                .into_iter()
                .map(|s| {
                    String::from_iter(
                        s.chars()
                            .filter_map(|c: char| (c.is_ascii_digit()).as_some(c)),
                    )
                })
                .collect();
            temp_without_k
                .clone()
                .contains(&measurement.device.temperature.unwrap_or(0).to_string())
        };
        let f_process = |measurement: &MeasurementCompact| {
            self.wafer == measurement.device.wafer.unwrap().to_string()
        };
        let f_die = |measurement: &MeasurementCompact| {
            let die = &measurement.device.die.clone().unwrap();
            self.dies.contains(&die)
        };
        let f_testtype = |measurement: &MeasurementCompact| {
            self.test_type == measurement.test_parameter.test_type.to_string()
        };
        let f_speed = |measurement: &MeasurementCompact| {
            self.measurement_speeds
                .contains(&measurement.test_parameter.measurement_speed.to_string())
        };
        let f_dates = |measurement: &MeasurementCompact| {
            let msmnt_date = measurement.test_time_stamp.year as u32 * 10000
                + measurement.test_time_stamp.month as u32 * 100
                + measurement.test_time_stamp.day as u32;
            let bottom_range = if let Some(time_stamp) = self.dates_between.0 {
                time_stamp.year as u32 * 10000
                    + time_stamp.month as u32 * 100
                    + time_stamp.day as u32
            } else {
                std::u32::MIN
            };
            let top_range = if let Some(time_stamp) = self.dates_between.1 {
                time_stamp.year as u32 * 10000
                    + time_stamp.month as u32 * 100
                    + time_stamp.day as u32
            } else {
                std::u32::MAX
            };
            msmnt_date >= bottom_range && msmnt_date <= top_range
        };

        let result: Vec<MeasurementCompact> =
            measurements.into_iter().filter(f_sheetname).collect();
        let result: Vec<MeasurementCompact> = result.into_iter().filter(f_width).collect();
        let result: Vec<MeasurementCompact> = result.into_iter().filter(f_length).collect();
        let result: Vec<MeasurementCompact> = result.into_iter().filter(f_temp).collect();
        let result: Vec<MeasurementCompact> = result.into_iter().filter(f_process).collect();
        let result: Vec<MeasurementCompact> = result.into_iter().filter(f_die).collect();
        let result: Vec<MeasurementCompact> = result.into_iter().filter(f_testtype).collect();
        let result: Vec<MeasurementCompact> = result.into_iter().filter(f_speed).collect();
        let result: Vec<MeasurementCompact> = result.into_iter().filter(f_dates).collect();
        result
    }
}

impl From<FilterOptions> for FilterQuery {
    fn from(options: FilterOptions) -> Self {
        FilterQuery {
            sheet_names: options.sheet_names.keys().map(|a| a.clone()).collect(),
            widths: options.widths.keys().map(|a| a.clone()).collect(),
            lengths: options.lengths.keys().map(|a| a.clone()).collect(),
            temps: options.temps.keys().map(|a| a.clone()).collect(),
            wafer: options
                .processes
                .keys()
                .map(|a| a.clone())
                .next()
                .unwrap_or(String::from("MINOXG")),
            dies: options.dies.keys().map(|a| a.clone()).collect(),
            test_type: String::from("Sampling"),
            measurement_speeds: options
                .measurement_speeds
                .keys()
                .map(|a| a.clone())
                .collect(),
            dates_between: (None, None),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterOptions {
    sheet_names: HashMap<String, u32>,
    widths: HashMap<String, u32>,
    lengths: HashMap<String, u32>,
    temps: HashMap<String, u32>,
    processes: HashMap<String, u32>,
    dies: HashMap<String, u32>,
    test_types: HashMap<String, u32>,
    measurement_speeds: HashMap<String, u32>,
    dates: HashMap<String, u32>,
}
impl FilterOptions {
    pub fn new(measurements: &Vec<MeasurementCompact>) -> FilterOptions {
        let sheet_name_keys: Vec<String> = measurements
            .into_iter()
            .map(|msmnt| msmnt.sheet_name.clone())
            .collect();
        let sheet_names = sheet_name_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.clone()).or_insert(0) += 1 as u32;
            acc
        });
        let width_keys: Vec<f64> = measurements
            .into_iter()
            .map(|msmnt| msmnt.device.width.unwrap_or(0.0))
            .collect();
        let widths = width_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });
        let length_keys: Vec<f64> = measurements
            .into_iter()
            .map(|msmnt| msmnt.device.length.unwrap_or(0.0))
            .collect();
        let lengths = length_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });
        let temp_keys: Vec<u32> = measurements
            .into_iter()
            .map(|msmnt| msmnt.device.temperature.unwrap_or(0))
            .collect();
        let temps = temp_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(format!("{}K", c.to_string())).or_insert(0) += 1 as u32;
            acc
        });
        let process_keys: Vec<Process> = measurements
            .into_iter()
            .map(|msmnt| msmnt.device.wafer.unwrap())
            .collect();
        let processes = process_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });
        let die_keys: Vec<String> = measurements
            .into_iter()
            .map(|msmnt| msmnt.device.die.clone().unwrap_or(String::from("")))
            .collect();
        let dies = die_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.clone()).or_insert(0) += 1 as u32;
            acc
        });
        let test_type_keys: Vec<&TestType> = measurements
            .into_iter()
            .map(|msmnt| &msmnt.test_parameter.test_type)
            .collect();
        let test_types = test_type_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });
        let measurement_speed_keys: Vec<&MeasurementSpeed> = measurements
            .into_iter()
            .map(|msmnt| &msmnt.test_parameter.measurement_speed)
            .collect();
        let measurement_speeds =
            measurement_speed_keys
                .iter()
                .fold(HashMap::new(), |mut acc, c| {
                    *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
                    acc
                });
        let date_keys: Vec<Date> = measurements
            .into_iter()
            .map(|msmnt| Date::from(msmnt.test_time_stamp))
            .collect();
        let dates = date_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });

        FilterOptions {
            sheet_names,
            widths,
            lengths,
            temps,
            processes,
            dies,
            test_types,
            measurement_speeds,
            dates,
        }
    }
    pub fn filtered(measurements: &Vec<MeasurementCompact>, filter: FilterQuery) -> FilterOptions {
        let mut sheet_name_keys: Vec<String> = measurements
            .clone()
            .into_iter()
            .map(|msmnt| msmnt.sheet_name.clone())
            .collect();
        sheet_name_keys.sort_unstable();
        sheet_name_keys.dedup();
        let filtern: FilterQuery = FilterQuery {
            sheet_names: sheet_name_keys,
            ..filter.clone()
        };
        let filteredn = filtern.filter(measurements.clone());
        let sheet_name_keys: Vec<String> = filteredn
            .into_iter()
            .map(|msmnt| msmnt.sheet_name.clone())
            .collect();
        let sheet_names = sheet_name_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.clone()).or_insert(0) += 1 as u32;
            acc
        });

        let mut width_keys: Vec<String> = measurements
            .clone()
            .into_iter()
            .map(|msmnt| msmnt.device.width.unwrap_or(0.0).to_string())
            .collect();
        width_keys.sort_unstable();
        width_keys.dedup();
        let filtern: FilterQuery = FilterQuery {
            widths: width_keys,
            ..filter.clone()
        };
        let filteredn = filtern.filter(measurements.clone());
        let width_keys: Vec<f64> = filteredn
            .into_iter()
            .map(|msmnt| msmnt.device.width.unwrap_or(0.0))
            .collect();
        let widths = width_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });

        let mut length_keys: Vec<String> = measurements
            .clone()
            .into_iter()
            .map(|msmnt| msmnt.device.length.unwrap_or(0.0).to_string())
            .collect();
        length_keys.sort_unstable();
        length_keys.dedup();
        let filtern: FilterQuery = FilterQuery {
            lengths: length_keys,
            ..filter.clone()
        };
        let filteredn = filtern.filter(measurements.clone());
        let length_keys: Vec<f64> = filteredn
            .into_iter()
            .map(|msmnt| msmnt.device.length.unwrap_or(0.0))
            .collect();
        let lengths = length_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });

        let mut temp_keys: Vec<String> = measurements
            .clone()
            .into_iter()
            .map(|msmnt| msmnt.device.temperature.unwrap_or(0).to_string())
            .collect();
        temp_keys.sort_unstable();
        temp_keys.dedup();
        let filtern: FilterQuery = FilterQuery {
            temps: temp_keys,
            ..filter.clone()
        };
        let filteredn = filtern.filter(measurements.clone());
        let temp_keys: Vec<u32> = filteredn
            .into_iter()
            .map(|msmnt| msmnt.device.temperature.unwrap_or(0))
            .collect();
        let temps = temp_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(format!("{}K", c.to_string())).or_insert(0) += 1 as u32;
            acc
        });

        let mut process_keys: Vec<String> = measurements
            .clone()
            .into_iter()
            .map(|msmnt| msmnt.device.wafer.unwrap().to_string())
            .collect();
        process_keys.sort_unstable();
        process_keys.dedup();
        let mut processes = HashMap::new();
        for process_key in process_keys.into_iter() {
            let filtern: FilterQuery = FilterQuery {
                wafer: process_key.clone(),
                ..filter.clone()
            };
            let filteredn = filtern.filter(measurements.clone());
            processes.insert(process_key, filteredn.len() as u32);
        }

        let mut die_keys: Vec<String> = measurements
            .clone()
            .into_iter()
            .map(|msmnt| msmnt.device.die.unwrap())
            .collect();
        die_keys.sort_unstable();
        die_keys.dedup();
        let filtern: FilterQuery = FilterQuery {
            dies: die_keys,
            ..filter.clone()
        };
        let filteredn = filtern.filter(measurements.clone());
        let die_keys: Vec<String> = filteredn
            .into_iter()
            .map(|msmnt| msmnt.device.die.unwrap())
            .collect();
        let dies = die_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.clone()).or_insert(0) += 1 as u32;
            acc
        });

        let mut test_type_keys: Vec<String> = measurements
            .clone()
            .into_iter()
            .map(|msmnt| msmnt.test_parameter.test_type.to_string().clone())
            .collect();
        test_type_keys.sort_unstable();
        test_type_keys.dedup();
        let mut test_types = HashMap::new();
        for test_type_key in test_type_keys.into_iter() {
            let filtern: FilterQuery = FilterQuery {
                test_type: test_type_key.clone(),
                ..filter.clone()
            };
            let filteredn = filtern.filter(measurements.clone());
            test_types.insert(test_type_key, filteredn.len() as u32);
        }

        let mut measurement_speed_keys: Vec<String> = measurements
            .clone()
            .into_iter()
            .map(|msmnt| msmnt.test_parameter.measurement_speed.to_string())
            .collect();
        measurement_speed_keys.sort_unstable();
        measurement_speed_keys.dedup();
        let filtern: FilterQuery = FilterQuery {
            measurement_speeds: measurement_speed_keys,
            ..filter.clone()
        };
        let filteredn = filtern.filter(measurements.clone());
        let measurement_speed_keys: Vec<MeasurementSpeed> = filteredn
            .into_iter()
            .map(|msmnt| msmnt.test_parameter.measurement_speed)
            .collect();
        let measurement_speeds =
            measurement_speed_keys
                .iter()
                .fold(HashMap::new(), |mut acc, c| {
                    *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
                    acc
                });

        let filtern: FilterQuery = FilterQuery {
            dates_between: (None, None),
            ..filter.clone()
        };
        let filteredn = filtern.filter(measurements.clone());
        let date_keys: Vec<Date> = filteredn
            .into_iter()
            .map(|msmnt| Date::from(msmnt.test_time_stamp))
            .collect();
        let dates = date_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });

        FilterOptions {
            sheet_names,
            widths,
            lengths,
            temps,
            processes,
            dies,
            test_types,
            measurement_speeds,
            dates,
        }
    }
}
