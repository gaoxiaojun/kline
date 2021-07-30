
use crate::sequence::Seq;

#[derive(Debug, Clone)]
pub struct SeqFx {
    pub s1: Seq,
    pub s2: Seq,
    pub s3: Seq
}

impl SeqFx {
    pub fn build_seq_fx(s1: &Seq, s2:&Seq, s3: &Seq) -> Self {
        Self {
            s1:s1.clone(),
            s2:s2.clone(),
            s3:s3.clone()
        }
    }

    // 检查分型
    pub fn check_fractal(s1: &Seq, s2: &Seq, s3: &Seq) -> Option<SeqFx> {
        if SeqFx::is_top_fractal(s1,s2,s3) || SeqFx::is_bottom_fractal(s1,s2,s3)
        {
            return Some(SeqFx::build_seq_fx(s1, s2, s3));
        }
        None
    }

    pub fn is_top_fractal(s1: &Seq, s2: &Seq, s3: &Seq) -> bool {
        if s1.high() < s2.high() && s2.high() > s3.high() {
            true
        } else {
            false
        }
    }

    pub fn is_bottom_fractal(s1: &Seq, s2: &Seq, s3: &Seq) -> bool {
        if s1.low() > s2.low() && s2.low() > s3.low() {
            true
        } else {
            false
        }
    }
}