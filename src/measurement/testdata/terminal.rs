use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum Terminal {
    Gate,
    Drain,
    Source,
    Bulk,
    Time,
}

impl Terminal {
    pub fn to_string_concise(&self) -> &str {
        match self {
            Terminal::Bulk => "b",
            Terminal::Drain => "d",
            Terminal::Gate => "g",
            Terminal::Source => "s",
            Terminal::Time => "T",
        }
    }
}

impl From<super::super::terminal_parameter::terminal::Terminal> for Terminal {
    fn from(setting_terminal: super::super::terminal_parameter::terminal::Terminal) -> Self {
        use super::super::terminal_parameter::terminal::Terminal as SettingTerminal;
        match setting_terminal {
            SettingTerminal::Bulk => Terminal::Bulk,
            SettingTerminal::Drain => Terminal::Drain,
            SettingTerminal::Gate => Terminal::Gate,
            SettingTerminal::Source => Terminal::Source,
        }
    }
}
