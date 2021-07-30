use crate::bar::Bar;
use crate::candle::Candle;
use crate::fractal::{FractalType, Fractal};
use crate::time::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::vec::Vec;

pub fn read_file_content(filename: &str)-> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

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

/* 
pub fn load_fx_from_csv(filename: &str) -> std::io::Result<Vec<Fx>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(load_fx_from_str(contents.as_str()))
}

 
fn load_fx_from_str(csv: &str) -> Vec<Fractal> {
    let mut fxs: Vec<Fractal> = Vec::new();
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
        let fx = if fx_mark == FractalType::Top {
            Fractal::new(time, fx_mark, fx, start_dt, end_dt, fx, fx_low, fx_high, fx_low, index)
        }else {
            Fractal::new(time, fx_mark, fx, start_dt, end_dt, fx_high, fx,fx_high, fx_low, index)
        };
        index += 1;
        fxs.push(fx);
    }
    fxs
}*/
/* 
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
        let fx = if fx_mark == FractalType::Top {
            Fx::new(time, fx_mark, fx, start_dt, end_dt, fx, fx_low, fx_high, fx_low, index)
        }else {
            Fx::new(time, fx_mark, fx, start_dt, end_dt, fx_high, fx,fx_high, fx_low, index)
        };
        index += 1;
        fxs.push(fx);
    }
    fxs
}


pub fn dump_bi_to_csv(filename: &str, bis: &Vec<Fractal>) -> Result<(), Box<dyn Error>> {
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
        let mark_str = if record.fractal_type() == FractalType::Top {
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
*/
pub fn dump_fx_to_csv(filename: &str, fxs: &Vec<Fractal>) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut wtr = csv::Writer::from_writer(file);
    // write header
    wtr.write_record(&[
        "datetime", "type","price","high", "low"
    ])?;
    for record in fxs {
        let dt_str = time_to_str(record.time());
        let mark_str = if record.fractal_type() == FractalType::Top {
            "Top".to_string()
        } else {
            "Bottom".to_string()
        };
        let price_str = format!("{}", record.price());
        let high_str = format!("{}", record.high());
        let low_str = format!("{}", record.low());
        wtr.write_record(&[
            dt_str, mark_str, price_str, high_str, low_str,
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn dump_candle_to_csv(filename: &str, candles: &Vec<Candle>) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut wtr = csv::Writer::from_writer(file);
    // write header
    wtr.write_record(&[
        "datatime","open","high","low","close","volume"
    ])?;
    for record in candles {
        let dt_str = time_to_str(record.bar.time);
        let open_str = format!("{}", record.bar.open);
        let high_str = format!("{}", record.bar.high);
        let low_str = format!("{}", record.bar.low);
        let close_str = format!("{}", record.bar.close);
        let vol_str = format!("{}", record.bar.vol);
        wtr.write_record(&[
            dt_str, open_str, high_str, low_str,close_str, vol_str
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn cargo_path(join_path:Option<&str>) -> PathBuf {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
     if join_path.is_some() {
      path = path.join(join_path.unwrap())
    }
    path
}

pub fn write_content_to_file(filename:&str, contents: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(contents.as_bytes())?;
    file.flush()?;
    Ok(())
}