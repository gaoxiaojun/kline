use crate::fx::{FractalType, Fx};
// 集中笔规则处理
pub fn detect_is_pen(f1: &Fx, f2: &Fx) -> bool {
    if f1.fx_mark == FractalType::Top
        && f2.fx_mark == FractalType::Bottom
        && f1.has_enough_distance(f2)
        && f2.price < f1.price
        && !fx_is_contain(f1,f2)
    {
        return true;
    }

    if f1.fx_mark == FractalType::Bottom
        && f2.fx_mark == FractalType::Top
        && f1.has_enough_distance(f2)
        && f2.price > f1.price
        && !fx_is_contain(f1,f2)
    {
        return true;
    }

    false
}

// 分型包含规则，第二根Candle的最高最低作为分型区间
pub fn fx_is_contain(lhs: &Fx, rhs: &Fx) -> bool {
    if (lhs.fx_mark == FractalType::Top && lhs.low < rhs.low && lhs.high < rhs.high)
        || (lhs.fx_mark == FractalType::Bottom && lhs.high > rhs.high && lhs.low > rhs.low)
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
pub fn merge_same_fx_type(f1: &Fx, f2: &Fx) -> MergeAction {
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
