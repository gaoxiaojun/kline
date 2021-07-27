use crate::bar::Bar;
use crate::candle::Candle;
use crate::fractal::Fx;
use crate::ringbuffer::RingBuffer;

pub struct FractalDetector {
    window: RingBuffer<Candle>,
    next_index: u64,
    candles: Option<Vec<Bar>>,
}

impl FractalDetector {
    pub fn new() -> Self {
        Self {
            window: RingBuffer::new(3),
            next_index: 0,
            candles: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_candles() -> Self {
        Self {
            window: RingBuffer::new(3),
            next_index: 0,
            candles: Some(Vec::new()),
        }
    }

    #[allow(dead_code)]
    pub fn get_candles(&self) -> Option<&Vec<Bar>> {
        self.candles.as_ref()
    }

    fn notify(&mut self) {
        if let Some(container) = self.candles.as_mut() {
            if self.window.len() > 0 {
                let last = self.window.get(-1).unwrap();
                container.push(last.bar.clone());
            }
        }
    }

    // 当确定当前Bar与前Candle不存在合并关系的时候，该方法被调用
    fn add_candle(&mut self, bar: &Bar) {
        self.notify();
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

        Candle::merge_old(direction, current, bar)
    }

    // 处理K线包含关系，更新内部缓冲区，检测分型
    pub fn on_new_bar(&mut self, bar: &Bar) -> Option<Fx> {
        let len = self.window.len();
        debug_assert!(len <= 3);

        // 初始边界条件验证，前两个candle必须是非包含的
        match len {
            0 => {
                // 队列中没有K线
                self.add_candle(bar);
            }
            1=> {
                self.add_candle(bar);
            }

            /*1 => {
                // 仅有一根K线
                // 起始开始的两K就存在包含关系，合理的处理方式是：
                // 1. 如果第一根K包含第二根K，直接忽略与第一根K存在包含的K线，直到遇到不包含的
                // 2. 如果第一根K包含在第二根K，忽略第一根K，从第二根K开始
                let last = self.window.get(-1).unwrap();
                let k1_include_k2 = last.bar.high >= bar.high && last.bar.low <= bar.low;
                let k2_include_k1 = last.bar.high <= bar.high && last.bar.low >= bar.low;
                if k1_include_k2 {
                    // 情况1，忽略当前Bar，直到遇到不包含的
                    return None;
                };

                if k2_include_k1 {
                    // 情况2，忽略K1,清空队列
                    self.window.clear();
                }
                // 当前Bar作为Candle放入队列
                self.add_candle(bar);
            }*/

            2 => {
                let merged = self.process_contain_relationship(bar);
                if !merged {
                    self.add_candle(bar);
                }
            }

            _ => {
                let merged = self.process_contain_relationship(bar);
                if !merged {
                    let result = self.check_fractal();
                    self.add_candle(bar);
                    return result;
                }
            }
        }
        None
    }


    // 处理K线包含关系，更新内部缓冲区，检测分型
    pub fn on_new_bar_old(&mut self, bar: &Bar) -> Option<Fx> {
        let len = self.window.len();
        debug_assert!(len <= 3);

        // 初始边界条件验证，前两个candle必须是非包含的
        match len {
            0 => {
                // 队列中没有K线
                self.add_candle(bar);
            }
            
            1 => {
                
                // 仅有一根K线
                // 起始开始的两K就存在包含关系，合理的处理方式是：
                // 1. 如果第一根K包含第二根K，直接忽略与第一根K存在包含的K线，直到遇到不包含的
                // 2. 如果第一根K包含在第二根K，忽略第一根K，从第二根K开始
                let last = self.window.get(-1).unwrap();
                let k1_include_k2 = last.bar.high >= bar.high && last.bar.low <= bar.low;
                let k2_include_k1 = last.bar.high <= bar.high && last.bar.low >= bar.low;
                if k1_include_k2 {
                    // 情况1，忽略当前Bar，直到遇到不包含的
                    println!("Ignore K2");
                    return None;
                };

                if k2_include_k1 {
                    println!("Ignore K1");
                    // 情况2，忽略K1,清空队列
                    self.window.clear();
                }
                println!("Only Once");
                // 当前Bar作为Candle放入队列
                self.add_candle(bar);
            }

            2 => {
                let merged = self.process_contain_relationship(bar);
                if !merged {
                    self.add_candle(bar);
                }
            }

            _ => {
                let merged = self.process_contain_relationship(bar);
                if !merged {
                    let result = self.check_fractal();
                    self.add_candle(bar);
                    return result;
                }
            }
        }
        None
    }

    /*
    // 处理K线包含关系，更新内部缓冲区，检测分型
    pub fn on_new_bar_old(&mut self, bar: &Bar) -> Option<Fractal> {
        let len = self.window.len();
        debug_assert!(len <= 3);

        // 初始边界条件验证，前两个candle必须是非包含的
        match len {
            0 => {
                // 队列中没有K线
                self.add_candle(bar);
            }
            
            1 => {
                // 仅有一根K线
                // 起始开始的两K就存在包含关系，合理的处理方式是：
                // 1. 如果第一根K包含第二根K，直接忽略与第一根K存在包含的K线，直到遇到不包含的
                // 2. 如果第一根K包含在第二根K，忽略第一根K，从第二根K开始
                let last = self.window.get(-1).unwrap();
                let k1_include_k2 = last.bar.high >= bar.high && last.bar.low <= bar.low;
                let k2_include_k1 = last.bar.high <= bar.high && last.bar.low >= bar.low;
                if k1_include_k2 {
                    // 情况1，忽略当前Bar，直到遇到不包含的
                    return None;
                };

                if k2_include_k1 {
                    // 情况2，忽略K1,清空队列
                    self.window.clear();
                }
                // 当前Bar作为Candle放入队列
                self.add_candle(bar);
            }

            2 => {
                let merged = self.process_contain_relationship(bar);
                if !merged {
                    self.add_candle(bar);
                }
            }

            _ => {
                let merged = self.process_contain_relationship(bar);
                if !merged {
                    let result = self.check_fractal();
                    self.add_candle(bar);
                    return result;
                }
            }
        }
        None
    }*/
}

impl std::fmt::Debug for FractalDetector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FractalDetector")
            .field("window", &self.window)
            .field("next_index", &self.next_index)
            .finish()
    }
}