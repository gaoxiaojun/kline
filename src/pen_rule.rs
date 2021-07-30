use crate::fractal::{FractalType, Fractal};
// 集中笔规则处理
pub fn detect_is_pen(f1: &Fractal, f2: &Fractal) -> bool {
    if f1.fractal_type() == FractalType::Top
        && f2.fractal_type() == FractalType::Bottom
        && f1.has_enough_distance(f2)
        && f2.price() < f1.price()
        && !fx_is_contain(f1,f2)
    {
        return true;
    }

    if f1.fractal_type() == FractalType::Bottom
        && f2.fractal_type() == FractalType::Top
        && f1.has_enough_distance(f2)
        && f2.price() > f1.price()
        && !fx_is_contain(f1,f2)
    {
        return true;
    }

    false
}

// 分型包含规则，第二根Candle的最高最低作为分型区间
pub fn fx_is_contain(lhs: &Fractal, rhs: &Fractal) -> bool {
    if (lhs.fractal_type() == FractalType::Top && lhs.low() < rhs.low() && lhs.high() < rhs.high())
        || (lhs.fractal_type() == FractalType::Bottom && lhs.high() > rhs.high() && lhs.low() > rhs.low())
    {
        true
    } else {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeAction {
    Keep,
    Replace,
}

// 同类分型合并规则
pub fn merge_same_fx_type(f1: &Fractal, f2: &Fractal) -> MergeAction {
    debug_assert!(f1.fractal_type() == f2.fractal_type());
    if f1.fractal_type() == FractalType::Top {
        if f1.highest() > f2.highest() {
            MergeAction::Keep
        } else {
            MergeAction::Replace
        }
    } else {
        if f1.lowest() < f2.lowest() {
            MergeAction::Keep
        } else {
            MergeAction::Replace
        }
    }
}
