use webview::*;
use serde::*;
use std::path::Path;
use std::collections::HashMap;
//use serde_json::*;
use calamine::*;
use strum_macros::AsRefStr;
use clap::{Arg, App};
use std::fs::{self};
use std::iter::FromIterator;
use boolinator::*;


#[derive(Debug,Serialize,Deserialize,Clone)]
struct Database {
    files_scanned_before    : Vec<String>,
    id_day_counter          : HashMap<String,u32>,
    measurements            : Vec<Measurement>
}

impl Database{
    fn new () ->Database{
        Database {files_scanned_before:vec!(), id_day_counter : HashMap::new(), measurements : vec!()}
    }
    
    fn generate_id(&mut self, time_stamp : TimeStamp) -> String {
        let string = format!("{:0>4}{:0>2}{:0>2}", time_stamp.year, time_stamp.month, time_stamp.day);
        let counter = self.id_day_counter.entry(string.clone()).or_insert(0);
        *counter += 1;

        format!("{}:{}", string.as_str(), counter)
    }
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Measurement {
    //file properties
    id              : String,
    file_path       : String,
    sheet_name      : String,
    //Device Under Test
    device          : Device,
    //Test parameters
    test_parameter  : TestParameter,
    test_time_stamp : TimeStamp,
    terminals       : Vec<TerminalParameter>,
    //data
    test_data       : Vec<TestData>
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct MeasurementCompact {
    //file properties
    id              : String,
    file_path       : String,
    sheet_name      : String,
    //Device Under Test
    device          : Device,
    //Test parameters
    test_parameter  : TestParameter,
    test_time_stamp : TimeStamp,
    terminals       : Vec<TerminalParameter>,
    //data
    test_data       : Vec<TestDataCompact>
}

impl Measurement {
    fn to_compact(&self) -> MeasurementCompact{
        MeasurementCompact {id:self.id.clone(), file_path:self.file_path.clone(), sheet_name:self.sheet_name.clone(), device : self.device.clone(), test_parameter:self.test_parameter.clone(), test_time_stamp:self.test_time_stamp, terminals:self.terminals.clone(), test_data: self.test_data.clone().into_iter().map(|t| t.to_compact()).collect::<Vec<TestDataCompact>>() }
    }

    fn extract_origin_position(sheet : &MyRange) -> Vec<(usize,usize)>{
        sheet.it.cells().filter_map(|cell| (
            if "Test Name" == cell.2.get_string().unwrap_or("") {
                Some((cell.0-3,cell.1))
            } else {
                None
            }
        ) ).collect()
    }

    fn translate_test_time_stamp(raw_string : &str) -> TimeStamp{
        let mut split_string = raw_string.split_ascii_whitespace();
        let mut date = split_string.next().unwrap().split('/');
        //keep this order for the Yankees
        let month   : u8  = date.next().unwrap_or("0").parse().expect("error parsing time stamp");
        let day     : u8  = date.next().unwrap_or("0").parse().expect("error parsing time stamp");
        let year    : u16 = date.next().unwrap_or("0").parse().expect("error parsing time stamp");
        
        let mut time = split_string.next().unwrap().split(':');
        let hour    : u8  = time.next().unwrap_or("0").parse().expect("error parsing time stamp");
        let minute    : u8  = time.next().unwrap_or("0").parse().expect("error parsing time stamp");
        let second    : u8  = time.next().unwrap_or("0").parse().expect("error parsing time stamp");
        
        TimeStamp{year,month,day,hour,minute,second}
    }
    
    fn extract_sheet_name (sheet: &MyRange) -> Option<&str>{
        sheet.it.get((0,0))?.get_string()
    }

    fn extract_time_stamp (sheet: &MyRange) -> Option<&str>{
        sheet.it.get((9,1))?.get_string()
    }

    fn extract_test_mode (sheet: &MyRange) -> Option<TestParameter>{
        match sheet.it.get((4,1))?.get_string()? {
            "Sweeping" => {
                let test_type = TestType::Sweeping;
                let measurement_speed = match sheet.it.get((5,1))?.get_string()? {
                        "Fast" => MeasurementSpeed::Fast,
                        "Normal" => MeasurementSpeed::Normal,
                        "Quiet" => MeasurementSpeed::Quiet,
                        _ => MeasurementSpeed::Custom
                    };
                let ad_aperture = None;
                let filter_factor = None;
                let interval_time = None;
                let sweep_delay_time = sheet.it.get((6,1))?.get_string()?.parse::<f64>().ok();
                let hold_time = sheet.it.get((7,1))?.get_string()?.parse::<f64>().ok()?;
                
                Some(TestParameter {test_type,measurement_speed,ad_aperture,filter_factor,interval_time,sweep_delay_time,hold_time })
            },
            "Sampling" => {
                let test_type = TestType::Sampling;
                let measurement_speed = match sheet.it.get((5,1))?.get_string()? {
                        "Fast" => MeasurementSpeed::Fast,
                        "Normal" => MeasurementSpeed::Normal,
                        "Quiet" => MeasurementSpeed::Quiet,
                        _ => MeasurementSpeed::Custom
                    };
                let ad_aperture = None;
                let filter_factor = None;
                let interval_time = sheet.it.get((6,1))?.get_string()?.parse::<f64>().ok();
                let sweep_delay_time = None;
                let hold_time = sheet.it.get((7,1))?.get_string()?.parse::<f64>().ok()?;
                
                Some(TestParameter {test_type, measurement_speed, ad_aperture, filter_factor, interval_time, sweep_delay_time,hold_time })
            },
            _ => None
        }
    }

    fn from_metric(string:String) -> Option<f64> {
        let splice_index = string.rfind(|c:char| c.is_ascii_alphanumeric() )?;
        let raw = string.split_at(splice_index-1);
        let (amount,unit) = (raw.0.parse::<f64>().ok(),  raw.1);
        let unit_factor :Option<f64>= match unit {
            "m"     => Some(1000000000.0),
            "mm"    => Some(1000000.0),
            "um"    => Some(1000.0),
            "nm"    => Some(1.0),
            "pm"    => Some(0.001),
            _       => None
        };
        amount.and_then(|am| unit_factor.map(|factor| am as f64*factor as f64) )
    }

    fn extract_device (path: String, sheet_name:String) -> Device {
        let mut strings : Vec<String> = path.split("\\").map(|s| s.to_string()).collect::<Vec<String>>();
        strings.push(sheet_name);
        strings = strings.iter().flat_map(|s| s.split_whitespace()).map(|s| s.to_string()).collect::<Vec<String>>();
        let process_string :&str= strings.iter().filter_map(|string| string.to_ascii_lowercase().contains("process").as_some(string) ).next().unwrap().rsplit("=").next().unwrap_or(" ");
        let wafer = match process_string.to_ascii_lowercase().as_str() {
            "minoxg"    => Some(Process::MINOXG),
            "gf22"      => Some(Process::GF22),
            _           => None
        };
        
        let die_string :&str= strings.iter().filter_map(|string| string.to_ascii_lowercase().contains("die").as_some(string)).next().unwrap().rsplit("=").next().unwrap_or("");
        let die :Option<(String,u32)>= Some( (String::from_iter(die_string.chars().filter_map(|c:char| (!c.is_ascii_digit()).as_some(c) ) ) , String::from_iter(die_string.chars().filter_map(|c| c.is_numeric().as_some(c) ) ).parse::<u32>().ok().unwrap() ) );
        
        let temp_string :&str= strings.iter().filter_map(|string| string.to_ascii_lowercase().contains("t").as_some(string)).next().unwrap().rsplit("=").next().unwrap_or("0");
        let temperature =   String::from_iter(temp_string.chars().filter_map(|c| c.is_numeric().as_some(c) ) ).parse::<u32>().ok() ;
        //println!("{:#?}", temp_string);

        let left_of_is = |string:String| string.rsplit("=").into_iter().next().unwrap().to_string();
        let w_is_filter = |string :String| string.to_ascii_lowercase().contains("w=").as_some(string) ;
        
        let mut w_string :Vec<String> = strings.iter().map(|s| s.to_string()).filter_map(w_is_filter).map(|s| s.to_string()).map(left_of_is).collect::<Vec<String>>();
        w_string.dedup();
        //println!("{:#?}", strings);
        
        let width :Option<f64> = if w_string.len()!=1 {
            None
        } else {
            Measurement::from_metric(w_string.first().unwrap().to_string())
        };
        
        
        let l_is_filter = |string :String| string.to_ascii_lowercase().contains("l=").as_some(string) ;
        
        let mut l_string :Vec<String> = strings.iter().map(|s| s.to_string()).filter_map(l_is_filter).map(|s| s.to_string()).map(left_of_is).collect::<Vec<String>>();
        l_string.dedup();

        let length :Option<f64> = if l_string.len()!=1 {
            None
            } else {
                Measurement::from_metric(l_string.first().unwrap().to_string())
            };

        
        Device{wafer,die,temperature,width,length}
    }


    fn extract_terminal_parameter (sheet: &MyRange, test_type : &TestType) -> Option<TerminalParameter> {
        let terminal = TerminalParameter::extract_terminal_type(sheet)?;
        let instrument = TerminalParameter::extract_instrument(sheet)?;
        let operational_mode = TerminalParameter::extract_opmode(sheet, test_type)?;
        let compliance = TerminalParameter::extract_compliance(sheet, test_type);
        let voltage_range = TerminalParameter::extract_voltage_range(sheet,test_type);
        let current_range = TerminalParameter::extract_current_range(sheet,test_type);
        let voltage = TerminalParameter::extract_voltage(sheet, test_type);
        let current = TerminalParameter::extract_current(sheet, test_type);

        Some(TerminalParameter {terminal, instrument, operational_mode, compliance, voltage, voltage_range, current, current_range})
    }

    fn extract_test_data (sheet: &MyRange) -> Vec<TestData> {
        let columns = (0..sheet.end().1+1).into_iter().map(|i| sheet.sub_range( (0, i), (sheet.end().0, i) ));
              
        let  hash :&mut HashMap<String, Vec<Vec<f64>>>= &mut HashMap::new();
        
        for column in columns{
            let data :Vec<f64>= column.sub_range((1,0),(column.end().0,0)).it.rows().filter_map(|e| e.into_iter().next().unwrap().get_float() ).collect();
            
            let mut header :String= column.it.get((0,0)).unwrap().get_string().unwrap().to_string();
            if header.contains("("){
                header = header.split("(").next().unwrap().to_string();
            }
            
            let header_options = vec!["DrainV","DrainI", "GateV","GateI","SourceV","SourceI","BulkV","BulkI","Time"];
            if header_options.contains(&header.as_str()){
                if hash.contains_key(&header){
                    hash.entry(header).or_insert(vec!(vec!())).push(data);
                } else {
                    hash.insert(header, vec!(data) );
                }
            }
        } 

        hash.drain().filter_map(|(header,data)| {match header.as_str() {
            "DrainV"    => Some(TestData {terminal: Terminal::Drain, unit: Unit::Voltage, data}),
            "DrainI"    => Some(TestData {terminal: Terminal::Drain, unit: Unit::Current, data}),
            "GateV"     => Some(TestData {terminal: Terminal::Gate, unit: Unit::Voltage, data}),
            "GateI"     => Some(TestData {terminal: Terminal::Gate, unit: Unit::Current, data}),
            "SourceV"   => Some(TestData {terminal: Terminal::Source, unit: Unit::Voltage, data}),
            "SourceI"   => Some(TestData {terminal: Terminal::Source, unit: Unit::Current, data}),
            "BulkV"     => Some(TestData {terminal: Terminal::Bulk, unit: Unit::Voltage, data}),
            "BulkI"     => Some(TestData {terminal: Terminal::Bulk, unit: Unit::Current, data}),
            "Time"      => Some(TestData {terminal: Terminal::Time, unit: Unit::Seconds, data}),
            _           => None
        }}).collect()
    }
}


#[derive(Debug,Serialize,Deserialize,Clone)]
struct Device {
    wafer           : Option<Process>,
    die             : Option<(String, u32)>,
    temperature     : Option<u32>,
    width           : Option<f64>,
    length          : Option<f64>
}


#[derive(Debug,Serialize,Deserialize,Copy,Clone, Eq, PartialEq,PartialOrd)]
pub enum Process {
    MINOXG,
    GF22
}
impl std::fmt::Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Process::MINOXG =>  write!(f, "MINOXG"),
            Process::GF22   =>  write!(f, "GF22")
        }
    }
}


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct TestParameter{
    pub test_type : TestType,
    pub measurement_speed : MeasurementSpeed,
    pub ad_aperture : Option <f64>,
    pub filter_factor : Option <f64>,
    pub interval_time : Option <f64>,
    pub sweep_delay_time : Option<f64>,
    pub hold_time : f64
}


