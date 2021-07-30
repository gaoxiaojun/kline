use crate::pen;
    fn search_first_bi(&mut self) -> bool {
        debug_assert!(self.bi_list.len() < 2);

        if self.bi_list.len() == 0 && self.fx_list.len() > 0 {
            self.bi_list.push(self.fx_list.first().unwrap().clone());
            return false;
        }



        let _1 = self.fx_list.len() -1;
        let _2 = self.fx_list.len() -2;
 
        let prev = &self.fx_list[_2];
        let next = &self.fx_list[_1];

        
        false
    }
    fn update_bi_list(&mut self) -> bool{
        if self.fx_list.len() < 2 {
            return false;
        }

        let _1 = self.fx_list.len() -1;
        let _2 = self.fx_list.len() -2;
 
        let prev = &self.fx_list[_2];
        let next = &self.fx_list[_1];
        if pen::fx_is_contain(prev,next) {
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
                // 前顶后底 and 距离足够 and 前高后低 => 新的下降笔
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

    // 检测并处理包含关系
    // 返回值: true:存在包含关系， false:没有包含关系
    // 一字板的处理会影响Candle的数量，暂时用merge函数取代
    pub fn merge_old(direction: Direction, current: &mut Candle, bar: &Bar) -> bool {
        // current,bar是否有包含关系
        if (current.bar.high >= bar.high && current.bar.low <= bar.low)
            || (current.bar.high <= bar.high && current.bar.low >= bar.low)
        {
            // 特殊的一字板与前一根K高低点相同情况的处理
            let high_eq_low = bar.high == bar.low; // 一字板

            match direction {
                Direction::Down => {
                    // 下包含，取低低
                    if high_eq_low && bar.low == current.bar.low {
                        // 一字板特例，不处理，直接忽略当前的bar
                        return true;
                    }

                    if current.bar.low > bar.low {
                        current.bar.time = bar.time;
                    }
                    current.bar.high = f64::min(bar.high, current.bar.high);
                    current.bar.low = f64::min(bar.low, current.bar.low);
                    current.bar.open = current.bar.high;
                    current.bar.close = current.bar.low;
                }

                Direction::Up => {
                    // 上包含，取高高
                    if high_eq_low && bar.high == current.bar.high {
                        // 一字板特例，不处理，直接忽略当前的bar
                        return true;
                    }

                    if current.bar.high < bar.high {
                        current.bar.time = bar.time;
                    }
                    current.bar.high = f64::max(bar.high, current.bar.high);
                    current.bar.low = f64::max(bar.low, current.bar.low);
                    current.bar.close = current.bar.high;
                    current.bar.open = current.bar.low;
                }
            }
            //current.bar.close = bar.close;
            true
        } else {
            false
        }
    }