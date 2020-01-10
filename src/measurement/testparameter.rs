mod testparameter {
    pub struct TestParameter{
        pub test_type : TestType,
        pub measurement_speed : MeasurementSpeed,
        pub ad_aperture : Option <f64>,
        pub filter_factor : Option <f64>,
        pub interval_time : Option <f64>,
        pub sweep_delay_time : Option<f64>,
        pub hold_time : f64
    }
}