#[derive(Debug,Serialize,Deserialize,Clone,PartialEq)]
pub enum TestType {
    Sampling,
    Sweeping
}
impl std::fmt::Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestType::Sampling => write!(f, "Sampling"),
            TestType::Sweeping => write!(f, "Sweeping")
        }
    }
}


#[derive(Debug,Serialize,Deserialize,Clone,PartialEq)]
pub enum MeasurementSpeed {
    Fast,
    Normal,
    Quiet,
    Custom
}
impl std::fmt::Display for MeasurementSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MeasurementSpeed::Fast =>   write!(f, "Fast"),
            MeasurementSpeed::Normal =>   write!(f, "Normal"),
            MeasurementSpeed::Quiet =>   write!(f, "Quiet"),
            MeasurementSpeed::Custom =>   write!(f, "Custom"),  
        }
    }
}


#[derive(Debug,Serialize,Deserialize,Copy,Clone, Eq, PartialEq, Hash)]
pub struct TimeStamp {
    year    : u16,
    month   : u8,
    day     : u8,
    hour    : u8,
    minute  : u8,
    second  : u8
}

impl TimeStamp {
    fn to_date(&self) -> Date {
        Date {year:self.year,month:self.month,day:self.day}
    }
}


#[derive(Debug,Serialize,Deserialize,Copy,Clone, Eq, PartialEq, Hash)]
pub struct Date {
    year    : u16,
    month   : u8,
    day     : u8
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.year,self.month,self.day)        
    }
}


