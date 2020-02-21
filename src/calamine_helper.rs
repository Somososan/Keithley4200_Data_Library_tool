use calamine::*;

//calamine helper struct
#[derive(Debug)]
pub struct MyRange {
    pub it: Range<DataType>,
    abs_pos: (u32, u32),
}

impl MyRange {
    pub fn new(range: Range<DataType>) -> MyRange {
        MyRange {
            it: range,
            abs_pos: (0, 0),
        }
    }

    pub fn sub_range(&self, start: (usize, usize), end: (usize, usize)) -> MyRange {
        let abs_pos = (
            self.abs_pos.0 + start.0 as u32,
            self.abs_pos.1 + start.1 as u32,
        );
        assert!(end <= self.end());
        MyRange {
            it: self.it.range(
                abs_pos,
                (self.abs_pos.0 + end.0 as u32, self.abs_pos.1 + end.1 as u32),
            ),
            abs_pos,
        }
    }

    pub fn end(&self) -> (usize, usize) {
        (
            self.it.end().unwrap().0 as usize,
            self.it.end().unwrap().1 as usize,
        )
    }
}
