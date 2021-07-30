use crate::fractal::Fractal;
use crate::ringbuffer::RingBuffer;
use crate::pen_rule::{self, MergeAction};

#[derive(Debug, Clone)]
pub enum PenEvent {
    First(Fractal, Fractal),
    New(Fractal),
    UpdateTo(Fractal),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PenDirection {
    Up,
    Down,
}

// TODO:考虑一种特殊情况就是顶分型高点相等或者底分型低点相等
#[derive(Debug)]
pub struct PenDetector {
    window: RingBuffer<Fractal>,
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
        pen_rule::detect_is_pen(
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

    fn state0(&mut self, f: Fractal) -> Option<PenEvent> {
        debug_assert!(self.window.len() == 0 && !self.has_pen);
        self.window.push(f);
        None
    }

    fn state1(&mut self, f: Fractal) -> Option<PenEvent> {
        debug_assert!(self.window.len() == 1 && !self.has_pen);
        let last = self.window.get(-1).unwrap();
        if last.is_same_type(&f) {
            // 1.1
            let action = pen_rule::merge_same_fx_type(last, &f);
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

    fn state2(&mut self, f: Fractal) -> Option<PenEvent> {
        debug_assert!(!self.ab_is_pen());
        debug_assert!(!self.has_pen);
        debug_assert!(self.window.len() == 2);

        let b = self.window.get(-1).unwrap();
        let bc_is_pen = pen_rule::detect_is_pen(b, &f);
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
                let action = pen_rule::merge_same_fx_type(b, &f);
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
                let action = pen_rule::merge_same_fx_type(a, &f);
                if action == MergeAction::Replace {
                    // 2.2.2.2
                    self.window.clear();
                    self.window.push(f);
                }
            }
        }
        None
    }

    fn state3(&mut self, f: Fractal) -> Option<PenEvent> {
        debug_assert!(self.ab_is_pen());
        debug_assert!(self.has_pen);
        debug_assert!(self.window.len() == 2);

        let b = self.window.get(-1).unwrap();
        let bc_is_pen = pen_rule::detect_is_pen(b, &f);
        if bc_is_pen {
            // 3.1
            let c = f.clone();
            self.window.pop_front();
            self.window.push(f);
            //self.ab_pen_complete_bc_pen_new();
            return Some(PenEvent::New(c));
        } else {
            if b.is_same_type(&f) {
                let action = pen_rule::merge_same_fx_type(b, &f);
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

    fn state4(&mut self, f: Fractal) -> Option<PenEvent> {
        debug_assert!(self.ab_is_pen());
        debug_assert!(!self
            .window
            .get(-2)
            .unwrap()
            .is_same_type(self.window.get(-1).unwrap()));
        debug_assert!(!pen_rule::detect_is_pen(
            self.window.get(-2).unwrap(),
            self.window.get(-1).unwrap()
        ));
        debug_assert!(self.has_pen);
        debug_assert!(self.window.len() == 3);

        let c = self.window.get(-1).unwrap();
        if c.is_same_type(&f) {
            // 4.1
            let action = pen_rule::merge_same_fx_type(c, &f);
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
            let action = pen_rule::merge_same_fx_type(b, &f);
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

    pub fn on_new_fractal(&mut self, f: Fractal) -> Option<PenEvent> {
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