#[derive(Debug,Serialize,Deserialize,Clone)]
struct TerminalParameter {
    terminal        : Terminal,
    instrument      : Instrument,
    operational_mode: OpMode,
    compliance      : Option<f64>, //current limit of the terminal
    voltage         : Option<UnitMeasured>,
    voltage_range   : Option<VRange>,
    current         : Option<UnitMeasured>,
    current_range   : Option<CRange>
}

impl TerminalParameter {
    fn extract_terminal_type (column: &MyRange) -> Option<Terminal>{
        let result = match column.it.get((0,0))?.get_string()?{
            "Gate" => Some(Terminal::Gate),
            "Drain" => Some(Terminal::Drain),
            "Source" => Some(Terminal::Source),
            "Bulk" => Some(Terminal::Bulk),
            _ => None
        };
        result
    }

    fn extract_instrument (column: &MyRange) -> Option<Instrument>{
        match column.it.get((1,0))?.get_string()?{
            "SMU1" => Some(Instrument::SMU1),
            "SMU2" => Some(Instrument::SMU2),
            "SMU3" => Some(Instrument::SMU3),
            "SMU4" => Some(Instrument::SMU4),
            "GNDU" => Some(Instrument::GNDU),
            "PMU1" => Some(Instrument::PMU1),
            "PMU2" => Some(Instrument::PMU2),
            "PMU3" => Some(Instrument::PMU3),
            "PMU4" => Some(Instrument::PMU4),
            _ => None
        }
    }

    fn extract_opmode (column : &MyRange, test_type : &TestType) -> Option<OpMode> {
        match test_type{
            TestType::Sweeping => {
                match column.it.get((3,0))?.get_string()?{
                    "Voltage Bias" => {
                        let op_type = OpModeType::VoltageBias;
                        let bias = column.it.get((5,0))?.get_string()?.parse::<f64>().ok();
                        let start = None;
                        let stop = None;
                        let stepsize = None;
                        Some(OpMode {op_type,bias,start,stop,stepsize})
                    },
                    "Current Bias" => {
                        let op_type = OpModeType::CurrentBias;
                        let bias = column.it.get((5,0))?.get_string()?.parse::<f64>().ok();
                        let start = None;
                        let stop = None;
                        let stepsize = None;
                        Some(OpMode {op_type,bias,start,stop,stepsize})
                    },
                    "Voltage Linear Sweep" => {
                        let op_type = OpModeType::VoltageLinearSweep;
                        let bias = None;
                        let start = column.it.get((5,0))?.get_string()?.parse::<f64>().ok();
                        let stop  = column.it.get((6,0))?.get_string()?.parse::<f64>().ok();
                        let stepsize  = column.it.get((7,0))?.get_string()?.parse::<f64>().ok();
                        Some(OpMode {op_type,bias,start,stop,stepsize})

                    },
                    "Current Linear Sweep" => {
                        let op_type = OpModeType::CurrentLinearSweep;
                        let bias = None;
                        let start = column.it.get((5,0))?.get_string()?.parse::<f64>().ok();
                        let stop  = column.it.get((6,0))?.get_string()?.parse::<f64>().ok();
                        let stepsize  = column.it.get((7,0))?.get_string()?.parse::<f64>().ok();
                        Some(OpMode {op_type,bias,start,stop,stepsize})
                    },
                    "Voltage Step" => {
                        let op_type = OpModeType::VoltageStep;
                        let bias = None;
                        let start = column.it.get((5,0))?.get_string()?.parse::<f64>().ok();
                        let stop  = column.it.get((6,0))?.get_string()?.parse::<f64>().ok();
                        let stepsize  = column.it.get((7,0))?.get_string()?.parse::<f64>().ok();
                        Some(OpMode {op_type,bias,start,stop,stepsize})
                    },
                    "Current Step" => {
                        let op_type = OpModeType::CurrentStep;
                        let bias = None;
                        let start = column.it.get((5,0))?.get_string()?.parse::<f64>().ok();
                        let stop  = column.it.get((6,0))?.get_string()?.parse::<f64>().ok();
                        let stepsize  = column.it.get((7,0))?.get_string()?.parse::<f64>().ok();
                        Some(OpMode {op_type,bias,start,stop,stepsize})
                    },
                    "Common" => {
                        let op_type = OpModeType::Common;
                        let bias = None;
                        let start = None;
                        let stop  = None;
                        let stepsize  = None;
                        Some(OpMode {op_type,bias,start,stop,stepsize})
                    },
                    "Floating" => {
                        let op_type = OpModeType::Floating;
                        let bias = None;
                        let start = None;
                        let stop  = None;
                        let stepsize  = None;
                        Some(OpMode {op_type,bias,start,stop,stepsize})
                    },
                    _ => None
                }
            },
            TestType::Sampling => {
                match column.it.get((3,0))?.get_string()?{
                    "Voltage Bias" => {
                        let op_type = OpModeType::VoltageBias;
                        let bias = column.it.get((4,0))?.get_string()?.parse::<f64>().ok();
                        let start = None;
                        let stop = None;
                        let stepsize = None;
                        Some(OpMode {op_type,bias,start,stop,stepsize})
                    },
                    "Current Bias" => {
                        let op_type = OpModeType::CurrentBias;
                        let bias = column.it.get((4,0))?.get_string()?.parse::<f64>().ok();
                        let start = None;
                        let stop = None;
                        let stepsize = None;
                        Some(OpMode {op_type,bias,start,stop,stepsize})
                    },
                    "Common" => {
                        let op_type = OpModeType::Common;
                        let bias = None;
                        let start = None;
                        let stop  = None;
                        let stepsize  = None;
                        Some(OpMode {op_type,bias,start,stop,stepsize})
                    },
                    "Floating" => {
                        let op_type = OpModeType::Floating;
                        let bias = None;
                        let start = None;
                        let stop  = None;
                        let stepsize  = None;
                        Some(OpMode {op_type,bias,start,stop,stepsize})
                    },
                    _ => None
                }
            }

        }
    }

    fn extract_compliance (column : &MyRange, test_type : &TestType) -> Option<f64>  {
        match test_type {
            TestType::Sweeping =>  column.it.get((9,0))?.get_string()?.parse::<f64>().ok(),
            TestType::Sampling =>  column.it.get((6,0))?.get_string()?.parse::<f64>().ok()
        }
    }

