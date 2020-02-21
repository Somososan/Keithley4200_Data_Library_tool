use boolinator::Boolinator;
use serde::{Deserialize, Serialize};
use std::iter::FromIterator;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub wafer: Option<process::Process>,
    pub die: Option<String>,
    pub temperature: Option<u32>,
    pub width: Option<f64>,
    pub length: Option<f64>,
}

pub mod process {
    use serde::*;
    #[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, PartialOrd)]
    pub enum Process {
        MINOXG,
        GF22,
    }
    impl std::fmt::Display for Process {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Process::MINOXG => write!(f, "MINOXG"),
                Process::GF22 => write!(f, "GF22"),
            }
        }
    }
}

impl Device {
    fn from_metric(string: String) -> Option<f64> {
        let splice_index = string.rfind(|c: char| c.is_ascii_alphanumeric())?;
        let raw = string.split_at(splice_index - 1);
        let (amount, unit) = (raw.0.parse::<f64>().ok(), raw.1);
        let unit_factor: Option<f64> = match unit {
            "m" => Some(1000000000.0),
            "mm" => Some(1000000.0),
            "um" => Some(1000.0),
            "nm" => Some(1.0),
            "pm" => Some(0.001),
            _ => None,
        };
        amount.and_then(|am| unit_factor.map(|factor| am as f64 * factor as f64))
    }

    pub fn extract(path: String, sheet_name: String) -> Device {
        let mut strings: Vec<String> = path
            .split("\\")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        strings.push(sheet_name);
        strings = strings
            .iter()
            .flat_map(|s| s.split_whitespace())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let process_string: &str = strings
            .iter()
            .filter_map(|string| {
                string
                    .to_ascii_lowercase()
                    .contains("process")
                    .as_some(string)
            })
            .next()
            .unwrap()
            .rsplit("=")
            .next()
            .unwrap_or(" ");
        let wafer = match process_string.to_ascii_lowercase().as_str() {
            "minoxg" => Some(process::Process::MINOXG),
            "gf22" => Some(process::Process::GF22),
            _ => None,
        };
        let die = strings
            .iter()
            .filter_map(|string| string.to_ascii_lowercase().contains("die").as_some(string))
            .next()
            .unwrap()
            .rsplit("=")
            .next()
            .map(|s| s.to_string());
        let temp_string: &str = strings
            .iter()
            .filter_map(|string| string.to_ascii_lowercase().contains("t").as_some(string))
            .next()
            .unwrap()
            .rsplit("=")
            .next()
            .unwrap_or("0");
        let temperature = String::from_iter(
            temp_string
                .chars()
                .filter_map(|c| c.is_numeric().as_some(c)),
        )
        .parse::<u32>()
        .ok();
        let left_of_is =
            |string: String| string.rsplit("=").into_iter().next().unwrap().to_string();
        let w_is_filter =
            |string: String| string.to_ascii_lowercase().contains("w=").as_some(string);
        let mut w_string: Vec<String> = strings
            .iter()
            .map(|s| s.to_string())
            .filter_map(w_is_filter)
            .map(|s| s.to_string())
            .map(left_of_is)
            .collect::<Vec<String>>();
        w_string.dedup();
        let width: Option<f64> = if w_string.len() != 1 {
            None
        } else {
            Device::from_metric(w_string.first().unwrap().to_string())
        };
        let l_is_filter =
            |string: String| string.to_ascii_lowercase().contains("l=").as_some(string);
        let mut l_string: Vec<String> = strings
            .iter()
            .map(|s| s.to_string())
            .filter_map(l_is_filter)
            .map(|s| s.to_string())
            .map(left_of_is)
            .collect::<Vec<String>>();
        l_string.dedup();
        let length: Option<f64> = if l_string.len() != 1 {
            None
        } else {
            Device::from_metric(l_string.first().unwrap().to_string())
        };
        Device {
            wafer,
            die,
            temperature,
            width,
            length,
        }
    }
}
