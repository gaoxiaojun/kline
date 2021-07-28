use crate::bar::Bar;
use crate::candle::Candle;
use crate::fractal::{FractalType,Fx};

#[derive(Debug)]
pub struct Analyzer {
    bars: Vec<Bar>,
    candle_list: Vec<Candle>,
    next_index: u64,
    fx_list: Vec<Fx>,
    bi_list: Vec<Fx>,
    xd_list: Vec<Fx>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            bars: Vec::new(),
            candle_list: Vec::new(),
            fx_list: Vec::new(),
            next_index: 0,
            bi_list: Vec::new(),
            xd_list: Vec::new(),
        }
    }

    pub fn get_bars(&self) -> &Vec<Bar> {
        &self.bars
    }

    pub fn get_candles(&self) -> &Vec<Candle> {
        &self.candle_list
    }

    pub fn get_fxs(&self) -> &Vec<Fx> {
        &self.fx_list
    }

    pub fn get_bis(&self) -> &Vec<Fx> {
        &self.bi_list
    }

    pub fn get_xd(&self) -> &Vec<Fx> {
        &self.xd_list
    }

    fn update_bi_list(&mut self) -> bool{
        if self.fx_list.len() < 2 {
            return false;
        }

        let _1 = self.fx_list.len() -1;
        let _2 = self.fx_list.len() -2;
 
        let prev = &self.fx_list[_2];
        let next = &self.fx_list[_1];
        if prev.is_contain(next) {
            return false;
        }
        let has_enough_distance = prev.has_enough_distance(next);
        let prev_price_higher = if prev.price > next.price {
            true
        }else {
            false
        };

        match (prev.fx_mark, next.fx_mark, has_enough_distance, prev_price_higher) {
            (FractalType::Top, FractalType::Bottom, true, true) => { 
                // 前顶后低 and 距离足够 and 前高后低 => 新的下降笔
                if self.bi_list.len() < 1 {
                    self.bi_list.push(prev.clone());
                }
                self.bi_list.push(next.clone());
            },

            (FractalType::Bottom, FractalType::Top, true, false) => { 
                // 前底后顶 and 距离足够 and 前低后高 => 新的上升笔
                if self.bi_list.len() < 1 {
                    self.bi_list.push(prev.clone());
                }
                self.bi_list.push(next.clone());
            },

            (FractalType::Top, FractalType::Bottom, _, false) => { 
                // 前顶后顶 and 前低后高 => 上升笔延伸
                if self.bi_list.len() > 1 {
                    self.bi_list.pop();
                    self.bi_list.push(next.clone());
                }
            },

            (FractalType::Bottom, FractalType::Top, _, true) => { 
                // 前底后底 and 前高后低 => 下降笔延伸
                if self.bi_list.len() > 1 {
                    self.bi_list.pop();
                    self.bi_list.push(next.clone());
                }
            },
            (_,_,_,_) => {

            }
        }

        false
    }

    fn update_xd_list(&mut self) ->bool{
        false
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

    fn update_candle_fx(&mut self, bar: &Bar) -> bool {
        let wlen = self.candle_list.len();
        match wlen {
            0 | 1 => self.add_candle(bar),
            _ => {
                let merged = self.merge_bar(bar);
                if !merged {
                    let result = self.check_fractal();
                    self.add_candle(bar);
                    if let Some(f) = result {
                        self.fx_list.push(f);
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn on_new_bar(&mut self, bar: &Bar) {
        self.bars.push(bar.clone());
        let has_new_fx = self.update_candle_fx(bar);
        let has_new_pen = if has_new_fx {
            self.update_bi_list()
        }else {
            false
        };
        let has_new_xd = if has_new_pen {
            self.update_xd_list()
        }else {
            false
        };
    }
}