    fn extract_voltage_range (column : &MyRange, test_type : &TestType) -> Option<VRange>  {
        let at_offset = |offset| match column.it.get((offset,0))?.get_string(){
            Some("Best Fixed") => Some(VRange::BestFixed),
            _ => None
        };
    
        match test_type{
            TestType::Sweeping => at_offset(13),
            TestType::Sampling => at_offset(10)
        }
    }

    fn extract_current_range (column : &MyRange, test_type : &TestType) -> Option<CRange> {
        let at_offset= |offset|->Option<CRange> {
            let string = column.it.get((offset,0))?.get_string()?;
            if string.starts_with("Limited") {
                Some(CRange::LimitedAuto( string.rsplit("=").next()?.to_string() ) )
            } else if string == "Auto"{
                Some(CRange::Auto)
            } else{
                None
            }
        };
        match test_type{
            TestType::Sweeping => at_offset(12),
            TestType::Sampling => at_offset(9)
        }
    }

    fn extract_voltage (column : &MyRange, test_type : &TestType) -> Option<UnitMeasured> {
        let extract_at = | offset| {
            match column.it.get((offset,0))?.get_string()?{
                "Measured" => Some( UnitMeasured::Measured ),
                "Programmed" => Some( UnitMeasured::Programmed ),
                _ => None
            }
        };
        
        match test_type{
            TestType::Sweeping => extract_at(11),
            TestType::Sampling => extract_at(8)
        }
    }

    fn extract_current (column : &MyRange, test_type : &TestType) -> Option<UnitMeasured> {
        let extract_at = | offset| {
            match column.it.get((offset,0))?.get_string()?{
                "Measured" => Some( UnitMeasured::Measured ),
                "Programmed" => Some( UnitMeasured::Programmed ),
                _ => None
            }
        };
        
        match test_type{
            TestType::Sweeping => extract_at(10),
            TestType::Sampling => extract_at(7)
        }
    }
}


#[derive(AsRefStr,Debug,Serialize,Deserialize,Copy,Clone,PartialEq)]
pub enum Terminal{
    Gate,
    Drain,
    Source,
    Bulk,
    Time
}

impl Terminal{
    fn to_string_concise(&self) -> &str {
        match self {
            Terminal::Bulk      => "b",
            Terminal::Drain     => "d",
            Terminal::Gate      => "g",
            Terminal::Source    => "s",
            Terminal::Time      => "T"
        }
    }
}

#[derive(AsRefStr,Debug,Serialize,Deserialize,Copy,Clone)]
pub enum Instrument {
    SMU1,
    SMU2,
    SMU3,
    SMU4,
    GNDU,
    PMU1,
    PMU2,
    PMU3,
    PMU4
}


#[derive(Debug,Serialize,Deserialize,Copy,Clone)]
pub struct OpMode {
    op_type : OpModeType,
    bias : Option<f64>,
    start : Option<f64>,
    stop : Option<f64>,
    stepsize : Option<f64>
}

#[derive(Debug,Serialize,Deserialize,Copy,Clone)]
pub enum OpModeType {
    VoltageBias,
    VoltageLinearSweep, // (start,stop,stepsize)
    VoltageStep, // (start,stop,stepsize)
    CurrentBias,
    CurrentLinearSweep, // (start,stop,stepsize)
    CurrentStep, // (start,stop,stepsize)
    Common,
    Floating
}


#[derive(Debug,Serialize,Deserialize,Copy,Clone)]
enum VRange {
    BestFixed
}


#[derive(Debug,Serialize,Deserialize,Copy,Clone)]
pub enum UnitMeasured {
    Measured,
    Programmed
}

#[derive(Debug,Serialize,Deserialize,Clone)]
enum CRange {
    LimitedAuto(String),
    Auto
}
#[derive(Debug,Serialize,Deserialize,Clone,Copy,PartialEq)]
pub enum Unit {
    Voltage,
    Current,
    Seconds
}

impl Unit{
   fn to_string_concise(&self) -> &str{
        match self {
            Unit::Voltage => "V",
            Unit::Current => "I",
            Unit::Seconds => "T"
        }
    }
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct TestData {
    terminal : Terminal,
    unit : Unit,
    data : Vec<Vec<f64> >
}

impl TestData{
    fn to_compact(&self) -> TestDataCompact{
        TestDataCompact {terminal: self.terminal, unit: self.unit, count : self.data.len()}
    }
    
    fn from_compact(&self, testdatacompact : &Vec<TestDataCompact>) -> Option<TestData>{
        let data : Vec<Vec<f64> >= testdatacompact.into_iter().map(|t|  self.data.get(t.count-1).unwrap_or(&Vec::new() ).clone() ).collect();
        if data.len()>0 {
            Some(TestData {terminal: self.terminal, unit: self.unit, data})
        } else {
            None
        }
    } 
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct TestDataCompact {
    terminal : Terminal,
    unit : Unit,
    count : usize
}

//calamine helper struct
#[derive(Debug)]
struct MyRange {
    it : Range<DataType>,
    abs_pos : (u32,u32)
}

impl MyRange {
    fn new(range:Range<DataType>) -> MyRange{
        MyRange {it:range,abs_pos:(0,0)}
    }

    fn sub_range(&self,start : (usize,usize), end : (usize,usize)) -> MyRange {
        let abs_pos = (self.abs_pos.0 + start.0 as u32, self.abs_pos.1 + start.1 as u32);
        assert!(end<=self.end());
        MyRange {it:  self.it.range(abs_pos, (self.abs_pos.0 + end.0 as u32, self.abs_pos.1 + end.1 as u32)),abs_pos}
    }

