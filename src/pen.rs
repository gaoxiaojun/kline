use crate::fractal::{FractalType,Fx};

pub fn detect_is_pen(f1: &Fx, f2: &Fx) -> bool {
    if f1.fx_mark == FractalType::Top
        && f2.fx_mark == FractalType::Bottom
        && f1.has_enough_distance(f2)
        && f2.low < f1.low
        && !f1.is_contain(f2)
    {
        return true;
    }

    if f1.fx_mark == FractalType::Bottom
        && f2.fx_mark == FractalType::Top
        && f1.has_enough_distance(f2)
        && f2.high > f1.high
        && !f1.is_contain(f2)
    {
        return true;
    }

    false
}