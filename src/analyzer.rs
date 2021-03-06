use crate::bar::Bar;
use crate::candle::Candle;
use crate::fd::FractalDetectorWithoutCandleList;
use crate::fractal::Fractal;
use crate::pd::*;
use crate::pen::Pen;
use crate::segment::Segment;


#[derive(Debug)]
pub struct Analyzer {
    bars: Vec<Bar>,
    next_index: u64,
    fx_list: Vec<Fractal>,
    bi_list: Vec<Fractal>,
    xd_list: Vec<Fractal>,
    pd: PenDetector,
    fd: FractalDetectorWithoutCandleList,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            bars: Vec::new(),
            fx_list: Vec::new(),
            next_index: 0,
            bi_list: Vec::new(),
            xd_list: Vec::new(),
            pd: PenDetector::new(),
            fd: FractalDetectorWithoutCandleList::new(),
        }
    }

    pub fn get_bars(&self) -> &Vec<Bar> {
        &self.bars
    }

    /*pub fn get_candles(&self) -> &Vec<Candle> {
        self.fd.get_candles()
    }*/

    pub fn get_fxs(&self) -> &Vec<Fractal> {
        &self.fx_list
    }

    pub fn get_bis(&self) -> &Vec<Fractal> {
        &self.bi_list
    }

    pub fn get_xd(&self) -> &Vec<Fractal> {
        &self.xd_list
    }

    pub fn on_new_segment(&mut self) {
        
    }

    pub fn on_new_pen(&mut self) -> Option<Segment> {

        None
    }

    pub fn on_new_fx(&mut self, f: Fractal) -> bool {
        let event = self.pd.on_new_fractal(f);
            if let Some(pen_event) = event {
                match pen_event {
                    PenEvent::First(a, b) => {
                        self.bi_list.push(a);
                        self.bi_list.push(b);
                        self.on_new_pen();
                        return true;
                    }
                    PenEvent::New(a) => {
                        self.bi_list.push(a);
                        self.on_new_pen();
                        return true;
                    }

                    PenEvent::UpdateTo(a) => {
                        self.bi_list.pop();
                        self.bi_list.push(a);
                    }
                }
            }
        false
    }
    
    pub fn on_new_bar(&mut self, bar: &Bar) {
        self.bars.push(bar.clone());
        let new_fx = self.fd.on_new_bar(bar);
        if new_fx.is_none() {
            return;
        }

        //self.fx_list.push(new_fx.clone().unwrap());
        self.on_new_fx(new_fx.unwrap());   
    }
}