    fn end(&self) -> (usize,usize){
        (self.it.end().unwrap().0 as usize, self.it.end().unwrap().1 as usize) 
    }
}

fn extract_measurements(root: String, relative_path: String, storage: &mut Database){
    let path = format!("{}{}", root.as_str(), relative_path.as_str() );
    if !storage.files_scanned_before.contains( &relative_path ){
        storage.files_scanned_before.push(relative_path.clone());

        let mut workbook :Xls<_>= open_workbook(&path).expect("cannot open file");
        let sett = MyRange::new(workbook.worksheet_range("Settings").expect("Cannot find 'Settings'").unwrap());
        
        let sheet = sett.sub_range((1,0), sett.end() );
        let position_of_test_name :Vec<(usize,usize)>= Measurement::extract_origin_position(&sheet);
        let mut subranges :Vec<MyRange> = position_of_test_name.iter().zip(position_of_test_name.iter().skip(1)).map(|((row, _),(row_next,_))|  sheet.sub_range( (*row, 0), (row_next-1,sheet.end().1) ) ).collect(); 
        subranges.push(sheet.sub_range(*position_of_test_name.last().unwrap(), sheet.end() ));

        for run_setting in subranges.iter(){
            let sheet_name :String= Measurement::extract_sheet_name(run_setting).expect("Sheet name extraction failure").to_string();
            let data_sheet = MyRange {it:workbook.worksheet_range(sheet_name.as_str()).expect(format!("Can not find sheet with the following name: {}",sheet_name).as_str()).unwrap() , abs_pos:(0,0)};
            
            let test_time_stamp = Measurement::translate_test_time_stamp(Measurement::extract_time_stamp(run_setting).expect("Time stamp extraction failure"));
            
            let id = storage.generate_id(test_time_stamp);

            let test_parameter = Measurement::extract_test_mode(run_setting).expect("Test type extraction failure");
            
            let device :Device= Measurement::extract_device(relative_path.clone(), sheet_name.clone());
            
            let columns = (1..(run_setting.end().1 +1)).into_iter().map(|i| run_setting.sub_range( (14, i), (run_setting.end().0, i) ));

            let terminals = columns.map(|c| Measurement::extract_terminal_parameter(&c,&test_parameter.test_type).expect("Terminal Parameter extraction failure")).collect();
            
            let test_data = Measurement::extract_test_data(&data_sheet);
            
            storage.measurements.push(Measurement{id,file_path : relative_path.clone(), sheet_name, device, test_parameter, test_time_stamp, terminals ,test_data})
        }
        
    }
}
fn populate_from_path(root: String, relative_dir: String, storage: &mut Database) -> std::io::Result<()> {
    let dir_string = format!("{}{}",root.clone(),relative_dir);
    
    let dir = Path::new( dir_string.as_str() );

    if dir.is_dir() {
        for entry in fs::read_dir(dir)?{
            let entry = entry?;
            let path = entry.path();
            let relative_path :String= path.to_str().unwrap().replace(root.clone().as_str(),""); 
            if path.is_dir() {
                populate_from_path( root.clone(), relative_path, storage)?;
                
            } else if path.extension().unwrap() == "xls" {
                extract_measurements(root.clone(),relative_path, storage);
            } else{}
        }
    }
 Ok(())
}



fn main() {
    let matches = App::new("Library tool")
        .version("0.1")
        .author("C Karaliolios")
        .about("Processes Keithley 4200 parameter analyzer data and generates images and a interface")
        .arg(Arg::with_name("input_directory")
            .short("i")
            .long("input_dir")
            .value_name("PATH")
            .help("Sets the directory to be searched"))
        .arg(Arg::with_name("output_directory")
            .short("o")
            .long("output_dir")
            .value_name("PATH")
            .help("Sets the directory for the outputs to be collected in"))
        .arg(Arg::with_name("script_directory")
            .short("s")
            .long("script_dir")
            .value_name("PATH")
            .help("Sets the directory where the python scripts selected out of"))    
        .get_matches();

    //default input directory
    let input_string : String = format!("{}/tests", env!("CARGO_MANIFEST_DIR") );
    // get input directory from CLI
    let input_dir = matches.value_of("input_directory").unwrap_or(input_string.as_str()).to_string();

    //default output directory
    let output_string : String = format!("{}/output", env!("CARGO_MANIFEST_DIR") );
    // get output directory from CLI
    let output_dir = matches.value_of("output_directory").unwrap_or(output_string.as_str()).to_string();

    //default script directory
    let script_string : String = format!("{}/scripts", env!("CARGO_MANIFEST_DIR") );
    // get input directory from CLI
    let script_dir = matches.value_of("script_directory").unwrap_or(script_string.as_str()).to_string();

    
    //initialize id and measurement vector
    let json = fs::read_to_string(format!("{}/result.json",output_dir ) );
    let storage :&mut Database = &mut Database::new();
    match json {
         Ok(string)  => *storage= serde_json::from_str::<Database>(string.as_str() ).unwrap_or(Database::new()),
         _ => ()
    };
    populate_from_path(input_dir,String::from(""), storage).expect("Error transfercing path");
 
    let v = serde_json::to_string(storage).unwrap();
    fs::write( format!("{}/result.json",output_dir ), v.as_str() ).expect("error writing json");
 
   
    let html = format!(r#"<!doctype html>
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
		styles = inline_style( include_str!("../elm-code/styles.css") ),
		scripts = inline_script(include_str!("../elm-code/elm.js")) + &inline_script(include_str!("../elm-code/app.js")),
	);

    let mut webview = webview::builder()
        .title("Ckaraliolios Data Analysis tool")
        .content(Content::Html(html))
        .size(1600,900)
        .resizable(true)
        .debug(true)
        .user_data({
            let message_nr = 0;
            let task_done = Task::Init;
            let measurements : Vec<MeasurementCompact> = storage.measurements.clone().into_iter().map(|m| m.to_compact()).collect();
            let filter_options = FilterOptions::new(&measurements);
            let measurements :Vec<MeasurementCompact> = filter_options.into_filter_query().filter(measurements);
            let filter_options = FilterOptions::new(&measurements);
            let result =ToElm {message_nr,task_done,measurements,filter_options};
            //println!("{}", serde_json::to_string_pretty(&result).unwrap());
            result

        })
        .invoke_handler(|webview, arg| {
            
            use FromElm::*;
            let compact_msmt :Vec<MeasurementCompact>= storage.measurements.clone().into_iter().map(|m| m.to_compact()).collect();
            let to_elm = webview.user_data_mut();
            if  serde_json::from_str::<FromElm>(arg).is_err()  {
                println!("{:#?}", arg);
            } 
            match serde_json::from_str(arg).unwrap() {
                Init => *to_elm = {
                    println!("Init {}",to_elm.message_nr);
                    let message_nr = to_elm.message_nr +1 ;
                    let task_done = Task::Init;
                    let measurements = compact_msmt.clone();
                    let filter_options = FilterOptions::new(&measurements);
                    let measurements :Vec<MeasurementCompact> = filter_options.into_filter_query().filter(measurements);
                    ToElm {message_nr,task_done,measurements,filter_options}
                },
                Log (string) => println!("{}", string), 
                Filter (query) => *to_elm = {
                    let message_nr = to_elm.message_nr +1 ;
                    println!("Filtering");
                    let task_done = Task::Filtering;
                    let measurements = query.filter(compact_msmt.clone());
                    let filter_options = FilterOptions::filtered(&compact_msmt,query);
                    ToElm {message_nr,task_done,measurements,filter_options}
                },
                Process (query) => *to_elm = {
                    println!("Processing");
                    let message_nr = to_elm.message_nr +1 ;
                    query.process(storage.measurements.clone(), output_dir.as_str(), script_dir.as_str() );
                    let task_done = Task::Processing;
                    let measurements = to_elm.measurements.clone();
                    let filter_options = to_elm.filter_options.clone();
                    ToElm {message_nr,task_done,measurements,filter_options}
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
        format!("app.ports.fromRust.send({})", serde_json::to_string(to_elm).unwrap())
    };
    webview.eval(&render_tasks)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "fromrust")]
struct ToElm {
    message_nr : u32,
    task_done : Task,
    measurements:Vec<MeasurementCompact>,
    filter_options : FilterOptions
}

#[derive(Debug,Deserialize)]
#[serde(tag = "torust",content = "content")]
pub enum FromElm {
    Init,
    Log (String),
    Filter(FilterQuery),
    Process (ProcessQuery)
}

//{"torust":{"Log":"updated model"}}
#[derive(Debug,Serialize,Deserialize)]
pub enum Task {
    Init,
    Filtering,
    Processing
}
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct FilterQuery{
    sheet_names : Vec<String>,
    widths : Vec< String>,
    lengths : Vec< String>,
    temps : Vec< String>,
    wafer : String,
    dies : Vec< String>,
    test_type : String,
    measurement_speeds : Vec <String>,
    dates_between : ( Option <Date>, Option <Date> )
}

impl FilterQuery {
    fn filter (&self, measurements : Vec<MeasurementCompact>) -> Vec<MeasurementCompact> {
        //filter functions
        let f_sheetname = |measurement:&MeasurementCompact| self.sheet_names.contains(&measurement.sheet_name);
        let f_width     = |measurement:&MeasurementCompact| self.widths.contains(&measurement.device.width.unwrap_or(0.001).to_string() );
        let f_length    = |measurement:&MeasurementCompact| self.lengths.contains(&measurement.device.length.unwrap_or(0.001).to_string()  );
        let f_temp      = |measurement:&MeasurementCompact| {
            let temp_without_k :Vec<String>= self.temps.clone().into_iter().map(|s| String::from_iter(s.chars().filter_map(|c:char| (c.is_ascii_digit()).as_some(c) ) ) ).collect();
            temp_without_k.clone().contains(&measurement.device.temperature.unwrap_or(0).to_string()  )
            };
        let f_process   = |measurement:&MeasurementCompact| self.wafer == measurement.device.wafer.unwrap().to_string() ;
        let f_die       = |measurement:&MeasurementCompact| {
            let die = &measurement.device.die.as_ref().unwrap();
            let die_string = format!("({},{})",die.0,die.1);
            self.dies.contains(&die_string )
            };
        let f_testtype  = |measurement:&MeasurementCompact| self.test_type == measurement.test_parameter.test_type.to_string();
        let f_speed  = |measurement:&MeasurementCompact| self.measurement_speeds.contains(&measurement.test_parameter.measurement_speed.to_string());
        let f_dates     = |measurement:&MeasurementCompact| {
            let msmnt_date = measurement.test_time_stamp.year as u32 * 10000  + measurement.test_time_stamp.month as u32 * 100 + measurement.test_time_stamp.day as u32;
            let bottom_range = if let Some(time_stamp) = self.dates_between.0 {
                time_stamp.year as u32 * 10000  + time_stamp.month as u32 * 100 + time_stamp.day as u32
            } else { std::u32::MIN};
            let top_range = if let Some(time_stamp) = self.dates_between.1 {
                time_stamp.year as u32 * 10000  + time_stamp.month as u32 * 100 + time_stamp.day as u32
            } else { std::u32::MAX};
            msmnt_date >= bottom_range && msmnt_date <=top_range
        };

        let result :Vec<MeasurementCompact>= measurements.into_iter().filter(f_sheetname).collect();
        //println!("after sheetname{:#?}",result.len());
        let result :Vec<MeasurementCompact>= result.into_iter().filter(f_width).collect();
        //println!("{:#?}",result.len());
        let result :Vec<MeasurementCompact>= result.into_iter().filter(f_length).collect();
        //println!("after length{:#?}",result.len());
        let result :Vec<MeasurementCompact>= result.into_iter().filter(f_temp).collect();
        //println!("{:#?}",result.len());
        let result :Vec<MeasurementCompact>= result.into_iter().filter(f_process).collect();
        //println!("after process{:#?}",result.len());
        let result :Vec<MeasurementCompact>= result.into_iter().filter(f_die).collect();
        //println!("{:#?}",result.len());
        let result :Vec<MeasurementCompact>= result.into_iter().filter(f_testtype).collect();
        //println!("after testtype{:#?}",result.len());
        let result :Vec<MeasurementCompact>= result.into_iter().filter(f_speed).collect();
        //println!("after speed{:#?}",result.len());
        let result :Vec<MeasurementCompact>= result.into_iter().filter(f_dates).collect();
        //println!("after dates{:#?}",result.len());
        result
        }
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct FilterOptions{
    sheet_names : HashMap<String,u32>,
    widths : HashMap< String,u32>,
    lengths : HashMap< String,u32>,
    temps : HashMap< String,u32>,
    processes : HashMap< String,u32>,
    dies : HashMap< String,u32>,
    test_types : HashMap< String,u32>,
    measurement_speeds : HashMap <String,u32>,
    dates : HashMap<String,u32>
}
impl FilterOptions {
    fn new(measurements : &Vec<MeasurementCompact>) -> FilterOptions {
        let sheet_name_keys :Vec<String>= measurements.into_iter().map(|msmnt| msmnt.sheet_name.clone()).collect();
        let sheet_names = sheet_name_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.clone()).or_insert(0) += 1 as u32;
            acc
        });
        let width_keys :Vec<f64>= measurements.into_iter().map(|msmnt| msmnt.device.width.unwrap_or(0.0)).collect();
        let widths = width_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });
        let length_keys :Vec<f64>= measurements.into_iter().map(|msmnt| msmnt.device.length.unwrap_or(0.0)).collect();
        let lengths = length_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });
        let temp_keys :Vec<u32>= measurements.into_iter().map(|msmnt| msmnt.device.temperature.unwrap_or(0)).collect();
        let temps = temp_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(format!("{}K",c.to_string())).or_insert(0) += 1 as u32;
            acc
        });
        let process_keys :Vec<Process>= measurements.into_iter().map(|msmnt| msmnt.device.wafer.unwrap()).collect();
        let processes = process_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });
        let die_keys :Vec<String>= measurements.into_iter().map(|msmnt| {
            let die = msmnt.device.die.as_ref().unwrap().clone();
            format!("({},{})",die.0,die.1)    
        }).collect();
        let dies = die_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.clone()).or_insert(0) += 1 as u32;
            acc
        });
        let test_type_keys :Vec<&TestType>= measurements.into_iter().map(|msmnt| &msmnt.test_parameter.test_type).collect();
        let test_types = test_type_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });
        let measurement_speed_keys :Vec<&MeasurementSpeed>= measurements.into_iter().map(|msmnt| &msmnt.test_parameter.measurement_speed).collect();
        let measurement_speeds = measurement_speed_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });
        let date_keys :Vec<Date>= measurements.into_iter().map(|msmnt| msmnt.test_time_stamp.to_date()).collect();
        let dates = date_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });

        FilterOptions {sheet_names,widths,lengths,temps,processes,dies,test_types,measurement_speeds,dates}
    }
    fn filtered(measurements : &Vec<MeasurementCompact>, filter : FilterQuery) -> FilterOptions {
        
        let mut sheet_name_keys :Vec<String>= measurements.clone().into_iter().map(|msmnt| msmnt.sheet_name.clone()).collect();
        sheet_name_keys.sort_unstable();
        sheet_name_keys.dedup();
        let filtern :FilterQuery= FilterQuery {sheet_names:sheet_name_keys,..filter.clone()};
        let filteredn = filtern.filter(measurements.clone());
        let sheet_name_keys :Vec<String>= filteredn.into_iter().map(|msmnt| msmnt.sheet_name.clone()).collect();
        let sheet_names = sheet_name_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.clone()).or_insert(0) += 1 as u32;
            acc
        });
        
        let mut width_keys :Vec<String>= measurements.clone().into_iter().map(|msmnt| msmnt.device.width.unwrap_or(0.0).to_string()).collect();
        width_keys.sort_unstable();
        width_keys.dedup();
        let filtern :FilterQuery= FilterQuery {widths:width_keys,..filter.clone()};
        let filteredn = filtern.filter(measurements.clone());
        let width_keys :Vec<f64>= filteredn.into_iter().map(|msmnt| msmnt.device.width.unwrap_or(0.0)).collect();        
        let widths = width_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });

        let mut length_keys :Vec<String>= measurements.clone().into_iter().map(|msmnt| msmnt.device.length.unwrap_or(0.0).to_string()).collect();
        length_keys.sort_unstable();
        length_keys.dedup();
        let filtern :FilterQuery= FilterQuery {lengths:length_keys,..filter.clone()};
        let filteredn = filtern.filter(measurements.clone());
        let length_keys :Vec<f64>= filteredn.into_iter().map(|msmnt| msmnt.device.length.unwrap_or(0.0)).collect();        
        let lengths = length_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });

        let mut temp_keys :Vec<String>= measurements.clone().into_iter().map(|msmnt| msmnt.device.temperature.unwrap_or(0).to_string()).collect();
        temp_keys.sort_unstable();
        temp_keys.dedup();
        let filtern :FilterQuery= FilterQuery {temps:temp_keys,..filter.clone()};
        let filteredn = filtern.filter(measurements.clone());
        let temp_keys :Vec<u32>= filteredn.into_iter().map(|msmnt| msmnt.device.temperature.unwrap_or(0)).collect(); 
        let temps = temp_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(format!("{}K",c.to_string())).or_insert(0) += 1 as u32;
            acc
        });

        let mut process_keys :Vec<String>= measurements.clone().into_iter().map(|msmnt| msmnt.device.wafer.unwrap().to_string()).collect();
        process_keys.sort_unstable();
        process_keys.dedup();
        let mut processes = HashMap::new();
        for process_key in process_keys.into_iter(){
            let filtern :FilterQuery= FilterQuery {wafer:process_key.clone(),..filter.clone()};
            let filteredn = filtern.filter(measurements.clone());
            processes.insert(process_key, filteredn.len() as u32);
        }

        let mut die_keys :Vec<String>= measurements.clone().into_iter().map(|msmnt| {
            let die = msmnt.device.die.as_ref().unwrap().clone();
            format!("({},{})",die.0,die.1)    
        }).collect();
        die_keys.sort_unstable();
        die_keys.dedup();
        let filtern :FilterQuery= FilterQuery {dies:die_keys,..filter.clone()};
        let filteredn = filtern.filter(measurements.clone());
        let die_keys :Vec<String>= filteredn.into_iter().map(|msmnt| {
            let die = msmnt.device.die.as_ref().unwrap().clone();
            format!("({},{})",die.0,die.1)    
        }).collect();
        let dies = die_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.clone()).or_insert(0) += 1 as u32;
            acc
        });

        let mut test_type_keys :Vec<String>= measurements.clone().into_iter().map(|msmnt| msmnt.test_parameter.test_type.to_string().clone()).collect();
        test_type_keys.sort_unstable();
        test_type_keys.dedup();
        let mut test_types = HashMap::new();
        for test_type_key in test_type_keys.into_iter(){
            let filtern :FilterQuery= FilterQuery {test_type:test_type_key.clone(),..filter.clone()};
            let filteredn = filtern.filter(measurements.clone());
            test_types.insert(test_type_key, filteredn.len() as u32);
        }

        let mut measurement_speed_keys :Vec<String>= measurements.clone().into_iter().map(|msmnt| msmnt.test_parameter.measurement_speed.to_string()).collect();
        measurement_speed_keys.sort_unstable();
        measurement_speed_keys.dedup();
        let filtern :FilterQuery= FilterQuery {measurement_speeds:measurement_speed_keys,..filter.clone()};
        let filteredn = filtern.filter(measurements.clone());
        let measurement_speed_keys :Vec<MeasurementSpeed>= filteredn.into_iter().map(|msmnt| msmnt.test_parameter.measurement_speed).collect();
        let measurement_speeds = measurement_speed_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });

        let filtern :FilterQuery= FilterQuery {dates_between:(None,None),..filter.clone()};
        let filteredn = filtern.filter(measurements.clone());
        let date_keys :Vec<Date>= filteredn.into_iter().map(|msmnt| msmnt.test_time_stamp.to_date()).collect();
        let dates = date_keys.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.to_string()).or_insert(0) += 1 as u32;
            acc
        });

        FilterOptions {sheet_names,widths,lengths,temps,processes,dies,test_types,measurement_speeds,dates}
    }

    fn into_filter_query (&self) -> FilterQuery {
        
                    FilterQuery { 
                    sheet_names : self.sheet_names.keys().map(|a| a.clone()).collect()
                    , widths    : self.widths.keys().map(|a| a.clone()).collect()
                    , lengths   : self.lengths.keys().map(|a| a.clone()).collect()
                    , temps     : self.temps.keys().map(|a| a.clone()).collect()
                    , wafer     : self.processes.keys().map(|a| a.clone()).next().unwrap_or(String::from("MINOXG"))
                    , dies      : self.dies.keys().map(|a| a.clone()).collect()
                    , test_type : String::from("Sampling")
                    , measurement_speeds : self.measurement_speeds.keys().map(|a| a.clone()).collect()
                    , dates_between : ( None, None )
                    }
    }

}

