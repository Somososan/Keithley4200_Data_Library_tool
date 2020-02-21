use super::super::testparameter::TestType;
use crate::calamine_helper::MyRange;

#[allow(non_snake_case)]
struct Positions {
    Sweeping: usize,
    Sampling: usize,
}

const OFFSETS: Positions = Positions {
    Sweeping: 9,
    Sampling: 6,
};

pub fn extract(column: &MyRange, test_type: &TestType) -> Option<f64> {
    let extract_at = |offset| {
        column
            .it
            .get((offset, 0))?
            .get_string()?
            .parse::<f64>()
            .ok()
    };

    match test_type {
        TestType::Sweeping => extract_at(OFFSETS.Sweeping),
        TestType::Sampling => extract_at(OFFSETS.Sampling),
    }
}
