use crate::bar::Bar;
use crate::fx::{Fx, FractalType};
use std::path::PathBuf;
use std::process::Command;
use std::vec::Vec;
use std::{env, fs::File};
use std::error::Error;
use serde::Serialize;
use std::io::prelude::*;
use manifest_dir_macros::path;
use crate::util::*;

const DEFAULT_HTML_APP_NOT_FOUND: &str = "Could not find default application for HTML files.";

fn templates_root_path() -> PathBuf {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let templates = root.join("plot");
    templates
}

fn render_bar_tradingview(bar: &Bar) -> String {
    format!(
        "{{ time:{}, open:{}, high:{}, low:{}, close:{} }}",
        bar.time / 1000,
        bar.open,
        bar.high,
        bar.low,
        bar.close
    )
}

fn render_fractal_tradingview(f: &Fx) -> String {
    let price = if f.fx_mark == FractalType::Top {
        f.high
    } else {
        f.low
    };
    format!("{{ time:{}, value: {} }}", f.time / 1000, price)
}

fn render_bars_tradingview(
    bars: &Vec<Bar>,
    pens: &Vec<Fx>,
    segments: &Vec<Fx>,
) -> String {
    let mut buf = String::new();
    let header = "Data ={ \n";
    let bar_header = "Bar : [\n";
    let bar_bottom = "],\n";
    let line_header = "Pen : [\n";
    let line_bottom = "],\n";
    let segment_header = "Segment : [\n";
    let segment_bottom = "]\n";
    let bottom = "}";
    buf.push_str(header);

    // candle data
    buf.push_str(bar_header);
    let bdata: Vec<String> = bars
        .into_iter()
        .map(|bar| render_bar_tradingview(bar))
        .collect();
    let bar_data = bdata.join(",\n");
    buf.push_str(bar_data.as_str());
    buf.push_str(bar_bottom);

    // line data
    buf.push_str(line_header);
    let fdata: Vec<String> = pens
        .into_iter()
        .map(|f| render_fractal_tradingview(f))
        .collect();
    let line_data = fdata.join(",\n");
    buf.push_str(line_data.as_str());
    buf.push_str(line_bottom);

    // segment data
    buf.push_str(segment_header);
    let sdata: Vec<String> = segments
        .into_iter()
        .map(|f| render_fractal_tradingview(f))
        .collect();
    let segment_data = sdata.join(",\n");
    buf.push_str(segment_data.as_str());
    buf.push_str(segment_bottom);
    //
    buf.push_str(bottom);
    buf
}

const INDEX_HTML_TEMPLATE: &str = path!("plot", "index_chart.html");
#[derive(Serialize)]
struct Context {
    prefix: String,
}

pub fn read_template(prefix:String)->Result<String, Box<dyn Error>>{
    let mut contents = read_file_content(INDEX_HTML_TEMPLATE).unwrap();
    let offset = contents.find("</script>").unwrap();
    contents.insert_str(offset + 9, &format!(r#"<script src="./{}-chart-data.json"></script>"#, prefix));
    let title_offset = contents.find("</title>").unwrap();
    contents.insert_str(title_offset, &prefix);
    Ok(contents)
}

pub fn draw_bar_tradingview(prefix:String, bars: &Vec<Bar>, pens: &Vec<Fx>, segments: &Vec<Fx>) ->Result<(), Box<dyn Error>>{
    
    let rendered = render_bars_tradingview(bars, pens, segments);
    let rendered = rendered.as_bytes();
    let mut temp = cargo_path(Some("temp"));

    // write data.json
    temp.push(format!("{}-{}", prefix, "chart-data.json"));
    let temp_path = temp.to_str().unwrap();
    {
        let mut file = File::create(temp_path).unwrap();
        file.write_all(rendered)
            .expect("failed to write html output");
        file.flush().unwrap();
    }
    temp.pop();

    // copy index.html
    temp.push(format!("index_{}.html", prefix));
    let content = read_template(prefix).unwrap();
    let temp_path = temp.to_str().unwrap();
    {
        let mut file = File::create(temp_path).unwrap();
        file.write_all(content.as_bytes())
            .expect("failed to write html output");
        file.flush().unwrap();
    }
    println!("write to {}", temp_path);
    // display in browser
    show_with_default_app(temp.to_str().unwrap());
    temp.pop();
    Ok(())
}

#[cfg(target_os = "linux")]
fn show_with_default_app(temp_path: &str) {
    Command::new("xdg-open")
        .args(&[temp_path])
        .output()
        .expect(DEFAULT_HTML_APP_NOT_FOUND);
}

#[cfg(target_os = "macos")]
fn show_with_default_app(temp_path: &str) {
    Command::new("open")
        .args(&[temp_path])
        .output()
        .expect(DEFAULT_HTML_APP_NOT_FOUND);
}

#[cfg(target_os = "windows")]
fn show_with_default_app(temp_path: &str) {
    Command::new("cmd")
        .arg("/C")
        .arg(format!(r#"start {}"#, temp_path))
        .output()
        .expect(DEFAULT_HTML_APP_NOT_FOUND);
}
