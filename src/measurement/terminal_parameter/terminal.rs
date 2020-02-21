use crate::calamine_helper::MyRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum Terminal {
    Gate,
    Drain,
    Source,
    Bulk,
}

impl Terminal {
    fn _to_string_concise(&self) -> &str {
        match self {
            Terminal::Bulk => "b",
            Terminal::Drain => "d",
            Terminal::Gate => "g",
            Terminal::Source => "s",
        }
    }
}

impl crate::Extract for Terminal {
    fn extract(column: &MyRange) -> Option<Terminal> {
        let result = match column.it.get((0, 0))?.get_string()? {
            "Gate" => Some(Terminal::Gate),
            "Drain" => Some(Terminal::Drain),
            "Source" => Some(Terminal::Source),
            "Bulk" => Some(Terminal::Bulk),
            _ => None,
        };
        result
    }
}
