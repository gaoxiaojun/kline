use crate::bar::Bar;
use crate::candle::Candle;
use crate::fractal::Fx;
use crate::ringbuffer::RingBuffer;


/// FractalDetectorWithoutCandleList用于检测分型
/// 输入Bar，输出分型
/// 与FractalDetector的区别是不再内部保留Candles列表，用于节省内存
///
/// TODO: 1. 初始边界条件验证，最初的两个candle必须是非包含的
///       2. 一字板处理


#[derive(Debug)]
pub struct FractalDetectorWithoutCandleList {
    window: RingBuffer<Candle>,
    next_index: u64,
}

impl FractalDetectorWithoutCandleList {
    pub fn new() -> Self {
        Self {
            window: RingBuffer::new(3),
            next_index: 0,
        }
    }

    // 当确定当前Bar与前Candle不存在合并关系的时候，该方法被调用
    fn add_candle(&mut self, bar: &Bar) {
        let c = Candle::from_bar(self.next_index, bar);
        self.next_index += 1;
        self.window.push(c);
    }

    // 检查是否为顶底分型
    fn check_fractal(&self) -> Option<Fx> {
        if self.candle_list.len() >= 3 {
            let k1 = self.window.get(-3).unwrap();
            let k2 = self.window.get(-2).unwrap();
            let k3 = self.window.get(-1).unwrap();

            Fx::check_fractal(k1, k2, k3)
        } else {
            None
        }
    }

    // 处理与当前bar的包含关系
    fn merge_bar(&mut self, bar: &Bar) -> bool {
        // 队列中有至少两个经过包含处理的Candle
        debug_assert!(self.window.len() >= 2);
        let direction = {
            let k1 = self.window.get(-2).unwrap();
            let k2 = self.window.get(-1).unwrap();
            Candle::check_direction(k1, k2)
        };

        let current = self.window.get_mut(-1).unwrap();

        Candle::merge_old(direction, current, bar)
    }

    // 处理K线包含关系，更新内部缓冲区，检测分型
    pub fn on_new_bar(&mut self, bar: &Bar) -> Option<Fx> {
        let len = self.window.len();
        debug_assert!(len <= 3);

        match len {
            0 | 1 => {
                // 队列中没有K线
                self.add_candle(bar);
            }

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
