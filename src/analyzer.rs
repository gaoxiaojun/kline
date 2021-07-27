use crate::bar::Bar;
use crate::candle::Candle;
use crate::fractal::Fx;

#[derive(Debug)]
pub struct Analyzer {
    bars: Vec<Bar>,
    candles:Vec<Candle>,
    next_index:u64,
    fx_list:Vec<Fx>,
    bi_list:Vec<Fx>,
    xd_list:Vec<Fx>
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            bars: Vec::new(),
            candles: Vec::new(),
            fx_list:Vec::new(),
            next_index:0,
            bi_list:Vec::new(),
            xd_list:Vec::new(),
        }
    }

    fn update_candle(&mut self){}

    fn update_fx(&mut self){}

    fn update_bi_list(&mut self){}

    fn update_xd_list(&mut self){}
}