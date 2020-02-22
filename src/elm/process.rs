use crate::measurement::testdata::{terminal::Terminal, units::Unit};
use crate::measurement::testdata::{TestData, TestDataCompact};
use crate::measurement::Measurement;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::{self, Write};
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct ProcessQuery {
    what: Vec<ProcessingType>,
    combined: bool,
    from: Vec<ProcessData>,
}

#[derive(Debug, Serialize)]
pub struct DataSeries {
    title: String,
    data: Vec<ExportData>,
}
#[derive(Debug, Serialize)]
pub struct ExportData {
    measurement_id: String,
    designator: String,
    data: Vec<Vec<f64>>,
}

impl ExportData {
    fn from_testdata(id: String, testdata: TestData) -> ExportData {
        let designator: String = {
            if testdata.terminal == Terminal::Time {
                format!("T(s)")
            } else {
                format!(
                    "{}{}",
                    testdata.unit.to_string_concise(),
                    testdata.terminal.to_string_concise()
                )
            }
        };
        let data = testdata.data;
        ExportData {
            measurement_id: id,
            designator,
            data,
        }
    }
}

impl ProcessQuery {
    pub fn process(&self, measurements: Vec<Measurement>, output_dir: &str, script_dir: &str) {
        let selected_measurements: Vec<Measurement> = {
            let ids: Vec<(String, Vec<TestDataCompact>)> = self
                .from
                .clone()
                .into_iter()
                .map(|pd| (pd.id, pd.data))
                .collect();
            let id: Vec<String> = ids.iter().map(|id| id.0.clone()).collect();
            let measurements = measurements.iter().filter(|m| id.clone().contains(&m.id));
            let result: Vec<Measurement> = measurements
                .map(|m| {
                    let testdatacompact: Vec<TestDataCompact> = ids
                        .iter()
                        .find_map(|id| {
                            if id.0 == m.id {
                                Some(id.1.clone())
                            } else {
                                None
                            }
                        })
                        .unwrap();
                    let mut a = m.clone();
                    a.test_data = m
                        .test_data
                        .iter()
                        .filter_map(|t| t.from_compact(&testdatacompact))
                        .collect::<Vec<TestData>>();
                    a
                })
                .collect();

            result
        };

        let diff_wafer: bool = {
            let mut stringy = selected_measurements
                .iter()
                .filter_map(|m| m.device.wafer)
                .map(|p| p.to_string())
                .collect::<Vec<String>>();
            stringy.sort_unstable();
            stringy.dedup();
            stringy.len() > 1
        };
        let diff_die: bool = {
            let mut stringy = selected_measurements
                .iter()
                .filter_map(|m| m.device.die.clone())
                .collect::<Vec<String>>();
            stringy.sort_unstable();
            stringy.dedup();
            stringy.len() > 1
        };
        let diff_speed: bool = {
            let mut stringy = selected_measurements
                .iter()
                .map(|m| m.test_parameter.measurement_speed.to_string())
                .collect::<Vec<String>>();
            stringy.sort_unstable();
            stringy.dedup();
            stringy.len() > 1
        };
        let diff_temp: bool = {
            let mut stringy = selected_measurements
                .iter()
                .filter_map(|m| m.device.temperature)
                .collect::<Vec<u32>>();
            stringy.sort_unstable();
            stringy.dedup();
            stringy.len() > 1
        };
        let diff_width: bool = {
            let mut stringy = selected_measurements
                .iter()
                .filter_map(|m| m.device.width)
                .map(|w| w.to_string())
                .collect::<Vec<String>>();
            stringy.sort_unstable();
            stringy.dedup();
            stringy.len() > 1
        };
        let diff_length: bool = {
            let mut stringy = selected_measurements
                .iter()
                .filter_map(|m| m.device.length)
                .map(|l| l.to_string())
                .collect::<Vec<String>>();
            stringy.sort_unstable();
            stringy.dedup();
            stringy.len() > 1
        };
        let diff_time: bool = {
            let mut stringy = selected_measurements
                .iter()
                .map(|m| m.test_time_stamp)
                .map(|l| l.to_string())
                .collect::<Vec<String>>();
            stringy.sort_unstable();
            stringy.dedup();
            stringy.len() > 1
        };

        self.what.par_iter().for_each(|pt| {
            let testdata_total = selected_measurements
                .iter()
                .map(|m| {
                    let testdata = m.test_data.clone();
                    let data: Vec<ExportData> = {
                        let temp = match pt {
                            ProcessingType::Raw => testdata,
                            ProcessingType::Id_versus_time
                            | ProcessingType::Id_normalized_versus_time
                            | ProcessingType::Psd => testdata
                                .into_iter()
                                .filter(|t| {
                                    t.terminal == Terminal::Time
                                        || (t.terminal == Terminal::Drain
                                            && t.unit == Unit::Current)
                                })
                                .collect::<Vec<TestData>>(),
                            ProcessingType::Id_bins | ProcessingType::Id_bins_normalized => {
                                testdata
                                    .into_iter()
                                    .filter(|t| {
                                        t.terminal == Terminal::Drain && t.unit == Unit::Current
                                    })
                                    .collect::<Vec<TestData>>()
                            }
                            ProcessingType::Ts_bins => testdata
                                .into_iter()
                                .filter(|t| t.terminal == Terminal::Time && t.unit == Unit::Seconds)
                                .collect::<Vec<TestData>>(),
                            ProcessingType::Id_for_swept_VDS_and_VGS => testdata
                                .into_iter()
                                .filter(|t| {
                                    (t.terminal == Terminal::Drain && t.unit == Unit::Current)
                                        || (t.terminal == Terminal::Drain
                                            && t.unit == Unit::Voltage)
                                        || (t.terminal == Terminal::Gate && t.unit == Unit::Voltage)
                                })
                                .collect::<Vec<TestData>>(),
                        };
                        temp.into_iter()
                            .map(|t| ExportData::from_testdata(m.id.clone(), t))
                            .collect::<Vec<ExportData>>()
                    };
                    let title: String = {
                        let wafer = if diff_wafer {
                            m.device
                                .wafer
                                .and_then(|w| Some(format!("P={} ", w)))
                                .unwrap_or("".to_string())
                        } else {
                            "".to_string()
                        };
                        let die = if diff_die {
                            m.device
                                .die
                                .clone()
                                .and_then(|d| Some(format!("D={} ", d.as_str())))
                                .unwrap_or("".to_string())
                        } else {
                            "".to_string()
                        };
                        let speed = if diff_speed {
                            format!("Spd={} ", m.test_parameter.measurement_speed.to_string())
                        } else {
                            "".to_string()
                        };
                        let temp = if diff_temp {
                            m.device
                                .temperature
                                .and_then(|t| Some(format!("T={}°K ", t)))
                                .unwrap_or("".to_string())
                        } else {
                            "".to_string()
                        };
                        let width = if diff_width {
                            m.device
                                .width
                                .and_then(|w| {
                                    if w > 100.0 {
                                        Some(format!("W={}µm ", (w / 1000.0)))
                                    } else {
                                        Some(format!("W={}nm ", w))
                                    }
                                })
                                .unwrap_or("".to_string())
                        } else {
                            "".to_string()
                        };
                        let length = if diff_length {
                            m.device
                                .length
                                .and_then(|l| {
                                    if l > 100.0 {
                                        Some(format!("L={}µm ", (l / 1000.0)))
                                    } else {
                                        Some(format!("L={}nm ", l))
                                    }
                                })
                                .unwrap_or("".to_string())
                        } else {
                            "".to_string()
                        };
                        let id =
                            if !(diff_wafer && diff_die && diff_temp && diff_width && diff_length)
                                && diff_time
                                || self.from.len() == 1
                            {
                                m.id.clone()
                            } else {
                                //println!("{} {} {} {} {} {} {}",diff_wafer,diff_die,diff_temp,diff_width,diff_length, diff_time,m.test_time_stamp);
                                "Error No Distinctive Parameters".to_string()
                            };

                        format!("{}{}{}{}{}{}{}", wafer, die, speed, temp, width, length, id)
                    };
                    //println!("title:{}", title);
                    DataSeries { title, data }
                })
                .collect::<Vec<DataSeries>>();

            let python_script = |script: &str| {
                let python_output = Command::new("python")
                    .arg(format!("{1}/{0}.py", script, script_dir))
                    .arg(format!("{}", script_dir))
                    .arg(format!("{}", output_dir))
                    .output()
                    .expect("process failed to execute");
                println!("status of {}.py: {}", script, python_output.status);
                io::stdout()
                    .write_all(&python_output.stdout)
                    .unwrap_or_default();
                io::stderr()
                    .write_all(&python_output.stderr)
                    .unwrap_or_default();
            };

            if self.combined {
                match pt {
                    ProcessingType::Raw => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(format!("{}/Raw.json", output_dir), v.as_str())
                            .expect("error writing json");
                    }
                    ProcessingType::Id_bins => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(format!("{}/data/id_bins.json", script_dir), v.as_str())
                            .expect("error writing json");
                        python_script("id_bins");
                    }
                    ProcessingType::Id_bins_normalized => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(
                            format!("{}/data/id_bins_normalized.json", script_dir),
                            v.as_str(),
                        )
                        .expect("error writing json");
                        python_script("id_bins_normalized");
                    }
                    ProcessingType::Ts_bins => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(format!("{}/data/ts_bins.json", script_dir), v.as_str())
                            .expect("error writing json");
                        python_script("ts_bins");
                    }
                    ProcessingType::Id_versus_time => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(
                            format!("{}/data/id_versus_time.json", script_dir),
                            v.as_str(),
                        )
                        .expect("error writing json");
                        python_script("id_versus_time");
                    }
                    ProcessingType::Id_normalized_versus_time => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(
                            format!("{}/data/id_versus_time_normalized.json", script_dir),
                            v.as_str(),
                        )
                        .expect("error writing json");
                        python_script("id_versus_time_normalized");
                    }
                    ProcessingType::Psd => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(format!("{}/data/psd.json", script_dir), v.as_str())
                            .expect("error writing json");
                        python_script("psd_nfft");
                    }
                    _ => {}
                }
            } else {
                match pt {
                    ProcessingType::Raw => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(format!("{}/Raw.json", output_dir), v.as_str())
                            .expect("error writing json");
                    }
                    ProcessingType::Id_bins => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(format!("{}/data/id_bins.json", script_dir), v.as_str())
                            .expect("error writing json");
                        python_script("id_bins");
                    }
                    ProcessingType::Id_bins_normalized => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(
                            format!("{}/data/id_bins_normalized.json", script_dir),
                            v.as_str(),
                        )
                        .expect("error writing json");
                        python_script("id_bins_normalized");
                    }
                    ProcessingType::Ts_bins => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(format!("{}/data/ts_bins.json", script_dir), v.as_str())
                            .expect("error writing json");
                        python_script("ts_bins");
                    }
                    ProcessingType::Id_versus_time => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(
                            format!("{}/data/id_versus_time.json", script_dir),
                            v.as_str(),
                        )
                        .expect("error writing json");
                        python_script("id_versus_time");
                    }
                    ProcessingType::Id_normalized_versus_time => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(
                            format!("{}/data/id_versus_time_normalized.json", script_dir),
                            v.as_str(),
                        )
                        .expect("error writing json");
                        python_script("id_versus_time_normalized");
                    }
                    ProcessingType::Psd => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write(format!("{}/data/psd.json", script_dir), v.as_str())
                            .expect("error writing json");
                        python_script("psd_nfft");
                    }
                    _ => {}
                }
            }
        });
    } //fn
} //impl

#[derive(Debug, Deserialize, Clone)]
pub struct ProcessData {
    id: String,
    data: Vec<TestDataCompact>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "process_type")]
pub enum ProcessingType {
    Raw,
    Id_versus_time,
    Id_normalized_versus_time,
    Id_bins,
    Id_bins_normalized,
    Ts_bins,
    Id_for_swept_VDS_and_VGS,
    Psd,
}
