use crate::bar::Bar;
use crate::fractal::{FractalType, Fx};
use crate::time::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::vec::Vec;

pub fn load_bar_from_csv(filename: &str) -> std::io::Result<Vec<Bar>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(load_csv_from_str(contents.as_str()))
}

fn parse_time(timestr: &str) -> Time {
    let dt = NaiveDateTime::parse_from_str(timestr, "%Y.%m.%d %H:%M:%S").unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(dt, Utc);
    datetime.timestamp_millis()
}
fn load_csv_from_str(csv: &str) -> Vec<Bar> {
    let mut bars: Vec<Bar> = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record.unwrap();
        let timestr: &str = AsRef::<str>::as_ref(&record[0]);
        let time = parse_time(timestr);
        let open = AsRef::<str>::as_ref(&record[1]).parse::<f64>().unwrap();
        let close = AsRef::<str>::as_ref(&record[4]).parse::<f64>().unwrap();
        let high = AsRef::<str>::as_ref(&record[2]).parse::<f64>().unwrap();
        let low = AsRef::<str>::as_ref(&record[3]).parse::<f64>().unwrap();
        let vol = AsRef::<str>::as_ref(&record[5]).parse::<f64>().unwrap();
        let bar = Bar::new(time, open, high, low, close, vol);
        bars.push(bar);
    }
    bars
}

pub fn load_fx_from_csv(filename: &str) -> std::io::Result<Vec<Fx>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(load_fx_from_str(contents.as_str()))
}

fn load_fx_from_str(csv: &str) -> Vec<Fx> {
    let mut fxs: Vec<Fx> = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv.as_bytes());
    let mut index: u64 = 0;
    for record in reader.records() {
        let record = record.unwrap();
        let time = parse_time(AsRef::<str>::as_ref(&record[0]));
        let fx_mark_str = AsRef::<str>::as_ref(&record[1]);
        let fx_mark = if fx_mark_str == "g" {
            FractalType::Top
        } else {
            FractalType::Bottom
        };
        let fx = AsRef::<str>::as_ref(&record[2]).parse::<f64>().unwrap();
        let start_dt = parse_time(AsRef::<str>::as_ref(&record[3]));
        let end_dt = parse_time(AsRef::<str>::as_ref(&record[4]));
        let fx_high = AsRef::<str>::as_ref(&record[5]).parse::<f64>().unwrap();
        let fx_low = AsRef::<str>::as_ref(&record[6]).parse::<f64>().unwrap();
        let fx = Fx::new(time, fx_mark, fx, start_dt, end_dt, fx_high, fx_low, index);
        index += 1;
        fxs.push(fx);
    }
    fxs
}

pub fn load_bi_from_csv(filename: &str) -> std::io::Result<Vec<Fx>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(load_bi_from_str(contents.as_str()))
}

fn load_bi_from_str(csv: &str) -> Vec<Fx> {
    let mut fxs: Vec<Fx> = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv.as_bytes());
    let mut index: u64 = 0;
    for record in reader.records() {
        let record = record.unwrap();
        let time = parse_time(AsRef::<str>::as_ref(&record[0]));
        let fx_mark_str = AsRef::<str>::as_ref(&record[1]);
        let fx_mark = if fx_mark_str == "g" {
            FractalType::Top
        } else {
            FractalType::Bottom
        };
        let fx = AsRef::<str>::as_ref(&record[6]).parse::<f64>().unwrap();
        let start_dt = parse_time(AsRef::<str>::as_ref(&record[2]));
        let end_dt = parse_time(AsRef::<str>::as_ref(&record[3]));
        let fx_high = AsRef::<str>::as_ref(&record[4]).parse::<f64>().unwrap();
        let fx_low = AsRef::<str>::as_ref(&record[5]).parse::<f64>().unwrap();
        let fx = Fx::new(time, fx_mark, fx, start_dt, end_dt, fx_high, fx_low, index);
        index += 1;
        fxs.push(fx);
    }
    fxs
}

pub fn dump_bi_to_csv(filename: &str, bis: &Vec<Fx>) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut wtr = csv::Writer::from_writer(file);
    // write header
    wtr.write_record(&[
        "datetime", "fx_mark", "fx", "start_dt", "end_st", "fx_high", "fx_low",
    ])?;
    for record in bis {
        let dt_str = time_to_str(record.time);
        let start_str = time_to_str(record.start);
        let end_str = time_to_str(record.end);
        let mark_str = if record.fx_mark == FractalType::Top {
            "g".to_string()
        } else {
            "d".to_string()
        };
        let price_str = format!("{}", record.price);
        let high_str = format!("{}", record.high);
        let low_str = format!("{}", record.low);
        wtr.write_record(&[
            dt_str, mark_str, price_str, start_str, end_str, high_str, low_str,
        ])?;
    }
    wtr.flush()?;
    Ok(())
}


fn cargo_path(join_path:Option<String>) -> PathBuf {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
     if join_path.is_some() {
      path = path.join(join_path.unwrap())
    }
    path
}