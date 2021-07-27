use chrono::{DateTime, Utc, TimeZone};

// 时间为EPOCH(1970/1/1)开始的毫秒数
pub type Time = i64;

pub const SECOND_UNIT: i64 = 1000; // 每秒 = 1000毫秒
pub const MINUTE_UNIT: i64 = SECOND_UNIT * 60;


pub fn timestamp_to_utc(time: Time) -> DateTime<Utc> {
    Utc.timestamp_millis(time)
}