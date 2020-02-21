use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod terminal;

pub mod units;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestData {
    pub terminal: terminal::Terminal,
    pub unit: units::Unit,
    pub data: Vec<Vec<f64>>,
}

impl TestData {
    pub fn to_compact(&self) -> TestDataCompact {
        TestDataCompact {
            terminal: self.terminal,
            unit: self.unit,
            count: self.data.len(),
        }
    }

    pub fn from_compact(&self, testdatacompact: &Vec<TestDataCompact>) -> Option<TestData> {
        let data: Vec<Vec<f64>> = testdatacompact
            .into_iter()
            .filter(|t| t.terminal == self.terminal && t.unit == self.unit)
            .map(|t| self.data.get(t.count - 1).unwrap_or(&Vec::new()).clone())
            .collect();
        if data.len() > 0 {
            Some(TestData {
                terminal: self.terminal,
                unit: self.unit,
                data,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestDataCompact {
    pub terminal: terminal::Terminal,
    pub unit: units::Unit,
    pub count: usize,
}

impl TestData {
    pub fn extract(sheet: &crate::calamine_helper::MyRange) -> Vec<TestData> {
        use terminal::Terminal;
        use units::Unit;
        let columns = (0..sheet.end().1 + 1)
            .into_iter()
            .map(|i| sheet.sub_range((0, i), (sheet.end().0, i)));

        let hash: &mut HashMap<String, Vec<Vec<f64>>> = &mut HashMap::new();

        for column in columns {
            let data: Vec<f64> = column
                .sub_range((1, 0), (column.end().0, 0))
                .it
                .rows()
                .filter_map(|e| e.into_iter().next().unwrap().get_float())
                .collect();

            let mut header: String = column
                .it
                .get((0, 0))
                .unwrap()
                .get_string()
                .unwrap()
                .to_string();
            if header.contains("(") {
                header = header.split("(").next().unwrap().to_string();
            }

            let header_options = vec![
                "DrainV", "DrainI", "GateV", "GateI", "SourceV", "SourceI", "BulkV", "BulkI",
                "Time",
            ];
            if header_options.contains(&header.as_str()) {
                if hash.contains_key(&header) {
                    hash.entry(header).or_insert(vec![vec![]]).push(data);
                } else {
                    hash.insert(header, vec![data]);
                }
            }
        }

        hash.drain()
            .filter_map(|(header, data)| match header.as_str() {
                "DrainV" => Some(TestData {
                    terminal: Terminal::Drain,
                    unit: units::Unit::Voltage,
                    data,
                }),
                "DrainI" => Some(TestData {
                    terminal: Terminal::Drain,
                    unit: Unit::Current,
                    data,
                }),
                "GateV" => Some(TestData {
                    terminal: Terminal::Gate,
                    unit: Unit::Voltage,
                    data,
                }),
                "GateI" => Some(TestData {
                    terminal: Terminal::Gate,
                    unit: Unit::Current,
                    data,
                }),
                "SourceV" => Some(TestData {
                    terminal: Terminal::Source,
                    unit: Unit::Voltage,
                    data,
                }),
                "SourceI" => Some(TestData {
                    terminal: Terminal::Source,
                    unit: Unit::Current,
                    data,
                }),
                "BulkV" => Some(TestData {
                    terminal: Terminal::Bulk,
                    unit: Unit::Voltage,
                    data,
                }),
                "BulkI" => Some(TestData {
                    terminal: Terminal::Bulk,
                    unit: Unit::Current,
                    data,
                }),
                "Time" => Some(TestData {
                    terminal: Terminal::Time,
                    unit: Unit::Seconds,
                    data,
                }),
                _ => None,
            })
            .collect()
    }
}
