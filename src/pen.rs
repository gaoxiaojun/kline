use crate::fractal::{FractalType, Fractal};
use crate::time::Time;
use crate::point::Point;

#[derive(Debug)]
pub struct Pen {
    pub from: Point,
    pub to: Point,
}


impl Pen {
    pub fn new(from: Point, to: Point) -> Self {
        Self {
            from,to
        }
    }
}