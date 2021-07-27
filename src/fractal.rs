use crate::bar::Bar;
use crate::{candle::Candle, time::Time};
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

fn has_gap(k1: &Bar, k2: &Bar, min_gap: f64) -> bool {
    assert!(k1.time < k2.time);
    if k1.high < k2.low * (1.0 - min_gap) || k2.high < k1.low * (1.0 - min_gap) {
        true
    } else {
        false
    }
}

fn has_gap2(k1: &Bar, k2: &Bar) -> bool {
    assert!(k1.time < k2.time);
    if k1.high < k2.low || k2.high < k1.low {
        true
    } else {
        false
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
        self.time() == other.time()
    }
}

#[derive(Debug, Clone)]
pub struct Fx {
    pub time: Time,
    pub fx_mark: FractalType,
    pub price: f64,
    pub start: Time,
    pub end: Time,
    pub high: f64,
    pub low: f64,
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
            index,
        }
    }
    pub fn is_same_type(&self, other: &Fx) -> bool {
        self.fx_mark == other.fx_mark
    }
    pub fn has_enough_distance(&self, rhs: &Fx) -> bool {
        self.distance(rhs) >= 4
    }
    fn get_low(k1: &Candle, k2: &Candle, k3: &Candle, gap1: bool, gap3: bool) -> f64 {
        match (gap1, gap3) {
            (true, true) => k2.bar.low,
            (true, false) => f64::min(k2.bar.low, k3.bar.low),
            (false, true) => f64::min(k1.bar.low, k2.bar.low),
            (false, false) => {
                let k1_k2_low = f64::min(k1.bar.low, k2.bar.low);
                f64::min(k1_k2_low, k3.bar.low)
            }
        }
    }

    fn get_high(k1: &Candle, k2: &Candle, k3: &Candle, gap1: bool, gap3: bool) -> f64 {
        match (gap1, gap3) {
            (true, true) => k2.bar.high,
            (true, false) => f64::max(k2.bar.high, k3.bar.high),
            (false, true) => f64::max(k1.bar.high, k2.bar.high),
            (false, false) => {
                let k1_k2_high = f64::max(k1.bar.high, k2.bar.high);
                f64::max(k1_k2_high, k3.bar.high)
            }
        }
    }

    fn build_fx(k1: &Candle, k2: &Candle, k3: &Candle) -> Fx {
        let is_top = (k1.bar.high < k2.bar.high) && (k2.bar.high > k3.bar.high);
        let ftype = if is_top {
            FractalType::Top
        } else {
            FractalType::Bottom
        };
        let price = if ftype == FractalType::Bottom {
            k2.bar.low
        } else {
            k2.bar.high
        };
        // 如果k1和K2之间有gap，去掉k1
        let k1_k2_has_gap = has_gap2(&k1.bar, &k2.bar);
        // 如果k2和K3之间有gap，去掉k3
        let k2_k3_has_gap = has_gap2(&k2.bar, &k3.bar);

        let fx = if ftype == FractalType::Top {
            let high = k2.bar.high;
            let low = Fx::get_low(k1, k2, k3, k1_k2_has_gap, k2_k3_has_gap);
            Fx::new(
                k2.bar.time,
                ftype,
                price,
                k1.bar.time,
                k3.bar.time,
                high,
                low,
                k2.index,
            )
        } else {
            let low = k2.bar.low;
            let high = Fx::get_high(k1, k2, k3, k1_k2_has_gap, k2_k3_has_gap);
            Fx::new(
                k2.bar.time,
                ftype,
                price,
                k1.bar.time,
                k3.bar.time,
                high,
                low,
                k2.index,
            )
        };
        fx
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

    // 分型包含规则，第二根Candle的最高最低作为分型区间
    pub fn is_contain(&self, other: &Fx) -> bool {
        if (self.fx_mark == FractalType::Top && self.low < other.low && self.high < other.high)
            || (self.fx_mark == FractalType::Bottom
                && self.high > other.high
                && self.low > other.low)
        {
            true
        } else {
            false
        }
    }
}

impl PartialEq for Fx {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}
