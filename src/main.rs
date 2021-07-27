use kline::util::load_bar_from_csv;
#[macro_use] extern crate manifest_dir_macros;
const EU_DATA: &str = path!("data","EU_2021.csv");
use kline::time::timestamp_to_utc;
use kline::analyzer::Analyzer;
fn main() {
    // load test data
    let bars = load_bar_from_csv(EU_DATA).unwrap();
    let bars_count = bars.len();
    let start_time = timestamp_to_utc(bars[0].time);
    let end_time  = timestamp_to_utc(bars.last().unwrap().time);
    println!("Load {} bars, start = {}, end = {}", bars_count, start_time, end_time);
    // process test data
    let mut analyzer = Analyzer::new();
    for bar in &bars {
        analyzer.on_new_bar(bar);
    }
    println!("Candle count: {}", analyzer.get_candles().len());
}
