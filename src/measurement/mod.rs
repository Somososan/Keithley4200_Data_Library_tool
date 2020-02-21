use crate::calamine_helper::MyRange;
use crate::Extract;
use calamine::*;
use serde::{Deserialize, Serialize};

pub mod device;

pub mod testparameter;

pub mod timestamp;

pub mod testdata;

pub mod terminal_parameter;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Measurement {
    //file properties
    pub id: String,
    file_path: String,
    pub sheet_name: String,
    //Device Under Test
    pub device: device::Device,
    //Test parameters
    pub test_parameter: testparameter::TestParameter,
    pub test_time_stamp: timestamp::TimeStamp,
    pub terminals: Vec<terminal_parameter::TerminalParameter>,
    //data
    pub test_data: Vec<testdata::TestData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeasurementCompact {
    //file properties
    pub id: String,
    file_path: String,
    pub sheet_name: String,
    //Device Under Test
    pub device: device::Device,
    //Test parameters
    pub test_parameter: testparameter::TestParameter,
    pub test_time_stamp: timestamp::TimeStamp,
    pub terminals: Vec<terminal_parameter::TerminalParameter>,
    //data
    pub test_data: Vec<testdata::TestDataCompact>,
}

impl Measurement {
    pub fn to_compact(&self) -> MeasurementCompact {
        MeasurementCompact {
            id: self.id.clone(),
            file_path: self.file_path.clone(),
            sheet_name: self.sheet_name.clone(),
            device: self.device.clone(),
            test_parameter: self.test_parameter.clone(),
            test_time_stamp: self.test_time_stamp,
            terminals: self.terminals.clone(),
            test_data: self
                .test_data
                .clone()
                .into_iter()
                .map(|t| t.to_compact())
                .collect::<Vec<testdata::TestDataCompact>>(),
        }
    }

    fn extract_origin_positions(sheet: &crate::calamine_helper::MyRange) -> Vec<(usize, usize)> {
        sheet
            .it
            .cells()
            .filter_map(|cell| {
                (if "Test Name" == cell.2.get_string().unwrap_or("") {
                    Some((cell.0 - 3, cell.1))
                } else {
                    None
                })
            })
            .collect()
    }

    fn extract_sheet_name(sheet: &MyRange) -> Option<&str> {
        sheet.it.get((0, 0))?.get_string()
    }

    pub fn extract(root: String, relative_path: String, storage: &mut crate::database::Database) {
        let path = format!("{}{}", root.as_str(), relative_path.as_str());
        if !storage.files_scanned_before.contains(&relative_path) {
            storage.files_scanned_before.push(relative_path.clone());
            let mut workbook: Xls<_> = open_workbook(&path).expect("cannot open file");
            let sett = MyRange::new(
                workbook
                    .worksheet_range("Settings")
                    .expect("Cannot find 'Settings'")
                    .unwrap(),
            );
            let sheet = sett.sub_range((1, 0), sett.end());
            let positions_of_test_name: Vec<(usize, usize)> =
                Measurement::extract_origin_positions(&sheet);
            let mut subranges: Vec<MyRange> = positions_of_test_name
                .iter()
                .zip(positions_of_test_name.iter().skip(1))
                .map(|((row, _), (row_next, _))| {
                    sheet.sub_range((*row, 0), (row_next - 1, sheet.end().1))
                })
                .collect();
            subranges.push(sheet.sub_range(*positions_of_test_name.last().unwrap(), sheet.end()));
            for run_setting in subranges.iter() {
                let sheet_name: String = Measurement::extract_sheet_name(run_setting)
                    .expect("Sheet name extraction failure")
                    .to_string();
                let data_sheet = MyRange::new(
                    workbook
                        .worksheet_range(sheet_name.as_str())
                        .expect(
                            format!("Can not find sheet with the following name: {}", sheet_name)
                                .as_str(),
                        )
                        .unwrap(),
                );
                let test_time_stamp =
                    timestamp::TimeStamp::extract(&run_setting).expect("Error parsing timestamp");
                let id = storage.generate_id(test_time_stamp);

                let test_parameter: testparameter::TestParameter =
                    testparameter::TestParameter::extract(&run_setting)
                        .expect("Test type extraction failure");
                let device: device::Device =
                    device::Device::extract(relative_path.clone(), sheet_name.clone());
                let terminals = (1..(run_setting.end().1 + 1))
                    .into_iter()
                    .map(|i| run_setting.sub_range((14, i), (run_setting.end().0, i)))
                    .map(|c| {
                        terminal_parameter::TerminalParameter::extract(
                            &c,
                            &test_parameter.test_type,
                        )
                        .expect("Terminal Parameter extraction failure")
                    })
                    .collect();
                let test_data = testdata::TestData::extract(&data_sheet);
                storage.measurements.push(Measurement {
                    id,
                    file_path: relative_path.clone(),
                    sheet_name,
                    device,
                    test_parameter,
                    test_time_stamp,
                    terminals,
                    test_data,
                })
            }
        }
    }
}
