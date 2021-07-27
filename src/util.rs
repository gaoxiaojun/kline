use std::fs::File;
use crate::bar::Bar;
use std::io::prelude::*;
use std::vec::Vec;
use chrono::{DateTime, NaiveDateTime, Utc};
use crate::fractal::{Fx,FractalType};
use crate::time::Time;

pub fn load_bar_from_csv(filename: &str) -> std::io::Result<Vec<Bar>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(load_csv_from_str(contents.as_str()))
}

fn parse_time(timestr:&str) -> Time {
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
    let mut index:u64 = 0;
    for record in reader.records() {
        let record = record.unwrap();
        let time = parse_time(AsRef::<str>::as_ref(&record[0]));
        let fx_mark_str = AsRef::<str>::as_ref(&record[1]);
        let fx_mark = if fx_mark_str == "g" {
            FractalType::Top
        }else {
            FractalType::Bottom
        };
        let fx = AsRef::<str>::as_ref(&record[2]).parse::<f64>().unwrap();
        let start_dt = parse_time(AsRef::<str>::as_ref(&record[3]));
        let end_dt = parse_time(AsRef::<str>::as_ref(&record[4]));
        let fx_high = AsRef::<str>::as_ref(&record[5]).parse::<f64>().unwrap();
        let fx_low = AsRef::<str>::as_ref(&record[6]).parse::<f64>().unwrap();
        let fx = Fx::new(time, fx_mark, fx, start_dt, end_dt, fx_high,fx_low, index);
        index += 1;
        fxs.push(fx);
    }
    fxs
}