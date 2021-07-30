use crate::fx::{FractalType, Fx};
use crate::time::Time;
use crate::pen::Pen;
use crate::point::Point;


#[derive(Debug)]
pub struct Segment {
    from: Point,
    to: Point,
    pens: Vec<Pen>,
}


impl Segment {
    pub fn new(from: Point, to: Point) -> Self {
        Self {
            from,to,
            pens:Vec::new()
        }
    }
}