#[macro_use]
extern crate manifest_dir_macros;

use kline::{analyzer::Analyzer, bar::Bar, candle::Candle, fractal::Fractal, plot::*, time::*, util::*};

const EU_DATA: &str = path!("data", "EU_2021.csv");
const EU_CANDLE: &str = path!("data", "candle_list.csv");
const EU_FX: &str = path!("data", "fx_list.csv");
const EU_BI: &str = path!("data", "bi_list.csv");
const DUMP_BI: &str = path!("data", "dump_bi.csv");

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

/* 
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
        println!("lhs: {:?} rhs: {:?}", lhs, rhs);
        assert!(lhs.time == rhs.time && lhs.high == rhs.high && lhs.low == rhs.low);
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
            lhs.time == rhs.time
                && lhs.fx_mark == rhs.fx_mark
                && lhs.high == rhs.high
                && lhs.low == rhs.low
        );
    }

    println!("Compare Fx Successful");
}

fn dump_bi(analyzer: &Analyzer) {
    let bis = analyzer.get_bis();
    println!("Bi Count: {}   dump to: {}", bis.len(), DUMP_BI);
    let _ = dump_bi_to_csv(DUMP_BI, bis);
}

fn compare_bi(analyzer: &Analyzer) {
    // compare fractals
    let bis = load_bi_from_csv(EU_BI).unwrap();
    let parsed_bi = analyzer.get_bis();
    println!(
        "Bi Count: {} Parsed_bi count: {}",
        bis.len(),
        parsed_bi.len()
    );

    assert!(bis.len() == parsed_bi.len());

    for i in 0..bis.len() {
        let lhs = &bis[i];
        let rhs = &parsed_bi[i];
        assert!(
            lhs.time == rhs.time
                && lhs.fx_mark == rhs.fx_mark
                && lhs.high == rhs.high
                && lhs.low == rhs.low
        );
    }

    println!("Compare Bi Successful");
}
*/
fn draw(analyzer: &Analyzer, prefix:&str) {
    let mut bars: Vec<Bar> = Vec::new();
    
        for bar in analyzer.get_bars() {
            bars.push(bar.clone());
        }
    
    let _ = draw_bar_tradingview(
        prefix.to_string(),
        &bars,
        analyzer.get_bis(),
        analyzer.get_xd(),
    );
}

fn draw_bi(analyzer: &Analyzer, prefix:&str, bi: &Vec<Fractal>){
    let mut bars: Vec<Bar> = Vec::new();
   
        for bar in analyzer.get_bars() {
            bars.push(bar.clone());
        }
    
    let _ = draw_bar_tradingview(
        prefix.to_string(),
        &bars,
        bi,
        analyzer.get_xd(),
    );  
}

fn main() {
    // load bar data
    let bars = load_bar_csv(EU_DATA);

    // process test data
    let mut analyzer = Analyzer::new();
    for bar in &bars {
        analyzer.on_new_bar(bar);
    }

    //comapre_candle(&analyzer);

    //compare_fx(&analyzer);

    //dump_bi(&analyzer);

    //compare_bi(&analyzer);
    println!("analyzer_bi_list count = {}", analyzer.get_bis().len());
    draw(&analyzer, "bar");
    //let bis = load_bi_from_csv(EU_BI).unwrap();
    //println!("json_bi_list count = {}", bis.len());
    //draw_bi(&analyzer, "bar", "bi", &bis);

    //let s = read_template("candle".to_string()).unwrap();
    //println!("index.html\n{}", s);
}
