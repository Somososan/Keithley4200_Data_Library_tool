mod measurement {
    use serde::*;    
    use calamine::*;

    
    pub mod testparameter;

    #[derive(Debug,Serialize,Deserialize,Clone)]
    pub struct Measurement {
        //file properties
        pub id              : String,
        pub file_path       : String,
        pub sheet_name      : String,
        //Device Under Test
        pub device          : Option<Device>,
        //Test parameters
        pub test_parameters : TestParameter,
        pub test_time_stamp : TimeStamp,
        pub terminals       : Vec<TerminalParameter>,
        //data
        pub test_data       : Vec<TestData>
    }
}