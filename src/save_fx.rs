#[macro_use]
use manifest_dir_macros::path;

use crate::{
    analyzer::Analyzer, bar::Bar, time::timestamp_to_utc, util::*,fx::Fx
};

const EU_DATA: &str = path!("data", "EU_2021.csv");
const EU_CANDLE: &str = path!("data", "candle_list.csv");
const EU_FX: &str = path!("data", "fx_list.csv");
const DUMP_FX: &str = path!("data", "dump_fx.csv");
const DUMP_CANDLE: &str = path!("data", "dump_candle.csv");

fn load_bar_csv(filename: &str) -> Vec<Bar> {
    let bars = load_bar_from_csv(filename).unwrap();
    let bars_count = bars.len();
    let start_time = timestamp_to_utc(bars[0].time);
    let end_time = timestamp_to_utc(bars.last().unwrap().time);
    println!(
        "Load {}\nTotal:{} bars, start = {}, end = {}",
        filename, bars_count, start_time, end_time
    );
    bars
}

fn comapre_candle(analyzer: &Analyzer) {
    // compare candles
    let candles = load_bar_csv(EU_CANDLE);
    let parsed_candle = analyzer.get_candles();
    println!(
        "Candle count: {} Parsed_Candle count: {}",
        candles.len(),
        parsed_candle.len()
    );

    assert!(candles.len() == parsed_candle.len());

    for i in 0..candles.len() {
        let lhs = &candles[i];
        let rhs = &parsed_candle[i].bar;
        //println!("lhs: {:?} rhs: {:?}", lhs, rhs);
        assert!(/*lhs.time == rhs.time &&*/ lhs.high == rhs.high && lhs.low == rhs.low);
    }
    println!("Compare Candle Successful");
}

fn compare_fx(analyzer: &Analyzer) {
    // compare fractals
    let fxs = load_fx_from_csv(EU_FX).unwrap();
    let parsed_fx = analyzer.get_fxs();
    println!(
        "Fx Count: {} Parsed_fx count: {}",
        fxs.len(),
        parsed_fx.len()
    );

    assert!(fxs.len() == parsed_fx.len());

    for i in 0..fxs.len() {
        let lhs = &fxs[i];
        let rhs = &parsed_fx[i];
        assert!(
            //lhs.time == rhs.time
                 lhs.fx_mark == rhs.fx_mark
                //&& lhs.high == rhs.high
                //&& lhs.low == rhs.low
        );
    }

    println!("Compare Fx Successful");
}

fn dump_fx(analyzer: &Analyzer) {
    let fxs = analyzer.get_fxs();
    println!("Fx Count: {}   dump to: {}", fxs.len(), DUMP_FX);
    let _ = dump_fx_to_csv(DUMP_FX, fxs);
}

fn dump_candle(analyzer: &Analyzer) {
    let candles = analyzer.get_candles();
    println!("Candle Count: {}   dump to: {}", candles.len(), DUMP_CANDLE);
    let _ = dump_candle_to_csv(DUMP_CANDLE, candles);
}

pub fn save_fx_main() {
    // load bar data
    let bars = load_bar_csv(EU_DATA);

    // process test data
    let mut analyzer = Analyzer::new();
    for bar in &bars {
        analyzer.on_new_bar(bar);
    }

    comapre_candle(&analyzer);

    compare_fx(&analyzer);

    dump_candle(&analyzer);
    dump_fx(&analyzer);
}