#[derive(Debug,Deserialize)]
pub struct ProcessQuery {
    what : Vec<ProcessingType>,
    combined : bool,
    from : Vec<ProcessData>
}

#[derive(Debug,Serialize)]
pub struct DataSeries {
    title : String,
    data : Vec<ExportData>
}
#[derive(Debug,Serialize)]
pub struct ExportData{
    measurement_id : String,
    designator : String,
    data : Vec<Vec<f64>>
}

impl ExportData{
    fn from_testdata(id:String, testdata : TestData) -> ExportData{
        let designator :String= {
            if testdata.terminal == Terminal::Time {
                format!("T(s)")
            } else {
                format!("{}{}", testdata.unit.to_string_concise(),testdata.terminal.to_string_concise())
            }
        };
        let data = testdata.data;
        ExportData{measurement_id:id,designator,data}
    }
}


impl ProcessQuery{
    fn process(&self, measurements : Vec<Measurement>,output_dir: &str, script_dir:&str){

        let selected_measurements : Vec<Measurement>= {
            let ids :Vec<(String,Vec<TestDataCompact>)> = self.from.clone().into_iter().map(| pd| {(pd.id,pd.data)} ).collect();
            let id :Vec<String> =  ids.iter().map(|id| id.0.clone()).collect();
            let  measurements = measurements
                                    .iter()
                                    .filter(|m| id.clone().contains(&m.id));
            println!("{:?}", measurements
                                    .clone()
                                    .map(|m| m.test_data.len())
                                    .collect::<Vec<usize>>() 
                    );
            let result :Vec<Measurement>= measurements.map(|m| {
                let testdatacompact :Vec<TestDataCompact>= ids.iter().find_map(|id| {
                    if id.0 == m.id{
                        Some(id.1.clone())
                    } else {
                        None
                    }
                }).unwrap();
                let mut a= m.clone();
                a.test_data=m.test_data
                                .iter()
                                .filter_map(|t| t.from_compact( &testdatacompact ))
                                .collect::<Vec<TestData>>();
                a
            }).collect();

            result
        };

        let diff_wafer : bool = {
            let mut stringy = selected_measurements.iter().filter_map(|m| m.device.wafer).map(|p| p.to_string()).collect::<Vec<String>>();
            stringy.sort_unstable();
            stringy.dedup();
            stringy.len() >1
        };
        let diff_die : bool = {
            let mut stringy = selected_measurements.iter().filter_map(|m| m.device.die.clone()).map(|d| format!("{}{}",d.0,d.1)).collect::<Vec<String>>();
            stringy.sort_unstable();
            stringy.dedup();
            stringy.len() >1
        };
        let diff_temp : bool = {
            let mut stringy = selected_measurements.iter().filter_map(|m| m.device.temperature).collect::<Vec<u32>>();
            stringy.sort_unstable();
            stringy.dedup();
            stringy.len() >1
        };
        let diff_width : bool = {
            let mut stringy = selected_measurements.iter().filter_map(|m| m.device.width).map(|w| w.to_string()).collect::<Vec<String>>();
            stringy.sort_unstable();
            stringy.dedup();
            stringy.len() >1
        };
        let diff_length : bool = {
            let mut stringy = selected_measurements.iter().filter_map(|m| m.device.length).map(|l| l.to_string()).collect::<Vec<String>>();
            stringy.sort_unstable();
            stringy.dedup();
            stringy.len() >1
        };


        for pt in self.what.iter() {
            let testdata_total =  selected_measurements.iter().map(|m| {
                let testdata = m.test_data.clone();
                let normalize = |t:TestData| {
                    let data :Vec<Vec<f64>>= t.data.iter().map(|col|{
                        let sum :f64= col.iter().sum();
                        let average :f64= sum/( col.len() as f64);
                        col.into_iter().map(|i| i/average).collect::<Vec<f64>>()
                    }).collect();
                    TestData{data,..t}
                };
                let data :Vec<ExportData> = {
                    let temp = match pt {
                        ProcessingType::Raw => {
                            testdata
                        },
                        ProcessingType::Id_versus_time => {
                            testdata.into_iter().filter(|t| t.terminal == Terminal::Time || (t.terminal == Terminal::Drain && t.unit == Unit::Current) ).collect::<Vec<TestData>>()
                        },
                        ProcessingType::Id_normalized_versus_time =>{
                            let ids :Vec<TestData>= testdata.clone().into_iter().filter(|t| t.terminal == Terminal::Drain && t.unit == Unit::Current ).map(normalize).collect();
                            let mut times :Vec<TestData>= testdata.into_iter().filter(|t| t.terminal == Terminal::Time ).collect();
                            ids.iter().for_each(|t| times.push(t.clone()));
                            times
                        },
                        ProcessingType::Id_bins =>{
                            testdata.into_iter().filter(|t|t.terminal == Terminal::Drain && t.unit == Unit::Current ).collect::<Vec<TestData>>()
                        },
                        ProcessingType::Id_bins_normalized => {
                            testdata.into_iter().filter(|t|t.terminal == Terminal::Drain && t.unit == Unit::Current ).map(normalize).collect::<Vec<TestData>>()
                        },
                        ProcessingType::Id_for_swept_VDS_and_VGS => {
                            testdata.into_iter().filter(|t| (t.terminal == Terminal::Drain && t.unit == Unit::Current) || (t.terminal == Terminal::Drain && t.unit == Unit::Voltage) || (t.terminal == Terminal::Gate && t.unit == Unit::Voltage) ).collect::<Vec<TestData>>()
                        }
                    };
                    temp.into_iter().map(|t| ExportData::from_testdata(m.id.clone(),t)).collect::<Vec<ExportData>>()
                };
                let title : String = {
                    let wafer = if diff_wafer {
                        m.device.wafer.and_then(|w| Some(format!("P={} " ,w)) ).unwrap_or("".to_string())
                    } else {"".to_string()};
                    let die = if diff_die {
                        m.device.die.clone().and_then(|d| Some(format!("D={}{} ",d.0,d.1)) ).unwrap_or("".to_string())
                    } else {"".to_string()};
                    let temp = if diff_temp {
                        m.device.temperature.and_then(|t| Some(format!("T={}°K " ,t)) ).unwrap_or("".to_string())
                    } else {"".to_string()};
                    let width = if diff_width {
                        m.device.width.and_then(|w| { if w > 100.0 {
                            Some(format!("W={}μm",(w/1000.0)))
                        } else {
                            Some(format!("W={}nm ",w))
                        }} ).unwrap_or("".to_string())
                    } else {"".to_string()};
                    let length = if diff_length {
                        m.device.length.and_then(|l| Some(format!("L={} " ,l)) ).unwrap_or("".to_string())
                    } else {"".to_string()};
                    let id = if diff_wafer&& diff_die && diff_temp && diff_width && diff_length {
                        m.id.clone()
                    } else {
                        "".to_string()
                    };

                    format!("{}{}{}{}{}{}",wafer,die,temp,width,length,id)
                };

                DataSeries{title,data}

            }).collect::<Vec<DataSeries>>();
            
            if self.combined{
                match pt {
                    ProcessingType::Raw => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write( format!("{}/Raw.json",output_dir ), v.as_str() ).expect("error writing json");
                    },
                    _ => {}
                }
            } else {
                match pt {
                    ProcessingType::Raw => {
                        let v = serde_json::to_string(&testdata_total).unwrap();
                        fs::write( format!("{}/Raw.json",output_dir ), v.as_str() ).expect("error writing json");
                    },
                    _ => {}
                }
            }
            
        }

    }//fm
}//impl

#[derive(Debug,Deserialize,Clone)]
pub struct ProcessData {
    id : String,
    data : Vec<TestDataCompact> 
}

#[derive(Debug,Deserialize,Clone)]
#[serde(tag = "process_type")]
pub enum ProcessingType {
    Raw,
    Id_versus_time,
    Id_normalized_versus_time,
    Id_bins,
    Id_bins_normalized,
    Id_for_swept_VDS_and_VGS
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
