use crate::bar::Bar;
use crate::time::*;
use crate::{candle::Candle, time::Time};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FractalType {
    Top,
    Bottom,
}
// 分型
#[derive(Debug, Clone)]
pub struct Fractal {
    pub k1: Candle,
    pub k2: Candle,
    pub k3: Candle,
    // cache
    ftype: FractalType,
}

// 计算分型之间K线的数量,K线是经过包含处理过的
fn distance(lhs: &Fractal, rhs: &Fractal) -> u64 {
    if rhs.k2.index > lhs.k2.index {
        rhs.k2.index - lhs.k2.index
    } else {
        lhs.k2.index - rhs.k2.index
    }
}

impl Fractal {
    pub fn new(k1: Candle, k2: Candle, k3: Candle) -> Self {
        debug_assert!(
            // 合并之后，分型的最高/最低是唯一的，所以没有等号
            ((k1.bar.high < k2.bar.high) && (k2.bar.high > k3.bar.high)) // Top
                || ((k1.bar.low > k2.bar.low) && (k2.bar.low < k3.bar.low)) // Bottom
        );

        let is_top = (k1.bar.high < k2.bar.high) && (k2.bar.high > k3.bar.high);
        let ftype = if is_top {
            FractalType::Top
        } else {
            FractalType::Bottom
        };

        Self { k1, k2, k3, ftype }
    }

    //  ------k2---------
    //  ------|----------
    //  -k1-|---|-k3-----
    //  ------|----------
    //  -----k2----------

    // 检查分型
    pub fn check_fractal(k1: &Candle, k2: &Candle, k3: &Candle) -> Option<Fractal> {
        debug_assert!(k1.index != k2.index && k1.index != k3.index && k2.index != k3.index);
        if ((k1.bar.high < k2.bar.high) && (k2.bar.high > k3.bar.high))
            || ((k1.bar.low > k2.bar.low) && (k2.bar.low < k3.bar.low))
        {
            return Some(Fractal::new(k1.clone(), k2.clone(), k3.clone()));
        }
        None
    }

    pub(crate) fn distance(&self, other: &Fractal) -> u64 {
        distance(self, other)
    }

    pub fn has_enough_distance(&self, other: &Fractal) -> bool {
        self.distance(other) >= 4
    }

    pub fn is_same_type(&self, other: &Fractal) -> bool {
        self.ftype == other.ftype
    }

    pub fn time(&self) -> Time {
        self.k2.bar.time
    }

    pub fn fractal_type(&self) -> FractalType {
        self.ftype
    }
    // 分型最高点
    pub fn highest(&self) -> f64 {
        if self.ftype == FractalType::Top {
            self.k2.bar.high
        } else {
            f64::max(self.k1.bar.high, self.k3.bar.high)
        }
    }

    // 分型最低点
    pub fn lowest(&self) -> f64 {
        if self.ftype == FractalType::Bottom {
            self.k2.bar.low
        } else {
            f64::min(self.k1.bar.low, self.k3.bar.low)
        }
    }

    pub fn high(&self) -> f64 {
        if self.ftype == FractalType::Top {
            self.k2.bar.high
        } else {
            self.lowest()
        }
    }

    pub fn low(&self) -> f64 {
        if self.ftype == FractalType::Top {
            self.highest()
        } else {
            self.k2.bar.low
        }
    }
    // 返回分型的极值
    pub fn price(&self) -> f64 {
        if self.ftype == FractalType::Bottom {
            self.k2.bar.low
        } else {
            self.k2.bar.high
        }
    }

    // 分型包含规则2，3根Candle的最高最低作为分型区间
    // rule2的包含规则是上述分型区间包含,要求后一个分型不能在前一个分型的区间内
    pub fn is_contain_rule2(&self, other: &Fractal) -> bool {
        if self.highest() >= other.highest() && self.lowest() <= other.lowest() {
            true
        } else {
            false
        }
    }

    // 分型包含规则1，第二根Candle的最高最低作为分型区间
    pub fn is_contain_rule1(&self, other: &Fractal) -> bool {
        if self.k2.high() >= other.k2.high() && self.k2.low() <= other.k2.low() {
            true
        } else {
            false
        }
    }

    pub fn is_contain(&self, other: &Fractal) -> bool {
        self.is_contain_rule1(other)
    }
}

impl PartialEq for Fractal {
    fn eq(&self, other: &Self) -> bool {
        self.time() == other.time() && self.price() == other.price()
    }
}
