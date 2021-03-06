use crate::fractal::{Fx, FractalType};
use crate::ringbuffer::RingBuffer;
use crate::pen::{self,*};

// 一、寻找第一笔
// state 0
// +-0-+                            +-1-+
// |   |<-----A             =====>  | A |
// +---+                            +---+
// 保留A，转state1

// state 1
// +-1-+                            +-1-+               +--2-3--+
// | A |<-----B             ======> |A/B|        or     | A | B |
// +---+                            +---+               +---+---+
// 1.1 AB同类型，按同类合并规则处理AB，转state1
// 1.2 AB不同类型，保存B
// 1.2.1 AB不成笔转state2
// 1.2.2 AB成笔, emit First(A,B), has_pen=true，转state3

// state 2
// +---2---+                        +---3---+           +--2-3--+       +-1-+
// | A | B |<-----C         =====>  | B | C |     or    | A |B/C|   or  |A/C|
// +---+---+                        +--2.1--+           +---+---+       +---+
// 前提：AB不成笔
// 2.1 BC成笔 ---- 去掉A，保留BC，emit First(B,C), has_pen=true, 转state3
// 2.2 BC不成笔
// 2.2.1 BC同类，按同类合并规则处理
// 2.2.1.1 如果保留C，要检测AC是否成笔
// 2.2.1.1.1如果AC成笔，emit First(A,C),has_pen = true, 转state3
// 2.2.1.1.2如果不成笔，转state 2
// 2.2.1.2 如果保留B，抛弃C，转state2
// 2.2.2 BC不同类，按同类合并规则处理AC
// 2.2.2.1 如果保留A，则保留B，抛弃C，转state2
// 2.2.2.2 如果保留C，抛弃AB，转state1

// 二、已经有笔
// state 3
// +---3---+                        +---3---+           +---+-4-+---+
// | A | B |<-----C         =====>  | B | C |      or   | A | B | C |
// +---+---+                        +---+---+           +---+---+---+
// 前提 AB成笔
//  3.1 BC成笔   --- AB笔完成，emit New(C)，去掉A，剩下BC,转state3
//  3.2 BC不成笔，
//  3.2.1 BC类型不同，保留C，转state4
//  3.2.2 BC类型相同，按同类合并规则处理BC
//  3.2.2.1如果保留C，emit UpdateTo(C)，转state3
//  3.2.2.2如果保留B，抛弃C，转state3

// state 4
// +---+-4-+---+                    +---+-4-+---+       +---3---+
// | A | B | C |<-----D     =====>  | A | B |C/D|   or  | A |B/D|
// +---+---+---+                    +---+---+---+       +---+---+
// 前提 AB成笔且BC类型不同且BC不成笔
// 4.1 CD同类型-----按同类合并规则处理CD
// 4.1.1 如果保留D，要检测BD是否成笔
// 4.1.1.1如果BD成笔，AB笔完成，emit New(D)，去掉A，剩下BD，转state3
// 4.1.1.2如果不成笔，转 state4
// 4.1.2 如果保留C，抛弃D，转state4
// 4.2 CD不同类-----去掉C，按同类合并规则处理BD
// 4.2.1 如果保留B,保留C，转state4
// 4.2.2 如果保留D，emit UpdateTo(D)，转state3

// 关于分型有效性的问题
// 1. 分型包含
// 笔的两个端点分型，不能有后包含关系（前分型包含后分型)，允许有前包含关系（后分型包含前分型),
// 因为前包含关系会破坏当下确定的总原则
// 举例：A-B-C-D, A-B是笔，当BC成笔的时候，AB已经确认
// 如果后续笔延伸到D的时候，如果D包含B，BD无法确认会导致AB要修改

// 上述算法解决的99%的笔问题，但是还有一种情况，无法完美处理
// 例子:
// A-B-C-D-E
// 以A-B为向上笔为例，
// A-B向上成笔 B-C向下成笔 C-D向上不成笔，但是D高于B，E低于C, DE成笔
// 按照规则A-B笔在BC成笔的已经确认，最终B-E成向下笔，但是B不是这一笔的最高点
// 按照完美成笔，应该A-D成向上笔，D-E成向下笔，这样笔的端点是中间K的最高最低点，完美符合笔忽略中间波动的要求
// 这种情况实际上是要修正已经确认完成的笔，与当下确认笔有冲突的大原则有冲突
// 按照缠论从A0(1分钟)开始做推笔，线段才是最基本的构件
// 非完美的笔对线段没有影响

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeAction {
    Keep,
    Replace,
}

pub fn _merge_same_type(f1: &Fx, f2: &Fx) -> MergeAction {
    debug_assert!(f1.fx_mark == f2.fx_mark);
    if f1.fx_mark == FractalType::Top {
        if f1.high > f2.high {
            MergeAction::Keep
        } else {
            MergeAction::Replace
        }
    } else {
        if f1.low < f2.low {
            MergeAction::Keep
        } else {
            MergeAction::Replace
        }
    }
}

// TODO: 后分型包含前分型的情况需要处理
pub fn is_pen(f1: &Fx, f2: &Fx) -> bool {
    pen::detect_is_pen(f1, f2)
}

#[derive(Debug, Clone)]
pub enum PenEvent {
    First(Fx, Fx),
    New(Fx),
    UpdateTo(Fx),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PenDirection {
    Up,
    Down,
}

// TODO:考虑一种特殊情况就是顶分型高点相等或者底分型低点相等
#[derive(Debug)]
pub struct PenDetector {
    window: RingBuffer<Fx>,
    has_pen: bool,
}

impl PenDetector {
    pub fn new() -> Self {
        Self {
            window: RingBuffer::new(3),
            has_pen: false,
        }
    }

