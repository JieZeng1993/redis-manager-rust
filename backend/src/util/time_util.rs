use chrono::{DateTime, Local, LocalResult, TimeZone};
use rbatis::DateTimeNative;

pub fn convert(source_time:Option<DateTimeNative>) ->Option<DateTime<Local>> {
    match source_time {
        Some(source_time) => {
            match Local.from_local_datetime(&*source_time) {
                LocalResult::Single(source_time) => Some(source_time),
                //可能是None,也可能是Ambiguous
                _ => { None }
            }
        }
        None => None
    }
}