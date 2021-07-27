#[macro_use] extern crate manifest_dir_macros;

use kline::{util::load_bar_from_csv,
time::timestamp_to_utc,
analyzer::Analyzer,
bar::Bar,
fractal_detector::FractalDetector,
};


const EU_DATA: &str = path!("data","EU_2021.csv");
const EU_CANDLE:&str = path!("data", "candle_list.csv");


fn load_bar_csv(filename: &str) -> Vec<Bar> {
    let bars = load_bar_from_csv(filename).unwrap();
    let bars_count = bars.len();
    let start_time = timestamp_to_utc(bars[0].time);
    let end_time  = timestamp_to_utc(bars.last().unwrap().time);
    println!("Load {}\nTotal:{} bars, start = {}, end = {}", filename, bars_count, start_time, end_time);
    bars
}
fn main() {
    // load bar data
    let bars = load_bar_csv(EU_DATA);
    let mut candles = load_bar_csv(EU_CANDLE);
    candles.pop(); // 最后一个Candle不用比较
    // process test data
    let mut analyzer = Analyzer::new();
    for bar in &bars {
        analyzer.on_new_bar(bar);
    }

    let parsed_candle = analyzer.get_candles();
    println!("Candle count: {} Parsed_Candle count: {}", candles.len(), parsed_candle.len());

    assert!(candles.len() == parsed_candle.len());

    for i in 0.. candles.len() {
        let lhs = &candles[i];
        let rhs = &parsed_candle[i].bar;
        //println!("lhs {:?} rhs {:?}", lhs, rhs);
        assert!(lhs.time == rhs.time && lhs.high == rhs.high && lhs.low == rhs.low);
    }
}
