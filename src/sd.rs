use crate::pen::Pen;
use crate::sequence::Seq;
use crate::seq_fx::SeqFx;
use crate::ringbuffer::RingBuffer;
use crate::sequence::MergeDirection;


/// SeqFractalDetectort用于检测标准特征序列分型
/// 输入Pen，输出分型


#[derive(Debug)]
pub struct SeqFractalDetector {
    window: RingBuffer<Seq>,
    direction: MergeDirection,
}

impl SeqFractalDetector {
    pub fn new() -> Self {
        Self {
            window: RingBuffer::new(3),
            direction: MergeDirection::Down
        }
    }

    // 当确定当前Pen与前Seq不存在合并关系的时候，该方法被调用
    fn add_seq(&mut self, pen: &Pen) {
        let c = Seq::from_pen(pen);
        self.window.push(c);
    }

    // 检查是否为顶底分型
    fn check_fractal(&self) -> Option<SeqFx> {
        if self.window.len() >= 3 {
            let k1 = self.window.get(-3).unwrap();
            let k2 = self.window.get(-2).unwrap();
            let k3 = self.window.get(-1).unwrap();

            SeqFx::check_fractal(k1, k2, k3)
        } else {
            None
        }
    }

    // 处理与当前Pen的包含关系
    fn merge_pen(&mut self, pen: &Pen) -> bool {
        // 队列中有至少两个经过包含处理的Seq
        debug_assert!(self.window.len() >= 2);

        let current = self.window.get_mut(-1).unwrap();

        let s = Seq::from_pen(pen);
        Seq::merge(current, &s, self.direction)
    }

    // 处理K线包含关系，更新内部缓冲区，检测分型
    pub fn on_new_pen(&mut self, pen: &Pen) -> Option<SeqFx> {
        let len = self.window.len();
        debug_assert!(len <= 3);

        match len {
            0 | 1 => {
                // 队列中没有K线
                self.add_seq(pen);
            }

            _ => {
                let merged = self.merge_pen(pen);
                if !merged {
                    let result = self.check_fractal();
                    self.add_seq(pen);
                    return result;
                }
            }
        }
        None
    }
}
