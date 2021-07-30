use crate::point::Point;
use crate::pen::Pen;

// 向上的线段采用向上合并, 向下的特征序列，找顶分型
// 向下的线段采用向下合并, 向上的特征序列，找底分型

#[derive(Debug)]
pub struct Seq {
    pub from: Point,
    pub to: Point
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeDirection {
    Up,
    Down,
}

impl Seq {
    pub fn new(
        from:Point,
        to:Point,
    ) -> Self {
        Self {
            from, to
        }
    }

    pub fn from_pen(p: &Pen) -> Self {
        Self {
            from: p.from,
            to: p.to
        }
    }

    pub fn start(&self) -> Point {
        self.from
    }

    pub fn end(&self) -> Point {
        self.to
    }

    pub fn high(&self) -> f64 {
        if self.from.price > self.to.price {
            self.from.price
        } else {
            self.to.price
        }
    }

    pub fn low(&self) -> f64 {
        if self.from.price < self.to.price {
            self.from.price
        } else {
            self.to.price
        }
    }

    pub fn merge_up(&mut self, rhs: &Seq) {
        let lhs_length = self.to.price - self.from.price;
        let rhs_length = self.to.price - self.from.price;
        let is_same =
            (lhs_length < 0.0 && rhs_length < 0.0) || (lhs_length > 0.0 && rhs_length > 0.0);

        let is_larger = (lhs_length.abs() - rhs_length.abs()) > 0.0;

        match (is_larger, is_same) {
            (false, true) => {
                self.from = self.to;
                self.to = rhs.from;
            }
            (false, false) => {
                self.to = rhs.from;
            }
            (true, true) => {
                self.to = rhs.to;
            }
            (true, false) => {
                self.from = self.to;
                self.to = rhs.to;
            }
        }
    }

    pub fn merge_down(&mut self, rhs: &Seq) {
        let lhs_length = self.to.price - self.from.price;
        let rhs_length = self.to.price - self.from.price;
        let is_same =
            (lhs_length < 0.0 && rhs_length < 0.0) || (lhs_length > 0.0 && rhs_length > 0.0);

        let is_large = (lhs_length.abs() - rhs_length.abs()) > 0.0;

        match (is_large, is_same) {
            (false, true) => {
                self.from = self.to;
                self.to = rhs.from;
            }
            (false, false) => {
                self.to = rhs.from;
            }
            (true, true) => {
                self.to = rhs.to;
            }
            (true, false) => {
                self.from = self.to;
                self.to = rhs.to;
            }
        }
    }

    pub fn merge(lhs: &mut Seq, rhs: &Seq, direction: MergeDirection) -> bool {
        let is_contain_1 = lhs.high() < rhs.high() && lhs.low() > rhs.low();
        let is_contain_2 = lhs.high() > rhs.high() && lhs.low() < rhs.low();
        let is_contain = is_contain_1 || is_contain_2;

        if !is_contain {
            return false;
        }

        match direction {
            MergeDirection::Up => lhs.merge_up(rhs),
            MergeDirection::Down => lhs.merge_down(rhs),
        }

        true
    }
}