    fn _is_pen(&self, start_index: usize) -> bool {
        debug_assert!(self.window.len() >= 2 + start_index);
        is_pen(
            self.window.get(start_index as isize).unwrap(),
            self.window.get((start_index + 1) as isize).unwrap(),
        )
    }

    fn bc_is_pen(&self) -> bool {
        self._is_pen(1)
    }

    fn ab_is_pen(&self) -> bool {
        self._is_pen(0)
    }

    fn state0(&mut self, f: Fx) -> Option<PenEvent> {
        debug_assert!(self.window.len() == 0 && !self.has_pen);
        self.window.push(f);
        None
    }

    fn state1(&mut self, f: Fx) -> Option<PenEvent> {
        debug_assert!(self.window.len() == 1 && !self.has_pen);
        let last = self.window.get(-1).unwrap();
        if last.is_same_type(&f) {
            // 1.1
            let action = _merge_same_type(last, &f);
            if action == MergeAction::Replace {
                self.window.pop_back();
                self.window.push(f);
            }
        } else {
            // 1.2
            self.window.push(f);
            if self.ab_is_pen() {
                // 1.2.2
                self.has_pen = true;
                return Some(PenEvent::First(
                    self.window.get(0).unwrap().clone(),
                    self.window.get(1).unwrap().clone(),
                ));
            }
        }
        None
    }

    fn state2(&mut self, f: Fx) -> Option<PenEvent> {
        debug_assert!(!self.ab_is_pen());
        debug_assert!(!self.has_pen);
        debug_assert!(self.window.len() == 2);

        let b = self.window.get(-1).unwrap();
        let bc_is_pen = is_pen(b, &f);
        if bc_is_pen {
            // 2.1
            self.window.push(f);
            self.window.pop_front();
            self.has_pen = true;
            return Some(PenEvent::First(
                self.window.get(0).unwrap().clone(),
                self.window.get(1).unwrap().clone(),
            ));
        } else {
            // 2.2
            if b.is_same_type(&f) {
                // 2.2.1
                let action = _merge_same_type(b, &f);
                if action == MergeAction::Replace {
                    // 2.2.1.1
                    self.window.pop_back(); // pop b
                    self.window.push(f);
                    // test ac is pen?
                    if self.ab_is_pen() {
                        // 2.2.1.1.1
                        self.has_pen = true;
                        return Some(PenEvent::First(
                            self.window.get(0).unwrap().clone(),
                            self.window.get(1).unwrap().clone(),
                        ));
                    }
                }
            } else {
                // 2.2.2
                let a = self.window.get(0).unwrap();
                let action = _merge_same_type(a, &f);
                if action == MergeAction::Replace {
                    // 2.2.2.2
                    self.window.clear();
                    self.window.push(f);
                }
            }
        }
        None
    }

    fn state3(&mut self, f: Fx) -> Option<PenEvent> {
        debug_assert!(self.ab_is_pen());
        debug_assert!(self.has_pen);
        debug_assert!(self.window.len() == 2);

        let b = self.window.get(-1).unwrap();
        let bc_is_pen = is_pen(b, &f);
        if bc_is_pen {
            // 3.1
            let c = f.clone();
            self.window.pop_front();
            self.window.push(f);
            //self.ab_pen_complete_bc_pen_new();
            return Some(PenEvent::New(c));
        } else {
            if b.is_same_type(&f) {
                let action = _merge_same_type(b, &f);
                if action == MergeAction::Replace {
                    // 3.2.2.1
                    self.window.pop_back();
                    let c = f.clone();
                    self.window.push(f);
                    //self.ab_pen_update();
                    return Some(PenEvent::UpdateTo(c));
                }
            } else {
                // 3.2.1
                self.window.push(f);
            }
        }

        None
    }

    fn state4(&mut self, f: Fx) -> Option<PenEvent> {
        debug_assert!(self.ab_is_pen());
        debug_assert!(!self
            .window
            .get(-2)
            .unwrap()
            .is_same_type(self.window.get(-1).unwrap()));
        debug_assert!(!is_pen(
            self.window.get(-2).unwrap(),
            self.window.get(-1).unwrap()
        ));
        debug_assert!(self.has_pen);
        debug_assert!(self.window.len() == 3);

        let c = self.window.get(-1).unwrap();
        if c.is_same_type(&f) {
            // 4.1
            let action = _merge_same_type(c, &f);
            if action == MergeAction::Replace {
                // 4.1.1
                self.window.pop_back();
                self.window.push(f);
                if self.bc_is_pen() {
                    // 4.1.1.1
                    self.window.pop_front();
                    return Some(PenEvent::New(self.window.get(-1).unwrap().clone()));
                }
            }
        } else {
            // 4.2
            //self.window.pop_back();
            let b = self.window.get(-2).unwrap();
            let action = _merge_same_type(b, &f);
            if action == MergeAction::Replace {
                // 4.2.2
                self.window.pop_back();
                self.window.pop_back();
                let c = f.clone();
                self.window.push(f);
                return Some(PenEvent::UpdateTo(c));
            }
        }

        None
    }

    pub fn on_new_fractal(&mut self, f: Fx) -> Option<PenEvent> {
        let len = self.window.len();
        let is_pen = self.has_pen;

        match (is_pen, len) {
            (false, 0) => self.state0(f),
            (false, 1) => self.state1(f),
            (false, 2) => self.state2(f),
            (true, 2) => self.state3(f),
            (true, 3) => self.state4(f),
            (_, _) => {
                unreachable!()
            }
        }
    }
}
