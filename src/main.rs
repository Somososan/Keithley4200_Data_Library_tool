use crate::database::Database;
use serde::*;
use std::collections::HashMap;
use std::path::Path;
use webview::*;
//use serde_json::*;
use boolinator::*;
use clap::{App, Arg};
use rayon::prelude::*;
use std::fs::{self};
use std::io::{self, Write};
use std::iter::FromIterator;
use std::process::Command;

mod measurement;

mod calamine_helper;

mod database;

pub trait Extract {
    fn extract(sheet: &calamine_helper::MyRange) -> Option<Self>
    where
        Self: std::marker::Sized;
}

fn populate_from_path(
    root: String,
    relative_dir: String,
    storage: &mut database::Database,
) -> std::io::Result<()> {
    let dir_string = format!("{}{}", root.clone(), relative_dir);

    let dir = Path::new(dir_string.as_str());

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let relative_path: String = path.to_str().unwrap().replace(root.clone().as_str(), "");
            if path.is_dir() {
                populate_from_path(root.clone(), relative_path, storage)?;
            } else if path.extension().unwrap() == "xls" {
                measurement::Measurement::extract(root.clone(), relative_path, storage);
            } else {
            }
        }
    }
    Ok(())
}

fn main() {
    let matches = App::new("Library tool")
        .version("0.1")
        .author("C Karaliolios")
        .about(
            "Processes Keithley 4200 parameter analyzer data and generates images and a interface",
        )
        .arg(
            Arg::with_name("input_directory")
                .short("i")
                .long("input_dir")
                .value_name("PATH")
                .help("Sets the directory to be searched"),
        )
        .arg(
            Arg::with_name("output_directory")
                .short("o")
                .long("output_dir")
                .value_name("PATH")
                .help("Sets the directory for the outputs to be collected in"),
        )
        .arg(
            Arg::with_name("script_directory")
                .short("s")
                .long("script_dir")
                .value_name("PATH")
                .help("Sets the directory where the python scripts selected out of"),
        )
        .get_matches();

    //default input directory
    let input_string: String = format!("{}/tests", env!("CARGO_MANIFEST_DIR"));
    // get input directory from CLI
    let input_dir = matches
        .value_of("input_directory")
        .unwrap_or(input_string.as_str())
        .to_string();

    //default output directory
    let output_string: String = format!("{}/output", env!("CARGO_MANIFEST_DIR"));
    // get output directory from CLI
    let output_dir = matches
        .value_of("output_directory")
        .unwrap_or(output_string.as_str())
        .to_string();

    //default script directory
    let script_string: String = format!("{}/scripts", env!("CARGO_MANIFEST_DIR"));
    // get input directory from CLI
    let script_dir = matches
        .value_of("script_directory")
        .unwrap_or(script_string.as_str())
        .to_string();

    //initialize id and measurement vector
    let json = fs::read_to_string(format!("{}/result.json", output_dir));
    let storage: &mut Database = &mut Database::new();
    match json {
        Ok(string) => {
            *storage = serde_json::from_str::<Database>(string.as_str()).unwrap_or(Database::new())
        }
        _ => (),
    };
    populate_from_path(input_dir, String::from(""), storage).expect("Error transfercing path");

    let v = serde_json::to_string(storage).unwrap();
    fs::write(format!("{}/result.json", output_dir), v.as_str()).expect("error writing json");

    let html = format!(
        r#"<!doctype html>
        <html>
        <head>
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta charset="UTF-8">
            {styles}
        </head>
        <body>
            <!--[if lt IE 11]>
            <div class="ie-upgrade-container">
                <p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
                <a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
            </div>
            <![endif]-->
            <div id="elm"></div>
            {scripts}
        </body>
        </html>
		"#,
        styles = inline_style(include_str!("../elm-code/styles.css")),
        scripts = inline_script(include_str!("../elm-code/elm.js"))
            + &inline_script(include_str!("../elm-code/app.js")),
    );

    let mut webview = webview::builder()
        .title("Ckaraliolios Data Analysis tool")
        .content(Content::Html(html))
        .size(1600, 900)
        .resizable(true)
        .debug(true)
        .user_data({
            let message_nr = 0;
            let task_done = Task::Init;
            let measurements: Vec<measurement::MeasurementCompact> = storage
                .measurements
                .clone()
                .into_iter()
                .map(|m| m.to_compact())
                .collect();
            let filter_options = FilterOptions::new(&measurements);
            let measurements: Vec<measurement::MeasurementCompact> =
                filter_options.into_filter_query().filter(measurements);
            let filter_options = FilterOptions::new(&measurements);
            let result = ToElm {
                message_nr,
                task_done,
                measurements,
                filter_options,
            };
            //println!("{}", serde_json::to_string_pretty(&result).unwrap());
            result
        })
        .invoke_handler(|webview, arg| {
            use FromElm::*;
            let compact_msmt: Vec<measurement::MeasurementCompact> = storage
                .measurements
                .clone()
                .into_iter()
                .map(|m| m.to_compact())
                .collect();
            let to_elm = webview.user_data_mut();
            if serde_json::from_str::<FromElm>(arg).is_err() {
                println!("{:#?}", arg);
            }
            match serde_json::from_str(arg).unwrap() {
                Init => {
                    *to_elm = {
                        println!("Init {}", to_elm.message_nr);
                        let message_nr = to_elm.message_nr + 1;
                        let task_done = Task::Init;
                        let measurements = compact_msmt.clone();
                        let filter_options = FilterOptions::new(&measurements);
                        let measurements: Vec<measurement::MeasurementCompact> =
                            filter_options.into_filter_query().filter(measurements);
                        ToElm {
                            message_nr,
                            task_done,
                            measurements,
                            filter_options,
                        }
                    }
                }
                Log(string) => println!("{}", string),
                Filter(query) => {
                    *to_elm = {
                        let message_nr = to_elm.message_nr + 1;
                        println!("Filtering");
                        let task_done = Task::Filtering;
                        let measurements = query.filter(compact_msmt.clone());
                        let filter_options = FilterOptions::filtered(&compact_msmt, query);
                        ToElm {
                            message_nr,
                            task_done,
                            measurements,
                            filter_options,
                        }
                    }
                }
                Process(query) => {
                    *to_elm = {
                        println!("Processing");
                        let message_nr = to_elm.message_nr + 1;
                        query.process(
                            storage.measurements.clone(),
                            output_dir.as_str(),
                            script_dir.as_str(),
                        );
                        let task_done = Task::Processing;
                        let measurements = to_elm.measurements.clone();
                        let filter_options = to_elm.filter_options.clone();
                        ToElm {
                            message_nr,
                            task_done,
                            measurements,
                            filter_options,
                        }
                    }
                }
            }

            render(webview)
        })
        .build()
        .unwrap();

    webview.set_color((255, 255, 255));

    let res = webview.run().expect("Error in webview part");

    println!("final state: {:?}", res);
}

