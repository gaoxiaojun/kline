use crate::time::*;
use crate::{candle::Candle, time::Time};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FractalType {
    Top,
    Bottom,
}

#[derive(Clone)]
pub struct Fx {
    pub time: Time,
    pub fx_mark: FractalType,
    pub price: f64,
    pub start: Time,
    pub end: Time,
    pub high: f64,
    pub low: f64,
    pub highest: f64,
    pub lowest: f64,
    pub index: u64,
}

impl Fx {
    pub fn new(
        time: Time,
        mark: FractalType,
        price: f64,
        start: Time,
        end: Time,
        high: f64,
        low: f64,
        highest: f64,
        lowest: f64,
        index: u64,
    ) -> Self {
        Self {
            time,
            fx_mark: mark,
            price,
            start,
            end,
            high,
            low,
            highest,
            lowest,
            index,
        }
    }
    
    pub fn is_same_type(&self, other: &Fx) -> bool {
        self.fx_mark == other.fx_mark
    }

    pub fn has_enough_distance(&self, rhs: &Fx) -> bool {
        self.distance(rhs) >= 4
    }

    fn get_lowest(k1: &Candle, k2: &Candle, k3: &Candle) -> f64 {
        f64::min(f64::min(k1.bar.low, k2.bar.low), k3.bar.low)
    }

    fn get_highest(k1: &Candle, k2: &Candle, k3: &Candle) -> f64 {
        f64::max(f64::max(k1.bar.high, k2.bar.high), k3.bar.high)
    }

    fn get_fx_type(k1: &Candle, k2: &Candle, k3: &Candle) -> FractalType {
        let is_top = (k1.bar.high < k2.bar.high) && (k2.bar.high > k3.bar.high);
        if is_top {
            FractalType::Top
        } else {
            FractalType::Bottom
        }
    }

    fn build_fx(k1: &Candle, k2: &Candle, k3: &Candle) -> Fx {
        let fx_type = Fx::get_fx_type(k1, k2, k3);

        if fx_type == FractalType::Top {
            return Fx::new(
                k2.bar.time,
                fx_type,
                k2.bar.high,
                k1.bar.time,
                k3.bar.time,
                k2.bar.high,
                k2.bar.low,
                k2.bar.high,
                Fx::get_lowest(k1, k2, k3),
                k2.index,
            );
        } else {
            return Fx::new(
                k2.bar.time,
                fx_type,
                k2.bar.low,
                k1.bar.time,
                k3.bar.time,
                k2.bar.high,
                k2.bar.low,
                Fx::get_highest(k1, k2, k3),
                k2.bar.low,
                k2.index,
            );
        };
    }

    pub fn distance(&self, rhs: &Fx) -> u64 {
        if self.index > rhs.index {
            self.index - rhs.index
        } else {
            rhs.index - self.index
        }
    }
    // 检查分型
    pub fn check_fractal(k1: &Candle, k2: &Candle, k3: &Candle) -> Option<Fx> {
        debug_assert!(k1.index != k2.index && k1.index != k3.index && k2.index != k3.index);
        if ((k1.bar.high < k2.bar.high) && (k2.bar.high > k3.bar.high))
            || ((k1.bar.low > k2.bar.low) && (k2.bar.low < k3.bar.low))
        {
            return Some(Fx::build_fx(k1, k2, k3));
        }
        None
    }
}

impl fmt::Display for Fx {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let time_str = time_to_str(self.time);
        let mark_str = if self.fx_mark == FractalType::Top {
            "顶"
        } else {
            "低"
        };
        write!(
            f,
            "time:{}, type:{}, price: {}, high:{}, low{}",
            time_str, mark_str, self.price, self.high, self.low
        )
    }
}

impl fmt::Debug for Fx {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let time_str = time_to_str(self.time);
        let mark_str = if self.fx_mark == FractalType::Top {
            "顶"
        } else {
            "低"
        };
        f.debug_struct("Fx")
            .field("time", &time_str)
            .field("mark", &mark_str)
            .field("price", &self.price)
            .field("high", &self.high)
            .field("low", &self.low)
            .finish()
    }
}

impl PartialEq for Fx {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}
