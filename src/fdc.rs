use crate::bar::Bar;
use crate::candle::Candle;
use crate::fx::Fx;

#[derive(Debug)]
pub struct FractalDetector {
    next_index: u64,
    candle_list: Vec<Candle>,
}

impl FractalDetector {
    pub fn new() -> Self {
        Self {
            next_index: 0,
            candle_list: Vec::new(),
        }
    }

    pub fn get_candles(&self) -> &Vec<Candle> {
        self.candle_list.as_ref()
    }

    fn add_candle(&mut self, bar: &Bar) {
        let c = Candle::from_bar(self.next_index, bar);
        self.next_index += 1;
        self.candle_list.push(c);
    }

    // 检查是否为顶底分型
    fn check_fractal(&self) -> Option<Fx> {
        if self.candle_list.len() >= 3 {
            let _1 = self.candle_list.len() - 1;
            let _2 = self.candle_list.len() - 2;
            let _3 = self.candle_list.len() - 3;
            let k1 = &self.candle_list[_3];
            let k2 = &self.candle_list[_2];
            let k3 = &self.candle_list[_1];

            Fx::check_fractal(k1, k2, k3)
        } else {
            None
        }
    }

    // 处理与当前bar的包含关系
    fn merge_bar(&mut self, bar: &Bar) -> bool {
        // 队列中有至少两个经过包含处理的Candle
        debug_assert!(self.candle_list.len() >= 2);
        let _1 = self.candle_list.len() - 1;
        let _2 = self.candle_list.len() - 2;
        let direction = {
            let k1 = &self.candle_list[_2];
            let k2 = &self.candle_list[_1];
            Candle::check_direction(k1, k2)
        };

        let current = &mut self.candle_list[_1];

        Candle::merge(direction, current, bar)
    }

    pub fn on_new_bar(&mut self, bar: &Bar) -> Option<Fx> {
        let wlen = self.candle_list.len();
        match wlen {
            0 | 1 => self.add_candle(bar),
            _ => {
                let merged = self.merge_bar(bar);
                if !merged {
                    let result = self.check_fractal();
                    self.add_candle(bar);
                    return result;
                }
            }
        }
        None
    }
}
