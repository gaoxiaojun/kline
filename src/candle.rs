use crate::bar::Bar;
use crate::time::Time;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

// Candle是经过包含处理的K线
#[derive(Debug, Clone)]
pub struct Candle {
    // index的作用是为了计算Candle之间的距离，严格笔要求分型之间有5根K，通过index2 - index1就很容易检测是否满足条件，而无需保存整个Candle序列
    // 检测到分型的时候，分型的index就是分型中间Candle的index
    // index必须保持严格的递增
    pub index: u64,
    pub bar: Bar,
}

impl Candle {
    pub(crate) fn new(
        index: u64,
        time: Time,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        vol: f64,
    ) -> Self {
        Self {
            index,
            bar: Bar::new(time, open, high, low, close, vol),
        }
    }

    pub(crate) fn high(&self) -> f64 {
        return self.bar.high
    }

    pub(crate) fn low(&self) -> f64 {
        return self.bar.low
    }

    pub(crate) fn from_bar(index: u64, bar: &Bar) -> Self {
        Self {
            index,
            bar: bar.clone(),
        }
    }

    // 检测包含方向
    pub fn check_direction(k1: &Candle, k2: &Candle) -> Direction {
        debug_assert!(k1.index != k2.index);
        if k1.bar.high + k1.bar.low > k2.bar.high + k2.bar.low {
            Direction::Down
        } else {
            Direction::Up
        }
    }

    // 检测并处理包含关系
    // 返回值: true:存在包含关系， false:没有包含关系
    pub fn merge(direction: Direction, current: &mut Candle, bar: &Bar) -> bool {
        // current,bar是否有包含关系
        if (current.bar.high >= bar.high && current.bar.low <= bar.low)
            || (current.bar.high <= bar.high && current.bar.low >= bar.low)
        {
            // 特殊的一字板与前一根K高低点相同情况的处理
            //let high_eq_low = bar.high == bar.low; // 一字板

            match direction {
                Direction::Down => {
                    // 下包含，取低低
                    //if high_eq_low && bar.low == current.bar.low {
                    //    // 一字板特例，不处理，直接忽略当前的bar
                    //    return true;
                    //}

                    if current.bar.low > bar.low {
                        current.bar.time = bar.time;
                    }
                    current.bar.high = f64::min(bar.high, current.bar.high);
                    current.bar.low = f64::min(bar.low, current.bar.low);
                    current.bar.open = current.bar.high;
                    current.bar.close = current.bar.low;
                    current.bar.vol += current.bar.vol;
                }

                Direction::Up => {
                    // 上包含，取高高
                    //if high_eq_low && bar.high == current.bar.high {
                    // 一字板特例，不处理，直接忽略当前的bar
                    //    return true;
                    //}

                    if current.bar.high < bar.high {
                        current.bar.time = bar.time;
                    }
                    current.bar.high = f64::max(bar.high, current.bar.high);
                    current.bar.low = f64::max(bar.low, current.bar.low);
                    current.bar.close = current.bar.high;
                    current.bar.open = current.bar.low;
                    current.bar.vol += current.bar.vol;
                }
            }
            current.bar.close = bar.close;
            true
        } else {
            false
        }
    }
}