fn render(webview: &mut WebView<ToElm>) -> WVResult {
    let render_tasks = {
        let to_elm = webview.user_data();
        format!(
            "app.ports.fromRust.send({})",
            serde_json::to_string(to_elm).unwrap()
        )
    };
    webview.eval(&render_tasks)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "fromrust")]
struct ToElm {
    message_nr: u32,
    task_done: Task,
    measurements: Vec<measurement::MeasurementCompact>,
    filter_options: FilterOptions,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "torust", content = "content")]
pub enum FromElm {
    Init,
    Log(String),
    Filter(FilterQuery),
    Process(ProcessQuery),
}

//{"torust":{"Log":"updated model"}}
#[derive(Debug, Serialize, Deserialize)]
pub enum Task {
    Init,
    Filtering,
    Processing,
}
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
    dates_between: (
        Option<measurement::timestamp::Date>,
        Option<measurement::timestamp::Date>,
    ),
}

impl FilterQuery {
    fn filter(
        &self,
        measurements: Vec<measurement::MeasurementCompact>,
    ) -> Vec<measurement::MeasurementCompact> {
        //filter functions
        let f_sheetname = |measurement: &measurement::MeasurementCompact| {
            self.sheet_names.contains(&measurement.sheet_name)
        };
        let f_width = |measurement: &measurement::MeasurementCompact| {
            self.widths
                .contains(&measurement.device.width.unwrap_or(0.001).to_string())
        };
        let f_length = |measurement: &measurement::MeasurementCompact| {
            self.lengths
                .contains(&measurement.device.length.unwrap_or(0.001).to_string())
        };
        let f_temp = |measurement: &measurement::MeasurementCompact| {
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
        let f_process = |measurement: &measurement::MeasurementCompact| {
            self.wafer == measurement.device.wafer.unwrap().to_string()
        };
        let f_die = |measurement: &measurement::MeasurementCompact| {
            let die = &measurement.device.die.clone().unwrap();
            self.dies.contains(&die)
        };
        let f_testtype = |measurement: &measurement::MeasurementCompact| {
            self.test_type == measurement.test_parameter.test_type.to_string()
        };
        let f_speed = |measurement: &measurement::MeasurementCompact| {
            self.measurement_speeds
                .contains(&measurement.test_parameter.measurement_speed.to_string())
        };
        let f_dates = |measurement: &measurement::MeasurementCompact| {
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

        let result: Vec<measurement::MeasurementCompact> =
            measurements.into_iter().filter(f_sheetname).collect();
        //println!("after sheetname{:#?}",result.len());
        let result: Vec<measurement::MeasurementCompact> =
            result.into_iter().filter(f_width).collect();
        //println!("{:#?}",result.len());
        let result: Vec<measurement::MeasurementCompact> =
            result.into_iter().filter(f_length).collect();
        //println!("after length{:#?}",result.len());
        let result: Vec<measurement::MeasurementCompact> =
            result.into_iter().filter(f_temp).collect();
        //println!("{:#?}",result.len());
        let result: Vec<measurement::MeasurementCompact> =
            result.into_iter().filter(f_process).collect();
        //println!("after process{:#?}",result.len());
        let result: Vec<measurement::MeasurementCompact> =
            result.into_iter().filter(f_die).collect();
        //println!("{:#?}",result.len());
        let result: Vec<measurement::MeasurementCompact> =
            result.into_iter().filter(f_testtype).collect();
        //println!("after testtype{:#?}",result.len());
        let result: Vec<measurement::MeasurementCompact> =
            result.into_iter().filter(f_speed).collect();
        //println!("after speed{:#?}",result.len());
        let result: Vec<measurement::MeasurementCompact> =
            result.into_iter().filter(f_dates).collect();
        //println!("after dates{:#?}",result.len());
        result
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
    fn new(measurements: &Vec<measurement::MeasurementCompact>) -> FilterOptions {
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
        let process_keys: Vec<measurement::device::process::Process> = measurements
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
        let test_type_keys: Vec<&measurement::testparameter::TestType> = measurements
            .into_iter()
            .map(|msmnt| &msmnt.test_parameter.test_type)
            .collect();
        let test_types = test_type_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });
        let measurement_speed_keys: Vec<&measurement::testparameter::MeasurementSpeed> =
            measurements
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
        let date_keys: Vec<measurement::timestamp::Date> = measurements
            .into_iter()
            .map(|msmnt| measurement::timestamp::Date::from(msmnt.test_time_stamp))
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
    fn filtered(
        measurements: &Vec<measurement::MeasurementCompact>,
        filter: FilterQuery,
    ) -> FilterOptions {
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
        let measurement_speed_keys: Vec<measurement::testparameter::MeasurementSpeed> = filteredn
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
        let date_keys: Vec<measurement::timestamp::Date> = filteredn
            .into_iter()
            .map(|msmnt| measurement::timestamp::Date::from(msmnt.test_time_stamp))
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

    fn into_filter_query(&self) -> FilterQuery {
        FilterQuery {
            sheet_names: self.sheet_names.keys().map(|a| a.clone()).collect(),
            widths: self.widths.keys().map(|a| a.clone()).collect(),
            lengths: self.lengths.keys().map(|a| a.clone()).collect(),
            temps: self.temps.keys().map(|a| a.clone()).collect(),
            wafer: self
                .processes
                .keys()
                .map(|a| a.clone())
                .next()
                .unwrap_or(String::from("MINOXG")),
            dies: self.dies.keys().map(|a| a.clone()).collect(),
            test_type: String::from("Sampling"),
            measurement_speeds: self.measurement_speeds.keys().map(|a| a.clone()).collect(),
            dates_between: (None, None),
        }
    }
}

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
    fn from_testdata(id: String, testdata: measurement::testdata::TestData) -> ExportData {
        use measurement::testdata::terminal::Terminal;
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
    fn process(
        &self,
        measurements: Vec<measurement::Measurement>,
        output_dir: &str,
        script_dir: &str,
    ) {
        let selected_measurements: Vec<measurement::Measurement> = {
            let ids: Vec<(String, Vec<measurement::testdata::TestDataCompact>)> = self
                .from
                .clone()
                .into_iter()
                .map(|pd| (pd.id, pd.data))
                .collect();
            let id: Vec<String> = ids.iter().map(|id| id.0.clone()).collect();
            let measurements = measurements.iter().filter(|m| id.clone().contains(&m.id));
            let result: Vec<measurement::Measurement> = measurements
                .map(|m| {
                    let testdatacompact: Vec<measurement::testdata::TestDataCompact> = ids
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
                        .collect::<Vec<measurement::testdata::TestData>>();
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
                        use measurement::testdata::terminal::Terminal;
                        use measurement::testdata::units::Unit;
                        use measurement::testdata::TestData;
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
    data: Vec<measurement::testdata::TestDataCompact>,
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

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
