use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Unit {
    Voltage,
    Current,
    Seconds,
}

impl Unit {
    pub fn to_string_concise(&self) -> &str {
        match self {
            Unit::Voltage => "V",
            Unit::Current => "I",
            Unit::Seconds => "T",
        }
    }
}
