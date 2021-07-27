use crate::bar::Bar;
use crate::candle::Candle;
use crate::fractal::Fx;
use crate::ringbuffer::RingBuffer;

#[derive(Debug)]
pub struct Analyzer {
    bars: Vec<Bar>,
    candles: Vec<Candle>,
    next_index: u64,
    fx_list: Vec<Fx>,
    bi_list: Vec<Fx>,
    xd_list: Vec<Fx>,
    //
    window: RingBuffer<Candle>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            bars: Vec::new(),
            candles: Vec::new(),
            fx_list: Vec::new(),
            next_index: 0,
            bi_list: Vec::new(),
            xd_list: Vec::new(),
            window: RingBuffer::new(3),
        }
    }

    pub fn get_candles(&self) -> &Vec<Candle> {
        &self.candles
    }

    fn update_candle(&mut self) {}

    fn update_fx(&mut self) {}

    fn update_bi_list(&mut self) {}

    fn update_xd_list(&mut self) {}

    fn add_candle(&mut self, bar: &Bar) {
        if self.window.len() > 0 {
            let last = self.window.get(-1).unwrap();
            self.candles.push(last.clone());
        }

        let c = Candle::from_bar(self.next_index, bar);
        self.next_index += 1;
        self.window.push(c);
    }
    // 检查是否为顶底分型
    fn check_fractal(&self) -> Option<Fx> {
        let k1 = self.window.get(-3).unwrap();
        let k2 = self.window.get(-2).unwrap();
        let k3 = self.window.get(-1).unwrap();

        Fx::check_fractal(k1, k2, k3)
    }

    // 处理与当前bar的包含关系
    fn process_contain_relationship(&mut self, bar: &Bar) -> bool {
        // 队列中有至少两个经过包含处理的Candle
        debug_assert!(self.window.len() >= 2);
        let direction = {
            let k1 = self.window.get(-2).unwrap();
            let k2 = self.window.get(-1).unwrap();
            Candle::check_direction(k1, k2)
        };

        let current = self.window.get_mut(-1).unwrap();

        Candle::merge(direction, current, bar)
    }
    pub fn on_new_bar(&mut self, bar: &Bar) -> Option<Fx> {
        self.bars.push(bar.clone());
        let wlen = self.window.len();
        match wlen {
            0 | 1 => self.add_candle(&bar),
            2 => {
                let merged = self.process_contain_relationship(bar);
                if !merged {
                    self.add_candle(&bar);
                }
            }

            _ => {
                let merged = self.process_contain_relationship(bar);
                if !merged {
                    let result = self.check_fractal();
                    self.add_candle(&bar);
                    return result;
                }
            }
        }
        None
    }
}
