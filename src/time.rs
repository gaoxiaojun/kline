use chrono::{DateTime, TimeZone, Utc};

// 时间为EPOCH(1970/1/1)开始的毫秒数
pub type Time = i64;

pub const SECOND_UNIT: i64 = 1000; // 每秒 = 1000毫秒
pub const MINUTE_UNIT: i64 = SECOND_UNIT * 60;

pub fn timestamp_to_utc(time: Time) -> DateTime<Utc> {
    Utc.timestamp_millis(time)
}

pub fn datetime_to_str(time: DateTime<Utc>) -> String {
    let format = time.format("%Y.%m.%d %H:%M:%S");
    format!("{}", time)
}

pub fn time_to_str(time:Time) -> String {
    let datetime = timestamp_to_utc(time);
    datetime_to_str(datetime)
}